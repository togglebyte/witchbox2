use std::collections::VecDeque;
use std::fmt::Write;

use anathema::{Color, Colors, Input, Line, Lines, Pos, ScrollBuffer, Size, Sub, Window};
use anyhow::Result;
use rodio::OutputStreamHandle;

use super::animation::{Animation, CharAnim};
use super::models::DisplayMessage;
use crate::audio::SoundPlayer;

pub struct EventDisplay {
    window: Window<Sub>,
    inner_win: Window<Sub>,
    queue: VecDeque<(CharAnim, Option<String>)>,
    current: Option<CharAnim>,
    todo: Option<String>,
    sound_player: Option<SoundPlayer>,
    output_handle: OutputStreamHandle,
    dirty: bool,
}

impl EventDisplay {
    pub fn new(window: Window<Sub>, output_handle: OutputStreamHandle, todo: Option<String>) -> Result<Self> {
        let pos = Pos::new(1, 1);
        let parent_size = window.size();
        let size = Size::new(parent_size.width - 2, parent_size.height - 2);
        let inner_win = window.new_window(pos, size)?;
        let inst = Self {
            window,
            inner_win,
            queue: VecDeque::with_capacity(100),
            current: None,
            todo,
            sound_player: None,
            output_handle,
            dirty: true,
        };
        Ok(inst)
    }

    fn wants_update(&self) -> bool {
        !self.queue.is_empty() || self.current.is_some()
    }

    fn show_todo(&self) -> Result<()> {
        let todo = match self.todo.as_ref() {
            Some(t) => t,
            None => return Ok(()),
        };

        self.inner_win.erase();

        let mut lines = Lines::new(self.inner_win.size().width as usize);
        lines.push_str(&todo, true);
        super::render_lines(lines, &self.inner_win, 0)?;

        Ok(())
    }

    fn next_frame(&mut self) -> Result<()> {
        // mark it as dirty so we re-draw the todo if there is one
        self.dirty = true;
        match self.current {
            Some(ref mut current) => {
                self.inner_win.erase()?;
                let chars = current.update();

                for c in chars {
                    if !self.inner_win.contains(c.current_pos) {
                        continue;
                    }
                    let color_id: i16 = c.color.into();
                    let pair = Colors::get_color_pair(color_id as u32);
                    self.inner_win.set_color(pair)?;
                    self.inner_win.add_char_at(c.current_pos, c.c)?;
                }

                let reset = Colors::get_color_pair(0);
                self.inner_win.set_color(reset)?;

                if current.is_done {
                    self.current.take();
                }
            }
            None => {
                if let Some((next_anim, audio)) = self.queue.pop_front() {
                    self.current = Some(next_anim);
                    if let Some(sound) = audio {
                        let mut player = SoundPlayer::new(sound, self.output_handle.clone());
                        // player.play(1.0);
                        self.sound_player = Some(player);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn handle(&mut self, msg: &DisplayMessage) {
        match msg {
            DisplayMessage::ChannelPoints(points_event) => {
                let animation = CharAnim::new(
                    &format!("{}: {}", points_event.user, points_event.title),
                    self.inner_win.size(),
                    Animation::Scatter,
                );
                self.queue.push_back((animation, points_event.sound_path.clone()));
            }
            DisplayMessage::Follow(followers, sound) => {
                let mut s = format!("Thank you for the follow ");
                if followers.len() > 1 {
                    write!(&mut s, "\n");
                }
                followers.iter().for_each(|f| {
                    write!(&mut s, "{}\n", f.0);
                });
                let anim = (followers.len() > 1).then(|| Animation::VertSlide).unwrap_or(Animation::HorzSlide);
                let animation = CharAnim::new(&s, self.inner_win.size(), anim);
                self.queue.push_back((animation, Some(sound.clone())));
            }
            DisplayMessage::TodoUpdate(new_todo) => {
                self.todo = Some(new_todo.clone());
                self.dirty = true;
            }
            DisplayMessage::Chat(_)
            | DisplayMessage::ChatEvent(_)
            | DisplayMessage::ClearChat
            | DisplayMessage::Sub(_) => return,
        };
    }

    pub fn resize(&mut self, size: Size) -> Result<()> {
        // Cancel any ongoing animations
        // before resizing the window
        let _ = self.current.take();

        self.window.resize(size)?;
        let inner_size = Size::new(size.width - 2, size.height - 2);
        self.inner_win.resize(inner_size);
        self.dirty = true;
        Ok(())
    }

    pub fn move_win(&mut self, pos: Pos) -> Result<()> {
        self.window.move_win(pos)?;
        let inner_pos = Pos::new(pos.x + 1, pos.y + 1);
        self.inner_win.move_win(inner_pos)?;
        self.dirty = true;
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        if !self.dirty && !self.wants_update() {
            return Ok(());
        }

        self.dirty = false;
        self.window.erase();
        self.inner_win.erase();

        if !self.wants_update() {
            self.show_todo()?;
        } else {
            self.next_frame()?;
        }

        self.window.draw_box();
        let blue: i16 = Color::Blue.into();
        let blue = Colors::get_color_pair(blue as u32);
        self.window.set_color(blue)?;
        self.window.print_at(Pos::new(2, 0), " Witchbox ")?;
        let reset = Colors::get_color_pair(7);
        self.window.set_color(reset)?;

        self.window.refresh()?;
        self.inner_win.refresh()?;

        Ok(())
    }
}
