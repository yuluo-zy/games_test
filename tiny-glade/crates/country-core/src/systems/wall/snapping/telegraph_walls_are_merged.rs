//! Event-owned wall-merge lines (B 14086ad70), with independent mesh lifetime.
//!
//! Original system signature: evidence/round12/app-edit-recording/registration-contract.json.
//! Full body: recovery/agent-team/planning/impl-walls-verify/decomp/
//! 0x14086ad70-telegraph_walls_are_merged.txt. Machine code 14086bbaa..bbc4
//! reverses easing input for flag != 0. The old all-wall timer/Raw UI stream
//! approximated an audio event as a factory; this system never uses that stream.
use bevy_ecs::prelude::*;
use glam::Vec3;
use utils::{curve::Curve3, easing_functions::EasingFunction};

use crate::{
    asset_libraries::{Handle, mesh_library::AssetMeshLibrary, shader_library::SharedAssetShaderLibrary},
    components::DashedLineLength,
    render::{mesh::Mesh, pass_builder::draw_list::DrawTopology},
    resources::{time::GameTime, walls::{WallId, public_walls::PublicWalls}},
    systems::{audio::PlayWallUiSound, ui::signifiers::SignifierEntityBuilder},
};

const TELEGRAPH_DURATION: f32 = 0.75;
const SLICE_MIN_LEN: f32 = 0.025;

/// Original event: wall @+8, direction flag @+0x10 including event ID.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayLineAcrossWall {
    pub wall: WallId,
    pub flag: u8,
}

/// The line owns its curve, so wall deletion does not invalidate its animation.
#[derive(Component, Clone, Debug)]
pub struct LineAcrossWallSignifier {
    pub curve: Curve3,
    pub wall: WallId,
    pub elapsed: f32,
    pub flag: u8,
}

fn slice_bounds(curve: &Curve3, t: f32, flag: u8) -> (f32, f32) {
    let e = EasingFunction::CircularInOut.apply(if flag == 0 { t } else { 1.0 - t });
    let mut lo = (e + e - 1.0).clamp(0.0, 1.0);
    let mut hi = (e + e).clamp(0.0, 1.0);
    let eps = SLICE_MIN_LEN / curve.length;
    if (hi - lo).abs() < eps + eps {
        lo = (lo - eps).clamp(0.0, 1.0);
        hi = (hi + eps).clamp(0.0, 1.0);
    }
    (lo, hi)
}

