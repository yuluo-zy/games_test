//! A4: one full VAO snapshot per world-graph declaration. The scope may only
//! skip a refresh while the snapshot is provably exact; every other case must
//! fall back to a full refresh.
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::{
        mesh_library::{AssetMeshLibrary, MeshAsset},
        shader_library::AssetShaderLibrary,
        vao_library::AssetVAOLibrary,
    },
    render::mesh::{Mesh, VertexAttributeValues},
    systems::render_loop::draw_passes::draw_lists::{DrawItem, DrawLists},
};
use std::sync::Arc;
use tiny_glade::render_bridge::CoreRenderBridge;

fn device() -> Option<Arc<rhapsody::device::Device>> {
    match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
        Ok(device) => Some(device),
        Err(error) => {
            assert!(
                !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                "strict GPU initialization: {error}"
            );
            eprintln!("SKIP vao scope: {error}");
            None
        }
    }
}

fn triangle() -> Mesh {
    let mut mesh = Mesh::new("scope");
    mesh.indices = vec![0, 1, 2];
    mesh.attributes.insert(
        "Vertex_Position".into(),
        VertexAttributeValues::Float2(vec![
            glam::vec2(-0.5, -0.5),
            glam::vec2(0.5, -0.5),
            glam::vec2(0.0, 0.5),
        ]),
    );
    mesh
}

#[test]
fn scope_skips_refresh_only_while_the_snapshot_is_exact() {
    let Some(device) = device() else { return };
    let root = std::env::temp_dir().join(format!("tg-vao-scope-{}", std::process::id()));
    let bridge =
        CoreRenderBridge::new(device, &root, Arc::new(AssetShaderLibrary::new())).unwrap();
    let raster = bridge.raster_resources().clone();
    let meshes = AssetMeshLibrary::new();
    let mesh = meshes.add(MeshAsset::new(triangle(), None));
    let mut vaos = AssetVAOLibrary::new();
    let vao = vaos
        .add(bridge.hosts().buffers.as_ref(), &meshes, mesh)
        .unwrap();
    let mut world = World::new();
    world.spawn(vao);
    world.insert_resource(vaos);
    let mut lists = DrawLists::new();
    fn unused(
        _: country_core::render::pass_builder::draw_builder::DrawBuilder<'_>,
        _: &country_core::render::pass_builder::draw_list::EntityDrawParams,
        _: &World,
    ) {
    }
    let mut item = DrawItem::new(1, 0);
    item.vao = vao.id().raw();
    lists.list_mut(0).push((item, unused));

    // Without a scope every call is a full refresh.
    let start = raster.vao_table_revision();
    raster.ensure_draw_vaos(&mut world, &lists).unwrap();
    let allocated = raster.vao_snapshot_allocations();
    raster.ensure_draw_vaos(&mut world, &lists).unwrap();
    assert_eq!(raster.vao_table_revision(), start + 2);
    assert_eq!(raster.vao_snapshot_allocations(), allocated,
        "static refresh shares immutable binding metadata and native views");

    {
        let _scope = raster.begin_vao_scope(&mut world, &lists).unwrap();
        let opened = raster.vao_table_revision();
        assert_eq!(opened, start + 3, "opening takes one full snapshot");
        for _ in 0..8 {
            raster.ensure_draw_vaos(&mut world, &lists).unwrap();
        }
        assert_eq!(raster.vao_table_revision(), opened, "same snapshot is reused");

        // A direct subset refresh replaces the table, so reuse must end.
        raster.refresh_vao_handles(&world, &[vao]).unwrap();
        let after_subset = raster.vao_table_revision();
        assert_eq!(after_subset, opened + 1);
        raster.ensure_draw_vaos(&mut world, &lists).unwrap();
        assert_eq!(raster.vao_table_revision(), after_subset + 1, "full refresh after subset");
        let restored = raster.vao_table_revision();
        raster.ensure_draw_vaos(&mut world, &lists).unwrap();
        assert_eq!(raster.vao_table_revision(), restored, "the open scope is re-armed");

        // Rebuilding a VAO keeps its identity but replaces its buffers.
        world
            .resource_mut::<AssetVAOLibrary>()
            .rebuild_vao(bridge.hosts().buffers.as_ref(), &meshes, mesh)
            .unwrap();
        raster.ensure_draw_vaos(&mut world, &lists).unwrap();
        assert_eq!(raster.vao_table_revision(), restored + 1, "library change refreshes");
        raster.ensure_draw_vaos(&mut world, &lists).unwrap();
        assert_eq!(raster.vao_table_revision(), restored + 1);

        // Different draw lists are never covered by the snapshot.
        let other = DrawLists::new();
        raster.ensure_draw_vaos(&mut world, &other).unwrap();
        assert_eq!(raster.vao_table_revision(), restored + 2, "other lists refresh");
    }
    // A cached view does not confer resurrection authority after host release.
    let native = world.resource::<AssetVAOLibrary>().get_by_handle(vao).unwrap().vao.bindings[0].buffer;
    assert!(bridge.hosts().buffers.release(&native));
    assert!(raster.refresh_vao_handles(&world, &[vao]).is_err());
    // Restore a valid publication before verifying the closed scope.
    world.resource_mut::<AssetVAOLibrary>()
        .rebuild_vao(bridge.hosts().buffers.as_ref(), &meshes, mesh).unwrap();
    // Closing the scope ends reuse.
    let closed = raster.vao_table_revision();
    raster.ensure_draw_vaos(&mut world, &lists).unwrap();
    assert_eq!(raster.vao_table_revision(), closed + 1);
}

