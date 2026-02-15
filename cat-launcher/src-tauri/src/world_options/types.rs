use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct OptionValueLabel {
  pub value: String,
  pub label: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum OptionType {
  Boolean,
  Number,
  Enum,
  String,
  Group,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WorldOptionMetadata {
  pub id: String,
  pub name: String,
  pub description: String,
  #[serde(rename = "type")]
  pub option_type: OptionType,
  pub options: Option<Vec<OptionValueLabel>>,
  pub min: Option<f64>,
  pub max: Option<f64>,
  pub children: Option<HashMap<String, WorldOptionMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WorldOption {
  pub info: String,
  pub default: String,
  pub name: String,
  pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct World {
  pub name: String,
}
