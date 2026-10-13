//! invalidate.rs — 脏楼梯伪影失效系统（WP21）。
//!
//! O：`invalidate_dirty_stair_artifacts`（0xb37180，3455B）：读 StairsState +
//! PlayermadeStairsAssemblyParams；drain 脏边集合；对每条脏边撤 RaycastWorld 碰撞体×3、
//! drop Arc<RaycastMeshData>、删 RtWorld RtMesh、删 StairSubSegment/StairArchWallSupportType
//! map、`StairsVisualState::remove_edge`；`contains_edge_or_stub` 决定撤视觉。
//! 依赖碰撞/RT/视觉域类型 —— 本文件只给骨架签名（typed gap 参数），系统级接线由
//! 集成 owner 完成。

use crate::resources::stairs::ColliderHandle;
use crate::resources::stairs::visual::StairsVisualState;
use crate::systems::collision::world::RaycastMeshData;
use crate::resources::stairs::stairs_state::StairGraphEdge;
use crate::typed_gap::Gap;

/// 脏楼梯伪影失效系统（O：函数面 0xb37180；参数域侧类型 U）。
pub fn invalidate_dirty_stair_artifacts(
    _state: &mut Gap,
    _assembly: &mut [crate::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams],
    _visual: &mut StairsVisualState,
    _colliders: &mut Vec<(StairGraphEdge, ColliderHandle, std::sync::Arc<RaycastMeshData>)>,
) {
    // 详见合同 stairs.md §invalidate；此处为骨架表面。
    let _ = (_state, _assembly, _visual, _colliders);
}
