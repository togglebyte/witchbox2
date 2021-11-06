use std::time::{Instant, Duration};

use neotwitch::SubscribeEvent;

use crate::display::models::{Subscription, SubType, Tier};

const SUB_DRAIN_LIFE_SECS: u64 = 2;

pub struct SubTransformer {
    subs: Vec<(Instant, Subscription)>
}

impl SubTransformer {
    pub fn new() -> Self {
        Self {
            subs: Vec::new(),
        }
    }

    pub fn transform(&mut self, sub: SubscribeEvent) {
        let updated_existing = self.subs
            .iter_mut()
            .filter(|(_, s)| s.gifter == sub.display_name && s.gift)
            .any(|(inst, existing_sub)| {
                if let Some(ref recipient) = sub.recipient_display_name {
                    existing_sub.recipients.push(recipient.clone());
                }
                *inst = Instant::now();
                true
            });

        if !updated_existing {
            let sub_type = match sub.context.as_ref() {
                "sub" => SubType::NewSub,
                "resub" => SubType::Resub,
                "subgift" | "anonsubgift" => SubType::Gift,
                "resubgift" | "anonresubgift" => SubType::Resub,
                _ => SubType::Unknown
            };

            let sub = Subscription {
                gift: sub.is_gift,
                gifter: sub.display_name,
                recipients: Vec::new(),
                tier: Tier::from(sub.sub_plan),
                message: sub.sub_message.message,
                sub_type,
            };
            self.subs.push((Instant::now(), sub));
        }
    }

    pub fn outstanding(&mut self) -> Vec<Subscription> {
        // Because drain_filter is nightly only for now
        let mut ready = vec![];
        let mut i = 0;
        while i < self.subs.len() {
            if self.subs[i].0.elapsed() > Duration::from_secs(SUB_DRAIN_LIFE_SECS) {
                let (_, val) = self.subs.remove(i);
                ready.push(val);
            } else {
                i += 1;
            }
        }

        ready
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use super::*;

    #[test]
    fn drain_subs() {
        let mut transformer = SubTransformer::new();
        transformer.transform(make_new_sub());
        transformer.transform(make_new_sub());
        assert_eq!(transformer.outstanding().len(), 0);
    }

    #[test]
    fn drain_subs_outstanding() {
        // Two subs, neither gifts, so there should be
        // two actual subs in there
        let mut transformer = SubTransformer::new();
        transformer.transform(make_new_sub());
        transformer.transform(make_new_sub());
        transformer.subs[0].0 -= Duration::from_secs(SUB_DRAIN_LIFE_SECS + 1);
        let len = transformer.outstanding().len();
        assert_eq!(len, 1);
        assert_eq!(transformer.subs.len(), 1);
    }

    #[test]
    fn gift_subs() {
        let mut transformer = SubTransformer::new();
        let subs = gift_five_subs();
        for s in subs {
            transformer.transform(s);
        }
        assert_eq!(transformer.subs.len(), 1);
    }

    fn make_new_sub() -> SubscribeEvent {
        SubscribeEvent {
            display_name: Some("user".to_string()),
            sub_plan: "2000".into(),
            cumulative_months: Some(1),
            streak_months: None,
            is_gift: false,
            context: "sub".to_string(),
            ..Default::default()
        }
    }

    fn gift_five_subs() -> Vec<SubscribeEvent> {
        (0..5).map(|i| 
            SubscribeEvent {
                display_name: Some("user".to_string()),
                sub_plan: "2000".into(),
                cumulative_months: Some(1),
                streak_months: None,
                is_gift: true,
                context: "sub".to_string(),
                recipient_display_name: Some(format!("recipient-{}", i)),
                ..Default::default()
            }
        ).collect()

    }
}
