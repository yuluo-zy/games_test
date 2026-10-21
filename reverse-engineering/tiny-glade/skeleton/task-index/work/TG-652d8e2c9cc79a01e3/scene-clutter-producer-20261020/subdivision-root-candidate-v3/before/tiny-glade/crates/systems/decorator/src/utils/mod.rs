//! B `system_decorator::utils/` — decorator 内部工具（D1 消费契约）。
//!
//! 本模块承载 decorator_visual 调用闭合所需的真实接线：
//! - `visual_entity_builder`：WindowVisualEntityBuilder（见子模块）。
//! - `DormerHoleSsbo`：B 中 country-core `resources/walls/dormers.rs` 的 48B 洞
//!   记录（core 侧未恢复，D1 于此处按机器码承载；dummy 0x141286660 / gothic
//!   0x141286680 / cottage 0x1412866e0，capdis 手解 O）。
//! - B utils crate 缺口函数（random_splits / inverse_lerp 等，按调用点承载）。

pub mod visual_entity_builder;

use fastrand::Rng;
use glam::{Mat3, Quat, Vec2, Vec3};

/// B DormerHoleSsbo：老虎窗洞 GPU 记录（0x30B=48B，gothic 尾区 4×f32 /
/// cottage 尾区 2×f32+f64；chapters/resources/walls/dormers.rs.md §DormerHoleSsbo
/// 构造器 + capdis 手解 O）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct DormerHoleSsbo {
    /// +0x00：16B 头区（gothic 构造 r8 源；语义 U）。
    pub a: [u8; 16],
    /// +0x10：16B 区（gothic 构造 r9 源；语义 U）。
    pub b: [u8; 16],
    /// +0x20：尾区（gothic 4×f32；cottage 2×f32 + f64@+0x28）。
    pub tail: [u8; 16],
}

impl DormerHoleSsbo {
    /// dummy 0x141286660（O）：全零 48B。
    pub fn dummy() -> Self {
        Self::default()
    }

    /// gothic 0x141286680（O，capdis）：{16B 拷贝, u32, 16B 拷贝, u32=0,
    /// 4×f32 尾区}。
    pub fn gothic(
        a: [u8; 16],
        x: u32,
        b: [u8; 16],
        tail: [f32; 4],
    ) -> Self {
        let mut out = Self::default();
        out.a = a;
        out.b = b;
        out.tail[0..4].copy_from_slice(&tail[0].to_le_bytes());
        out.tail[4..8].copy_from_slice(&tail[1].to_le_bytes());
        out.tail[8..12].copy_from_slice(&tail[2].to_le_bytes());
        out.tail[12..16].copy_from_slice(&tail[3].to_le_bytes());
        // 机器码：+0xc = edx（u32），+0x1c = 0。调用点（generate_arrow_slits
        // 0x1407bbee0）传 8 参：{out, u32, &16B, &16B, f32, f32, f32, 2.5, 0.05}
        // —— 尾区语义 U；此处以 x 承载 +0xc 字段并登记。
        out.b[4..8].copy_from_slice(&x.to_le_bytes());
        out
    }

    /// cottage 0x1412866e0（O，章节）：同构，尾区 +0x28 为 f64。
    pub fn cottage(a: [u8; 16], x: u32, b: [u8; 16], tail: [f32; 2], d: f64) -> Self {
        let mut out = Self::gothic(a, x, b, [tail[0], tail[1], 0.0, 0.0]);
        out.tail[8..16].copy_from_slice(&d.to_le_bytes());
        out
    }
}

/// B utils crate `xz_projective_rotation_bad`（0x140c911d0，O）：由 XZ 向量
/// {x@0, z@2} 构造投影旋转基（规范化 + 叉积正交基，写 12 浮点矩阵）。
/// 名称 "bad" = 旧旋转约定（与 to_projective_xz_mat2 的 Mat2 不同）。
pub fn xz_projective_rotation_bad(dir: Vec2) -> Mat3 {
    let forward = dir.normalize_or(Vec2::new(0.0, 1.0));
    // 机器码：先规范化 {x,z}，再以单位轴叉积构造正交基（_UNK_1429cd0c0..cc
    // = ±1 轴常量）。
    let f = Vec3::new(forward.x, 0.0, forward.y);
    let up = Vec3::Y;
    let right = up.cross(f).normalize();
    let true_up = f.cross(right);
    Mat3::from_cols(right, true_up, f)
}

/// B utils crate `xz_rotation_bad`（0x140c90fd0，O）：与
/// xz_projective_rotation_bad 同构（两标量输入版本）。
pub fn xz_rotation_bad(x: f32, z: f32) -> Mat3 {
    xz_projective_rotation_bad(Vec2::new(x, z))
}

