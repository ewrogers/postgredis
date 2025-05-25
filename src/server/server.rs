use crate::client::{handle_client, ClientEvent};
use crate::server::handler::handle_client_event;
use std::collections::VecDeque;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub struct Server {
    listener: TcpListener,
    client_event_tx: UnboundedSender<ClientEvent>,
    client_event_rx: UnboundedReceiver<ClientEvent>,
}

impl Server {
    pub async fn new(addr: SocketAddr) -> Self {
        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind to address");

        let (tx, rx) = unbounded_channel();

        Server {
            listener,
            client_event_tx: tx,
            client_event_rx: rx,
        }
    }

    pub async fn run(&mut self) {
        let mut event_queue: VecDeque<ClientEvent> = VecDeque::new();

        loop {
            tokio::select! {
                // Accept new client connections
                Ok((stream, addr)) = self.listener.accept() => {
                    println!("Client connected {}", addr);
                    let tx = self.client_event_tx.clone();

                    // Start a new task for the client to handle send/recv loop
                    tokio::spawn(async move {
                        handle_client(stream, tx).await
                    });
                }

                // Receive the next available event from clients
                Some(event) = self.client_event_rx.recv() => {
                    event_queue.push_back(event);
                }
            }

            // Process the event queue
            while let Some(event) = event_queue.pop_front() {
                handle_client_event(&event);
            }
        }
    }
}
