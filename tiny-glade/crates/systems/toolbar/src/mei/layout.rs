//! Original B MEI box layout. Original RNE 140fe0e10 / 140fe12a0 / 140fe19a0.
//! The source-level structures preserve semantics; Vec and trait object ABI are
//! deliberately supplied by Rust. See LAYOUT_CONTRACT.md in task TG-19924eac9946ff87be.
use glam::Vec2;
use serde::Deserialize;

/// B stores one discriminant and one f32; direct numbers mean absolute lengths.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct LengthPercentage {
    pub is_percent: bool,
    pub value: f32,
}
impl LengthPercentage {
    pub const fn absolute(value: f32) -> Self {
        Self {
            is_percent: false,
            value,
        }
    }
    pub fn resolve(&self, base: f32) -> f32 {
        if self.is_percent {
            self.value * base
        } else {
            self.value
        }
    }
    fn minimum(self) -> f32 {
        if self.is_percent { 0.0 } else { self.value }
    }
    /// Original points getter (inline, 24 instances): the raw f32 of an
    /// absolute length; percentages carry their fraction unchanged.
    pub fn points(self) -> f32 {
        self.value
    }
    /// Original points_or_zero (inline, 11 instances): percentages contribute
    /// zero, absolute lengths contribute their value.
    pub fn points_or_zero(self) -> f32 {
        self.minimum()
    }
}

impl std::ops::Add for LengthPercentage {
    type Output = LengthPercentage;
    /// Native Rect::width/height use two generic adds; absolute lengths add.
    /// U: the mixed percent/absolute form has no recovered consumer.
    fn add(self, rhs: LengthPercentage) -> LengthPercentage {
        LengthPercentage {
            is_percent: self.is_percent || rhs.is_percent,
            value: self.value + rhs.value,
        }
    }
}

/// Native 16-byte Size{width@0, height@8}; each component is a
/// LengthPercentage. LayoutStyle keeps width at +0x28 and height at +0x30
/// (historical Rust field names `size`/`min` in this crate).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Size {
    pub width: LengthPercentage,
    pub height: LengthPercentage,
}
impl Size {
    pub fn points(self) -> Vec2 {
        Vec2::new(self.width.points(), self.height.points())
    }
    pub fn points_or_zero(self) -> Vec2 {
        Vec2::new(self.width.points_or_zero(), self.height.points_or_zero())
    }
}
impl From<Vec2> for Size {
    fn from(value: Vec2) -> Self {
        Size {
            width: LengthPercentage::absolute(value.x),
            height: LengthPercentage::absolute(value.y),
        }
    }
}

/// Native left/top/right/bottom rect used by the untagged padding
/// deserializer140fe2560 and Rect::size_vec2. `width`/`height` are the two
/// absolute edge sums (O: two `core::ops::arith::add` calls).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rect {
    pub left: LengthPercentage,
    pub top: LengthPercentage,
    pub right: LengthPercentage,
    pub bottom: LengthPercentage,
}
impl Rect {
    pub fn width(self) -> f32 {
        (self.left + self.right).points_or_zero()
    }
    pub fn height(self) -> f32 {
        (self.top + self.bottom).points_or_zero()
    }
    /// Original size_vec2 (inline, 4 instances): (width, height).
    pub fn size_vec2(self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }
    pub fn points(self) -> [f32; 4] {
        [
            self.left.points(),
            self.top.points(),
            self.right.points(),
            self.bottom.points(),
        ]
    }
    pub fn points_or_zero(self) -> [f32; 4] {
        [
            self.left.points_or_zero(),
            self.top.points_or_zero(),
            self.right.points_or_zero(),
            self.bottom.points_or_zero(),
        ]
    }
}
impl From<(LengthPercentage, LengthPercentage, LengthPercentage, LengthPercentage)> for Rect {
    fn from(
        value: (LengthPercentage, LengthPercentage, LengthPercentage, LengthPercentage),
    ) -> Self {
        Rect {
            left: value.0,
            top: value.1,
            right: value.2,
            bottom: value.3,
        }
    }
}
impl From<Size> for Rect {
    fn from(value: Size) -> Self {
        Rect {
            left: value.width,
            top: value.height,
            right: LengthPercentage::default(),
            bottom: LengthPercentage::default(),
        }
    }
}
impl<'de> Deserialize<'de> for LengthPercentage {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        enum Explicit {
            Points(f32),
            Frac(f32),
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Value {
            Number(f32),
            Explicit(Explicit),
        }
        Ok(match Value::deserialize(de)? {
            Value::Number(value) | Value::Explicit(Explicit::Points(value)) => {
                Self::absolute(value)
            }
            Value::Explicit(Explicit::Frac(value)) => Self {
                is_percent: true,
                value,
            },
        })
    }
}

