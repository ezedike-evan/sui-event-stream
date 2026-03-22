use sui_event_stream::source::{CheckpointEventSource};
use sui_event_stream::types::EventFilter;
use sui_event_stream::emit::emit_events;
use sui_rpc::Client;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    println!("Starting sui-event-stream pipeline...");

    let client = Client::new("https://fullnode.mainnet.sui.io:443").unwrap();
    let mut source = CheckpointEventSource::new(client);

    let filter = EventFilter {
        package_id: Some("0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809".to_string()),
        module: None,
        event_type: None,
    };

    let (tx, mut rx) = mpsc::channel::<sui_event_stream::types::RawEvent>(100);

    tokio::spawn(async move {
        emit_events(&mut source, filter, tx).await;
    });

    while let Some(event) = rx.recv().await {
        println!("DeepBook event: {}", event.event_type);
        println!("  Package:    {}", event.package_id);
        println!("  Module:     {}", event.module);
        println!("  Checkpoint: {}", event.checkpoint_sequence_number);
        println!("---");
    }
}