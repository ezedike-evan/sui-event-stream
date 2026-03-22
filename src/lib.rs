pub mod types;
pub mod source;
pub mod filter;
pub mod emit;

use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};
use sui_rpc::Client;

/// Subscribe to filtered on-chain Sui events in real time.
///
/// Connects to the Sui network via the provided client, streams checkpoints,
/// extracts events and delivers only those matching the provided filter
/// through an unbounded async channel.
///
/// # Example
///
/// ```no_run
/// use sui_event_stream::types::EventFilter;
/// use sui_rpc::Client;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("https://fullnode.mainnet.sui.io:443").unwrap();
///
///     let filter = EventFilter {
///         package_id: Some("0x2c8d...".to_string()),
///         module: None,
///         event_type: None,
///     };
///
///     let mut rx = sui_event_stream::subscribe(client, filter).await;
///
///     while let Some(event) = rx.recv().await {
///         println!("Event: {}", event.event_type);
///     }
/// }
/// ```

pub async fn subscribe(client: Client, filter: types::EventFilter) -> UnboundedReceiver<types::RawEvent> {
    let (tx, rx) = unbounded_channel::<types::RawEvent>();
    let mut source = source::CheckpointEventSource::new(client);

    tokio::spawn(async move {
        emit::emit_events(&mut source, filter, tx).await;
    });
    rx
}