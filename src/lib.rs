pub mod types;
pub mod source;
pub mod filter;
pub mod emit;

use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};
use sui_rpc::Client;

pub async fn subscribe(client: Client, filter: types::EventFilter) -> UnboundedReceiver<types::RawEvent> {
    let (tx, rx) = unbounded_channel::<types::RawEvent>();
    let mut source = source::CheckpointEventSource::new(client);

    tokio::spawn(async move {
        emit::emit_events(&mut source, filter, tx).await;
    });
    rx
}