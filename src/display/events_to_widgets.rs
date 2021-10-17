use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::Hasher;

use chrono::prelude::*;
use neotwitch::IrcMessage;
use tinybit::widgets::{ScrollView, Text, Widget};
use tinybit::{Color, Pixel, ScreenSize};

use super::lines;

pub enum Line {
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
                pixels.extend(apply_offset(nick.pixels(), offset));
                offset += nick.size().width;
                pixels.extend(apply_offset(msg.pixels(), offset));
                pixels
            }
            Self::Event(ts, msg) => {
                let mut pixels = ts.pixels();
                let mut offset = ts.size().width;
                pixels.extend(apply_offset(msg.pixels(), offset));
                pixels
            }
            Self::Cont(msg) => msg.pixels(),
        }
    }

    fn size(&self) -> ScreenSize {
        match self {
            Self::Start(ts, nick, msg) => ScreenSize::new(ts.size().width + nick.size().width + msg.size().width, 1),
            Self::Event(ts, msg) => ScreenSize::new(ts.size().width + msg.size().width, 1),
            Self::Cont(msg) => ScreenSize::new(msg.size().width, 1),
        }
    }
}

fn apply_offset(mut pixels: Vec<Pixel>, offset: u16) -> Vec<Pixel> {
    pixels.iter_mut().for_each(|mut p| p.pos.x += offset);
    pixels
}

pub fn message_to_widget(
    timestamp: DateTime<Local>,
    chat_event: &crate::Event,
    max_width: usize,
) -> Vec<Box<dyn Widget>> {
    match chat_event {
        crate::Event::Chat(irc) => irc_to_widget(timestamp, irc, max_width),
        crate::Event::ChatEvent(msg) => event_to_widget(timestamp, msg, max_width),
        _ => {
            todo!()
        } // ChatEntry::Event(event) => event_to_widget(
    }
}

fn colour_from_hash(name: &str) -> Color {
    let num = hash_str(&name);
    let bytes = num.to_ne_bytes();
    Color::Rgb { r: bytes[0], g: bytes[1], b: bytes[2] }
}

fn hash_str<T: std::hash::Hash>(input: &T) -> u32 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let num = hasher.finish() as u32;
    num
}

fn colour_name(tags: &HashMap<String, String>, name: &str) -> Color {
    match tags.get("color") {
        Some(col) if col.len() > 6 => {
            let mut r = u8::from_str_radix(&col[1..=2], 16).unwrap_or(0);
            let mut g = u8::from_str_radix(&col[3..=4], 16).unwrap_or(255);
            let mut b = u8::from_str_radix(&col[5..=6], 16).unwrap_or(0);
            Color::Rgb { r, g, b }
        }
        _ => colour_from_hash(name),
    }
}

// -----------------------------------------------------------------------------
//     - Convert IRC message to widget -
// -----------------------------------------------------------------------------
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

fn event_to_widget(timestamp: DateTime<Local>, msg: &str, max_width: usize) -> Vec<Box<dyn Widget>> {
    let timestamp = timestamp.format("%H:%M:%S   ").to_string();
    let padding = timestamp.len();
    let mut m = String::with_capacity(padding + msg.len());
    (0..padding).for_each(|_| m.push('+'));
    m.push_str("- ");
    m.push_str(&msg);
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
