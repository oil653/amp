use std::{error, time::Duration};

use async_trait::async_trait;
use messages::{
    actor::Actor,
    prelude::{Address, Context, Handler, Notifiable},
};
use rinf::{DartSignal, debug_print};
use rodio::{
    DeviceSinkBuilder, DeviceSinkError, MixerDeviceSink, Player as RP, Source, source::SineWave,
};
use thiserror::Error;
use tokio::task::JoinSet;

pub mod types;

use crate::modules::{
    media_manager::{MediaManager, signals::Playback},
    player::types::Load,
};

#[derive(Debug, Error)]
#[allow(unused)]
pub enum PlayerManagerError {
    #[error("Failed to load media from path: {0}")]
    LoadError(String),
    #[error("No current player to load media to.")]
    NoPlayer,
}

pub struct PlayerManager {
    _owned_tasks: JoinSet<()>,
    player: Option<Player>,
}
impl Actor for PlayerManager {}

impl PlayerManager {
    pub fn new(self_address: Address<Self>, manager_address: Address<MediaManager>) -> Self {
        let mut owned_tasks = JoinSet::new();
        owned_tasks.spawn(Self::listen_for_playback_change(manager_address));

        let mut this = Self {
            _owned_tasks: owned_tasks,
            player: None,
        };

        let player = Player::new_default_sink(Box::new(move || {
            Self::clear_player(self_address.clone());
        }))
        .ok();

        this.player = player;
        this
    }

    async fn listen_for_playback_change(mut manager_address: Address<MediaManager>) {
        let rcx = Playback::get_dart_signal_receiver();
        while let Some(pack) = rcx.recv().await {
            let _ = manager_address.notify(pack.message).await;
        }
    }

    fn can_play(&self) -> bool {
        self.player
            .as_ref()
            .map_or(false, |player| player.player.len() > 0)
    }

    fn clear_player(self_address: Address<Self>) {
        debug_print!("@@@ clear_player called");
    }
}

#[async_trait]
impl Handler<Load> for PlayerManager {
    type Result = Result<(), PlayerManagerError>;

    async fn handle(&mut self, input: Load, _: &Context<Self>) -> Self::Result {
        // debug_print!("@@@ PlayerManager: loading file");
        let player = self.player.as_ref().ok_or(PlayerManagerError::NoPlayer)?;
        player.player.clear();
        // TODO: Load the audio file
        let source = SineWave::new(440.0).take_duration(Duration::from_secs(10));
        player.player.append(source);
        // debug_print!("@@@ PlayerManager: loaded file");
        Ok(())
    }
}

#[async_trait]
impl Notifiable<Playback> for PlayerManager {
    async fn notify(&mut self, input: Playback, _: &Context<Self>) {
        let player = match &self.player {
            Some(p) => p,
            None => return,
        };
        use Playback::*;
        match input {
            Stopped => player.player.clear(),
            Paused => player.player.pause(),
            Playing => player.player.play(),
        }
    }
}
// TODO: Add handle to when the sink is dropped.

/// A player object that implemetns a way to tell the manager that it's dropped and run a Fn when it happens.
struct Player {
    sink: MixerDeviceSink, // Inner type
    pub player: RP,
    on_drop: Option<Box<dyn FnOnce() + Send + Sync>>,
}
impl Player {
    pub fn new(sink: MixerDeviceSink, on_drop: Box<dyn FnOnce() + Send + Sync>) -> Self {
        let player = RP::connect_new(sink.mixer());
        Self {
            sink,
            player,
            on_drop: Some(on_drop),
        }
    }

    /// Attempts to create a new instance by using the default sink of the OS.
    pub fn new_default_sink(
        on_drop: Box<dyn FnOnce() + Send + Sync>,
    ) -> Result<Self, DeviceSinkError> {
        Ok(Self::new(DeviceSinkBuilder::open_default_sink()?, on_drop))
    }
}
impl Drop for Player {
    fn drop(&mut self) {
        if let Some(f) = self.on_drop.take() {
            f();
        }
    }
}
