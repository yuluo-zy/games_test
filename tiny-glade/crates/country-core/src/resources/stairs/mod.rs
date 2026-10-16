pub mod invalidate;
pub mod stairs_assembly_params;
pub mod stairs_collider;
pub mod stairs_removed_supports;
pub mod stairs_state;
pub mod structural_graph;
pub mod visual;

pub use invalidate::invalidate_dirty_stair_artifacts;
pub use stairs_assembly_params::{
    length_ws, ObservedAlignKind, ObservedStairNodeAnchor, StairWallAnchor,
};
pub use stairs_collider::StairsCollider;
pub use stairs_removed_supports::StairsRemovedSupports;
pub use stairs_state::{
    ObservedStairEndpoint, StairGraphEdge, StairsHistoryDiff, StairsSnapshot, StairsState,
    TransientStairEdgeId, TransientStairEdgeIdAllocator,
};
pub use structural_graph::{StairColliderHandle, StairsStructuralGraph};
pub use visual::{MagicSupports, StairsVisualState};
