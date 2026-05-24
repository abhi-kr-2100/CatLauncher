# GitHub Mock API

A server and library for mocking the GitHub API. Written in Rust, with cross-language bindings available — see [docs/cross-language.md](docs/cross-language.md).

## Usage

### As a Standalone Binary

```bash
github-mock-api --host 0.0.0.0 --port 3000
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
github-mock-api = "0.1"
```

Use in your tests:

```rust
use github_mock_api::MockServer;

#[tokio::test]
async fn test_github_api() {
    // Start the mock server on 127.0.0.1 with a random available port
    let server = MockServer::start().await.expect("Failed to start mock server.");
    
    let uri = server.uri();
    
    println!("Mock server running on {}", uri);
    
    // Use the server in your tests
    // The server automatically shuts down when `server` is dropped
}
```

For more control over the host and port:

```rust
use std::net::{IpAddr, Ipv4Addr};

use github_mock_api::MockServer;

#[tokio::main]
async fn main() {
    // Start on a specific host and port (use 0 for random port)
    let host = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let server = MockServer::start_on(host, 8080).await.expect("Failed to start mock server.");
    
    println!("Server running on {}", server.uri());
    
    // Server shuts down when dropped
}
```

## License

GitHub Mock API © 2026 by Abhishek Kumar is licensed under CC BY-NC-ND 4.0. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc-nd/4.0/