pub fn telegraph_walls_are_merged(
    mut requests: EventReader<PlayLineAcrossWall>,
    public: Res<PublicWalls>,
    mut lines: Query<(Entity, &mut LineAcrossWallSignifier, &Handle<Mesh>, &mut DashedLineLength)>,
    mut meshes: ResMut<AssetMeshLibrary>,
    shaders: Res<SharedAssetShaderLibrary>,
    mut commands: Commands,
    mut audio: EventWriter<PlayWallUiSound>,
    time: Res<GameTime>,
) {
    for request in requests.read() {
        let Some(wall) = public.get(request.wall) else { continue; };
        let height = wall.building_projection().float_0x284();
        let offset = Vec3::new(height * 0.0 * 0.5, height * 0.5, height * 0.0 * 0.5);
        let points = wall.geometry.outline.iter().map(|&point| Vec3::from(point) + offset).collect();
        let curve = Curve3::try_new_from_points(points, false).expect("invalid wall-merge line curve");
        audio.write(PlayWallUiSound { kind: 5, selector: 0, value: 0.0, position: curve.points[0] });
        let entity = SignifierEntityBuilder::from_points(curve.points.clone())
            .custom_shader("dashed_line").animation_duration(0.0)
            .build(&meshes, &shaders.0, &mut commands);
        commands.entity(entity).insert((
            DrawTopology::Alt(3.0), DashedLineLength(curve.length),
            LineAcrossWallSignifier { curve, wall: request.wall, elapsed: 0.0, flag: request.flag },
        ));
    }
    for (entity, mut line, mesh, mut length) in &mut lines {
        line.elapsed += time.delta;
        let t = (line.elapsed / TELEGRAPH_DURATION).clamp(0.0, 1.0);
        if t == 1.0 {
            commands.entity(entity).despawn();
            continue;
        }
        let (lo, hi) = slice_bounds(&line.curve, t, line.flag);
        let slice = line.curve.try_slice(lo, hi).expect("invalid animated wall-merge line slice");
        length.0 = slice.length;
        // Move the generated points into the mesh; get_mut publishes the actual
        // dirty mesh and COW preserves any already extracted render version.
        crate::utils::geo::update_curve_mesh(meshes.get_mut(*mesh), slice.points);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_libraries::{shader_library::AssetShaderLibrary, transient_mesh::delete_dropped_transient_meshes},
        render::{shader::RasterShaderProgram, test_device::TestDevice},
        resources::walls::{InnerWalls, WallState, publication_source::TerrainHeightGrid512},
        systems::ui::signifiers::UiSignifierStream,
    };
    use std::sync::Arc;

    fn fixture() -> (World, Schedule, WallId) {
        let mut world = World::new();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall_from_curve(WallState::new(
            crate::resources::roofs::geometry::surface::Curve2::try_new(vec![[0., 0.], [8., 0.]], false).unwrap(),
            3.0, 0, 2, false,
        ));
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut public = PublicWalls::default();
        public.consume(&mut inner, &terrain);
        assert!(public.get(wall).is_some());
        world.insert_resource(public);
        world.init_resource::<AssetMeshLibrary>();
        let shaders = Arc::new(AssetShaderLibrary::new());
        shaders.add_raster(RasterShaderProgram::new_draw("dashed_line", Default::default(), &TestDevice(1)), false);
        world.insert_resource(SharedAssetShaderLibrary(shaders));
        world.init_resource::<Events<PlayLineAcrossWall>>();
        world.init_resource::<Events<PlayWallUiSound>>();
        world.init_resource::<GameTime>();
        world.init_resource::<UiSignifierStream>();
        let mut schedule = Schedule::default();
        schedule.add_systems(telegraph_walls_are_merged);
        (world, schedule, wall)
    }

    #[test]
    fn walls_without_merge_events_never_accumulate_ui_records_or_spawn_lines() {
        let (mut world, mut schedule, _) = fixture();
        for _ in 0..4 { schedule.run(&mut world); }
        assert_eq!(world.query::<&LineAcrossWallSignifier>().iter(&world).count(), 0);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        assert!(world.resource::<Events<PlayWallUiSound>>().is_empty());
    }

    #[test]
    fn merge_event_owns_a_real_mesh_updates_it_and_reclaims_after_wall_deletion() {
        let (mut world, mut schedule, wall) = fixture();
        let offset = world.resource::<PublicWalls>().get(wall).unwrap().building_projection().float_0x284() * 0.5;
        world.send_event(PlayLineAcrossWall { wall, flag: 0 });
        schedule.run(&mut world);
        let mut query = world.query::<(Entity, &LineAcrossWallSignifier, &Handle<Mesh>, &DrawTopology, &DashedLineLength)>();
        let (entity, line, mesh, topology, length) = query.single(&world).unwrap();
        assert_eq!(*topology, DrawTopology::Alt(3.0));
        assert_eq!(line.curve.points[0].y, offset);
        assert_eq!(length.0, line.curve.length);
        let (entity, mesh) = (entity, *mesh);
        let event = world.resource::<Events<PlayWallUiSound>>().iter_current_update_events().next().unwrap();
        assert_eq!(event.kind, 5);
        assert_eq!(event.position, line.curve.points[0]);
        world.insert_resource(PublicWalls::default());
        world.resource_mut::<GameTime>().delta = 0.1875;
        schedule.run(&mut world);
        assert!(world.get::<LineAcrossWallSignifier>(entity).is_some());
        assert!(world.get::<DashedLineLength>(entity).unwrap().0 < 8.0);
        assert!(world.resource::<AssetMeshLibrary>().get(mesh).is_some());
        world.resource_mut::<GameTime>().delta = 0.75;
        schedule.run(&mut world);
        assert!(world.get_entity(entity).is_err());
        let mut meshes = world.remove_resource::<AssetMeshLibrary>().unwrap();
        let mut vaos = crate::asset_libraries::vao_library::AssetVAOLibrary::new();
        delete_dropped_transient_meshes(&TestDevice(1), &mut meshes, &mut vaos);
        assert!(meshes.get(mesh).is_none());
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
    }

    #[test]
    fn direction_flag_reverses_easing_without_reversing_the_owned_curve() {
        let curve = Curve3::try_new_from_points(vec![Vec3::ZERO, Vec3::X * 8.0], false).unwrap();
        let forward = slice_bounds(&curve, 0.25, 0);
        let reverse = slice_bounds(&curve, 0.25, 1);
        assert!(forward.0 == 0.0 && forward.1 < 0.5);
        assert!(reverse.0 > 0.5 && reverse.1 == 1.0);
        assert_eq!(forward, slice_bounds(&curve, 0.75, 1));
    }
}
