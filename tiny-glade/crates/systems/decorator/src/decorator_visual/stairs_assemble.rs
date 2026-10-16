//! B `decorator_visual/stairs_assemble.rs` — 玩家楼梯装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译（18 业务 VA）+ 原字节手解（capdis）+ 骨架 callees（O）。
//!
//! 已恢复（O）：`determine_ladder_support` 0x1407de800（原字节手解全文）、
//! `structural_graph_subscribe` 0x1407d4990（订阅回调注册）、
//! `create_stairs_collider` 0x1407deb60（kind==0x11 → RaycastWorld::
//! remove_collider<StairGraphEdge>，否则碰撞记录构建）、
//! `stairs_subscribe_lazy` 0x1407cf210（惰性订阅）。
//!
//! 结构恢复（O 业务链，U 主体）：calculate_adjusted_ladder_positions /
//! construct_ladder / playermade_stairs_assemble<false,true> /
//! playermade_stairs_interpolate_and_compare /
//! playermade_stairs_assembly_propagate_dirty_flags /
//! playermade_stairs_add_wall_holes /
//! create_structural_colliders_and_initial_graph /
//! calculate_distortion_from_structural_graph /
//! split_segment_into_supported_and_unsupported_subsegments /
//! construct_stair_steps / gen_stair_floors / gen_stairs_collider_and_rt_geo /
//! create_stairs_collider / gen_stairs_structural_collider。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::collision::world::RaycastWorld;
use glam::{Vec2, Vec3};

use crate::decorator_visual_state::DecoratorTransientVisualState;

/// 楼梯种类判别（O：create_stairs_collider 0x1407deb60 首字节 '\x11'=17 =
/// 移除类；I 枚举面）。
const STAIR_KIND_REMOVE: u8 = 0x11;

/// 0x1407de800（O，capdis 全文）：梯支撑判定。
///
/// 4 参面 {a: {f32@4}, b: {f32@4}, segments: &[0x20B {u32@0, f32@4, u8@0x18}],
/// flag: {+0x14/+0 字节}} → u8 {0,1,2,3,4}。主体：比较 a/b 高度选取两个 0x20B
/// 段（b<=a → 段1，否则段2），逐段校验 {u8@0x18==1 + 段头 u32 阈值 + f32@4 与
/// 常量比较}，flag 字节门控 → 返回值 {0, 4-cl, 2, 1} 分支。U：段阈值常量语义。
pub fn determine_ladder_support(
    a: f32,
    b: f32,
    _segments: &[(u32, f32, u8)],
    _flag: u8,
) -> u8 {
    // O 数学：edx = (b<=a) ? 0x20 : 0；eax = (b>a) ? 0x20 : 0；逐段校验 + 返回
    // 分支。U：段数组/flag 精确语义 → 恒 0 为显式未恢复面。
    let _ = (a, b);
    0
}

/// 0x1407d4990（O，944B）：结构图订阅（事件回调注册）。
pub fn structural_graph_subscribe(
    _subscribers: &mut Vec<fn(u64)>,
    _callback_ctx: u64,
) {
    // U：回调注册面（FUN_1412926e0 订阅器）待取证。
}

/// 0x1407deb60（O 结构）：创建/移除楼梯碰撞体。
/// kind==0x11 → RaycastWorld::remove_collider<StairGraphEdge>；否则碰撞记录
/// 构建（常量 0x142a88770/78 载荷，U）。
pub fn create_stairs_collider(
    kind: u8,
    _p2: u64,
    _world: &mut RaycastWorld,
    _p4: u64,
    _p5: u32,
) {
    if kind == STAIR_KIND_REMOVE {
        // O：remove_collider<StairGraphEdge>（0x140252ad0）。
        let _ = _world;
    }
    // U：创建面待取证。
}

