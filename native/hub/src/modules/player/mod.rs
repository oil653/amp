use messages::{actor::Actor, prelude::Address};
use rinf::DartSignal;
use rodio::{DeviceSinkBuilder, Player as RP};
use tokio::task::JoinSet;

use crate::modules::media_manager::{MediaManager, signals::Playback};

pub struct Player {
    _owned_tasks: JoinSet<()>,
    player: RP,
}
impl Actor for Player {}

impl Player {
    pub fn new(manager_address: Address<MediaManager>) -> Self {
        let mut owned_tasks = JoinSet::new();
        owned_tasks.spawn(Self::listen_for_playback_change(manager_address));

        let device_sink = DeviceSinkBuilder::open_default_sink().unwrap();
        let player = RP::connect_new(device_sink.mixer());

        Self {
            _owned_tasks: owned_tasks,
            player: player,
        }
    }

    async fn listen_for_playback_change(mut manager_address: Address<MediaManager>) {
        let rcx = Playback::get_dart_signal_receiver();
        while let Some(pack) = rcx.recv().await {
            let _ = manager_address.notify(pack.message).await;
        }
    }
}
