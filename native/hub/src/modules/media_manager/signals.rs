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
    pub played: Vec<Track>,
    pub playing: Option<Track>,
    pub will_play: Vec<Track>,
}
impl Queue {
    pub fn new(played: Vec<Track>, playing: Option<Track>, will_play: Vec<Track>) -> Self {
        Self {
            played,
            playing,
            will_play,
        }
    }

    pub fn clear(&mut self) {
        self.played.clear();
        self.will_play.clear();
        self.playing = None;
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
    path: String,
    title: Option<String>,
    artists: Vec<String>,
    album: Option<String>,
    release_year: Option<i32>,
}
impl Track {
    pub fn new(
        path: String,
        title: Option<String>,
        artists: Vec<String>,
        album: Option<String>,
        release_year: Option<i32>,
    ) -> Self {
        Self {
            path,
            title,
            artists,
            album,
            release_year,
        }
    }

    pub fn new_from_path(path: String) -> Self {
        // TODO: Read media tags
        // TODO: Validate MIME type

        Self {
            path,
            album: None,
            artists: Vec::new(),
            release_year: None,
            title: None,
        }
    }
}

#[derive(Debug, Clone, DartSignal, Deserialize)]
pub struct OpenMedia {
    pub file_path: String,
    pub action_type: OpenMediaAction,
}

#[derive(Debug, Clone, SignalPiece, Deserialize)]
pub enum OpenMediaAction {
    AddToQueue,
    ReplaceQueue,
}