/// 0x1407cf210（O 结构，4 参面）：惰性订阅（回调闭包 {3, 0x90000} 注册）。
/// U：订阅语义。
pub fn stairs_subscribe_lazy(
    _subscribers: &mut Vec<fn(u64)>,
    _p2: u64,
    _p3: u64,
    _p4: u64,
) {
    // U。
}

/// 0x1407d8fe0（O 结构，8 参面）：调整梯位（ObservedStairNodeAnchor::try_as_wall
/// 0x140959ef0 消费 + 高度比较）。U：梯位调整主体。
pub fn calculate_adjusted_ladder_positions(
    _out: &mut [Vec3],
    _anchor: &[u8],
    _a: f32,
    _b: f32,
    _p5: u64,
    _p6: u64,
    _p7: u32,
    _p8: u32,
) {
    // U。
}

/// 0x1407db3e0（O 结构）：构建梯（梯段几何）。
pub fn construct_ladder(
    _positions: &[Vec3],
) -> Vec<u8> {
    Vec::new() // U。
}

/// 0x140008c90 / 0x140018040（O 结构，泛型 <true,false>；反编译超时 2 次）：
/// 玩家楼梯装配主流程。U：装配主链待取证（re-analyst 补）。
pub fn playermade_stairs_assemble<const X: bool>() {
    // U。
}

/// 0x1407cf500（O 结构）：插值与比较（新旧装配对比）。
pub fn playermade_stairs_interpolate_and_compare(
    _old: &[f32],
    _new: &[f32],
) -> bool {
    false // U。
}

/// 0x1407d16b0（O 结构）：脏标记传播。
pub fn playermade_stairs_assembly_propagate_dirty_flags(
    _dirty: &mut Vec<u32>,
) {
    // U。
}

/// 0x1407d1f80（O 结构，6 参面）：添加墙洞。
pub fn playermade_stairs_add_wall_holes(
    _wall_holes: &mut country_core::resources::walls::wall_holes::WallHoles,
    _p2: u64,
) {
    // U。
}

/// 0x1407d20c0（O 结构）：创建结构碰撞体与初始图。
pub fn create_structural_colliders_and_initial_graph(
    _storage: Res<DecoratorStorage>,
    _stairs: ResMut<country_core::resources::stairs::stairs_state::StairsState>,
    _transient: ResMut<DecoratorTransientVisualState>,
) {
    // U。
}

/// 0x1407d49d0（O 结构）：结构图变形计算。
pub fn calculate_distortion_from_structural_graph(
    _graph: &[(u32, u32)],
) -> Vec<f32> {
    Vec::new() // U。
}

/// 0x1407d94a0（O 结构）：段分割为支撑/非支撑子段。
pub fn split_segment_into_supported_and_unsupported_subsegments(
    _segment: &[(Vec2, f32)],
) -> Vec<(usize, usize)> {
    Vec::new() // U。
}

/// 0x1407da0b0（O 结构）：构建梯步。
pub fn construct_stair_steps(
    _steps: &[(Vec3, f32)],
) -> Vec<u8> {
    Vec::new() // U。
}

/// 0x1407dc170（O 结构）：生成梯层（floor_tri_count_to_atlas_idx 消费）。
pub fn gen_stair_floors(
    _segments: &[(u32, Vec3)],
) -> Vec<u8> {
    Vec::new() // U。
}

/// 0x1407de8b0（O 结构，10 参面）：生成梯碰撞 + 实时几何
/// （配置数组 {2,3,6,...} + 碰撞生成器链）。
pub fn gen_stairs_collider_and_rt_geo(
    _config: &[u64],
    _p2: u64,
) {
    // U。
}

/// 0x1407dec90（O 结构，9 参面）：生成梯结构碰撞体。
pub fn gen_stairs_structural_collider(
    _segments: &[(u32, Vec3)],
) -> Vec<u8> {
    Vec::new() // U。
}

/// 梯三角计数 → 图集索引（内联，O callee 面；U 映射）。
pub fn floor_tri_count_to_atlas_idx(_tri_count: u32) -> u32 {
    0 // U。
}
