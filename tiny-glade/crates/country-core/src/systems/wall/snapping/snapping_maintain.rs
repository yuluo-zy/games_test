//! B `systems/wall/snapping/snapping_maintain.rs`: mode maintenance,
//! disable-notification and the snapping preview signifiers.
//!
//! Evidence: `chapters/systems/wall/snapping/snapping_maintain.rs.md` and
//! `snapping/snapping_signifiers.rs.md` (O). The PDB regroups
//! `snapping_signifiers` (0x140a6a650) under this file.

use bevy_ecs::prelude::*;
use glam::Vec3;

use super::{DisableSnapping, SNAP_MODE_END, SNAP_MODE_SEGMENT, SnappingMode};
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// Local proxy for the B flag resource read by check_for_disable_snapping
/// (param_2 first byte): non-zero means a modifier that disables snapping is
/// held. Exact resource identity 待取证.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DisableSnappingInput(pub bool);

/// B 0x140a6a560 (3-param). `flag == 1` also clears the mode; `flag == 0`
/// only writes the disable notification.
pub fn check_for_disable_snapping(
    mut mode: ResMut<SnappingMode>,
    input: Res<DisableSnappingInput>,
    mut events: EventWriter<DisableSnapping>,
) {
    match (mode.mode, input.0) {
        (SNAP_MODE_END, false) | (SNAP_MODE_SEGMENT, false) => {
            events.write(DisableSnapping(false));
        }
        (SNAP_MODE_END, true) | (SNAP_MODE_SEGMENT, true) => {
            events.write(DisableSnapping(true));
            mode.mode = super::SNAP_MODE_NONE;
        }
        (_, true) => {
            mode.mode = super::SNAP_MODE_NONE;
        }
        _ => {}
    }
}

/// B 0x140a6ad00: publish the full active region to the `+0x20..+0x38` copy
/// (mode@+0x20, snapped@+0x21, snap@+0x24..+0x30, wall@+0x30, end@+0x38).
/// Rust 此前只发布模式字节（1 字节），fresh 证实原发布**全状态副本**。
pub fn snapping_maintain(mut mode: ResMut<SnappingMode>) {
    mode.published.mode = mode.mode;
    mode.published.snapped = mode.snapped;
    mode.published.snap = mode.snap;
    mode.published.wall = mode.wall;
    mode.published.end = mode.end;
}

