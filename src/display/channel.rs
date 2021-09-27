use std::collections::VecDeque;

use tinybit::{Pixel, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport};

use super::animation::{Anim, Animation};
use crate::twitch::Twitch;

pub struct ChannelEvents {
    viewport: Viewport,
    animation_queue: VecDeque<(Anim, Twitch)>,
    current: Option<(Anim, Twitch)>,
    triggers: crate::events::triggers::Triggers,
}

impl ChannelEvents {
    fn clear_viewport(&mut self) {
        for x in 0..self.viewport.size.width {
            for y in 0..self.viewport.size.height {
                self.viewport.draw_pixel(Pixel::new(
                    ' ',
                    ScreenPos::new(x, y),
                    None,
                    None,
                ));
            }
        }
    }

    pub fn new(size: ScreenSize) -> Self {
        let triggers = crate::events::triggers::Triggers::new();
        let size = ScreenSize::new(size.width, size.height / 2);
        let viewport = Viewport::new(ScreenPos::zero(), size);
        Self {
            viewport,
            animation_queue: VecDeque::new(),
            triggers,
            current: None,
        }
    }

    pub fn event(&mut self, twitch: Twitch) {
        self.clear_viewport();
        let input = self.triggers.display(&twitch);
        let anim = Anim::new(input, &mut self.viewport);
        self.animation_queue.push_back((anim, twitch));
    }

    pub fn animating(&self) -> bool {
        self.current.is_some() || !self.animation_queue.is_empty()
    }
}

impl super::View for ChannelEvents {
    fn draw(&mut self, renderer: &mut Renderer<StdoutTarget>) {
        if self.current.is_none() {
            match self.animation_queue.pop_front() {
                Some((anim, twitch)) => {
                    self.triggers.trigger(&twitch);
                    self.current = Some((anim, twitch));
                }
                None => return,
            }
        }

        match self.current.as_mut() {
            None => return,
            Some((anim, twitch)) => {
                let is_done = anim.update(renderer, &mut self.viewport);
                if is_done {
                    self.current = None;
                }
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.viewport.resize(width, height);
    }
}
