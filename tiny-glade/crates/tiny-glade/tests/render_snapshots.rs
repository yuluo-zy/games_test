//! Render snapshots retain only declaration state, share immutable pixels,
//! and do not acquire the game's allocation/free authority.
use country_core::{
    asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType},
    render::{
        sparse_instance_buffer::{SLOTS_PER_CHUNK, SparseInstanceBuffer, SparseSsbo},
        ssbo::{
            StorageElement,
            rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice},
        },
    },
    rhapsody_types::{BufferGraphBackend, ImportedBuffer, RenderGraph},
};
use std::{
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
struct Instance(u32);
impl StorageElement for Instance {
    const NAME: &'static str = "SnapshotTestInstance";
    const BUFFER_LEN: usize = 128 * SLOTS_PER_CHUNK;
}
// SAFETY: transparent u32 has no padding; capacity is exactly chunks * 32.
unsafe impl SparseSsbo for Instance {
    const CHUNK_COUNT: usize = 128;
}
impl SsboType for Instance {
    type Storage = SparseInstanceBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        SparseInstanceBuffer::new_storage_buffer(device, initial)
    }
}

#[derive(Default)]
struct Device {
    released: AtomicUsize,
}
impl GraphicsDevice for Device {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        _: &str,
        _: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        Ok(Buffer {
            id: 23,
            generation: 7,
            byte_size: desc.size as u64,
            ..Default::default()
        })
    }
    fn update_buffer_bytes(
        &self,
        _: &Buffer,
        _: &[u8],
        _: &[Range<usize>],
        _: usize,
    ) -> Result<(), BackendError> {
        Ok(())
    }
    fn release_buffer(&self, _: &Buffer) {
        self.released.fetch_add(1, Ordering::SeqCst);
    }
}

struct Graph;
impl BufferGraphBackend for Graph {
    fn import_unowned(&mut self, resource: u64, access: u8) -> ImportedBuffer {
        ImportedBuffer {
            resource,
            generation: 1,
            access: access as u32,
        }
    }
    fn export(&mut self, _: ImportedBuffer, _: u8) {}
}

#[test]
fn snapshot_preserves_draws_across_game_edits_and_reclamation_and_leases_gpu_owner() {
    let device = Arc::new(Device::default());
    let mut game = AssetSsboLibrary::new();
    game.register::<Instance>(SparseInstanceBuffer::try_new_owned(device.clone()).unwrap());
    let storage = game.get_mut::<Instance>().unwrap();
    let allocation = storage.new_allocation();
    storage
        .writer(&allocation)
        .replace_all(&[Instance(1); 35])
        .unwrap();
    let extracted_allocation = allocation.clone();
    let mut render = game.clone();
    let retained = render.clone();

    assert!(
        render.get::<Instance>().is_none(),
        "snapshot must not expose the CPU storage type"
    );
    assert!(
        render.get_mut::<Instance>().is_none(),
        "snapshot must not grant write/free authority"
    );
    assert_eq!(
        render.get_buffer::<Instance>(),
        game.get_buffer::<Instance>()
    );
    assert_eq!(
        render
            .sparse_draw_view::<Instance>()
            .unwrap()
            .to_draw_multi_instanced(),
        [(0, 35)]
    );

    let storage = game.get_mut::<Instance>().unwrap();
    storage
        .writer(&allocation)
        .replace_all(&[Instance(2); 3])
        .unwrap();
    assert_eq!(storage.to_draw_multi_instanced(), [(0, 3)]);
    drop(allocation);
    // Taking a snapshot does not create or consume entries in the game's waste queue.
    assert_eq!(storage.waste.lock().len(), 1);
    storage.take_out_waste();
    assert!(storage.to_draw_multi_instanced().is_empty());
    assert_eq!(
        render
            .sparse_draw_view::<Instance>()
            .unwrap()
            .allocation_to_draw_multi_instanced(&extracted_allocation),
        [(0, 35)]
    );

    let mut graph = RenderGraph::with_backend(Graph);
    let imported = render.get_rg_buffer::<Instance>(&mut graph);
    assert_eq!(retained.get_rg_buffer::<Instance>(&mut graph), imported);
    render.export_from_rg(&mut graph);
    drop(game);
    assert_eq!(device.released.load(Ordering::SeqCst), 0);
    drop(render);
    assert_eq!(device.released.load(Ordering::SeqCst), 0);
    assert_eq!(
        retained
            .sparse_draw_view::<Instance>()
            .unwrap()
            .to_draw_multi_instanced(),
        [(0, 35)]
    );
    drop(retained);
    assert_eq!(device.released.load(Ordering::SeqCst), 1);
}

