use std::collections::VecDeque;

use anyhow::Result;
use anathema::{Input, Colors, Color, Sub, Window};

use super::{DisplayHandler, DisplayContext};
use super::models::DisplayMessage;
use super::animation::Animation;

pub struct EventDisplay {
    // Option should contain a sound
    queue: VecDeque<(Animation, Option<()>)>,
    current: Option<Animation>,
}

impl EventDisplay {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::with_capacity(100),
            current: None,
        }
    }

    fn next_frame(&mut self, window: &Window<Sub>) {
        match self.current {
            Some(ref mut c) => {
                let chars = c.update();
                if c.is_done() {
                    self.current.take();
                }

                for c in chars {
                    let color_id: i16 = c.color.into();
                    let pair = Colors::get_color_pair(color_id as u32);
                    window.set_color(pair);
                    window.add_char_at(c.current_pos, c.c);
                }

                let reset = Colors::get_color_pair(0);
                window.set_color(reset);
            }
            None => {
                if let Some((next_anim, audio)) = self.queue.pop_front() {
                    self.current = Some(next_anim);
                    // play audio
                }
            }
        }
    }
}

impl DisplayHandler for EventDisplay {
    fn input(&mut self, context: DisplayContext, input: Input) -> Result<()> {
        Ok(())
    }

    fn handle(&mut self, context: DisplayContext, msg: &DisplayMessage) {
        let points_event = match msg {
            DisplayMessage::ChannelPoints(points_event) => points_event,
            DisplayMessage::Chat(_) | DisplayMessage::ClearChat => return,
        };

        let animation = Animation::char_anim("hello world", context.window.size());
        self.queue.push_back((animation, None));
    }

    fn update(&mut self, context: DisplayContext) {
        context.window.erase();
        let pair = Colors::get_color_pair(2);
        context.window.set_color(pair);
        context.window.draw_box();
        self.next_frame(context.window);
    }
}
