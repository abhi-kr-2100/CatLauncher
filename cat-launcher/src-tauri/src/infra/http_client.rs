use std::env;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;

fn create_github_pat_headers() -> Option<HeaderMap> {
    if let Ok(github_pat) = env::var("GITHUB_PAT") {
        if !github_pat.is_empty() {
            let authorization = format!("Bearer {}", github_pat);
            if let Ok(header_value) = HeaderValue::from_str(&authorization) {
                let mut headers = HeaderMap::new();
                headers.insert(AUTHORIZATION, header_value);
                return Some(headers);
            }
        }
    }
    None
}

pub fn create_http_client() -> Client {
    let mut builder = Client::builder().user_agent("cat-launcher");

    if let Some(headers) = create_github_pat_headers() {
        builder = builder.default_headers(headers);
    }

    builder.build().expect("Failed to build reqwest client")
}
