//! B Roof component and its observed 0x58-byte game fields.
//! Geometry is shared with the recovered system-roof crate. The byte projection
//! preserves appearance fields whose original Rust enum schema is not yet known.

use crate::resources::walls::WallId;
use bevy_ecs::component::Component;
use glam::Vec3;

#[derive(Component, Clone, Debug)]
pub struct Roof(pub(crate) [u8; 88]);

impl Roof {
    pub fn from_observed(bytes: [u8; 88]) -> Self {
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 88] {
        &self.0
    }
    pub fn wall_id(&self) -> WallId {
        WallId(u64::from_le_bytes(self.0[..8].try_into().unwrap()))
    }
    pub fn floor_y(&self) -> f32 {
        self.float_at(0x4c)
    }
    pub fn height_01(&self) -> f32 {
        self.float_at(0x30)
    }
    pub fn is_pitched(&self) -> bool {
        ty_observed(self.height_01())
    }
    pub fn is_flat(&self) -> bool {
        self.height_01() <= f32::from_bits(0x3dcccccd)
    }
    pub fn is_flat_w_no_rim(&self) -> bool {
        self.is_flat() && self.0[0x3d] == 0
    }
    pub fn is_gable(&self) -> bool {
        is_gable_observed(self.height_01(), self.float_at(0x34))
    }
    pub fn gable_sides(&self) -> Option<[u8; 2]> {
        self.is_gable().then_some([self.0[0x3c], self.0[0x3c] | 2])
    }
    pub fn height(&self) -> f32 {
        system_roof::surface::height(&self.0)
    }
    pub fn pivot_3d(&self) -> Vec3 {
        let tag = u32::from_le_bytes(self.0[8..12].try_into().unwrap()) as usize;
        assert!(tag < 2, "unsupported Roof enclosure");
        Vec3::new(
            self.float_at(12 + tag * 8),
            self.floor_y(),
            self.float_at(16 + tag * 8),
        )
    }
    pub fn shape_dims(&self) -> [f32; 2] {
        if self.0[8] & 1 == 0 {
            let diameter = self.float_at(0x14) + self.float_at(0x14);
            [diameter; 2]
        } else {
            [self.float_at(0x1c), self.float_at(0x20)]
        }
    }
    pub fn shape_at(&self, coordinate: f32) -> system_roof::surface::RoofSection {
        system_roof::surface::section(&self.0, coordinate)
    }
    pub fn parameters(&self) -> system_roof::parameters::RoofParameterBlock {
        system_roof::parameters::RoofParameterBlock::from_roof(&self.0)
    }
    /// `Roof::profile_curve_normalized`（0x1408e3420，O 209 案例）：20 个原始 f32 常量 u，
    /// 纵坐标 u^q（q=1.5p+(1−p)0.5），末点强制 (1,1)。= `system_roof::surface::profile_curve_normalized`。
    pub fn profile_curve_normalized(&self) -> [[f32; 2]; 20] {
        system_roof::surface::profile_curve_normalized(self.float_at(0x2c))
    }
    /// `Roof::roof_tip_offset_xz`（0x1408e3780，O）：circle→2r·tip·0.5；rect→旋转
    /// (tip·dim·0.5)。= `system_roof::ridge::roof_tip_offset_xz_observed`。
    pub fn roof_tip_offset_xz(&self) -> [f32; 2] {
        system_roof::ridge::roof_tip_offset_xz_observed(&self.0)
    }
    /// `Roof::profile_curve_ws`（0x1408e3d40，O 1,512 全链案例）：把 20 点变
    /// `(u·top+(1−u)·bottom, v·height)`，重算 Curve2 弧长/归一。
    /// = `system_roof::surface::world_profile_curve`。
    pub fn profile_curve_ws(
        &self,
        bottom: f32,
        top: f32,
        height: f32,
    ) -> Result<system_roof::surface::Curve2, system_roof::surface::CurveError> {
        system_roof::surface::world_profile_curve(self.float_at(0x2c), bottom, top, height)
    }
    /// `Roof::rotation_yaw_radians`（0x1408e3b10，O）：矩形→`atan2(−frame.z, frame.x)`
    /// （frame.z 符号位翻转 0x1429258f0）；圆→`−π/2 − CircularF32(center_y).as_f32()`
    /// （常量 _UNK_142a10ff0 = −π/2，0xbfc90fdb rne-read）。
    pub fn rotation_yaw_radians(&self) -> f32 {
        if self.0[8] & 1 == 0 {
            -std::f32::consts::FRAC_PI_2
                - utils::circular_f32::CircularF32::new(self.float_at(0x18)).as_f32()
        } else {
            let frame_x = self.float_at(0xc);
            let frame_z = self.float_at(0x10);
            (-frame_z).atan2(frame_x)
        }
    }
    /// `Roof::roof_shape_at_intersection_y`（0x1408e3820，O）：高度 y 处截面——
    /// y==floor 直接拷帧体（本实现以 t=0 截面近似）；y 越界 → [`RoofIntersectionSection::OutOfBounds`]；
    /// 否则 `t=(y−floor)/cap` 处截面（借 system_roof surface::section）。
    pub fn roof_shape_at_intersection_y(&self, y: f32) -> RoofIntersectionSection {
        let b = &self.0;
        let floor = self.float_at(0x4c);
        if y == floor {
            return RoofIntersectionSection::Section(system_roof::surface::section(b, 0.0));
        }
        let cap = cap_height(b);
        if y < floor || cap + floor <= y {
            return RoofIntersectionSection::OutOfBounds;
        }
        let t = (y - floor) / cap;
        RoofIntersectionSection::Section(system_roof::surface::section(b, t))
    }
    fn float_at(&self, offset: usize) -> f32 {
        f32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }
}