#[test]
fn full_table_wrappers_survive_only_the_scoped_terrain_subset() {
    let Some(device) = device() else { return };
    let root = std::env::temp_dir().join(format!("tg-vao-wrapper-{}", std::process::id()));
    let bridge = CoreRenderBridge::new(device.clone(), &root, Arc::new(AssetShaderLibrary::new())).unwrap();
    let raster = bridge.raster_resources().clone();
    let meshes = AssetMeshLibrary::new();
    let first_mesh = meshes.add(MeshAsset::new(triangle(), None));
    let second_mesh = meshes.add(MeshAsset::new(triangle(), None));
    let mut vaos = AssetVAOLibrary::new();
    let first = vaos.add(bridge.hosts().buffers.as_ref(), &meshes, first_mesh).unwrap();
    let second = vaos.add(bridge.hosts().buffers.as_ref(), &meshes, second_mesh).unwrap();
    let old_index = vaos.get_by_handle(second).unwrap().vao.index_buffer;
    let old_owner = bridge.hosts().buffers.managed_buffer(&old_index).unwrap().shared();
    let old_weak = Arc::downgrade(&old_owner);
    drop(old_owner);
    let mut world = World::new();
    let first_entity = world.spawn(first).id();
    let second_entity = world.spawn(second).id();
    world.insert_resource(vaos);
    let mut lists = DrawLists::new();
    fn unused(
        _: country_core::render::pass_builder::draw_builder::DrawBuilder<'_>,
        _: &country_core::render::pass_builder::draw_list::EntityDrawParams,
        _: &World,
    ) {}
    for (handle, entity) in [(first, first_entity), (second, second_entity)] {
        let mut item = DrawItem::new(1, 0);
        item.vao = handle.id().raw();
        item.params.entity = entity;
        lists.list_mut(0).push((item, unused));
    }
    raster.refresh_draw_vaos(&mut world, &lists).unwrap();
    let allocated = raster.vao_snapshot_allocations();
    assert_eq!(allocated, 2);
    {
        let _wrappers = raster.preserve_vao_wrappers();
        raster.refresh_vao_handles(&world, &[first]).unwrap();
        let after_subset = raster.vao_table_revision();
        raster.refresh_draw_vaos(&mut world, &lists).unwrap();
        assert_eq!(raster.vao_table_revision(), after_subset + 1,
            "the second full publication still occurs");
        assert_eq!(raster.vao_snapshot_allocations(), allocated,
            "same current physical owners reuse both complete wrappers");

        // A retained physical owner is not authority to resurrect a dead ID.
        let stale = world.resource::<AssetVAOLibrary>().get_by_handle(second).unwrap().vao.bindings[0].buffer;
        assert!(bridge.hosts().buffers.release(&stale));
        assert!(raster.refresh_draw_vaos(&mut world, &lists).is_err());
        world.resource_mut::<AssetVAOLibrary>()
            .rebuild_vao(bridge.hosts().buffers.as_ref(), &meshes, second_mesh).unwrap();
        raster.refresh_draw_vaos(&mut world, &lists).unwrap();
        assert_eq!(raster.vao_snapshot_allocations(), allocated + 1,
            "same logical VAO with new native owners gets a new wrapper");
        device.wait_idle_and_collect().unwrap();
        assert!(old_weak.upgrade().is_some(), "only this declaration still retains the old wrapper");
    }
    device.wait_idle_and_collect().unwrap();
    assert!(old_weak.upgrade().is_none(), "scope end releases the previous physical owner");

    // There is no implicit cache outside the explicit synchronous guard.
    raster.refresh_vao_handles(&world, &[first]).unwrap();
    let no_scope = raster.vao_snapshot_allocations();
    raster.refresh_draw_vaos(&mut world, &lists).unwrap();
    assert_eq!(raster.vao_snapshot_allocations(), no_scope + 1);

    let before_cancel = raster.vao_snapshot_allocations();
    let cancelled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _wrappers = raster.preserve_vao_wrappers();
        raster.refresh_vao_handles(&world, &[first]).unwrap();
        panic!("cancel synchronous world declaration after terrain subset");
    }));
    assert!(cancelled.is_err());
    raster.refresh_draw_vaos(&mut world, &lists).unwrap();
    assert_eq!(raster.vao_snapshot_allocations(), before_cancel + 1,
        "cancelled declaration cannot retain its prior full table into another frame");
}

