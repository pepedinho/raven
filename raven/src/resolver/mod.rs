use tokio::net::TcpStream;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Protocol {
    Http,
    WebSocket,
    Custom,
}

const HTTP_METHODS: [&str; 9] = [
    "GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS", "CONNECT", "TRACE",
];

impl Protocol {
    fn detect_http(data: &str) -> bool {
        if let Some(space_idx) = data.find(' ') {
            let methode = &data[..space_idx];
            if HTTP_METHODS.contains(&methode) && data.contains("HTTP/") {
                return true;
            }
        }
        false
    }

    fn detect_websocket(data: &str) -> bool {
        Self::detect_http(data) && data.to_ascii_lowercase().contains("upgrade: websocket")
    }

    pub async fn resolve(stream: &mut TcpStream) -> anyhow::Result<Protocol> {
        let mut buf = [0u8; 1024];

        let n = stream.peek(&mut buf).await?;
        let data = std::str::from_utf8(&buf[..n])?;

        if Self::detect_websocket(data) {
            return Ok(Protocol::WebSocket);
        } else if Self::detect_http(data) {
            return Ok(Protocol::Http);
        }

        Ok(Protocol::Custom)
    }
}