/// `Roof::roof_shape_at_intersection_y` 产出：截面或高度越界（tag 2）。
/// （`system_roof::surface::RoofSection` 不 impl PartialEq，故手写无 PartialEq。）
#[derive(Clone, Debug)]
pub enum RoofIntersectionSection {
    /// 圆/矩截面（system_roof 同构）。
    Section(system_roof::surface::RoofSection),
    /// 高度越界（原 tag 2）。
    OutOfBounds,
}

/// `ridge_length_01_from_rectangle_ratio`（0x1408e3b60，O）：矩形纵横比→ridge_length_01：
/// `ratio=max(w/h,h/w)`、`u=clamp((max_dim−2.25)/0.94736844,0,1)`、
/// `blend=(1.5·smoothstep(u)+ratio−1.5)/1.5`、返回 `smoothstep(clamp(blend,0,1))`
/// （常量 _UNK_142aa50dc=0.94736844、_UNK_1429df180=−1.5、1.5 rne-read）。
pub fn ridge_length_01_from_rectangle_ratio(dim_x: f32, dim_z: f32) -> f32 {
    let ratio_xz = dim_x / dim_z;
    let ratio_zx = dim_z / dim_x;
    let ratio = sse_max(ratio_zx, ratio_xz);
    let max_dim = sse_max(dim_x, dim_z);
    let u = sse_min(1.0, sse_max(0.0, (max_dim + -2.25) / 0.94736844));
    let smooth = (3.0 - 2.0 * u) * u * u;
    let blended = (1.5 * smooth + ratio + -1.5) / 1.5;
    let t = sse_min(1.0, sse_max(0.0, blended));
    (3.0 - 2.0 * t) * t * t
}

