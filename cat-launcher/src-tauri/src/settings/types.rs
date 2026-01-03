use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(
  Debug, Clone, Serialize, Deserialize, TS, PartialEq, Eq, Hash,
)]
#[ts(export)]
pub struct Font {
  pub name: String,
  pub path: String,
}
