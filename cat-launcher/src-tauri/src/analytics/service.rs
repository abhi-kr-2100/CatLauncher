use std::collections::HashMap;

use async_trait::async_trait;
use posthog_rs::Error;

use super::r#trait::{Analytics, AnalyticsError};

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
    type Error = Error;
    async fn track_event(
        &self,
        distinct_id: &str,
        event_name: &str,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<(), AnalyticsError<Self::Error>> {
        let mut event = posthog_rs::Event::new(event_name, distinct_id);
        for (key, value) in properties {
            if let Err(e) = event.insert_prop(key, value) {
                eprintln!("Failed to insert prop into event: {}", e);
            }
        }

        self.client
            .capture(event)
            .await
            .map_err(AnalyticsError::Capture)?;
        Ok(())
    }
}
