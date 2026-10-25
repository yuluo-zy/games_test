//! Explicit input views for recovered draw callbacks. These expose only fields
//! observed by those callbacks and are not authoritative game resource schemas.

/// Per-ivy point counts consumed by draw callback A. The original hash-map
/// payload field is observed; full IvyStorage creation/growth is not imported.
#[derive(bevy_ecs::resource::Resource, Default)]
pub struct IvyDrawView {
    pub point_counts: Vec<usize>,
}

