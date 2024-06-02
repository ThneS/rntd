//! tcp server

use tokio::net::TcpListener;
use tracing::info;

use crate::tools::try_new_pool;
mod connection;
mod wiki;
use connection::{handler, Connection};

async fn start() {
    let listener = TcpListener::bind("0.0.0.0:9302").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        info!("accepted connection from: {}", socket.peer_addr().unwrap());
        let connection = Connection::new(socket);
        tokio::spawn(async move {
            handler(connection).await;
        });
    }
}

pub async fn collector_run() {
    let pool = try_new_pool().await.unwrap();
    wiki::wiki_update(pool.clone()).await;
    tokio::spawn(start());
}
