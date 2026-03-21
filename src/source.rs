use tonic::Streaming;

use sui_rpc::proto::sui::rpc::v2::{
    SubscribeCheckpointsRequest,
    SubscribeCheckpointsResponse,
    subscription_service_client::SubscriptionServiceClient,
};

use crate::{EventSource, RawEvent};

/// Implements EventSource by consuming Sui's checkpoint stream
/// and extracting events from each checkpoint as it arrives.
pub struct CheckpointEventSource {
    stream: Streaming<SubscribeCheckpointsResponse>,
}

impl CheckpointEventSource {
    /// Connect to a Sui full node and open the checkpoint stream.
    ///
    /// Use the constants from the crate root:
    /// `CheckpointEventSource::connect(sui_event_stream::MAINNET_URL).await`
    pub async fn connect(url: &str) -> Result<Self, tonic::transport::Error> {
        let mut client = SubscriptionServiceClient::connect(url.to_string()).await?;

        let stream = client
            .subscribe_checkpoints(SubscribeCheckpointsRequest::default())
            .await
            .expect("failed to subscribe to checkpoints")
            .into_inner();

        Ok(Self { stream })
    }
}

impl EventSource for CheckpointEventSource {
    async fn next_events(&mut self) -> Vec<RawEvent> {
        // Wait for the next checkpoint from the stream.
        // Returns empty vec if the stream ends or errors.
        let response = match self.stream.message().await {
            Ok(Some(r)) => r,
            _ => return vec![],
        };

        let checkpoint_seq = response.cursor();

        let checkpoint = match response.checkpoint_opt() {
            Some(c) => c,
            None => return vec![],
        };

        // Walk every transaction in the checkpoint,
        // extract every event from each transaction.
        let mut raw_events = Vec::new();

        for tx in checkpoint.transactions() {
            let tx_events = match tx.events_opt() {
                Some(e) => e,
                None => continue,
            };

            for event in &tx_events.events {
                let package_id = match &event.package_id {
                    Some(p) => p.clone(),
                    None => continue,
                };
                let module = match &event.module {
                    Some(m) => m.clone(),
                    None => continue,
                };
                let event_type = match &event.event_type {
                    Some(t) => t.clone(),
                    None => continue,
                };
                let contents = match &event.contents {
                    Some(bcs) => bcs.value().to_vec(),
                    None => vec![],
                };

                raw_events.push(RawEvent {
                    checkpoint_sequence_number: checkpoint_seq,
                    package_id,
                    module,
                    event_type,
                    contents,
                });
            }
        }

        raw_events
    }
}