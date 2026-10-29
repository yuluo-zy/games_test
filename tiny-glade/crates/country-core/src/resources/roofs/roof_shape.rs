//! B Roof component and its observed 0x58-byte game fields.
//! Geometry is shared with the recovered system-roof crate. The byte projection
//! retains the existing runtime layout; Serde projects the original named fields.

use crate::resources::walls::WallId;
use bevy_ecs::component::Component;
use glam::Vec3;
use ::utils::bytes::read_f32_le as at;
use ::utils::scalar::{clamp, clamp01, max_f32};
use serde::ser::{SerializeStruct, SerializeStructVariant};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Component, Clone, Debug)]
#[component(on_insert = crate::render::extract::record_component_change::<Self>,
    on_remove = crate::render::extract::record_component_change::<Self>)]
pub struct Roof(pub(crate) [u8; 88]);

impl Roof {
    pub fn from_observed(bytes: [u8; 88]) -> Self {
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 88] {
        &self.0
    }
    /// 可写观测投影（system-roof 的编辑/方向系统消费）。
    pub fn observed_mut(&mut self) -> &mut [u8; 88] {
        &mut self.0
    }
    pub fn wall_id(&self) -> WallId {
        WallId(u64::from_le_bytes(self.0[..8].try_into().unwrap()))
    }
    /// B `Roof::new`（0x1408e31c0；`new_observed` 的 8 参数面）：
    /// {wall_id, params(0x24..0x40), shape(0x08..0x24), floor(+0x4c),
    ///  color_id(+0x50), gable style(+0x54), flat style(+0x40..0x4c)}。
    /// `animation`/`gable_data` retain the existing parameter names; the native
    /// serializer identifies their stored fields as color_id / flat_roof_style.
    pub fn new(
        wall: [u8; 8],
        params: [u8; 28],
        shape: [u8; 28],
        floor: [u8; 4],
        animation: [u8; 4],
        style_tag: u8,
        gable_data: [u8; 12],
    ) -> Self {
        let mut bytes = [0; 88];
        bytes[..8].copy_from_slice(&wall);
        bytes[0x24..0x40].copy_from_slice(&params);
        bytes[8..0x24].copy_from_slice(&shape);
        bytes[0x4c..0x50].copy_from_slice(&floor);
        bytes[0x50..0x54].copy_from_slice(&animation);
        bytes[0x54] = style_tag;
        bytes[0x40..0x4c].copy_from_slice(&gable_data);
        Self(bytes)
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
        // B1408e3390：SETNE检查原版本字节非零，不能按no_rim名称反推极性。
        self.is_flat() && self.0[0x3d] != 0
    }
    pub fn is_gable(&self) -> bool {
        is_gable_observed(self.height_01(), self.float_at(0x34))
    }
    pub fn gable_sides(&self) -> Option<[u8; 2]> {
        self.is_gable().then_some([self.0[0x3c], self.0[0x3c] | 2])
    }
    pub fn height(&self) -> f32 {
        crate::resources::roofs::geometry::surface::height(&self.0)
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
    pub fn shape_at(
        &self,
        coordinate: f32,
    ) -> crate::resources::roofs::geometry::surface::RoofSection {
        crate::resources::roofs::geometry::surface::section(&self.0, coordinate)
    }
    pub fn parameters(&self) -> crate::resources::roofs::geometry::parameters::RoofParameterBlock {
        crate::resources::roofs::geometry::parameters::RoofParameterBlock::from_roof(&self.0)
    }
    /// `Roof::profile_curve_normalized`（0x1408e3420，O 209 案例）：20 个原始 f32 常量 u，
    /// 纵坐标 u^q（q=1.5p+(1−p)0.5），末点强制 (1,1)。= `crate::resources::roofs::geometry::surface::profile_curve_normalized`。
    pub fn profile_curve_normalized(&self) -> [[f32; 2]; 20] {
        crate::resources::roofs::geometry::surface::profile_curve_normalized(self.float_at(0x2c))
    }
    /// `Roof::roof_tip_offset_xz`（0x1408e3780，O）：circle→2r·tip·0.5；rect→旋转
    /// (tip·dim·0.5)。= `crate::resources::roofs::geometry::ridge::roof_tip_offset_xz_observed`。
    pub fn roof_tip_offset_xz(&self) -> [f32; 2] {
        crate::resources::roofs::geometry::ridge::roof_tip_offset_xz_observed(&self.0)
    }
    /// `Roof::profile_curve_ws`（0x1408e3d40，O 1,512 全链案例）：把 20 点变
    /// `(u·top+(1−u)·bottom, v·height)`，重算 Curve2 弧长/归一。
    /// = `crate::resources::roofs::geometry::surface::world_profile_curve`。
    pub fn profile_curve_ws(
        &self,
        bottom: f32,
        top: f32,
        height: f32,
    ) -> Result<
        crate::resources::roofs::geometry::surface::Curve2,
        crate::resources::roofs::geometry::surface::CurveError,
    > {
        crate::resources::roofs::geometry::surface::world_profile_curve(
            self.float_at(0x2c),
            bottom,
            top,
            height,
        )
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
    /// y==floor 直接拷 `roof[8..0x24]` 28 字节原始帧体（机器码 0x1408e3820 y==floor 分支
    /// 逐 8 字节拷贝：out[0..]=roof[8..]、out[0xc..]=roof[0x14..]、out[0x14..]=roof[0x1c..]，
    /// 覆盖 out[0..0x1c]=roof[8..0x24]；旧实现以 t=0 截面近似、末 8 字节=计算出的 bottom 尺寸≠原始 dims）；
    /// y 越界 → [`RoofIntersectionSection::OutOfBounds`]；
    /// 否则 `t=(y−floor)/cap` 处截面（借 system_roof surface::section）。
    pub fn roof_shape_at_intersection_y(&self, y: f32) -> RoofIntersectionSection {
        let b = &self.0;
        let floor = self.float_at(0x4c);
        if y == floor {
            // fresh 0x1408e3820 y==floor 分支：逐 8 字节拷贝
            // out[0..]=roof[8..]、out[8..]=roof[0x10..]、out[0xc..]=roof[0x14..]、
            // out[0x14..]=roof[0x1c..] → out[0..0x1c] = roof[8..0x24] 全部 28 字节原样。
            // 用原始帧体变体保留全部字节（tag word@roof[8]、Circle 尾部@out[0x14..0x1c]）。
            let mut raw = [0u8; 28];
            raw.copy_from_slice(&b[8..0x24]);
            return RoofIntersectionSection::RawSection(raw);
        }
        let cap = cap_height(b);
        if !(y >= floor) || !(cap + floor > y) {
            return RoofIntersectionSection::OutOfBounds;
        }
        // 原1408e39ba是inverse_lerp(floor, floor+height, y)，大floor下
        // 不能约掉两次加减；且这里是未扩檐的建筑截面，不是visual surface。
        let t = (y - floor) / ((cap + floor) - floor);
        let profile = self.float_at(0x2c);
        let exponent = 1.5 * profile + (1.0 - profile) * 0.5;
        let complement = 1.0 - t.powf(1.0 / exponent);
        use crate::resources::roofs::geometry::surface::RoofSection;
        let section = if b[8] & 1 == 0 {
            RoofSection::Circle {
                frame: [self.float_at(0xc), self.float_at(0x10)],
                radius: complement * self.float_at(0x14),
                word_0x18: u32::from_le_bytes(b[0x18..0x1c].try_into().unwrap()),
            }
        } else {
            let x = self.float_at(0x1c);
            let z = self.float_at(0x20);
            let ridge = self.float_at(0x34);
            let inset = (1.0 - ridge) * 0.1;
            let (top_x, top_z) = if b[0x3c] == 0 {
                (ridge * x + inset, 0.1)
            } else {
                (0.1, ridge * z + inset)
            };
            // 原两次补数都会舍入，不以pow结果直接替换。
            let blend = 1.0 - complement;
            let bottom = 1.0 - blend;
            let dimensions = [top_x * blend + x * bottom, top_z * blend + z * bottom];
            let axis = [self.float_at(0xc), self.float_at(0x10)];
            // 使用已恢复的同一个原Rectangle2d构造器，保finite/单位轴失败。
            let _validated = utils::geometry::Rectangle2d::new(
                glam::Vec2::new(self.float_at(0x14), self.float_at(0x18)),
                glam::Vec2::from_array(dimensions), glam::Vec2::from_array(axis));
            RoofSection::Rectangle {
                frame: [axis[0], axis[1], self.float_at(0x14), self.float_at(0x18)],
                dimensions,
            }
        };
        RoofIntersectionSection::Section(section)
    }
    fn float_at(&self, offset: usize) -> f32 {
        f32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }
}

/// `Roof::roof_shape_at_intersection_y` 产出：截面或高度越界（tag 2）。
/// （`crate::resources::roofs::geometry::surface::RoofSection` 不 impl PartialEq，故手写无 PartialEq。）
#[derive(Clone, Debug)]
pub enum RoofIntersectionSection {
    /// y==floor：原样 28 字节帧体 `roof[8..0x24]`（fresh 0x1408e3820 逐 8 字节拷贝；
    /// 含 tag word 全部字节与 Circle 尾部，非解码近似）。
    RawSection([u8; 28]),
    /// 圆/矩截面（system_roof 同构）。
    Section(crate::resources::roofs::geometry::surface::RoofSection),
    /// 高度越界（原 tag 2）。
    OutOfBounds,
}

/// `ridge_length_01_from_rectangle_ratio`（0x1408e3b60，O）：矩形纵横比→ridge_length_01：
/// `ratio=max(w/h,h/w)`、`u=clamp((max_dim−2.25)/5.25,0,1)`、
/// `blend=(1.5·smoothstep(u)+ratio−1.5)/1.5`、返回 `smoothstep(clamp(blend,0,1))`。
/// （reviewer-C 机器码裁决：除数 **5.25**（0x142aa50dc 实测 0x40a80000）；
/// 0.94736844 系相邻常量读错地址，旧注释 rne-read 声称作废。）
pub fn ridge_length_01_from_rectangle_ratio(dim_x: f32, dim_z: f32) -> f32 {
    let ratio_xz = dim_x / dim_z;
    let ratio_zx = dim_z / dim_x;
    let ratio = max_f32(ratio_zx, ratio_xz);
    let max_dim = max_f32(dim_x, dim_z);
    let u = clamp01((max_dim + -2.25) / 5.25);
    let smooth = (3.0 - 2.0 * u) * u * u;
    let blended = (1.5 * smooth + ratio + -1.5) / 1.5;
    let t = clamp01(blended);
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
            (max_f32(at(wall_shape, 0x14), at(wall_shape, 0x18)) + -2.25) / 12.75,
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
    if height_01 <= 0.1 {
        return 0.0;
    }
    // reviewer-C 轮 3 终验：circle 高度基 = **8.0**
    // （roof_shape_at_intersection_y 0x1408e3820 用 _UNK_142934604 = 8.0；
    // 轮 2 只改了 roof_visual_shape，此处漏改——现已统一）。
    let (dim_n, base) = if b[8] & 1 == 0 {
        ((at(b, 0x14) + f32::from_bits(0xbf66_6667)) / 7.1, 8.0)
    } else {
        ({ let x = at(b, 0x1c); let z = at(b, 0x20);
            let greatest = if x.is_nan() || z > x { z } else { x };
            (greatest + -2.25) / 12.75 }, 9.0)
    };
    let x = dim_n / 0.4;
    let lower = if 0.0 > x { 0.0 } else { x };
    let t = if 1.0 < lower { 1.0 } else { lower };
    let smooth = (3.0 - (t + t)) * (t * t);
    let cap_base = smooth * 12.0 + (1.0 - smooth) * base;
    assert!(cap_base >= 0.0, "original height cap assertion");
    let wanted = height_01 * 12.0 + (1.0 - height_01) * 0.4;
    let lower = if 0.0 > wanted { 0.0 } else { wanted };
    if cap_base < lower { cap_base } else { lower }
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
        crate::resources::roofs::geometry::surface::eave_length_ws(at(&self.0, 8), at(&self.0, 20))
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

/// B style byte: 0 = Stepped, 1 = Overhang. The existing Rust variant names
/// remain API-compatible; their serialized names follow the native schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GableRoofStyle {
    #[serde(rename = "Stepped")]
    Matching,
    #[serde(rename = "Overhang")]
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

/// B FlatRoofStyle: tag byte0; Override has material byte1, ColorId u32@4,
/// and pattern_id u32@8. Inactive payload and padding remain uninterpreted.
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
    /// Legacy byte observer retained for source compatibility. This reads the
    /// material byte; native serialized `pattern_id` is the independent u32@+8.
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

// B capture 0x1417a2252 -> collect_seq 0x14158faf0 -> Roof serializer
// 0x1416ea700. This is a logical Serde projection, not a native ABI declaration.
impl Serialize for Roof {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::resources::color_id::ColorId;
        use crate::systems::wall::valid_enclosure::ShapeParameters;

        let mut state = serializer.serialize_struct("Roof", 7)?;
        state.serialize_field("id", &self.wall_id().0)?;
        state.serialize_field("params", &RoofShapeParams::from_roof(&self.0))?;
        state.serialize_field(
            "wall_shape",
            &ShapeParameters::from_observed(self.0[8..0x24].try_into().unwrap()),
        )?;
        state.serialize_field("pivot_y", &self.floor_y())?;
        state.serialize_field(
            "color_id",
            &ColorId(u32::from_le_bytes(self.0[0x50..0x54].try_into().unwrap())),
        )?;
        let gable_style = if self.0[0x54] == 0 {
            GableRoofStyle::Matching
        } else {
            GableRoofStyle::Override
        };
        state.serialize_field("gable_style", &gable_style)?;
        state.serialize_field(
            "flat_roof_style",
            &FlatRoofStyle::from_observed(self.0[0x40..0x4c].try_into().unwrap()),
        )?;
        state.end()
    }
}

// B 0x1416eaa40: all seven fields are emitted, including the independent rim byte.
impl Serialize for RoofShapeParams {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("RoofShapeParams", 7)?;
        state.serialize_field("profile_01", &self.profile_01())?;
        state.serialize_field("height_01", &self.height_01())?;
        state.serialize_field(
            "tip_offset_01",
            &glam::Vec2::new(at(&self.0, 0), at(&self.0, 4)),
        )?;
        state.serialize_field("ridge_length_01", &self.ridge_length_01())?;
        let direction = if self.0[24] == 0 {
            tg_history::RidgeDirection::Width
        } else {
            tg_history::RidgeDirection::Length
        };
        state.serialize_field("ridge_dir", &direction)?;
        state.serialize_field("eave_length_01", &at(&self.0, 20))?;
        let rim = if self.0[25] == 0 { "Rim" } else { "NoRim" };
        state.serialize_field("flat_rim_style", rim)?;
        state.end()
    }
}

// B 0x1416e8f20. The unit variants never read inactive union payload/padding.
impl Serialize for FlatRoofStyle {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.tag() {
            0 => serializer.serialize_unit_variant("FlatRoofStyle", 0, "Matching"),
            1 => serializer.serialize_unit_variant("FlatRoofStyle", 1, "MatchingStoneOnly"),
            _ => {
                let mut state =
                    serializer.serialize_struct_variant("FlatRoofStyle", 2, "Override", 3)?;
                let color = crate::resources::color_id::ColorId(u32::from_le_bytes(
                    self.0[4..8].try_into().unwrap(),
                ));
                state.serialize_field("color_id", &color)?;
                let material = if self.0[1] == 0 { "Stone" } else { "Wood" };
                state.serialize_field("material", material)?;
                state.serialize_field(
                    "pattern_id",
                    &u32::from_le_bytes(self.0[8..12].try_into().unwrap()),
                )?;
                state.end()
            }
        }
    }
}

