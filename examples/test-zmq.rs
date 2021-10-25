use std::time::Duration;

use futures::SinkExt;
use tokio::time::sleep;

use std::env;
use tmq::{push, Context, Result};

use uuid::{Builder, Uuid};

use log::{info, trace, warn, error};

extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[path = "../src/generated_flatbuffer/mod.rs"]
mod generated_flatbuffer;
pub use generated_flatbuffer::worldql_fb::messages::{Message, MessageArgs};
extern crate flexbuffers;
use flexbuffers::{BitWidth, Builder as FlexBuilder, Reader, ReaderError};

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    info!("Connecting as {}", uuid);

    let mut flex_builder = FlexBuilder::default();
    let mut playerProps = flex_builder.start_map();
    playerProps.push("pitch", 56);
    playerProps.push("yaw", 44);
    playerProps.end_map();

    let data = flex_builder.view();


    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(128);
    let instruction = builder.create_string("ZeroMQHandshake");
    let sender_uuid = builder.create_string(&*uuid);
    let flex = builder.create_vector(data);

    let message = Message::create(&mut builder, &MessageArgs {
        instruction: Some(instruction),
        sender_uuid: Some(sender_uuid),
        flex: Some(flex),
        ..Default::default()
    });

    builder.finish(message, None);

    let buf = builder.finished_data();


    let mut socket = push(&Context::new()).connect("tcp://127.0.0.1:5555")?;

    let mut i = 0;
    loop {
        let message = Uuid::new_v4().to_hyphenated().to_string();
        i += 1;
        println!("Push: {}", message);
        socket.send(tmq::Message::from(buf)).await?;
        sleep(Duration::from_millis(1000)).await;
    }
}
