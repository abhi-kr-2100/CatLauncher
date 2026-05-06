use serde::Deserialize;

/// Represents a collection of game tips.
#[derive(Debug, Deserialize, Clone)]
pub struct Tip {
  /// The list of tip strings.
  pub text: Vec<String>,
}