// B reader 0x1415c5460 / 0x1415a9230. These private wire views are transient
// Serde projections; the existing 88/28/12-byte runtime owners stay authoritative.
#[derive(Deserialize)]
#[serde(rename = "Roof")]
struct SavedRoof {
    id: u64,
    params: RoofShapeParams,
    wall_shape: crate::systems::wall::valid_enclosure::ShapeParameters,
    pivot_y: f32,
    color_id: u32,
    #[serde(default = "saved_gable_default")]
    gable_style: GableRoofStyle,
    #[serde(default = "saved_flat_default")]
    flat_roof_style: FlatRoofStyle,
}

fn saved_gable_default() -> GableRoofStyle {
    GableRoofStyle::Matching
}

fn saved_flat_default() -> FlatRoofStyle {
    let mut bytes = [0; 12];
    bytes[0] = 1; // MatchingStoneOnly, not Matching.
    FlatRoofStyle(bytes)
}

impl<'de> Deserialize<'de> for Roof {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let saved = SavedRoof::deserialize(deserializer)?;
        Ok(Self::new(
            saved.id.to_le_bytes(),
            saved.params.0,
            *saved.wall_shape.bytes(),
            saved.pivot_y.to_le_bytes(),
            saved.color_id.to_le_bytes(),
            match saved.gable_style {
                GableRoofStyle::Matching => 0,
                GableRoofStyle::Override => 1,
            },
            saved.flat_roof_style.0,
        ))
    }
}

