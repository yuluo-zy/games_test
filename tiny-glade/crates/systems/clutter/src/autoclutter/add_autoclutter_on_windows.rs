//! B1422cd7d0/1622B: drain canonical Window candidates into actual mesh outputs.
//! Native identity and exact value flow are bound in the Window consumer handoff.

use bevy_ecs::prelude::*;
use country_core::{
    resources::clutter_meshes::{ClutterMeshes, ClutterMeshTy},
    systems::collision::{WindowAutoClutterCandidates, WindowAutoClutterKind},
};
use glam::Vec3;

use super::{AutoClutterPayload, AutoClutterSpawn, AutoClutterState, ResolvedClutterMesh};

pub fn add_autoclutter_on_windows(
    mut state: ResMut<AutoClutterState>,
    meshes: Res<ClutterMeshes>,
    mut candidates: ResMut<WindowAutoClutterCandidates>,
) {
    for candidate in candidates.0.drain(..) {
        let (mesh, marker, selector_value) = match candidate.kind {
            WindowAutoClutterKind::WindowFlowerpot =>
                (meshes.mesh(ClutterMeshTy::WindowFlowerpot), 2, 0),
            WindowAutoClutterKind::WindowFlowerbed { mesh_index } =>
                (&meshes.window_flowerbeds()[mesh_index], 2, 0),
            WindowAutoClutterKind::WindowShutter { side, native_u32_04 } =>
                (meshes.mesh(ClutterMeshTy::WindowShutter), u8::from(!side), native_u32_04),
            WindowAutoClutterKind::DoorBell =>
                (meshes.mesh(ClutterMeshTy::DoorBell), 2, 0),
            WindowAutoClutterKind::DoorKrans =>
                (meshes.mesh(ClutterMeshTy::DoorKrans), 2, 0),
            WindowAutoClutterKind::DoorKransAutumn =>
                (meshes.mesh(ClutterMeshTy::DoorKransAutumn), 2, 0),
        };
        // Original+10 is a full quaternion, not a Vec3 with an invented w=0.
        let direction = candidate.rotation * Vec3::Z;
        let translation = candidate.translation;
        state.spawn_candidates.push(AutoClutterSpawn {
            kind: 5,
            payload: AutoClutterPayload::Mesh { metadata: ResolvedClutterMesh::from(mesh) },
            f18: 1,
            key: candidate.key,
            marker,
            selector_value,
            position: translation,
            host_id: candidate.wall.0,
            // Retain represented native scalar values, without copying padding.
            f48: u64::from(translation.x.to_bits()) | (u64::from(translation.y.to_bits()) << 32),
            f50: u64::from(translation.z.to_bits()),
            f54: direction.x.to_bits(),
            f58: direction.y.to_bits(),
            f5c: direction.z.to_bits(),
            scale: 1.0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::autoclutter::window_test_helpers::{candidate, world};

    #[test]
    fn actual_drain_routes_six_kinds_and_preserves_pose_key_wall_and_color() {
        let kinds = [WindowAutoClutterKind::WindowFlowerpot,
            WindowAutoClutterKind::WindowFlowerbed { mesh_index: 1 },
            WindowAutoClutterKind::WindowShutter { side: true, native_u32_04: 0xfedc_ba98 },
            WindowAutoClutterKind::DoorBell, WindowAutoClutterKind::DoorKrans,
            WindowAutoClutterKind::DoorKransAutumn];
        let mut world = world(kinds.into_iter().enumerate().map(|(i, kind)|
            candidate(0x1234_5678_0000_0000 + i as u64, kind)).collect());
        let expected = {
            let meshes = world.resource::<ClutterMeshes>();
            [meshes.mesh(ClutterMeshTy::WindowFlowerpot).name_hash,
                meshes.window_flowerbeds()[1].name_hash,
                meshes.mesh(ClutterMeshTy::WindowShutter).name_hash,
                meshes.mesh(ClutterMeshTy::DoorBell).name_hash,
                meshes.mesh(ClutterMeshTy::DoorKrans).name_hash,
                meshes.mesh(ClutterMeshTy::DoorKransAutumn).name_hash]
        };
        let mut schedule = Schedule::default();
        schedule.add_systems(add_autoclutter_on_windows);
        schedule.run(&mut world);
        assert!(world.resource::<WindowAutoClutterCandidates>().0.is_empty());
        for (i, spawn) in world.resource::<AutoClutterState>().spawn_candidates.iter().enumerate() {
            let AutoClutterPayload::Mesh { metadata } = &spawn.payload else {
                panic!("real metadata required")
            };
            assert_eq!(metadata.name_hash.0, expected[i].0);
            assert_eq!(spawn.key, 0x1234_5678_0000_0000 + i as u64);
            assert_eq!(spawn.host_id, 0xfedc_ba98_7654_3210);
            assert_eq!(spawn.position, Vec3::new(3.0, 7.0, -2.0));
            let direction = Vec3::new(f32::from_bits(spawn.f54), f32::from_bits(spawn.f58), f32::from_bits(spawn.f5c));
            assert!(direction.abs_diff_eq(Vec3::X, 1e-6));
            assert_eq!(spawn.scale, 1.0);
            assert_eq!(spawn.marker, if i == 2 { 0 } else { 2 });
            assert_eq!(spawn.selector_value, if i == 2 { 0xfedc_ba98 } else { 0 });
        }
        schedule.run(&mut world);
        assert_eq!(world.resource::<AutoClutterState>().spawn_candidates.len(), 6,
            "a drained input must not replay without a real producer recollection");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn flowerbed_index_keeps_original_checked_lookup_failure() {
        let mut world = world(vec![candidate(1, WindowAutoClutterKind::WindowFlowerbed { mesh_index: 2 })]);
        let mut schedule = Schedule::default();
        schedule.add_systems(add_autoclutter_on_windows);
        schedule.run(&mut world);
    }
}
