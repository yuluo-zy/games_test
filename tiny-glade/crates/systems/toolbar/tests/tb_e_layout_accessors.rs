//! TB-E (round 11): `mei::layout` style builders and Size/Rect accessors.
//!
//! Contract `tb-toolbar-CONTRACT.md` §2.4 (#21-#25): align/space builders,
//! `points`/`points_or_zero`, and `Rect::size_vec2`; plus the O builders from
//! the TB delivery (`horizontal`/`vertical`/`gap`/`padding`/`expand`/`size`).
//! Every assertion reads the public output fields or `place_childen` results.
use glam::Vec2;
use toolbar::mei::layout::{
    ChildLayoutInput, Justify, LayoutStyle, LengthPercentage, ParentLayout, Rect, Size,
    place_childen,
};

fn abs(value: f32) -> LengthPercentage {
    LengthPercentage::absolute(value)
}

fn frac(value: f32) -> LengthPercentage {
    LengthPercentage {
        is_percent: true,
        value,
    }
}

#[test]
fn default_style_matches_native_layout_defaults() {
    let style = LayoutStyle::default();
    assert_eq!(style.swizzle, 0);
    assert_eq!(style.justify, Justify::DEFAULT);
    assert_eq!(style.justify, Justify(4));
    assert_eq!(style.align, 0);
    assert_eq!(style.gap, LengthPercentage::default());
    assert_eq!(style.padding, [LengthPercentage::default(); 4]);
    assert_eq!(style.size, LengthPercentage::default());
    assert_eq!(style.min, LengthPercentage::default());
    assert_eq!(style.expand, 0.0);
    assert_eq!(style.offset, Vec2::ZERO);
    // The native style is a 72-byte block with size@+0x28 and height@+0x30.
    assert_eq!(std::mem::size_of::<LayoutStyle>(), 72);
}