/// B utils crate `random_splits`（调用点 O：fastrand f32 + ceil +
/// Vec2::lerp 链）：把 [0, total] 随机切分为 n 段，返回分段长度（I 语义）。
/// 供 chimney_assemble_bricks / door_stairs_assemble_bricks 砖列随机分割。
pub fn random_splits(rng: &mut Rng, total: f32, n: usize) -> Vec<f32> {
    if n == 0 {
        return Vec::new();
    }
    let mut splits = Vec::with_capacity(n + 1);
    let mut acc = 0.0f32;
    for i in 0..n {
        let remaining = n - i;
        // 每段占剩余期望的均匀随机份额（[0, 2/remaining) 均值 1/remaining）。
        let frac = rng.f32() * 2.0 / remaining as f32;
        let seg = (total - acc) * frac.min(1.0);
        acc += seg;
        splits.push(seg);
    }
    // 尾差并入末段（累计舍入闭合）。
    if let Some(last) = splits.last_mut() {
        *last += total - acc;
    }
    splits
}

/// B utils crate `inverse_lerp`（0x1407d9780 / 0x140c904a0，调用点 O）：
/// `(v - a) / (b - a)`。
pub fn inverse_lerp(a: f32, b: f32, v: f32) -> f32 {
    (v - a) / (b - a)
}

/// B utils crate `smoothstep`（调用点 O：chimney_assemble_bricks 闭包链）。
/// Hermite 平滑：`t*t*(3-2t)`，t 钳制 [0,1]。
pub fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// 旧三标量接口的调用者尚缺原本地位姿合同；不能假设零中心替换。
/// **显式未恢复**：返回 None，不以 default 冒充几何。真实 clutter 已由
/// new_clutter_cuboid 传完整尺寸与本地位姿；本包现明确启用 native-physics。
pub fn new_cuboid_collider(
    _x: f32,
    _y: f32,
    _z: f32,
) -> Option<country_core::systems::collision::world::RaycastMeshData> {
    None
}

/// B helper@1408b8e90: reuse the real loader-owned canonical metadata.
pub use country_core::resources::clutter_meshes::clutter_mesh_ty_data;

/// Actual native-physics constructor. Full dimensions and local translation are
/// separate inputs; the canonical core implementation owns the Parry geometry.
/// Production builds require country-core/native-physics, as does this producer.
pub fn new_clutter_cuboid(
    dimensions: Vec3,
    local_translation: Vec3,
) -> country_core::systems::collision::world::RaycastMeshData {
    country_core::systems::collision::world::RaycastMeshData::new_cuboid_at(
        dimensions,
        local_translation,
    )
}

/// B utils crate `IsoTransform`（B 未恢复；D1 承载）：刚性变换 {pos, rot}
/// （无缩放）。消费面：flowerbed_xforms 等装饰几何输出。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsoTransform {
    pub pos: Vec3,
    pub rot: Quat,
}

impl IsoTransform {
    pub fn new(pos: Vec3, rot: Quat) -> Self {
        Self { pos, rot }
    }
    /// 全局平移（装饰几何消费面）。
    pub fn translate(&self, delta: Vec3) -> Self {
        Self { pos: self.pos + delta, rot: self.rot }
    }
    /// 旋转（装饰几何消费面）。
    pub fn rotate(&self, delta: Quat) -> Self {
        Self { pos: self.pos, rot: self.rot * delta }
    }
}

/// 原 B utils/mod.rs 的墙装饰分段：逐点走同一真实墙曲线，再组合相邻点。
/// 原调用方收集迭代器；这里不建立第二份点数组或几何 owner。
/// 宽度、零乘法与各标量求值顺序来自 1407ed6c7..7be 和完整 141260ca0。
pub fn subdivide_wall_decorator<'a>(
    space: utils::wall_space::WallSpace,
    coord: utils::wall_space::WallSpaceCoord,
    width: f32,
    pieces: usize,
    curve: &'a impl utils::wall_space::CurveSampling,
) -> impl Iterator<Item = country_core::utils::components::transform::Transform> + 'a {
    use itertools::Itertools;
    use utils::comparative_space::ComparativeSpace;
    use utils::wall_space::WallSpaceCoord;

    let half_offset = width * f32::from_bits(0xbf00_0000);
    let start_u = space.project_unbounded(coord.u + half_offset);
    let start_y = half_offset * 0.0 + coord.y;
    let step = width / pieces as f32;
    (0..pieces + 1)
        .map(move |index| {
            let distance = index as f32 * step;
            let y_offset = 0.0 * distance;
            let u = space.project_unbounded(start_u + distance);
            WallSpaceCoord::new_unchecked(u, start_y + y_offset).to_world_space(curve)
        })
        .tuple_windows()
        .map(|(first, second): (Vec3, Vec3)| {
            let delta = second - first;
            // 原首条路径先求 dy²+dx²，再加 dz²；不改成仅 XZ 归一化。
            let reciprocal = 1.0 / ((delta.y * delta.y + delta.x * delta.x)
                + delta.z * delta.z).sqrt();
            let normal = Vec2::new(-delta.z * reciprocal, delta.x * reciprocal);
            country_core::utils::components::transform::Transform {
                // 原版在归一化之后仍调用同一个 bad-XZ 旋转，不改用 atan2。
                rotation: utils::xz_rotation_bad(normal),
                translation: Vec3::new(
                    (first.x + second.x) * 0.5,
                    (first.y + second.y) * 0.5,
                    (first.z + second.z) * 0.5,
                ),
                scale: Vec3::ONE,
            }
        })
}

