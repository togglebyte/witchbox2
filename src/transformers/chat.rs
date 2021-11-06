use neotwitch::IrcMessage;
use crate::display::models::{DisplayMessage, ChatEvent, ChatMessage};

pub struct IrcTransformer {
}

impl IrcTransformer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn transform(&mut self, mut message: IrcMessage) -> DisplayMessage {
        DisplayMessage::Chat(ChatMessage::from(message))
        //     DisplayMessage::Chat(ChatMessage {
        //         timestamp: message.timestamp.format("%H:%M:%S").to_string(),
        //         nick: message.nick,
        //         color: message.tags.remove("color"),
        //         message: message.message,
        //         action: 
        //     }),
        // }
    }
}
