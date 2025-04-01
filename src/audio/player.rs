use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

pub struct AudioPlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Option<Sink>,
    current_file: Option<String>,
    state: PlaybackState,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        Ok(Self {
            stream,
            stream_handle,
            sink: None,
            current_file: None,
            state: PlaybackState::Stopped,
        })
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        
        let sink = Sink::try_new(&self.stream_handle)?;
        sink.append(source);
        self.sink = Some(sink);
        self.current_file = Some(path.as_ref().to_string_lossy().into_owned());
        self.state = PlaybackState::Stopped;
        Ok(())
    }

    pub fn play(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
            self.state = PlaybackState::Playing;
        }
    }

    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
            self.state = PlaybackState::Paused;
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
            self.state = PlaybackState::Stopped;
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        if let Some(sink) = &self.sink {
            sink.set_volume(volume);
        }
    }

    pub fn get_state(&self) -> PlaybackState {
        self.state
    }

    pub fn get_current_file(&self) -> Option<&str> {
        self.current_file.as_deref()
    }
}
