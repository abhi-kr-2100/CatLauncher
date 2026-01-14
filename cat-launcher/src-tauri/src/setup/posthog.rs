use tauri::{App, Manager};

use crate::users::{
  repository::sqlite_users_repository::SqliteUsersRepository,
  service::get_or_create_user_id,
};

pub fn manage_posthog(app: &App) {
  let api_key =
    option_env!("VITE_PUBLIC_POSTHOG_KEY").unwrap_or_default();
  let host =
    option_env!("VITE_PUBLIC_POSTHOG_HOST").unwrap_or_default();

  if api_key.is_empty() || host.is_empty() {
    eprintln!(
      "PostHog key or host not found, skipping initialization"
    );
    return;
  }

  let api_endpoint =
    format!("{}/capture/", host.trim_end_matches('/'));

  let options = posthog_rs::ClientOptionsBuilder::default()
    .api_key(api_key.to_string())
    .api_endpoint(api_endpoint)
    .build();

  match options {
    Ok(options) => {
      let client = posthog_rs::client(options);
      let handle = app.handle().clone();

      app.manage(client);

      tauri::async_runtime::spawn(async move {
        let user_repo: tauri::State<SqliteUsersRepository> =
          handle.state();
        let user_id = match get_or_create_user_id(user_repo.inner())
          .await
        {
          Ok(id) => id,
          Err(e) => {
            eprintln!(
                            "Failed to get or create user for PostHog identification: {}",
                            e
                        );
            return;
          }
        };

        if let Some(posthog) =
          handle.try_state::<posthog_rs::Client>()
        {
          let mut event =
            posthog_rs::Event::new("$identify", &user_id);
          let _ = event.insert_prop(
            "$set",
            std::collections::HashMap::from([(
              "is_user_identified",
              true,
            )]),
          );
          if let Err(e) = posthog.capture(event).await {
            eprintln!("Failed to capture identify event: {}", e);
          }
        }
      });
    }
    Err(e) => {
      eprintln!("Failed to build PostHog options: {}", e);
    }
  }
}
