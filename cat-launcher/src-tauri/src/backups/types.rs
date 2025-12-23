use crate::variants::GameVariant;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct BackupEntry {
  pub id: i64,
  pub game_variant: GameVariant,
  pub release_version: String,
  pub timestamp: u64,
}
