use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub struct Link {
  pub label: String,
  pub href: String,
}
