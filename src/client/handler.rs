use crate::client::commands::ClientCommand;
use crate::client::ClientEvent;
use crate::resp::{RespParser, RespValue};
use crate::server::ServerCommand;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

pub async fn handle_client(stream: TcpStream, client_event_tx: UnboundedSender<ClientEvent>) {
    // Create a channel for the server to respond to client events
    let (server_command_tx, mut server_command_rx) = unbounded_channel::<ServerCommand>();
    let (mut reader, mut writer) = stream.into_split();

    // Start the background writer (send) loop
    tokio::spawn(async move {
        while let Some(command) = server_command_rx.recv().await {
            let resp: RespValue = command.into();
            let output = resp.to_string();

            if let Err(err) = writer.write_all(&output.as_bytes()).await {
                eprintln!("Failed to send server command: {}", err);
                break;
            }
        }
    });

    let mut parser = RespParser::new();
    let mut recv_buffer = [0; 4096];

    // Reader (recv) loop
    loop {
        match reader.read(&mut recv_buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                parser.append(&recv_buffer[..n]);
                while let Some(resp) = parser.parse() {
                    match ClientCommand::try_from(resp) {
                        Ok(command) => {
                            let event = ClientEvent::new(command, server_command_tx.clone());
                            client_event_tx.send(event).unwrap();
                        }
                        Err(e) => println!("Client command error: {:?}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Client error: {:?}", e);
                break;
            }
        }
    }
}
