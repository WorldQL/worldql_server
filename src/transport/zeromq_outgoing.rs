use std::collections::HashMap;

use color_eyre::Result;
use futures_util::SinkExt;
use tmq::push::Push;
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::{debug, info, trace};
use uuid::Uuid;

use crate::structures::{DecodeError, Instruction, Message};

pub type ZmqOutgoingMessagePair = (Vec<u8>, Uuid);

/// This exists because ZeroMQ Push sockets don't exist.
/// It would be very nice to store them in [PeerConnection] like we do for WebSocket.
/// Unfortunately, we cannot without wrapping them in a mutex which isn't a good solution.
pub async fn start_zeromq_outgoing(
    mut msg_rx: UnboundedReceiver<ZmqOutgoingMessagePair>,
    ctx: tmq::Context,
) -> Result<()> {
    info!("Started ZeroMQ PUSH Manager");

    // TODO: Rework this entire function. This is just a quick and dirty approach. I need to figure out a way to pass this handshakes but also pass it bytes for outgoing messages.
    let mut zeromq_peer_map: HashMap<Uuid, Push> = HashMap::new();
    let zeromq_server_uuid = Uuid::new_v4(); // used for outgoing handshake.


    loop {
        let message = msg_rx.recv().await;
        if message.is_none() {
            // Channel is closed, should exit thread.
            break;
        }

        let (bytes, uuid) = message.unwrap();
        // TODO: Handle

        // //region: Handle incoming handshakes
        // // This is NOT an outgoing message.
        // // This is here because the push sockets need to be created here.
        // if message.instruction == Instruction::Handshake && !message.parameter.is_none() {
        //     let parameter = message.parameter.ok_or_else(|| DecodeError::MissingRequiredField("parameter".into()))?;
        //     let endpoint = format!("tcp://{}", parameter);
        //     trace!("endpoint = {}", endpoint);

        //     let mut new_push_socket = tmq::push(&ctx).connect(&endpoint).unwrap();
        //     let outgoing_message = Message {
        //         instruction: Instruction::Handshake,
        //         parameter: Some("It worked!".into()),
        //         sender_uuid: zeromq_server_uuid,
        //         ..Default::default()
        //     };

        //     zeromq_peer_map.insert(message.sender_uuid, new_push_socket);
        //     debug!("Added new peer at {} to map", endpoint);

        //     let outgoing_socket = zeromq_peer_map.get_mut(&message.sender_uuid);
        //     trace!("attempting to send outgoing message to {}", message.sender_uuid);
        //     // TODO: Can we avoid waiting for this?? Might slow down the replies.
        //     outgoing_socket
        //         .unwrap()
        //         .send(vec![outgoing_message.serialize()])
        //         .await;

        //     continue;
        // }
        // //endregion

        // // Otherwise, broadcast.
        // // TODO: Remove this and work into PeerConnection into some way that isn't crazy redundant.
        // for (connected_uuid, socket) in zeromq_peer_map.iter_mut() {
        //     if connected_uuid != &message.sender_uuid {
        //         socket
        //             .send(vec![message.clone().serialize()])
        //             .await;
        //     }
        // }
    }

    Ok(())
}