// B 0x1416eadf0 accepts the three older names; 0x1415c7ed0 only requires
// profile/height/tip, then fills the four independent compatibility defaults.
#[derive(Deserialize)]
#[serde(rename = "RoofShapeParams")]
struct SavedRoofShapeParams {
    #[serde(alias = "profile")]
    profile_01: f32,
    #[serde(alias = "height")]
    height_01: f32,
    #[serde(alias = "tip_offset")]
    tip_offset_01: glam::Vec2,
    #[serde(default = "saved_ridge_length_default")]
    ridge_length_01: f32,
    #[serde(default = "saved_ridge_direction_default")]
    ridge_dir: tg_history::RidgeDirection,
    #[serde(default = "saved_eave_length_default")]
    eave_length_01: f32,
    #[serde(default = "saved_flat_rim_default")]
    flat_rim_style: SavedFlatRimStyle,
}

type SavedFlatRimStyle = FlatRoofRimStyle;

fn saved_ridge_length_default() -> f32 {
    f32::from_bits(0x3ecccccd) // B @0x142925970: 0.4.
}

fn saved_eave_length_default() -> f32 {
    f32::from_bits(0x3e4ccccd) // B @0x1429cd068: 0.2.
}

fn saved_ridge_direction_default() -> tg_history::RidgeDirection {
    tg_history::RidgeDirection::Width
}

fn saved_flat_rim_default() -> SavedFlatRimStyle {
    SavedFlatRimStyle::Rim
}

impl<'de> Deserialize<'de> for RoofShapeParams {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let saved = SavedRoofShapeParams::deserialize(deserializer)?;
        let mut bytes = [0; 28];
        bytes[..4].copy_from_slice(&saved.tip_offset_01.x.to_le_bytes());
        bytes[4..8].copy_from_slice(&saved.tip_offset_01.y.to_le_bytes());
        bytes[8..12].copy_from_slice(&saved.profile_01.to_le_bytes());
        bytes[12..16].copy_from_slice(&saved.height_01.to_le_bytes());
        bytes[16..20].copy_from_slice(&saved.ridge_length_01.to_le_bytes());
        bytes[20..24].copy_from_slice(&saved.eave_length_01.to_le_bytes());
        bytes[24] = match saved.ridge_dir {
            tg_history::RidgeDirection::Width => 0,
            tg_history::RidgeDirection::Length => 1,
        };
        bytes[25] = match saved.flat_rim_style {
            SavedFlatRimStyle::Rim => 0,
            SavedFlatRimStyle::NoRim => 1,
        };
        // Inactive payload/padding has no wire representation. Initializing it
        // is a host memory-safety choice, not a native birth/default contract.
        Ok(Self(bytes))
    }
}

#[derive(Deserialize)]
#[serde(rename = "FlatRoofStyle")]
enum SavedFlatRoofStyle {
    Matching,
    MatchingStoneOnly,
    Override {
        color_id: u32,
        material: SavedFlatRoofMaterial,
        #[serde(default)]
        pattern_id: u32,
    },
}

#[derive(Deserialize)]
enum SavedFlatRoofMaterial {
    Stone,
    Wood,
}

// B 0x1416e91c0 -> 0x1415d9440: Override requires color/material; pattern
// defaults to zero. Union payload is not read for either unit variant.
impl<'de> Deserialize<'de> for FlatRoofStyle {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut bytes = [0; 12];
        match SavedFlatRoofStyle::deserialize(deserializer)? {
            SavedFlatRoofStyle::Matching => {}
            SavedFlatRoofStyle::MatchingStoneOnly => bytes[0] = 1,
            SavedFlatRoofStyle::Override {
                color_id,
                material,
                pattern_id,
            } => {
                bytes[0] = 2;
                bytes[1] = match material {
                    SavedFlatRoofMaterial::Stone => 0,
                    SavedFlatRoofMaterial::Wood => 1,
                };
                bytes[4..8].copy_from_slice(&color_id.to_le_bytes());
                bytes[8..12].copy_from_slice(&pattern_id.to_le_bytes());
            }
        }
        Ok(Self(bytes))
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

/// B `Roof::clone_behavior`（0x1408e6af0，真实现）：返回 None（观测克隆
/// 行为桩；跨模块共享的 0x140002370 空桩不属本函数）。
pub fn clone_behavior(_roof: &Roof) -> Option<Roof> {
    None
}

/// 失败码 = 原版输出 `*out = {discriminant=1, code}` 的 `code`（machine 0x1408e4090）。
///
/// 机器路径（逐分支）：
///  * 第一表缺键，或槽值 `handle >> 32 == 0`（generation 0）→ **0**（`{1,0}`）；
///  * `world+0x3e10` arena 空 / 第二表缺键 / 槽控制字节 `== 0x11` → **1**（`{1,1}`）；
///  * 形状 cast 失败或 `rapier_hit_to_raycast_world_result` 判别 `== 2`（未命中）→
///    **3**（`*out = 0x0301`）；
///  * 命中但判别 `& 1 == 0`（无 uv）→ **4**（`{1,4}`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoofUvFailure {
    /// 0：`GlobalId` 未注册 / 注册句柄 generation 为 0。
    MissingGlobalId,
    /// 1：arena 空或第二级句柄表缺失/槽空（控制字节 0x11）。
    MissingArena,
    /// 3：形状射线未命中（含 `cast_ray` 判别 4）。
    RayMiss,
    /// 4：命中但 `rapier_hit_to_raycast_world_result` 无 barycentric uv。
    NoUv,
}

impl RoofUvFailure {
    /// 原版写回 `out[1]` 的字节。
    pub const fn code(self) -> u8 {
        match self {
            RoofUvFailure::MissingGlobalId => 0,
            RoofUvFailure::MissingArena => 1,
            RoofUvFailure::RayMiss => 3,
            RoofUvFailure::NoUv => 4,
        }
    }
}

/// 把 `rapier_hit_to_raycast_world_result` 的输出映射为管线结果。
///
/// fresh 0x1408e4090：`local_50[0] != 4`（形状 cast 成功）且判别 `!= 2` 才继续；
/// 之后 `判别 & 1` 门控 uv：成功 → `uv = extra[0..2]`、`point = position`。
pub fn roof_uv_from_raycast_hit(
    hit: &crate::systems::collision::world::RaycastWorldResult,
) -> Result<((f32, f32), glam::Vec3), RoofUvFailure> {
    if hit.is_miss() {
        return Err(RoofUvFailure::RayMiss);
    }
    if hit.discriminant & 1 == 0 {
        return Err(RoofUvFailure::NoUv);
    }
    Ok(((hit.extra[0], hit.extra[1]), hit.position))
}

