#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Endpoint {
  host: String,
  port: Option<u16>,
  scheme: String,
}

impl Endpoint {
  pub fn new(
    host: impl Into<String>,
    port: Option<u16>,
    scheme: impl Into<String>,
  ) -> Self {
    Self {
      host: host.into(),
      port,
      scheme: scheme.into(),
    }
  }

  pub fn host(&self) -> &str {
    &self.host
  }

  pub fn port(&self) -> Option<u16> {
    self.port
  }

  pub fn scheme(&self) -> &str {
    &self.scheme
  }
}
