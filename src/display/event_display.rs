use tinybit::ScreenSize;

use crate::twitch::Twitch;

pub struct EventDisplay {
}

impl EventDisplay {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn push_event(&mut self, event: Twitch) {
    }

    pub fn resize(&mut self, size: ScreenSize) {
    }
}
