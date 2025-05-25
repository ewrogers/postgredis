use crate::client::commands::ClientCommand;
use crate::client::ClientEvent;
use crate::resp::RespParser;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

pub async fn handle_client(mut stream: TcpStream, client_event_tx: UnboundedSender<ClientEvent>) {
    let mut parser = RespParser::new();
    let mut recv_buffer = [0; 4096];

    let (tx, rx) = unbounded_channel();

    loop {
        match stream.read(&mut recv_buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                parser.append(&recv_buffer[..n]);
                while let Some(resp) = parser.parse() {
                    match ClientCommand::try_from(resp) {
                        Ok(cmd) => {
                            let event = ClientEvent::new(cmd, tx.clone());
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
