use neotwitch::SubscribeEvent;
use crate::display::models::Subscription;

pub struct SubTransformer {
}

impl SubTransformer {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn transform(&mut self, sub: SubscribeEvent) -> Option<Subscription> {
        let sub = Subscription {
            gift: sub.is_gift,
            gifter: sub.display_name,
            recipients: Vec::new(),
        };

        Some(sub)
    }
}
