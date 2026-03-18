use async_trait::async_trait;
use messages::{
    actor::Actor,
    prelude::{Address, Context, Notifiable},
};
use rinf::{RustSignal, debug_print};
use tokio::task::JoinSet;

pub mod signals;
use crate::modules::{
    media_manager::signals::{Playback, Queue},
    player::Player,
};

/// A queue manages the music playing, and up to be played.
pub struct MediaManager {
    _owned_tasks: JoinSet<()>,

    queue: Queue,
    playback: Playback,
}
impl Actor for MediaManager {}

impl MediaManager {
    pub fn new(_player_address: Address<Player>) -> Self {
        let owned_tasks = JoinSet::new();
        // TODO: restore session

        // Sends the initial value (containing nothing)
        let queue = Queue::default();
        queue.send_signal_to_dart();

        Playback::Stopped.send_to_dart();

        Self {
            _owned_tasks: owned_tasks,
            queue,
            playback: Playback::Stopped,
        }
    }
}

#[async_trait]
impl Notifiable<Playback> for MediaManager {
    async fn notify(&mut self, input: Playback, _: &Context<Self>) {
        debug_print!("@@@ Mediamanager: playback state changed to: {:?}", input);
        input.send_to_dart();
        self.playback = input;
    }
}
