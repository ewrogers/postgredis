#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerCommand {
    Pong(Option<String>),
}