#[test]
fn concurrent_highlight_filters_leave_snapshot_visibility_unchanged() {
    let device = Device::default();
    let mut game = AssetSsboLibrary::new();
    game.add_new::<Instance>(&device);
    let storage = game.get_mut::<Instance>().unwrap();
    let first = storage.new_allocation();
    let second = storage.new_allocation();
    let hidden = storage.new_allocation();
    storage
        .writer(&first)
        .replace_all(&[Instance(1); 32])
        .unwrap();
    storage
        .writer(&second)
        .replace_all(&[Instance(2); 35])
        .unwrap();
    storage
        .writer(&hidden)
        .replace_all(&[Instance(3); 2])
        .unwrap();
    storage.hide(&hidden);
    let render = game.clone();
    let view = render.sparse_draw_view::<Instance>().unwrap();
    let before = view.to_draw_multi_instanced();
    assert_eq!(before, [(0, 67)]);
    std::thread::scope(|scope| {
        let a = scope.spawn(|| view.to_draw_multi_instanced_excluding(&first));
        let b = scope.spawn(|| view.to_draw_multi_instanced_excluding(&second));
        let c = scope.spawn(|| view.to_draw_multi_instanced_excluding(&hidden));
        assert_eq!(a.join().unwrap(), [(32, 35)]);
        assert_eq!(b.join().unwrap(), [(0, 32)]);
        assert_eq!(c.join().unwrap(), before);
    });
    assert_eq!(view.to_draw_multi_instanced(), before);
    assert_eq!(
        game.sparse_draw_view::<Instance>()
            .unwrap()
            .to_draw_multi_instanced(),
        before
    );
}

#[test]
fn raster_snapshots_share_both_buffers_and_detach_only_the_edited_one() {
    use country_core::resources::world_raster::{DirtyTilePublisher, WorldRaster};
    let mut game = WorldRaster::<512>::new(DirtyTilePublisher::unique());
    game.composite_mut()[0] = 17;
    game.scratch_mut()[0] = 29;
    let mut staging = vec![0; 512 * 512];
    game.dirty_mut().set_all();
    game.copy_to_staging(1, &mut staging);
    let mut render = game.clone();
    let retained = render.clone();
    assert!(std::ptr::eq(
        game.composite().as_ptr(),
        render.composite().as_ptr()
    ));
    assert!(std::ptr::eq(
        game.scratch().as_ptr(),
        render.scratch().as_ptr()
    ));
    assert_eq!(render.current_buffer().len(), 512 * 512);
    assert_eq!(render.current_buffer()[0], 29);

    game.scratch_mut()[0] = 71;
    assert!(!std::ptr::eq(
        game.scratch().as_ptr(),
        render.scratch().as_ptr()
    ));
    assert!(std::ptr::eq(
        game.composite().as_ptr(),
        render.composite().as_ptr()
    ));
    assert_eq!(render.current_buffer()[0], 29);
    game.commit_scratch_to_composite();
    assert!(std::ptr::eq(
        game.composite().as_ptr(),
        game.scratch().as_ptr()
    ));
    assert_eq!(game.composite()[0], 71);
    assert_eq!(render.composite()[0], 17);
    game.copy_composite_to_scratch();
    game.scratch_mut()[0] = 93;
    assert_eq!(game.composite()[0], 71);

    // A render snapshot may update its flags/hash bookkeeping independently.
    // Its staging scratch is valid, and it must not publish to game subscribers.
    let mut subscriber = game.subscribe();
    let old_hash = retained.tile_hash(0);
    render.scratch_mut()[0] = 51;
    render.dirty_mut().set_all();
    staging.fill(0);
    render.copy_to_staging(1, &mut staging);
    assert_eq!(staging[0], 51);
    assert_ne!(render.tile_hash(0), old_hash);
    assert_eq!(retained.tile_hash(0), old_hash);
    assert_eq!(retained.current_buffer()[0], 29);
    assert!(subscriber.take().is_none());
    game.clear_all();
    assert!(game.composite().iter().all(|&v| v == 0));
    assert_eq!(retained.composite()[0], 17);
}

