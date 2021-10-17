use crate::Event;

pub struct Filter;

impl Filter {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn filter(&mut self, event: crate::Event) -> Option<crate::Event> {
        match event {
            Event::Chat(irc_msg) if irc_msg.message.starts_with("!") => None,
            _ => Some(event)
        }
    }
}
