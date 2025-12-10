use async_trait::async_trait;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum AnalyticsError<E> {
    #[error("failed to capture event: {0}")]
    Capture(E),
}

#[async_trait]
pub trait Analytics: Send + Sync {
    type Error;
    async fn track_event(
        &self,
        distinct_id: &str,
        event_name: &str,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<(), AnalyticsError<Self::Error>>;
}
