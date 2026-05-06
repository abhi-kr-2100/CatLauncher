use serde::Deserialize;

/// Represents the structure of the `lastworld.json` file used by the game.
#[derive(Deserialize)]
pub struct LastWorld {
  /// The name of the world that was last played.
  pub world_name: String,
}
