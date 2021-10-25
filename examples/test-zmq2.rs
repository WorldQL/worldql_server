use std::time::Duration;

use futures::SinkExt;
use tokio::time::sleep;

use std::env;
use tmq::{push, Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "debug");
    }

    let mut socket = push(&Context::new()).connect("tcp://127.0.0.1:5555")?;

    let mut i = 0;
    loop {
        let message = format!("Push #{}", i);
        i += 1;

        println!("Push: {}", message);
        let multipart = vec![message.as_bytes()];
        socket.send(message.as_bytes()).await?;
        sleep(Duration::from_millis(1000)).await;
    }
}
