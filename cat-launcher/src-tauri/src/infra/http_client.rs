use std::sync::LazyLock;

use reqwest::Client;

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("cat-launcher")
        .build()
        .expect("Failed to build reqwest client")
});
