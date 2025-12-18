use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Tip {
  pub text: Vec<String>,
}