/// `roof_height_ws_from_wall_shape`（0x1408e3c70，O）：墙形状→屋顶世界高度——
/// 尺寸归一后经 `EasingFunction::SineInOut`（tag 2 @0x142aa5938）采样，映射高度带
/// （circle low=10/high=4、rect low=6/high=3，常量 rne-read），钳 [0,12]。
/// 墙形状字节 = {+0x0 tag bit0, circle 半径 @+0xc, rect dims @+0x14/+0x18}（decomp）。
pub fn roof_height_ws_from_wall_shape(wall_shape: &[u8]) -> f32 {
    let (dim_n, low, high) = if wall_shape[0] & 1 == 0 {
        ((at(wall_shape, 0xc) + -0.9) / 7.1, 10.0, 4.0)
    } else {
        (
            (sse_max(at(wall_shape, 0x14), at(wall_shape, 0x18)) + -2.25) / 12.75,
            6.0,
            3.0,
        )
    };
    let u = utils::easing_functions::EasingFunction::SineInOut.apply(dim_n);
    (low * u + (1.0 - u) * high).clamp(0.0, 12.0)
}

/// `roof_height_01_from_wall_shape`（内联 444，I）：`height_ws` 的 01 归一
/// （= 上式先 /12 再钳 [0,1]）。
pub fn roof_height_01_from_wall_shape(wall_shape: &[u8]) -> f32 {
    let ws = roof_height_ws_from_wall_shape(wall_shape);
    (ws / 12.0).clamp(0.0, 1.0)
}

/// Roof 高度上限 cap（roof_shape_at_intersection_y / get_height_01 共用块）。
fn cap_height(b: &[u8; 88]) -> f32 {
    let height_01 = at(b, 0x30);
    if !(0.1 < height_01) {
        return 0.0;
    }
    let (dim_n, base) = if b[8] & 1 == 0 {
        ((at(b, 0x14) + -0.9) / 7.1, 16.0)
    } else {
        ((sse_max(at(b, 0x1c), at(b, 0x20)) + -2.25) / 12.75, 9.0)
    };
    let t = sse_min(1.0, sse_max(0.0, dim_n / 0.4));
    let smooth = (3.0 - (t + t)) * t * t;
    let cap_base = smooth * 12.0 + (1.0 - smooth) * base;
    assert!(cap_base >= 0.0, "original height cap assertion");
    sse_min(cap_base, sse_max(0.0, height_01 * 12.0 + (1.0 - height_01) * 0.4))
}

/// `RoofShapeParams`（28 字节参数块；B 序列化映射 parameters.rs 已建立）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofShapeParams([u8; 28]);
impl RoofShapeParams {
    pub fn from_roof(roof: &[u8; 88]) -> Self {
        let mut bytes = [0u8; 28];
        bytes.copy_from_slice(&roof[0x24..0x40]);
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 28] {
        &self.0
    }
    /// `RoofShapeParams::eave_length_ws`（0x1408e3c30，O 2,500 案例）：
    /// `(sqrt(p)+(1−sqrt(p))·0.5)·(1.5e+(1−e)·0)`。
    pub fn eave_length_ws(&self) -> f32 {
        system_roof::surface::eave_length_ws(at(&self.0, 8), at(&self.0, 20))
    }
    /// `RoofShapeParams::height_ws`（内联 394，I）：参数高度→世界高度
    /// （= `Roof::height` 的 `h·12+(1−h)·0.4` 上游，无尺寸 cap）。
    pub fn height_ws(&self) -> f32 {
        let h = at(&self.0, 12);
        h * 12.0 + (1.0 - h) * 0.4
    }
    pub fn profile_01(&self) -> f32 {
        at(&self.0, 8)
    }
    pub fn height_01(&self) -> f32 {
        at(&self.0, 12)
    }
    pub fn ridge_length_01(&self) -> f32 {
        at(&self.0, 16)
    }
    pub fn ridge_dir_bit(&self) -> u32 {
        u32::from_le_bytes(self.0[24..28].try_into().unwrap())
    }
}

/// `GableRoofStyle`（B 风格枚举；serde 变体串 "Matching"/"Override"，8 字节）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GableRoofStyle {
    #[serde(rename = "Matching")]
    Matching,
    #[serde(rename = "Override")]
    Override,
}
impl GableRoofStyle {
    /// `is_stepped`（0x140833230，O）：`tag==0`（与 wall 谓词共享去重体）。
    pub fn is_stepped(self) -> bool {
        matches!(self, GableRoofStyle::Matching)
    }
    /// `is_overhang`（0x1408c5680，O）：`tag!=0`。
    pub fn is_overhang(self) -> bool {
        !matches!(self, GableRoofStyle::Matching)
    }
}

