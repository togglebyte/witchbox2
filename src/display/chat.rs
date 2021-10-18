use crate::Event;
use anathema::{Colors, Input, Line, ScrollBuffer, Sub, Window};
use anyhow::Result;

use super::models::ChatMessage;
use super::draw_lines;

pub struct Chat {
    window: Window<Sub>,
    buffer: ScrollBuffer<Line>,
    colors: Colors,
}

impl Chat {
    pub fn new(window: Window<Sub>) -> Self {
        let height = window.size().height as usize;
        let colors = Colors::new();
        Self { window, buffer: ScrollBuffer::new(height, 1024), colors }
    }

    pub fn reset_window(&mut self, window: Window<Sub>) {
        self.window = window;
        self.buffer.resize(self.window.size().height as usize);
    }

    pub fn handle_message(&mut self, msg: &ChatMessage) {
        let lines = msg.to_lines(&mut self.colors, self.window.size().width as usize);
        lines
            .into_iter()
            .for_each(|line| self.buffer.push(line));
    }

    pub fn clear_chat(&mut self) {
        self.buffer.clear();
    }

    pub fn update(&mut self) {
        draw_lines(&mut self.buffer, &self.window, &mut self.colors);
    }

    pub fn input(&mut self, input: Input) -> Result<()> {
        match input {
            Input::Character('k') => {
                // up
                self.buffer.scroll_up(1);
                self.window.erase()?;
            }
            Input::Character('j') => {
                // down
                self.buffer.scroll_down(1);
                self.window.erase()?;
            }
            _ => {}
        }

        Ok(())
    }
}
