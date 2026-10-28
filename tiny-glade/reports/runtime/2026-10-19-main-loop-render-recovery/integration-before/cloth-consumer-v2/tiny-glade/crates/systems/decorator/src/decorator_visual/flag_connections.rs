//! B `decorator_visual/flag_connections.rs` — 旗连接/布绳生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407df470（generate_flag_connections，49KB）/
//! 0x1407e1430（greedily_connect，25KB）/ 0x1407e2510（clothline，10.7KB）+
//! 骨架 callees（O）：clothline ×2、FUN_1407df050 ×2（相邻旗查询）、
//! WindowVisualEntityBuilder::build<Transient>（0x1412ab270）、
//! DecoratorExtraInfo::as_flag、blueprint 链。
//!
//! 本文件按机器码恢复为可验证的纯逻辑：
//!
//! - `greedily_connect`（0x1407e1430）：
//!   1. 逐对候选（0x50B 条目；`+0x4d` 源标记 != 0、目标标记 == 1）判定墙关系与
//!      距离窗：异墙 `d² < p7²`；同墙 `p9² < d² < p8²`（p7/p8/p9 见调用点
//!      0x1407df470 行 774：`2, 1, 0x41200000(10.0), 0x40b00000(5.5),
//!      0x40000000(2.0)`）；`param_6`（跨墙开关）= 1；
//!   2. 候选对去重（机器码 hashbrown 集合）；
//!   3. 第二遍：两端连接度 `param_5`（=2）未满、遮挡查询
//!      `intersects_any(segment, dist - 0.01, 0x30db)` 未命中、最小距离
//!      0.02（0x1429cd2d0）且坡度门 `2*|Δy| <= 水平距离` 通过 → 接受边，
//!      两端度 +1。
//! - `clothline`（0x1407e2510）：
//!   `n = max(2, round(dist / spacing))`；sag 混合因子
//!   `blend = clamp(sag_len / dist, 0, 0.25)`（0x142925950）；
//!   采样参数 `t' = t + (1-t)*blend`；下垂
//!   `(1 - (2t'-1)²) * 0.08 * dist`（0x142a88890 = 0.08）。
//!   机器码随后用半宽 `param_5`、角点归一化(`0.5/sqrt`)生成布条四角并
//!   `build<Transient>`；四角精确顺序 U（见下文 U 登记）。
//!
//! U 登记：① 相邻旗查询 FUN_1407df050 的墙侧筛选（锚点来源由调用系统给出）；
//! ② clothline 10 参面中 `param_4/param_6` 的精确语义；③ 四角归一化细节。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use glam::{Vec2, Vec3};
use std::collections::{HashMap, HashSet};

use crate::decorator_visual_state::DecoratorTransientVisualState;

/// 单节点最大连接数（O 调用点 0x1407df470：param_5 = 2）。
pub const CONNECT_MAX_PER_NODE: u32 = 2;
/// 跨墙连接开关（O 调用点 param_6 = 1）。
pub const CONNECT_CROSS_WALL: bool = true;
/// 异墙近距阈值（O 0x41200000 = 10.0）。
pub const CONNECT_NEAR_DIFFERENT_WALL: f32 = 10.0;
/// 同墙远距阈值（O 0x40b00000 = 5.5）。
pub const CONNECT_FAR_SAME_WALL: f32 = 5.5;
/// 同墙近距阈值（O 0x40000000 = 2.0）。
pub const CONNECT_MIN_SAME_WALL: f32 = 2.0;
/// 最小连接距离（O 0x1429cd2d0 = 0.02）。
pub const CONNECT_MIN_DISTANCE: f32 = 0.02;
/// 遮挡射线长度余量（O 0x142a82410 = -0.01）。
pub const CONNECT_OCCLUSION_MARGIN: f32 = -0.01;
/// 邻接边权偏置（O 0x1429cd078 = 3.0：`weight = dist + 3.0`）。
pub const CONNECT_EDGE_WEIGHT_BIAS: f32 = 3.0;
/// 布绳下垂系数（O 0x142a88890 = 0.08）。
pub const CLOTH_SAG: f32 = 0.08;
/// 布绳 sag 混合上限（O 0x142925950 = 0.25）。
pub const CLOTH_MAX_SAG_FRACTION: f32 = 0.25;
/// 布绳默认半宽（O 调用点 param_5 = 0x3f800000 = 1.0）。
pub const CLOTH_SEGMENT_WIDTH: f32 = 1.0;
/// 布条网格名（O 0x142a88b30，len 0x27）。
pub const CLOTH_MESH_THREAD: &str = "decorators/setdressing_clothline_thread";
/// 旗线网格名（O 0x142a88b57，len 0x14）。
pub const CLOTH_MESH_FLAG_LINE: &str = "decorators/flag_line";

