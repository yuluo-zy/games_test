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
