use crate::play_time::repository::{
  PlayTimeRepository, PlayTimeRepositoryError,
};
use crate::variants::game_variant::GameVariant;

pub async fn get_play_time_for_variant(
  game_variant: &GameVariant,
  play_time_repository: &impl PlayTimeRepository,
) -> Result<i64, PlayTimeRepositoryError> {
  play_time_repository
    .get_play_time_for_variant(game_variant)
    .await
}

pub async fn get_play_time_for_version(
  game_variant: &GameVariant,
  version: &str,
  play_time_repository: &impl PlayTimeRepository,
) -> Result<i64, PlayTimeRepositoryError> {
  play_time_repository
    .get_play_time_for_version(game_variant, version)
    .await
}