#[test]
fn shared_reference_ssbo_registration_is_extracted_without_an_ecs_tick_change() {
    use bevy_ecs::world::World;
    use country_core::render::extract::RenderExtract;
    use tiny_glade::render_thread::RenderExchange;
    let device = Device::default();
    let mut game = World::new();
    let mut render = World::new();
    game.init_resource::<AssetSsboLibrary>();
    let mut exchange = RenderExchange::default();
    // The production manifest must register the revision-aware extractor.
    tiny_glade::world_walls::render_extract(&mut exchange);
    exchange.asset_library::<AssetSsboLibrary>();
    exchange.extract.run(&mut game, &mut render);
    let source_tick = game
        .get_resource_change_ticks::<AssetSsboLibrary>()
        .unwrap()
        .changed;
    game.resource::<AssetSsboLibrary>()
        .add_new::<Instance>(&device);
    assert_eq!(
        game.get_resource_change_ticks::<AssetSsboLibrary>()
            .unwrap()
            .changed,
        source_tick
    );
    exchange.extract.run(&mut game, &mut render);
    assert!(
        render
            .resource::<AssetSsboLibrary>()
            .get_buffer::<Instance>()
            .is_some()
    );
    assert!(
        render
            .resource::<AssetSsboLibrary>()
            .sparse_draw_view::<Instance>()
            .is_some()
    );
    assert_eq!(exchange.extract.stats().resources_copied, 1);
    exchange.extract.run(&mut game, &mut render);
    assert_eq!(
        exchange.extract.stats().resources_copied,
        0,
        "unchanged library must reuse its snapshot"
    );
    game.remove_resource::<AssetSsboLibrary>();
    exchange.extract.run(&mut game, &mut render);
    assert!(!render.contains_resource::<AssetSsboLibrary>());
    game.init_resource::<AssetSsboLibrary>();
    exchange.extract.run(&mut game, &mut render);
    assert!(!render.resource::<AssetSsboLibrary>().contains::<Instance>());
}

#[test]
fn shared_reference_mesh_registration_is_extracted_and_old_asset_version_stays_alive() {
    use bevy_ecs::world::World;
    use country_core::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
    use country_core::render::mesh::Mesh;
    use tiny_glade::render_thread::RenderExchange;
    let mut game = World::new();
    let mut render = World::new();
    game.init_resource::<AssetMeshLibrary>();
    let mut exchange = RenderExchange::default();
    tiny_glade::render_bridge::render_extract(&mut exchange);
    exchange.extract.run(&mut game, &mut render);
    let before = game
        .get_resource_change_ticks::<AssetMeshLibrary>()
        .unwrap()
        .changed;
    let handle = game
        .resource::<AssetMeshLibrary>()
        .add(MeshAsset::new(Mesh::default(), Some("shared".into())));
    assert_eq!(
        game.get_resource_change_ticks::<AssetMeshLibrary>()
            .unwrap()
            .changed,
        before
    );
    exchange.extract.run(&mut game, &mut render);
    assert!(render.resource::<AssetMeshLibrary>().get(handle).is_some());
    exchange.extract.run(&mut game, &mut render);
    assert_eq!(exchange.extract.stats().resources_copied, 0);
    let retained = render.resource::<AssetMeshLibrary>().clone();
    game.resource_mut::<AssetMeshLibrary>().remove(handle);
    exchange.extract.run(&mut game, &mut render);
    assert!(render.resource::<AssetMeshLibrary>().get(handle).is_none());
    assert!(retained.get(handle).is_some());
}