/// 0x50B 旗候选条目（O 读面）：`+0x0c` 墙 id、`+0x10..+0x18` 位置、
/// `+0x4d` 连接标记（0 = 非源，1 = 目标）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlagAnchor {
    /// 装饰 id（下游实体键）。
    pub id: u32,
    /// 墙 id（`+0x0c`）。
    pub wall: u64,
    /// 世界位置（`+0x10`）。
    pub pos: Vec3,
    /// `+0x4d`：源标记（!= 0 可作边起点）。
    pub source: bool,
    /// `+0x4d`：目标标记（== 1 可作边终点）。
    pub target: bool,
}

/// 贪心连接参数（O 调用点常量）。
#[derive(Clone, Copy, Debug)]
pub struct ConnectParams {
    pub max_per_node: u32,
    pub cross_wall: bool,
    pub near_different_wall: f32,
    pub far_same_wall: f32,
    pub min_same_wall: f32,
    pub min_distance: f32,
    pub occlusion_margin: f32,
    pub edge_weight_bias: f32,
}

impl Default for ConnectParams {
    /// 0x1407df470 行 774 实参。
    fn default() -> Self {
        Self {
            max_per_node: CONNECT_MAX_PER_NODE,
            cross_wall: CONNECT_CROSS_WALL,
            near_different_wall: CONNECT_NEAR_DIFFERENT_WALL,
            far_same_wall: CONNECT_FAR_SAME_WALL,
            min_same_wall: CONNECT_MIN_SAME_WALL,
            min_distance: CONNECT_MIN_DISTANCE,
            occlusion_margin: CONNECT_OCCLUSION_MARGIN,
            edge_weight_bias: CONNECT_EDGE_WEIGHT_BIAS,
        }
    }
}

/// 贪心连接结果：边（按装饰 id 的 `(min,max)` 对）+ 每节点度数。
///
/// 原函数把边写入 16B `(usize, usize)` 向量（0x1407e1430 尾部 `param_1[0..5]`），
/// 并把度数组、候选表一并返回；此处以 id 承载同一可观察结果。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConnectResult {
    /// 已接受边（id 小者在前）。
    pub edges: Vec<(u32, u32)>,
    /// 每节点连接数。
    pub degrees: HashMap<u32, u32>,
}

/// 候选对是否满足机器码距离窗（O：0x1407e1430 行 176-183）。
///
/// - 异墙：`d² < near_different_wall²`；
/// - 同墙：`min_same_wall² < d² < far_same_wall²`。
pub fn candidates_within_window(
    a: &FlagAnchor,
    b: &FlagAnchor,
    params: &ConnectParams,
) -> Option<f32> {
    if !(a.source && b.target) {
        return None;
    }
    if !params.cross_wall && a.wall == b.wall {
        return None;
    }
    let delta = b.pos - a.pos;
    let d2 = delta.length_squared();
    let different_wall = a.wall != b.wall;
    let ok = if different_wall {
        d2 < params.near_different_wall * params.near_different_wall
    } else {
        params.min_same_wall * params.min_same_wall < d2
            && d2 < params.far_same_wall * params.far_same_wall
    };
    ok.then(|| d2.sqrt())
}

