use sui_rpc::Client;
use sui_rpc::proto::sui::rpc::v2::SubscribeCheckpointsResponse;
use sui_rpc::proto::sui::rpc::v2::SubscribeCheckpointsRequest;
use sui_rpc::proto::sui::rpc::v2::GetCheckpointRequest;
use sui_rpc::proto::sui::rpc::v2::get_checkpoint_request::CheckpointId;
use tonic::Streaming;
use crate::types::RawEvent;
use async_trait::async_trait;
use futures_util::StreamExt;
use prost_types::FieldMask;

#[async_trait]
pub trait EventSource: Send {
    async fn next_events(&mut self) -> Vec<RawEvent>;
}

pub struct CheckpointEventSource {
    client: Client,
    stream: Option<Streaming<SubscribeCheckpointsResponse>>,
}

impl CheckpointEventSource {
    pub fn new(client: Client) -> Self {
        Self { 
            client,
            stream: None,
        }
    }
}

#[async_trait]
impl EventSource for CheckpointEventSource {
    async fn next_events(&mut self) -> Vec<RawEvent> {
        if self.stream.is_none() {
            let mut subscription = self.client.subscription_client();
            match subscription.subscribe_checkpoints(SubscribeCheckpointsRequest::default()).await{
                Ok(r) => self.stream = Some(r.into_inner()),
                Err(e) => {
                    println!("Subscription error: {}", e);
                    return Vec::new();
                }
            }
        }
        let seq = match self.stream.as_mut().unwrap().next().await {
            Some(Ok(response)) => match response.cursor {
                Some(s) => s,
                None => return Vec::new(),
            },
            _ => {
                self.stream = None;
                return Vec::new();
            }
        };

        let mut mask = FieldMask::default();
        mask.paths = vec![
            "sequence_number".to_string(),
            "transactions.digest".to_string(),
            "transactions.events".to_string(),
            "transactions.timestamp".to_string(),
        ];

        let mut checkpoint_request = GetCheckpointRequest::default();
        checkpoint_request.checkpoint_id = Some(CheckpointId::SequenceNumber(seq));
        checkpoint_request.read_mask = Some(mask);

        let mut ledger = self.client.ledger_client();

        let transactions = match ledger.get_checkpoint(checkpoint_request).await {
            Ok(resp) => resp
                .into_inner()
                .checkpoint
                .map(|c| c.transactions)
                .unwrap_or_default(),
            Err(e) => {
                println!("GetCheckpoint error: {}", e);
                return Vec::new();
            }
        };

        let mut events = Vec::new();

        for tx in transactions {
            if let Some(event_list) = tx.events {
                for event in event_list.events {
                    events.push(RawEvent {
                        checkpoint_sequence_number: seq,
                        package_id: event.package_id.unwrap_or_default(),
                        module: event.event_type
                            .as_ref()
                            .and_then(|t| t.split("::").nth(1))
                            .unwrap_or_default()
                            .to_string(),
                        event_function: event.event_type
                            .as_ref()
                            .and_then(|t| t.split("::").nth(2))
                            .unwrap_or_default()
                            .to_string(),
                        event_type: event.event_type.unwrap_or_default(),
                        contents: event.contents
                            .and_then(|c| c.value)
                            .map(|b| b.to_vec())
                            .unwrap_or_default(),
                    });
                }
            }
        }

        events
    }
}