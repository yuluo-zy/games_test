//! B wire contract. Names are read from original serde field literals, not inferred.
//! The serde implementation is reused; derives are not recovered business algorithms.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveEvent {
    pub start: i64,
    pub end: i64,
    pub cid: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventsResponse {
    pub events: Vec<ActiveEvent>,
}
