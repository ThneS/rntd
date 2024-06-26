//! connection module
#[allow(unused_imports)]
use crate::tools::{Event, Frame, FrameCodec};
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::Decoder;
use tracing::info;

/// tcpstream and buffer with bytesmut
pub struct Connection {
    socket: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    /// create a new connection
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            socket,
            buffer: BytesMut::new(),
        }
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.socket.read(buf).await
    }
    #[allow(unused)]
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        self.socket.write_all(buf).await
    }

    async fn parse_frame(&mut self) -> Option<Frame> {
        if let Some(frame) = FrameCodec.decode(&mut self.buffer).unwrap() {
            return Some(frame);
        }
        loop {
            let mut buf = [0; 1024];
            let n = self.read(&mut buf).await.unwrap();
            if n == 0 {
                return None;
            }
            self.buffer.extend_from_slice(&buf[..n]);
            if let Some(frame) = FrameCodec.decode(&mut self.buffer).unwrap() {
                return Some(frame);
            }
        }
    }

    async fn read_frame(&mut self) -> Option<Frame> {
        if let Some(frame) = self.parse_frame().await {
            return Some(frame);
        }
        None
    }
}

pub async fn handler(mut conn: Connection) {
    while let Some(frame) = conn.read_frame().await {
        info!("frame: {:?}", frame);
    }
}

#[cfg(test)]
mod tests {
    use crate::collector::start;

    use super::*;
    use tokio_util::codec::Encoder;
    #[tokio::test]
    async fn test_collector_handler() {
        test_start().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let mut socket = TcpStream::connect("127.0.0.1:9302").await.unwrap();
        let frame = Frame::new(0x5a5a5a5a5a5a5a5a, 5, "hello".to_string());
        let mut buf = BytesMut::new();
        FrameCodec.encode(frame, &mut buf).unwrap();
        socket.write_all(&buf).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let res = FrameCodec.decode(&mut buf).unwrap();
        println!("{:?}", res);
        assert!(res.is_some());
    }

    async fn test_start() {
        tokio::spawn(start());
    }
}
