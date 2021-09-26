use neotwitch::TwitchMessage;
use tinybit::{Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport};

use super::animation::Animation;

pub struct ChannelEvents {
    viewport: Viewport,
    animation_queue: Vec<Animation>,
}

impl ChannelEvents {
    pub fn new(size: ScreenSize) -> Self {
        let viewport = Viewport::new(ScreenPos::zero(), size);
        Self {
            viewport,
            animation_queue: Vec::new(),
        }
    }

    pub fn event(&mut self, twitch: TwitchMessage) {
    }
}

impl super::View for ChannelEvents {
    fn draw(&mut self, renderer: &mut Renderer<StdoutTarget>) {
        renderer.render(&mut self.viewport);
        // self.viewport.swap_buffers();
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.viewport.resize(width, height);
    }
}
