use std::thread;

use neotwitch::{ChannelPoints, FollowEvent, Irc, IrcMessage, SubscribeEvent};

mod audio;
mod transformers;
mod twitch;
mod display;
mod testdata;
mod todo;

pub type EventSender = tokio::sync::mpsc::Sender<Event>;
pub type EventReceiver = tokio::sync::mpsc::Receiver<Event>;

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

#[tokio::main]
async fn main() {
    tinylog::init_logger()
        .await
        .expect("Failed to start logger");

    let mut args = std::env::args().skip(1);

    let mut ret = false;
    while let Some(s) = args.next() {
        ret = true;
        match s.as_ref() {
            "hydrate" => testdata::hydrate().await,
            "bits" => testdata::bits().await,
            "giftsub" => testdata::gift_sub().await,
            "anongiftsub" => testdata::anon_gift_sub().await,
            "oslash" => testdata::oslash().await,
            "follow" => testdata::follow().await,
            "chat" => testdata::chat().await,
            "action" => testdata::action().await,
            _ => {}
        }
    }

    if ret {
        return;
    }

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let (display_tx, display_rx) = display::channel();

    tokio::spawn(transformers::run(rx, display_tx.clone()));
    tokio::spawn(todo::watch_todo(display_tx, "/home/togglebit/wiki/todo.md"));
    tokio::spawn(twitch::start(tx.clone()));

    if let Err(e) = display::run(display_rx) {
        eprintln!("Fail: {}", e);
    }
}


















//     let mut orig_messages = vec![
//          Entry { color: Some(random_color_string()), nick: "suuuuperlonglurpdjjrpsomekindoflinenamethatisreallyinconvenientbutitsgoingtobeherefornowsoicantestthis".into(),    message: "4 first blah blah".into() },
//          Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
//          Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
//          Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
//          Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
//          Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
//          Entry { color: Some(random_color_string()), nick: "florpy".into(),    message: "🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅🍅".into() },
//     ];
