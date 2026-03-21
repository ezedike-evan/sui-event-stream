mod source;

/// The gRPC endpoint for Sui mainnet.
pub const MAINNET_URL: &str = "https://fullnode.mainnet.sui.io";

/// The gRPC endpoint for Sui testnet.
pub const TESTNET_URL: &str = "https://fullnode.testnet.sui.io";

/// The gRPC endpoint for Sui devnet.
pub const DEVNET_URL: &str = "https://fullnode.devnet.sui.io";

/// A single raw event extracted from a checkpoint.
#[derive(Debug, Clone)]
pub struct RawEvent {
    /// The checkpoint this event came from.
    pub checkpoint_sequence_number: u64,
    /// The package that emitted this event.
    pub package_id: String,
    /// The Move module that emitted this event.
    pub module: String,
    /// The full Move type string e.g. `0x2c8d...::order::OrderFilled`.
    pub event_type: String,
    /// The raw BCS-encoded event payload.
    pub contents: Vec<u8>,
}


/// A source of raw events from the Sui blockchain.
///
/// Implementors pull events from wherever they can —
/// currently from the checkpoint stream, in future
/// potentially from the execution pipeline directly.
pub trait EventSource {
    /// Return the next batch of events.
    /// Blocks until at least one checkpoint arrives.
    async fn next_events(&mut self) -> Vec<RawEvent>;
}