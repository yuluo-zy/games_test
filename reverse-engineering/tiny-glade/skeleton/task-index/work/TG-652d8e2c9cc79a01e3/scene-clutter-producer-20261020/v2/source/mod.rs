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

/// B RaycastMeshData::new_cuboid（0x1408cfff0；core 恢复时该构造器位于
/// `native-physics` feature 域，system-decorator 不启用该 feature）。
/// **显式未恢复**：返回 None，不再以 `RaycastMeshData::default()` 冒充几何。
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

// ---- D4 追加（subdivide_wall_decorator，PDB utils/mod.rs 行 12 内联）----

/// B `subdivide_wall_decorator`（utils/mod.rs 行 12，inline；closure$0 行 26
/// 用 `WallSpaceCoord::with_offset_unbounded`，O）。
///
/// 墙装饰细分：把装饰位置按 `(du, dy)` 偏移复制一份（细分墙时装饰在两侧各
/// 保留一份）。返回 `(原位置, 偏移位置)`。
/// 签名面 I（调用点为 D2 ui_systems handle_splitting_* 内联宿主，待 reviewer
/// 按调用点核对参数序）。
pub fn subdivide_wall_decorator(
    space: &utils::wall_space::WallSpace,
    coord: utils::wall_space::WallSpaceCoord,
    offset: Vec2,
) -> (utils::wall_space::WallSpaceCoord, utils::wall_space::WallSpaceCoord) {
    let split = coord.with_offset_unbounded(space, offset);
    (coord, split)
}

/// 装饰细分闭包宿主（closure$0 行 26：`with_offset_unbounded` 实例）。
pub fn subdivide_wall_decorator_closure(
    space: &utils::wall_space::WallSpace,
) -> impl Fn(utils::wall_space::WallSpaceCoord, Vec2) -> (utils::wall_space::WallSpaceCoord, utils::wall_space::WallSpaceCoord)
       + '_ {
    move |coord, offset| subdivide_wall_decorator(space, coord, offset)
}
