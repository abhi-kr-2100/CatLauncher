use crate::soundpacks::repository::installed_soundpacks_repository::{
    InstalledSoundpacksRepository, InstalledSoundpacksRepositoryError,
};
use crate::soundpacks::types::SoundpackInstallationStatus;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartySoundpackInstallationStatusError {
  #[error("failed to check soundpack installation status: {0}")]
  Repository(#[from] InstalledSoundpacksRepositoryError),
}

pub async fn get_third_party_soundpack_installation_status(
  soundpack_id: &str,
  variant: &GameVariant,
  repository: &impl InstalledSoundpacksRepository,
) -> Result<
  SoundpackInstallationStatus,
  GetThirdPartySoundpackInstallationStatusError,
> {
  let is_installed = repository
    .is_soundpack_installed(soundpack_id, variant)
    .await?;

  Ok(if is_installed {
    SoundpackInstallationStatus::Installed
  } else {
    SoundpackInstallationStatus::NotInstalled
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
  use crate::soundpacks::repository::sqlite_installed_soundpacks_repository::SqliteInstalledSoundpacksRepository;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_get_installation_status_not_installed() -> TestResult
  {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let status = get_third_party_soundpack_installation_status(
      "uninstalled-pack",
      &variant,
      &repo,
    )
    .await?;
    assert_eq!(status, SoundpackInstallationStatus::NotInstalled);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_installed() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "installed-pack";

    repo.add_installed_soundpack(soundpack_id, &variant).await?;

    let status = get_third_party_soundpack_installation_status(
      soundpack_id,
      &variant,
      &repo,
    )
    .await?;
    assert_eq!(status, SoundpackInstallationStatus::Installed);

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_after_uninstalled()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;
    let soundpack_id = "toggle-pack";

    repo.add_installed_soundpack(soundpack_id, &variant).await?;
    assert_eq!(
      get_third_party_soundpack_installation_status(
        soundpack_id,
        &variant,
        &repo
      )
      .await?,
      SoundpackInstallationStatus::Installed
    );

    repo
      .delete_installed_soundpack(soundpack_id, &variant)
      .await?;
    assert_eq!(
      get_third_party_soundpack_installation_status(
        soundpack_id,
        &variant,
        &repo
      )
      .await?,
      SoundpackInstallationStatus::NotInstalled
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_variant_isolation()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());
    let soundpack_id = "isolated-pack";

    repo
      .add_installed_soundpack(
        soundpack_id,
        &GameVariant::DarkDaysAhead,
      )
      .await?;

    assert_eq!(
      get_third_party_soundpack_installation_status(
        soundpack_id,
        &GameVariant::DarkDaysAhead,
        &repo
      )
      .await?,
      SoundpackInstallationStatus::Installed
    );
    assert_eq!(
      get_third_party_soundpack_installation_status(
        soundpack_id,
        &GameVariant::BrightNights,
        &repo
      )
      .await?,
      SoundpackInstallationStatus::NotInstalled
    );
    assert_eq!(
      get_third_party_soundpack_installation_status(
        soundpack_id,
        &GameVariant::TheLastGeneration,
        &repo
      )
      .await?,
      SoundpackInstallationStatus::NotInstalled
    );

    Ok(())
  }

  #[tokio::test]
  async fn test_get_installation_status_all_variants() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo =
      SqliteInstalledSoundpacksRepository::new(db.pool().clone());

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let soundpack_id = format!("status-pack-{}", variant);
      assert_eq!(
        get_third_party_soundpack_installation_status(
          &soundpack_id,
          &variant,
          &repo
        )
        .await?,
        SoundpackInstallationStatus::NotInstalled
      );

      repo
        .add_installed_soundpack(&soundpack_id, &variant)
        .await?;
      assert_eq!(
        get_third_party_soundpack_installation_status(
          &soundpack_id,
          &variant,
          &repo
        )
        .await?,
        SoundpackInstallationStatus::Installed
      );
    }

    Ok(())
  }
}
