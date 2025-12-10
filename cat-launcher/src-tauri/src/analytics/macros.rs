#[macro_export]
macro_rules! track_event {
    ($analytics:expr, $user_repo:expr, $event_name:expr, $props:expr) => {
        tauri::async_runtime::spawn(async move {
            let mut props = $props;
            crate::analytics::helpers::track_event(
                &$analytics,
                &$user_repo,
                $event_name,
                props,
            )
            .await;
        });
    };
}
