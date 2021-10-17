use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Duration;

use anathema::*;
use log::info;
use neotwitch::{ChannelPoints, FollowEvent, Irc, IrcMessage, SubscribeEvent, TwitchMessage};
use rand::distributions::{DistIter, DistString};
use rand::prelude::*;
use twitch::Twitch;

mod display;
mod events;
mod sound_player;
mod twitch;
// mod line;

// pub use line::{Line, Lines, Instruction};

// pub type Sendy = std::sync::mpsc::Sender<tinybit::events::Event<Event>>;
pub type Sendy = std::sync::mpsc::Sender<Event>;

pub enum Event {
    Chat(IrcMessage),
    ChatEvent(String),
    ClearChat,
    Twitch(twitch::Twitch),
    Quit,
}

impl Event {
    fn from_irc(irc: Irc) -> Self {
        match irc {
            Irc::Message(msg) => Self::Chat(msg),
            Irc::ClearChat => Self::ClearChat,
        }
    }

    fn from_bits(bits: neotwitch::BitsEvent) -> Self {
        Self::Twitch(twitch::Twitch::Bits(bits))
    }

    fn from_channel_event(channel_ev: ChannelPoints) -> Self {
        Self::Twitch(twitch::Twitch::ChannelEvent(channel_ev))
    }

    fn from_follow(follow: FollowEvent) -> Self {
        Self::Twitch(twitch::Twitch::Follow(follow))
    }

    fn from_sub(sub: SubscribeEvent) -> Self {
        Self::Twitch(twitch::Twitch::Sub(sub))
    }
}

// impl Into<tinybit::events::Event<Event>> for Event {
//     fn into(self) -> tinybit::events::Event<Event> {
//         tinybit::events::Event::User(self)
//     }
// }

// -----------------------------------------------------------------------------
//     - Everything from here on out... -
//     ... is mostly rubbish
// -----------------------------------------------------------------------------

enum Inst {
    Line(String),
    Color(u32),
    Pad(usize),
    Reset,
}

fn random_color_string() -> String {
    let mut ret = "#".to_string();
    for _ in 0..6 {
        let c = "0123456789ABCDEF"
            .chars()
            .choose(&mut thread_rng())
            .unwrap();
        ret.push(c);
    }
    ret
}

#[derive(Debug)]
struct Entry {
    color: Option<String>,
    nick: String,
    message: String,
}

fn entry_to_inst(entry: &Entry, colors: &mut Colors, width: usize) -> Vec<Line> {
    let mut lines = Lines::new(width);

    if let Some(ref col) = entry.color {
        if let Ok(col) = colors.from_hex(col).and_then(Colors::init_fg) {
            lines.push(Instruction::Color(col));
        }
    }

    lines.push_str(&entry.nick);
    lines.push(Instruction::Reset);

    lines.push(Instruction::Pad(1));

    lines.push_str(&entry.message);
    lines.complete()
}

fn event_to_inst(event: Twitch, colors: &mut Colors, width: usize) -> Option<Vec<Line>> {
    if let Twitch::ChannelEvent(cp) = event {
        let mut lines = Lines::new(width);
        let top = format!("{:->width$}", "", width = width);
        let bottom = format!("{:->width$}", "", width = width);

        if let Ok(color) = colors.from_hex("#ff0000").and_then(Colors::init_fg) {
            lines.push(Instruction::Color(color));
        }

        lines.push_str(&top);

        let message = format!("{} - {}", cp.user.display_name, cp.reward.title);
        let messages = split(&message, width, 2); // add one space for each side of the border

        for message in messages {
            lines.push_str(&format!("| {} {:>width$}", message, "|", width=width));
        }

        lines.push_str(&bottom);
        lines.push(Instruction::Reset);

        return Some(lines.complete());

        // Draw top border
        // Message
        // Draw bottom border

        // id: String,
        // pub user: User,
        // channel_id: String,
        // redeemed_at: String,
        // pub reward: Reward,
        // status: String,
        // pub user_input: Option<String>,
    }

    None
}

