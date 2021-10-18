use neotwitch::IrcMessage;

pub struct ChatFilter {
}

impl ChatFilter {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn filter(&mut self, message: IrcMessage) -> Option<IrcMessage> {
        Some(message)
    }
}
