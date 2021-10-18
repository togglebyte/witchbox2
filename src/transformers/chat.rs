use neotwitch::IrcMessage;
use crate::display::models::ChatMessage;

pub struct IrcTransformer {
}

impl IrcTransformer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn transform(&mut self, mut message: IrcMessage) -> ChatMessage {
        ChatMessage {
            nick: message.nick,
            color: message.tags.remove("color"),
            message: message.message
        }
    }
}
