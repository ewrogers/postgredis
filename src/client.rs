use crate::commands::ClientCommand;
use crate::resp::RespParser;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedSender;

pub async fn handle_client(mut stream: TcpStream, _event_tx: UnboundedSender<ClientCommand>) {
    let mut parser = RespParser::new();
    let mut recv_buffer = [0; 4096];

    loop {
        match stream.read(&mut recv_buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                parser.append(&recv_buffer[..n]);
                while let Some(resp) = parser.parse() {
                    println!("Client message: \"{:?}\"", resp);
                    // TODO: map resp value to command (or error)
                }
            }
            Err(e) => {
                eprintln!("Client error: {:?}", e);
                break;
            }
        }
    }
}
