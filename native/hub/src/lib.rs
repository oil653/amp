use rinf::{dart_shutdown, write_interface};
use tokio::spawn;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;


write_interface!();

#[tokio::main(flavor = "current_thread")]
async fn main() {
    spawn(create_actors());
    dart_shutdown().await;
}

/// Create the actors
async fn create_actors() {
  // The
}