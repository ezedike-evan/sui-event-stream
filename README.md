# sui-event-stream

A Rust crate for subscribing to filtered Sui on-chain events in real time over gRPC.

Sui deprecated `suix_subscribeEvent` (JSON-RPC WebSocket) in July 2024. The gRPC
replacement — `SubscriptionService` — currently only exposes `SubscribeCheckpoints`.
This crate bridges the gap by consuming the checkpoint stream and re-streaming
filtered events via a `SubscribeEvents` gRPC endpoint, mirroring Sui's own
`SubscribeCheckpoints` conventions.

## Status

🚧 Under active development

## Architecture

- **Layer 1 — EventSource**: Consumes `SubscribeCheckpoints` via `sui-rpc`
- **Layer 2 — Filter**: Filters events by package ID, module, event type
- **Layer 3 — Emit**: Streams matching events via gRPC server-side streaming

## License

MIT