#[test]
fn align_gap_padding_size_expand_builders_write_public_outputs() {
    let padding = Rect {
        left: abs(1.0),
        top: abs(2.0),
        right: abs(3.0),
        bottom: abs(4.0),
    };
    let style = LayoutStyle::default()
        .vertical()
        .gap(abs(6.0))
        .padding(padding)
        .expand()
        .size(Size {
            width: abs(30.0),
            height: abs(20.0),
        })
        .space_between()
        .xalign_center();

    assert_eq!(style.swizzle, 1, "vertical selects the transposed axis");
    assert_eq!(style.gap.value, 6.0);
    assert!(!style.gap.is_percent);
    assert_eq!(style.padding.map(|p| p.value), [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(style.expand, 1.0);
    // Native stores width at the `size` slot and height at the `min` slot.
    assert_eq!(style.size.value, 30.0);
    assert_eq!(style.min.value, 20.0);
    assert_eq!(style.justify, Justify(3), "space_between tag 3");
    assert_eq!(style.align, 2, "xalign_center tag 2");

    let centered = style.align_center();
    assert_eq!(centered.justify, Justify(1), "align_center tag 1");
    // horizontal is the swizzle-0 builder.
    assert_eq!(LayoutStyle::default().vertical().horizontal().swizzle, 0);
}

#[test]
fn length_size_rect_accessors_resolve_and_zero_percentages() {
    // LengthPercentage: absolute vs frac.
    assert_eq!(abs(7.0).points(), 7.0);
    assert_eq!(abs(7.0).points_or_zero(), 7.0);
    assert_eq!(frac(0.5).points(), 0.5);
    assert_eq!(frac(0.5).points_or_zero(), 0.0);
    assert_eq!(frac(0.5).resolve(200.0), 100.0);
    assert_eq!(abs(7.0).resolve(200.0), 7.0);
    assert_eq!(LengthPercentage::default(), abs(0.0));
    // Native Rect::width/height use the two generic `add` calls.
    let mixed = abs(1.0) + frac(0.5);
    assert!(mixed.is_percent);
    assert_eq!(mixed.value, 1.5);
    assert_eq!((frac(0.25) + frac(0.25)).value, 0.5);

    // Size projections.
    let size = Size {
        width: abs(9.0),
        height: frac(0.5),
    };
    assert_eq!(size.points(), Vec2::new(9.0, 0.5));
    assert_eq!(size.points_or_zero(), Vec2::new(9.0, 0.0));
    assert_eq!(Size::from(Vec2::new(4.0, 6.0)), Size { width: abs(4.0), height: abs(6.0) });

    // Rect width/height/size_vec2 and point projections.
    let rect = Rect {
        left: abs(2.0),
        top: abs(3.0),
        right: abs(4.0),
        bottom: abs(5.0),
    };
    assert_eq!(rect.width(), 6.0);
    assert_eq!(rect.height(), 8.0);
    assert_eq!(rect.size_vec2(), Vec2::new(6.0, 8.0));
    assert_eq!(rect.points(), [2.0, 3.0, 4.0, 5.0]);
    assert_eq!(rect.points_or_zero(), [2.0, 3.0, 4.0, 5.0]);

    // Percentages contribute zero to the rect edge sums; the mixed Add keeps
    // the percent tag (registered U for mixed edges), so the sum is zero too.
    let percent_rect = Rect {
        left: frac(0.5),
        top: frac(0.25),
        right: frac(0.5),
        bottom: frac(0.25),
    };
    assert_eq!(percent_rect.width(), 0.0);
    assert_eq!(percent_rect.height(), 0.0);
    assert_eq!(percent_rect.points(), [0.5, 0.25, 0.5, 0.25]);

    // Tuple/Size conversions preserve the native edge order.
    let from_tuple: Rect = (abs(1.0), abs(2.0), abs(3.0), abs(4.0)).into();
    assert_eq!(from_tuple.points(), [1.0, 2.0, 3.0, 4.0]);
    let from_size: Rect = size.into();
    assert_eq!(from_size.left, abs(9.0));
    assert_eq!(from_size.top, frac(0.5));
    assert_eq!(from_size.right, abs(0.0));
    assert_eq!(from_size.bottom, abs(0.0));
}

#[test]
fn place_childen_consumes_padding_gap_expand_and_size() {
    // Horizontal parent: 100 main, 30 cross, padding 5/5, gap 10.
    let style = LayoutStyle {
        padding: [abs(5.0), abs(0.0), abs(5.0), abs(0.0)],
        gap: abs(10.0),
        justify: Justify(0),
        align: 0,
        ..Default::default()
    };
    let intrinsic_x = [abs(20.0), abs(0.0)];
    let intrinsic_y = [abs(10.0), abs(0.0)];
    let child_style = LayoutStyle {
        expand: 1.0,
        ..Default::default()
    };
    let children = [
        ChildLayoutInput {
            style: &child_style,
            padding: &intrinsic_x,
            prev_x: 20.0,
            prev_y: 0.0,
        },
        ChildLayoutInput {
            style: &child_style,
            padding: &intrinsic_y,
            prev_x: 10.0,
            prev_y: 0.0,
        },
    ];
    let layout = place_childen(
        &style,
        &ParentLayout {
            main: 100.0,
            cross: 30.0,
            offset: Vec2::ZERO,
        },
        &children,
    );
    // remaining = 100 - 5 - 5 - 10 - (20 + 10) = 50, split evenly by expand=1.
    assert_eq!(layout[0].size, Vec2::new(45.0, 30.0));
    assert_eq!(layout[0].pos, Vec2::new(5.0, 0.0));
    // Second child's intrinsic main size is 10, so expand adds 25 on top.
    assert_eq!(layout[1].size, Vec2::new(35.0, 30.0));
    assert_eq!(layout[1].pos, Vec2::new(60.0, 0.0));

    // Center alignment without expand: free space is split around the group.
    let centered = LayoutStyle {
        padding: [abs(5.0), abs(0.0), abs(5.0), abs(0.0)],
        gap: abs(10.0),
        justify: Justify(1),
        align: 2,
        ..Default::default()
    };
    let plain = LayoutStyle::default();
    let intrinsic_a = [abs(20.0), abs(10.0)];
    let intrinsic_b = [abs(10.0), abs(6.0)];
    let plain_children = [
        ChildLayoutInput {
            style: &plain,
            padding: &intrinsic_a,
            prev_x: 10.0,
            prev_y: 10.0,
        },
        ChildLayoutInput {
            style: &plain,
            padding: &intrinsic_b,
            prev_x: 10.0,
            prev_y: 6.0,
        },
    ];
    let layout = place_childen(
        &centered,
        &ParentLayout {
            main: 100.0,
            cross: 30.0,
            offset: Vec2::splat(1.0),
        },
        &plain_children,
    );
    // free = 100 - 5 - 5 - 10 - (20 + 10) = 50 -> cursor 5 + 25 = 30.
    assert_eq!(layout[0].pos, Vec2::new(31.0, 11.0));
    assert_eq!(layout[1].pos, Vec2::new(61.0, 13.0));
    // xalign center: each child keeps its intrinsic cross size and is centered.
    assert_eq!(layout[0].size, Vec2::new(20.0, 10.0));
    assert_eq!(layout[1].size, Vec2::new(10.0, 6.0));
    assert_eq!(layout[0].pos.y, 1.0 + (30.0 - 10.0) * 0.5);
    assert_eq!(layout[1].pos.y, 1.0 + (30.0 - 6.0) * 0.5);

    // The `size` builder contributes the minimum main-axis size.
    let sized = LayoutStyle::default();
    let sized_child = LayoutStyle {
        size: abs(30.0),
        ..Default::default()
    };
    let sized_children = [ChildLayoutInput {
        style: &sized_child,
        padding: &intrinsic_x,
        prev_x: 0.0,
        prev_y: 0.0,
    }];
    let layout = place_childen(
        &sized,
        &ParentLayout {
            main: 100.0,
            cross: 30.0,
            offset: Vec2::ZERO,
        },
        &sized_children,
    );
    assert_eq!(layout[0].size.x, 30.0);
}