/// `FlatRoofStyle` 观察投影（12B = {tag u8, pattern u8, payload}）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlatRoofStyle([u8; 12]);
impl FlatRoofStyle {
    pub fn from_observed(bytes: [u8; 12]) -> Self {
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 12] {
        &self.0
    }
    pub fn tag(&self) -> u8 {
        self.0[0]
    }
    /// `pattern_id`（内联 86，I）：风格花纹 id（匹配序列化串）= 字节 +1。
    pub fn pattern_id(&self) -> u8 {
        self.0[1]
    }
    /// `is_wooden`（0x1408e31a0，I 手解）：tag0 → other；tag1 → false；tag2 → field@+1。
    pub fn is_wooden(&self, other: bool) -> bool {
        match self.tag() {
            0 => other,
            1 => false,
            _ => self.0[1] != 0,
        }
    }
}

pub fn ty_observed(value_at_0x30: f32) -> bool {
    let threshold = f32::from_bits(0x3dcccccd);
    // UCOMISS + SETC: 无序比较（NaN）也设置 CF。
    threshold < value_at_0x30 || threshold.is_nan() || value_at_0x30.is_nan()
}

pub fn is_gable_observed(value_at_0x30: f32, value_at_0x34: f32) -> bool {
    // CMPNLEPS（含无序比较）与 CMPEQPS 的低 lane 按位相与。
    ty_observed(value_at_0x30) && value_at_0x34 == 1.0
}

fn at(b: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes(b[offset..offset + 4].try_into().unwrap())
}
fn sse_max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
fn sse_min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

