//! 墙内地板构造（construct_floors.rs）。
//!
//! `construct_floors`（0x1407661e0）。**权威签名**（Ghidra
//! `get_xrefs_to(0x1407661e0)` 的 `run_unsafe` 泛型名逐字，2026-10-17 fresh）：
//!
//! ```text
//! construct_floors(
//!     Res<PublicWalls>,
//!     EventReader<OnWallChanged>,
//!     EventReader<OnWallDeleted>,
//!     Local<FloorStorage>,
//!     Commands,
//! )
//! ```
//!
//! （此前 skeleton 的参数顺序 `(changed, deleted, walls, storage, commands)`
//! 与机器码不符，已按 xref 证据更正。）
//!
//! fresh 反编译（`work/restorer-remaining-wallconstructor/evidence/0x1407661e0.c`）
//! 已解出的控制流：
//! - 先消费 **OnWallDeleted**：逐事件 `wall_id`（`EventInstance` @+8）→
//!   `FloorStorage.entities`（`HashMap<WallId, Entity>`）`remove_entry`；
//!   命中且 `Entities::contains` → `commands.entity(e).despawn()`。
//! - 再消费 **OnWallChanged**：逐事件 `wall_id` → 同样先从 `FloorStorage`
//!   `remove_entry` + despawn 旧实体；随后 `PublicWalls` 哈希表探测
//!   （墙记录 stride 0x2a8）：
//!   * 命中且 `0.0 < wall.height（记录 -0x20 处 f32）` 且
//!     `*(u32 *)(wall - 0x48) < 2` 才生成；
//!   * `PublicWallState::get_wall_shape()`；shape 首 qword bit0 == 0 时
//!     `log2f(max(shape+0xc, 1.0) * 3.0)`（floor 高度缩放）；
//!   * `DecoratorRank::new` / `NaniInstanceMeshDesc::from` /
//!     `ShapeParameters::center` / `xvy` / `Transform::default` 装配
//!     `NaniBundle<StoneFloorInstanceData>` + `Floor` 的 insert 命令
//!     （命令载荷 0xb8B，bundle 0x48B @payload+8，Floor 载荷其后）；
//!   * `FloorStorage` 插入 `(wall_id, new_entity)`。
//!
//! **未实现（core-type-gap）**：`country_core::resources::walls::Floor`
//! 组件（约 0x50B 载荷：wall id + wall shape + 计算高度等）未落 country-core
//! （`grep` 全 workspace 无该类型），其字段布局未锚定——无法逐语句重建
//! insert 命令载荷。解锁条件：core 落 `Floor` 组件布局（O 面：wall_id@0、
//! shape 字段、floor 高度、byte@0x88=0 等）后按上述全序实现。
use bevy_ecs::prelude::*;
use country_core::resources::walls::public_walls::{OnWallChanged, OnWallDeleted, PublicWalls};

/// B `system_wall_constructor::construct_floors::FloorStorage`（Local）。
#[derive(Default, Debug)]
pub struct FloorStorage {
    /// 墙 id → 地板实体（跨帧保留）。
    pub entities: hashbrown::HashMap<u64, Entity>,
}

/// B 0x1407661e0。**未实现**：changed 分支的 `NaniBundle<StoneFloorInstanceData>`
/// + `country_core::resources::walls::Floor` insert 载荷依赖缺失的 `Floor`
/// 组件布局；`FloorStorage` 生命周期与墙门控条件已解（见模块文档），
/// 不写近似体。
pub fn construct_floors(
    _walls: Res<PublicWalls>,
    _changed: EventReader<OnWallChanged>,
    _deleted: EventReader<OnWallDeleted>,
    _storage: Local<FloorStorage>,
    _commands: Commands,
) {
    todo!(
        "未实现: construct_floors 0x1407661e0（缺 country_core::resources::walls::Floor 组件布局；deleted/changed 的 FloorStorage 生命周期与墙门控已解，见模块文档）"
    )
}
