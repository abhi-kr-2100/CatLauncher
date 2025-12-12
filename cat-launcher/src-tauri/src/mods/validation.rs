use thiserror::Error;

#[derive(Error, Debug)]
#[error("invalid mod ID: {mod_id}. Mod IDs must be non-empty, contain only alphanumeric characters, underscores, or hyphens, and cannot contain path separators or dots")]
pub struct InvalidModIdError {
    pub mod_id: String,
}

pub fn validate_mod_id(mod_id: &str) -> Result<(), InvalidModIdError> {
    if is_valid_mod_id(mod_id) {
        Ok(())
    } else {
        Err(InvalidModIdError {
            mod_id: mod_id.to_string(),
        })
    }
}

fn is_valid_mod_id(mod_id: &str) -> bool {
    !mod_id.is_empty()
        && !mod_id.contains(['/', '\\', '.'])
        && mod_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}
