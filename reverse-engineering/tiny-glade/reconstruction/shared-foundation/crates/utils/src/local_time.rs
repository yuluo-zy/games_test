use std::{sync::RwLock, time::SystemTime};
use time::{OffsetDateTime, UtcOffset};
static LOCAL_OFFSET: RwLock<Option<UtcOffset>> = RwLock::new(None);
/// Refresh the cached local offset, falling back to UTC on discovery failure.
/// B overwrites the cache on every call; startup call frequency is a consumer rule.
pub fn init_local_time() {
    let now = OffsetDateTime::now_utc();
    *LOCAL_OFFSET.write().expect("local offset lock") = UtcOffset::local_offset_at(now).ok();
}
pub fn system_time_to_local_time(time: SystemTime) -> OffsetDateTime {
    let time = OffsetDateTime::from(time);
    if let Some(offset) = *LOCAL_OFFSET.read().expect("local offset lock") {
        time.to_offset(offset)
    } else {
        time
    }
}
