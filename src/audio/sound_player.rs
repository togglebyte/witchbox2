use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use rodio::{Decoder, OutputStreamHandle, Sink};

pub struct SoundPlayer {
    sound_path: PathBuf,
    output_handle: OutputStreamHandle,
    sink: Option<Sink>,
}

impl SoundPlayer {
    pub fn new(sound_path: impl AsRef<Path>, output_handle: OutputStreamHandle) -> Self {
        Self {
            sound_path: sound_path.as_ref().into(),
            output_handle,
            sink: None,
        }
    }

    pub fn play(&mut self, volume: f32) -> Option<()> {
        let file = File::open(&self.sound_path).ok()?;
        let file_reader = BufReader::new(file);
        let source = Decoder::new(file_reader).ok()?;

        if self.sink.is_none() {
            let sink = Sink::try_new(&self.output_handle);
            match sink {
                Ok(s) => self.sink = Some(s),
                Err(e) => eprintln!("{:?}", e)
            }
        }

        self.sink.as_ref().map(|ref sink| {
            sink.set_volume(volume);
            sink.append(source);
        });


        Some(())
    }
}