#[cfg(test)]
mod subdivision_native_tests {
    use super::*;
    use country_core::resources::roofs::geometry::surface::Curve2;
    use utils::wall_space::{WallSpace, WallSpaceCoord};

    fn close(actual: Quat, expected: [f32; 4]) {
        for (actual, expected) in actual.to_array().into_iter().zip(expected) {
            assert!((actual - expected).abs() < 2.0e-7,
                "旋转分量不一致：actual={actual:?} ({:#010x}), expected={expected:?} ({:#010x})",
                actual.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn real_bent_curve_uses_each_adjacent_pose_instead_of_endpoint_lerp() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0]], true).unwrap();
        let poses: Vec<_> = subdivide_wall_decorator(
            WallSpace { is_loop: false, length: 4.0 },
            WallSpaceCoord::new_unchecked(2.0, 3.0), 4.0, 2, &curve).collect();
        assert_eq!(poses.len(), 2);
        assert_eq!(poses[0].translation, Vec3::new(1.0, 3.0, 0.0));
        assert_eq!(poses[1].translation, Vec3::new(2.0, 3.0, 1.0));
        close(poses[0].rotation, [0.0, 0.0, 0.0, 1.0]);
        close(poses[1].rotation, [0.0, -std::f32::consts::FRAC_1_SQRT_2, 0.0,
            std::f32::consts::FRAC_1_SQRT_2]);
        assert!(poses.iter().all(|pose| pose.scale == Vec3::ONE));
    }

    #[test]
    fn open_curve_reuses_original_negative_endpoint_direction_and_positive_extension() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [2.0, 0.0]], true).unwrap();
        let poses: Vec<_> = subdivide_wall_decorator(
            WallSpace { is_loop: false, length: 2.0 },
            WallSpaceCoord::new_unchecked(1.0, 5.0), 6.0, 2, &curve).collect();
        // 原 to_world_space 对负 U 做 position - tangent * U，故 -2 映射为 +2。
        assert_eq!(poses[0].translation, Vec3::new(1.5, 5.0, 0.0));
        assert_eq!(poses[1].translation, Vec3::new(2.5, 5.0, 0.0));
        close(poses[0].rotation, [0.0, 1.0, 0.0, 0.0]);
        close(poses[1].rotation, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn loop_projection_wraps_each_sample_across_the_actual_curve_seam() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0],
            [0.0, 2.0], [0.0, 0.0]], true).unwrap();
        let poses: Vec<_> = subdivide_wall_decorator(
            WallSpace { is_loop: true, length: 8.0 },
            WallSpaceCoord::new_unchecked(0.0, 4.0), 4.0, 2, &curve).collect();
        assert_eq!(poses[0].translation, Vec3::new(0.0, 4.0, 1.0));
        assert_eq!(poses[1].translation, Vec3::new(1.0, 4.0, 0.0));
        close(poses[0].rotation, [0.0, std::f32::consts::FRAC_1_SQRT_2, 0.0,
            std::f32::consts::FRAC_1_SQRT_2]);
        close(poses[1].rotation, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn zero_width_retains_degenerate_normal_and_original_shared_rotation_policy() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [2.0, 0.0]], true).unwrap();
        let poses: Vec<_> = subdivide_wall_decorator(
            WallSpace { is_loop: false, length: 2.0 },
            WallSpaceCoord::new_unchecked(1.0, 2.0), 0.0, 2, &curve).collect();
        assert_eq!(poses.len(), 2);
        for pose in poses {
            assert_eq!(pose.translation, Vec3::new(1.0, 2.0, 0.0));
            close(pose.rotation, [0.0, std::f32::consts::FRAC_1_SQRT_2, 0.0,
                std::f32::consts::FRAC_1_SQRT_2]);
        }
    }

    #[test]
    fn zero_count_and_nonfinite_width_preserve_actual_curve_failures() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [2.0, 0.0]], true).unwrap();
        for (width, pieces) in [(1.0, 0), (f32::INFINITY, 2), (f32::NAN, 2)] {
            assert!(std::panic::catch_unwind(|| {
                subdivide_wall_decorator(WallSpace { is_loop: false, length: 2.0 },
                    WallSpaceCoord::new_unchecked(1.0, 2.0), width, pieces, &curve)
                    .collect::<Vec<_>>()
            }).is_err());
        }
    }
}
