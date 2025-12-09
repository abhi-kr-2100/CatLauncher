use serde::ser::SerializeStruct;
use serde::Serializer;
use strum::IntoStaticStr;
use tauri::State;

use crate::users::repository::sqlite_users_repository::SqliteUsersRepository;
use crate::users::service::{get_or_create_user_id, GetOrCreateUserIdError};

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum GetUserIdCommandError {
    #[error("failed to get or create user id: {0}")]
    GetOrCreateUserId(#[from] GetOrCreateUserIdError),
}

#[tauri::command]
pub async fn get_user_id(
    repo: State<'_, SqliteUsersRepository>,
) -> Result<String, GetUserIdCommandError> {
    let user_id = get_or_create_user_id(repo.inner()).await?;
    Ok(user_id)
}

impl serde::Serialize for GetUserIdCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("GetUserIdCommandError", 2)?;

        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;

        let msg = self.to_string();
        st.serialize_field("message", &msg)?;

        st.end()
    }
}
