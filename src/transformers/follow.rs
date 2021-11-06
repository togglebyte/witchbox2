use std::time::{Instant, Duration};

use neotwitch::FollowEvent;

use crate::display::models::Follow;

const FOLLOW_DRAIN_LIFE_SECS: u64 = 2;

pub struct FollowTransformer {
    last_drain: Instant,
    follows: Vec<Follow>
}

impl FollowTransformer {
    pub fn new() -> Self {
        Self {
            last_drain: Instant::now(),
            follows: Vec::new(),
        }
    }

    pub fn transform(&mut self, follow: FollowEvent) {
        self.follows.push(Follow(follow.display_name));
    }

    pub fn outstanding(&mut self) -> Option<Vec<Follow>> {
        if self.follows.is_empty() {
            return None;
        }

        if self.last_drain.elapsed() > Duration::from_secs(FOLLOW_DRAIN_LIFE_SECS) {
            self.last_drain = Instant::now();
            Some(self.follows.drain(..).collect())
        } else {
            None
        }
    }
}
