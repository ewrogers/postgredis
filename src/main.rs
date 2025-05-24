#![warn(clippy::pedantic)]

use crate::server::Server;
use std::net::SocketAddr;

mod client;
mod commands;
mod resp;
mod server;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 6379));

    let mut server = Server::new(addr).await;
    println!("Server listening on {}", addr);

    server.run().await;
}
