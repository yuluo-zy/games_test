//! B `systems/wall/valid_enclosure/mod.rs`: premade-shape enclosure parameters.
//!
//! Evidence: `chapters/systems/wall/valid_enclosure/mod.rs.md` (O, VAs below)
//! and `wall/DIGEST.md` §2.6. `ShapeParameters` is the 28 B record
//! `{tag u32@+0, payload 24B@+4}`; the discriminant is tag bit 0
//! (0 = Circle, 1 = Rectangle); tags >= 2 are freehand and not this record.
//! Serde derives the observed enum name `IsEnclosedNoYesPremadeShapeIsEnclosedV2`.

use glam::Vec2;
use serde::{Deserialize, Serialize};
use utils::curve::{Curve2, Curve3XZ};
use utils::geometry::{Circle2d, Rectangle2d};

use crate::resources::walls::publication_fields::WallShapeSnapshot;

/// Typed premade-shape view over the payload (B tag bit 0).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Circle(Circle2d),
    Rectangle(Rectangle2d),
}

/// Circle payload is 16 B {cx@+4, cy@+8, r@+0xc, spare@+0x10}; rectangle
/// payload is 24 B {cos@+4, sin@+8, cx@+0xc, cy@+0x10, hw@+0x14, hh@+0x18}.
/// These offsets are relative to the 28 B record (public_wall_state reads the
/// same bytes via `shape_float(4 + index*4)`).
pub const CIRCLE_PAYLOAD_FLOATS: usize = 4;
pub const RECTANGLE_PAYLOAD_FLOATS: usize = 6;

/// B contains_shape 0x1411ff330 circle boundary samples（fresh 机器码：`XORPS`=0、
/// 0x142c197c8=1/6、0x142a86330=1/3、0x142925870=1/2、0x142b1e8b8=2/3、
/// 0x142c197cc=5/6——**含 1/2，非 1.0**）。自=圆/他=圆与自=矩形/他=圆共用这 6 点。
const CIRCLE_CONTAINS_SAMPLES: [f32; 6] = [
    0.0,
    1.0 / 6.0,
    1.0 / 3.0,
    1.0 / 2.0,
    2.0 / 3.0,
    5.0 / 6.0,
];

impl Shape {
    pub fn tag(self) -> u32 {
        match self {
            Self::Circle(_) => 0,
            Self::Rectangle(_) => 1,
        }
    }

    pub fn center(self) -> Vec2 {
        match self {
            Self::Circle(c) => c.center,
            Self::Rectangle(r) => r.center,
        }
    }

    pub fn perimeter(self) -> f32 {
        match self {
            Self::Circle(c) => c.perimeter(),
            Self::Rectangle(r) => r.perimeter(),
        }
    }

    /// B dispatch: circle 0x140c975a0, rectangle 0x140c99800.
    pub fn get_pos_at_u(self, u: f32) -> Vec2 {
        match self {
            Self::Circle(c) => c.get_pos_at_u(u),
            Self::Rectangle(r) => r.get_pos_at_u(u),
        }
    }

    /// B dispatch: circle 0x140c978e0, rectangle 0x140c99e10.
    pub fn get_normal_at_u(self, u: f32) -> Vec2 {
        match self {
            Self::Circle(c) => c.get_normal_at_u(u),
            Self::Rectangle(r) => r.get_normal_at_u(u),
        }
    }

    /// B dispatch: circle 0x140c97970, rectangle 0x140c99530.
    pub fn get_closest_u_from_pos(self, p: Vec2) -> f32 {
        match self {
            Self::Circle(c) => c.get_closest_u_from_pos(p),
            Self::Rectangle(r) => rectangle_closest_u_from_pos(r, p),
        }
    }

    /// B dispatch: circle 0x140c980ee (|P-C|² < r²), rectangle 0x140c9a62e.
    pub fn contains_point(self, p: Vec2) -> bool {
        match self {
            Self::Circle(c) => c.contains_point(p),
            Self::Rectangle(r) => r.contains_point(p),
        }
    }

