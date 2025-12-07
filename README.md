
```
+------------------+         +------------------+         +------------------+
|  Client / App    |  <--->  | Core Rust Server |  <--->  | Embedded Lua VM  |
| HTTP / TCP / WS  |         | Async Tokio      |         |  mlua            |
+------------------+         +------------------+         +------------------+
       |                         ^       ^                    ^
       |                         |       |                    |
       |                         |       +---- API Rust ↔ Lua-+
       |                         |
       v                         |
   Logging / Metrics              |
                                 Hooks / Scripts
```