//! B wire contract. Names are read from original serde field literals, not inferred.
//! The serde implementation is reused; derives are not recovered business algorithms.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Protocol seconds are signed; conversion to local `SystemTime` belongs to the
/// fetcher. The wire schema does not validate interval order or the CID format.
pub struct ActiveEvent {
    pub start: i64,
    pub end: i64,
    pub cid: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// `events` is required even when empty. The original B visitor accepts unknown
/// fields, rejects duplicate known fields, and also accepts serde struct sequences.
pub struct EventsResponse {
    pub events: Vec<ActiveEvent>,
}
