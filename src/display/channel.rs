use tinybit::{Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport, Pixel};

use super::animation::{Animation, Anim};
use crate::twitch::Twitch;

pub struct ChannelEvents {
    viewport: Viewport,
    animation_queue: Vec<Anim>,
}

impl ChannelEvents {

    fn clear_viewport(&mut self) {
        for x in 0..self.viewport.size.width {
            for y in 0..self.viewport.size.height {
                self.viewport.draw_pixel(Pixel::new(' ', ScreenPos::new(x, y), None, None));
            }
        }
    }

    pub fn new(size: ScreenSize) -> Self {
        let size = ScreenSize::new(size.width, size.height / 2);
        let viewport = Viewport::new(ScreenPos::zero(), size);
        Self {
            viewport,
            animation_queue: Vec::new(),
        }
    }

    pub fn event(&mut self, twitch: Twitch) {
        self.clear_viewport();
        let animation = Animation::WriteOut;
        let input = match twitch {
            Twitch::Bits(bits) => format!("Bits..."),
            Twitch::ChannelEvent(chan) => format!("{}: {}", chan.user.display_name, chan.reward.title),
        };
        let anim = Anim::new(input.into(), animation, &mut self.viewport);
        self.animation_queue.push(anim);
    }

    pub fn animating(&self) -> bool {
        !self.animation_queue.is_empty()
    }

    pub fn animate(&mut self) {
        self.clear_viewport();
        let animation = Animation::WriteOut;
        let input = "this is a message. messages be like.. yay";
        let anim = Anim::new(input.into(), animation, &mut self.viewport);
        self.animation_queue.push(anim);
    }
}

impl super::View for ChannelEvents {
    fn draw(&mut self, renderer: &mut Renderer<StdoutTarget>) {
        if let Some(anim) = self.animation_queue.first_mut() {
            if anim.update(renderer, &mut self.viewport) {
                self.animation_queue.remove(0);
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.viewport.resize(width, height);
    }
}
