use std::collections::VecDeque;

use rodio::OutputStreamHandle;
use anathema::{Input, Line, Pos, ScrollBuffer, Sub, Window, Size, Colors};
use anyhow::Result;
use rand::prelude::*;

use super::animation::{FrameAnim, CharAnim};
use super::models::DisplayMessage;
use super::{DisplayContext, DisplayHandler};
use crate::sound_player::SoundPlayer;


fn random_sub() -> String {
    let sounds = (1..=14)
        .map(|id| format!("/home/togglebit/projects/stream/sounds/sub{}.mp3", id))
        .collect::<Vec<String>>();

    let mut rng = thread_rng();
    sounds.choose(&mut rng).unwrap().to_owned()
}

pub struct FullscreenDisplay {
    queue: VecDeque<(FrameAnim, CharAnim)>,
    current: Option<(FrameAnim, CharAnim)>,
    sound_player: Option<SoundPlayer>,
    output_handle: OutputStreamHandle,
}

impl FullscreenDisplay {
    pub fn new(output_handle: OutputStreamHandle) -> Self {
        Self { queue: VecDeque::with_capacity(100), current: None, sound_player: None, output_handle }
    }

    pub fn wants_update(&self) -> bool {
        !self.queue.is_empty() || self.current.is_some()
    }

    fn next_frame(&mut self, window: &Window<Sub>, buffer: &mut ScrollBuffer<Line>) -> Result<()> {
        match &mut self.current {
            Some((frame, text)) => {
                let mut lines = frame.update();
                // let chars = text.update();

                // // Draw the text first.
                // // TODO: inc all X with 1 because of the border
                // for c in chars {
                //     let color_id: i16 = c.color.into();
                //     let pair = Colors::get_color_pair(color_id as u32);
                //     window.set_color(pair)?;
                //     window.add_char_at(c.current_pos, c.c)?;
                // }

                // let reset = Colors::get_color_pair(0);
                // window.set_color(reset)?;

                // ... Then the frame anim
                // Position the animation at the bottom of the window
                let y = window.size().height - lines.len() as i32;
                window.move_cursor(Pos::new(0, y - 1))?;

                buffer.clear();
                for line in lines.drain() {
                    buffer.push(line);
                }

                if frame.is_done {
                    self.current.take();
                }
            }
            None => match self.queue.pop_front() {
                Some(next_anim) => {
                    self.current = Some(next_anim);
                    // if let Some(ref mut player) = self.sound_player {
                    //     player.play(1.0);
                    // }
                }
                None => {}
            },
        }

        Ok(())
    }
}

impl DisplayHandler for FullscreenDisplay {
    fn update(&mut self, context: DisplayContext) -> Result<()> {
        context.window.erase()?;
        // context.window.draw_box();
        self.next_frame(context.window, context.buffer)?;
        Ok(())
    }

    fn input(&mut self, _: DisplayContext, _: Input) -> Result<()> {
        Ok(())
    }

    fn rebuild(&mut self, context: DisplayContext) -> Result<()> {
        Ok(())
    }

    fn handle(&mut self, context: DisplayContext, msg: &DisplayMessage) {
        let sub = match msg {
            DisplayMessage::Sub(sub) => sub,
            DisplayMessage::ChannelPoints(_)
            | DisplayMessage::Chat(_)
            | DisplayMessage::TodoUpdate(_)
            | DisplayMessage::ChatEvent(_)
            | DisplayMessage::ClearChat => return,
        };

        let sound = random_sub();
        self.sound_player = Some(SoundPlayer::new(sound, self.output_handle.clone()));
        let width = context.window.size().width - 2;
        let height = 4;
        let animation = FrameAnim::new("animations/bender.txt", width as usize);
        let char_anim = CharAnim::new("Not implemented yet", Size::new(width, height));
        self.queue.push_back((animation, char_anim));
    }
}
