use anathema::{Colors, Line, Lines, Instruction};

pub enum Display {
    Chat(ChatMessage),
    ClearChat,
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
            log::info!("{:?}", res);
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
