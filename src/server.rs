use crate::client::handle_client;
use crate::commands::ClientCommand;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::time::interval;

pub struct Server {
    listener: TcpListener,
    client_event_tx: UnboundedSender<ClientCommand>,
    client_event_rx: UnboundedReceiver<ClientCommand>,
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
        // Frame interval for 120 FPS
        let mut frame_interval = interval(Duration::from_micros(1_000_000 / 120));
        let mut client_command_queue: VecDeque<ClientCommand> = VecDeque::new();

        loop {
            tokio::select! {
                // Accept new client connections
                Ok((stream, addr)) = self.listener.accept() => {
                    println!("Client connected {}", addr);
                    let tx = self.client_event_tx.clone();
                    tokio::spawn(async move {
                        handle_client(stream, tx).await
                    });
                }
                // Tick the frame
                _ = frame_interval.tick() => {
                    // Drain events from the channel
                    while let Ok(cmd) = self.client_event_rx.try_recv() {
                        client_command_queue.push_back(cmd);
                    }

                    // Process queued client commands
                    while let Some(cmd) = client_command_queue.pop_front() {
                        println!("Received command: {:?}", cmd);
                        // TODO: reply to client
                    }
                }
            }
        }
    }
}
