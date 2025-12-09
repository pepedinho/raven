
<h1>
  <img width="50" height="50" alt="image" src="https://github.com/user-attachments/assets/b5a404e5-a34f-4063-b3a5-fcc072c4dd81" /> 
  Raven
</h1>


**Raven** is a modular mock server written in Rust, designed to handle multiple network protocols such as HTTP and WebSocket. Its architecture is built to be highly extensible, allowing developers to easily add support for new protocols and customize request handling logic.

---

## Features (Current)

* TCP listener with protocol detection
* Modular protocol handlers (HTTP, WebSocket)
* Clean, colorized, and flexible logging system
* Async handling with Tokio

---
<h2>
  Planned Features
</h2>

<img src="https://github.com/user-attachments/assets/64cb7b43-49eb-445d-82e3-da7f69c1a91f" width="200" align="right" />

* Lua integration for dynamic protocol handling
* Persistent client tracking and session management
* Advanced protocol detection and routing
* Full HTTP/WS keep-alive and request lifecycle management
* Extensible plugin system for custom protocols
* Enhanced logging with structured output and configurable channels
* Web interface for monitoring active connections and protocols

---

## Architecture


```rust
+------------------+        +-------------------+        +---------------------+
|   TCP Listener   |  --->  | Protocol Resolver |  --->  | Protocol Handler    |
+------------------+        | (HTTP / WS / ..)  |        | (process & respond) |
                            +-------------------+        +---------------------+
```

* **Listener**: Accepts new TCP connections.
* **Resolver**: Detects the protocol of incoming connections.
* **Handler**: Implements protocol-specific logic, including response handling and keep-alive management.

---

## Usage (Example)

```rust
use raven::listener::Perch;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = Arc::new(Perch::new(8080));
    listener.listen_on("127.0.0.1").await?;
    Ok(())
}
```

* Logs are automatically formatted with protocol, sender, and message.
* Handlers can easily be extended for custom protocols.

---

## License

MIT
