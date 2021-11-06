use neotwitch::IrcMessage;
use anathema::{Colors, Line, Lines, Instruction};
use crate::display::random_color;

#[derive(Debug, Clone)]
pub enum DisplayMessage {
    Chat(ChatMessage),
    ChatEvent(ChatEvent),
    ClearChat,
    ChannelPoints(ChannelPointsMessage),
    TodoUpdate(String),
    Sub(Subscription, String),
    Follow(Vec<Follow>, String),
}

#[derive(Debug, Clone)]
pub struct Follow(pub String);

// -----------------------------------------------------------------------------
//     - Models -
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub nick: String,
    pub timestamp: String,
    pub message: String,
    pub color: Option<String>,
    pub action: bool,
}

impl From<IrcMessage> for ChatMessage {
    fn from(mut irc: IrcMessage) -> Self {
        Self {
            nick: irc.nick,
            message: irc.message,
            color: irc.tags.remove("color"),
            timestamp: irc.timestamp.format("%H:%M:%S").to_string(),
            action: irc.action,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatEvent(pub String);

#[derive(Debug, Clone)]
pub struct ChannelPointsMessage {
    pub user: String,
    pub title: String,
    pub sound_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub gift: bool,
    pub gifter: Option<String>,
    pub recipients: Vec<String>,
    pub tier: Tier,
    pub message: String,
    pub sub_type: SubType,
}

#[derive(Debug, Clone, Copy)]
pub enum Tier {
    Prime,
    One,
    Two,
    Three,
    Unknown,
}

impl Tier {
    pub fn from(s: String) -> Tier {
        match s.as_ref() {
            "Prime" => Tier::Prime,
            "1000" => Tier::One,
            "2000" => Tier::Two,
            "3000" => Tier::Three,
            _ => Tier::Unknown,
        }
    }
}

// * sub
// * resub
// * subgift
// * anonsubgift
// * resubgift
// * anonresubgift
#[derive(Debug, Clone, Copy)]
pub enum SubType {
    NewSub,
    Resub,
    Gift,
    Unknown,
}
