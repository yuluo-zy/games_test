//! 主墙构造 CPU 链（wall_constructor.rs）。
//!
//! `WallConstructor::from_curve`（0x140769760，G04 冻结）：
//! 调用面（building-constructor README）：RCX=normalized-hole 输出 Vec owner、
//! RDX=Curve2、R8D=material、R9=arch/height 数组 + stack +20 arch count、
//! +28 第二 authority 高度 ptr、+30 count、+38 support/elevation ptr、
//! +40 count、+48 construction kind byte、+50 seed u64、+58 FinalizedWallHoles
//! ref、+60 row-end perturbation f32、+68 输出 Vec<96B> ptr、+70 lower/base f32、
//! +78 top/rim bounds ref、+80/+88 option flag+f32、+90/+98 option flag+f32、
//! +A0 wall float、+A8 bool（construct_walls 实传 0）。
//!
//! CPU 链已冻结（3516/3516 案例）：split_into_random_rows/trim_rows/
//! normalize_wall_holes/resolve_hole_overlap/cut_wall_bricks/
//! emit_resolved_wall_brick（96B BrickTransformSsbo）→ 本 crate 直接复用
//! country-core shared building modules 的同一链。新的生产入口直接复用
//! country-core 的完整 authority→row→proposal→96B 算法；旧 resolved
//! proposal 叶 API 仅作为证据核对入口保留，不再作为 World 的生产边。

pub use country_core::systems::wall::construction::bricks::WallBrick;
pub use country_core::systems::wall::construction::bricks::WallBrickInstance;
pub use country_core::systems::wall::construction::bricks::WallEmissionContext;
pub use country_core::systems::wall::construction::bricks::WallRow;
pub use country_core::systems::wall::construction::bricks::construct_resolved_plain_wall_proposal;
pub use country_core::systems::wall::construction::bricks::consume_wall_proposal;
pub use country_core::systems::wall::construction::bricks::cut_wall_bricks;
pub use country_core::systems::wall::construction::bricks::emit_plain_wall_brick;
pub use country_core::systems::wall::construction::bricks::emit_resolved_wall_brick;
pub use country_core::systems::wall::construction::bricks::normalize_wall_holes;
pub use country_core::systems::wall::construction::bricks::resolve_hole_overlap;
pub use country_core::systems::wall::construction::bricks::select_native_row_holes;
pub use country_core::systems::wall::construction::bricks::split_into_random_rows;
pub use country_core::systems::wall::construction::bricks::split_plain_row_horizontal;
pub use country_core::systems::wall::construction::bricks::trim_rows;

/// 96B 实例记录（BrickTransformSsbo 观测投影；48 affine + 16 deformation +
/// 12 surface + 4 seed + 4 material + 8 wall_distance + 4 flags）。
pub type BrickSsboRecord = [u8; 96];

/// Full authoritative producer, sharing the restored country-core algorithm.
/// Source arrays, cached U, holes, policy and seeds are required inputs.
pub fn from_curve_resolved(
    input: country_core::systems::wall::construction::production::PlainWallProductionInput<'_>,
    branch: country_core::systems::wall::construction::production::WallProductionBranch,
) -> Result<
    Vec<BrickSsboRecord>,
    country_core::systems::wall::construction::production::WallProductionError,
> {
    country_core::systems::wall::construction::production::produce_wall(input, branch).map(
        |instances| {
            instances
                .iter()
                .map(|instance| instance.observed_bytes())
                .collect()
        },
    )
}

/// `construct_walls`（0x140757800）调用的墙角/裙边/阶梯指示符几何
/// （add_wall_corners 0x141215540 / add_skirts 0x1412221f0 /
/// stairs_signifiers 0x141223d10；building-structures findings G05）。
///
/// 三个 helper 都委托到各自的原函数；在对应算法未闭合前保持显式 panic，
/// 不以空函数体吞掉真实输出。
pub mod helpers {
    /// 墙角砖（O：construct_walls 实调；几何数值见 building-structures）。
    pub fn add_wall_corners(points: &[[f32; 3]], out: &mut Vec<[u8; 96]>) {
        let points: Vec<glam::Vec3> = points.iter().map(|p| glam::Vec3::from_array(*p)).collect();
        crate::utils::wall_corners::add_wall_corners_new(&points, out);
    }
    /// 墙裙边（O：实调）。
    pub fn add_skirts(points: &[[f32; 3]], out: &mut Vec<[u8; 96]>) {
        let points: Vec<glam::Vec3> = points.iter().map(|p| glam::Vec3::from_array(*p)).collect();
        crate::utils::wall_skirts::add_skirts_new(&points, out);
    }
    /// 阶梯指示符（O：实调；stairs_signifiers 0x141223d10）。
    pub fn stairs_signifiers(wall: u64, points: &[[f32; 3]], out: &mut Vec<[u8; 96]>) {
        let points: Vec<glam::Vec3> = points.iter().map(|p| glam::Vec3::from_array(*p)).collect();
        crate::utils::stairs_signifiers::stairs_signifiers_new(wall, &points, out);
    }
}
