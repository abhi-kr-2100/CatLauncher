use crate::active_release::repository::{
  ActiveReleaseRepository, ActiveReleaseRepositoryError,
};
use crate::variants::GameVariant;

/// Errors that can occur when interacting with the active release.
#[derive(thiserror::Error, Debug)]
pub enum ActiveReleaseError {
  /// An error occurred in the active release repository.
  #[error("failed to access active release: {0}")]
  Repository(#[from] ActiveReleaseRepositoryError),
}

impl GameVariant {
  /// Retrieves the version of the currently active release for this variant.
  pub async fn get_active_release(
    &self,
    repository: &dyn ActiveReleaseRepository,
  ) -> Result<Option<String>, ActiveReleaseError> {
    Ok(repository.get_active_release(self).await?)
  }

  /// Sets the version of the active release for this variant.
  pub async fn set_active_release(
    &self,
    version: &str,
    repository: &dyn ActiveReleaseRepository,
  ) -> Result<(), ActiveReleaseError> {
    repository.set_active_release(self, version).await?;
    Ok(())
  }
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
  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::infra::testing::test_database::TestDatabase;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_get_active_release_initial_none() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteActiveReleaseRepository::new(db.pool().clone());

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let active = variant.get_active_release(&repo).await?;
      assert_eq!(active, None);
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_set_and_get_active_release() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteActiveReleaseRepository::new(db.pool().clone());

    let variants = [
      (GameVariant::DarkDaysAhead, "v0.H"),
      (GameVariant::BrightNights, "cbn-v0.12.0"),
      (GameVariant::TheLastGeneration, "tlg-v1.0"),
    ];

    for (variant, version) in &variants {
      variant.set_active_release(version, &repo).await?;
    }

    for (variant, version) in &variants {
      let active = variant.get_active_release(&repo).await?;
      assert_eq!(active, Some(version.to_string()));
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_update_active_release() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteActiveReleaseRepository::new(db.pool().clone());

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      variant.set_active_release("v1.0.0", &repo).await?;
      assert_eq!(
        variant.get_active_release(&repo).await?,
        Some("v1.0.0".to_string())
      );

      variant.set_active_release("v2.0.0", &repo).await?;
      assert_eq!(
        variant.get_active_release(&repo).await?,
        Some("v2.0.0".to_string())
      );
    }

    Ok(())
  }
}
