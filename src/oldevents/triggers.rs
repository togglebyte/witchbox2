use std::path::Path;
use std::fmt::Write;

use neotwitch::SubscribeEvent;
use rand::prelude::*;
use rodio::{OutputStream, OutputStreamHandle};

use crate::sound_player::SoundPlayer;
use crate::twitch::Twitch;

pub struct Triggers {
    output_handle: Option<OutputStreamHandle>,
    output_stream: Option<OutputStream>,
    current_player: Option<SoundPlayer>,
    sendy: crate::Sendy,
}

impl Triggers {
    fn play(&mut self, path: impl AsRef<Path>, volume: f32) {
        if let Some(ref handle) = self.output_handle {
            let mut player = SoundPlayer::new(path, handle.clone());
            player.play(volume);
            self.current_player = Some(player);
        }
    }

    pub fn new(sendy: crate::Sendy) -> Self {
        let (output_stream, output_handle) = match OutputStream::try_default() {
            Ok((stream, handle)) => (Some(stream), Some(handle)),
            Err(_e) => (None, None),
        };
        Self { output_handle, output_stream, current_player: None, sendy }
    }

    pub fn trigger(&mut self, twitch: &Twitch) {
        match twitch {
            Twitch::Bits(bits) => self.play(random_bits(), 1.0),
            Twitch::ChannelEvent(chan) => match chan.reward.title.as_ref() {
                "Work on: Witchbox 2" => drop(self.sendy.send(crate::Event::ChatEvent("Work on Witchbox 2".into()))),
                "Work on: Tiny Route" => drop(self.sendy.send((crate::Event::ChatEvent("Work on Tiny Route".into())))),
                "Work on: Mixel" => drop(self.sendy.send(crate::Event::ChatEvent("Work on Mixel".into()))),
                "what os are you using" => self.play(random_arch(), 1.0),
                "hydrate! (maybe)" => {
                    drop(self.sendy.send(crate::Event::ChatEvent("Consume some sort of beverage".into())));
                    self.play("/home/togglebit/projects/rust/witchbox2/sounds/glass.mp3", 0.7);
                }
                _ => {}
            },
            Twitch::Follow(_) => self.play(random_follow(), 1.0),
            Twitch::Sub(_) => self.play(random_sub(), 1.0),
        };
    }

    pub fn display(&mut self, twitch: &Twitch) -> String {
        match twitch {
            Twitch::Bits(bits) => {
                match bits.data.bits_used {
                    1 => match &bits.data.user_name {
                        Some(name) => format!("Thank you for the bit, {}", name),
                        None => format!("Thank for the anon bit"),
                    }
                    bit_count => match &bits.data.user_name {
                        Some(name) => format!("Thank you for the {} bits, {}", bit_count, name),
                        None => format!("Thank you for the {} anon bits", bit_count),
                    }
                }
            }
            Twitch::Follow(follow) => format!("{} is now following!", follow.username),
            Twitch::Sub(sub) => sub_input(sub),
            Twitch::ChannelEvent(chan) => {
                match chan.reward.title.as_ref() {
                    "what os are you using" => {}
                    _ => {}
                }
                format!("{}: {}", chan.user.display_name, chan.reward.title)
            }
        }
    }
}

// -----------------------------------------------------------------------------
//     - Arch, btw -
// -----------------------------------------------------------------------------
fn random_arch() -> String {
    let sounds = (1..=13)
        .map(|id| {
            format!("/home/togglebit/projects/stream/misc/arch{}.mp3", id)
        })
        .collect::<Vec<String>>();

    let mut rng = thread_rng();
    sounds.choose(&mut rng).unwrap().to_owned()
}

// -----------------------------------------------------------------------------
//     - Subs! -
// -----------------------------------------------------------------------------
fn random_sub() -> String {
    let sounds = (1..=14)
        .map(|id| format!("/home/togglebit/projects/stream/sounds/sub{}.mp3", id))
        .collect::<Vec<String>>();

    let mut rng = thread_rng();
    sounds.choose(&mut rng).unwrap().to_owned()
}

// -----------------------------------------------------------------------------
//     - Bits -
// -----------------------------------------------------------------------------
fn random_bits() -> String {
    let sounds = (1..=5)
        .map(|id| format!("/home/togglebit/projects/stream/misc/bits{}.mp3", id))
        .collect::<Vec<String>>();

    let mut rng = thread_rng();
    sounds.choose(&mut rng).unwrap().to_owned()
}

// -----------------------------------------------------------------------------
//     - Follow -
// -----------------------------------------------------------------------------
fn random_follow() -> String {
    let sounds = (1..=2)
        .map(|id| format!("/home/togglebit/projects/stream/misc/follow{}.mp3", id))
        .collect::<Vec<String>>();

    let mut rng = thread_rng();
    sounds.choose(&mut rng).unwrap().to_owned()
}


// -----------------------------------------------------------------------------
//     - Sub input -
// -----------------------------------------------------------------------------
fn sub_input(event: &SubscribeEvent) -> String {
    match event.context.as_str() {
        "sub" => format!("{} just subscribed!!!", display_name(&event.display_name)),
        "resub" => {
            let mut output = format!("{} subscribed for {} months!!!",
                display_name(&event.display_name),
                event.cumulative_months.unwrap_or(1)
            );
            if event.streak_months.unwrap_or(1) > 1 {
                let _ = write!(output, "\nA {} month streak!!!", event.streak_months.unwrap_or(1));
            }
            output
        }
        "subgift" | "resubgift" | "anonsubgift" | "anonresubgift" => {
            let mut tail = String::new();

            let _ = write!(
                tail,
                "just gifted {} a subscription",
                display_name(&event.recipient_display_name)
            );

            let mut months = 0;
            if event.cumulative_months.unwrap_or(1) > 1 {
                months = event.cumulative_months.unwrap_or(1);
            } else if event.months.unwrap_or(1) > 1 {
                months = event.months.unwrap_or(1);
            }

            if months > 1 {
                let _ = write!(tail, " for month {}!!!", months);
            } else {
                tail.push_str("!!!");
            }

            months = 0;
            if event.streak_months.unwrap_or(1) > 1 {
                months = event.streak_months.unwrap_or(1);
            } else if event.multi_month_duration.unwrap_or(1) > 1 {
                months = event.multi_month_duration.unwrap_or(1);
            }
            if months > 1 {
                let _ = write!(tail, "\nA {} month streak", months);
            }

            match &event.display_name {
                Some(name) if name.len() > 0 => drop(format!("{} {}", name, tail)),
                _ => drop(format!("Anonymous {}", tail)),
            }

            tail
        }
        _ => String::new(),
    }



}

fn display_name(name: &Option<String>) -> String {
    name.clone().unwrap_or("[Anon]".into())
}
