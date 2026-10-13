//! components — ECS components wall systems create/query (typed-gap facades).

use bevy_ecs::prelude::Component;

use crate::typed_gap::Gap;

/// Debug test cube marker.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TestCube {
    pub raw: Gap,
}

/// Preview marker when a wall end snaps to another wall's segment.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SnappingToSegmentPreview {
    pub raw: Gap,
}

/// Preview marker when a wall end snaps to another wall's end.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SnappingToEndPreview {
    pub raw: Gap,
}

/// Child wall (eraser-split result) wrapper.
#[derive(Component, Clone, Debug, Default)]
pub struct ChildWallComponent {
    pub raw: Gap,
}
