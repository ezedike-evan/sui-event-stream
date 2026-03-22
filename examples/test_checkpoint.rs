use sui_rpc::Client;
use sui_rpc::proto::sui::rpc::v2::SubscribeCheckpointsRequest;
use sui_rpc::proto::sui::rpc::v2::GetCheckpointRequest;
use sui_rpc::proto::sui::rpc::v2::get_checkpoint_request::CheckpointId;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    println!("Connecting...");

    let mut client = Client::new("https://fullnode.mainnet.sui.io:443").unwrap();
    let mut subscription = client.subscription_client();
    let request = SubscribeCheckpointsRequest::default();

    let mut stream = subscription
        .subscribe_checkpoints(request)
        .await
        .unwrap()
        .into_inner();

    if let Some(Ok(response)) = stream.next().await {
        let seq = response.cursor.unwrap();
        println!("Got checkpoint sequence: {}", seq);

        let mut ledger = client.ledger_client();
        let mut checkpoint_request = GetCheckpointRequest::default();
        checkpoint_request.checkpoint_id = Some(CheckpointId::SequenceNumber(seq));

        match ledger.get_checkpoint(checkpoint_request).await {
            Ok(resp) => {
                let checkpoint = resp.into_inner();
                println!("Transactions in checkpoint: {}", 
                    checkpoint.checkpoint
                        .as_ref()
                        .map(|c| c.transactions.len())
                        .unwrap_or(0)
                );
            }
            Err(e) => println!("GetCheckpoint failed: {}", e),
        }
    }
}