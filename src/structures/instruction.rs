use super::{Decode, DecodeError, Encode};
use crate::flatbuffers::Instruction as InstructionFB;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    Heartbeat,
    Handshake,
    LocalMessage,
    GlobalMessage,
    RecordCreate,
    RecordRead,
    RecordUpdate,
    RecordDelete,
    RecordReply,
    AreaSubscribe,
    AreaUnsubscribe,

    Unknown,
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Encode<InstructionFB> for Instruction {
    #[inline]
    fn encode(self) -> InstructionFB {
        match self {
            Instruction::Heartbeat => InstructionFB::Heartbeat,
            Instruction::Handshake => InstructionFB::Handshake,
            Instruction::LocalMessage => InstructionFB::LocalMessage,
            Instruction::GlobalMessage => InstructionFB::GlobalMessage,
            Instruction::RecordCreate => InstructionFB::RecordCreate,
            Instruction::RecordRead => InstructionFB::RecordRead,
            Instruction::RecordUpdate => InstructionFB::RecordUpdate,
            Instruction::RecordDelete => InstructionFB::RecordDelete,
            Instruction::RecordReply => InstructionFB::RecordReply,
            Instruction::AreaSubscribe => InstructionFB::AreaSubscribe,
            Instruction::AreaUnsubscribe => InstructionFB::AreaUnsubscribe,
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
            InstructionFB::LocalMessage => Instruction::LocalMessage,
            InstructionFB::GlobalMessage => Instruction::GlobalMessage,
            InstructionFB::RecordCreate => Instruction::RecordCreate,
            InstructionFB::RecordRead => Instruction::RecordRead,
            InstructionFB::RecordUpdate => Instruction::RecordUpdate,
            InstructionFB::RecordDelete => Instruction::RecordDelete,
            InstructionFB::RecordReply => Instruction::RecordReply,
            InstructionFB::AreaSubscribe => Instruction::AreaSubscribe,
            InstructionFB::AreaUnsubscribe => Instruction::AreaUnsubscribe,

            _ => Instruction::Unknown,
        };

        Ok(instruction)
    }
}
