use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::Hasher;

use chrono::prelude::*;
use neotwitch::IrcMessage;
use tinybit::widgets::{ScrollView, Text, Widget};
use tinybit::{Color, Pixel, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport};

use super::lines;

const MIN_COLOR: u8 = 60;

enum Line {
    Start(Text, Text, Text),
    Event(Text, Text),
    Cont(Text),
}

impl Widget for Line {
    fn pixels(&self) -> Vec<Pixel> {
        match self {
            Self::Start(ts, nick, msg) => {
                let mut pixels = ts.pixels();
                let mut offset = ts.size().width;
                pixels.extend(
                    nick.pixels()
                        .into_iter()
                        .map(|mut p| {
                            p.pos.x += offset;
                            p
                        })
                        .collect::<Vec<Pixel>>(),
                );
                pixels.extend(
                    nick.pixels()
                        .into_iter()
                        .map(|mut p| {
                            p.pos.x += offset;
                            p
                        })
                        .collect::<Vec<Pixel>>(),
                );
                offset += nick.size().width;
                pixels.extend(
                    msg.pixels()
                        .into_iter()
                        .map(|mut p| {
                            p.pos.x += offset;
                            p
                        })
                        .collect::<Vec<Pixel>>(),
                );
                pixels
            }
            Self::Event(ts, msg) => {
                todo!()
            }
            Self::Cont(msg) => {
                todo!()
            }
        }
    }

    fn size(&self) -> ScreenSize {
        todo!()
    }
}

fn hash_str<T: std::hash::Hash>(input: &T) -> u32 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let num = hasher.finish() as u32;
    num
}

fn colour_from_hash(name: &str) -> Color {
    let num = hash_str(&name);
    let bytes = num.to_ne_bytes();
    Color::Rgb { r: bytes[0].max(MIN_COLOR), g: bytes[1].max(MIN_COLOR), b: bytes[2].max(MIN_COLOR) }
}

fn colour_name(tags: &HashMap<String, String>, name: &str) -> Color {
    match tags.get("color") {
        Some(col) if col.len() > 6 => {
            let mut r = u8::from_str_radix(&col[1..=2], 16).unwrap_or(0);
            let mut g = u8::from_str_radix(&col[3..=4], 16).unwrap_or(255);
            let mut b = u8::from_str_radix(&col[5..=6], 16).unwrap_or(0);

            if r < MIN_COLOR {
                r += MIN_COLOR;
            }
            if g < MIN_COLOR {
                g += MIN_COLOR;
            }
            if b < MIN_COLOR {
                b += MIN_COLOR;
            }

            Color::Rgb { r, g, b }
        }
        _ => colour_from_hash(name),
    }
}

fn message_to_widget(timestamp: DateTime<Local>, chat_entry: &ChatEntry, max_width: usize) -> Vec<Box<dyn Widget>> {
    match chat_entry {
        ChatEntry::Irc(irc) => irc_to_widget(timestamp, irc, max_width),
        ChatEntry::Event(event) => {
            let timestamp = timestamp.format("%H:%M:%S   ").to_string();
            let padding = timestamp.len();
            let mut m = String::with_capacity(padding + event.len());
            (0..padding).for_each(|_| m.push('+'));
            m.push_str("- ");
            m.push_str(&event);
            m.push_str(" -");

            let mut message_lines = lines(&m, max_width);
            let mut first_line = message_lines.remove(0);
            first_line = &first_line[padding..];

            let first_line = Line::Event(
                Text::new(&timestamp, Some(Color::DarkGrey), None),
                Text::new(first_line, Some(Color::Yellow), None),
            );

            let mut lines: Vec<Box<dyn Widget>> = vec![Box::new(first_line)];
            for message in message_lines {
                let line = Line::Cont(Text::new(message, Some(Color::Yellow), None));

                lines.push(Box::new(line));
            }

            lines
        }
    }
}

fn irc_to_widget(timestamp: DateTime<Local>, irc_message: &IrcMessage, max_width: usize) -> Vec<Box<dyn Widget>> {
    let timestamp = timestamp.format("%H:%M:%S   ").to_string();

    let nick = match irc_message.action {
        true => format!("{} ", irc_message.user),
        false => format!("{}: ", irc_message.user),
    };

    let nick_fg = Some(colour_name(&irc_message.tags, &irc_message.user));

    let msg_fg = match irc_message.action {
        true => nick_fg,
        false => None,
    };

    // Pad the message string so the word wrapping happens
    // in the right place.
    //
    // It's a bit of a hack but it will do for now.
    let padding = nick.len() + timestamp.len();
    let mut m = String::with_capacity(padding + irc_message.message.len());
    (0..padding).for_each(|_| m.push('+'));
    m.push_str(&irc_message.message);

    let mut message_lines = lines(&m, max_width);

    let mut first_line = message_lines.remove(0);
    first_line = &first_line[padding..];

    // Create the first line
    let first_line = Line::Start(
        Text::new(&timestamp, Some(Color::DarkGrey), None),
        Text::new(&nick, nick_fg, None),
        Text::new(first_line, msg_fg, None),
    );

    let mut lines: Vec<Box<dyn Widget>> = vec![Box::new(first_line)];
    for message in message_lines {
        let line = Line::Cont(Text::new(message, msg_fg, None));
        lines.push(Box::new(line));
    }

    lines
}

