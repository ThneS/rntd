//! tcp server
#[allow(unused_imports)]
use crate::tools::{Event, Frame, FrameCodec};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::info;

async fn start() {
    let listener = TcpListener::bind("0.0.0.0:9302").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        info!("accepted connection from: {}", socket.peer_addr().unwrap());
        tokio::spawn(async move {
            handler(socket).await;
        });
    }
}

async fn handler(mut socket: TcpStream) {
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await.unwrap();
    let _ = String::from_utf8_lossy(&buf[..n]);
    socket.write_all(b"ok\n").await.unwrap();
}

pub async fn collector_run() {
    tokio::spawn(start());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler() {
        test_start().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let mut socket = TcpStream::connect("127.0.0.1:9302").await.unwrap();
        socket.write_all(b"hello").await.unwrap();
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        let response = String::from_utf8_lossy(&buf[..n]);
        assert_eq!(response, "ok\n");
    }

    async fn test_start() {
        tokio::spawn(start());
    }
}
