use std::collections::VecDeque;

use anathema::{Colors, Input, Line, Pos, ScrollBuffer, Sub, Window};
use anyhow::Result;

use super::animation::FrameAnim;
use super::models::DisplayMessage;
use super::{DisplayContext, DisplayHandler};

pub struct FullscreenDisplay {
    queue: VecDeque<FrameAnim>,
    current: Option<FrameAnim>,
}

impl FullscreenDisplay {
    pub fn new() -> Self {
        Self { queue: VecDeque::with_capacity(100), current: None }
    }

    pub fn wants_update(&self) -> bool {
        !self.queue.is_empty() || self.current.is_some()
    }

    fn next_frame(&mut self, window: &Window<Sub>, buffer: &mut ScrollBuffer<Line>) -> Result<()> {
        match self.current {
            Some(ref mut current) => {
                let lines = current.update();

                // Position the animation at the bottom of the window
                let y = window.size().height - lines.len() as i32;
                window.move_cursor(Pos::new(0, y - 1))?;

                buffer.clear();
                for line in lines {
                    buffer.push(line);
                }

                if current.is_done {
                    self.current.take();
                }
            }
            None => match self.queue.pop_front() {
                Some(next_anim) => self.current = Some(next_anim),
                None => {}
            }
        }

        Ok(())
    }
}

impl DisplayHandler for FullscreenDisplay {
    fn update(&mut self, context: DisplayContext) -> Result<()> {
        context.window.erase();
        context.window.draw_box();
        self.next_frame(context.window, context.buffer)?;
        Ok(())
    }

    fn input(&mut self, context: DisplayContext, input: Input) -> Result<()> {
        match input {
            _ => {}
        }
        Ok(())
    }

    fn handle(&mut self, context: DisplayContext, msg: &DisplayMessage) {
        let sub = match msg {
            DisplayMessage::Sub(sub) => sub,
            DisplayMessage::ChannelPoints(_) | DisplayMessage::Chat(_) | DisplayMessage::ClearChat => return,
        };

        let animation = FrameAnim::new("animations/bender.txt", context.window.size().width as usize - 2);
        self.queue.push_back(animation);
        if self.current.is_none() {
            self.current = self.queue.pop_front();
        }
    }
}