/// B 0x140a6a650：先清两个预览实体，再更新端点预览或提交完整段线和球工厂。
pub fn snapping_signifiers(
    mode: Res<SnappingMode>,
    mut ends: Query<&mut crate::utils::components::transform::Transform,
        (With<crate::components::SnappingToEndPreview>, Without<crate::components::SnappingToSegmentPreview>)>,
    mut segments: Query<&mut crate::utils::components::transform::Transform,
        (With<crate::components::SnappingToSegmentPreview>, Without<crate::components::SnappingToEndPreview>)>,
    public: Res<PublicWalls>,
    terrain: Res<crate::resources::terrain_height::TerrainHeightsData>,
    mut stream: ResMut<UiSignifierStream>,
) {
    let mut segment = segments.single_mut().expect("原段吸附预览实体必须唯一");
    segment.scale = Vec3::ZERO;
    let mut end = ends.single_mut().expect("原端点吸附预览实体必须唯一");
    end.scale = Vec3::ZERO;
    if mode.mode <= super::SNAP_MODE_SEGMENT_CANDIDATE {
        return;
    }
    if mode.mode == SNAP_MODE_END {
        end.translation = Vec3::new(mode.snap.y, mode.snap.x, mode.snap.z);
        end.scale = Vec3::splat(1.5);
        return;
    }
    let Some(state) = public.get(mode.wall) else {
        return;
    };
    // 原生调用方以参考点（snap 目标）作 position 传入 preview_snapping_line；
    // 函数内部做 get_approx_u_from_pos 投影（对称切片中点 = 投影点）。
    let position = Vec3::new(mode.snap.y, mode.snap.x, mode.snap.z);
    let Some(line) = crate::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line_data(
        state, position, &terrain, 4.0, 0.0, 0.0,
    ) else {
        return;
    };
    crate::utils::debug_viz::DebugLine::new(line.points).line_width(2.0).show(&mut stream);
    crate::utils::debug_viz::DebugSphere::new(line.anchor).radius(0.2).solid().show(&mut stream);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::wall::snapping::SNAP_MODE_NONE;

    #[test]
    fn original_preview_entities_and_segment_factories_have_real_owners() {
        use crate::{components::{SnappingToEndPreview, SnappingToSegmentPreview},
            resources::{roofs::geometry::surface::Curve2, terrain_height::TerrainHeightsData,
                walls::{InnerWalls, WallId, WallState, publication_source::TerrainHeightGrid512,
                    public_walls::{WallPublicationDirty, WallSpatialHash}}},
            systems::ui::signifiers::signifier_create::UiSignifierEntry,
            utils::components::transform::Transform};
        let mut inner = InnerWalls::new();
        let wall = WallId(42);
        inner.new_wall_with_id(wall, WallState::new(
            Curve2::try_new(vec![[0., 0.], [8., 0.]], false).unwrap(), 3., 0, 0, false));
        let terrain_grid = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain_grid, None,
            &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut world = World::new();
        let end = world.spawn((SnappingToEndPreview, Transform::identity())).id();
        let segment = world.spawn((SnappingToSegmentPreview, Transform::identity())).id();
        let mut mode = SnappingMode::new(); mode.mode = SNAP_MODE_END;
        mode.snap = Vec3::new(3., 4., 5.); mode.wall = wall;
        world.insert_resource(mode); world.insert_resource(public);
        world.insert_resource(TerrainHeightsData::<crate::resources::terrain_height::TerrainHeightSsbo>::new());
        world.init_resource::<UiSignifierStream>();
        let mut schedule = Schedule::default(); schedule.add_systems(snapping_signifiers);
        schedule.run(&mut world);
        let transform = world.get::<Transform>(end).unwrap();
        assert_eq!(transform.translation, Vec3::new(4., 3., 5.));
        assert_eq!(transform.scale, Vec3::splat(1.5));
        assert_eq!(world.get::<Transform>(segment).unwrap().scale, Vec3::ZERO);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        world.resource_mut::<SnappingMode>().mode = SNAP_MODE_SEGMENT;
        world.resource_mut::<SnappingMode>().snap = Vec3::new(0., 4., 0.);
        schedule.run(&mut world);
        assert_eq!(world.get::<Transform>(end).unwrap().scale, Vec3::ZERO);
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        assert_eq!(entries.len(), 2);
        for entry in entries {
            let UiSignifierEntry::Factory(factory) = entry else { panic!("原吸附工厂不能是数字raw") };
            assert!(factory.has_closure());
            assert!(factory.name().is_some_and(|name| !name.is_empty()));
        }
    }

    fn run_disable(input: bool, mode: u8) -> (u8, Vec<DisableSnapping>) {
        let mut world = World::new();
        world.insert_resource(SnappingMode::new());
        world.insert_resource(DisableSnappingInput(input));
        world.init_resource::<Events<DisableSnapping>>();
        world.resource_mut::<SnappingMode>().mode = mode;
        let mut schedule = Schedule::default();
        schedule.add_systems(check_for_disable_snapping);
        schedule.run(&mut world);
        let events = world
            .resource_mut::<Events<DisableSnapping>>()
            .drain()
            .collect::<Vec<_>>();
        (world.resource::<SnappingMode>().mode, events)
    }

    #[test]
    fn disable_without_flag_only_notifies() {
        let (mode, events) = run_disable(false, SNAP_MODE_END);
        assert_eq!(mode, SNAP_MODE_END);
        assert_eq!(events, vec![DisableSnapping(false)]);
    }

    #[test]
    fn disable_with_flag_also_resets_mode() {
        let (mode, events) = run_disable(true, SNAP_MODE_SEGMENT);
        assert_eq!(mode, SNAP_MODE_NONE);
        assert_eq!(events, vec![DisableSnapping(true)]);
    }
}