#[test]
fn production_raster_projection_keeps_pixels_exclusively_on_the_producer() {
    use country_core::resources::{world_raster::DirtyTilePublisher, world_raster_defs::PathRaster};
    let mut game = bevy_ecs::world::World::new();
    let mut render = bevy_ecs::world::World::new();
    let mut raster = PathRaster::new(DirtyTilePublisher::unique());
    raster.composite_mut()[0] = 11;
    raster.dirty_mut().set_cell(0, 0);
    let address = raster.composite().as_ptr();
    game.insert_resource(raster);
    let mut exchange = tiny_glade::render_extract::production_exchange();
    exchange.extract.run(&mut game, &mut render);
    assert_eq!(render.resource::<PathRaster>().retained_cpu_bytes(), 0);
    assert!(render.resource::<PathRaster>().dirty().contains(0, 0));
    game.resource_mut::<PathRaster>().composite_mut()[0] = 27;
    assert_eq!(game.resource::<PathRaster>().composite().as_ptr(), address);
    assert_eq!(game.resource::<PathRaster>().composite()[0], 27);
    render.resource_mut::<PathRaster>().consume_reupload();
    assert!(game.resource::<PathRaster>().dirty().contains(0, 0));
}

#[test]
fn sparse_visibility_shared_mutation_reaches_incremental_extraction_without_ecs_change() {
    use bevy_ecs::world::World;
    let device = Device::default();
    let mut source = AssetSsboLibrary::new();
    source.add_new::<Instance>(&device);
    let storage = source.get_mut::<Instance>().unwrap();
    let allocation = storage.new_allocation();
    storage.writer(&allocation).replace_all(&[Instance(7); 3]).unwrap();
    let mut game = World::new();
    game.insert_resource(source);
    let mut render = World::new();
    let mut exchange = tiny_glade::render_extract::production_exchange();
    exchange.extract.run(&mut game, &mut render);
    let retained = render.resource::<AssetSsboLibrary>().clone();
    let source_tick = game.get_resource_change_ticks::<AssetSsboLibrary>().unwrap().changed;
    game.resource::<AssetSsboLibrary>().get::<Instance>().unwrap().hide(&allocation);
    assert_eq!(game.get_resource_change_ticks::<AssetSsboLibrary>().unwrap().changed, source_tick);
    exchange.extract.run(&mut game, &mut render);
    assert!(render.resource::<AssetSsboLibrary>().sparse_draw_view::<Instance>().unwrap()
        .to_draw_multi_instanced().is_empty());
    assert_eq!(retained.sparse_draw_view::<Instance>().unwrap().to_draw_multi_instanced(), [(0, 3)]);
    game.resource::<AssetSsboLibrary>().get::<Instance>().unwrap().show(&allocation);
    exchange.extract.run(&mut game, &mut render);
    assert_eq!(render.resource::<AssetSsboLibrary>().sparse_draw_view::<Instance>().unwrap()
        .to_draw_multi_instanced(), [(0, 3)]);
    let revision = game.resource::<AssetSsboLibrary>().snapshot_revision();
    game.resource::<AssetSsboLibrary>().get::<Instance>().unwrap().show(&allocation);
    assert_eq!(game.resource::<AssetSsboLibrary>().snapshot_revision(), revision,
        "an already-visible allocation must not enqueue another snapshot update");
}

#[test]
fn incremental_extract_rebuilds_for_new_worlds_and_retains_unchanged_entry_allocations() {
    use bevy_ecs::world::World;
    use country_core::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
    use country_core::render::mesh::Mesh;
    let mut game = World::new();
    game.init_resource::<AssetMeshLibrary>();
    let a = game.resource::<AssetMeshLibrary>().add(MeshAsset::new(Mesh::new("a"), Some("a".into())));
    let b = game.resource::<AssetMeshLibrary>().add(MeshAsset::new(Mesh::new("b"), Some("b".into())));
    let mut render = World::new();
    let mut exchange = tiny_glade::render_extract::production_exchange();
    exchange.extract.run(&mut game, &mut render);
    let unchanged = render.resource::<AssetMeshLibrary>().get(b).unwrap() as *const Mesh;
    game.resource_mut::<AssetMeshLibrary>().get_mut(a).unwrap().label = "edited".into();
    exchange.extract.run(&mut game, &mut render);
    assert_eq!(render.resource::<AssetMeshLibrary>().get(a).unwrap().label, "edited");
    assert_eq!(render.resource::<AssetMeshLibrary>().get(b).unwrap() as *const Mesh, unchanged);
    let mut new_render = World::new();
    exchange.extract.run(&mut game, &mut new_render);
    assert_eq!(new_render.resource::<AssetMeshLibrary>().get(a).unwrap().label, "edited");
    let mut new_game = World::new();
    new_game.init_resource::<AssetMeshLibrary>();
    exchange.extract.run(&mut new_game, &mut new_render);
    assert!(new_render.resource::<AssetMeshLibrary>().is_empty());
    assert!(new_render.resource::<AssetMeshLibrary>().try_get_handle_by_name("a").is_none());
}

