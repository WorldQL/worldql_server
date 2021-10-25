use std::fs::read;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use futures::StreamExt;
use tokio::task::JoinHandle;
use tmq::{pull, TmqError};
use log::{info, trace, warn, error, debug};

#[allow(dead_code, unused_imports)]
#[path = "./generated_flatbuffer/mod.rs"]
mod generated_flatbuffer;

pub use generated_flatbuffer::worldql_fb::messages::{Message, root_as_message};

extern crate flexbuffers;

use flexbuffers::{BitWidth, Builder as FlexBuilder, Reader, ReaderError};
use crate::connected_client::ConnectedClient;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::worldql_instruction::WorldQLInstruction;

pub fn start_main_server(zmq_context: tmq::Context) -> JoinHandle<Result<(), tmq::TmqError>> {
    tokio::spawn(async move {
        debug!("Starting main ZeroMQ PULL listener.");
        let mut port_counter = 29900;
        let mut zmq_pull_main_socket = pull(&zmq_context).bind("tcp://127.0.0.1:5555").unwrap();
        let mut zmq_client_push_sockets: Arc<RwLock<Vec<ConnectedClient>>> = Arc::new(RwLock::new(vec![]));

        while let Some(msg) = zmq_pull_main_socket.next().await {
            let m = msg?;
            let msg_vec = m.iter()
                .map(|item| item.deref())
                .collect::<Vec<&[u8]>>();

            if msg_vec.len() != 1 {
                panic!("Multipart ZeroMQ message received but not expected. If you are an end user please contact the WorldQL team.")
            }
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH).unwrap().as_millis();

            let mut instruction:WorldQLInstruction;

            let msg_bytes = msg_vec[0];
            // TODO: High prio! Add error handling if any of these value unwraps fail. Failure of these should just continue the loop.
            let message = root_as_message(msg_bytes).unwrap();

            match message.instruction().unwrap() {
                "ZeroMQHandshake" => instruction = WorldQLInstruction::ZeroMQHandshake,
                _ => instruction = WorldQLInstruction::Unknown,
            }

            if matches!(instruction, WorldQLInstruction::Unknown) {
                debug!("Received unknown instruction.");
                continue;
            }

            let flexbuffer = Reader::get_root(message.flex().unwrap()).unwrap();
            let read_props = flexbuffer.as_map();

            println!("{}", message.sender_uuid().unwrap());
            println!("{}", read_props.idx("pitch").as_i32())
        }
        Ok::<(), TmqError>(())
    })
}