/// 原1408e4090先校验身份表，再只投射该实例；不查询其它屋顶或整个物理世界。
pub fn roof_uv_and_point_indexed(
    walls: &crate::resources::walls::public_walls::PublicWalls,
    world: &crate::systems::collision::world::RaycastWorld,
    key: WallId,
    xz: glam::Vec2,
) -> Result<(glam::Vec2, glam::Vec3), RoofUvFailure> {
    let entity = walls
        .roof_entity(key)
        .ok_or(RoofUvFailure::MissingGlobalId)?;
    if entity.to_bits() >> 32 == 0 {
        return Err(RoofUvFailure::MissingGlobalId);
    }
    #[cfg(feature = "native-physics")]
    {
        use rapier3d::prelude::{Point, Ray, Vector};
        let instance = world
            .roof_collider_instance(entity)
            .filter(|instance| instance.slot != 17)
            .ok_or(RoofUvFailure::MissingArena)?;
        let collider = world
            .get_rapier_collider_ref(instance.slot, instance.collider)
            .expect("roof UV: physical collider missing");
        // 原先检查物理句柄，再检查完整mesh receipt；不能以miss吞掉失效注册。
        let receipt = collider.user_data as u64;
        // 原1408e430e先OR世代低位，再比较注册表世代；不能把偶数raw世代直接拒绝。
        let mesh = world.get_collider_mesh(receipt as u32, ((receipt >> 32) as u32) | 1);
        let origin = glam::Vec3::new(xz.x, f32::from_bits(0x4780_0000), xz.y);
        let direction = glam::Vec3::new(-0.0, -1.0, -0.0);
        let ray = Ray::new(
            Point::new(origin.x, origin.y, origin.z),
            Vector::new(direction.x, direction.y, direction.z),
        );
        let hit = collider
            .shape()
            .cast_ray_and_get_normal(collider.position(), &ray, f32::MAX, true)
            .ok_or(RoofUvFailure::RayMiss)?;
        let tri = match hit.feature {
            rapier3d::parry::shape::FeatureId::Face(id) => Some(id),
            _ => None,
        };
        let pose = collider.position();
        let converted =
            crate::systems::collision::world::rapier_hit_to_raycast_world_result_at_pose(
                &mesh.data,
                origin,
                direction,
                hit.time_of_impact,
                glam::Vec3::new(hit.normal.x, hit.normal.y, hit.normal.z),
                tri,
                std::array::from_fn(|i| {
                    u64::from_le_bytes(mesh.source.0[i * 8..i * 8 + 8].try_into().unwrap())
                }),
                glam::Vec3::new(pose.translation.x, pose.translation.y, pose.translation.z),
                glam::Quat::from_xyzw(
                    pose.rotation.i,
                    pose.rotation.j,
                    pose.rotation.k,
                    pose.rotation.w,
                ),
            );
        roof_uv_from_raycast_hit(&converted).map(|((u, v), point)| (glam::Vec2::new(u, v), point))
    }
    #[cfg(not(feature = "native-physics"))]
    {
        let _ = (world, xz);
        // 无物理后端不会存在原实体碰撞体表，保留第二级缺失，而不伪造全世界命中。
        Err(RoofUvFailure::MissingArena)
    }
}

/// 原1408e4040仅映射Result的成功值，错误码原样传递。
pub fn roof_uv_at_ws_pos(
    walls: &crate::resources::walls::public_walls::PublicWalls,
    world: &crate::systems::collision::world::RaycastWorld,
    key: WallId,
    xz: glam::Vec2,
) -> Result<glam::Vec2, RoofUvFailure> {
    roof_uv_and_point_indexed(walls, world, key, xz).map(|(uv, _)| uv)
}

pub fn roof_uv_indexed(
    walls: &crate::resources::walls::public_walls::PublicWalls,
    world: &crate::systems::collision::world::RaycastWorld,
    key: WallId,
    xz: glam::Vec2,
) -> Result<glam::Vec2, RoofUvFailure> {
    roof_uv_at_ws_pos(walls, world, key, xz)
}

