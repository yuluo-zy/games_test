//! B `decorator_visual/gothic_balcony_door.rs` — 哥特阳台门装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407f0900（generate_gothic_balcony_doors，60KB）/
//! 0x1407f3340（check_for_door_stairs，6.5KB，**O 全文**）+ 骨架 callees（O）：
//! check_for_door_stairs ×2、get_door_holes、float_grid bilinear_sample
//! （0x140a14df0）、DecoratorLibKey 族、blueprint 链、WindowVisualEntityBuilder 链。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 门子类型判别：0x1407f0900 行 347 `subtype != 0x09 → skip`
//!   （哥特门子类型 = 9；与 derive 的 wall_attachment_subtype gothic door 一致）；
//! - `check_for_door_stairs`（0x1407f3340）：
//!   * `fVar10 = max(wall_space_eval(..., door_u=param_3[4], door_axis=param_3[6]), 0)`；
//!   * 地形高 `bilinear_sample(grid, param_3[4], param_3[6], 130.0)`（0x43020000）；
//!   * `fVar12 = param_3[5] + (param_4[0x1c] - param_4[0x18]) * (-0.5)`
//!     （0xbf000000 = -0.5）；
//!   * 地形高 <= 0：`fVar10 <= fVar12 + (-0.2)` → 无梯（0xbe4ccccd = -0.2）；
//!   * 地形高 > 0：`fVar12 <= 0.1 || fVar12 - 0.2 < fVar10` 才继续，否则无梯
//!     （0x3dcccccd = 0.1）；
//!   * 继续后取三个 wall-space 采样点的最小值 `best`；
//!     `0.1 < fVar10 - second` 与 `1.5 <= fVar10 - best`（0x3fc00000 = 1.5，
//!     未置位时）为门控；输出 32B `{x, y, 归一化方向(x,z), fVar10, best,
//!     max(height-best,0), height}`（0x20B 分配）。
//!
//! U 登记：① 门/梯几何主体 ② 参数排序 ③ 三次 wall-space 采样的精确位置
//! （本实现以输入显式承载）。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorMeshLibrary, DerivedDecoratorSubtypes,
};
use glam::{Quat, Vec2};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 哥特门子类型（O：0x1407f0900 行 347 `!= 0x09`）。
pub const GOTHIC_DOOR_SUBTYPE: u8 = 0x09;
/// 门高偏移半系数（O：0xbf000000 = -0.5）。
pub const DOOR_STAIRS_HALF: f32 = -0.5;
/// 地形无梯间隙容差（O：0xbe4ccccd = -0.2）。
pub const DOOR_STAIRS_GAP_TOLERANCE: f32 = -0.2;
/// 门下沿阈值（O：0x3dcccccd = 0.1）。
pub const DOOR_STAIRS_SILL: f32 = 0.1;
/// 最大台阶高（O：0x3fc00000 = 1.5）。
pub const DOOR_STAIRS_MAX_STEP: f32 = 1.5;
/// 地形双线性采样尺度（O：0x43020000 = 130.0）。
pub const DOOR_STAIRS_TERRAIN_SCALE: f32 = 130.0;

/// `check_for_door_stairs` 的可复核输入（O 参数面语义 U，字段按公式命名）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DoorStairsInput {
    /// `param_3[4]`：门 u（wall space）。
    pub door_u: f32,
    /// `param_3[5]`：门高参数。
    pub door_height: f32,
    /// `param_3[6]`：门轴坐标。
    pub door_axis: f32,
    /// `param_4[0x18]`。
    pub bottom: f32,
    /// `param_4[0x1c]`。
    pub top: f32,
    /// `max(wall_space_eval(door_u, door_axis), 0)`（fVar10）。
    pub wall_y: f32,
    /// `bilinear_sample(terrain, door_u, door_axis, 130.0)`。
    pub terrain_height: f32,
    /// 继续分支的三个 wall-space 采样（fVar10 的候选）。
    pub samples: [f32; 3],
}

/// `check_for_door_stairs` 输出（O 32B 记录的关键字段）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DoorStairsInfo {
    /// `fVar10`（墙面高度，>=0）。
    pub wall_y: f32,
    /// `best`（三采样最小值）。
    pub best: f32,
    /// `max(door_height - best, 0)`（O 行 144-147）。
    pub clearance: f32,
    /// 归一化方向（O 行 148-161）。
    pub direction: Vec2,
}

