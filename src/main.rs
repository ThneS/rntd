//! main.rs is the entry point of the program.
#![deny(missing_docs)]
#![deny(warnings)]
mod center;
mod collector;
mod tools;
use collector::collector_run;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("start collector");
    collector_run().await;
    loop {
        std::thread::park();
    }
}
