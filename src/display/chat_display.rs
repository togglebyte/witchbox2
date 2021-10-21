use anathema::Input;
use anyhow::Result;

use super::models::DisplayMessage;
use super::{DisplayHandler, DisplayContext};

pub struct ChatDisplay;

impl DisplayHandler for ChatDisplay {
    fn handle(&mut self, context: DisplayContext, msg: &DisplayMessage) {
        match msg {
            DisplayMessage::Chat(msg) => {
                let lines = msg.to_lines(context.colors, context.window.size().width as usize);
                lines
                    .into_iter()
                    .for_each(|line| context.buffer.push(line));
            },
            DisplayMessage::ChatEvent(ev) => {
                let lines = ev.to_lines(context.window.size().width as usize);
                lines
                    .into_iter()
                    .for_each(|line| context.buffer.push(line));
            }
            DisplayMessage::ClearChat => context.buffer.clear(),
            DisplayMessage::Sub(_)
            | DisplayMessage::ChannelPoints(_) => {}
        };

    }


    fn input(&mut self, context: DisplayContext, input: Input) -> Result<()> {
        match input {
            Input::Character('k') => {
                // up
                context.buffer.scroll_up(1);
                context.window.erase()?;
            }
            Input::Character('j') => {
                // down
                context.buffer.scroll_down(1);
                context.window.erase()?;
            }
            _ => {}
        }

        Ok(())
    }
}
