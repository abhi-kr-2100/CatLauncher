use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "guide/GuideEntry.ts")]
pub struct GuideEntry {
  pub id: String,
  pub name: Option<String>,
  pub entry_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "guide/GuideEntityDetail.ts")]
pub struct GuideEntityDetail {
  #[ts(type = "unknown")]
  pub raw_json: serde_json::Value,
}
