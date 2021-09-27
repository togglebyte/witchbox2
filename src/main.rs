use neotwitch::{Irc, IrcMessage, TwitchMessage, ChannelPoints};
use tinybit::events::{events, EventModel};

mod display;
mod twitch;

pub enum Event {
    Chat { nick: String, msg: String, action: bool },
    ClearChat,
    Twitch(twitch::Twitch),
    Log(String),
    Quit,
}

impl Event {
    fn from_irc(irc: Irc) -> Self {
        match irc {
            Irc::Message(msg) => Self::Chat {
                msg: msg.message.into(),
                nick: msg.user.into(),
                action: msg.action,
            },
            Irc::ClearChat => Self::ClearChat,
        }
    }

    fn from_bits(bits: neotwitch::BitsEvent) -> Self {
        Self::Twitch(twitch::Twitch::Bits(bits))
    }

    fn from_channel_event(channel_ev: ChannelPoints) -> Self {
        Self::Twitch(twitch::Twitch::ChannelEvent(channel_ev))
    }
}

impl Into<tinybit::events::Event<Event>> for Event {
    fn into(self) -> tinybit::events::Event<Event> {
        tinybit::events::Event::User(self)
    }
}

#[tokio::main]
async fn main() {
    let (tx, events) = events::<crate::Event>(EventModel::Fps(20));
    let handle = std::thread::spawn(move || {
        display::run(events);
    });
    tokio::spawn(twitch::start(tx));
    let _ = handle.join();
}
