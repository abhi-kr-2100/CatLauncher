use std::collections::HashMap;

use crate::{
    analytics::r#trait::Analytics,
    users::{repository::sqlite_users_repository::SqliteUsersRepository, service::get_or_create_user_id},
};

pub async fn track_event<A: Analytics + Send + Sync>(
    analytics: &A,
    user_repo: &SqliteUsersRepository,
    event_name: &str,
    properties: HashMap<String, serde_json::Value>,
) where
    A::Error: std::fmt::Display,
{
    let user_id = match get_or_create_user_id(user_repo).await {
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
