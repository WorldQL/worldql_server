use std::fmt::Display;

use super::{Decode, DecodeError, Encode};
use crate::flatbuffers::Instruction as InstructionFB;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    Heartbeat,
    Handshake,
    PeerConnect,
    PeerDisconnect,
    AreaSubscribe,
    AreaUnsubscribe,
    GlobalMessage,
    LocalMessage,
    RecordCreate,
    RecordRead,
    RecordUpdate,
    RecordDelete,
    RecordReply,

    Unknown,
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Unknown
    }
}

// region: Codec Traits
impl Encode<InstructionFB> for Instruction {
    #[inline]
    fn encode(self) -> InstructionFB {
        match self {
            Instruction::Heartbeat => InstructionFB::Heartbeat,
            Instruction::Handshake => InstructionFB::Handshake,
            Instruction::PeerConnect => InstructionFB::PeerConnect,
            Instruction::PeerDisconnect => InstructionFB::PeerDisconnect,
            Instruction::AreaSubscribe => InstructionFB::AreaSubscribe,
            Instruction::AreaUnsubscribe => InstructionFB::AreaUnsubscribe,
            Instruction::GlobalMessage => InstructionFB::GlobalMessage,
            Instruction::LocalMessage => InstructionFB::LocalMessage,
            Instruction::RecordCreate => InstructionFB::RecordCreate,
            Instruction::RecordRead => InstructionFB::RecordRead,
            Instruction::RecordUpdate => InstructionFB::RecordUpdate,
            Instruction::RecordDelete => InstructionFB::RecordDelete,
            Instruction::RecordReply => InstructionFB::RecordReply,

            Instruction::Unknown => InstructionFB::Unknown,
        }
    }
}

impl Decode<InstructionFB> for Instruction {
    #[inline]
    fn decode(encoded: InstructionFB) -> Result<Self, DecodeError> {
        let instruction = match encoded {
            InstructionFB::Heartbeat => Instruction::Heartbeat,
            InstructionFB::Handshake => Instruction::Handshake,
            InstructionFB::PeerConnect => Instruction::PeerConnect,
            InstructionFB::PeerDisconnect => Instruction::PeerDisconnect,
            InstructionFB::AreaSubscribe => Instruction::AreaSubscribe,
            InstructionFB::AreaUnsubscribe => Instruction::AreaUnsubscribe,
            InstructionFB::GlobalMessage => Instruction::GlobalMessage,
            InstructionFB::LocalMessage => Instruction::LocalMessage,
            InstructionFB::RecordCreate => Instruction::RecordCreate,
            InstructionFB::RecordRead => Instruction::RecordRead,
            InstructionFB::RecordUpdate => Instruction::RecordUpdate,
            InstructionFB::RecordDelete => Instruction::RecordDelete,
            InstructionFB::RecordReply => Instruction::RecordReply,

            _ => Instruction::Unknown,
        };

        Ok(instruction)
    }
}
// endregion

// region: Display Trait
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Heartbeat => "Heartbeat",
            Self::Handshake => "Handshake",
            Self::PeerConnect => "PeerConnect",
            Self::PeerDisconnect => "PeerDisconnect",
            Self::AreaSubscribe => "AreaSubscribe",
            Self::AreaUnsubscribe => "AreaUnsubscribe",
            Self::GlobalMessage => "GlobalMessage",
            Self::LocalMessage => "LocalMessage",
            Self::RecordCreate => "RecordCreate",
            Self::RecordRead => "RecordRead",
            Self::RecordUpdate => "RecordUpdate",
            Self::RecordDelete => "RecordDelete",
            Self::RecordReply => "RecordReply",

            Self::Unknown => "Unknown",
        };

        write!(f, "{}", name)
    }
}
// endregion
