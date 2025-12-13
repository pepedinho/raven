use raven::request::{Request, http::HttpRequest};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::test]
async fn parse_simple_request() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        socket
            .write_all(b"GET /hello HTTP/1.1\r\nHost: test\r\n\r\n")
            .await
            .unwrap();
    });

    let mut stream = TcpStream::connect(addr).await.unwrap();
    let req = HttpRequest::parse(&mut stream).await.unwrap();

    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/hello");
    assert_eq!(req.headers["host"], "test");
}
