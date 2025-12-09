use strum::IntoStaticStr;
use tauri::State;

use cat_macros::CommandErrorSerialize;

use crate::users::repository::sqlite_users_repository::SqliteUsersRepository;
use crate::users::service::{get_or_create_user_id, GetOrCreateUserIdError};

#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
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
