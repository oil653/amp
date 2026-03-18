use messages::prelude::Context;
use tokio::spawn;

use crate::modules::{media_manager::MediaManager, player::Player};

mod media_manager;
mod player;

/// Create the actors
pub async fn create_actors() {
    // Player
    let player_context = Context::new();
    let player_address = player_context.address();

    let mm_context = Context::new();
    let mm_address = mm_context.address();

    let player = Player::new(mm_address.clone());
    let mm = MediaManager::new(player_address);

    spawn(mm_context.run(mm));
    spawn(player_context.run(player));
}
