use async_trait::async_trait;
use messages::{
    actor::Actor,
    prelude::{Address, Context, Notifiable},
};
use rinf::{DartSignal, RustSignal, debug_print};
use tokio::task::JoinSet;

pub mod signals;
use crate::modules::{
    media_manager::signals::{OpenMedia, OpenMediaAction, Playback, Queue, Track},
    player::{PlayerManager, types::Load},
};

/// A queue manages the music playing, and up to be played.
pub struct MediaManager {
    _owned_tasks: JoinSet<()>,

    queue: Queue,
    playback: Playback,
    player_manager_address: Address<PlayerManager>,
}
impl Actor for MediaManager {}

impl MediaManager {
    pub fn new(self_address: Address<Self>, player_address: Address<PlayerManager>) -> Self {
        let mut owned_tasks = JoinSet::new();
        owned_tasks.spawn(Self::listen_for_open_media(self_address.clone()));
        // TODO for later: restore session

        // Sends the initial value (containing nothing)
        let queue = Queue::default();
        queue.send_signal_to_dart();

        Playback::Stopped.send_to_dart();

        Self {
            _owned_tasks: owned_tasks,
            queue,
            playback: Playback::Stopped,
            player_manager_address: player_address,
        }
    }

    async fn listen_for_open_media(mut self_address: Address<Self>) {
        let rcx = OpenMedia::get_dart_signal_receiver();
        while let Some(media) = rcx.recv().await {
            let _ = self_address.notify(media.message).await;
        }
    }
}

#[async_trait]
impl Notifiable<Playback> for MediaManager {
    async fn notify(&mut self, input: Playback, _: &Context<Self>) {
        debug_print!("@@@ Mediamanager: playback state changed to: {:?}", input);
        input.send_to_dart();
        self.playback = input.clone();
        let _ = self.player_manager_address.notify(input).await;
    }
}

#[async_trait]
impl Notifiable<OpenMedia> for MediaManager {
    async fn notify(&mut self, media: OpenMedia, _: &Context<Self>) {
        debug_print!("@@@ Mediamanager: loading file: {}", media.file_path);
        match media.action_type {
            OpenMediaAction::AddToQueue => self
                .queue
                .will_play
                .push(Track::new_from_path(media.file_path)),
            OpenMediaAction::ReplaceQueue => {
                self.queue.clear();
                self.queue.playing = Some(Track::new_from_path(media.file_path.clone()));
                let _ = self
                    .player_manager_address
                    .send(Load {
                        path: media.file_path,
                    })
                    .await;
            }
        }
        self.queue.send_signal_to_dart();
        // debug_print!("@@@ Queue is now: {:#?}", self.queue)
        debug_print!("@@@ Mediamanager:: Loaded file!");
    }
}
