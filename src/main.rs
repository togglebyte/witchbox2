use neotwitch::{IrcMessage, TwitchMessage};
use tinybit::events::{events, EventModel};

mod display;
mod twitch;

pub enum Event {
    Chat { nick: String, msg: String, action: bool },
    Twitch(TwitchMessage),
    Log(String),
    Quit,
}

impl Event {
    fn from_irc(irc_msg: IrcMessage) -> Self {
        Self::Chat {
            msg: irc_msg.message.into(),
            nick: irc_msg.user.into(),
            action: irc_msg.is_action,
        }
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
