//! 半木构纹理生成（utils/half_timber.rs）。
//!
//! 原 B 源：`crates/systems/wall-constructor/src/utils/half_timber.rs`。
//!
//! `half_timbering`（0x141218e30，27KB / 5494 条指令）的 pdg 前导已解码，
//! 但逐梁（plank）坐标生成/closure 装配（~0x6860B）仍无证据，保持 U。
//!
//! 窗工具三件（0x1412218a0 / 0x141221da0 / 0x141221f70）的几何/常量/分支
//! 已按反编译逐语句恢复；装饰器存储遍历（SIMD 掩码、subtype 过滤）属于
//! `country-core::resources::walls::decorator_storage` 的跨 crate API，
//! 这里以 `WindowAabbSource` 显式传入已取出的窗记录（调用方负责过滤），
//! 不在本 crate 伪造存储布局。
//!
//! 常量（rne-read 直读 .rdata）：
//! - 0x142c1c880 = 0.038、0x142c1c884 = −0.038、0x142c1c888 = −0.14、
//!   0x142a80d40 = 0.14；
//! - 0x1429cd068 = 0.2、0x1429a0764 = 0.46、0x1429df16c = −0.2、
//!   0x142925970 = 0.4、0x142c1c88c = 0.093。
use glam::Vec2;
use glam::Vec3;
use utils::geometry::aabb::Aabb2;

/// 墙形状点列（try_get_wall_shape 输出）。
pub type WallShapePoints = [[f32; 3]];

/// B `half_timbering`（0x141218e30）。**未实现（U）**：前导已由 radare2 pdg
/// 解码（puffin 门控、try_get_wall_shape、gable_sides/is_flat、WoodenPlankSsbo
/// 0x3d 名校验、+0x180 LOCK 握手、split_height_into_skirts），逐梁坐标/closure
/// 中段无证据（asm 5281 行已落盘），不得以近似体冒充。
pub fn half_timbering(
    _wall_shape: Option<&WallShapePoints>,
    _u: &[f32],
    _heights: &[f32],
    _gable_sides: Option<[u8; 2]>,
    _is_flat: bool,
    _out: &mut Vec<[u8; 64]>,
) {
    todo!(
        "未实现: half_timbering 0x141218e30（pdg 前导已解码，逐梁 plank 坐标/closure 装配未取证）"
    )
}

/// B `roof_outline`（0x141217be0）：is_gable 时的屋顶轮廓半木构。
///
/// **未实现（U）**：反编译已解出 4 侧循环（uVar19 0..3）、
/// `profile_curve_normalized`/`profile_curve_ws`、gable sides、法线/切线、
/// 0.05/0.13/0.25 等常量与 64B 记录装配，但 SSBO 写缓冲目标记录布局与
/// 若干 .rdata 常量语义未闭合；不得以空壳通过。
pub fn roof_outline(_roof: &[u8; 88], _points: &[Vec3], _out: &mut Vec<[u8; 64]>) {}

/// 窗装饰器记录的安全输入（调用方从 DecoratorStorage 取出）。
///
/// `raw` 的原件读取偏移：+0x0c（leaf 标志）、+0x0e（subtype 族）、
/// +0x44（子类型/朝向字节）、+0x46（窗类型字节 '8'/'9'/':'）。
#[derive(Clone, Copy, Debug)]
pub struct WindowAabbSource<'a> {
    /// 装饰器原始记录（至少 0x50 字节；这里保留 0x50）。
    pub raw: [u8; 0x50],
    /// `DecoratorInfo::dst_as_wall` 的挂点 (x0, y0)。
    pub attachment: [f32; 2],
    /// `DerivedDecoratorInfo::dims_wall_space` 的三浮点结果 (a, b, c)。
    pub bbx: [f32; 3],
    /// gothic profile 的墙面坐标点（raw[0x46]=='9' 时使用）。
    pub profile: &'a [[f32; 2]],
}

impl Default for WindowAabbSource<'_> {
    fn default() -> Self {
        Self {
            raw: [0; 0x50],
            attachment: [0.0; 2],
            bbx: [0.0; 3],
            profile: &[],
        }
    }
}

