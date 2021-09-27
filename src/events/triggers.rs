use rodio::{OutputStreamHandle, OutputStream};

use crate::twitch::Twitch;
use crate::sound_player::SoundPlayer;

pub struct Triggers {
    output_handle: Option<OutputStreamHandle>,
    output_stream: Option<OutputStream>,
    current_player: Option<SoundPlayer>,
}

impl Triggers {
    fn play(&mut self, path: &str, volume: f32) {
        if let Some(ref handle) = self.output_handle {
            let mut player = SoundPlayer::new(path, handle.clone());
            player.play(volume);
            self.current_player = Some(player);
        }
    }

    pub fn new() -> Self {
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();
        // let (output_stream, output_handle) = match OutputStream::try_default() {
        //     Ok((stream, handle)) => Some(handle),
        //     Err(_e) => (None, None),
        // };
        Self {
            output_handle: Some(handle),
            output_stream: Some(stream),
            current_player: None,
        }
    }

    pub fn trigger(&mut self, twitch: &Twitch) {
        match twitch {
            Twitch::Bits(bits) => {
            }
            Twitch::ChannelEvent(chan) => {
                match chan.reward.title.as_ref() {
                    "what os are you using" => {}
                    "hydrate! (maybe)" => {
                        self.play("/home/togglebit/projects/rust/witchbox2/sounds/glass.mp3", 1.0);
                    }
                    _ => {}
                }
            }
        };
    }

    pub fn display(&mut self, twitch: &Twitch) -> String {
        let input = match twitch {
            Twitch::Bits(bits) => format!("Bits..."),
            Twitch::ChannelEvent(chan) => {
                match chan.reward.title.as_ref() {
                    "what os are you using" => {}
                    _ => {}
                }
                format!("{}: {}", chan.user.display_name, chan.reward.title)
            }
        };

        input
    }
}