    /// B 0x1411ff330. fresh 反编译四路：
    /// - 自=圆 / 他=圆：**6 次** `Circle2d::get_pos_at_u + contains_point`
    ///   （[`CIRCLE_CONTAINS_SAMPLES`]，含 1/2 非 1.0）。
    /// - 自=圆 / 他=矩形：**4 次** `Rectangle2d::get_point + Circle2d::contains_point`（4 角）。
    /// - 自=矩形 / 他=圆：6 次 `get_pos_at_u + Rectangle2d::contains_point`（同采样集）。
    /// - 自=矩形 / 他=矩形：4 次 `get_point`。
    /// restorer §11.3 的"u=0 边界点纠偏"（单点）与机器码不符（原非单点）。
    pub fn contains_shape(self, other: Shape) -> bool {
        match self {
            Self::Circle(c) => match other {
                Self::Circle(oc) => CIRCLE_CONTAINS_SAMPLES
                    .iter()
                    .all(|&u| c.contains_point(oc.get_pos_at_u(u))),
                Self::Rectangle(or) => (0..4).all(|i| c.contains_point(or.get_point(i))),
            },
            Self::Rectangle(r) => match other {
                Self::Circle(oc) => CIRCLE_CONTAINS_SAMPLES
                    .iter()
                    .all(|&u| r.contains_point(oc.get_pos_at_u(u))),
                Self::Rectangle(or) => (0..4).all(|i| r.contains_point(or.get_point(i))),
            },
        }
    }

    /// Circle2d::as_curve2(0.2) / Rectangle2d::as_curve2 (utils Curve2).
    pub fn as_curve2(self) -> Curve2 {
        match self {
            Self::Circle(c) => c.as_curve2(0.2),
            Self::Rectangle(r) => r.as_curve2(),
        }
    }

    /// Circle2d::as_curve3(0.2, z) / Rectangle2d::as_curve3(z).
    pub fn as_curve3(self, height: f32) -> Curve3XZ {
        match self {
            Self::Circle(c) => c.as_curve3(0.2, height),
            Self::Rectangle(r) => r.as_curve3(height),
        }
    }

    /// Build the 28 B record for a premade shape (tag + payload).
    pub fn to_record(self) -> [u8; 28] {
        let mut record = [0; 28];
        record[..4].copy_from_slice(&self.tag().to_le_bytes());
        match self {
            Self::Circle(c) => {
                let floats = [c.center.x, c.center.y, c.radius, c.rotation];
                for (i, value) in floats.into_iter().enumerate() {
                    record[4 + i * 4..8 + i * 4].copy_from_slice(&value.to_le_bytes());
                }
            }
            Self::Rectangle(r) => {
                let floats = [
                    r.axis.x,
                    r.axis.y,
                    r.center.x,
                    r.center.y,
                    r.size.x,
                    r.size.y,
                ];
                for (i, value) in floats.into_iter().enumerate() {
                    record[4 + i * 4..8 + i * 4].copy_from_slice(&value.to_le_bytes());
                }
            }
        }
        record
    }
}

/// B 0x140c99530: closest perimeter parameter. `u` is arc-length normalized
/// along the four edges (Rectangle2d::corner_us).
fn rectangle_closest_u_from_pos(rect: Rectangle2d, p: Vec2) -> f32 {
    let corners = rect.as_points2();
    let us = rect.corner_us();
    let mut best_u = 0.0;
    let mut best_d = f32::MAX;
    for edge in 0..4 {
        let e1 = corners[edge];
        let edge_vec = corners[(edge + 1) % 4] - e1;
        let len2 = edge_vec.length_squared().max(1e-20);
        let t = ((p - e1).dot(edge_vec) / len2).clamp(0.0, 1.0);
        let closest = e1 + edge_vec * t;
        let d = (p - closest).length_squared();
        if d < best_d {
            best_d = d;
            best_u = us[edge] + (us[edge + 1] - us[edge]) * t;
        }
    }
    best_u
}

/// B `ShapeParameters` (0x1411ff2a0..0x1411ff740): 28 B record.
///
/// PDB reports an enum type; the reconstruction already carries the identical
/// bytes as [`WallShapeSnapshot`], so this newtype keeps byte-for-byte parity
/// with the authority record while offering the typed [`Shape`] dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShapeParameters([u8; 28]);

impl ShapeParameters {
    pub fn from_observed(record: [u8; 28]) -> Self {
        Self(record)
    }

    pub fn bytes(&self) -> &[u8; 28] {
        &self.0
    }