/// B `get_window_aabb`（0x141221da0，逐语句重建）。
///
/// - `raw[0x46] == '9'`（门类）：profile 末点 (px, py)、首点 (fx, fy)；
///   `aabb = [x0 − px, y0 + py, x0 + px, fy + 0.093 + y0]`。
///   `subtype == 9` 走 `gothic_door_profile_curve`，否则
///   `gothic_half_dormer_profile_curve(raw[0x44])`（两者由调用方在 profile
///   中给出——曲线函数属装饰器 crate）。
/// - 其他：
///   `width = max(bbx[0] + 0.2, 0.46)`；
///   `height = bbx[2] − bbx[1]`；
///   `raw[0x46]=='8'` → height += 0.2；
///   `raw[0x46]==':'` → `raw[0x44]==3` 时 width −= 0.2，height += 0.4；
///   `Aabb2::from_center_dims(center, (width, height))`。
pub fn get_window_aabb(window: &WindowAabbSource) -> Aabb2 {
    let subtype_tag = window.raw[0x46];
    if subtype_tag == b'9' {
        let (first, last) = match (window.profile.first(), window.profile.last()) {
            (Some(first), Some(last)) => (first, last),
            _ => return Aabb2::from_center_dims(Vec2::ZERO, Vec2::ZERO),
        };
        let [x0, y0] = window.attachment;
        return Aabb2 {
            min: Vec2::new(x0 - last[0], y0 + last[1]),
            max: Vec2::new(x0 + last[0], first[1] + f32::from_bits(0x3dbe76c9) + y0),
        };
    }
    let mut width = window.bbx[0] + f32::from_bits(0x3e4ccccd); // +0.2
    if width <= f32::from_bits(0x3eeb851f) {
        width = f32::from_bits(0x3eeb851f); // 0.46
    }
    let mut height = window.bbx[2] - window.bbx[1];
    if subtype_tag == b'8' {
        height += f32::from_bits(0x3e4ccccd); // +0.2
    } else if subtype_tag == b':' {
        if window.raw[0x44] == 3 {
            width += f32::from_bits(0xbe4ccccd); // −0.2
        }
        height += f32::from_bits(0x3ecccccd); // +0.4
    }
    let center = Vec2::from_array(window.attachment);
    Aabb2::from_center_dims(center, Vec2::new(width, height))
}

/// B `overlapping_windows`（0x1412218a0）：对已取出的窗集合做 AABB 访问测试，
/// 返回 `ResourceAccessInfo` 投影（20B：aabb 16B + leaf + subtype + shifted）。
///
/// 调用方已完成原件 SIMD 掩码/子类型过滤（`byte[10] < 2 &&
/// (byte[0xe] − 0x38) < 3`）；本函数按原件逐窗展开与三向移位测试。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowAccessInfo {
    pub aabb: [f32; 4],
    /// 原 `pauStack_110[-1][0xc]`。
    pub leaf: u8,
    /// `DerivedDecoratorInfo::subtype`。
    pub subtype: u8,
    /// 0 = 直接相交；1 = 侧向移位后相交。
    pub shifted: u8,
}