#[test]
fn production_identity_journals_skip_unchanged_rows_and_cover_replacement_remove_and_despawn() {
    use bevy_ecs::world::World;
    use country_core::{asset_libraries::Handle, render::{mesh::Mesh, pass_builder::draw_list::DrawTopology}};
    let mut game = World::new();
    let mut render = World::new();
    let mut exchange = tiny_glade::render_extract::production_exchange();
    let entities: Vec<_> = (0..1000).map(|_| game.spawn((Handle::<Mesh>::new(), DrawTopology::Default(1.0))).id()).collect();
    exchange.extract.run(&mut game, &mut render);
    assert_eq!(exchange.extract.stats().component_rows_scanned, 2000);
    exchange.extract.run(&mut game, &mut render);
    assert_eq!(exchange.extract.stats().component_rows_scanned, 0,
        "unchanged immutable identities must not run a Changed query over all rows");
    let replacement = Handle::<Mesh>::new();
    game.entity_mut(entities[0]).insert((replacement, DrawTopology::Alt(3.0)));
    game.entity_mut(entities[1]).remove::<Handle<Mesh>>();
    game.despawn(entities[2]);
    exchange.extract.run(&mut game, &mut render);
    assert_eq!(exchange.extract.stats().component_rows_scanned, 0);
    assert_eq!(exchange.extract.stats().components_copied, 2);
    assert_eq!(render.get::<Handle<Mesh>>(entities[0]), Some(&replacement));
    assert_eq!(render.get::<DrawTopology>(entities[0]), Some(&DrawTopology::Alt(3.0)));
    assert!(render.get::<Handle<Mesh>>(entities[1]).is_none());
    assert!(render.get::<DrawTopology>(entities[1]).is_some());
    assert!(!render.entities().contains(entities[2]));
    game.entity_mut(entities[1]).insert(replacement);
    exchange.extract.run(&mut game, &mut render);
    assert_eq!(render.get::<Handle<Mesh>>(entities[1]), Some(&replacement));
    assert_eq!(exchange.extract.stats().component_rows_scanned, 0);
}

#[test]
fn identity_journal_gap_reconciles_deleted_rows_and_world_changes() {
    use bevy_ecs::world::World;
    use country_core::{asset_libraries::Handle, render::mesh::Mesh};
    let mut game = World::new();
    let mut render = World::new();
    let mut exchange = tiny_glade::render_extract::production_exchange();
    let removed = game.spawn(Handle::<Mesh>::new()).id();
    let live = game.spawn(Handle::<Mesh>::new()).id();
    exchange.extract.run(&mut game, &mut render);
    game.despawn(removed);
    for _ in 0..5000 { game.entity_mut(live).insert(Handle::<Mesh>::new()); }
    exchange.extract.run(&mut game, &mut render);
    assert!(!render.entities().contains(removed));
    assert_eq!(render.get::<Handle<Mesh>>(live), game.get::<Handle<Mesh>>(live));
    assert_eq!(exchange.extract.stats().component_rows_scanned, 1);
    assert_eq!(exchange.extract.stats().component_removal_rescans, 1);
    let mut new_render = World::new();
    exchange.extract.run(&mut game, &mut new_render);
    assert_eq!(new_render.get::<Handle<Mesh>>(live), game.get::<Handle<Mesh>>(live));
    let mut new_game = World::new();
    exchange.extract.run(&mut new_game, &mut new_render);
    assert!(!new_render.entities().contains(live));
}
