use reqwest::Client;

pub fn create_http_client() -> Client {
    Client::builder()
        .user_agent("cat-launcher")
        .build()
        .expect("Failed to build reqwest client")
}