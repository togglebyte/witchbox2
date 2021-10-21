use anathema::{Colors, Line, Lines, Instruction};
use crate::display::random_color;

pub enum DisplayMessage {
    Chat(ChatMessage),
    ChatEvent(ChatEvent),
    ClearChat,
    ChannelPoints(ChannelPointsMessage),
    Sub(Subscription),
}

// -----------------------------------------------------------------------------
//     - Models -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct ChatMessage {
    pub nick: String,
    pub timestamp: String,
    pub message: String,
    pub color: Option<String>,
}

impl ChatMessage {
    pub fn to_lines(&self, colors: &mut Colors, width: usize) -> Vec<Line> {
        let mut lines = Lines::new(width);

        if let Ok(col) = Colors::init_fg(crate::display::GREY) {
            lines.push(Instruction::Color(col));
        }
        lines.push_str(&self.timestamp);

        if let Some(ref col) = self.color {
            let res = colors.from_hex(col).and_then(Colors::init_fg);
            if let Ok(col) = res {
                lines.push(Instruction::Color(col));
            }
        }

        lines.push_str(&self.nick);
        lines.push(Instruction::Reset);

        lines.push(Instruction::Pad(1));

        lines.push_str(&self.message);
        lines.complete()
    }
}

pub struct ChatEvent(String);

impl ChatEvent {
    pub fn new(ev: String) -> Self {
        Self(ev)
    }

    pub fn to_lines(&self, width: usize) -> Vec<Line> {
        let mut lines = Lines::new(width);

        // Get a random colour
        let color = random_color();

        if let Ok(col) = Colors::init_fg(color) {
            lines.push(Instruction::Color(col));
        }

        lines.push_str(&format!("{:─>width$}", "─", width=width));
        lines.push_str(&format!("{:<width$}", self.0, width=width));
        lines.push_str(&format!("{:─>width$}", "─", width=width));
        lines.push(Instruction::Reset);
        lines.complete()
    }
}

#[derive(Debug)]
pub struct ChannelPointsMessage {
    pub user: String,
    pub title: String,
}

#[derive(Debug)]
pub struct Subscription {
    pub gift: bool,
    pub gifter: Option<String>,
    pub recipients: Vec<String>,
    pub tier: Tier,
    pub message: String,
    pub sub_type: SubType,
}

#[derive(Debug)]
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
#[derive(Debug)]
pub enum SubType {
    NewSub,
    Resub,
    Gift,
    Unknown,
}
