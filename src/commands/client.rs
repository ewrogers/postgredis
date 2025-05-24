#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientCommand {
    Ping(Option<String>),
}
