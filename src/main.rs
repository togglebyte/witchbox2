use neotwitch::{Irc, IrcMessage, TwitchMessage, ChannelPoints};
use tinybit::events::{events, EventModel};

mod display;
mod twitch;
mod events;
mod sound_player;

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
    // let sink = rodio::Sink::try_new(&handle).unwrap();

    // let file = std::fs::File::open("examples/music.wav").unwrap();
    // sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).unwrap());

    // sink.sleep_until_end();

    // let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    // let mut player = sound_player::SoundPlayer::new("/home/togglebit/projects/rust/witchbox2/sounds/glass.mp3", handle.clone());
    // let val = player.play(1.0);
    // loop {}


    let (tx, events) = events::<crate::Event>(EventModel::Fps(20));
    tokio::spawn(twitch::start(tx));
    display::run(events);
}
