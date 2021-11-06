use std::fs::read_to_string;

use anathema::{Attribute, Colors, Input, Instruction, Lines, Pos, Size, Sub, Window};
use anyhow::Result;
use unicode_width::UnicodeWidthStr;

use super::models::DisplayMessage;
use super::random_color;

const BORDER_1: &str =
    "----------------------------------------------------------------------------------------------------";
// const BORDER_2: &str =
//     "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=--=-=-=-=-=-=-=-=-=-=-=-";


fn empty_chat() -> String {
    read_to_string("default_chat.txt").unwrap_or(String::new())
}

pub struct ChatDisplay {
    messages: Vec<DisplayMessage>,
    offset: usize,
    dirty: bool,
    window: Window<Sub>,
    default_text: String,
}

impl ChatDisplay {
    pub fn new(window: Window<Sub>) -> Self {
        Self { messages: Vec::with_capacity(500), offset: 0, dirty: true, window, default_text: empty_chat() }
    }

    pub fn touch(&mut self) {
        self.dirty = true;
    }

    pub fn handle(&mut self, msg: &DisplayMessage) {
        match msg {
            DisplayMessage::Chat(_) 
            | DisplayMessage::ChatEvent(_) 
            | DisplayMessage::Quote(..) => {
                self.messages.push(msg.clone());
                self.dirty = true;
            }
            DisplayMessage::ClearChat => {
                self.messages.clear();
                self.dirty = true;
            }
            _ => {}
        }
    }

    pub fn update(&mut self, colors: &mut Colors) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }

        self.dirty = false;

        let mut lines = Lines::new(self.window.size().width as usize);
        for msg in &self.messages {
            match msg {
                DisplayMessage::Chat(msg) => {
                    lines.reset_color();
                    lines.reset_style();
                    if let Ok(col) = Colors::init_fg(crate::display::GREY) {
                        lines.color(col);
                    }

                    lines.push_str(&msg.timestamp, true);
                    lines.pad(1);

                    if let Some(ref col) = msg.color {
                        let res = colors.from_hex(col).and_then(Colors::init_fg);
                        if let Ok(col) = res {
                            lines.color(col);
                        }
                    }

                    if msg.action {
                        lines.style(Attribute::Italic);
                    }

                    lines.push_str(&msg.nick, true);
                    if !msg.action {
                        lines.reset_color();
                    }

                    lines.pad(1);

                    lines.push_str(&msg.message, true);
                    if msg.action {
                        lines.reset_style();
                    }
                    lines.force_new_line();
                }
                DisplayMessage::ChatEvent(ev) => {
                    lines.reset_style();
                    let color = random_color();

                    if let Ok(col) = Colors::init_fg(color) {
                        lines.color(col);
                    }

                    let width = self.window.size().width as usize;
                    let padding = (width / 2).saturating_sub(ev.0.width() / 2);

                    let border = BORDER_1;
                    lines.push_str(&border[..width.min(border.len())], true);
                    lines.force_new_line();
                    lines.pad(padding);
                    lines.push_str(&ev.0, true);
                    lines.force_new_line();
                    lines.push_str(&border[..width.min(border.len())], true);
                }
                DisplayMessage::Quote(quote, color) => {
                    log::info!("{}", quote);
                    lines.reset_style();

                    if let Ok(col) = Colors::init_fg(*color) {
                        lines.color(col);
                    }

                    lines.style(Attribute::Italic);
                    lines.push_str("> ", true);
                    lines.push_str(&quote, true);
                    lines.force_new_line();
                }
                DisplayMessage::Follow(_, _)
                | DisplayMessage::ClearChat
                | DisplayMessage::Sub(_, _)
                | DisplayMessage::TodoUpdate(_)
                | DisplayMessage::ChannelPoints(_) => {}
            };
        }

        // Fix offset
        let height = self.window.size().height as usize;
        let max = (lines.len().max(height) - height);
        self.offset = self.offset.min(max);

        // Draw a default if lines are empty:
        if lines.is_empty() {
            for l in self.default_text.lines() {
                lines.push_str(l, true);
                lines.force_new_line();
            }
        }

        self.window.erase()?;
        super::render_lines(lines, &self.window, self.offset)?;
        self.window.refresh()?;

        Ok(())
    }

    pub fn resize(&mut self, size: Size) -> Result<()> {
        self.window.resize(size)?;
        self.dirty = true;
        Ok(())
    }

    pub fn move_win(&mut self, pos: Pos) -> Result<()> {
        self.window.move_win(pos)?;
        self.dirty = true;
        Ok(())
    }

    pub fn input(&mut self, input: Input) -> Result<()> {
        match input {
            Input::Character('k') => {
                // up
                self.offset += 1;
                self.dirty = true;
            }
            Input::Character('j') => {
                if self.offset == 0 {
                    return Ok(());
                }

                // down
                self.offset -= 1;
                self.dirty = true;
            }
            _ => {}
        }

        Ok(())
    }
}
