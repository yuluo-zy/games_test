//! B0x140a92f80: use the existing renderer ArrowGizmoView component identity.
//! The original query also requires Transform, but does not read its fields.
//! Event readers are consumed inside the row loop, and axis guards live across
//! that loop. Numeric decay is0.1 per invocation; there is no clock parameter.
use crate::{
    resources::window::CursorMovedScreenSpace,
    systems::render_loop::draw_passes::draw_callbacks_a::ArrowGizmoView,
    utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;

#[derive(Event, Clone, Copy, Debug, Default)]
pub struct EnableArrowGizmoStretch;

fn decay(value: f32) -> f32 {
    let value = value + f32::from_bits(0xbdcc_cccd);
    if value > 0.0 { value } else { 0.0 }
}
fn increase(value: f32, delta: f32) -> f32 {
    let value = (delta * 0.02).abs() + value;
    if value < 1.0 { value } else { 1.0 }
}

/// Original consumes one shared enable reader for each matched row, and only
/// reads cursor deltas when that row sees an enable message. Thus no matched
/// rows consume no events; later rows see the already advanced reader cursors.
pub fn arrow_gizmo_animation(
    mut arrows: Query<ComponentWrite<ArrowGizmoView>, With<Transform>>,
    mut enabled: EventReader<EnableArrowGizmoStretch>,
    mut cursor: EventReader<CursorMovedScreenSpace>,
) {
    let mut horizontal = false;
    let mut vertical = false;
    for mut arrow in &mut arrows {
        let delta = if enabled.read().last().is_some() {
            cursor.read().last().map(|e| e.0)
        } else {
            None
        };
        let moving = delta.filter(|d| 0.0 < (d.y * d.y + d.x * d.x).sqrt());
        let mut next = [arrow.f0, arrow.f4];
        let mut changed = false;
        if let Some(delta) = moving {
            if delta.y.abs() > delta.x.abs() {
                next[1] = increase(next[1], delta.y);
                vertical = true;
            } else {
                next[0] = increase(next[0], delta.x);
                horizontal = true;
            }
            changed = true;
        }
        if !vertical {
            next[1] = decay(next[1]);
            changed = true;
        }
        if !horizontal {
            next[0] = decay(next[0]);
            changed = true;
        }
        if changed {
            arrow.f0 = next[0];
            arrow.f4 = next[1];
        }
    }
}

#[cfg(test)]
mod original_loop_tests {
    use super::*;
    fn observe(axes: &[[u32; 2]], enable_count: usize, delta: &[[u32; 2]]) -> Vec<[u32; 2]> {
        let mut world = World::new();
        world.init_resource::<Events<EnableArrowGizmoStretch>>();
        world.init_resource::<Events<CursorMovedScreenSpace>>();
        let entities: Vec<_> = axes
            .iter()
            .map(|a| {
                world
                    .spawn((
                        Transform::default(),
                        ArrowGizmoView {
                            f0: f32::from_bits(a[0]),
                            f4: f32::from_bits(a[1]),
                        },
                    ))
                    .id()
            })
            .collect();
        let excluded = world.spawn(ArrowGizmoView { f0: 0.5, f4: 0.6 }).id();
        for _ in 0..enable_count {
            world.send_event(EnableArrowGizmoStretch);
        }
        for d in delta {
            world.send_event(CursorMovedScreenSpace(glam::Vec2::new(
                f32::from_bits(d[0]),
                f32::from_bits(d[1]),
            )));
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(arrow_gizmo_animation);
        schedule.run(&mut world);
        assert_eq!(
            *world.get::<ArrowGizmoView>(excluded).unwrap(),
            ArrowGizmoView { f0: 0.5, f4: 0.6 }
        );
        entities
            .into_iter()
            .map(|entity| {
                let a = world.get::<ArrowGizmoView>(entity).unwrap();
                [a.f0.to_bits(), a.f4.to_bits()]
            })
            .collect()
    }
    #[test]
    fn original_full_loop_0() {
        assert_eq!(observe(&[], 0, &[]), Vec::<[u32; 2]>::new());
    }
    #[test]
    fn original_full_loop_1() {
        assert_eq!(observe(&[], 0, &[[0, 0]]), Vec::<[u32; 2]>::new());
    }
    #[test]
    fn original_full_loop_2() {
        assert_eq!(
            observe(&[], 0, &[[1092616192, 1065353216]]),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_3() {
        assert_eq!(
            observe(&[], 0, &[[1065353216, 3240099840]]),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_4() {
        assert_eq!(
            observe(&[], 0, &[[1073741824, 1073741824]]),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_5() {
        assert_eq!(
            observe(&[], 0, &[[2143289344, 1065353216]]),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_6() {
        assert_eq!(
            observe(
                &[],
                0,
                &[[1065353216, 1065353216], [1077936128, 1101004800]]
            ),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_7() {
        assert_eq!(observe(&[], 2, &[]), Vec::<[u32; 2]>::new());
    }
    #[test]
    fn original_full_loop_8() {
        assert_eq!(observe(&[], 2, &[[0, 0]]), Vec::<[u32; 2]>::new());
    }
    #[test]
    fn original_full_loop_9() {
        assert_eq!(
            observe(&[], 2, &[[1092616192, 1065353216]]),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_10() {
        assert_eq!(
            observe(&[], 2, &[[1065353216, 3240099840]]),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_11() {
        assert_eq!(
            observe(&[], 2, &[[1073741824, 1073741824]]),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_12() {
        assert_eq!(
            observe(&[], 2, &[[2143289344, 1065353216]]),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_13() {
        assert_eq!(
            observe(
                &[],
                2,
                &[[1065353216, 1065353216], [1077936128, 1101004800]]
            ),
            Vec::<[u32; 2]>::new()
        );
    }
    #[test]
    fn original_full_loop_14() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 0, &[]),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_15() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 0, &[[0, 0]]),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_16() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 0, &[[1092616192, 1065353216]]),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_17() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 0, &[[1065353216, 3240099840]]),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_18() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 0, &[[1073741824, 1073741824]]),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_19() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 0, &[[2143289344, 1065353216]]),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_20() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330]],
                0,
                &[[1065353216, 1065353216], [1077936128, 1101004800]]
            ),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_21() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 2, &[]),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_22() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 2, &[[0, 0]]),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_23() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 2, &[[1092616192, 1065353216]]),
            vec![[1060320051, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_24() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 2, &[[1065353216, 3240099840]]),
            vec![[1053609165, 1061997773]]
        );
    }
    #[test]
    fn original_full_loop_25() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 2, &[[1073741824, 1073741824]]),
            vec![[1057635697, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_26() {
        assert_eq!(
            observe(&[[1056964608, 1058642330]], 2, &[[2143289344, 1065353216]]),
            vec![[1053609165, 1056964608]]
        );
    }
    #[test]
    fn original_full_loop_27() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330]],
                2,
                &[[1065353216, 1065353216], [1077936128, 1101004800]]
            ),
            vec![[1053609165, 1065353216]]
        );
    }
    #[test]
    fn original_full_loop_28() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                0,
                &[]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_29() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                0,
                &[[0, 0]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_30() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                0,
                &[[1092616192, 1065353216]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_31() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                0,
                &[[1065353216, 3240099840]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_32() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                0,
                &[[1073741824, 1073741824]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_33() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                0,
                &[[2143289344, 1065353216]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_34() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                0,
                &[[1065353216, 1065353216], [1077936128, 1101004800]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_35() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                2,
                &[]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_36() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                2,
                &[[0, 0]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_37() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                2,
                &[[1092616192, 1065353216]]
            ),
            vec![[1060320051, 1056964608], [1060320051, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_38() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                2,
                &[[1065353216, 3240099840]]
            ),
            vec![[1053609165, 1061997773], [1058642329, 1061997773]]
        );
    }
    #[test]
    fn original_full_loop_39() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                2,
                &[[1073741824, 1073741824]]
            ),
            vec![[1057635697, 1056964608], [1060320051, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_40() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                2,
                &[[2143289344, 1065353216]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051]]
        );
    }
    #[test]
    fn original_full_loop_41() {
        assert_eq!(
            observe(
                &[[1056964608, 1058642330], [1060320051, 1061997773]],
                2,
                &[[1065353216, 1065353216], [1077936128, 1101004800]]
            ),
            vec![[1053609165, 1065353216], [1058642329, 1061997773]]
        );
    }
    #[test]
    fn original_full_loop_42() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                0,
                &[]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_43() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                0,
                &[[0, 0]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_44() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                0,
                &[[1092616192, 1065353216]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_45() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                0,
                &[[1065353216, 3240099840]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_46() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                0,
                &[[1073741824, 1073741824]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_47() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                0,
                &[[2143289344, 1065353216]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_48() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                0,
                &[[1065353216, 1065353216], [1077936128, 1101004800]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_49() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                2,
                &[]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_50() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                2,
                &[[0, 0]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_51() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                2,
                &[[1092616192, 1065353216]]
            ),
            vec![
                [1060320051, 1056964608],
                [1060320051, 1060320051],
                [3204448256, 0]
            ]
        );
    }
    #[test]
    fn original_full_loop_52() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                2,
                &[[1065353216, 3240099840]]
            ),
            vec![
                [1053609165, 1061997773],
                [1058642329, 1061997773],
                [0, 1028443341]
            ]
        );
    }
    #[test]
    fn original_full_loop_53() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                2,
                &[[1073741824, 1073741824]]
            ),
            vec![
                [1057635697, 1056964608],
                [1060320051, 1060320051],
                [3204448256, 0]
            ]
        );
    }
    #[test]
    fn original_full_loop_54() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                2,
                &[[2143289344, 1065353216]]
            ),
            vec![[1053609165, 1056964608], [1058642329, 1060320051], [0, 0]]
        );
    }
    #[test]
    fn original_full_loop_55() {
        assert_eq!(
            observe(
                &[
                    [1056964608, 1058642330],
                    [1060320051, 1061997773],
                    [3204448256, 1028443341]
                ],
                2,
                &[[1065353216, 1065353216], [1077936128, 1101004800]]
            ),
            vec![
                [1053609165, 1065353216],
                [1058642329, 1061997773],
                [0, 1028443341]
            ]
        );
    }
}

#[cfg(test)]
mod original_cursor_lifecycle_tests {
    use super::*;
    fn world() -> World {
        let mut world = World::new();
        world.init_resource::<Events<EnableArrowGizmoStretch>>();
        world.init_resource::<Events<CursorMovedScreenSpace>>();
        world
    }
    fn axes(world: &World, id: Entity) -> [u32; 2] {
        let a = world.get::<ArrowGizmoView>(id).unwrap();
        [a.f0.to_bits(), a.f4.to_bits()]
    }
    #[test]
    fn no_matching_rows_preserve_both_readers_for_a_later_spawn() {
        let mut world = world();
        world.send_event(EnableArrowGizmoStretch);
        world.send_event(EnableArrowGizmoStretch);
        world.send_event(CursorMovedScreenSpace(glam::Vec2::new(3., 20.)));
        let mut schedule = Schedule::default();
        schedule.add_systems(arrow_gizmo_animation);
        schedule.run(&mut world);
        let id = world
            .spawn((Transform::default(), ArrowGizmoView { f0: 0.5, f4: 0.6 }))
            .id();
        schedule.run(&mut world);
        assert_eq!(axes(&world, id), [1053609165, 1065353216]);
    }
    #[test]
    fn no_enable_preserves_cursor_until_a_later_enable() {
        let mut world = world();
        world.send_event(CursorMovedScreenSpace(glam::Vec2::new(3., 20.)));
        let id = world
            .spawn((Transform::default(), ArrowGizmoView { f0: 0.5, f4: 0.6 }))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(arrow_gizmo_animation);
        schedule.run(&mut world);
        world.send_event(EnableArrowGizmoStretch);
        schedule.run(&mut world);
        assert_eq!(axes(&world, id), [1050253722, 1063675494]);
    }
}

use crate::render::extract::ComponentWrite;
