use crate::types::{RawEvent, EventFilter};

pub fn apply_filter(events: Vec<RawEvent>, filter: &EventFilter) -> Vec<RawEvent> {
    events.into_iter().filter(|event| {
        let package_match = filter.package_id
            .as_ref()
            .map(|p| event.package_id.starts_with(p))
            .unwrap_or(true);

        let module_match = filter.module
            .as_ref()
            .map(|m| &event.module == m)
            .unwrap_or(true);
        
        let function_match = filter.function
            .as_ref()
            .map(|f| &event.event_function == f)
            .unwrap_or(true);

        let type_match = filter.event_type
            .as_ref()
            .map(|t| &event.event_type == t)
            .unwrap_or(true);

        package_match && module_match && function_match && type_match
    }).collect()
}