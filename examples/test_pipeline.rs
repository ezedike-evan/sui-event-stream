use sui_event_stream::types::EventFilter;
use sui_rpc::Client;

#[tokio::main]
async fn main() {
    println!("Starting sui-event-stream pipeline...");

    let client = Client::new("https://fullnode.mainnet.sui.io:443").unwrap();

    let filter = EventFilter {
        package_id: Some("0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809".to_string()),
        module: Some("order".to_string()),
        event_type: None
    };

    let mut rx = sui_event_stream::subscribe(client, filter).await;

    while let Some(event) = rx.recv().await {
        println!("DeepBook event: {}", event.event_type);
        println!("  Package:    {}", event.package_id);
        println!("  Module:     {}", event.module);
        println!("  Function:   {}", event.event_function);
        println!("  Checkpoint: {}", event.checkpoint_sequence_number);
        println!("---");
    }
}