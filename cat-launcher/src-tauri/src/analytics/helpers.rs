use std::collections::HashMap;

use tauri::{AppHandle, Manager, State};

use crate::{
    analytics::service::Analytics,
    users::{repository::sqlite_users_repository::SqliteUsersRepository, service::get_or_create_user_id},
};

pub async fn track_event(
    handle: &AppHandle,
    event_name: &str,
    properties: HashMap<String, serde_json::Value>,
) {
    if let Some(analytics) = handle.try_state::<Box<dyn Analytics>>() {
        let user_repo: State<SqliteUsersRepository> = handle.state();
        let user_id = match get_or_create_user_id(user_repo.inner()).await {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Failed to get or create user for analytics: {}", e);
                return;
            }
        };

        if let Err(e) = analytics.track_event(&user_id, event_name, properties).await {
            eprintln!("Failed to track event: {}", e);
        }
    }
}