    pub fn tag(&self) -> u32 {
        u32::from_le_bytes(self.0[..4].try_into().unwrap())
    }

    pub fn is_premade(&self) -> bool {
        self.tag() < 2
    }

    fn float_at(&self, offset: usize) -> f32 {
        f32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }

    /// Typed view; None for freehand (tag >= 2).
    pub fn get_shape(&self) -> Option<Shape> {
        match self.tag() {
            0 => Some(Shape::Circle(Circle2d {
                center: Vec2::new(self.float_at(4), self.float_at(8)),
                radius: self.float_at(0xc),
                rotation: self.float_at(0x10),
            })),
            1 => Some(Shape::Rectangle(Rectangle2d {
                axis: Vec2::new(self.float_at(4), self.float_at(8)),
                center: Vec2::new(self.float_at(0xc), self.float_at(0x10)),
                size: Vec2::new(self.float_at(0x14), self.float_at(0x18)),
            })),
            _ => None,
        }
    }

    pub fn from_shape(shape: Shape) -> Self {
        Self(shape.to_record())
    }

    pub fn to_snapshot(&self) -> WallShapeSnapshot {
        WallShapeSnapshot::from_observed(self.0)
    }

    /// The raw 28 B record (tag + payload).
    pub fn to_record(&self) -> [u8; 28] {
        self.0
    }

    pub fn from_snapshot(snapshot: &WallShapeSnapshot) -> Self {
        Self(*snapshot.bytes())
    }

    /// B 0x1411ff2a0: `payload[tag*8]` two floats.
    pub fn center(&self) -> Vec2 {
        let tag = self.tag() as usize;
        Vec2::new(
            self.float_at(4 + tag * 8),
            self.float_at(8 + tag * 8),
        )
    }

    pub fn perimeter(&self) -> f32 {
        self.get_shape().expect("premade shape").perimeter()
    }

    pub fn get_pos_at_u(&self, u: f32) -> Vec2 {
        self.get_shape().expect("premade shape").get_pos_at_u(u)
    }

    pub fn get_normal_at_u(&self, u: f32) -> Vec2 {
        self.get_shape()
            .expect("premade shape")
            .get_normal_at_u(u)
    }

    pub fn get_closest_u_from_pos(&self, p: Vec2) -> f32 {
        self.get_shape()
            .expect("premade shape")
            .get_closest_u_from_pos(p)
    }

    pub fn contains_point(&self, p: Vec2) -> bool {
        self.get_shape().expect("premade shape").contains_point(p)
    }

    pub fn contains_shape(&self, other: &Self) -> bool {
        match (self.get_shape(), other.get_shape()) {
            (Some(a), Some(b)) => a.contains_shape(b),
            _ => false,
        }
    }

    pub fn as_curve2(&self) -> Curve2 {
        self.get_shape().expect("premade shape").as_curve2()
    }

    pub fn as_curve3(&self, height: f32) -> Curve3XZ {
        self.get_shape().expect("premade shape").as_curve3(height)
    }

    /// B 0x1411ff6f0: circle payload or fmt panic.
    pub fn as_circle(&self) -> Circle2d {
        match self.get_shape() {
            Some(Shape::Circle(c)) => c,
            _ => panic!("as_circle called on a non-circle ShapeParameters"),
        }
    }

    /// B 0x1411ff740: rectangle payload or fmt panic.
    pub fn as_rectangle(&self) -> Rectangle2d {
        match self.get_shape() {
            Some(Shape::Rectangle(r)) => r,
            _ => panic!("as_rectangle called on a non-rectangle ShapeParameters"),
        }
    }

    /// B 0x1411ff680. fresh 反编译：tag==0（圆）→ **仅 r(radius) += amount**，
    /// +0x14/+0x18 **保持原值不变**（机器码 `~uVar7 & (2a+fVar5) | fVar5 & uVar7`，
    /// uVar7 圆时为全 1 → 保留原 fVar4/fVar5）。tag≠0（矩形）→ +0x14/+0x18
    /// **在原值上加 2*amount**（`2a + fVar4/fVar5`，非覆写）。
    /// restorer 对圆车道"+0x14/+0x18 +=2a"与矩形"覆写 2a"的描述与机器码相反。
    pub fn expand(&self, amount: f32) -> Self {
        let mut out = *self;
        let twice = amount + amount;
        match self.tag() {
            0 => {
                let radius = self.float_at(0xc) + amount;
                out.0[0xc..0x10].copy_from_slice(&radius.to_le_bytes());
                // +0x14/+0x18 spare lanes preserved unchanged.
            }
            _ => {
                let hw = self.float_at(0x14) + twice;
                let hh = self.float_at(0x18) + twice;
                out.0[0x14..0x18].copy_from_slice(&hw.to_le_bytes());
                out.0[0x18..0x1c].copy_from_slice(&hh.to_le_bytes());
            }
        }
        out
    }
}

