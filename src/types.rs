/// A normalised on-chain Sui event extracted from a checkpoint.
#[derive(Debug, Clone)]
pub struct RawEvent {
    /// The checkpoint this event came from.
    pub checkpoint_sequence_number: u64,
    /// The package that emitted this event.
    pub package_id: String,
    /// The Move module that emitted this event.
    pub module: String,
    /// The function that emitted this event.
    pub event_function: String,
    /// The full Move type string e.g. `0x2c8d...::order::OrderFilled`.
    pub event_type: String,
    /// The raw BCS-encoded event payload.
    pub contents: Vec<u8>,
}


/// Filter criteria for selecting which on-chain events to receive.
/// All fields are optional — unset fields match everything.
#[derive(Debug, Clone)]
pub struct EventFilter {
    pub package_id: Option<String>,
    pub module: Option<String>,
    pub function: Option<String>,
    pub event_type: Option<String>,
}