enum ChatEntry {
    Irc(IrcMessage),
    Event(String),
}

pub struct Chat {
    messages: VecDeque<(DateTime<Local>, ChatEntry)>,
    viewport: Viewport,
    scroll_view: ScrollView,
    unread: usize,
    history_size: usize,
}

impl Chat {
    pub fn new(size: ScreenSize) -> Self {
        let mut messages = VecDeque::new();

        Self {
            messages,
            viewport: Viewport::new(ScreenPos::zero(), size),
            scroll_view: ScrollView::new(size),
            unread: 0,
            history_size: 1024,
        }
    }

    pub fn rebuild_widgets(&mut self) {
        // // If the viewport is too small then bail
        // if self.viewport.size.width < 15 {
        //     if self.viewport.size.width > 5 {
        //         self.viewport.draw_widget(
        //             &Text::new(" - ??? - ", None, Some(Color::Red)),
        //             ScreenPos::new(0, self.max_lines() as u16 - 1),
        //         );
        //     }
        //     return;
        // }

        // Build new widgets
        for message in &self.messages {
            let mut widgets = message_to_widget(message.0, &message.1, self.viewport.size.width as usize);
            self.scroll_view.add_widgets(widgets);
        }

        // // Draw the widgets onto the viewport
        // let offset = {
        //     match self.widgets.len() > self.max_lines() {
        //         true => {
        //             self.widgets.len()
        //                 - self.max_lines()
        //                 - self.scroll_offset as usize
        //         }
        //         false => 0,
        //     }
        // };
        // let mut y = 0;
        // for widget in self.widgets.iter().skip(offset) {
        //     match widget {
        //         Line::Start(timestamp, nick, msg) => {
        //             self.viewport.draw_widget(timestamp, ScreenPos::new(0, y));
        //             let mut offset = timestamp.0.len() as u16;
        //             self.viewport.draw_widget(nick, ScreenPos::new(offset, y));
        //             offset += nick.0.len() as u16;
        //             self.viewport.draw_widget(msg, ScreenPos::new(offset, y));
        //         }
        //         Line::Event(timestamp, msg) => {
        //             self.viewport.draw_widget(timestamp, ScreenPos::new(0, y));
        //             let mut offset = timestamp.0.len() as u16;
        //             self.viewport.draw_widget(msg, ScreenPos::new(offset, y));
        //         }
        //         Line::Cont(msg) => {
        //             self.viewport.draw_widget(msg, ScreenPos::new(0, y as u16));
        //         }
        //     }
        //     y += 1;
        // }

        // // If scroll offset isn't zero, show a bar at the bottom
        // if self.scroll_offset != 0 {
        //     let width = self.viewport.size.width as usize;
        //     let new_msg = format!("- New messages: {} -", self.unread);
        //     let msg = format!("{1:^0$}", width, new_msg);
        //     self.viewport.draw_widget(
        //         &Text::new(&msg, None, Some(Color::DarkGrey)),
        //         ScreenPos::new(0, self.max_lines() as u16 - 1),
        //     );
        // }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_view.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll_view.scroll_down();
    }

    pub fn reset_scroll(&mut self) {
        self.scroll_view.reset_scroll();
    }

    pub fn new_message(&mut self, message: IrcMessage) {
        self.new_entry(ChatEntry::Irc(message));
    }

    pub fn new_event_entry(&mut self, input: String) {
        self.new_entry(ChatEntry::Event(input));
    }

    fn new_entry(&mut self, entry: ChatEntry) {
        let now = Local::now();
        self.messages.push_back((now, entry));

        // If the scroll is not at the bottom,
        // offset it by number of new lines so the history
        // doesn't jump around
        let mut offest_offset = 0;
        while self.messages.len() > self.history_size {
            self.messages.pop_front();
            offest_offset += 1;
        }

        if self.scroll_view.offset() != 0 {
            self.unread += 1;
        }

        self.rebuild_widgets();
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.scroll_view.clear();
        self.reset_scroll();
    }
}

impl super::View for Chat {
    fn draw(&mut self, renderer: &mut Renderer<StdoutTarget>) {
        self.viewport.render(renderer);
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.viewport.resize(width, height);
    }
}
