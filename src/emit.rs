use crate::source::EventSource;
use crate::types::EventFilter;
use crate::types::RawEvent;
use crate::filter::apply_filter;
use tokio::sync::mpsc::UnboundedSender;

pub async fn emit_events(
    source: &mut dyn EventSource,
    filter: EventFilter,
    tx: UnboundedSender<RawEvent>,
) {
    loop {
        let events = source.next_events().await;
        let filtered = apply_filter(events, &filter);

        for event in filtered {
            if tx.send(event).is_err() {
                println!("Consumer disconnected, stopping emit");
                return;
            }
        }
    }
}