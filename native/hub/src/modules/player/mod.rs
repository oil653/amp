use messages::{actor::Actor, prelude::Address};
use rinf::{DartSignal, debug_print};
use rodio::{
    DeviceSinkBuilder, DeviceSinkError, MixerDeviceSink, Player as RP, speakers::available_outputs,
};
use tokio::task::JoinSet;

use crate::modules::media_manager::{MediaManager, signals::Playback};

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

    fn clear_player(self_address: Address<Self>) {
        debug_print!("@@@ clear_player called");
    }
}

/// A player object that implemetns a way to tell the manager that it's dropped.
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