#[test]
fn vao_table_budget_denial_leaves_publication_unchanged_and_recovers() {
    let Some(device) = device() else { return };
    let root = std::env::temp_dir().join(format!("tg-vao-budget-{}", std::process::id()));
    let bridge = CoreRenderBridge::new(device.clone(), &root, Arc::new(AssetShaderLibrary::new())).unwrap();
    let raster = bridge.raster_resources().clone();
    let meshes = AssetMeshLibrary::new();
    let mesh = meshes.add(MeshAsset::new(triangle(), None));
    let mut vaos = AssetVAOLibrary::new();
    let vao = vaos.add(bridge.hosts().buffers.as_ref(), &meshes, mesh).unwrap();
    let mut world = World::new();
    world.insert_resource(vaos);
    device.wait_idle_and_collect().unwrap();
    let stats = device.allocator_stats();
    let current = stats.cpu_memory_usage().total();
    stats.set_host_budget(current).unwrap();
    let publication = raster.vao_table_revision();
    assert!(matches!(raster.refresh_vao_handles(&world, &[vao]), Err(rhapsody::Error::OutOfHostMemory)));
    assert_eq!(raster.vao_table_revision(), publication);
    assert_eq!(raster.vao_snapshot_allocations(), 0);
    assert_eq!(stats.cpu_memory_usage().total(), current);
    stats.set_host_budget(0).unwrap(); // Restore the API's unbounded host limit.
    raster.refresh_vao_handles(&world, &[vao]).unwrap();
    assert_eq!(raster.vao_table_revision(), publication + 1);
    assert_eq!(raster.vao_snapshot_allocations(), 1);
}

#[test]
fn two_table_budget_drops_optional_backup_and_completes_successive_frames() {
    let Some(device) = device() else { return };
    let root = std::env::temp_dir().join(format!("tg-vao-two-tables-{}", std::process::id()));
    let bridge = CoreRenderBridge::new(device.clone(), &root, Arc::new(AssetShaderLibrary::new())).unwrap();
    let raster = bridge.raster_resources().clone();
    let meshes = AssetMeshLibrary::new();
    let mut vaos = AssetVAOLibrary::new();
    let mut world = World::new();
    let mut lists = DrawLists::new();
    fn unused(
        _: country_core::render::pass_builder::draw_builder::DrawBuilder<'_>,
        _: &country_core::render::pass_builder::draw_list::EntityDrawParams,
        _: &World,
    ) {}
    let mut handles = Vec::new();
    for _ in 0..2 {
        let mesh = meshes.add(MeshAsset::new(triangle(), None));
        let handle = vaos.add(bridge.hosts().buffers.as_ref(), &meshes, mesh).unwrap();
        let entity = world.spawn(handle).id();
        let mut item = DrawItem::new(1, 0);
        item.vao = handle.id().raw(); item.params.entity = entity;
        lists.list_mut(0).push((item, unused));
        handles.push(handle);
    }
    world.insert_resource(vaos);
    device.wait_idle_and_collect().unwrap();
    raster.refresh_draw_vaos(&mut world, &lists).unwrap();
    let mut budget = 0;
    for frame in 0..3 {
        let wrappers = raster.preserve_vao_wrappers();
        raster.refresh_vao_handles(&world, &handles[..1]).unwrap();
        if frame == 0 {
            // Two tiny published maps already fit. This margin admits the
            // normal old/new table allocation peak, but not a third live map.
            budget = device.allocator_stats().cpu_memory_usage().total() + 128;
            device.allocator_stats().set_host_budget(budget).unwrap();
        }
        let before = raster.vao_snapshot_allocations();
        raster.refresh_draw_vaos(&mut world, &lists).unwrap();
        assert_eq!(raster.vao_snapshot_allocations(), before + 1,
            "dropping the backup makes the missing wrapper follow the original rebuild path");
        assert!(device.allocator_stats().cpu_memory_usage().total() <= budget);
        drop(wrappers);
        // Keep the same fixed limit: no full/subset/third-table OOM cycle.
        raster.refresh_draw_vaos(&mut world, &lists).unwrap();
    }
    device.allocator_stats().set_host_budget(0).unwrap();
}
