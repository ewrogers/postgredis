use crate::client::handle_client;
use crate::client::ClientCommand;
use std::collections::VecDeque;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

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

                // Receive the next command from the client queue
                Some(resp) = self.client_event_rx.recv() => {
                    match ClientCommand::try_from(resp) {
                        Ok(command) => client_command_queue.push_back(command),
                        Err(err) => {
                            eprintln!("Client error: {}", err);
                        }
                    }
                }
            }

            // Process the event queue
            while let Some(command) = client_command_queue.pop_front() {
                println!("Command: {:?}", command);
            }
        }
    }
}
