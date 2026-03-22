use sui_event_stream::source::{CheckpointEventSource, EventSource};
use sui_rpc::Client;

#[tokio::main]
async fn main() {
    println!("Connecting to Sui mainnet...");

    let client = Client::new("https://fullnode.mainnet.sui.io:443").unwrap();
    let mut source = CheckpointEventSource::new(client);

    loop {
        let events = source.next_events().await;
        println!("Checkpoint received — {} events", events.len());

        for event in &events {
            println!("  {} :: {}", event.package_id, event.event_type);
        }
    }
}