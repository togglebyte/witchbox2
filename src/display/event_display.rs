use std::collections::VecDeque;

use rodio::OutputStreamHandle;
use anathema::{Colors, Color, Input, Pos, Sub, Window, Line, Lines};
use anyhow::Result;
use rand::prelude::*;

use super::animation::CharAnim;
use super::models::DisplayMessage;
use super::{DisplayContext, DisplayHandler};

fn random_arch() -> String {
    let sounds = (1..=13)
        .map(|id| format!("/home/togglebit/projects/stream/misc/arch{}.mp3", id))
        .collect::<Vec<String>>();

    let mut rng = thread_rng();
    sounds.choose(&mut rng).unwrap().to_owned()
}

pub struct EventDisplay {
    // Option should contain a sound
    queue: VecDeque<(CharAnim, Option<()>)>,
    current: Option<CharAnim>,
    default: Vec<String>,
}

impl EventDisplay {
    pub fn new(output_handle: OutputStreamHandle) -> Self {
        let default = vec![
            "Step 1".to_string(),
            "Step two".into(),
            "msfklsdafkj askjlsd ".into()
        ];
        Self { queue: VecDeque::with_capacity(100), current: None, default: Vec::new() }
    }

    pub fn wants_update(&self) -> bool {
        !self.queue.is_empty() || self.current.is_some()
    }

    fn next_frame(&mut self, window: &Window<Sub>) -> Result<()> {
        match self.current {
            Some(ref mut current) => {
                let chars = current.update();

                for c in chars {
                    let color_id: i16 = c.color.into();
                    let pair = Colors::get_color_pair(color_id as u32);
                    window.set_color(pair)?;
                    window.add_char_at(c.current_pos, c.c)?;
                }

                let reset = Colors::get_color_pair(0);
                window.set_color(reset)?;

                if current.is_done {
                    self.current.take();
                }
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
            DisplayMessage::Chat(_) 
            | DisplayMessage::ChatEvent(_) 
            | DisplayMessage::ClearChat 
            | DisplayMessage::Sub(_) => return,
        };

        let animation = CharAnim::new(&format!("{}: {}", points_event.user, points_event.title), context.window.size());
        self.queue.push_back((animation, None));
    }

    fn update(&mut self, context: DisplayContext) -> Result<()> {
        context.window.erase()?;
        let size = context.window.size();
        context.window.horizontal_line_at(Pos::zero(), '=', size.width)?;
        context.window.horizontal_line_at(Pos::new(0, size.height - 1), '=', size.width)?;

        let blue: i16 = Color::Blue.into();
        let blue = Colors::get_color_pair(blue as u32);
        context.window.set_color(blue)?;
        context.window.print_at(Pos::new(2, 0), " Witchbox 2 ")?;
        let reset = Colors::get_color_pair(0);
        context.window.set_color(reset)?;
        self.next_frame(context.window)?;

        // If nothing is playing, show some defaults here,
        // make sure not to keep rebuilding the defaults thought,
        // as that would be silly
        if self.current.is_none() {
            let mut lines = Lines::new(context.window.size().width as usize);
            for line in self.default.iter() {
                lines.push_str(&line);
            }

            for line in lines.complete() {
                context.buffer.push(line);
            }
        }

        Ok(())
    }
}
