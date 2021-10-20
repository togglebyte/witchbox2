use std::collections::VecDeque;

use anathema::{Colors, Input, Pos, Sub, Window};
use anyhow::Result;

use super::animation::CharAnim;
use super::models::DisplayMessage;
use super::{DisplayContext, DisplayHandler, DisplayState};

pub struct EventDisplay {
    // Option should contain a sound
    queue: VecDeque<(CharAnim, Option<()>)>,
    current: Option<CharAnim>,
}

impl EventDisplay {
    pub fn new() -> Self {
        Self { queue: VecDeque::with_capacity(100), current: None }
    }

    pub fn wants_update(&self) -> bool {
        !self.queue.is_empty() || self.current.is_some()
    }

    fn next_frame(&mut self, window: &Window<Sub>) -> Result<()> {
        match self.current {
            Some(ref mut current) => {
                let chars = current.update();
                if current.is_done {
                    self.current.take();
                }

                for c in chars {
                    let color_id: i16 = c.color.into();
                    let pair = Colors::get_color_pair(color_id as u32);
                    window.set_color(pair)?;
                    window.add_char_at(c.current_pos, c.c)?;
                }

                let reset = Colors::get_color_pair(0);
                window.set_color(reset)?;
            }
            None => {
                if let Some((next_anim, audio)) = self.queue.pop_front() {
                    self.current = Some(next_anim);
                    // play audio
                } else {
                    // Change state
                }
            }
        }

        Ok(())
    }
}

impl DisplayHandler for EventDisplay {
    fn input(&mut self, _: DisplayContext, _: Input) -> Result<()> {
        Ok(())
    }

    fn handle(&mut self, context: DisplayContext, msg: &DisplayMessage) {
        let points_event = match msg {
            DisplayMessage::ChannelPoints(points_event) => points_event,
            DisplayMessage::Chat(_) | DisplayMessage::ClearChat | DisplayMessage::Sub(_) => return,
        };

        let animation = CharAnim::new(&format!("{}: {}", points_event.user, points_event.title), context.window.size());
        self.queue.push_back((animation, None));
    }

    fn update(&mut self, context: DisplayContext) -> Result<()> {
        context.window.erase()?;
        let size = context.window.size();
        context.window.horizontal_line_at(Pos::zero(), '=', size.width);
        context.window.horizontal_line_at(Pos::new(0, size.height - 1), '=', size.width);
        self.next_frame(context.window)?;
        Ok(())
    }
}