// 原入口的八参数观察合同保持不变。
#[allow(clippy::too_many_arguments)]
pub fn new_observed(
    output: &mut [u8; 0x58],
    arg2: &[u8; 8],
    arg3: &[u8; 28],
    arg4: &[u8; 28],
    arg5: &[u8; 4],
    arg6: &[u8; 4],
    arg7: u8,
    arg8: &[u8; 12],
) {
    output[0..8].copy_from_slice(arg2);
    output[0x24..0x40].copy_from_slice(arg3);
    output[8..0x24].copy_from_slice(arg4);
    output[0x4c..0x50].copy_from_slice(arg5);
    output[0x50..0x54].copy_from_slice(arg6);
    output[0x54] = arg7;
    output[0x40..0x4c].copy_from_slice(arg8);
    // 原作没有写入 +0x55..+0x57。0x58 来自调用者组件步长，非完整 Rust 类型证明。
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roof_with(bytes: &[(usize, u32)]) -> Roof {
        let mut b = [0u8; 88];
        for (offset, value) in bytes {
            b[*offset..*offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        Roof::from_observed(b)
    }

    #[test]
    fn gable_style_predicates() {
        assert!(GableRoofStyle::Matching.is_stepped());
        assert!(!GableRoofStyle::Matching.is_overhang());
        assert!(!GableRoofStyle::Override.is_stepped());
        assert!(GableRoofStyle::Override.is_overhang());
    }

    #[test]
    fn flat_style_is_wooden_by_tag() {
        let tag0 = FlatRoofStyle::from_observed([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(tag0.is_wooden(true));
        assert!(!tag0.is_wooden(false));
        let tag1 = FlatRoofStyle::from_observed([1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!tag1.is_wooden(true), "tag1 恒 false");
        let tag2 = FlatRoofStyle::from_observed([2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(tag2.is_wooden(false), "tag2 → field@+1");
        let tag2_zero = FlatRoofStyle::from_observed([2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(!tag2_zero.is_wooden(true));
        assert_eq!(tag2.pattern_id(), 1);
    }

    #[test]
    fn rotation_yaw_rect_frame() {
        // frame {cos=1, sin=0} → atan2(−0, 1)=0。
        let roof = roof_with(&[(8, 1), (0xc, f32::to_bits(1.0)), (0x10, 0)]);
        assert!(roof.rotation_yaw_radians().abs() < 1e-6);
        // frame {cos=0, sin=1} → atan2(−1, 0)=−π/2。
        let roof2 = roof_with(&[(8, 1), (0xc, 0), (0x10, f32::to_bits(1.0))]);
        assert!((roof2.rotation_yaw_radians() + std::f32::consts::FRAC_PI_2).abs() < 1e-5);
    }

    #[test]
    fn ridge_length_ratio_bounds() {
        // 正方形 → ratio=1、max_dim=小 → 尺寸项 0 → blend=(0+1−1.5)/1.5=−0.333 → 钳 0 → smoothstep 0。
        let square = ridge_length_01_from_rectangle_ratio(3.0, 3.0);
        assert!((0.0..=1.0).contains(&square));
        // 极端长条 → ratio 大 → 趋于 1。
        let strip = ridge_length_01_from_rectangle_ratio(20.0, 2.0);
        assert!((0.0..=1.0).contains(&strip));
    }

    #[test]
    fn roof_height_ws_from_wall_shape_ranges() {
        // circle 半径 10 (+0xc = 0x41200000) → dim_n=(10−0.9)/7.1=1.28 → SineInOut(1.28)≈0.5。
        let circle = roof_height_ws_from_wall_shape(&[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x41, 0x20, 0x00, 0x00,
        ]);
        assert!((0.0..=12.0).contains(&circle));
        // rect tag=1：dim_x@+0x14=4.0(0x40800000)、dim_z@+0x18=6.0(0x40c00000)。
        let rect = roof_height_ws_from_wall_shape(&[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x40, 0x80, 0, 0,
            0x40, 0xc0, 0, 0,
        ]);
        assert!((0.0..=12.0).contains(&rect));
        assert!(roof_height_01_from_wall_shape(&[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x41, 0x20, 0x00, 0x00
        ]) <= 1.0);
    }

    #[test]
    fn shape_params_eave_and_height() {
        let mut bytes = [0u8; 28];
        bytes[8..12].copy_from_slice(&f32::to_bits(0.5).to_le_bytes());   // profile
        bytes[20..24].copy_from_slice(&f32::to_bits(0.5).to_le_bytes());  // eave
        bytes[12..16].copy_from_slice(&f32::to_bits(0.5).to_le_bytes());  // height
        let params = RoofShapeParams(bytes);
        let eave_ws = params.eave_length_ws();
        let expected =
            (0.5f32.sqrt() + (1.0 - 0.5f32.sqrt()) * 0.5) * (1.5 * 0.5 + (1.0 - 0.5) * 0.0);
        assert!((eave_ws - expected).abs() < 1e-6);
        assert!((params.height_ws() - (0.5 * 12.0 + 0.5 * 0.4)).abs() < 1e-6);
    }

    #[test]
    fn shape_at_intersection_y_sections() {
        // 平顶（height_01 0.05）：cap=0 → y 越界（除 y==floor）。
        let flat = roof_with(&[(0x30, 0x3d4ccccd), (8, 0), (0x14, f32::to_bits(3.0))]);
        match flat.roof_shape_at_intersection_y(5.0) {
            RoofIntersectionSection::OutOfBounds => {}
            other => panic!("flat roof at y=5 should be out of bounds, got {other:?}"),
        }
        // 坡顶（height_01 0.5, rect）：floor 处出截面。
        let pitched = roof_with(&[
            (0x30, f32::to_bits(0.5)),
            (0x2c, f32::to_bits(0.5)),
            (0x38, f32::to_bits(0.5)),
            (0x1c, f32::to_bits(4.0)),
            (0x20, f32::to_bits(6.0)),
            (8, 1),
        ]);
        match pitched.roof_shape_at_intersection_y(0.0) {
            RoofIntersectionSection::Section(_) => {}
            other => panic!("pitched roof at floor should yield a section, got {other:?}"),
        }
    }
}
