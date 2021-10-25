pub enum WorldQLInstruction {
    /// Server-bound.
    Heartbeat,
    /// Server-bound
    ZeroMQHandshake,
    /// Client-bound
    ZeroMQPortAssignment,
    LocalMessage,
    GlobalMessage,
    /// Server-bound.
    RecordCreate,
    /// Server-bound.
    RecordUpdate,
    /// Server-bound.
    RecordDelete,
    /// Server-bound.
    RecordGet,
    /// Client-bound.
    RecordReply,
    /// Server-bound.
    AreaSubscribe,
    /// Server-bound.
    AreaUnsubscribe,
    Unknown,
}