use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum AssetContent {
    Bytes(Vec<u8>),
    File(PathBuf),
}

#[derive(Debug, Clone)]
pub struct Asset {
    pub(crate) name: String,
    pub(crate) content: AssetContent,
    pub(crate) content_type: String,
}

impl Asset {
    pub fn from_bytes(
        name: impl Into<String>,
        bytes: Vec<u8>,
        content_type: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            content: AssetContent::Bytes(bytes),
            content_type: content_type.into(),
        }
    }

    pub fn from_path(
        name: impl Into<String>,
        path: impl Into<PathBuf>,
        content_type: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            content: AssetContent::File(path.into()),
            content_type: content_type.into(),
        }
    }
}