/// B 140fe28e0: the five variants are the entire dispatch table.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
#[repr(u8)]
pub enum Align {
    Start = 0,
    Center = 1,
    End = 2,
    SpaceBetween = 3,
    SpaceEvenly = 4,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
#[repr(u8)]
pub enum XAlign {
    Stretch = 0,
    Start = 1,
    Center = 2,
    End = 3,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Justify(pub u8);
impl Justify {
    pub const DEFAULT: Self = Self(4);
}

/// B72-byte style. Historical Rust field names `size`/`min` are preserved for
/// callers; native offsets28/30 are respectively width and height dimensions.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct LayoutStyle {
    pub padding: [LengthPercentage; 4],
    pub gap: LengthPercentage,
    pub size: LengthPercentage,
    pub min: LengthPercentage,
    pub expand: f32,
    pub offset: Vec2,
    pub swizzle: u8,
    pub align: u8,
    pub justify: Justify,
    pub _pad: [u8; 1],
}
const _: () = assert!(std::mem::size_of::<LayoutStyle>() == 72);
impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            padding: [LengthPercentage::default(); 4],
            gap: LengthPercentage::default(),
            size: LengthPercentage::default(),
            min: LengthPercentage::default(),
            expand: 0.,
            offset: Vec2::ZERO,
            swizzle: 0,
            align: 0,
            justify: Justify::DEFAULT,
            _pad: [0],
        }
    }
}
pub fn horizontal() -> LayoutStyle {
    LayoutStyle::default()
}

/// Native 16-byte result stores size first and origin second.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ComputedLayout {
    pub size: Vec2,
    pub pos: Vec2,
}
impl ComputedLayout {
    pub fn center(&self) -> Vec2 {
        self.pos + self.size * 0.5
    }
}
/// Actual x/y size and absolute origin, independent of the selected main axis.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ParentLayout {
    pub main: f32,
    pub cross: f32,
    pub offset: Vec2,
}
/// The historical `padding` name refers to content intrinsic dimensions at+8;
/// prev_x/prev_y carry the just-computed child minimum size.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ChildLayoutInput<'a> {
    pub style: &'a LayoutStyle,
    pub padding: &'a [LengthPercentage; 2],
    pub prev_x: f32,
    pub prev_y: f32,
}
const _: () = assert!(std::mem::size_of::<ChildLayoutInput>() == 24);
pub trait Swizzle {
    const VERTICAL: bool;
}
pub struct SwizzleXY;
pub struct SwizzleYX;
impl Swizzle for SwizzleXY {
    const VERTICAL: bool = false;
}
impl Swizzle for SwizzleYX {
    const VERTICAL: bool = true;
}

