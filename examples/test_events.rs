use sui_event_stream::source::{CheckpointEventSource, EventSource};
use sui_rpc::Client;
use sui_event_stream::filter::apply_filter;
use sui_event_stream::types::EventFilter;

#[tokio::main]
async fn main() {
    println!("Connecting to Sui mainnet...");

    let client = Client::new("https://fullnode.mainnet.sui.io:443").unwrap();
    let mut source = CheckpointEventSource::new(client);

    loop {
        let events = source.next_events().await;
        println!("Checkpoint received — {} events", events.len());

        let filter = EventFilter {
            package_id: Some("0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809".to_string()),
            module: None,
            function: None,
            event_type: None,
        };

        let filtered = apply_filter(events, &filter);
        println!("DeepBook events this checkpoint: {}", filtered.len());

        for event in &filtered {
            println!("  {}", event.event_type);
        }
        
    }
}