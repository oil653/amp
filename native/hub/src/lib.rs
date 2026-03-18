use rinf::{dart_shutdown, write_interface};
use tokio::spawn;

mod modules;
use crate::modules::create_actors;

write_interface!();

#[tokio::main(flavor = "current_thread")]
async fn main() {
    spawn(create_actors());
    dart_shutdown().await;
}
