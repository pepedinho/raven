

# Raven

**Raven** is a modular mock server written in Rust, designed to handle multiple network protocols such as HTTP and WebSocket. Its architecture is built to be highly extensible, allowing developers to easily add support for new protocols and customize request handling logic.

---

## Features (Current)

* TCP listener with protocol detection
* Modular protocol handlers (HTTP, WebSocket)
* Clean, colorized, and flexible logging system
* Async handling with Tokio

---

## Planned Features

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
