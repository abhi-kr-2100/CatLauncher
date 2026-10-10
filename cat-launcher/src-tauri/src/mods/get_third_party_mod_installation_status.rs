use crate::mods::repository::installed_mods_repository::{
  InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::mods::types::ModInstallationStatus;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModInstallationStatusError {
  #[error("failed to check mod installation status: {0}")]
  Repository(#[from] InstalledModsRepositoryError),
}

pub async fn get_third_party_mod_installation_status(
  mod_id: &str,
  variant: &GameVariant,
  repository: &impl InstalledModsRepository,
) -> Result<
  ModInstallationStatus,
  GetThirdPartyModInstallationStatusError,
> {
  let is_installed =
    repository.is_mod_installed(mod_id, variant).await?;

  Ok(if is_installed {
    ModInstallationStatus::Installed
  } else {
    ModInstallationStatus::NotInstalled
  })
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_get_installation_status_not_installed() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let status = get_third_party_mod_installation_status(
      "uninstalled-mod",
      &variant,
      &repo,
    )
    .await?;
    assert_eq!(status, ModInstallationStatus::NotInstalled);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_installed() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "installed-mod";

    repo.add_installed_mod(mod_id, &variant).await?;

    let status = get_third_party_mod_installation_status(
      mod_id, &variant, &repo,
    )
    .await?;
    assert_eq!(status, ModInstallationStatus::Installed);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_after_uninstalled()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let mod_id = "toggle-mod";

    repo.add_installed_mod(mod_id, &variant).await?;
    assert_eq!(
      get_third_party_mod_installation_status(
        mod_id, &variant, &repo
      )
      .await?,
      ModInstallationStatus::Installed
    );

    repo.delete_installed_mod(mod_id, &variant).await?;
    assert_eq!(
      get_third_party_mod_installation_status(
        mod_id, &variant, &repo
      )
      .await?,
      ModInstallationStatus::NotInstalled
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());
    let mod_id = "isolated-mod";

    repo
      .add_installed_mod(mod_id, &GameVariant::DarkDaysAhead)
      .await?;

    assert_eq!(
      get_third_party_mod_installation_status(
        mod_id,
        &GameVariant::DarkDaysAhead,
        &repo
      )
      .await?,
      ModInstallationStatus::Installed
    );
    assert_eq!(
      get_third_party_mod_installation_status(
        mod_id,
        &GameVariant::BrightNights,
        &repo
      )
      .await?,
      ModInstallationStatus::NotInstalled
    );
    assert_eq!(
      get_third_party_mod_installation_status(
        mod_id,
        &GameVariant::TheLastGeneration,
        &repo
      )
      .await?,
      ModInstallationStatus::NotInstalled
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_all_variants() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteInstalledModsRepository::new(db.pool().clone());

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let mod_id = format!("status-mod-{}", variant);
      assert_eq!(
        get_third_party_mod_installation_status(
          &mod_id, &variant, &repo
        )
        .await?,
        ModInstallationStatus::NotInstalled
      );

      repo.add_installed_mod(&mod_id, &variant).await?;
      assert_eq!(
        get_third_party_mod_installation_status(
          &mod_id, &variant, &repo
        )
        .await?,
        ModInstallationStatus::Installed
      );
    }

    Ok(())
  }
}
