use std::collections::HashMap;

use color_eyre::Result;
use futures_util::SinkExt;
use tmq::push;
use tmq::push::Push;
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::info;
use uuid::Uuid;

use crate::structures::{DecodeError, Instruction, Message};

/// Used to attach the destination of a message in the UnboundedReceiver of Messages
pub struct MessageAndClientUUID {
    pub(crate) message: Message,
    pub(crate) client: Uuid,
}

/// This exists because ZeroMQ Push sockets don't exist.
/// It would be very nice to store them in [PeerConnection] like we do for WebSocket.
/// Unfortunately, we cannot without wrapping them in a mutex which isn't a good solution.
pub async fn start_outgoing_zeromq_thread(
    mut msg_rx: UnboundedReceiver<MessageAndClientUUID>,
    ctx: tmq::Context,
) -> Result<()> {
    // TODO: Rework this entire function. This is just a quick and dirty approach. I need to figure out a way to pass this handshakes but also pass it bytes for outgoing messages.
    let mut zeromq_peer_map: HashMap<Uuid, Push> = HashMap::new();
    let mut connected_uuids = vec![]; // TODO: Remove.
    let zeromq_server_uuid = Uuid::new_v4(); // used for outgoing handshake.
    loop {
        let message = msg_rx.recv().await;
        if message.is_none() {
            continue;
        }
        let mc = message.unwrap();

        println!("{:?}", mc.message.instruction);

        //region: Handle incoming handshakes
        // This is NOT an outgoing message.
        // This is here because the push sockets need to be created here.
        if mc.message.instruction == Instruction::Handshake && !mc.message.parameter.is_none() {
            let endpoint = "tcp://".to_owned()
                + &mc.message.parameter.ok_or_else(|| {
                    DecodeError::MissingRequiredField("parameter on Handshake".into())
                })?;
            println!("{}", endpoint);
            let mut new_push_socket = push(&ctx).connect(&*endpoint).unwrap();
            let outgoing_message = Message {
                instruction: Instruction::Handshake,
                parameter: Some("It worked!".parse()?),
                sender_uuid: zeromq_server_uuid,
                world_name: None,
                records: vec![],
                entities: vec![],
                position: None,
                flex: None,
            };

            zeromq_peer_map.insert(mc.message.sender_uuid, new_push_socket);
            connected_uuids.push(mc.message.sender_uuid);
            info!("Added new peer at {} to map", endpoint);

            let outgoing_socket = zeromq_peer_map.get_mut(&mc.message.sender_uuid);
            info!("attempting to send outgoing message");
            // TODO: Can we avoid waiting for this?? Might slow down the replies.
            outgoing_socket
                .unwrap()
                .send(vec![outgoing_message.serialize()])
                .await;

            continue;
        }
        //endregion

        // Otherwise, broadcast.
        // TODO: Remove this and work into PeerConnection into some way that isn't crazy redundant.
        for connected_uuid in &connected_uuids {
            if connected_uuid != &mc.message.sender_uuid {
                let outgoing_socket = zeromq_peer_map.get_mut(&connected_uuid);
                outgoing_socket
                    .unwrap()
                    .send(vec![mc.message.clone().serialize()])
                    .await;
            }
        }
    }

    Ok(())
}
