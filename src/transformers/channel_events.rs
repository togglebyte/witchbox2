use neotwitch::ChannelPoints;
use fortune_countdown::Quotes;

use crate::display::models::{ChannelPointsMessage, DisplayMessage, ChatEvent};
use crate::display::random_color;
use crate::audio::{default_sound, random_arch};

pub struct ChannelPointsTransformer {
    quotes: Quotes,
}

impl ChannelPointsTransformer {
    pub fn new() -> Self {
        Self {
            quotes: Quotes::new("/home/togglebit/projects/rust/fortune-countdown/datfiles/").expect("Quotes missing"),
        }
    }

    pub fn transform(&mut self, chan_points: ChannelPoints) -> Option<DisplayMessage> {
        match chan_points.reward.title.as_ref() {
            "hydrate! (maybe)" => Some(DisplayMessage::ChannelPoints(ChannelPointsMessage {
                user: chan_points.user.display_name,
                title: chan_points.reward.title,
                sound_path: Some(default_sound()),
            })),
            "what os are you using" => Some(DisplayMessage::ChannelPoints(ChannelPointsMessage {
                user: chan_points.user.display_name,
                title: chan_points.reward.title,
                sound_path: Some(random_arch()),
            })),
            "random quote" => {
                let color = random_color();
                Some(DisplayMessage::Quote(self.quotes.next().quote, color))
            }
            "Work on: Mixel" => Some(DisplayMessage::ChatEvent(ChatEvent("Work on Mixel".into()))),
            "Work on: Witchbox" => Some(DisplayMessage::ChatEvent(ChatEvent("Work on Witchbox".into()))),
            "Work on: Tiny Route" => Some(DisplayMessage::ChatEvent(ChatEvent("Work on Tiny Route".into()))),
            "Work on: Terminal Social Network" => Some(DisplayMessage::ChatEvent(ChatEvent("Work on o/".into()))),

            _ => { 
                log::warn!("Unknown channel event: {}", chan_points.reward.title);
                None
            }
        }
    }
}

