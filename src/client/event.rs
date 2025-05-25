use crate::client::ClientCommand;
use crate::server::ServerCommand;
use tokio::sync::mpsc::UnboundedSender;

pub struct ClientEvent {
    pub command: ClientCommand,
    pub responder: UnboundedSender<ServerCommand>,
}

impl ClientEvent {
    pub fn new(command: ClientCommand, responder: UnboundedSender<ServerCommand>) -> Self {
        ClientEvent { command, responder }
    }
}
