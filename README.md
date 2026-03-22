# sui-event-stream

A real-time event streaming library for Sui's DeepBook V3 and any on-chain Sui package.

## The Problem

Sui's JSON-RPC WebSocket event subscription is deprecated with no gRPC equivalent yet. 
`sui-event-stream` fills that gap by building on top of Sui's gRPC checkpoint stream 
and exposing a clean, simple API for subscribing to filtered on-chain events.

## Usage

```toml
[dependencies]
sui-event-stream = "0.1.0"
```

```rust
use sui_event_stream::types::EventFilter;
use sui_rpc::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("https://fullnode.mainnet.sui.io:443").unwrap();

    let filter = EventFilter {
        package_id: Some("0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809".to_string()),
        module: None,
        event_type: None,
    };

    let mut rx = sui_event_stream::subscribe(client, filter).await;

    while let Some(event) = rx.recv().await {
        println!("Event: {}", event.event_type);
    }
}
```

## How It Works

Three clean layers:

- **Layer 1 — Source**: Subscribes to Sui's gRPC checkpoint stream and extracts all events from each checkpoint
- **Layer 2 — Filter**: Keeps only events matching your filter criteria
- **Layer 3 — Emit**: Delivers matching events through an unbounded async channel

## EventFilter

All fields are optional. Unset fields match everything.

| Field | Type | Description |
|-------|------|-------------|
| `package_id` | `Option<String>` | Filter by the package that emitted the event |
| `module` | `Option<String>` | Filter by the defining module |
| `event_type` | `Option<String>` | Filter by the full event type string |
| `function` | `Option<String>` | Filter by the function that emitted the event |

## RawEvent

Each event you receive contains:

| Field | Type | Description |
|-------|------|-------------|
| `checkpoint_sequence_number` | `u64` | The checkpoint this event came from |
| `package_id` | `String` | The package that emitted this event |
| `module` | `String` | The defining module |
| `function` | `String` | The function that emitted this event |
| `event_type` | `String` | Full type string e.g. `0x2c8d...::order::OrderFilled` |
| `contents` | `Vec<u8>` | Raw BCS-encoded event payload |

## The Road Ahead

The current implementation sources events from Sui's checkpoint stream — adding ~200-400ms 
latency from the checkpoint batching interval. A future execution-level implementation 
would hook directly into the full node pipeline, delivering events before checkpointing 
for significantly lower latency. See the research paper for the full architectural proposal.

## License

MIT