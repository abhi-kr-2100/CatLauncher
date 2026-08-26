use serde::Deserialize;

#[derive(Deserialize)]
pub struct LastWorld {
  pub world_name: String,
}
