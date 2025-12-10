<h1> <img width="50" height="50" alt="image" src="https://github.com/user-attachments/assets/b5a404e5-a34f-4063-b3a5-fcc072c4dd81" /> Raven </h1>

**Raven** is a modular mock server written in Rust, designed to handle multiple network protocols such as HTTP and WebSocket. Its architecture is highly extensible, allowing developers to add new protocols, define dynamic mocks, and customize request handling logic.

---

## Features (Current)

* TCP listener with protocol detection
* Modular protocol handlers (HTTP, WebSocket, Custom)
* TOML-based mock engine with hierarchical route resolution
* Async handling with Tokio
* Clean, colorized, and flexible logging system
* Full request parsing (headers, path, body) for HTTP

---

<h2>
  Planned Features
</h2>

<img src="https://github.com/user-attachments/assets/64cb7b43-49eb-445d-82e3-da7f69c1a91f" width="200" align="right" />

* Lua integration for dynamic response generation
* Persistent client tracking and session management
* Advanced protocol detection and routing
* Full HTTP/WS keep-alive and request lifecycle management
* Extensible plugin system for custom protocols
* Enhanced structured logging with configurable output channels
* Web interface for monitoring active connections, mocks, and protocol stats
* Dynamic path handling in mocks, including parameters (e.g., `/user/{id}`)

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
* **Handler**: Implements protocol-specific logic, including response handling, mock matching, and keep-alive management.

---

## Mock Engine

* Mocks are defined in **TOML** files.
* The mock name comes from the rule defined inside the TOML, not the file name. For example, a file `hello.toml` containing a rule `[mock.goodbye]` will produce the path `game::new::goodbye` if located in `mock_test/game/new/hello.toml`.
* Supports hierarchical directory structure, e.g.:

```zsh
mock_test
├── game
│   ├── game.toml
│   └── new
│       └── hello.toml  # contains rule [mock.goodbye]
└── user
    └── user.toml
```

* HTTP mocks support:

  * Method and path matching
  * Header validation
  * Custom response body, status, headers, and options (like delay or connection close)
* Dynamic path parameters supported with `{param}` syntax.

---

## CLI Usage

```fish
raven --port 8080 --host 127.0.0.1 --mock mocks/user.toml
raven --port 8080 --host 127.0.0.1 --mocks mocks/ --watch
```

* `--port` / `-p`: TCP port to listen on (default 8080)
* `--host` / `-h`: Host/IP to bind the server (default 127.0.0.1)
* `--mock`: Single mock file to load
* `--mocks`: Directory containing multiple mock files
* `--watch`: Optional flag to watch mock directory for changes

---

## Usage (Example)

```rust
use raven::listener::Perch;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load mocks from a directory
    let cli = Cli::new(); // Example CLI parsing with mocks directory and port
    let listener = Arc::new(Perch::new(&cli)?);
    
    listener.listen_on("127.0.0.1").await?;
    Ok(())
}
```

* Logs are automatically formatted with protocol, sender, and message.
* Handlers can be extended to support new protocols or custom logic.
* Mocks are resolved dynamically per incoming request, based on the rule name inside the TOML file.

---

## License

MIT