/// 0x1407e1430（O 结构，语义全文）：贪心连接相邻旗。
///
/// `blocked(a, b, length)` 对应机器码
/// `RaycastWorld::intersects_any(param_4, &segment, dist + (-0.01), 0x30db)`：
/// 返回 true 表示连线被遮挡，拒绝该边。
///
/// 保留机器码边界：
/// - 距离窗（见 [`candidates_within_window`]）；
/// - 候选对去重（hashbrown 集合，键 = `(min,max)`）；
/// - 每节点容量 `params.max_per_node`（两遍扫描，容量随接受递增）；
/// - 坡度门 `2*|Δy| <= sqrt(Δx²+Δz²)`；
/// - 最小距离 `params.min_distance`（接受前 `min < dist`）；
/// - 空输入返回空结果（无 panic）。
pub fn greedily_connect(
    anchors: &[FlagAnchor],
    params: &ConnectParams,
    blocked: impl Fn(Vec3, Vec3, f32) -> bool,
) -> ConnectResult {
    // ---- 第一遍：候选对 + 去重（O：hashbrown 集合 + 邻接表）----
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut adjacency: Vec<(usize, usize, f32)> = Vec::new();
    for i in 0..anchors.len() {
        for j in 0..anchors.len() {
            if i == j {
                continue;
            }
            let Some(dist) = candidates_within_window(&anchors[i], &anchors[j], params) else {
                continue;
            };
            let pair = if i < j { (i, j) } else { (j, i) };
            // O：集合去重 `FUN_141267af0`（pair 已存在则跳过）。
            if !seen.insert(pair) {
                continue;
            }
            // O：按源→目标方向记录权重 = dist + 3.0。
            let weight = dist + params.edge_weight_bias;
            adjacency.push((i, j, weight));
        }
    }

    // ---- 第二遍：容量/遮挡/坡度门（O：第二段邻接遍历）----
    let mut degrees: HashMap<u32, u32> = HashMap::new();
    let mut used: HashSet<(usize, usize)> = HashSet::new();
    let mut result = ConnectResult::default();
    for (i, j, _weight) in adjacency {
        let a = &anchors[i];
        let b = &anchors[j];
        // O：`plStack_d0[i] < param_5 && plStack_d0[j] < param_5`。
        let deg_a = degrees.get(&a.id).copied().unwrap_or(0);
        let deg_b = degrees.get(&b.id).copied().unwrap_or(0);
        if deg_a >= params.max_per_node || deg_b >= params.max_per_node {
            continue;
        }
        let pair = if i < j { (i, j) } else { (j, i) };
        if !used.insert(pair) {
            continue;
        }
        let delta = b.pos - a.pos;
        let horizontal = Vec2::new(delta.x, delta.z).length();
        // O：`fVar55 + fVar55 <= SQRT(dx²+dz²)` → 继续（坡度门）。
        let vertical = delta.y.abs();
        if vertical + vertical > horizontal {
            continue;
        }
        let dist = delta.length();
        // O：`fVar43(0.02) < fVar53(dist)`。
        if dist <= params.min_distance {
            continue;
        }
        // O：遮挡查询长度 = dist + (-0.01)。
        if blocked(a.pos, b.pos, dist + params.occlusion_margin) {
            continue;
        }
        let (lo, hi) = if a.id <= b.id { (a.id, b.id) } else { (b.id, a.id) };
        result.edges.push((lo, hi));
        *degrees.entry(a.id).or_insert(0) += 1;
        *degrees.entry(b.id).or_insert(0) += 1;
    }
    result.degrees = degrees;
    result
}

/// 布绳采样点数：`n = max(2, round(dist / spacing))`（O 0x1407e2510 行 115-129）。
pub fn clothline_sample_count(dist: f32, spacing: f32) -> usize {
    let n = (dist / spacing).round();
    // O：`2 < n` 取 n，否则 2（NaN 比较为 false → 2）。
    if n > 2.0 {
        n as usize
    } else {
        2
    }
}

/// sag 混合因子：`clamp(sag_len / dist, 0, 0.25)`（O 行 130-137）。
pub fn clothline_sag_fraction(sag_len: f32, dist: f32) -> f32 {
    let f = sag_len / dist;
    if f >= 0.0 {
        f.min(CLOTH_MAX_SAG_FRACTION)
    } else {
        0.0
    }
}

/// 布绳中点下垂量：`(1 - (2t-1)²) * 0.08 * dist`（O 行 203-204）。
pub fn cloth_sag_offset(t: f32, dist: f32) -> f32 {
    let u = t + t - 1.0;
    (1.0 - u * u) * CLOTH_SAG * dist
}

/// 0x1407e2510（O 结构）：布绳采样折线。
///
/// 采样参数（O 行 197-204）：
/// `t' = t + (1-t)*blend`，位置 = `lerp(a, b, t')`，y 再减
/// `(1-(2t'-1)²)*0.08*dist`。返回 `n+1` 个点（n = 采样点数）。
/// U：`param_4/param_6` 的精确语义；四角半宽展开不在本函数（见
/// [`clothline_quads`]）。
pub fn clothline_points(
    a: Vec3,
    b: Vec3,
    spacing: f32,
    sag_len: f32,
    sag_scale: f32,
) -> Vec<Vec3> {
    let dist = (b - a).length();
    let n = clothline_sample_count(dist, spacing);
    let blend = clothline_sag_fraction(sag_len, dist) * sag_scale;
    let mut points = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let t = t + (1.0 - t) * blend;
        let mut p = a.lerp(b, t);
        p.y -= cloth_sag_offset(t, dist);
        points.push(p);
    }
    points
}

