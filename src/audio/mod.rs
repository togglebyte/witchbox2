use rand::prelude::*;

mod sound_player;
pub use sound_player::SoundPlayer;

pub fn default_sound() -> String {
    "/home/togglebit/projects/stream/misc/default.mp3".into()
}

pub fn random_arch() -> String {
    let sounds =
        (1..=13).map(|id| format!("/home/togglebit/projects/stream/misc/arch{}.mp3", id)).collect::<Vec<String>>();

    let mut rng = thread_rng();
    sounds.choose(&mut rng).unwrap().to_owned()
}

pub fn random_follow() -> String {
    let sounds =
        (1..=3).map(|id| format!("/home/togglebit/projects/stream/misc/follow{}.mp3", id)).collect::<Vec<String>>();

    let mut rng = thread_rng();
    sounds.choose(&mut rng).unwrap().to_owned()
}

pub fn random_sub() -> String {
    let sounds = (1..=14)
        .map(|id| format!("/home/togglebit/projects/stream/sounds/sub{}.mp3", id))
        .collect::<Vec<String>>();

    let mut rng = thread_rng();
    sounds.choose(&mut rng).unwrap().to_owned()
}

