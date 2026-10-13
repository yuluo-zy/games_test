//! add_* 自动杂物放置规则业务恢复草稿（B 各函数独立系统）
//!
//! 证据：reva 反编译 + 反汇编 + radare2 RNE .data 字符串（gate_right/gate_left/lantern/tree）。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use crate::utils::rng::{Circle2d, Rng};

use super::populate_autoclutter_regions::DirtyTiles;
use super::{AutoClutterRegion, AutoClutterState, ClutterRegionId, WallId};

/// 复用输出候选写入（state+0x48，0x68 stride，+0x58 计数）：
/// 本草稿以 push 到 spawn_candidates 表达；字段语义见 AutoClutterSpawn（unknown 标注）。
macro_rules! push_spawn {
    ($state:expr, $kind:expr, $name:expr, $key:expr, $position:expr) => {
        // TODO(evidence): 精确字段布局（+0x18=2、+0x30 byte、+0x60 缩放等）待 0x68 记录重排。
        let _ = &mut *$state;
        let _ = ($kind, $name, $key, $position);
    };
}

// ------------------------------------------------------------------
// add_autoclutter_around_gates（B 0x1422cc390，size 2910，源行 21–68）
// ------------------------------------------------------------------
/// 门周围杂物放置（证据）：
/// - 遍历门（宿主查询 +0xf8/+0x100/+0x108）；每门经 WallId + tick 取墙条目（stride 0x38）。
/// - 墙点数 >2 才放置；取墙首/末点（stride 0xc = Vec3）。
/// - 2 个放置：prefab `gate_right`（0x143042260 len 10）与 `gate_left`（len 9，.data 相邻）；
///   位置=墙端中点 + 方向偏移（1/SQRT 归一化、x0y swizzle）。
/// - 键 calculate_hash<tuple<&str,u32,WallId>>（141cf7440）→ AutoClutterState::add_region（1 处）。
pub fn add_autoclutter_around_gates(
    mut state: ResMut<AutoClutterState>,
    _gates: Query<&Gate>,
) {
    for gate in &_gates {
        // evidence: 每门取墙点，构造两个区域候选（gate_right / gate_left）。
        let wall_id: WallId = gate.wall_id;
        let _key = hash_gate_placement("gate_right", gate_index(&_gates, gate), wall_id);
        // add_region(&mut state, key, &region);
    }
}

/// 门组件（country_core；字段布局 unknown）。
#[derive(Component)]
pub struct Gate {
    pub wall_id: WallId,
    /// unknown。
    pub _idx: u32,
}

fn gate_index(_q: &Query<&Gate>, _g: &Gate) -> u32 {
    // TODO(evidence): 门在查询中的序号。
    0
}

fn hash_gate_placement(_name: &str, _u32: u32, _wall: WallId) -> ClutterRegionId {
    // evidence: calculate_hash<tuple<&str,u32,WallId>>。
    ClutterRegionId(0) // TODO(evidence): hash 值来源。
}

// ------------------------------------------------------------------
// add_autoclutter_around_windows（B 0x1422cefe0，size 4303，源行 31–181）
// ------------------------------------------------------------------
/// 窗周围杂物放置（证据）：
/// - 遍历窗（spawns_clutter/subtype/transform/iter_removed_ids 查询）；
///   extend_right_until<Vec3,CurveCoordSemantic3D> 沿窗曲线延伸。
/// - 3 个 add_region（140ee3fd0×3）；键 calculate_hash<tuple<&str,i32,usize,u32>>×1 +
///   <tuple<&str,usize,u32>>×2；x0y/xvy 位置/朝向。
pub fn add_autoclutter_around_windows(
    mut state: ResMut<AutoClutterState>,
    _windows: Query<&Window>,
) {
    for _w in &_windows {
        // evidence: 每窗 3 个 add_region。
    }
}

/// 窗组件（country_core WindowShutter 相关；subtype 查询 B 140b26970）。
#[derive(Component)]
pub struct Window;

// ------------------------------------------------------------------
// add_autoclutter_on_windows（B 0x1422cd7d0，size 1622，源行 17–65）
// ------------------------------------------------------------------
/// 窗上杂物放置（反编译质量差；反汇编证据）：
/// - 读候选字节 [R9-0x50]==6 判类型；LEA RDX [0x143042370] 跳表分派（表仅前 6 项有效，
///   后接 rustc 路径字符串）；常量 1.0/0.6（0x142925904）、Vec 0,0,1,0（0x1429cd0c0）。
/// - 语义未完全钉死。
pub fn add_autoclutter_on_windows(_state: ResMut<AutoClutterState>) {
    // TODO(evidence): 跳表目标与各分支语义待精读。
}

