use neotwitch::ChannelPoints;
use crate::display::models::ChannelPointsMessage;

pub struct ChannelPointsTransformer {
}

impl ChannelPointsTransformer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn transform(&mut self, chan_points: ChannelPoints) -> ChannelPointsMessage {
        ChannelPointsMessage {
            user: chan_points.user.display_name,
            title: chan_points.reward.title,
        }
    }
}