pub fn roof_uv_and_point_at_ws_pos(
    walls: &crate::resources::walls::public_walls::PublicWalls,
    world: &crate::systems::collision::world::RaycastWorld,
    key: WallId,
    xz: glam::Vec2,
) -> Result<(glam::Vec2, glam::Vec3), RoofUvFailure> {
    roof_uv_and_point_indexed(walls, world, key, xz)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_without_rim_uses_original_nonzero_version_byte() {
        // B1408e3390..33a8的UCOMISS/SETAE和CMP/SETNE；阈值含等号，NaN为假。
        for (height, flat) in [(0.0, true), (0.1, true), (f32::from_bits(0x3dcc_ccce), false),
            (-1.0, true), (f32::NAN, false), (f32::INFINITY, false)] {
            for version in [0, 1, 2, 255] {
                let mut bytes = [0; 88];
                bytes[0x30..0x34].copy_from_slice(&height.to_le_bytes());
                bytes[0x3d] = version;
                assert_eq!(Roof::from_observed(bytes).is_flat_w_no_rim(), flat && version != 0);
            }
        }
    }

    fn roof_with(bytes: &[(usize, u32)]) -> Roof {
        let mut b = [0u8; 88];
        for (offset, value) in bytes {
            b[*offset..*offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        Roof::from_observed(b)
    }

    fn snapshot_roof(shape_tag: u32, flat_tag: u8) -> Roof {
        let mut bytes = [0xa5; 88];
        bytes[..8].copy_from_slice(&0xfedc_ba98_7654_3210_u64.to_le_bytes());
        bytes[8..12].copy_from_slice(&shape_tag.to_le_bytes());
        let shape: [f32; 6] = if shape_tag == 0 {
            [2.0, -3.0, 4.0, 0.0, 999.0, 999.0]
        } else {
            [0.0, 1.0, 2.0, -3.0, 4.0, 5.0]
        };
        for (index, number) in shape.into_iter().enumerate() {
            bytes[12 + index * 4..16 + index * 4].copy_from_slice(&number.to_le_bytes());
        }
        for (index, number) in [0.25_f32, -0.5, 0.75, 0.125, 0.875, 0.5]
            .into_iter()
            .enumerate()
        {
            bytes[0x24 + index * 4..0x28 + index * 4].copy_from_slice(&number.to_le_bytes());
        }
        bytes[0x3c] = 0;
        bytes[0x3d] = 1;
        bytes[0x40] = flat_tag;
        bytes[0x41] = 1;
        bytes[0x44..0x48].copy_from_slice(&0xfedc_ba98_u32.to_le_bytes());
        bytes[0x48..0x4c].copy_from_slice(&0x1234_5678_u32.to_le_bytes());
        bytes[0x4c..0x50].copy_from_slice(&(-2.5_f32).to_le_bytes());
        bytes[0x50..0x54].copy_from_slice(&0x8765_4321_u32.to_le_bytes());
        bytes[0x54] = 0;
        Roof::from_observed(bytes)
    }

    #[test]
    fn snapshot_roof_preserves_named_fields_and_full_integer_widths() {
        let roof = snapshot_roof(0, 2);
        let before = *roof.observed();
        let value = serde_json::to_value(&roof).unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "id": 0xfedc_ba98_7654_3210_u64,
                "params": {
                    "profile_01": 0.75, "height_01": 0.125,
                    "tip_offset_01": [0.25, -0.5], "ridge_length_01": 0.875,
                    "ridge_dir": "Width", "eave_length_01": 0.5,
                    "flat_rim_style": "NoRim"
                },
                "wall_shape": {"Circle": {"center": [2.0, -3.0], "radius": 4.0}},
                "pivot_y": -2.5, "color_id": 0x8765_4321_u32,
                "gable_style": "Stepped",
                "flat_roof_style": {"Override": {
                    "color_id": 0xfedc_ba98_u32, "material": "Wood", "pattern_id": 0x1234_5678_u32
                }}
            })
        );
        assert_eq!(*roof.observed(), before);
        assert_eq!(value.as_object().unwrap().len(), 7);
    }

    #[test]
    fn snapshot_roof_reuses_shape_serde_and_distinct_style_bytes() {
        for shape_tag in 0..=1 {
            for gable in 0..=1 {
                for ridge in 0..=1 {
                    for rim in 0..=1 {
                        for material in 0..=1 {
                            for flat in 0..=2 {
                                let mut roof = snapshot_roof(shape_tag, flat);
                                roof.0[0x3c] = ridge;
                                roof.0[0x3d] = rim;
                                roof.0[0x41] = material;
                                roof.0[0x54] = gable;
                                let value = serde_json::to_value(&roof).unwrap();
                                assert_eq!(
                                    value["gable_style"],
                                    if gable == 0 { "Stepped" } else { "Overhang" }
                                );
                                assert_eq!(
                                    value["params"]["ridge_dir"],
                                    if ridge == 0 { "Width" } else { "Length" }
                                );
                                assert_eq!(
                                    value["params"]["flat_rim_style"],
                                    if rim == 0 { "Rim" } else { "NoRim" }
                                );
                                match flat {
                                    0 => assert_eq!(value["flat_roof_style"], "Matching"),
                                    1 => assert_eq!(value["flat_roof_style"], "MatchingStoneOnly"),
                                    _ => assert_eq!(
                                        value["flat_roof_style"]["Override"]["material"],
                                        if material == 0 { "Stone" } else { "Wood" }
                                    ),
                                }
                                if shape_tag == 1 {
                                    assert_eq!(
                                        value["wall_shape"],
                                        serde_json::json!({"Rectangle": {
                                            "orientation": [0.0, 1.0], "width": 4.0, "length": 5.0, "center": [2.0, -3.0]
                                        }})
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn snapshot_circle_zero_rotation_and_nonfinite_values_use_dependency_serde() {
        for rotation in [0.0_f32, -0.0, 0.75, f32::NAN] {
            let mut roof = snapshot_roof(0, 0);
            roof.0[0x18..0x1c].copy_from_slice(&rotation.to_le_bytes());
            roof.0[0x2c..0x30].copy_from_slice(&f32::INFINITY.to_le_bytes());
            let value = serde_json::to_value(roof).unwrap();
            assert!(value["params"]["profile_01"].is_null());
            let circle = &value["wall_shape"]["Circle"];
            if rotation == 0.0 {
                assert!(circle.get("rotation").is_none());
            } else if rotation.is_nan() {
                assert!(circle["rotation"]["val"].is_null());
            } else {
                assert_eq!(circle["rotation"], serde_json::json!({"val": 0.75}));
            }
        }
    }

    #[test]
    fn snapshot_inactive_flat_payload_and_padding_are_not_json_fields() {
        for tag in [0, 1] {
            let mut first = snapshot_roof(0, tag);
            let second = first.clone();
            first.0[0x41..0x4c].fill(0x5a);
            first.0[0x3e..0x40].fill(0x11);
            first.0[0x55..0x58].fill(0xff);
            assert_eq!(
                serde_json::to_value(first).unwrap(),
                serde_json::to_value(second).unwrap()
            );
        }
        assert_eq!(
            serde_json::to_value(GableRoofStyle::Matching).unwrap(),
            "Stepped"
        );
        assert_eq!(
            serde_json::from_str::<GableRoofStyle>("\"Overhang\"").unwrap(),
            GableRoofStyle::Override
        );
        assert!(serde_json::from_str::<GableRoofStyle>("\"Matching\"").is_err());
    }

    #[test]
    fn snapshot_actual_roof_query_serializes_a_roof_sequence() {
        let mut world = bevy_ecs::world::World::new();
        let first = snapshot_roof(0, 0);
        let mut second = snapshot_roof(1, 1);
        second.0[..8].copy_from_slice(&17_u64.to_le_bytes());
        world.spawn(first);
        world.spawn(second);
        world.spawn_empty();
        let mut query = world.query::<&Roof>();
        let mut roofs: Vec<Roof> = query.iter(&world).cloned().collect();
        roofs.sort_by_key(|roof| roof.wall_id().0);
        let value = serde_json::to_value(&roofs).unwrap();
        assert_eq!(value.as_array().unwrap().len(), 2);
        assert_eq!(value[0]["id"], 17_u64);
        assert_eq!(value[1]["id"], 0xfedc_ba98_7654_3210_u64);
        assert!(value[0]["wall_shape"].get("Rectangle").is_some());
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
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x40, 0x80, 0, 0, 0x40,
            0xc0, 0, 0,
        ]);
        assert!((0.0..=12.0).contains(&rect));
        assert!(
            roof_height_01_from_wall_shape(&[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x41, 0x20, 0x00, 0x00
            ]) <= 1.0
        );
    }

    #[test]
    fn shape_params_eave_and_height() {
        let mut bytes = [0u8; 28];
        bytes[8..12].copy_from_slice(&f32::to_bits(0.5).to_le_bytes()); // profile
        bytes[20..24].copy_from_slice(&f32::to_bits(0.5).to_le_bytes()); // eave
        bytes[12..16].copy_from_slice(&f32::to_bits(0.5).to_le_bytes()); // height
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
            RoofIntersectionSection::RawSection(_) => {}
            other => panic!("pitched roof at floor should yield a raw section, got {other:?}"),
        }
    }

    #[test]
    fn intersection_y_floor_copies_raw_frame_body() {
        // 机器码 0x1408e3820：y==floor 直接拷 roof[8..0x24] 28 字节原始帧体
        // （非 t=0 截面近似——t=0 截面末 8 字节为计算出的 bottom 尺寸）。
        let pitched = roof_with(&[
            (0x30, f32::to_bits(0.5)),
            (0x2c, f32::to_bits(0.5)),
            (0x38, f32::to_bits(0.5)),
            (0x34, f32::to_bits(0.5)),
            (0x1c, f32::to_bits(4.0)),
            (0x20, f32::to_bits(6.0)),
            (8, 1),
            (0xc, f32::to_bits(0.25)),
            (0x10, f32::to_bits(0.75)),
            (0x14, f32::to_bits(1.5)),
            (0x18, f32::to_bits(2.5)),
        ]);
        let expected: [u8; 28] = pitched.observed()[8..0x24].try_into().unwrap();
        match pitched.roof_shape_at_intersection_y(0.0) {
            RoofIntersectionSection::RawSection(raw) => {
                assert_eq!(raw, expected);
                // 逐字段解读：tag word@0、frame@4、dimensions@0x14。
                assert_eq!(u32::from_le_bytes(raw[0..4].try_into().unwrap()), 1);
                for (i, want) in [0.25f32, 0.75, 1.5, 2.5, 4.0, 6.0].iter().enumerate() {
                    let got = f32::from_le_bytes(raw[4 + i * 4..8 + i * 4].try_into().unwrap());
                    assert!((got - want).abs() < 1e-6, "field {i}: {got} != {want}");
                }
            }
            other => panic!("floor intersection must be raw frame bytes, got {other:?}"),
        }
    }

    #[test]
    fn raycast_result_mapping_matches_barycentric_gate() {
        use crate::systems::collision::world::RaycastWorldResult;
        let mut hit = RaycastWorldResult::miss();
        hit.discriminant = 1; // 奇数 = 有 uv
        hit.extra = [0.25, 0.75];
        hit.position = glam::Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            roof_uv_from_raycast_hit(&hit),
            Ok(((0.25, 0.75), glam::Vec3::new(1.0, 2.0, 3.0)))
        );
        hit.discriminant = 0; // 偶数 = 无 uv
        assert_eq!(roof_uv_from_raycast_hit(&hit), Err(RoofUvFailure::NoUv));
        assert_eq!(RoofUvFailure::NoUv.code(), 4);
        let miss = RaycastWorldResult::miss();
        assert_eq!(roof_uv_from_raycast_hit(&miss), Err(RoofUvFailure::RayMiss));
    }
}

#[cfg(test)]
mod original_roof_reader_tests {
    use super::*;
    use serde_json::{Value, json};

    fn required_params() -> Value {
        json!({"profile_01": -2.0, "height_01": 3.0, "tip_offset_01": [0.25, -0.5]})
    }

    fn required_roof() -> Value {
        json!({
            "id": u64::MAX,
            "params": required_params(),
            "wall_shape": {"Circle": {"center": [2.0, -4.0], "radius": 3.0}},
            "pivot_y": -0.0,
            "color_id": u32::MAX
        })
    }

    #[test]
    fn minimal_map_uses_original_defaults_without_clamping_fields() {
        let roof: Roof = serde_json::from_value(required_roof()).unwrap();
        assert_eq!(roof.wall_id().0, u64::MAX);
        assert_eq!(roof.floor_y().to_bits(), (-0.0_f32).to_bits());
        assert_eq!(roof.height_01(), 3.0);
        assert_eq!(roof.0[0x54], 0, "missing gable style is Stepped");
        assert_eq!(roof.0[0x40], 1, "missing flat style is MatchingStoneOnly");
        let params = RoofShapeParams::from_roof(&roof.0);
        assert_eq!(params.profile_01(), -2.0);
        assert_eq!(params.ridge_length_01().to_bits(), 0x3ecccccd);
        assert_eq!(at(params.observed(), 20).to_bits(), 0x3e4ccccd);
        assert_eq!(&params.observed()[24..26], &[0, 0]);
        let value = serde_json::to_value(&roof).unwrap();
        assert_eq!(value["color_id"], u32::MAX);
        assert_eq!(value["gable_style"], "Stepped");
        assert_eq!(value["flat_roof_style"], "MatchingStoneOnly");
        assert_eq!(value["params"]["ridge_dir"], "Width");
        assert_eq!(value["params"]["flat_rim_style"], "Rim");
    }

    #[test]
    fn old_param_aliases_and_unknown_fields_follow_the_original_field_visitor() {
        let params: RoofShapeParams = serde_json::from_value(json!({
            "profile": -1.25, "height": 2.5, "tip_offset": [-0.0, 12.0],
            "unknown": {"nested": [null, true, "ignored"]}
        }))
        .unwrap();
        assert_eq!(params.profile_01(), -1.25);
        assert_eq!(params.height_01(), 2.5);
        assert_eq!(at(params.observed(), 0).to_bits(), (-0.0_f32).to_bits());
        assert_eq!(at(params.observed(), 4), 12.0);
        assert!(
            serde_json::from_str::<RoofShapeParams>(
                r#"{"profile":1,"profile_01":2,"height":3,"tip_offset":[0,0]}"#
            )
            .is_err()
        );
        assert!(serde_json::from_str::<Roof>(
            r#"{"id":1,"id":2,"params":{"profile":1,"height":1,"tip_offset":[0,0]},"wall_shape":{"Circle":{"center":[0,0],"radius":1}},"pivot_y":0,"color_id":0}"#
        ).is_err());
        let mut roof = required_roof();
        roof["unused"] = json!([1, {"anything": true}]);
        assert!(serde_json::from_value::<Roof>(roof).is_ok());
    }

    #[test]
    fn missing_required_null_and_invalid_enum_values_are_rejected() {
        for field in ["id", "params", "wall_shape", "pivot_y", "color_id"] {
            let mut value = required_roof();
            value.as_object_mut().unwrap().remove(field);
            assert!(serde_json::from_value::<Roof>(value).is_err(), "{field}");
        }
        for field in ["profile_01", "height_01", "tip_offset_01"] {
            let mut value = required_params();
            value.as_object_mut().unwrap().remove(field);
            assert!(
                serde_json::from_value::<RoofShapeParams>(value).is_err(),
                "{field}"
            );
        }
        for field in ["gable_style", "flat_roof_style"] {
            let mut value = required_roof();
            value[field] = Value::Null;
            assert!(serde_json::from_value::<Roof>(value).is_err(), "{field}");
        }
        for field in [
            "ridge_length_01",
            "ridge_dir",
            "eave_length_01",
            "flat_rim_style",
        ] {
            let mut value = required_params();
            value[field] = Value::Null;
            assert!(
                serde_json::from_value::<RoofShapeParams>(value).is_err(),
                "{field}"
            );
        }
        for (field, invalid) in [("gable_style", "Matching"), ("flat_roof_style", "Stone")] {
            let mut value = required_roof();
            value[field] = json!(invalid);
            assert!(serde_json::from_value::<Roof>(value).is_err());
        }
        for (field, invalid) in [("ridge_dir", "Diagonal"), ("flat_rim_style", "None")] {
            let mut value = required_params();
            value[field] = json!(invalid);
            assert!(serde_json::from_value::<RoofShapeParams>(value).is_err());
        }
        let mut value = required_roof();
        value["color_id"] = json!(u64::from(u32::MAX) + 1);
        assert!(serde_json::from_value::<Roof>(value).is_err());
        assert!(serde_json::from_str::<Roof>(
            r#"{"id":18446744073709551616,"params":{"profile":1,"height":1,"tip_offset":[0,0]},"wall_shape":{"Circle":{"center":[0,0],"radius":1}},"pivot_y":0,"color_id":0}"#
        ).is_err());
    }

    #[test]
    fn sequence_order_defaults_and_extra_element_guards_match_the_original() {
        let params = json!([0.25, 0.75, [1.5, -2.5]]);
        let shape = json!({"Circle": {"center": [0.0, 0.0], "radius": 1.0}});
        let roof: Roof =
            serde_json::from_value(json!([17, params.clone(), shape.clone(), 9.0, 23])).unwrap();
        assert_eq!(roof.wall_id().0, 17);
        assert_eq!(roof.floor_y(), 9.0);
        assert_eq!((roof.0[0x54], roof.0[0x40]), (0, 1));
        let roof: Roof = serde_json::from_value(json!([
            17,
            params.clone(),
            shape.clone(),
            9.0,
            23,
            "Overhang",
            "Matching"
        ]))
        .unwrap();
        assert_eq!((roof.0[0x54], roof.0[0x40]), (1, 0));
        assert!(
            serde_json::from_value::<Roof>(json!([17, params.clone(), shape.clone(), 9.0]))
                .is_err()
        );
        assert!(
            serde_json::from_value::<Roof>(json!([
                17,
                params.clone(),
                shape,
                9.0,
                23,
                "Stepped",
                "Matching",
                0
            ]))
            .is_err()
        );
        let decoded: RoofShapeParams = serde_json::from_value(json!([
            0.25,
            0.75,
            [1.5, -2.5],
            0.8,
            "Length",
            0.9,
            "NoRim"
        ]))
        .unwrap();
        assert_eq!(decoded.ridge_length_01(), 0.8);
        assert_eq!(at(decoded.observed(), 20), 0.9);
        assert_eq!(&decoded.observed()[24..26], &[1, 1]);
        assert!(serde_json::from_value::<RoofShapeParams>(json!([0.25, 0.75])).is_err());
        assert!(
            serde_json::from_value::<RoofShapeParams>(json!([
                0.25,
                0.75,
                [1.5, -2.5],
                0.8,
                "Length",
                0.9,
                "NoRim",
                0
            ]))
            .is_err()
        );
    }

    #[test]
    fn flat_override_requires_color_and_material_but_defaults_full_width_pattern() {
        for (material, bit) in [("Stone", 0), ("Wood", 1)] {
            let style: FlatRoofStyle = serde_json::from_value(json!({
                "Override": {"color_id": u32::MAX, "material": material}
            }))
            .unwrap();
            assert_eq!(&style.observed()[..2], &[2, bit]);
            assert_eq!(&style.observed()[4..8], &u32::MAX.to_le_bytes());
            assert_eq!(&style.observed()[8..12], &[0; 4]);
            let style: FlatRoofStyle = serde_json::from_value(json!({
                "Override": {"color_id": 1, "material": material, "pattern_id": u32::MAX}
            }))
            .unwrap();
            assert_eq!(&style.observed()[8..12], &u32::MAX.to_le_bytes());
            assert_eq!(
                serde_json::to_value(style).unwrap()["Override"]["pattern_id"],
                u32::MAX
            );
        }
        for value in [
            json!({"Override": {"material": "Stone"}}),
            json!({"Override": {"color_id": 1}}),
            json!({"Override": {"color_id": 1, "material": "Metal"}}),
            json!({"Override": {"color_id": 1, "material": "Stone", "pattern_id": -1}}),
            json!({"Override": {"color_id": 1, "material": "Stone", "pattern_id": 4294967296_u64}}),
            json!({"Override": [1, "Stone", 2]}),
        ] {
            assert!(serde_json::from_value::<FlatRoofStyle>(value).is_err());
        }
    }

    #[test]
    fn actual_world_uses_the_same_deserialized_roof_component_and_shape_owner() {
        use bevy_ecs::prelude::World;
        let value = json!({
            "id": 91, "params": required_params(),
            "wall_shape": {"Rectangle": {
                "orientation": [2.0, 3.0], "width": -4.0,
                "length": 8.0, "center": [5.0, 6.0]
            }}, "pivot_y": 7.0, "color_id": 13,
            "gable_style": "Overhang", "flat_roof_style": "Matching"
        });
        let mut world = World::new();
        let entity = world
            .spawn(serde_json::from_value::<Roof>(value).unwrap())
            .id();
        let roof = world.query::<&Roof>().single(&world).unwrap();
        assert_eq!(roof.wall_id().0, 91);
        assert_eq!(roof.shape_dims(), [-4.0, 8.0]);
        assert_eq!((at(&roof.0, 0xc), at(&roof.0, 0x10)), (2.0, 3.0));
        assert_eq!(world.get::<Roof>(entity).unwrap().floor_y(), 7.0);
        let legacy: Roof = serde_json::from_value({
            let mut value = required_roof();
            value["wall_shape"] = json!({"Circle": {"origin": [8.0, 9.0], "radius": 2.0}});
            value
        })
        .unwrap();
        assert_eq!((at(&legacy.0, 0xc), at(&legacy.0, 0x10)), (8.0, 9.0));
        assert_eq!(at(&legacy.0, 0x18).to_bits(), 0);
    }
}

/// Original B GCC/roof wire uses the same two named rim variants.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlatRoofRimStyle {
    #[default]
    Rim = 0,
    NoRim = 1,
}
impl FlatRoofRimStyle {
    pub fn native_tag(self)->u8 { self as u8 }
}
impl TryFrom<u8> for FlatRoofRimStyle {
    type Error=u8;
    fn try_from(value:u8)->Result<Self,u8> {
        match value {0=>Ok(Self::Rim),1=>Ok(Self::NoRim),_=>Err(value)}
    }
}

impl crate::render::extract::JournaledComponent for Roof {}

#[cfg(test)]
mod dormer_native_intersection_tests {
    use super::*;
    use crate::resources::roofs::geometry::surface::RoofSection;
    fn observed(section: RoofIntersectionSection) -> [u32; 7] {
        let mut bytes = [0xcdu8; 28];
        let mut put = |o: usize, word: u32| bytes[o..o+4].copy_from_slice(&word.to_le_bytes());
        match section {
            RoofIntersectionSection::OutOfBounds => put(0, 2),
            RoofIntersectionSection::RawSection(raw) => bytes = raw,
            RoofIntersectionSection::Section(RoofSection::Circle { frame, radius, word_0x18 }) => {
                put(0, 0); put(4, frame[0].to_bits()); put(8, frame[1].to_bits());
                put(12, radius.to_bits()); put(16, word_0x18);
            }
            RoofIntersectionSection::Section(RoofSection::Rectangle { frame, dimensions }) => {
                put(0, 1);
                for (i, f) in frame.into_iter().chain(dimensions).enumerate() { put(4+i*4, f.to_bits()); }
            }
        }
        std::array::from_fn(|i| u32::from_le_bytes(bytes[i*4..i*4+4].try_into().unwrap()))
    }
    #[test]
    fn original_dormer_intersection_complete_function_keeps_surface_domain_and_failure_bits() {
        // 原RNE1408e3820完整执行；仅Windows CRT powf由宿主C float边界提供。
        // 验证200输入，含两形状、阈值前后、非有限参数、floor原样及真实构造器panic。
        let cases: &[(u8,u32,u32,u32,bool,[u32;7])] = &[
            (0,1028443341,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,0,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1028443341,0,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,1056964608,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1028443341,1056964608,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,1065353216,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1028443341,1065353216,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,2143289344,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1028443341,2143289344,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1028443341,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,0,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1036831949,0,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,1056964608,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1036831949,1056964608,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,1065353216,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1036831949,1065353216,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,2143289344,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1036831949,2143289344,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831949,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,0,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1036831950,0,1065353216,false,[0, 0, 0, 1075236442, 0, 3452816845, 3452816845]),
            (0,1036831950,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,1056964608,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1036831950,1056964608,1065353216,false,[0, 0, 0, 1069009790, 0, 3452816845, 3452816845]),
            (0,1036831950,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,1065353216,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1036831950,1065353216,1065353216,false,[0, 0, 0, 1065573084, 0, 3452816845, 3452816845]),
            (0,1036831950,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,2143289344,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1036831950,2143289344,1065353216,false,[0, 0, 0, 2143289344, 0, 3452816845, 3452816845]),
            (0,1036831950,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1036831950,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,0,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1058642330,0,1065353216,false,[0, 0, 0, 1081820716, 0, 3452816845, 3452816845]),
            (0,1058642330,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,1056964608,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1058642330,1056964608,1065353216,false,[0, 0, 0, 1079850919, 0, 3452816845, 3452816845]),
            (0,1058642330,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,1065353216,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1058642330,1065353216,1065353216,false,[0, 0, 0, 1077696374, 0, 3452816845, 3452816845]),
            (0,1058642330,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,2143289344,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,1058642330,2143289344,1065353216,false,[0, 0, 0, 2143289344, 0, 3452816845, 3452816845]),
            (0,1058642330,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,1058642330,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,0,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,2143289344,0,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,1056964608,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,2143289344,1056964608,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,1065353216,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,2143289344,1065353216,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,2143289344,0,false,[0, 0, 0, 1082130432, 0, 0, 0]),
            (0,2143289344,2143289344,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (0,2143289344,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,0,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1028443341,0,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,1056964608,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1028443341,1056964608,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,1065353216,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1028443341,1065353216,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,2143289344,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1028443341,2143289344,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1028443341,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,0,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1036831949,0,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,1056964608,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1036831949,1056964608,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,1065353216,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1036831949,1065353216,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,2143289344,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1036831949,2143289344,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831949,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,0,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1036831950,0,1065353216,false,[1, 1065353216, 0, 0, 0, 1083711225, 1080350405]),
            (1,1036831950,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,1056964608,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1036831950,1056964608,1065353216,false,[1, 1065353216, 0, 0, 0, 1077667264, 1074655968]),
            (1,1036831950,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,1065353216,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1036831950,1065353216,1065353216,false,[1, 1065353216, 0, 0, 0, 1074273516, 1070500971]),
            (1,1036831950,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,2143289344,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1036831950,2143289344,1065353216,true,[3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1036831950,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,0,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1058642330,0,1065353216,false,[1, 1065353216, 0, 0, 0, 1090213195, 1086096320]),
            (1,1058642330,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,1056964608,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1058642330,1056964608,1065353216,false,[1, 1065353216, 0, 0, 0, 1088268021, 1084643595]),
            (1,1058642330,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,1065353216,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1058642330,1065353216,1065353216,false,[1, 1065353216, 0, 0, 0, 1086140408, 1083054618]),
            (1,1058642330,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,2143289344,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,1058642330,2143289344,1065353216,true,[3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,1058642330,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,0,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,0,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,2143289344,0,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,0,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,0,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,1056964608,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,1056964608,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,2143289344,1056964608,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,1056964608,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,1056964608,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,1065353216,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,1065353216,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,2143289344,1065353216,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,1065353216,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,1065353216,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,2143289344,3212836864,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,2143289344,0,false,[1, 1065353216, 0, 0, 0, 1090519040, 1086324736]),
            (1,2143289344,2143289344,1065353216,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,2143289344,1092616192,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
            (1,2143289344,2143289344,2143289344,false,[2, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845, 3452816845]),
        ];
        for (i, &(shape, height, profile, y, panics, expected)) in cases.iter().enumerate() {
            let mut b = [0u8;88]; b[..8].copy_from_slice(&7u64.to_le_bytes());
            b[8..12].copy_from_slice(&u32::from(shape).to_le_bytes());
            let frame = if shape == 1 {[1.,0.,0.,0.,8.,6.]} else {[0.,0.,4.,0.,0.,0.]};
            for (j,v) in frame.into_iter().enumerate() { b[12+j*4..16+j*4].copy_from_slice(&f32::to_le_bytes(v)); }
            b[0x30..0x34].copy_from_slice(&height.to_le_bytes());
            b[0x2c..0x30].copy_from_slice(&profile.to_le_bytes());
            b[0x38..0x3c].copy_from_slice(&0.9f32.to_le_bytes());
            let actual = std::panic::catch_unwind(|| observed(Roof::from_observed(b).roof_shape_at_intersection_y(f32::from_bits(y))));
            if panics { assert!(actual.is_err(), "original panic case {i}"); }
            else { assert_eq!(actual.unwrap(), expected, "original case {i}"); }
        }
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod roof_selected_collider_tests {
    use super::*;
    use crate::systems::collision::world::{
        RaycastMeshData, rapier_hit_to_raycast_world_result_at_pose,
    };

    #[test]
    fn native_complete_hit_converter_matches_all_words() {
        type Row = (
            [u32; 6],
            [u32; 4],
            [u32; 3],
            [u32; 1],
            [u32; 9],
            [u32; 9],
            [u32; 6],
            [u32; 6],
            [u32; 2],
        );
        let rows: &[Row] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../reports/module-recovery-2026-10-23/m01-roof-uv-native-cases.rs"
        ));
        for (i, (ray, q, translation, toi, vertices, normals, uvs, expected, extra)) in
            rows.iter().enumerate()
        {
            let vector = |x: &[u32]| {
                glam::Vec3::new(
                    f32::from_bits(x[0]),
                    f32::from_bits(x[1]),
                    f32::from_bits(x[2]),
                )
            };
            let vertices: Vec<_> = vertices.chunks_exact(3).map(vector).collect();
            let normals: Vec<_> = normals.chunks_exact(3).map(vector).collect();
            let uvs: Vec<_> = uvs
                .chunks_exact(2)
                .map(|x| glam::Vec2::new(f32::from_bits(x[0]), f32::from_bits(x[1])))
                .collect();
            let mesh =
                RaycastMeshData::new_indexed(&[[0, 1, 2]], &vertices, Some(&normals), Some(&uvs));
            let result = rapier_hit_to_raycast_world_result_at_pose(
                &mesh,
                vector(&ray[..3]),
                vector(&ray[3..]),
                f32::from_bits(toi[0]),
                glam::Vec3::ZERO,
                Some((i % 2) as u32),
                [0; 3],
                vector(translation),
                glam::Quat::from_array(q.map(f32::from_bits)),
            );
            let actual = [
                result.position.x.to_bits(),
                result.position.y.to_bits(),
                result.position.z.to_bits(),
                result.normal.x.to_bits(),
                result.normal.y.to_bits(),
                result.normal.z.to_bits(),
            ];
            assert_eq!(actual, *expected, "native converter case {i}");
            assert_eq!(result.extra.map(f32::to_bits), *extra, "native UV case {i}");
        }
    }
    fn published_world() -> (bevy_ecs::world::World, bevy_ecs::schedule::Schedule) {
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::public_walls::{
            PublicWalls, WallPublicationDirty, WallSpatialHash,
        };
        use crate::resources::walls::publication_source::TerrainHeightGrid512;
        use crate::resources::walls::{InnerWalls, WallState};
        let mut inner = InnerWalls::default();
        for id in [WallId(11), WallId(12), WallId(13), WallId(14)] {
            inner.new_wall_with_id(
                id,
                WallState::new(
                    Curve2::try_new(vec![[10.0, -5.0], [14.0, -5.0]], false).unwrap(),
                    4.0,
                    0,
                    0,
                    false,
                ),
            );
        }
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(
            &mut inner,
            &terrain,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        for id in [WallId(11), WallId(12)] {
            assert!(walls.get(id).is_some());
        }
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(walls);
        world.insert_resource(crate::systems::collision::world::RaycastWorld::new());
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(crate::systems::collision::world::update_roof_rapier_colliders);
        (world, schedule)
    }

    fn spawn_roof(
        world: &mut bevy_ecs::world::World,
        id: WallId,
        y: f32,
        uv: bool,
    ) -> bevy_ecs::entity::Entity {
        use crate::resources::roofs::RoofTriggerVolume;
        let mut roof = [0; 88];
        roof[..8].copy_from_slice(&id.0.to_le_bytes());
        roof[0x4c..0x50].copy_from_slice(&y.to_le_bytes());
        let mut trigger = RoofTriggerVolume::default();
        trigger.set_triangles(vec![[
            Vec3::new(10.0, 0.0, -5.0),
            Vec3::new(14.0, 0.0, -5.0),
            Vec3::new(10.0, 0.0, -1.0),
        ]]);
        trigger.set_normals(vec![[Vec3::Y; 3]]);
        if uv {
            trigger.set_uvs(vec![[glam::Vec2::ZERO, glam::Vec2::X, glam::Vec2::Y]]);
        }
        let entity = world.spawn((Roof::from_observed(roof), trigger)).id();
        world
            .resource_mut::<crate::resources::walls::public_walls::PublicWalls>()
            .associate_roof(id, entity);
        entity
    }

    fn uv(
        world: &bevy_ecs::world::World,
        key: WallId,
        xz: glam::Vec2,
    ) -> Result<(glam::Vec2, Vec3), RoofUvFailure> {
        roof_uv_and_point_at_ws_pos(
            world.resource::<crate::resources::walls::public_walls::PublicWalls>(),
            world.resource::<crate::systems::collision::world::RaycastWorld>(),
            key,
            xz,
        )
    }

    #[test]
    fn real_roof_producer_selects_the_requested_roof_before_global_nearest() {
        let (mut world, mut schedule) = published_world();
        spawn_roof(&mut world, WallId(11), 2.0, true);
        spawn_roof(&mut world, WallId(12), 7.0, true);
        let p = glam::Vec2::new(11.0, -3.0);
        assert_eq!(uv(&world, WallId(11), p), Err(RoofUvFailure::MissingArena));
        schedule.run(&mut world);
        assert_eq!(
            uv(&world, WallId(11), p),
            Ok((glam::Vec2::new(0.25, 0.5), Vec3::new(11.0, 2.0, -3.0)))
        );
        assert_eq!(
            uv(&world, WallId(12), p),
            Ok((glam::Vec2::new(0.25, 0.5), Vec3::new(11.0, 7.0, -3.0)))
        );
        let mut raycast = world.resource_mut::<crate::systems::collision::world::RaycastWorld>();
        raycast.flush_colliders();
        let global = raycast.raycast(Vec3::new(11.0, 65536.0, -3.0), Vec3::NEG_Y, u32::MAX);
        assert_eq!(global.position.y.to_bits(), 7.0_f32.to_bits());
        assert_eq!(
            uv(&world, WallId(11), p).unwrap().1.y.to_bits(),
            2.0_f32.to_bits()
        );
        assert_eq!(
            uv(&world, WallId(99), p),
            Err(RoofUvFailure::MissingGlobalId)
        );
        assert_eq!(
            uv(&world, WallId(11), glam::Vec2::new(-100.0, -100.0)),
            Err(RoofUvFailure::RayMiss)
        );
    }

    #[test]
    fn real_roof_pose_receipt_normalization_and_original_failure_order() {
        use crate::systems::collision::world::RaycastWorld;
        use rapier3d::na::UnitQuaternion;
        use rapier3d::prelude::{Isometry, Translation};
        let (mut world, mut schedule) = published_world();
        let entity = spawn_roof(&mut world, WallId(11), 2.0, true);
        schedule.run(&mut world);
        let instance = *world
            .resource::<RaycastWorld>()
            .roof_collider_instance(entity)
            .unwrap();
        {
            let mut raycast = world.resource_mut::<RaycastWorld>();
            let collider = raycast
                .get_rapier_collider(instance.slot, instance.collider)
                .unwrap();
            collider.set_position(Isometry::from_parts(
                Translation::new(100.0, 3.0, -100.0),
                UnitQuaternion::new_unchecked(rapier3d::na::Quaternion::new(0.0, 0.0, 1.0, 0.0)),
            ));
            collider.user_data ^= 1_u128 << 32;
        }
        let p = glam::Vec2::new(89.0, -97.0);
        assert_eq!(
            uv(&world, WallId(11), p),
            Ok((glam::Vec2::new(0.25, 0.5), Vec3::new(89.0, 5.0, -97.0)))
        );
        {
            let mut raycast = world.resource_mut::<RaycastWorld>();
            let collider = raycast
                .get_rapier_collider(instance.slot, instance.collider)
                .unwrap();
            collider.user_data ^= 2_u128 << 32;
        }
        // 即使该点肯定不命中，原也先拒绝失效receipt，不能在cast miss之后才校验。
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| uv(
                &world,
                WallId(11),
                glam::Vec2::splat(-1000.0)
            )))
            .is_err()
        );
    }

    #[test]
    fn real_roof_unload_entity_reuse_empty_uv_panics_and_fixed_origin() {
        use crate::resources::roofs::RoofMarkedForDeletion;
        use crate::systems::collision::world::RaycastWorld;
        let (mut world, mut schedule) = published_world();
        let old = spawn_roof(&mut world, WallId(11), 2.0, true);
        spawn_roof(&mut world, WallId(13), 70000.0, true);
        spawn_roof(&mut world, WallId(14), 4.0, false);
        schedule.run(&mut world);
        let p = glam::Vec2::new(11.0, -3.0);
        assert_eq!(uv(&world, WallId(13), p), Err(RoofUvFailure::RayMiss));
        // 真Roof producer始终传Some(volume.uvs())；空Vec是Some(empty)，
        // 原转换器须bounds panic，不能把它当None伪通过NoUv码4。
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||uv(&world,WallId(14),p))).is_err());
        let (instance, mesh_owner) = {
            let raycast = world.resource::<RaycastWorld>();
            let instance = *raycast.roof_collider_instance(old).unwrap();
            let mesh = raycast.registered_mesh_for_collider(instance.slot, instance.collider);
            (instance, std::sync::Arc::downgrade(&mesh.data))
        };
        world.clear_trackers();
        assert!(world.despawn(old));
        schedule.run(&mut world);
        assert_eq!(uv(&world, WallId(11), p), Err(RoofUvFailure::MissingArena));
        assert!(world.resource::<RaycastWorld>()
            .get_rapier_collider_ref(instance.slot, instance.collider).is_none());
        assert!(mesh_owner.upgrade().is_none(), "删除同时释放真实mesh receipt的数据所有权");
        world.clear_trackers();
        let new = spawn_roof(&mut world, WallId(12), 9.0, true);
        assert_eq!(new.index(), old.index());
        assert_ne!(new.to_bits(), old.to_bits());
        schedule.run(&mut world);
        assert_eq!(uv(&world, WallId(11), p), Err(RoofUvFailure::MissingArena));
        assert_eq!(
            uv(&world, WallId(12), p).unwrap().1.y.to_bits(),
            9.0_f32.to_bits()
        );
        world.clear_trackers();
        world.entity_mut(new).insert(RoofMarkedForDeletion);
        schedule.run(&mut world);
        assert_eq!(uv(&world, WallId(12), p), Err(RoofUvFailure::MissingArena));
        world
            .resource_mut::<crate::resources::walls::public_walls::PublicWalls>()
            .remove_roof(WallId(12));
        assert_eq!(
            uv(&world, WallId(12), p),
            Err(RoofUvFailure::MissingGlobalId)
        );
    }
}