// ------------------------------------------------------------------
// add_birdnests（B 0x1422c9c70，size 3143，源行 37–141；闭包 closure$0=L84、closure$1=L105）
// ------------------------------------------------------------------
/// 鸟巢放置（证据）：
/// - Subscriber::take<DirtyTiles> → 复制 0x200B 脏位集 → new_clear。
/// - 遍历脏格行（cell_from_world_space；hash_one<&GlobalId> 查已存在集）；未处理格调
///   closure_0（1422ca8c0）+ HashMap<u64,bool>::insert 去重。
/// - 直接写输出候选（state+0x48）：+0x0=4（type）、+0x8 Arc 强计数、位置、
///   +0x60=1.0 缩放。即每脏格 1 鸟巢候选。
pub fn add_birdnests(
    mut state: ResMut<AutoClutterState>,
    dirty_tiles: Res<DirtyTiles>,
    _bird_locs: Res<BirdNestCandidateLocations>,
) {
    let _cells = dirty_tiles.rows.clone();
    // evidence: 逐脏格插 HashMap<u64,bool> 去重后 push type=4 候选。
    let _ = &mut state;
}

/// 鸟巢候选位置资源（build 里 insert_resource BirdNestCandidateLocations）。
#[derive(Resource)]
pub struct BirdNestCandidateLocations;

// ------------------------------------------------------------------
// add_lantern_water_setdresssing（B 0x1422d2fa0，size 2135，源行 22–102）
// ------------------------------------------------------------------
/// 灯笼/水面装饰放置（证据）：
/// - as_inner_mut 借 WaterRaster（+0x298 双缓冲）；is_corrupted 门控；
///   DecoratorStorage::iter_terrain_decorators 遍历地形装饰物。
/// - 每装饰物 Circle2d::new + init_rng（fastrand）；内层 5 次（i/5.0）：
///   fastrand f32×3 + fmodf 圆周 jitter → get_pos_at_u → 水位采样（0.0<采样=入水才放）→
///   归一化朝向 + 键 calculate_hash<tuple<u32,usize,&str>>（`lantern` @0x143042d28 len 7）→
///   is_started 选偏移向量。
/// - 直接写输出候选（0x68）：+0x0=5、+0x8=lantern、+0x18=2、+0x1c 键、+0x30=5、
///   +0x38 位置、+0x60 缩放。
pub fn add_lantern_water_setdresssing(
    mut state: ResMut<AutoClutterState>,
    _decorators: Query<&TerrainDecorator>,
) {
    for _d in &_decorators {
        let mut rng = Rng::new();
        for i in 0..5u32 {
            let u = (i as f32 / 5.0 + rng.f32() * 0.1).fract();
            let pos = Circle2d::new(Vec2::ZERO, 1.0).get_pos_at_u(u);
            // evidence: 水位采样后才写；此处仅骨架。
            let _pos = pos.extend(0.0);
            // 键 = calculate_hash<tuple<u32,usize,&str>>：第一项为循环下标 i（u32），
            // 非 f32 采样 u（草稿原传 u 与签名不符，已按证据改用 i）。
            push_spawn!(state, 5, "lantern", hash_setdressing(i, "lantern"), _pos);
        }
    }
}

/// 地形装饰物组件。
#[derive(Component)]
pub struct TerrainDecorator;

fn hash_setdressing(_u: u32, _name: &str) -> u64 {
    // evidence: calculate_hash<tuple<u32,usize,&str>>（灯笼）/<tuple<TreeStorageIndex,usize,&str>>（树）。
    0 // TODO(evidence): 键组成。
}

// ------------------------------------------------------------------
// add_tree_setdressing（B 0x1422d3800，size 2594，源行 19–100）
// ------------------------------------------------------------------
/// 树植被放置（证据）：
/// - 遍历树（PlacementDataSsbo::position×2 取位）；is_dragging / is_started×4 / is_corrupted×4
///   多重门控。
/// - 每树 Circle2d::new + init_rng；内层 12 次（i/12.0）：f32+fmodf 圆周 jitter →
///   get_pos_at_u → 水位采样 → 入水 → 键 calculate_hash<tuple<TreeStorageIndex,usize,&str>>
///   （`tree` @0x143042d91 len 4）→ 写输出候选（+0x0=5、+0x30=5）。
pub fn add_tree_setdressing(
    mut state: ResMut<AutoClutterState>,
    _trees: Query<&TreeEntry>,
) {
    for _t in &_trees {
        let mut rng = Rng::new();
        for i in 0..12u32 {
            let u = (i as f32 / 12.0 + rng.f32() * 0.1).fract();
            let pos = Circle2d::new(Vec2::ZERO, 1.0).get_pos_at_u(u).extend(0.0);
            push_spawn!(state, 5, "tree", hash_setdressing(i, "tree"), pos);
        }
    }
}

/// 树记录（TreeStorageIndex 键；position 查询 B 140ad51a0）。
#[derive(Component)]
pub struct TreeEntry;
