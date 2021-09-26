use std::collections::VecDeque;

use chrono::prelude::*;
use tinybit::widgets::Text;
use tinybit::{Color, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport};

use super::lines;

enum Line {
    Start(Text, Text, Text),
    Cont(Text),
}

fn colour_from_nick(nick: &str) -> Color {
    Color::Green
}

fn message_to_widget(timestamp: DateTime<Local>, nick: &str, message: &str, action: bool, max_width: usize) -> Vec<Line> {
    let nick = match action {
        true => format!("{} ", nick),
        false => format!("{}> ", nick),
    };

    let (nick_fg, msg_fg) = match action {
        true => (Some(Color::Red), Some(Color::Red)),
        false => (Some(colour_from_nick(&nick)), None)
    };

    let offset = nick.len();
    let mut message_lines = lines(message, max_width);

    let first_line = message_lines.remove(0);

    let first_line = Line::Start(
        Text::new(&timestamp.format("%H:%M:%S : ").to_string(), None, None),
        Text::new(&nick, nick_fg, None),
        Text::new(first_line, msg_fg, None)
    );

    let mut lines = vec![first_line];
    for message in message_lines {
        lines.push(Line::Cont(Text::new(message, msg_fg, None)));
    }

    lines
}

pub struct Chat {
    messages: VecDeque<(DateTime<Local>, String, String, bool)>,
    widgets: Vec<Line>,
    viewport: Viewport,
    scroll_offset: isize,
}

impl Chat {
    pub fn new(size: ScreenSize) -> Self {
        let mut messages = VecDeque::new();

        let msg = "hello this is a longer name than expected and some more chars here and bluh bleh blah blop bop plop blarp lark lork flerp florp fiddlestick and boring bo bo tricks and I ran out of i".to_string();
        // let msg = "hello this is a longer name than expected".to_string();
        for i in 0..70 {
            messages.push_back((Local::now(), format!("User-{}", i), msg.clone(), i % 5 == 0));
        }

        Self {
            messages,
            widgets: Vec::new(),
            viewport: Viewport::new(ScreenPos::zero(), size),
            scroll_offset: 0,
        }
    }

    pub fn rebuild_widgets(&mut self) {
        // Clear all widgets and rebuild the widget this.
        // This is just me being lazy
        self.widgets.clear();

        // Build new widgets
        for message in &self.messages {
            let mut widgets = message_to_widget(
                message.0, 
                &message.1,
                &message.2,
                message.3, 
                self.viewport.size.width as usize
            );
            self.widgets.append(&mut widgets);
        }

        // Draw the widgets onto the viewport
        let offset = self.widgets.len() - self.max_lines() - self.scroll_offset as usize;
        let mut y = 0;
        for widget in self.widgets.iter().skip(offset) {
            match widget {
                Line::Start(timestamp, nick, msg) => {
                    self.viewport.draw_widget(timestamp, ScreenPos::new(0, y as u16));
                    let mut offset = timestamp.0.len() as u16;
                    self.viewport.draw_widget(nick, ScreenPos::new(offset, y as u16));
                    offset += nick.0.len() as u16;
                    self.viewport.draw_widget(msg, ScreenPos::new(offset, y as u16));
                }
                Line::Cont(msg) => {
                    self.viewport.draw_widget(msg, ScreenPos::new(0, y as u16));
                }
            }
            y += 1;
        }
    }

    pub fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn scroll(&mut self, up: bool, amount: isize) {
        match up {
            true => {
                let max = self.widgets.len() as isize - self.max_lines() as isize;
                let amount = (max - self.scroll_offset).min(amount);
                self.scroll_offset += amount
            }
            false => {
                if self.scroll_offset - amount < 0 {
                    self.scroll_offset = 0;
                } else {
                    self.scroll_offset -= amount;
                }
            }
        }
    }

    pub fn max_lines(&self) -> usize {
        self.viewport.size.height as usize
    }

    pub fn new_message(&mut self, nick: String, msg: String, action: bool) {
        let now = Local::now();
        self.messages.push_back((now, nick, msg, action));
        let mut offest_offset = 0;
        while self.messages.len() > self.max_lines() {
            self.messages.pop_front();
            offest_offset += 1;
        }

        self.scroll_offset -= offest_offset;

        self.rebuild_widgets();
    }
}

impl super::View for Chat {
    fn draw(&mut self, renderer: &mut Renderer<StdoutTarget>) {
        renderer.render(&mut self.viewport);
        self.viewport.swap_buffers();
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.viewport.resize(width, height);
    }
}