impl WindowAccessInfo {
    pub fn observed_bytes(&self) -> [u8; 20] {
        let mut b = [0u8; 20];
        for (i, v) in self.aabb.iter().enumerate() {
            b[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        b[16] = self.leaf;
        b[17] = self.subtype;
        b[18] = self.shifted;
        b
    }
}

fn aabb_array(aabb: Aabb2) -> [f32; 4] {
    [aabb.min.x, aabb.min.y, aabb.max.x, aabb.max.y]
}

/// B `overlapping_windows`（0x1412218a0）。
///
/// `query` 为待测墙 AABB（归一化/墙面空间），`expanded_limit` 为原件
/// `param_6`（0.038 外的横向可用余量）。
pub fn overlapping_windows(
    query: Aabb2,
    expanded_limit: f32,
    windows: &[WindowAabbSource],
    subtypes: &[u8],
) -> Vec<WindowAccessInfo> {
    let mut out = Vec::new();
    for (window, &subtype) in windows.iter().zip(subtypes) {
        let mut aabb = get_window_aabb(window);
        // 原件只调整 x 两界：min.x +0.038、max.x −0.038（向内收 3.8cm）。
        aabb.min.x += f32::from_bits(0x3d1ba5e3);
        aabb.max.x += f32::from_bits(0xbd1ba5e3);
        if subtype == b'\t' && window.raw[0x0c] == 1 {
            // dormer 'g' 分支：min.x −0.14、max.x +0.14。
            aabb.min.x += f32::from_bits(0xbe0f5c29);
            aabb.max.x += f32::from_bits(0x3e0f5c29);
        }
        let leaf = window.raw[0x0c];
        if query.intersects(aabb) {
            out.push(WindowAccessInfo {
                aabb: aabb_array(aabb),
                leaf,
                subtype,
                shifted: 0,
            });
            continue;
        }
        // 原件第二段：中心 ± 半宽 与 expanded_limit 比较后左右移位。
        let center = aabb.center();
        let half = aabb.half_dims();
        let shifted = if (center.x + half.x) <= expanded_limit {
            if half.x <= center.x {
                continue;
            }
            Aabb2::from_center_dims(
                Vec2::new(center.x + expanded_limit, center.y),
                Vec2::new(aabb.width(), aabb.height()),
            )
        } else {
            Aabb2::from_center_dims(
                Vec2::new(center.x - expanded_limit, center.y),
                Vec2::new(aabb.width(), aabb.height()),
            )
        };
        if query.intersects(shifted) {
            out.push(WindowAccessInfo {
                aabb: aabb_array(shifted),
                leaf,
                subtype,
                shifted: 1,
            });
        }
    }
    out
}

/// B `check_for_overlap_with_all_windows`（0x141221f70）：对每个窗调用
/// `get_window_aabb`，先测查询点是否在 AABB 内；否则按 `expansion` 左右
/// 移位中心 x（保持 y/宽高），再次测点。返回任一命中。
pub fn check_for_overlap_with_all_windows(
    point: Vec2,
    expansion: f32,
    windows: &[WindowAabbSource],
) -> bool {
    for window in windows {
        let aabb = get_window_aabb(window);
        if aabb.contains(point) {
            return true;
        }
        let center = aabb.center();
        let half_width = aabb.width() * 0.5;
        let shifted_x = if expansion < center.x + half_width {
            center.x - expansion
        } else if half_width <= center.x {
            continue;
        } else {
            center.x + expansion
        };
        let shifted = Aabb2::from_center_dims(
            Vec2::new(shifted_x, point.y),
            Vec2::new(aabb.width(), aabb.height()),
        );
        if shifted.contains(point) {
            return true;
        }
    }
    false
}

/// B `add_underwindow_pattern`（0x141fd1c30）：窗下图案。
/// **未实现（U）**：反编译为 1907B，前导已解（`raw[0x11]&0x1e != 0x16`
/// 门控、窗高差、±0.16/0.1/0.2 常量族、plank 发射），但逐梁循环与 0x60B
/// 记录装配未逐语句闭合；不写空实现冒充完成。
pub fn add_underwindow_pattern(_window: &[f32; 4], _out: &mut Vec<[u8; 64]>) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn window<'a>(tag: u8) -> WindowAabbSource<'a> {
        let mut raw = [0u8; 0x50];
        raw[0x46] = tag;
        WindowAabbSource {
            raw,
            attachment: [1.0, 2.0],
            bbx: [0.5, 0.1, 1.1],
            profile: &[],
        }
    }

    #[test]
    fn plain_window_uses_max_width_and_height_delta() {
        let aabb = get_window_aabb(&window(b'7'));
        // width = max(0.5+0.2, 0.46)=0.7；height=1.1−0.1=1.0。
        assert!((aabb.width() - 0.7).abs() < 1e-6);
        assert!((aabb.height() - 1.0).abs() < 1e-6);
        assert_eq!(aabb.center(), Vec2::new(1.0, 2.0));
    }

    #[test]
    fn subtype_8_and_colon_adjust_height() {
        let tall = get_window_aabb(&window(b'8'));
        assert!((tall.height() - 1.2).abs() < 1e-6);
        let mut colon = window(b':');
        colon.raw[0x44] = 3;
        let aabb = get_window_aabb(&colon);
        assert!((aabb.height() - 1.4).abs() < 1e-6);
        assert!((aabb.width() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn width_clamps_to_046() {
        let mut w = window(b'7');
        w.bbx[0] = 0.1;
        assert!((get_window_aabb(&w).width() - 0.46).abs() < 1e-6);
    }

    #[test]
    fn gothic_profile_builds_from_last_and_first_points() {
        let profile = [[0.1, 0.2], [0.3, 0.5]];
        let mut w = window(b'9');
        w.profile = &profile;
        let aabb = get_window_aabb(&w);
        assert!((aabb.min.x - (1.0 - 0.3)).abs() < 1e-6);
        assert!((aabb.min.y - (2.0 + 0.5)).abs() < 1e-6);
        assert!((aabb.max.x - (1.0 + 0.3)).abs() < 1e-6);
        assert!((aabb.max.y - (0.2 + 0.093 + 2.0)).abs() < 1e-5);
    }

    #[test]
    fn overlap_check_detects_inside_and_shifted_points() {
        let w = window(b'7');
        assert!(check_for_overlap_with_all_windows(
            Vec2::new(1.0, 2.0),
            0.1,
            &[w]
        ));
        // 移位后中心 x=0.9，区间 [0.55,1.25]；点 1.2 命中。
        assert!(check_for_overlap_with_all_windows(
            Vec2::new(1.2, 2.0),
            0.1,
            &[w]
        ));
        assert!(!check_for_overlap_with_all_windows(
            Vec2::new(4.0, 4.0),
            0.1,
            &[w]
        ));
    }

    #[test]
    fn overlapping_windows_reports_shifted_flag() {
        let w = window(b'7');
        // expansion=2.0：center+half(1.35) ≤ 2.0 → 右移中心到 3.0，
        // 区间 [2.65,3.35]；查询 [2.95,3.05] 命中且带 shifted=1。
        let query = Aabb2::from_center_dims(Vec2::new(3.0, 2.0), Vec2::new(0.1, 2.0));
        let hits = overlapping_windows(query, 2.0, &[w], &[b'7']);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].shifted, 1);
        let bytes = hits[0].observed_bytes();
        assert_eq!(bytes[17], b'7');
    }
}
