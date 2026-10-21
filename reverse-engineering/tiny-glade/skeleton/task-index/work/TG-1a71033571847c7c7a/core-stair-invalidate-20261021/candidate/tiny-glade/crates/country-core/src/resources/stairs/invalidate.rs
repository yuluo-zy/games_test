//! 楼梯脏边的物理、视觉、光追与历史失效；B 0x140b37180/4290B。
//! 原八个 owner 保留；装配参数影响集与权威图脏边集各自承担不同的删除范围。
#![cfg(feature = "native-physics")]

use std::hash::RandomState;

use bevy_ecs::prelude::*;
use hashbrown::HashSet;

use crate::{
    resources::{
        rt::{RtMeshHandle, RtWorld},
        stairs::{
            stairs_assembly_params::{PlayermadeStairsAssemblyRequests, StairGraphEdgeIdentity},
            stairs_collider::StairColliderRegistry,
            stairs_state::{StairGraphEdge, StairSegment, StairsState},
            structural_graph::StairsStructuralGraph,
            visual::StairsVisualState,
        },
        walls::decorator::DecoratorId,
    },
    systems::collision::world::{RaycastWorld, dispose_collider_mesh_instance},
};

/// 先失效受装配参数影响的结构碰撞，再按权威图脏边处理真实物理和视觉 owner。
pub fn invalidate_dirty_stair_artifacts(
    mut world: ResMut<RaycastWorld>,
    mut colliders: ResMut<StairColliderRegistry>,
    mut stairs: ResMut<StairsState>,
    mut structure: ResMut<StairsStructuralGraph>,
    mut visual: ResMut<StairsVisualState>,
    mut rt: ResMut<RtWorld>,
    mut requests: ResMut<PlayermadeStairsAssemblyRequests>,
    mut commands: Commands,
) {
    // 原源集合先完整 drain，空集合也必须清掉视觉全局脏位。
    let dirty_edges: HashSet<_, RandomState> = stairs.drain_dirty_edges().collect();
    visual.clear_dirty();

    let mut affected: HashSet<_, RandomState> = HashSet::with_hasher(RandomState::new());
    for params in &mut requests.requests {
        let (a, b) = params.segment.nodes();
        let edge = StairGraphEdge::new(a.0, b.0);
        // 原先写参数 dirty，再向影响集插入；已脏参数不依赖本轮图编辑。
        params.dirty = params.dirty || dirty_edges.contains(&edge);
        if params.dirty {
            affected.insert(edge);
        }
    }

    if !affected.is_empty() {
        let counts = structure.parts_mut().segment_counts;
        for edge in &affected {
            let _ = counts.remove(edge);
        }
        colliders
            .structural_colliders
            .retain(|(edge, _), instance| {
                if affected.contains(edge) {
                    // 网格强 owner 在同 world.mesh_sources 中，真实 dispose 负责释放。
                    dispose_collider_mesh_instance(&mut world, instance);
                    false
                } else {
                    true
                }
            });
    }

    for edge in dirty_edges {
        // 三个独立 owner 严格按 lane 0、1、2 删除，不能只从 registry 擦除句柄。
        let _ = colliders.remove_edge_colliders(&mut world, edge);
        let (a, b) = edge.nodes();
        if stairs.contains_edge_or_stub(a, b) {
            visual.force_rebuild(edge);
        } else {
            visual.remove_edge(edge, &mut commands);
            for neighbor in stairs.graph().neighbors(a) {
                visual.force_rebuild(StairGraphEdge::new(a, neighbor));
            }
            for neighbor in stairs.graph().neighbors(b) {
                visual.force_rebuild(StairGraphEdge::new(b, neighbor));
            }
        }

        // 原 packed identity 的首节点位于高32位，不能取 edge 内存的原生 qword。
        let identity =
            StairGraphEdgeIdentity::from(StairSegment::new(DecoratorId(a), DecoratorId(b)));
        rt.remove_mesh_if_present(&RtMeshHandle::from_handle(identity.packed()));
        let _ = requests.previous.sub_segments.remove(&identity);
        let _ = requests.previous.arch_wall_supports.remove(&identity);
        // sampled_points 由原插值 producer 维护，本消费者不删它。
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resources::{
            rt::{DirtyCow, RtMesh},
            stairs::stairs_assembly_params::{
                PlayermadeStairsAssemblyParams, SampledStairsPoints, StairArchWallSupportType,
                StairNodeAnchor, StairSamplesMetadata, StairSubSegment,
            },
            walls::WallId,
        },
        systems::collision::world::{
            ColliderMeshInstance, ColliderUpsert, RaycastGeoSource, RaycastMeshData,
            SubworldColliderHandle, upsert_collider,
        },
    };
    use bevy_ecs::world::CommandQueue;
    use glam::{Quat, Vec2, Vec3};
    use std::{
        collections::HashMap,
        sync::{Arc, Weak},
    };

    fn edge(a: u32, b: u32) -> StairGraphEdge {
        StairGraphEdge::new(a, b)
    }

    fn identity(a: u32, b: u32) -> StairGraphEdgeIdentity {
        StairSegment::new(DecoratorId(a), DecoratorId(b)).into()
    }

    fn params(a: u32, b: u32, dirty: bool) -> PlayermadeStairsAssemblyParams {
        PlayermadeStairsAssemblyParams {
            curve: utils::curve::Curve2::try_new_from_points(vec![Vec2::ZERO, Vec2::X], true)
                .unwrap(),
            heights: vec![0.0; 2],
            widths: vec![1.0; 2],
            tangents: vec![Vec2::X; 2],
            auxiliary_samples: Vec::new(),
            interpolated_curve: None,
            nodes: [(StairNodeAnchor::Terrain { height: 0.0 }, 0); 2],
            terrain_contacts: Default::default(),
            segment: StairSegment::new(DecoratorId(a), DecoratorId(b)),
            sub_segments: Default::default(),
            dirty,
        }
    }

    fn history(requests: &mut PlayermadeStairsAssemblyRequests, id: StairGraphEdgeIdentity) {
        requests.previous.sampled_points.insert(
            id,
            SampledStairsPoints {
                points: vec![Vec3::ZERO, Vec3::X],
                tangents: vec![Vec2::X; 2],
                widths: vec![1.0; 2],
                auxiliary_words: Vec::new(),
                metadata: StairSamplesMetadata {
                    info_28: 17,
                    extra_08: 1.0f32.to_bits(),
                    extra_0c: 0,
                    removed_support: false,
                    extra_10: 0,
                },
            },
        );
        requests
            .previous
            .arch_wall_supports
            .insert(id, StairArchWallSupportType::Kind1);
        requests.previous.sub_segments.insert(
            id,
            [StairSubSegment {
                kind: 0,
                flag: 0,
                count: 2,
            }]
            .into_iter()
            .collect(),
        );
    }

    fn owners(stairs: StairsState) -> World {
        let mut world = World::new();
        world.insert_resource(RaycastWorld::new());
        world.init_resource::<StairColliderRegistry>();
        world.insert_resource(stairs);
        world.init_resource::<StairsStructuralGraph>();
        world.init_resource::<StairsVisualState>();
        world.init_resource::<RtWorld>();
        world.init_resource::<PlayermadeStairsAssemblyRequests>();
        world
    }

    fn run(world: &mut World) {
        let mut schedule = Schedule::default();
        schedule.add_systems(invalidate_dirty_stair_artifacts);
        schedule.run(world);
    }

    fn insert_collider<K: Copy + std::hash::Hash + Eq>(
        world: &mut RaycastWorld,
        map: &mut HashMap<K, ColliderMeshInstance>,
        key: K,
        x: f32,
    ) -> (ColliderMeshInstance, Weak<RaycastMeshData>) {
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO));
        let weak = Arc::downgrade(&mesh);
        assert!(matches!(
            upsert_collider(
                world,
                map,
                key,
                RaycastGeoSource::Slot0,
                mesh,
                Vec3::new(x, 2.0, 0.0),
                Quat::IDENTITY
            ),
            ColliderUpsert::Inserted { .. }
        ));
        (map[&key], weak)
    }

    fn make_visual_dirty(world: &World) -> StairsVisualState {
        let mut visual = StairsVisualState::default();
        visual.edges_mut().entry(edge(90, 91)).or_default();
        let mut queue = CommandQueue::default();
        visual.remove_edge(edge(90, 91), &mut Commands::new(&mut queue, world));
        assert!(visual.is_dirty());
        visual
    }

    #[test]
    fn empty_dirty_source_still_clears_visual_global_flag() {
        let mut world = owners(StairsState::new());
        let visual = make_visual_dirty(&world);
        world.insert_resource(visual);
        run(&mut world);
        assert!(!world.resource::<StairsVisualState>().is_dirty());
        assert!(!world.resource::<RtWorld>().has_pending_changes());
        assert!(
            world
                .resource_mut::<StairsState>()
                .drain_dirty_edges()
                .next()
                .is_none()
        );
    }

    #[test]
    fn already_dirty_params_retire_only_their_structural_owners() {
        let mut world = owners(StairsState::new());
        let target = edge(2, 3);
        let unrelated = edge(4, 5);
        let target_id = identity(2, 3);
        let mut physics = world.remove_resource::<RaycastWorld>().unwrap();
        let mut registry = world.remove_resource::<StairColliderRegistry>().unwrap();
        let mut removed = Vec::new();
        let mut kept = Vec::new();
        for sample in [0, 9] {
            removed.push(insert_collider(
                &mut physics,
                &mut registry.structural_colliders,
                (target, sample),
                sample as f32,
            ));
        }
        kept.push(insert_collider(
            &mut physics,
            &mut registry.structural_colliders,
            (unrelated, 1),
            20.0,
        ));
        for (lane, map) in registry.edge_colliders.iter_mut().enumerate() {
            kept.push(insert_collider(
                &mut physics,
                map,
                target,
                30.0 + lane as f32,
            ));
        }
        world.insert_resource(physics);
        world.insert_resource(registry);
        world
            .resource_mut::<StairsStructuralGraph>()
            .parts_mut()
            .segment_counts
            .extend([(target, 2), (unrelated, 1)]);
        {
            let mut requests = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
            requests
                .requests
                .extend([params(2, 3, true), params(4, 5, false)]);
            history(&mut requests, target_id);
        }
        world.resource_mut::<RtWorld>().meshes.insert(
            RtMeshHandle::from_handle(target_id.packed()),
            DirtyCow::new(RtMesh::default()),
        );
        world
            .resource_mut::<StairsVisualState>()
            .edges_mut()
            .entry(target)
            .or_default();
        run(&mut world);
        let registry = world.resource::<StairColliderRegistry>();
        assert!(
            registry
                .structural_colliders
                .keys()
                .all(|key| key.0 != target)
        );
        assert!(registry.structural_colliders.contains_key(&(unrelated, 1)));
        assert!(
            registry
                .edge_colliders
                .iter()
                .all(|lane| lane.contains_key(&target))
        );
        assert!(removed.iter().all(|(_, weak)| weak.upgrade().is_none()));
        assert!(kept.iter().all(|(_, weak)| weak.upgrade().is_some()));
        assert!(
            !world
                .resource::<StairsStructuralGraph>()
                .segment_counts()
                .contains_key(&target)
        );
        assert_eq!(
            world.resource::<StairsStructuralGraph>().segment_counts()[&unrelated],
            1
        );
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert!(requests.requests[0].dirty);
        assert!(!requests.requests[1].dirty);
        assert!(requests.previous.sub_segments.contains_key(&target_id));
        assert!(
            requests
                .previous
                .arch_wall_supports
                .contains_key(&target_id)
        );
        assert!(requests.previous.sampled_points.contains_key(&target_id));
        assert!(
            world
                .resource::<RtWorld>()
                .meshes
                .contains_key(&RtMeshHandle::from_handle(target_id.packed()))
        );
        assert!(!world.resource::<RtWorld>().has_pending_changes());
        assert!(
            !world
                .resource::<StairsVisualState>()
                .get(&target)
                .unwrap()
                .force_rebuild_requested()
        );
        let mut physics = world.resource_mut::<RaycastWorld>();
        assert!(removed.iter().all(|(instance, _)| {
            physics
                .get_rapier_collider(instance.slot, instance.collider)
                .is_none()
        }));
        assert!(kept.iter().all(|(instance, _)| {
            physics
                .get_rapier_collider(instance.slot, instance.collider)
                .is_some()
        }));
    }

    #[test]
    fn source_stub_removes_three_lanes_in_order_and_only_its_two_histories() {
        let mut stairs = StairsState::new();
        stairs.add_node(7);
        let mut world = owners(stairs);
        let target = edge(7, 7);
        let target_id = identity(7, 7);
        let other_id = identity(8, 9);
        let mut physics = world.remove_resource::<RaycastWorld>().unwrap();
        let mut changes = physics.mesh_intersect_collider_diff_publisher().subscribe();
        let mut registry = world.remove_resource::<StairColliderRegistry>().unwrap();
        let removed: Vec<_> = registry
            .edge_colliders
            .iter_mut()
            .enumerate()
            .map(|(lane, map)| insert_collider(&mut physics, map, target, lane as f32))
            .collect();
        let _ = changes.take();
        world.insert_resource(physics);
        world.insert_resource(registry);
        {
            let mut requests = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
            requests.requests.push(params(7, 7, false));
            history(&mut requests, target_id);
            history(&mut requests, other_id);
        }
        let mut rt_weak = None;
        {
            let mut rt = world.resource_mut::<RtWorld>();
            for handle in [
                RtMeshHandle::from_handle(target_id.packed()),
                RtMeshHandle::from_wall_id(WallId(target_id.packed())),
                RtMeshHandle::from_prefab(target_id.packed()),
                RtMeshHandle::from_handle(other_id.packed()),
            ] {
                let entry = DirtyCow::new(RtMesh::default());
                if handle.tag == 1 && handle.payload == target_id.packed() {
                    rt_weak = Some(Arc::downgrade(&entry.inner));
                }
                rt.meshes.insert(handle, entry);
            }
        }
        world
            .resource_mut::<StairsVisualState>()
            .edges_mut()
            .entry(target)
            .or_default();
        run(&mut world);
        assert!(
            world
                .resource::<StairsVisualState>()
                .get(&target)
                .unwrap()
                .force_rebuild_requested()
        );
        assert!(!world.resource::<StairsVisualState>().is_dirty());
        assert!(
            world
                .resource::<StairColliderRegistry>()
                .edge_colliders
                .iter()
                .all(HashMap::is_empty)
        );
        assert!(removed.iter().all(|(_, weak)| weak.upgrade().is_none()));
        let diff = changes.take().unwrap();
        let handles: Vec<SubworldColliderHandle> = removed
            .iter()
            .map(|(instance, _)| instance.handle())
            .collect();
        assert_eq!(diff.removed, handles);
        assert!(diff.added.is_empty());
        assert!(rt_weak.unwrap().upgrade().is_none());
        let rt = world.resource::<RtWorld>();
        assert!(rt.has_pending_changes());
        assert_eq!(rt.meshes.len(), 3);
        assert!(
            !rt.meshes
                .contains_key(&RtMeshHandle::from_handle(target_id.packed()))
        );
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert!(requests.requests[0].dirty);
        assert!(!requests.previous.sub_segments.contains_key(&target_id));
        assert!(
            !requests
                .previous
                .arch_wall_supports
                .contains_key(&target_id)
        );
        assert!(requests.previous.sampled_points.contains_key(&target_id));
        assert!(requests.previous.sub_segments.contains_key(&other_id));
        assert!(requests.previous.arch_wall_supports.contains_key(&other_id));
        assert!(
            world
                .resource_mut::<StairsState>()
                .drain_dirty_edges()
                .next()
                .is_none()
        );
    }

    #[test]
    fn source_without_params_does_not_become_a_structural_invalidation() {
        let mut stairs = StairsState::new();
        stairs.add_node(7);
        let mut world = owners(stairs);
        let target = edge(7, 7);
        let mut physics = world.remove_resource::<RaycastWorld>().unwrap();
        let mut registry = world.remove_resource::<StairColliderRegistry>().unwrap();
        let (instance, weak) = insert_collider(
            &mut physics,
            &mut registry.structural_colliders,
            (target, 3),
            0.0,
        );
        world.insert_resource(physics);
        world.insert_resource(registry);
        world
            .resource_mut::<StairsStructuralGraph>()
            .parts_mut()
            .segment_counts
            .insert(target, 4);
        run(&mut world);
        assert!(weak.upgrade().is_some());
        assert_eq!(
            world
                .resource::<StairColliderRegistry>()
                .structural_colliders[&(target, 3)],
            instance
        );
        assert_eq!(
            world.resource::<StairsStructuralGraph>().segment_counts()[&target],
            4
        );
        assert!(!world.resource::<StairsVisualState>().contains(&target));
        assert!(!world.resource::<RtWorld>().has_pending_changes());
    }

    #[test]
    fn removed_edge_despawns_owned_entity_and_marks_each_surviving_neighbor() {
        let mut stairs = StairsState::new();
        for (a, b) in [(1, 2), (1, 3), (1, 5), (2, 4), (8, 9)] {
            stairs.add_edge(a, b);
        }
        let _ = stairs.drain_dirty_edges().count();
        stairs.disconnect_edge(1, 2);
        let mut world = owners(stairs);
        let entity = world.spawn_empty().id();
        {
            let mut visual = world.resource_mut::<StairsVisualState>();
            visual
                .edges_mut()
                .entry(edge(1, 2))
                .or_default()
                .entities
                .push(entity);
            for item in [edge(1, 3), edge(2, 4), edge(8, 9)] {
                visual.edges_mut().entry(item).or_default();
            }
        }
        run(&mut world);
        assert!(!world.entities().contains(entity));
        let visual = world.resource::<StairsVisualState>();
        assert!(!visual.contains(&edge(1, 2)));
        assert!(visual.get(&edge(1, 3)).unwrap().force_rebuild_requested());
        assert!(visual.get(&edge(2, 4)).unwrap().force_rebuild_requested());
        assert!(!visual.get(&edge(8, 9)).unwrap().force_rebuild_requested());
        assert!(!visual.contains(&edge(1, 5)));
        assert!(visual.is_dirty());
        assert!(!world.resource::<RtWorld>().has_pending_changes());
    }
}