impl LayoutStyle {
    /// Original LayoutStyle::horizontal (inline, 2 instances): swizzle 0.
    pub fn horizontal(mut self) -> Self {
        self.swizzle = 0;
        self
    }
    /// Original LayoutStyle::vertical (inline, 1 instance): swizzle 1.
    pub fn vertical(mut self) -> Self {
        self.swizzle = 1;
        self
    }
    /// Original 0x14068e8b0: gap LengthPercentage at +0x20/+0x24.
    pub fn gap(mut self, gap: LengthPercentage) -> Self {
        self.gap = gap;
        self
    }
    /// Original LayoutStyle::padding (inline, 2 instances): LTRB rect into the
    /// four padding slots.
    pub fn padding(mut self, padding: Rect) -> Self {
        self.padding = [padding.left, padding.top, padding.right, padding.bottom];
        self
    }
    /// Original 0x14068e8f0: expand flag writes 1.0 at +0x38.
    pub fn expand(mut self) -> Self {
        self.expand = 1.;
        self
    }
    /// Original 0x14068e930: justify tag 3 at +0x46.
    pub fn space_between(mut self) -> Self {
        self.justify = Justify(3);
        self
    }
    /// Original 0x14068e960: justify tag 1 at +0x46.
    pub fn align_center(mut self) -> Self {
        self.justify = Justify(1);
        self
    }
    /// Original 0x14068e990: cross align tag 2 at +0x45.
    pub fn xalign_center(mut self) -> Self {
        self.align = 2;
        self
    }
    /// Original 0x14068e9c0: write Size{width@+0x28, height@+0x30}.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size.width;
        self.min = size.height;
        self
    }
    /// Original helper140fe0e10, called on the reverse topological order.
    pub fn compute_min_size_with_children(
        &self,
        intrinsic: [LengthPercentage; 2],
        children: impl IntoIterator<Item = Vec2>,
    ) -> Vec2 {
        let children: Vec<_> = children.into_iter().collect();
        let mut result = Vec2::ZERO;
        for child in &children {
            if self.swizzle == 0 {
                result.x += child.x;
                result.y = result.y.max(child.y);
            } else {
                result.x = result.x.max(child.x);
                result.y += child.y;
            }
        }
        let gaps = self.gap.minimum() * (children.len().saturating_sub(1) as f32);
        if self.swizzle == 0 {
            result.x += gaps;
        } else {
            result.y += gaps;
        }
        result += Vec2::new(
            self.padding[0].minimum() + self.padding[2].minimum(),
            self.padding[1].minimum() + self.padding[3].minimum(),
        );
        result.x = result
            .x
            .max(self.size.minimum())
            .max(intrinsic[0].minimum());
        result.y = result.y.max(self.min.minimum()).max(intrinsic[1].minimum());
        result
    }
}
pub fn place_childen(
    style: &LayoutStyle,
    parent: &ParentLayout,
    children: &[ChildLayoutInput<'_>],
) -> Vec<ComputedLayout> {
    if style.swizzle == 0 {
        place_childen_impl::<SwizzleXY>(style, parent, children)
    } else {
        place_childen_impl::<SwizzleYX>(style, parent, children)
    }
}
/// Original140fe12a0/140fe19a0. Main and cross axes transpose together,
/// including style dimensions, intrinsic sizes, padding and final output.
pub fn place_childen_impl<S: Swizzle>(
    style: &LayoutStyle,
    parent: &ParentLayout,
    children: &[ChildLayoutInput<'_>],
) -> Vec<ComputedLayout> {
    let (main, cross, ms, me, cs, ce) = if S::VERTICAL {
        (parent.cross, parent.main, 1, 3, 0, 2)
    } else {
        (parent.main, parent.cross, 0, 2, 1, 3)
    };
    let start_pad = style.padding[ms].resolve(main);
    let end_pad = style.padding[me].resolve(main);
    let cross_start = style.padding[cs].resolve(cross);
    let cross_end = style.padding[ce].resolve(cross);
    let without_pads = main - start_pad - end_pad;
    let mut gap = style.gap.resolve(without_pads);
    let gap_count = children.len().saturating_sub(1) as f32;
    let available = without_pads - gap_count * gap;
    let cross_available = cross - cross_start - cross_end;
    let base_main = |child: &ChildLayoutInput<'_>| {
        let (dimension, intrinsic, minimum) = if S::VERTICAL {
            (child.style.min, child.padding[1], child.prev_y)
        } else {
            (child.style.size, child.padding[0], child.prev_x)
        };
        minimum.max(
            intrinsic
                .resolve(available)
                .max(dimension.resolve(available)),
        )
    };
    let sum: f32 = children.iter().map(base_main).sum();
    let sum_expand: f32 = children.iter().map(|c| c.style.expand).sum();
    let remaining = main - start_pad - end_pad - gap_count * gap - sum;
    let alignment_free = (1. - sum_expand).clamp(0., 1.) * remaining;
    let mut cursor = start_pad;
    match style.justify.0 {
        0 => {}
        1 => cursor += alignment_free * 0.5,
        2 => cursor += alignment_free,
        3 if children.len() > 1 => gap += alignment_free / gap_count,
        3 => cursor += alignment_free * 0.5,
        4 => {
            let each = alignment_free / (children.len() as f32 + 1.);
            gap += each;
            cursor += each;
        }
        _ => unreachable!("invalid native MEI Align tag"),
    }
    let divisor = sum_expand.max(f32::from_bits(0x1e3ce508));
    children
        .iter()
        .map(|child| {
            let child_main = base_main(child) + (child.style.expand / divisor) * remaining.max(0.);
            let (dimension, intrinsic, minimum) = if S::VERTICAL {
                (child.style.size, child.padding[0], child.prev_x)
            } else {
                (child.style.min, child.padding[1], child.prev_y)
            };
            let mut child_cross = minimum.max(
                intrinsic
                    .resolve(cross_available)
                    .max(dimension.resolve(cross_available)),
            );
            let cross_offset = match style.align {
                0 => {
                    child_cross = cross_available;
                    0.
                }
                1 => 0.,
                2 => (cross_available - child_cross) * 0.5,
                3 => cross_available - child_cross,
                _ => unreachable!("invalid native MEI XAlign tag"),
            };
            let (size, pos) = if S::VERTICAL {
                (
                    Vec2::new(child_cross, child_main),
                    Vec2::new(cross_start + cross_offset, cursor),
                )
            } else {
                (
                    Vec2::new(child_main, child_cross),
                    Vec2::new(cursor, cross_start + cross_offset),
                )
            };
            cursor += child_main + gap;
            ComputedLayout {
                size,
                pos: pos + parent.offset + style.offset,
            }
        })
        .collect()
}

#[cfg(test)]
mod native_layout_tests {
    use super::*;
    #[test]
    fn expand_uses_remaining_space_and_axes_transpose() {
        let mut parent = LayoutStyle {
            gap: LengthPercentage::absolute(10.),
            justify: Justify(0),
            align: 2,
            ..Default::default()
        };
        let child_style = LayoutStyle {
            expand: 1.,
            ..Default::default()
        };
        let intrinsic = [LengthPercentage::default(); 2];
        let children = [
            ChildLayoutInput {
                style: &child_style,
                padding: &intrinsic,
                prev_x: 20.,
                prev_y: 10.,
            },
            ChildLayoutInput {
                style: &child_style,
                padding: &intrinsic,
                prev_x: 20.,
                prev_y: 10.,
            },
        ];
        let layout = place_childen(
            &parent,
            &ParentLayout {
                main: 100.,
                cross: 40.,
                offset: Vec2::new(5., 7.),
            },
            &children,
        );
        assert_eq!(
            layout[0],
            ComputedLayout {
                size: Vec2::new(45., 10.),
                pos: Vec2::new(5., 22.)
            }
        );
        assert_eq!(layout[1].pos, Vec2::new(60., 22.));
        parent.swizzle = 1;
        let layout = place_childen(
            &parent,
            &ParentLayout {
                main: 40.,
                cross: 100.,
                offset: Vec2::new(5., 7.),
            },
            &children,
        );
        assert_eq!(
            layout[0],
            ComputedLayout {
                size: Vec2::new(20., 45.),
                pos: Vec2::new(15., 7.)
            }
        );
        assert_eq!(layout[1].pos, Vec2::new(15., 62.));
    }
    #[test]
    fn minimum_ignores_percentages_and_retains_padding_and_content() {
        let style = LayoutStyle {
            padding: [LengthPercentage::absolute(2.); 4],
            gap: LengthPercentage::absolute(3.),
            size: LengthPercentage {
                is_percent: true,
                value: 1.,
            },
            ..Default::default()
        };
        let intrinsic = [LengthPercentage::absolute(40.), LengthPercentage::default()];
        assert_eq!(
            style.compute_min_size_with_children(
                intrinsic,
                [Vec2::new(10., 8.), Vec2::new(20., 5.)]
            ),
            Vec2::new(40., 12.)
        );
        assert_eq!(
            style.compute_min_size_with_children([LengthPercentage::default(); 2], []),
            Vec2::splat(4.)
        );
    }
    /// O: builder tags from 0x68e930/960/990 and Size at +0x28/+0x30.
    #[test]
    fn style_builders_write_native_tags_and_size() {
        let style = LayoutStyle::default()
            .horizontal()
            .space_between()
            .xalign_center()
            .align_center()
            .gap(LengthPercentage::absolute(6.))
            .padding(Rect {
                left: LengthPercentage::absolute(1.),
                top: LengthPercentage::absolute(2.),
                right: LengthPercentage::absolute(3.),
                bottom: LengthPercentage::absolute(4.),
            })
            .expand()
            .size(Size {
                width: LengthPercentage::absolute(30.),
                height: LengthPercentage::absolute(20.),
            });
        assert_eq!(style.swizzle, 0);
        assert_eq!(style.justify, Justify(1));
        assert_eq!(style.align, 2);
        assert_eq!(style.gap.value, 6.);
        assert_eq!(style.padding.map(|p| p.value), [1., 2., 3., 4.]);
        assert_eq!(style.expand, 1.);
        assert_eq!(style.size.value, 30.);
        assert_eq!(style.min.value, 20.);
        assert_eq!(LayoutStyle::default().vertical().swizzle, 1);
    }
    /// O: points/points_or_zero on LengthPercentage/Size/Rect.
    #[test]
    fn points_accessors_resolve_or_zero() {
        let percent = LengthPercentage {
            is_percent: true,
            value: 0.5,
        };
        assert_eq!(percent.points(), 0.5);
        assert_eq!(percent.points_or_zero(), 0.);
        assert_eq!(LengthPercentage::absolute(7.).points_or_zero(), 7.);
        let size = Size {
            width: LengthPercentage::absolute(9.),
            height: percent,
        };
        assert_eq!(size.points(), Vec2::new(9., 0.5));
        assert_eq!(size.points_or_zero(), Vec2::new(9., 0.));
        let rect = Rect {
            left: LengthPercentage::absolute(2.),
            top: LengthPercentage::absolute(3.),
            right: LengthPercentage::absolute(4.),
            bottom: LengthPercentage::absolute(5.),
        };
        assert_eq!(rect.width(), 6.);
        assert_eq!(rect.height(), 8.);
        assert_eq!(rect.size_vec2(), Vec2::new(6., 8.));
        assert_eq!(rect.points(), [2., 3., 4., 5.]);
    }
}
