use anathema::{Colors, Line, Lines, Instruction};

pub enum DisplayMessage {
    Chat(ChatMessage),
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
    pub message: String,
    pub color: Option<String>,
}

impl ChatMessage {
    pub fn to_lines(&self, colors: &mut Colors, width: usize) -> Vec<Line> {
        let mut lines = Lines::new(width);

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
}
