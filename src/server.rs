use crate::client::handle_client;
use crate::commands::Command;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::time::interval;

pub struct Server {
    listener: TcpListener,
    event_tx: UnboundedSender<Command>,
    event_rx: UnboundedReceiver<Command>,
}

impl Server {
    pub async fn new(addr: SocketAddr) -> Self {
        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind to address");

        let (tx, rx) = unbounded_channel();
        Server {
            listener,
            event_tx: tx,
            event_rx: rx,
        }
    }

    pub async fn run(&mut self) {
        // Frame interval for 120 FPS
        let mut frame_interval = interval(Duration::from_micros(1_000_000 / 120));
        let mut event_queue: VecDeque<Command> = VecDeque::new();

        loop {
            tokio::select! {
                // Accept new client connections
                Ok((stream, addr)) = self.listener.accept() => {
                    println!("Client connected {}", addr);
                    let tx = self.event_tx.clone();
                    tokio::spawn(async move {
                        handle_client(stream, tx).await
                    });
                }
                // Tick the frame
                _ = frame_interval.tick() => {
                    // Drain events from the channel
                    while let Ok(cmd) = self.event_rx.try_recv() {
                        event_queue.push_back(cmd);
                    }

                    // Process queued commands
                    while let Some(cmd) = event_queue.pop_front() {
                        println!("Received command: {:?}", cmd);
                        // TODO: reply to client
                    }
                }
            }
        }
    }
}
