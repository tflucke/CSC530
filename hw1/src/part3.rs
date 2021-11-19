use super::data::Ticket;
use std::collections::HashMap;

pub fn tally_tickets(tickets: &[Ticket]) -> HashMap<&str, u32> {
    let mut map = HashMap::new();
    for t in tickets {
        let count = map.entry(t.event_id.as_str()).or_insert(0);
        *count += 1;
    }
    return map;
}