/// 0x1407f3340（O，5 参面，返回 u64）：检查门前楼梯。
///
/// 返回 `None` 对应原版返回 0（无梯）；`Some` 对应返回非 0 并写入 0x20B
/// 参数块。保留全部分支（地形高 ≤0 / >0、0.1/0.2/1.5 门控）。
pub fn check_for_door_stairs(input: &DoorStairsInput) -> Option<DoorStairsInfo> {
    let wall_y = input.wall_y.max(0.0);
    // O：`fVar12 = param_3[5] + (param_4[0x1c]-param_4[0x18]) * (-0.5)`。
    let sill = input.door_height + (input.top - input.bottom) * DOOR_STAIRS_HALF;
    if input.terrain_height <= 0.0 {
        // O：`if (fVar10 <= fVar12 + -0.2) return 0`。
        if wall_y <= sill + DOOR_STAIRS_GAP_TOLERANCE {
            return None;
        }
    } else if !(sill <= DOOR_STAIRS_SILL || sill + DOOR_STAIRS_GAP_TOLERANCE < wall_y) {
        // O：`if (fVar12 <= 0.1 || fVar12 - 0.2 < fVar10) goto 继续; else return 0`。
        return None;
    }

    // O：三次 wall-space 采样，`best = min(...)`。
    let second = input.samples[0];
    if !(DOOR_STAIRS_SILL < wall_y - second) {
        return None;
    }
    let best = second.min(input.samples[1]).min(input.samples[2]);
    // O：`1.5 <= fVar10 - best`（uVar8 未置位）→ 无梯。
    if DOOR_STAIRS_MAX_STEP <= wall_y - best {
        return None;
    }
    let clearance = (input.door_height - best).max(0.0);
    // O：方向 = 归一化 (dx, dz)（0x142925904 = 1.0 / sqrt）。
    let dir = Vec2::new(
        input.samples[1] - input.samples[0],
        input.samples[2] - input.samples[0],
    )
    .normalize_or(Vec2::X);
    Some(DoorStairsInfo {
        wall_y,
        best,
        clearance,
        direction: dir,
    })
}

/// 0x1407f0900（O 结构）：生成哥特阳台门装饰视觉。
pub fn generate_gothic_balcony_doors(
    decorator_storage: Res<DecoratorStorage>,
    derived: Res<DerivedDecoratorSubtypes>,
    cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    _mesh_lib: Res<DecoratorMeshLibrary>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：门子类型 = 9（0x1407f0900 行 347）。
            if derived.get(id).subtype().0 != GOTHIC_DOOR_SUBTYPE {
                continue;
            }
            let d = derived.get(id);
            let transform = cached.transform(id);
            // O：门前楼梯检测（check_for_door_stairs）。输入采样面 U → 仅当
            // 调用者提供真实采样时才产生结果；系统侧不伪造数值。
            let stairs = check_for_door_stairs(&DoorStairsInput {
                wall_y: render_door_wall_y(info),
                ..DoorStairsInput::default()
            });
            let _ = stairs;
            // O：蓝图 + 视觉实体。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            let builder =
                WindowVisualEntityBuilder::new(transform.translation, Quat::IDENTITY, id);
            let _ = &mut transient;
            let _ = &builder;
        }
    }
}

/// 门墙高来源（U：DecoratorInfo 未暴露 wall-space 采样；此处显式 0 表示
/// “未提供”，`check_for_door_stairs` 会走无梯分支而不是伪造楼梯）。
fn render_door_wall_y(_info: &DecoratorInfo) -> f32 {
    0.0 // U：wall-space 采样消费面待宿主提供。
}

/// 哥特门判别（O：派生子类型 0x09）。
pub fn is_gothic_door(derived_subtype: u8) -> bool {
    derived_subtype == GOTHIC_DOOR_SUBTYPE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(terrain: f32, wall: f32, height: f32, samples: [f32; 3]) -> DoorStairsInput {
        DoorStairsInput {
            door_u: 0.0,
            door_height: height,
            door_axis: 0.0,
            bottom: 0.0,
            top: 0.0,
            wall_y: wall,
            terrain_height: terrain,
            samples,
        }
    }

    #[test]
    fn below_terrain_gate_rejects_low_wall() {
        // terrain<=0，wall <= height-0.2 → None。
        assert!(check_for_door_stairs(&input(0.0, 0.5, 1.0, [0.0, 0.0, 0.0])).is_none());
        // wall 高出门槛 → 继续，但 0.1 < wall - second 且 best=0；
        // 1.5 <= wall - 0 → 无梯。
        assert!(check_for_door_stairs(&input(0.0, 3.0, 1.0, [0.0, 0.0, 0.0])).is_none());
    }

    #[test]
    fn stairs_found_when_gap_between_limits() {
        // wall=2.0, samples 0.0 -> gap 2.0 >=1.5 → None；samples 0.8 -> gap 1.2 ✓
        assert!(check_for_door_stairs(&input(1.0, 2.0, 1.0, [0.8, 0.8, 0.8])).is_some());
    }

    #[test]
    fn sill_rule_for_positive_terrain() {
        // terrain>0，sill=1.0>0.1，sill-0.2=0.8 >= wall 0.5 → None。
        assert!(check_for_door_stairs(&input(1.0, 0.5, 1.0, [0.0, 0.0, 0.0])).is_none());
    }

    #[test]
    fn clearance_uses_door_height_minus_best() {
        let info = check_for_door_stairs(&input(1.0, 2.0, 1.5, [0.8, 0.9, 0.7])).unwrap();
        assert_eq!(info.best, 0.7);
        assert!((info.clearance - 0.8).abs() < 1e-6);
    }

    #[test]
    fn predicate_is_subtype_nine() {
        assert!(is_gothic_door(9));
        assert!(!is_gothic_door(4));
    }
}