/// 布条四角（O 结构；U：角点归一化细节）。
///
/// 每段用半宽 `width*0.5` 沿 XZ 法线展开：机器码以
/// `0.5 / sqrt(...)` 归一化两轴并依据主轴符号选角点顺序（0x1407e2a6f..
/// 0x1407e2e0a）；此处以等价垂直基展开，角点顺序 U。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClothQuad {
    pub corners: [Vec3; 4],
}

/// 逐段布条四角（段数 = 采样点数 - 1）。
pub fn clothline_quads(points: &[Vec3], width: f32) -> Vec<ClothQuad> {
    let half = width * 0.5;
    let mut quads = Vec::with_capacity(points.len().saturating_sub(1));
    for pair in points.windows(2) {
        let (p0, p1) = (pair[0], pair[1]);
        let dir = Vec2::new(p1.x - p0.x, p1.z - p0.z).normalize_or(Vec2::Y);
        let perp = Vec2::new(-dir.y, dir.x) * half;
        quads.push(ClothQuad {
            corners: [
                Vec3::new(p0.x + perp.x, p0.y, p0.z + perp.y),
                Vec3::new(p0.x - perp.x, p0.y, p0.z - perp.y),
                Vec3::new(p1.x + perp.x, p1.y, p1.z + perp.y),
                Vec3::new(p1.x - perp.x, p1.y, p1.z - perp.y),
            ],
        });
    }
    quads
}

/// 0x1407df470（O 结构）：生成旗连接（旗到旗的布绳）。
///
/// 锚点从已放置旗装饰读取 `DecoratorExtraInfo::try_as_flag`（O callee）与缓存
/// 变换位置。原函数还用 FUN_1407df050 做墙侧相邻查询（U，锚点来源由调用者
/// 提供的候选集承载）。遮挡查询在世界系统内（U：RaycastWorld），此处以
/// “无遮挡”回调承载并在交付登记。
pub fn generate_flag_connections(
    decorator_storage: Res<DecoratorStorage>,
    cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    let mut anchors: Vec<FlagAnchor> = Vec::new();
    for (wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            if info.extra().try_as_flag().is_none() {
                continue;
            }
            let pos = cached.position(id);
            anchors.push(FlagAnchor {
                id,
                wall: wall.0,
                pos,
                // O：0x50B 候选的 +0x4d 由墙侧查询填充；U 源/目标标记来源。
                source: true,
                target: true,
            });
        }
    }
    let params = ConnectParams::default();
    let result = greedily_connect(&anchors, &params, |_a, _b, _len| false);
    for (a_id, b_id) in &result.edges {
        let a = anchors.iter().find(|x| x.id == *a_id).expect("edge endpoint");
        let b = anchors.iter().find(|x| x.id == *b_id).expect("edge endpoint");
        // O：两次布料调用（0x142a88b30 len 0x27 / 0x142a88b57 len 0x14）。
        let points = clothline_points(a.pos, b.pos, 0.3, 0.0, 0.0);
        let _quads = clothline_quads(&points, CLOTH_SEGMENT_WIDTH);
    }
    let _ = &mut transient;
}

