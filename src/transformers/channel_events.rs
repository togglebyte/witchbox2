use neotwitch::ChannelPoints;
use crate::display::models::{ChannelPointsMessage, DisplayMessage, ChatEvent};

pub struct ChannelPointsTransformer {
}

impl ChannelPointsTransformer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn transform(&mut self, chan_points: ChannelPoints) -> Option<DisplayMessage> {
        match chan_points.reward.title.as_ref() {
            "hydrate! (maybe)" => Some(DisplayMessage::ChannelPoints(ChannelPointsMessage {
                user: chan_points.user.display_name,
                title: chan_points.reward.title,
            })),
            // "random quote" => Some(DisplayMessage::ChannelPoints(ChannelPointsMessage {
            //     user: chan_points.user.display_name,
            //     title: chan_points.reward.title,
            // })),
            // "what os are you using" => Some(DisplayMessage::ChannelPoints(ChannelPointsMessage {
            //     user: chan_points.user.display_name,
            //     title: chan_points.reward.title,
            // })),
            // "Work on: Mixel" => Some(DisplayMessage::ChannelPoints(ChannelPointsMessage {
            //     user: chan_points.user.display_name,
            //     title: chan_points.reward.title,
            // })),
            // "Work on: Witchbox 2" => Some(DisplayMessage::ChannelPoints(ChannelPointsMessage {
            //     user: chan_points.user.display_name,
            //     title: chan_points.reward.title,
            // })),
            // "Work on: Tiny Route" => Some(DisplayMessage::ChannelPoints(ChannelPointsMessage {
            //     user: chan_points.user.display_name,
            //     title: chan_points.reward.title,
            // })),
            "Work on: Terminal Social Network" => Some(DisplayMessage::ChatEvent(ChatEvent::new("Work on o-slash".into()))),

            _ => { 
                log::warn!("Unknown channel event: {}", chan_points.reward.title);
                None
            }
        }
    }
}

