//! 原B140b37180/4290B：独立保留图脏A与请求脏B，按原8组资源回收楼梯伪影。
//! B仅失效结构采样，A再处理三lane/视觉/RT与旧装配表；不能合并后全清。
//! 原存在边仅force_rebuild，不调用reset_edge、不清仍存活的实例分配。
#[cfg(feature = "native-physics")]
use crate::resources::{rt::{RtWorld, RtMeshHandle}, stairs::{
    stairs_state::{StairsState, StairGraphEdge, StairSegment},
    stairs_collider::StairColliderRegistry, structural_graph::StairsStructuralGraph,
    stairs_assembly_params::{PlayermadeStairsAssemblyRequests, StairGraphEdgeIdentity},
    visual::StairsVisualState}, walls::decorator::DecoratorId};
#[cfg(feature = "native-physics")]
use bevy_ecs::prelude::*;

/// 原完整8组参数；无物理后端的构建不提供可冒充生产系统的降级空投影。
#[cfg(feature = "native-physics")]
#[allow(clippy::too_many_arguments)]
pub fn invalidate_dirty_stair_artifacts(
    mut world: ResMut<crate::systems::collision::world::RaycastWorld>,
    mut registry: ResMut<StairColliderRegistry>,
    mut stairs: ResMut<StairsState>,
    mut structural: ResMut<StairsStructuralGraph>,
    mut visual: ResMut<StairsVisualState>,
    mut rt: ResMut<RtWorld>,
    mut requests: ResMut<PlayermadeStairsAssemblyRequests>,
    mut commands: Commands,
) {
    use crate::systems::collision::world::{remove_collider, dispose_collider_mesh_instance};
    // 原先完整drain图脏A，再清视觉全局dirty；后面的force本身不能重新置全局dirty。
    let graph_dirty: std::collections::HashSet<_> = stairs.drain_dirty_edges().collect();
    visual.clear_dirty();
    let mut request_dirty = std::collections::HashSet::new();
    for request in &mut requests.requests {
        let (a, b) = request.segment.nodes();
        let edge = StairGraphEdge::new(a.0, b.0);
        request.dirty = request.dirty || graph_dirty.contains(&edge);
        if request.dirty { request_dirty.insert(edge); }
    }
    // 原B先删segment_counts，再按B retain真实采样collider。A\B的删除边不混入此阶段。
    if !request_dirty.is_empty() {
        for edge in &request_dirty { structural.parts_mut().segment_counts.remove(edge); }
        registry.structural_colliders.retain(|(edge, _), instance| {
            if !request_dirty.contains(edge) { return true; }
            dispose_collider_mesh_instance(&mut world, instance);
            false
        });
    }
    for edge in graph_dirty {
        for lane in &mut registry.edge_colliders { remove_collider(&mut world, lane, &edge); }
        let (a, b) = edge.nodes();
        if stairs.contains_edge_or_stub(a, b) {
            visual.force_rebuild(edge);
        } else {
            visual.remove_edge_with_commands(edge, &mut commands);
            // 原删除分支分别遍历两个端点的当前邻居；不创建缺失视觉entry。
            for node in [a, b] {
                for neighbor in stairs.graph().neighbors(node) {
                    visual.force_rebuild(StairGraphEdge::new(node, neighbor));
                }
            }
        }
        let identity = StairGraphEdgeIdentity::from(StairSegment::new(DecoratorId(a), DecoratorId(b)));
        rt.remove_mesh_if_present(&RtMeshHandle::from_handle(identity.packed()));
        // 原只移除这两张前帧表；sampled_points和本轮请求/有向图必须保留。
        requests.previous.sub_segments.remove(&identity);
        requests.previous.arch_wall_supports.remove(&identity);
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod tests {
    use super::*;
    use crate::{resources::{rt::{DirtyCow, RtMesh}, stairs::stairs_assembly_params::{
        PlayermadeStairsAssemblyParams, StairNodeAnchor, StairArchWallSupportType,
        SampledStairsPoints, StairSamplesMetadata}},
        systems::collision::world::{RaycastWorld, RaycastMeshData, RaycastGeoSource, upsert_collider}};
    use bevy_ecs::system::RunSystemOnce;
    use glam::{Vec2, Vec3, Quat};
    use std::sync::Arc;

    fn setup() -> World {
        let mut world = World::new();
        world.insert_resource(RaycastWorld::new());
        world.init_resource::<StairColliderRegistry>();
        world.init_resource::<StairsState>();
        world.init_resource::<StairsStructuralGraph>();
        world.init_resource::<StairsVisualState>();
        world.init_resource::<RtWorld>();
        world.init_resource::<PlayermadeStairsAssemblyRequests>();
        world
    }

    fn identity(edge: StairGraphEdge) -> StairGraphEdgeIdentity {
        let (a, b) = edge.nodes();
        StairGraphEdgeIdentity::from(StairSegment::new(DecoratorId(a), DecoratorId(b)))
    }

    fn request(edge: StairGraphEdge, dirty: bool) -> PlayermadeStairsAssemblyParams {
        let (a, b) = edge.nodes();
        PlayermadeStairsAssemblyParams {
            curve: utils::curve::Curve2::crashy_new_from_points(vec![Vec2::ZERO, Vec2::X], false),
            heights: vec![0.0, 1.0], widths: vec![1.0, 1.0], tangents: vec![Vec2::X; 2],
            auxiliary_samples: vec![], interpolated_curve: None,
            nodes: [(StairNodeAnchor::Terrain { height: 0.0 }, 0); 2],
            terrain_contacts: bitvec::vec::BitVec::repeat(false, 2),
            segment: StairSegment::new(DecoratorId(a), DecoratorId(b)),
            sub_segments: Default::default(), dirty,
        }
    }

    fn install_colliders(world: &mut World, edges: &[StairGraphEdge]) {
        let mut raycast = world.remove_resource::<RaycastWorld>().unwrap();
        let mut registry = world.remove_resource::<StairColliderRegistry>().unwrap();
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO));
        for &edge in edges {
            for lane in &mut registry.edge_colliders {
                upsert_collider(&mut raycast, lane, edge, RaycastGeoSource::PlatformStair,
                    mesh.clone(), Vec3::ZERO, Quat::IDENTITY);
            }
            for sample in 0..2 {
                upsert_collider(&mut raycast, &mut registry.structural_colliders, (edge, sample),
                    RaycastGeoSource::PlatformStair, mesh.clone(), Vec3::ZERO, Quat::IDENTITY);
            }
            world.resource_mut::<StairsStructuralGraph>().parts_mut().segment_counts.insert(edge, 2);
        }
        world.insert_resource(raycast); world.insert_resource(registry);
    }

    #[test]
    fn existing_edges_force_without_default_reset_and_removed_edges_reach_neighbor_visuals() {
        let mut world = setup();
        let kept = StairGraphEdge::new(1, 2); let removed = StairGraphEdge::new(3, 4);
        let neighbor = StairGraphEdge::new(3, 5);
        for (a, b) in [(1, 2), (3, 4), (3, 5)] { world.resource_mut::<StairsState>().add_edge(a, b); }
        let _ = world.resource_mut::<StairsState>().drain_dirty_edges().count();
        for edge in [kept, removed, neighbor] { world.resource_mut::<StairsVisualState>().reset_edge(edge); }
        world.resource_mut::<StairsState>().disconnect_edge(3, 4);
        // 原remove传播只依赖当前图邻接；对缺失entry不会伪造新视觉。
        world.run_system_once(invalidate_dirty_stair_artifacts).unwrap();
        let visual = world.resource::<StairsVisualState>();
        assert!(!visual.get(&kept).unwrap().force_rebuild_requested());
        assert!(visual.get(&neighbor).unwrap().force_rebuild_requested());
        assert!(!visual.contains(&removed)); assert!(visual.is_dirty());
        assert_eq!(visual.edges().len(), 2);
        assert!(world.resource_mut::<StairsState>().drain_dirty_edges().next().is_none());
        world.run_system_once(invalidate_dirty_stair_artifacts).unwrap();
        assert!(!world.resource::<StairsVisualState>().is_dirty());
    }

    #[test]
    fn graph_dirty_and_request_dirty_keep_original_different_retention_domains() {
        let mut world = setup();
        let a = StairGraphEdge::new(1, 2); let b = StairGraphEdge::new(5, 6); let c = StairGraphEdge::new(9, 10);
        for edge in [a, b, c] { let (x, y) = edge.nodes(); world.resource_mut::<StairsState>().add_edge(x, y); }
        let _ = world.resource_mut::<StairsState>().drain_dirty_edges().count();
        world.resource_mut::<StairsState>().disconnect_edge(1, 2); // A只有a。
        install_colliders(&mut world, &[a, b, c]);
        let mut rt_weak = Vec::new();
        for edge in [a, b, c] {
            world.resource_mut::<StairsVisualState>().reset_edge(edge);
            let id = identity(edge);
            let mut requests = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
            requests.previous.sub_segments.insert(id, Default::default());
            requests.previous.arch_wall_supports.insert(id, StairArchWallSupportType::Kind1);
            requests.previous.sampled_points.insert(id, SampledStairsPoints {
                points: vec![Vec3::new(9.0, 8.0, 7.0)], tangents: vec![Vec2::Y], widths: vec![3.0],
                auxiliary_words: vec![0x7fc1_2345], metadata: StairSamplesMetadata {
                    info_28: 71, extra_08: 72, extra_0c: 73, removed_support: false, extra_10: 9 } });
            drop(requests);
            let cow = DirtyCow::new(RtMesh::empty()); rt_weak.push(Arc::downgrade(&cow.inner));
            world.resource_mut::<RtWorld>().meshes.insert(RtMeshHandle::from_handle(id.packed()), cow);
        }
        world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests = vec![request(b, true), request(c, false)];
        world.run_system_once(invalidate_dirty_stair_artifacts).unwrap();
        let registry = world.resource::<StairColliderRegistry>();
        for lane in &registry.edge_colliders { assert!(!lane.contains_key(&a)); assert!(lane.contains_key(&b)); assert!(lane.contains_key(&c)); }
        assert!(registry.structural_colliders.contains_key(&(a, 0))); // A\B保持原合同。
        assert!(!registry.structural_colliders.contains_key(&(b, 0))); assert!(!registry.structural_colliders.contains_key(&(b, 1)));
        assert!(registry.structural_colliders.contains_key(&(c, 0)));
        let counts = world.resource::<StairsStructuralGraph>().segment_counts();
        assert_eq!(counts[&a], 2); assert!(!counts.contains_key(&b)); assert_eq!(counts[&c], 2);
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(requests.requests.len(), 2); assert!(requests.requests[0].dirty); assert!(!requests.requests[1].dirty);
        assert!(!requests.previous.sub_segments.contains_key(&identity(a)));
        assert!(!requests.previous.arch_wall_supports.contains_key(&identity(a)));
        assert_eq!(requests.previous.sampled_points[&identity(a)].auxiliary_words, vec![0x7fc1_2345]);
        assert!(requests.previous.sub_segments.contains_key(&identity(b)));
        assert!(requests.previous.arch_wall_supports.contains_key(&identity(c)));
        assert!(rt_weak[0].upgrade().is_none()); assert!(rt_weak[1].upgrade().is_some()); assert!(rt_weak[2].upgrade().is_some());
        assert!(world.resource::<RtWorld>().has_pending_changes());
        assert!(!world.resource::<StairsVisualState>().get(&b).unwrap().force_rebuild_requested());
        // 下一帧B仍为true（原下游装配负责清），但不会误删其RT/三lane/视觉。
        world.run_system_once(invalidate_dirty_stair_artifacts).unwrap();
        assert!(world.resource::<RtWorld>().meshes.contains_key(&RtMeshHandle::from_handle(identity(b).packed())));
        assert!(world.resource::<StairColliderRegistry>().edge_colliders[0].contains_key(&b));
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m01-invalidate-stair-request-predicate-cases.rs"));

    #[test]
    fn real_requests_match_original_fold_for_every_canonical_input_fixture() {
        for &(table, old, nodes, expected) in ORIGINAL_REQUEST_DIRTY {
            let mut world = setup(); let edge = StairGraphEdge::new(nodes[0], nodes[1]);
            match table {
                0 => {},
                1 if nodes[0] == nodes[1] => world.resource_mut::<StairsState>().add_node(nodes[0]),
                1 => world.resource_mut::<StairsState>().add_edge(nodes[0], nodes[1]),
                _ => world.resource_mut::<StairsState>().add_edge(91, 92),
            }
            world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests.push(request(edge, old));
            world.resource_mut::<StairsStructuralGraph>().parts_mut().segment_counts.insert(edge, 7);
            world.run_system_once(invalidate_dirty_stair_artifacts).unwrap();
            assert_eq!(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].dirty, expected,
                "原fold table={table} old={old} edge={nodes:?}");
            assert_eq!(!world.resource::<StairsStructuralGraph>().segment_counts().contains_key(&edge), expected);
        }
    }

    #[test]
    fn absent_visual_and_rt_are_not_new_entries_or_false_dirty_and_empty_frames_stay_stable() {
        let mut world = setup();
        world.resource_mut::<StairsState>().add_edge(1, 2);
        let _ = world.resource_mut::<StairsState>().drain_dirty_edges().count();
        world.resource_mut::<StairsState>().disconnect_edge(1, 2);
        let unrelated = StairGraphEdge::new(9, 10);
        world.resource_mut::<StairsVisualState>().reset_edge(unrelated);
        world.resource_mut::<RtWorld>().meshes.insert(RtMeshHandle::from_prefab(17), DirtyCow::new(RtMesh::empty()));
        for _ in 0..3 {
            world.run_system_once(invalidate_dirty_stair_artifacts).unwrap();
            assert!(!world.resource::<StairsVisualState>().is_dirty());
            assert_eq!(world.resource::<StairsVisualState>().edges().len(), 1);
            assert!(!world.resource::<RtWorld>().has_pending_changes());
            assert_eq!(world.resource::<RtWorld>().meshes.len(), 1);
        }
    }

    #[test]
    fn no_dirty_requests_does_not_scan_or_mark_actual_physics_and_structure_owners_changed() {
        let mut world = setup();
        let registry = world.get_resource_ref::<StairColliderRegistry>().unwrap().last_changed();
        let physics = world.get_resource_ref::<RaycastWorld>().unwrap().last_changed();
        let structural = world.get_resource_ref::<StairsStructuralGraph>().unwrap().last_changed();
        let rt = world.get_resource_ref::<RtWorld>().unwrap().last_changed();
        world.increment_change_tick();
        world.run_system_once(invalidate_dirty_stair_artifacts).unwrap();
        assert_eq!(world.get_resource_ref::<StairColliderRegistry>().unwrap().last_changed(), registry);
        assert_eq!(world.get_resource_ref::<RaycastWorld>().unwrap().last_changed(), physics);
        assert_eq!(world.get_resource_ref::<StairsStructuralGraph>().unwrap().last_changed(), structural);
        assert_eq!(world.get_resource_ref::<RtWorld>().unwrap().last_changed(), rt);
    }

    #[test]
    fn removed_edge_despawns_real_entities_and_recycles_all_four_real_sparse_allocations() {
        use crate::{geometry::instanced_wall::BrickTransformSsbo,
            render::{sparse_instance_buffer::{SparseInstanceBuffer, MeshAtlasAllocation}, test_device::TestDevice}};
        let mut first = SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&TestDevice(71)).unwrap();
        let mut second = SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&TestDevice(72)).unwrap();
        let initial_first = first.byte_map.to_vec(); let initial_second = second.byte_map.to_vec();
        let free_first = first.free_chunks.read().len(); let free_second = second.free_chunks.read().len();
        let mut world = setup(); let edge = StairGraphEdge::new(41, 42);
        world.resource_mut::<StairsState>().add_edge(41, 42);
        let _ = world.resource_mut::<StairsState>().drain_dirty_edges().count();
        world.resource_mut::<StairsState>().disconnect_edge(41, 42);
        let entities = [world.spawn_empty().id(), world.spawn_empty().id()];
        {
            let mut visual = world.resource_mut::<StairsVisualState>(); visual.reset_edge(edge);
            let item = visual.get_mut(&edge).unwrap(); item.entities.extend(entities);
            let (a, b) = item.allocations_mut(); *a = Some(MeshAtlasAllocation::new(&first)); *b = Some(MeshAtlasAllocation::new(&second));
            first.writer_for(a.as_ref().unwrap().allocation()).push(&BrickTransformSsbo::default()).unwrap();
            second.writer_for(b.as_ref().unwrap().allocation()).push(&BrickTransformSsbo::default()).unwrap();
            item.magic_supports.clear(&mut first, &mut second);
            let (a, b) = item.magic_supports.allocations();
            first.writer_for(a.unwrap().allocation()).push(&BrickTransformSsbo::default()).unwrap();
            second.writer_for(b.unwrap().allocation()).push(&BrickTransformSsbo::default()).unwrap();
        }
        assert_eq!(first.free_chunks.read().len(), free_first - 2); assert_eq!(second.free_chunks.read().len(), free_second - 2);
        world.run_system_once(invalidate_dirty_stair_artifacts).unwrap();
        assert!(entities.into_iter().all(|entity| world.get_entity(entity).is_err()));
        assert_eq!(first.waste.lock().len(), 2); assert_eq!(second.waste.lock().len(), 2);
        first.take_out_waste(); second.take_out_waste();
        assert_eq!(first.free_chunks.read().len(), free_first); assert_eq!(second.free_chunks.read().len(), free_second);
        // 原末尾是已满哨兵32，不是可归还chunk；完整byte map必须逐字节回到初始值。
        assert_eq!(&*first.byte_map, &initial_first); assert_eq!(&*second.byte_map, &initial_second);
        world.run_system_once(invalidate_dirty_stair_artifacts).unwrap();
        assert!(first.waste.lock().is_empty()); assert!(second.waste.lock().is_empty());
    }
}
