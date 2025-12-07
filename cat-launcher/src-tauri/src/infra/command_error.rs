use serde::{ser::SerializeStruct, Serialize, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message}")]
pub struct SerializableError {
    #[source]
    source: Box<dyn std::error::Error + Send + Sync>,
    error_type: &'static str,
    message: String,
}

impl SerializableError {
    pub fn new<E>(source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
        for<'a> &'a E: Into<&'static str>,
    {
        Self {
            message: source.to_string(),
            error_type: (&source).into(),
            source: Box::new(source),
        }
    }
}

impl Serialize for SerializableError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("SerializableError", 2)?;
        st.serialize_field("type", self.error_type)?;
        st.serialize_field("message", &self.message)?;
        st.end()
    }
}
