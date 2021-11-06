use std::collections::VecDeque;
use std::fmt::Write;

use anathema::{Colors, Input, Line, Lines, Pos, ScrollBuffer, Size, Sub, Window};
use anyhow::Result;
use rand::prelude::*;
use rodio::OutputStreamHandle;

use super::animation::{CharAnim, FrameAnim, Animation};
use super::models::{DisplayMessage, Subscription};
use crate::audio::SoundPlayer;

pub struct FullscreenDisplay {
    queue: VecDeque<(String, FrameAnim, CharAnim, String)>,
    current: Option<(FrameAnim, CharAnim)>,
    sound_player: Option<SoundPlayer>,
    output_handle: OutputStreamHandle,
    window: Window<Sub>,
}

impl FullscreenDisplay {
    pub fn new(window: Window<Sub>, output_handle: OutputStreamHandle) -> Self {
        Self { queue: VecDeque::with_capacity(100), current: None, sound_player: None, output_handle, window }
    }

    pub fn wants_update(&self) -> bool {
        !self.queue.is_empty() || self.current.is_some()
    }

    fn next_frame(&mut self) -> Result<()> {
        match &mut self.current {
            Some((frame, text)) => {
                text.draw(&mut self.window)?;

                // ... Then the frame anim
                let mut lines = frame.update();
                // Position the animation at the bottom of the window
                let y = self.window.size().height - lines.len() as i32;
                self.window.move_cursor(Pos::new(0, y - 1))?;
                super::render_lines(lines, &self.window, 0)?;

                if frame.is_done && text.is_done {
                    self.current.take();
                }
            }
            None => match self.queue.pop_front() {
                Some(next_anim) => {
                    let (message, anim, text, sound_path) = next_anim;
                    self.current = Some((anim, text));
                    let mut player = SoundPlayer::new(sound_path, self.output_handle.clone());
                    player.play(1.0);
                    self.sound_player = Some(player);
                }
                None => {}
            },
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.window.erase()?;
        self.next_frame()?;
        self.window.refresh();
        Ok(())
    }

    pub fn resize(&mut self, size: Size) -> Result<()> {
        // Cancel any ongoing animations
        // before resizing the window
        let _ = self.current.take();

        self.window.resize(size)?;
        Ok(())
    }

    pub fn handle(&mut self, msg: &DisplayMessage) {
        let (sub, sound_path) = match msg {
            DisplayMessage::Sub(sub, sound_path) => (sub, sound_path),
            DisplayMessage::ChannelPoints(_)
            | DisplayMessage::Chat(_)
            | DisplayMessage::Follow(_, _)
            | DisplayMessage::TodoUpdate(_)
            | DisplayMessage::ChatEvent(_)
            | DisplayMessage::ClearChat => return,
        };

        let width = self.window.size().width;
        let animation = FrameAnim::new("animations/bender.txt", width as usize);

        let height = self.window.size().height;
        let text_anim_height = height - animation.height as i32; 
        let message = sub_to_message(&sub, text_anim_height as usize);
        let char_anim = CharAnim::new(&message, Size::new(width, text_anim_height), Animation::Scatter);
        self.queue.push_back((message, animation, char_anim, sound_path.clone()));
    }
}

fn sub_to_message(sub: &Subscription, max_lines: usize) -> String {
    let mut s = String::new();

    if sub.gift {
        write!(&mut s, "{} gifted ", sub.gifter.as_deref().unwrap_or("[Anonymous]"));
        if sub.recipients.len() == 1 {
            write!(&mut s, "a sub to {}!", sub.recipients.first().unwrap());
        } else {
            write!(&mut s, "{} subs to \n", sub.recipients.len());
            sub.recipients.iter().take(max_lines.saturating_sub(5)).for_each(|r| { write!(&mut s, "{}\n", r); });

            if sub.recipients.len() > max_lines.saturating_sub(5) {
                write!(&mut s, "... And many more!!!!");
            }

        }
    }

    s
}
