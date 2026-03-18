use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, DartSignal, Deserialize, SignalPiece, Serialize)]
pub enum Playback {
    /// No media is loaded, and nothing is available to be played
    #[default]
    Stopped,
    /// When media is playing, and can be paused or stopped
    Playing,
    /// When the media is paused, but can be stopped or started
    Paused,
}
impl Playback {
    pub fn send_to_dart(&self) {
        PlaybackResponse {
            playback: self.clone(),
        }
        .send_signal_to_dart();
    }
}

#[derive(Debug, Clone, RustSignal, Serialize)]
pub struct PlaybackResponse {
    playback: Playback,
}

#[derive(Debug, RustSignal, Serialize, Clone)]
pub struct Queue {
    played: Vec<Track>,
    playing: Option<Track>,
    will_play: Vec<Track>,
}
impl Queue {
    pub fn new(played: Vec<Track>, playing: Option<Track>, will_play: Vec<Track>) -> Self {
        Self {
            played,
            playing,
            will_play,
        }
    }
}
impl Default for Queue {
    fn default() -> Self {
        Self {
            played: Vec::new(),
            playing: None,
            will_play: Vec::new(),
        }
    }
}

/// A piece of media that can be played
#[derive(Debug, SignalPiece, Serialize, Clone)]
pub struct Track {
    title: Option<String>,
    artists: Option<Vec<String>>,
    album: Option<String>,
    release_year: Option<i32>,
}
