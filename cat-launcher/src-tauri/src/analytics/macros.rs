#[macro_export]
macro_rules! track_event {
    ($analytics:expr, $user_repo:expr, $event_name:expr, $props:expr) => {
        tauri::async_runtime::spawn(async move {
            crate::analytics::helpers::track_event(
                $analytics.inner(),
                $user_repo.inner(),
                $event_name,
                props,
            )
            .await;
        });
    };
}
