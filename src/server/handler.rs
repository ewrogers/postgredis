use crate::client::{ClientCommand, ClientEvent};
use crate::server::ServerCommand;

pub fn handle_client_event(event: &ClientEvent) {
    let tx = &event.responder;

    let response: Option<ServerCommand> = match &event.command {
        ClientCommand::Ping(message) => Some(ServerCommand::Pong(message.clone())),
        _ => None,
    };

    if let Some(command) = response {
        tx.send(command).unwrap();
    }
}
