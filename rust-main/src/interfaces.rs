use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Command {
    Shutdown = 0,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "command")]
    pub command: Option<Command>,
    #[serde(rename = "msg")]
    pub msg: Option<String>,
}
