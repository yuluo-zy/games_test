use super::*;
pub fn json_string(value: &str) -> String {
    let mut escaped = String::from("\"");
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            control if control.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", control as u32))
            }
            other => escaped.push(other),
        }
    }
    escaped.push('"');
    escaped
}
impl Snapshot {
    pub fn to_json(&self) -> String {
        let names = self
            .pass_order
            .iter()
            .map(|name| json_string(name))
            .collect::<Vec<_>>()
            .join(",");
        let ranges = self
            .pass_ranges
            .iter()
            .map(|range| format!("[{},{}]", range.start, range.end))
            .collect::<Vec<_>>()
            .join(",");
        let physical = self.physical_mapping.iter().map(|mapping| format!("{{\"logical\":{},\"physical\":{},\"reused\":{},\"initial_access\":{},\"payload_tag\":{},\"token\":{}}}", mapping.logical, mapping.physical, mapping.reused, mapping.initial_access, mapping.payload_tag, mapping.token.map_or_else(|| "null".to_string(), |token| token.to_string()))).collect::<Vec<_>>().join(",");
        let barriers = |items: &[Transition]| {
            items
                .iter()
                .map(|item| {
                    format!(
                        "{{\"resource\":{},\"physical\":{},\"from\":{},\"to\":{}}}",
                        item.resource, item.physical, item.from, item.to
                    )
                })
                .collect::<Vec<_>>()
                .join(",")
        };
        let pre = self
            .pre
            .iter()
            .map(|items| format!("[{}]", barriers(items)))
            .collect::<Vec<_>>()
            .join(",");
        let batches = self
            .batches
            .iter()
            .map(|batch| {
                format!(
                    "{{\"start\":{},\"end\":{},\"export_barriers\":{}}}",
                    batch.pass_range.start, batch.pass_range.end, batch.export_barriers
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let accesses = self.accesses.iter().map(|access| format!("{{\"resource\":{},\"previous\":{},\"next\":{},\"synchronization_group\":{},\"code\":{}}}", access.resource_index, access.previous_access_index, access.next_access_index, access.synchronization_group, access.access_code)).collect::<Vec<_>>().join(",");
        let endpoint = |value: tg_rhapsody_reconstruction::resource_info::PassEndpoint| {
            if value.tag == 1 {
                value.pass_index.to_string()
            } else {
                "null".to_owned()
            }
        };
        let lifetimes = self
            .lifetimes
            .iter()
            .map(|lifetime| {
                format!(
                    "{{\"first_pass\":{},\"last_pass\":{},\"first_access\":{},\"last_access\":{}}}",
                    endpoint(lifetime.first_pass),
                    endpoint(lifetime.last_pass),
                    lifetime.first_access_index,
                    lifetime.last_access_index
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("{{\"status\":\"candidate_static\",\"pass_order\":[{}],\"pass_ranges\":[{}],\"accesses\":[{}],\"lifetimes\":[{}],\"image_usage\":{:?},\"buffer_usage\":{:?},\"lifetime_events\":{:?},\"physical_mapping\":[{}],\"pending_initial\":{:?},\"initial_transitions\":[{}],\"pre_transitions\":[{}],\"export_transitions\":[{}],\"batches\":[{}],\"imports\":{:?},\"exports\":{:?},\"pool_hits\":{:?}}}", names, ranges, accesses, lifetimes, self.image_usage, self.buffer_usage, self.lifetime_events, physical, self.pending_initial, barriers(&self.initial), pre, barriers(&self.export), batches, self.imports, self.exports, self.pool_hits)
    }
}