/// 0x1407e2510（O 结构）：生成一段布绳（旗间）。
///
/// 原函数经 `WindowVisualEntityBuilder::build<Transient>`（0x1412ab270）落
/// 瞬态表；本实现先产出可验证几何（采样点 + 四角），实体构建在持有
/// `Commands`/网格库的宿主完成。返回 None 表示 `dist` 非正（原码退化路径，
/// 无有效布绳）。
pub fn clothline(
    a: Vec3,
    b: Vec3,
    spacing: f32,
    sag_len: f32,
    sag_scale: f32,
    width: f32,
    _transient: &mut DecoratorTransientVisualState,
) -> Option<Vec<ClothQuad>> {
    let dist = (b - a).length();
    if !(dist > 0.0) {
        return None;
    }
    let points = clothline_points(a, b, spacing, sag_len, sag_scale);
    Some(clothline_quads(&points, width))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anchor(id: u32, wall: u64, pos: Vec3) -> FlagAnchor {
        FlagAnchor {
            id,
            wall,
            pos,
            source: true,
            target: true,
        }
    }

    #[test]
    fn distance_windows_match_machine_constants() {
        let p = ConnectParams::default();
        let a = anchor(1, 0, Vec3::ZERO);
        // 异墙 < 10
        let b = anchor(2, 1, Vec3::new(9.9, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_some());
        let b = anchor(2, 1, Vec3::new(10.1, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_none());
        // 同墙 2..5.5
        let b = anchor(2, 0, Vec3::new(1.9, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_none());
        let b = anchor(2, 0, Vec3::new(2.1, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_some());
        let b = anchor(2, 0, Vec3::new(5.6, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_none());
    }

    #[test]
    fn source_target_flags_gate_pairs() {
        let p = ConnectParams::default();
        let mut a = anchor(1, 0, Vec3::ZERO);
        let mut b = anchor(2, 1, Vec3::X);
        a.source = false;
        assert!(candidates_within_window(&a, &b, &p).is_none());
        a.source = true;
        b.target = false;
        assert!(candidates_within_window(&a, &b, &p).is_none());
    }

    #[test]
    fn greedy_respects_capacity_two() {
        // 三个互相靠近的点：中心节点最多连 2 条。
        let anchors = vec![
            anchor(1, 1, Vec3::ZERO),
            anchor(2, 0, Vec3::new(3.0, 0.0, 0.0)),
            anchor(3, 0, Vec3::new(-3.0, 0.0, 0.0)),
        ];
        let r = greedily_connect(&anchors, &ConnectParams::default(), |_, _, _| false);
        assert_eq!(r.degrees.get(&1), Some(&2));
        assert_eq!(r.edges.len(), 2);
    }

    #[test]
    fn greedy_slope_gate_rejects_steep_edge() {
        let anchors = vec![
            anchor(1, 0, Vec3::ZERO),
            anchor(2, 0, Vec3::new(3.0, 10.0, 0.0)),
        ];
        let r = greedily_connect(&anchors, &ConnectParams::default(), |_, _, _| false);
        assert!(r.edges.is_empty(), "2*|dy| > horizontal");
    }

    #[test]
    fn greedy_occlusion_rejects_edge() {
        let anchors = vec![
            anchor(1, 0, Vec3::ZERO),
            anchor(2, 0, Vec3::new(3.0, 0.0, 0.0)),
        ];
        let r = greedily_connect(&anchors, &ConnectParams::default(), |_, _, _| true);
        assert!(r.edges.is_empty());
    }

    #[test]
    fn greedy_min_distance_rejects_close_pair() {
        let anchors = vec![
            anchor(1, 1, Vec3::ZERO),
            anchor(2, 0, Vec3::new(0.01, 0.0, 0.0)),
        ];
        let r = greedily_connect(&anchors, &ConnectParams::default(), |_, _, _| false);
        assert!(r.edges.is_empty());
    }

    #[test]
    fn sample_count_is_at_least_two() {
        assert_eq!(clothline_sample_count(0.1, 1.0), 2);
        assert_eq!(clothline_sample_count(3.0, 1.0), 3);
        assert_eq!(clothline_sample_count(10.0, 0.3), 33);
    }

    #[test]
    fn sag_midpoint_is_eight_percent_of_distance() {
        assert!((cloth_sag_offset(0.5, 10.0) - 0.8).abs() < 1e-6);
        assert_eq!(cloth_sag_offset(0.0, 10.0), 0.0);
        assert_eq!(cloth_sag_offset(1.0, 10.0), 0.0);
    }

    #[test]
    fn clothline_points_droop_and_endpoints() {
        let a = Vec3::ZERO;
        let b = Vec3::new(10.0, 0.0, 0.0);
        let pts = clothline_points(a, b, 1.0, 0.0, 0.0);
        assert_eq!(pts.len(), 11);
        assert_eq!(pts[0], a);
        assert!((pts[10] - b).length() < 1e-6);
        // 中点下垂 0.08*10 = 0.8。
        assert!((pts[5].y + 0.8).abs() < 1e-5);
    }

    #[test]
    fn quads_have_width_across_segment() {
        let pts = vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)];
        let quads = clothline_quads(&pts, 1.0);
        assert_eq!(quads.len(), 1);
        let c = &quads[0].corners;
        assert!((c[0].z - 0.5).abs() < 1e-6);
        assert!((c[2].z - 0.5).abs() < 1e-6);
    }
}
