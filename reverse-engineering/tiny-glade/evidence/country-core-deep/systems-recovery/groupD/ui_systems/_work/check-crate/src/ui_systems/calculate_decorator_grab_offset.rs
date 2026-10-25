//! B `ui_systems/calculate_decorator_grab_offset.rs` — 抓取偏移计算。
//!
//! - `try_get_grab_offset_ss_if_valid` 0x141277bb0（O）：按 DecoratorInfo 目的
//!   tag 查跳表 UNK_142c23990：`idx = (tag-2)<3 ? tag-1 : 0` → 墙（tag 0/1）
//!   自有处理，屋顶/地形/楼梯（tag 2/3/4）共用同一处理（跳表项 1..3 同址）；
//! - `raycast_with_ss_offset` 0x141278050（O 结构）：光标 + 屏幕偏移 → 世界射线
//!   （Ray::from_mouse_ss 0x14127cab0），碰撞遮罩 0x30ab（'?' 装饰类型走 0x23
//!   排除装饰碰撞体——calculate_decorator_dst 的 unreachable 前提），地形射线
//!   （TerrainHeightsData::intersect_ray 0x1412773f0）兜底，输出 0x78B 交点记录
//!   （7×u64 拷贝 + 浮点载荷）。
use country_core::resources::walls::decorator::DecoratorInfo;
use glam::{Vec2, Vec3};
use utils::geometry::ray::Ray;

/// 屏幕空间抓取偏移（I：字段语义 U；原处理体未恢复）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GrabOffsetSs {
    /// 屏幕坐标。
    pub pos: Vec2,
    /// 附加浮点。
    pub extra: f32,
}

/// 0x78B 交点记录（I：{7×u64@+0..0x38, 载荷@+0x40..}；字段语义 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RaycastIntersection {
    pub data: [u64; 7],
    pub payload: [f32; 8],
}

/// 0x141277bb0（O 结构）：目的 tag 跳表派发（墙独立/其余同址；处理体 U）。
/// 返回 None 表示本实现未接线（宿主经 `raycast_with_ss_offset` 走通用路径）。
pub fn try_get_grab_offset_ss_if_valid(
    _info: &DecoratorInfo,
    _hit: &RaycastIntersection,
) -> Option<GrabOffsetSs> {
    let dst_tag = _info.dst_tag();
    let _idx = if dst_tag.wrapping_sub(2) < 3 {
        dst_tag.wrapping_sub(1)
    } else {
        0
    };
    let _ = _hit;
    None
}

/// 0x141278050（O 结构）：屏幕偏移射线 + 地形兜底。`'?'`（DecoratorType 0x3f）
/// 时遮罩 0x23，否则 0x30ab（O）；未命中时按平面投影回退（O 公式）。
pub fn raycast_with_ss_offset(
    cursor_ss: Vec2,
    offset: Vec2,
    window: Vec2,
    is_question_mark: bool,
    ray_from_mouse: impl Fn(Vec2, Vec2, Vec2) -> Ray,
    terrain_intersect: impl Fn(&Ray) -> Option<Vec3>,
) -> RaycastIntersection {
    let _mask = if is_question_mark { 0x23u32 } else { 0x30abu32 };
    let ray = ray_from_mouse(cursor_ss + offset, window, Vec2::ZERO);
    let mut out = RaycastIntersection::default();
    if let Some(hit) = terrain_intersect(&ray) {
        // O：地形命中判别 0xb（convert_raycast_hit_to_decorator_dst disc）。
        out.data[0] = 0xb;
        out.payload[0] = hit.x;
        out.payload[1] = hit.y;
        out.payload[2] = hit.z;
    } else {
        // O：`t = (0.0 - o.y)/dir.y` 平面投影。
        let t = if ray.direction.y.abs() > 1e-6 {
            -ray.origin.y / ray.direction.y
        } else {
            0.0
        };
        let p = ray.origin + ray.direction * t;
        out.payload[0] = p.x;
        out.payload[1] = p.y;
        out.payload[2] = p.z;
    }
    out
}
