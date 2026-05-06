use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Represents a single achievement in the game.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
  /// The unique identifier of the achievement.
  pub id: String,
  /// The display name of the achievement.
  pub name: String,
}

/// Represents the achievements earned by a specific character.
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct CharacterAchievements {
  /// The name of the character.
  pub character_name: String,
  /// The list of achievements earned by this character.
  pub achievements: Vec<Achievement>,
}