#[tokio::main]
async fn main() {
    tinylog::init_logger()
        .await
        .expect("Failed to start logger");

    let (tx, rx) = mpsc::channel();
    tokio::spawn(twitch::start(tx.clone()));
    // display::run(events, tx);

    let mut colors = Colors::new();

    // -----------------------------------------------------------------------------
    //     - Pancurses setup -
    // -----------------------------------------------------------------------------
    // let window = initscr();
    let window = Window::main(true).unwrap();
    window.set_cursor_visibility(Cursor::Hide);
    window.no_delay(true);
    // window.enable_raw();
    let size = window.size();

    let chat_height: i32 = size.height - 7;

    let event_size = Size::new(size.width, size.height - chat_height);
    let event_pos = Pos::new(0, 0);
    let eventwin = window.new_window(event_pos, event_size).unwrap();

    let chat_size = Size::new(size.width, chat_height);
    let chat_pos = Pos::new(0, event_size.height);
    let mut outer_chatwin = window.new_window(chat_pos, chat_size).unwrap();

    let chat_size = Size::new(size.width - 2, chat_size.height - 2);
    let chat_pos = Pos::new(1, chat_pos.y + 1);
    let mut chatwin = window.new_window(chat_pos, chat_size).unwrap();

    Colors::init_pair(0, Color::White, Color::Black);

    let mut height = chatwin.size().height as usize;

    let mut orig_messages = vec![
         Entry { color: Some(random_color_string()), nick: "suuuuperlonglurpdjjrpsomekindoflinenamethatisreallyinconvenientbutitsgoingtobeherefornowsoicantestthis".into(),    message: "4 first blah blah".into() },
         Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
         Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
         Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
         Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
         Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
         Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
         // Entry { color: Some(random_color_string()), nick: "Blrp".to_string(),          message: "firstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlinfirstlineeeeeeeeeeeeeeeeefirstline".to_string() },
         // Entry { color: Some(random_color_string()), nick: "Blorpsdafasdfkdfs".into(),  message: "0 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Blawdajorp".into(),         message: "1 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Blawdorp".into(),           message: "2 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Bloawd arp".into(),         message: "3 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Bloawd arp".into(),         message: "3 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Bloawd arp".into(),         message: "3 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Bloawd arp".into(),         message: "3 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Bl".into(),                 message: "5 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "ABCBlorp".into(),           message: "6 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Rainbowlarks".into(),       message: "7 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Superusernumberfive".into(),message: "8 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Blsadfsorp".into(),         message: "9 first blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Blorp".into(),              message: "second block of blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Borp".into(),               message: "0 second blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Bosdfsdfkrp".into(),        message: "1 second blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Borp".into(),               message: "2 second blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Blosdfrp".into(),           message: "3 second blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Blorp".into(),              message: "4 second blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Bdfssdfop".into(),          message: "5 second blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Blrp".into(),               message: "6 second blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Blorp".into(),              message: "7 second blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Brp".into(),                message: "8 second blah blah".into() },
         // Entry { color: Some(random_color_string()), nick: "Bob".to_string(),           message: "last line and this should span multiple lines and this is a difficult one to fix but I reckon we can do it if we can store the buffer somewhere".to_string() },
    ];

    // let orig_messages = (10..202)
    //     .step_by(10)
    //     .map(|i| Entry {
    //         color: Some(random_color_string()),
    //         message: format!("{:-<i$}b", i = i),
    //         nick: "hi".to_string(),
    //     })
    //     .collect::<Vec<_>>();

    let mut chat_messages = orig_messages;

    let messages = chat_messages
        .iter()
        .map(|e| entry_to_inst(e, &mut colors, chatwin.size().width as usize))
        .flatten()
        .collect::<Vec<_>>();

    let mut scroll_buffer: ScrollBuffer<Line> = ScrollBuffer::from_vec(messages, height, 124);
    scroll_buffer.scroll_to_end();

    loop {
        while let Ok(event) = rx.try_recv() {
            match event {
                Event::Chat(irc) => {
                    let entry = Entry {
                        color: irc.tags.get("color").cloned(),
                        nick: irc.user,
                        message: irc.message,
                    };
                    let imfedup = entry_to_inst(&entry, &mut colors, chatwin.size().width as usize);

                    for val in imfedup {
                        scroll_buffer.push(val);
                    }
                    chat_messages.push(entry);
                }
                Event::Twitch(t) => {
                    if let Some(lines) =
                        event_to_inst(t, &mut colors, chatwin.size().width as usize)
                    {
                        for line in lines {
                            scroll_buffer.push(line);
                        }
                    }
                }
                _ => {}
            }
        }

        // outer_chatwin.set_border();

        if scroll_buffer.is_dirty() {
            chatwin.erase();
        }

        for line in scroll_buffer.lines() {
            let mut pos = chatwin.get_cursor();
            for inst in line.instructions() {
                match inst {
                    Instruction::Color(col) => {
                        let pair_id = Colors::get_color_pair(*col);
                        chatwin.set_color(pair_id);
                    }
                    Instruction::Reset => {
                        let pair_id = Colors::get_color_pair(0);
                        chatwin.set_color(pair_id);
                    }
                    Instruction::Line(line) => drop(chatwin.print(line)),
                    Instruction::Pad(pad) => {
                        let mut pos = chatwin.get_cursor();
                        pos.x += *pad as i32;
                        chatwin.move_cursor(pos);
                    }
                }
            }
            pos.y += 1;
            pos.x = 0;
            chatwin.move_cursor(pos);
        }

        eventwin.refresh();
        chatwin.refresh();

        if let Some(key) = window.get_input() {
            match key {
                Input::Character('k') => {
                    // up
                    scroll_buffer.scroll_up(1);
                    chatwin.erase();
                }
                Input::Character('j') => {
                    // down
                    scroll_buffer.scroll_down(1);
                    chatwin.erase();
                }
                Input::Character('c') => break,
                _ => {}
            }
        }

        //     // if let Some(Input::KeyResize) = window.getch() {
        //     //     window.erase();
        //     //     window.refresh();
        //     //     let (height, width) = window.get_max_yx();
        //     //     chatwin = window.subwin(height - 7, width, 7, 0).unwrap();
        //     //     chatwin.scrollok(true);
        //     // }

        window.nap(Duration::from_millis(50));
    }

    // endwin();
}