impl Serialize for ShapeParameters {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let payload: [f32; 6] = [
            self.float_at(4),
            self.float_at(8),
            self.float_at(0xc),
            self.float_at(0x10),
            self.float_at(0x14),
            self.float_at(0x18),
        ];
        let mut state = serializer.serialize_struct("ShapeParameters", 2)?;
        state.serialize_field("tag", &self.tag())?;
        state.serialize_field("payload", &payload)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ShapeParameters {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RecordVisitor;
        impl<'de> serde::de::Visitor<'de> for RecordVisitor {
            type Value = ShapeParameters;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShapeParameters record")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut tag: Option<u32> = None;
                let mut payload: Option<[f32; 6]> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "tag" => tag = Some(map.next_value()?),
                        "payload" => payload = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let tag = tag.ok_or_else(|| serde::de::Error::missing_field("tag"))?;
                let payload =
                    payload.ok_or_else(|| serde::de::Error::missing_field("payload"))?;
                let mut record = [0u8; 28];
                record[..4].copy_from_slice(&tag.to_le_bytes());
                for (offset, value) in [4usize, 8, 0xc, 0x10, 0x14, 0x18]
                    .into_iter()
                    .zip(payload)
                {
                    record[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
                }
                Ok(ShapeParameters(record))
            }
        }
        deserializer.deserialize_struct("ShapeParameters", &["tag", "payload"], RecordVisitor)
    }
}

/// B serde name `IsEnclosedNoYesPremadeShapeIsEnclosedV2` (serialize
/// 0x1416eb640: variant 0 → `"No"`, 1 → `"Yes"`, 2 → `"PremadeShape"`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IsEnclosedNoYesPremadeShapeIsEnclosedV2")]
pub enum IsEnclosed {
    No,
    Yes,
    PremadeShape(ShapeParameters),
}

