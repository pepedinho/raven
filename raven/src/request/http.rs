use std::collections::HashMap;

use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::request::Request;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[async_trait::async_trait]
impl Request for HttpRequest {
    type Output = HttpRequest;

    async fn parse(stream: &mut TcpStream) -> anyhow::Result<Self::Output> {
        let mut buffer = Vec::new();
        loop {
            let mut chunk = [0u8; 1024];
            let read = stream.read(&mut chunk).await?;
            if read == 0 {
                break; //client closed
            }

            buffer.extend_from_slice(&chunk[..read]);

            if buffer.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }

            if buffer.len() > 8 * 1024 * 1024 {
                return Err(anyhow::anyhow!("Http headers too large"));
            }
        }

        let request_str = String::from_utf8_lossy(&buffer);
        let (header_part, body_part) = request_str
            .split_once("\r\n\r\n")
            .unwrap_or((request_str.as_ref(), ""));

        let mut lines = header_part.split("\r\n");
        let start_line = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing start-line"))?;

        let mut parts = start_line.split_whitespace();
        let method = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing method"))?
            .to_string();
        let path = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing path"))?
            .to_string();
        let version = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing version"))?
            .to_string();

        let mut headers = HashMap::new();

        for line in lines {
            if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
            }
        }

        let mut body = body_part.to_string();

        if let Some(len_str) = headers.get("content-length")
            && let Ok(len) = len_str.parse::<usize>()
        {
            let current_len = body.len();

            if current_len < len {
                let to_read = len - current_len;
                let mut rest = vec![0u8; to_read];
                stream.read_exact(&mut rest).await?;
                body.push_str(&String::from_utf8_lossy(&rest));
            }
        }

        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}
