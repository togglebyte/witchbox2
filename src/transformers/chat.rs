use neotwitch::IrcMessage;
use crate::display::models::{DisplayMessage, ChatMessage};

pub struct IrcTransformer {
}

impl IrcTransformer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn transform(&mut self, message: IrcMessage) -> DisplayMessage {
        DisplayMessage::Chat(ChatMessage::from(message))
    }
}