/// Build a premade ShapeParameters from observed wall-authority bytes.
pub fn shape_parameters_from_snapshot(snapshot: &WallShapeSnapshot) -> Option<ShapeParameters> {
    let params = ShapeParameters::from_snapshot(snapshot);
    params.is_premade().then_some(params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    #[test]
    fn circle_record_round_trips_through_the_typed_view() {
        let circle = Circle2d {
            center: Vec2::new(1.5, -2.0),
            radius: 4.0,
            rotation: 0.0,
        };
        let params = ShapeParameters::from_shape(Shape::Circle(circle));
        assert_eq!(params.tag(), 0);
        assert!(params.is_premade());
        assert_eq!(params.center(), Vec2::new(1.5, -2.0));
        let shape = params.get_shape().unwrap();
        assert_eq!(shape, Shape::Circle(circle));
        assert_eq!(shape.perimeter(), 8.0 * std::f32::consts::PI);
        let start = shape.get_pos_at_u(0.0);
        assert!((start.x - 5.5).abs() < 1e-4, "{start:?}");
        assert!((start.y + 2.0).abs() < 1e-4, "{start:?}");
        // |P-C|² < r².
        assert!(shape.contains_point(Vec2::new(2.0, -2.0)));
        assert!(!shape.contains_point(Vec2::new(6.0, -2.0)));
    }

    #[test]
    fn rectangle_record_layout_matches_the_28b_record() {
        let rect = Rectangle2d::new_axis_aligned(Vec2::new(3.0, 4.0), Vec2::new(2.0, 6.0));
        let params = ShapeParameters::from_shape(Shape::Rectangle(rect));
        assert_eq!(params.tag(), 1);
        assert_eq!(params.center(), Vec2::new(3.0, 4.0));
        let bytes = params.bytes();
        // cos=1, sin=0, cx=3, cy=4, size=(2,6) full dimensions.
        assert_eq!(f32::from_le_bytes(bytes[4..8].try_into().unwrap()), 1.0);
        assert_eq!(f32::from_le_bytes(bytes[8..12].try_into().unwrap()), 0.0);
        assert_eq!(f32::from_le_bytes(bytes[12..16].try_into().unwrap()), 3.0);
        assert_eq!(f32::from_le_bytes(bytes[16..20].try_into().unwrap()), 4.0);
        assert_eq!(f32::from_le_bytes(bytes[20..24].try_into().unwrap()), 2.0);
        assert_eq!(f32::from_le_bytes(bytes[24..28].try_into().unwrap()), 6.0);
    }

    #[test]
    fn expand_grows_circle_radius_and_rectangle_grows_dimensions() {
        let circle = ShapeParameters::from_shape(Shape::Circle(Circle2d {
            center: Vec2::ZERO,
            radius: 2.0,
            rotation: 0.0,
        }));
        let grown = circle.expand(0.5);
        assert_eq!(grown.as_circle().radius, 2.5);
        // +0x14/+0x18 spare lanes preserved unchanged（fresh 0x1411ff680）。
        assert_eq!(grown.float_at(0x14), 0.0);
        assert_eq!(grown.float_at(0x18), 0.0);

        let rect = ShapeParameters::from_shape(Shape::Rectangle(Rectangle2d::new_axis_aligned(
            Vec2::ZERO,
            Vec2::new(4.0, 8.0),
        )));
        let grown = rect.expand(0.5);
        // 矩形在原 hw/hh 上加 2a（非覆写）：4+1 / 8+1。
        assert_eq!(grown.as_rectangle().size, Vec2::new(5.0, 9.0));
    }

    #[test]
    fn rectangle_contains_circle_samples_every_point() {
        let big = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(10.0, 10.0));
        let circle = Circle2d::new(Vec2::new(1.0, 0.0), 1.0);
        assert!(Shape::Rectangle(big).contains_shape(Shape::Circle(circle)));
        let outside = Circle2d::new(Vec2::new(6.0, 0.0), 1.0);
        assert!(!Shape::Rectangle(big).contains_shape(Shape::Circle(outside)));
    }

    #[test]
    fn circle_contains_shape_samples_other_circle_boundary() {
        // self=circle/other=circle：6 边界采样点 {0,1/6,1/3,1/2,2/3,5/6}（fresh）。
        let big = Circle2d::new(Vec2::ZERO, 5.0);
        // other 圆心 (2,0) r=1 → 全部采样点距原点 ≤3 <5 → 包含。
        let inner = Circle2d::new(Vec2::new(2.0, 0.0), 1.0);
        assert!(Shape::Circle(big).contains_shape(Shape::Circle(inner)));
        // other 圆心 (4,0) r=1 → u=0 采样点 (5,0) 恰在半径上（严格 < → 不包含）。
        let touching = Circle2d::new(Vec2::new(4.0, 0.0), 1.0);
        assert!(!Shape::Circle(big).contains_shape(Shape::Circle(touching)));
        let outside = Circle2d::new(Vec2::new(8.0, 0.0), 1.0);
        assert!(!Shape::Circle(big).contains_shape(Shape::Circle(outside)));

        // self=circle/other=rectangle：4 角采样（fresh 0x1411ff330）。
        let rect = Rectangle2d::new_axis_aligned(Vec2::new(1.0, 0.0), Vec2::new(2.0, 2.0));
        assert!(Shape::Circle(big).contains_shape(Shape::Rectangle(rect)));
        let far_rect = Rectangle2d::new_axis_aligned(Vec2::new(6.0, 0.0), Vec2::new(2.0, 2.0));
        assert!(!Shape::Circle(big).contains_shape(Shape::Rectangle(far_rect)));
    }

    #[test]
    fn is_enclosed_serde_uses_the_observed_variant_names() {
        let yes = serde_json::to_string(&IsEnclosed::Yes).unwrap();
        assert_eq!(yes, "\"Yes\"");
        let no = serde_json::to_string(&IsEnclosed::No).unwrap();
        assert_eq!(no, "\"No\"");
        assert_eq!(
            serde_json::from_str::<IsEnclosed>("\"No\"").unwrap(),
            IsEnclosed::No
        );
    }
}
