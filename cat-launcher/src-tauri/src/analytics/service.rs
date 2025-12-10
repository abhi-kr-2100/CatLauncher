use std::collections::HashMap;

use async_trait::async_trait;

#[derive(thiserror::Error, Debug)]
pub enum AnalyticsError {
    #[error("failed to capture event: {0}")]
    Capture(String),
}

#[async_trait]
pub trait Analytics: Send + Sync {
    async fn track_event(
        &self,
        distinct_id: &str,
        event_name: &str,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<(), AnalyticsError>;
}

pub struct PosthogAnalytics {
    client: posthog_rs::Client,
}

impl PosthogAnalytics {
    pub fn new(client: posthog_rs::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Analytics for PosthogAnalytics {
    async fn track_event(
        &self,
        distinct_id: &str,
        event_name: &str,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<(), AnalyticsError> {
        let mut event = posthog_rs::Event::new(event_name, distinct_id);
        for (key, value) in properties {
            let _ = event.insert_prop(key, value);
        }

        if let Err(e) = self.client.capture(event).await {
            return Err(AnalyticsError::Capture(e.to_string()));
        }
        Ok(())
    }
}
