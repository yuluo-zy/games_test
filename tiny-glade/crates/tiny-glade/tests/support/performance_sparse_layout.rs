//! Explicit common-fixture preparation, once after fresh boot and BEFORE the
//! first gameplay graph/warm-up. Never call from a production frame or retry a
//! mutated/failed fixture. Source-audited fixture call order is part of proof;
//! arbitrary escaped raw integer offsets cannot be discovered by a Rust API.
use anyhow::{ensure, Context, Result};
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::ssbo_library::{AssetSsboLibrary, SsboId},
    geometry::instanced_wall::{BrickTransformSsbo, InstancedWall},
    render::sparse_instance_buffer::{SparseInstanceBuffer, SparseInstanceBufferAllocation, SparseSsbo, SLOTS_PER_CHUNK},
    resources::walls::public_walls::PublicWalls,
};
use serde::Serialize;
use crate::test_support::BridgeTestDriver as _;
use tiny_glade::render_bridge::{CompletedGameplayWarmup, CoreRenderBridge};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct OwnerKey { wall: u64, lane: u8 }
struct Owner<'a> {
    key: OwnerKey,
    allocation: &'a SparseInstanceBufferAllocation<BrickTransformSsbo>,
}
struct Segment { used: usize, data: Vec<BrickTransformSsbo> }
struct OwnerImage { key: OwnerKey, visible: bool, segments: Vec<Segment> }
#[derive(bevy_ecs::resource::Resource)]
struct CanonicalLayoutAttempted;

#[derive(Serialize)]
pub struct CanonicalBrickLayout {
    pub schema: &'static str,
    pub phase: &'static str,
    pub owner_count: usize,
    pub chunk_count: usize,
    pub used_records: usize,
    pub owner_record_bytes_blake3: String,
    pub owner_full_chunk_bytes_blake3: String,
    pub physical_layout_blake3: String,
    pub free_stack_blake3: String,
    pub normal_ranges: Vec<(u32, u32)>,
    pub raw_index_escape_scope: &'static str,
}

fn active(storage: &SparseInstanceBuffer<BrickTransformSsbo>, chunk: usize) -> bool {
    storage.active_word(chunk / usize::BITS as usize) & (1usize << (chunk % usize::BITS as usize)) != 0
}

fn collect_owners(world: &World) -> Result<Vec<Owner<'_>>> {
    let mut owners = Vec::new();
    let public = world.get_resource::<PublicWalls>().context("PublicWalls absent before canonical preparation")?;
    let mut wall_ids: Vec<_> = public.wall_ids().collect();
    wall_ids.sort_by_key(|id| id.0);
    let constructors = world.get_resource::<system_wall_constructor::construct_arches::WallConstructorOutput>();
    for row in world.iter_entities() {
        if let Some(wall) = row.get::<InstancedWall<BrickTransformSsbo>>() {
            ensure!(!wall.is_upload_pending() && !wall.is_awaiting_production(), "wall {} is not settled", wall.id);
            ensure!(wall_ids.binary_search_by_key(&wall.id, |id| id.0).is_ok(), "unpublished wall owner {}", wall.id);
            for (lane, allocation) in [(0, wall.allocation()), (1, wall.chimney_allocation()), (2, wall.door_stairs_allocation())] {
                if let Some(allocation) = allocation { owners.push(Owner { key: OwnerKey { wall: wall.id, lane }, allocation }); }
            }
        }
    }
    if let Some(constructors) = constructors {
        for id in wall_ids {
            // This existing lookup is keyed only by wall. Arches/Gates share
            // the same REAL owner; do not treat the two ConstructorPart values
            // as two allocations or duplicate their records.
            let key = system_wall_constructor::gpu_output::ConstructorKey {
                wall: id.0, part: system_wall_constructor::gpu_output::ConstructorPart::Arches,
            };
            if let Some(allocation) = constructors.gpu.brick_allocation(key) {
                owners.push(Owner { key: OwnerKey { wall: id.0, lane: 3 }, allocation });
            }
        }
    }
    owners.sort_by_key(|owner| owner.key);
    ensure!(owners.windows(2).all(|pair| pair[0].key != pair[1].key), "duplicate semantic brick owner");
    ensure!(!owners.is_empty(), "no real brick owners for canonical preparation");
    Ok(owners)
}

/// Build the COMPLETE ownership/space proof and copy every physical 32-record
/// segment before the first clear. Hidden owners and unused tail bytes count.
fn capture(storage: &SparseInstanceBuffer<BrickTransformSsbo>, owners: &[Owner<'_>]) -> Result<Vec<OwnerImage>> {
    let chunks = <BrickTransformSsbo as SparseSsbo>::CHUNK_COUNT;
    ensure!(storage.byte_map.len() == chunks + 1 && storage.data.len() == chunks * SLOTS_PER_CHUNK,
        "unexpected sparse capacity");
    ensure!(storage.byte_map[chunks] as usize == SLOTS_PER_CHUNK, "bad sparse sentinel");
    ensure!(storage.waste.lock().is_empty(), "pending waste must settle through the original boot schedule");
    let references = storage.fixture_reference_state();
    ensure!(!references.cached_draw_snapshot && references.draw_snapshot_strong == 0 && references.draw_snapshot_weak == 0,
        "a sparse render wrapper/view was already published: {references:?}");
    // Original init_populate_ssbo_library uses SsboType::new_storage_buffer ->
    // try_new, whose actual device owner is BufferHost, not OwnedSparseBuffer.
    // An absent optional owner is permitted only alongside the caller's exact
    // live host-target/reference proof; never replace it with a fabricated Arc.
    ensure!(references.owned_buffer_strong == usize::from(references.owned_buffer_present)
            && references.owned_buffer_weak == 0,
        "optional native sparse owner escaped: {references:?}");
    let mut partition = vec![false; chunks];
    {
        let free = storage.free_chunks.read();
        ensure!(free.capacity() >= chunks, "free stack cannot replay without allocation");
        for &chunk in free.iter() {
            let chunk = chunk as usize;
            ensure!(chunk < chunks && !partition[chunk], "invalid/duplicate free chunk");
            ensure!(storage.byte_map[chunk] == 0 && !active(storage, chunk), "free chunk is live");
            partition[chunk] = true;
        }
    }
    let mut images = Vec::new();
    images.try_reserve_exact(owners.len())?;
    for owner in owners {
        ensure!(storage.owns_allocation(owner.allocation), "foreign or non-owning extracted allocation {:?}", owner.key);
        let allocation = owner.allocation.handle.0.as_ref().context("null allocation")?;
        let ids = allocation.chunk_ids.lock();
        ensure!(ids.capacity() >= ids.len(), "invalid owner capacity");
        let mut segments = Vec::new();
        segments.try_reserve_exact(ids.len())?;
        let visible = ids.first().is_some_and(|&chunk| active(storage, chunk as usize));
        for &chunk in ids.iter() {
            let chunk = chunk as usize;
            ensure!(chunk < chunks && !partition[chunk], "chunk aliases another owner/free slot");
            let used = storage.byte_map[chunk] as usize;
            ensure!((1..=SLOTS_PER_CHUNK).contains(&used), "unsupported allocated zero-length chunk");
            ensure!(active(storage, chunk) == visible, "mixed per-chunk visibility is unsupported, cannot flatten it");
            let mut data = Vec::new();
            data.try_reserve_exact(SLOTS_PER_CHUNK)?;
            data.extend_from_slice(&storage.data[chunk * SLOTS_PER_CHUNK..(chunk + 1) * SLOTS_PER_CHUNK]);
            segments.push(Segment { used, data });
            partition[chunk] = true;
        }
        images.push(OwnerImage { key: owner.key, visible, segments });
    }
    ensure!(partition.iter().all(|covered| *covered), "unknown occupied brick owner; refusing partial normalization");
    ensure!(storage.active_words().enumerate().all(|(i, word)| {
        let first = i * usize::BITS as usize;
        first + usize::BITS as usize <= chunks || word >> (chunks - first) == 0
    }), "active bits exceed typed capacity");
    Ok(images)
}

fn image_hash(images: &[OwnerImage], full: bool) -> String {
    let mut hash = blake3::Hasher::new();
    hash.update(if full { b"brick-layout-owner-full-chunks-v1\0" } else { b"brick-layout-owner-used-records-v1\0" });
    for owner in images {
        hash.update(&owner.key.wall.to_le_bytes()); hash.update(&[owner.key.lane, owner.visible as u8]);
        hash.update(&(owner.segments.len() as u64).to_le_bytes());
        for segment in &owner.segments {
            hash.update(&(segment.used as u64).to_le_bytes());
            for record in &segment.data[..if full { SLOTS_PER_CHUNK } else { segment.used }] {
                hash.update(&record.observed_bytes());
            }
        }
    }
    hash.finalize().to_hex().to_string()
}

fn replay(storage: &mut SparseInstanceBuffer<BrickTransformSsbo>, owners: &[Owner<'_>], images: &[OwnerImage]) -> Result<CanonicalBrickLayout> {
    let used_hash = image_hash(images, false);
    let full_hash = image_hash(images, true);
    // All fallible snapshots, complete ownership partition checks and
    // replay capacity checks completed above. No explicit fallible reserve or
    // device operation occurs in the mutation loop. Existing hide/notifier
    // side effects remain; this is not an allocation-free production path. Existing Vec capacity
    // holds the SAME number of original chunks, including partial chunks.
    for owner in owners { storage.writer(owner.allocation).clear(); }
    storage.free_chunks.write().sort_unstable_by(|a, b| b.cmp(a));
    for (owner, image) in owners.iter().zip(images) {
        for segment in &image.segments {
            // A NEW writer begins at the full sentinel. Reopening it for
            // each original segment preserves partial boundaries; using
            // replace_all/append_writer here would compact the workload.
            let mut writer = storage.writer(owner.allocation);
            for record in &segment.data[..segment.used] {
                writer.push(record).expect("complete space proof guarantees unchanged chunk count");
            }
        }
        let ids = owner.allocation.handle.0.as_ref().expect("preflight nonnull").chunk_ids.lock();
        for (&chunk, segment) in ids.iter().zip(&image.segments) {
            let start = chunk as usize * SLOTS_PER_CHUNK;
            storage.data[start..start + SLOTS_PER_CHUNK].copy_from_slice(&segment.data);
        }
        drop(ids);
        if !image.visible { storage.hide(owner.allocation); }
    }
    // Recheck bytes/counts/visibility and the complete partition. Any
    // failure aborts this fixture; it is never a successful partial replay.
    let after = capture(storage, owners)?;
    ensure!(image_hash(&after, false) == used_hash && image_hash(&after, true) == full_hash,
        "owner/chunk data changed during preparation");
    let normal_ranges = storage.to_draw_multi_instanced();
    let mut physical = blake3::Hasher::new();
    physical.update(b"canonical-brick-physical-v1\0");
    physical.update(&(owners.len() as u64).to_le_bytes());
    for (owner, image) in owners.iter().zip(&after) {
        physical.update(&owner.key.wall.to_le_bytes()); physical.update(&[owner.key.lane, image.visible as u8]);
        physical.update(&(image.segments.len() as u64).to_le_bytes());
        let ids = owner.allocation.handle.0.as_ref().unwrap().chunk_ids.lock();
        for (&chunk, segment) in ids.iter().zip(&image.segments) {
            physical.update(&chunk.to_le_bytes()); physical.update(&(segment.used as u64).to_le_bytes());
            for record in &segment.data { physical.update(&record.observed_bytes()); }
        }
    }
    let mut free_hash = blake3::Hasher::new();
    free_hash.update(b"canonical-brick-free-stack-v1\0");
    {
        let free = storage.free_chunks.read();
        free_hash.update(&(free.len() as u64).to_le_bytes());
        for id in free.iter() { free_hash.update(&id.to_le_bytes()); }
    }
    // Real original transfer path, before the first gameplay graph. These
    // setup uploads may differ from the old noncanonical boot and are NOT
    // charged as optimization gains. Any refusal aborts the fixture.
    Ok(CanonicalBrickLayout {
        schema: "canonical-brick-layout-v1", phase: "fresh_boot_before_first_gameplay_warmup",
        owner_count: owners.len(), chunk_count: after.iter().map(|owner| owner.segments.len()).sum(),
        used_records: after.iter().flat_map(|owner| &owner.segments).map(|segment| segment.used).sum(),
        owner_record_bytes_blake3: used_hash, owner_full_chunk_bytes_blake3: full_hash,
        physical_layout_blake3: physical.finalize().to_hex().to_string(),
        free_stack_blake3: free_hash.finalize().to_hex().to_string(), normal_ranges,
        raw_index_escape_scope: "Known wrappers/imports/owners checked; arbitrary untracked integers require audited fresh-boot call order. Not a public defragmenter.",
    })
}

/// Only the audited fresh-boot CPU fixture may invoke this function. The
/// preparation is mandatory in BOTH new common binaries, including feature-off;
/// never enable only the candidate or select runs by resulting range count.
pub fn before_first_gameplay_warmup(world: &mut World, bridge: &mut CoreRenderBridge) -> Result<CanonicalBrickLayout> {
    ensure!(!world.contains_resource::<CanonicalLayoutAttempted>(), "canonical fixture preparation is once-only; never retry a failed fixture");
    ensure!(!world.contains_resource::<CompletedGameplayWarmup>(), "gameplay GPU warm-up already completed");
    drop(bridge.warmup_owners()?); // Existing guard rejects named/pending graph owners.
    bridge.with_session(|session| {
        ensure!(session.pending_frame_count() == 0 && session.named_resource_count() == 0, "live gameplay frame/history");
        Ok(())
    })?;
    world.insert_resource(CanonicalLayoutAttempted);
    // Boot may already have queued/submitted native uploads. They are real GPU
    // work, not gameplay visibility/index consumers. Drain those owners before
    // proving that no physical read wrapper remains. This is outside warmup
    // observations and outside the measured CPU envelope in BOTH binaries.
    bridge.prepare_test_execution()?;
    bridge.hosts().buffers.graphics_device().wait_idle_and_collect()?;
    ensure!(bridge.hosts().buffers.queued_upload_stats().is_none_or(|stats| stats.queued_jobs == 0), "boot upload jobs remain");
    let library = world.get_resource::<AssetSsboLibrary>().context("SSBO library missing")?;
    let leases = library.snapshot_graph_leases();
    let lease = leases.iter().find(|lease| lease.id == SsboId::of::<BrickTransformSsbo>()).context("brick graph lease absent")?;
    ensure!(lease.rg_handle().is_none(), "brick indices already imported into a graph");
    let target = *library.get_buffer::<BrickTransformSsbo>().context("brick buffer absent")?;
    let host_state = bridge.hosts().buffers.fixture_reference_state(&target)?;
    ensure!(!host_state.frame_bound && !host_state.active_read_lease && host_state.version_strong == 1 && host_state.version_weak == 0
            && host_state.snapshot_pins == 0 && host_state.physical_strong == 1
            && host_state.known_queue_history_weak.checked_add(host_state.known_cow_retired_weak) == Some(host_state.physical_weak),
        "retained native/frame view exists: {host_state:?}");
    drop(leases);
    let buffers = bridge.hosts().buffers.clone();
    let result = world.resource_scope(|world, mut library: bevy_ecs::change_detection::Mut<AssetSsboLibrary>| -> Result<CanonicalBrickLayout> {
        let owners = collect_owners(world)?;
        let storage = library.get_mut::<BrickTransformSsbo>().context("actual writable brick storage absent")?;
        let images = capture(storage, &owners)?;
        let report = replay(storage, &owners, &images)?;
        storage.gpu_flush(buffers.as_ref())?;
        Ok(report)
    })?;
    bridge.prepare_test_execution()?;
    buffers.graphics_device().wait_idle_and_collect()?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    fn storage() -> SparseInstanceBuffer<BrickTransformSsbo> {
        SparseInstanceBuffer::try_new_owned(Arc::new(country_core::render::test_device::TestDevice(1))).unwrap()
    }
    fn fill(storage: &mut SparseInstanceBuffer<BrickTransformSsbo>, allocation: &SparseInstanceBufferAllocation<BrickTransformSsbo>, seed: u32, counts: &[usize]) {
        for (segment, &count) in counts.iter().enumerate() {
            let mut writer = storage.writer(allocation);
            for i in 0..count {
                writer.push(&BrickTransformSsbo { seed: seed + (segment * 32 + i) as u32, ..Default::default() }).unwrap();
            }
        }
        let ids = allocation.handle.0.as_ref().unwrap().chunk_ids.lock();
        for (segment, (&id, &used)) in ids.iter().zip(counts).enumerate() {
            for i in used..32 {
                storage.data[id as usize * 32 + i] = BrickTransformSsbo {
                    seed: 0xf0000000 | seed | (segment as u32 * 32 + i as u32), ..Default::default()
                };
            }
        }
    }
    #[test]
    fn opposing_real_allocation_orders_replay_identical_segments_full_bytes_and_free_stack() {
        fn once(reverse: bool) -> CanonicalBrickLayout {
            let mut s = storage(); let a = s.new_allocation(); let b = s.new_allocation();
            if reverse { fill(&mut s, &b, 1024, &[32, 1]); fill(&mut s, &a, 64, &[3, 4]); }
            else { fill(&mut s, &a, 64, &[3, 4]); fill(&mut s, &b, 1024, &[32, 1]); }
            s.hide(&b);
            let owners = [Owner { key: OwnerKey { wall: 1, lane: 0 }, allocation: &a }, Owner { key: OwnerKey { wall: 2, lane: 0 }, allocation: &b }];
            let captured = capture(&s, &owners).unwrap();
            let result = replay(&mut s, &owners, &captured).unwrap();
            assert_eq!(result.chunk_count, 4); assert_eq!(result.used_records, 40);
            assert_eq!(s.allocation_to_draw_multi_instanced(&a), vec![(0, 3), (32, 4)]);
            assert_eq!(s.active_word(0), 3, "hidden owner must remain hidden after relocation");
            result
        }
        let a = once(false); let b = once(true);
        assert_eq!(a.owner_record_bytes_blake3, b.owner_record_bytes_blake3);
        assert_eq!(a.owner_full_chunk_bytes_blake3, b.owner_full_chunk_bytes_blake3);
        assert_eq!(a.physical_layout_blake3, b.physical_layout_blake3);
        assert_eq!(a.free_stack_blake3, b.free_stack_blake3);
        assert_eq!(a.normal_ranges, b.normal_ranges);
    }
    #[test]
    fn unknown_owner_and_mixed_visibility_refuse_before_clear() {
        let mut s = storage(); let a = s.new_allocation(); let unknown = s.new_allocation();
        fill(&mut s, &a, 1, &[1]); fill(&mut s, &unknown, 2, &[1]);
        let owners = [Owner { key: OwnerKey { wall: 1, lane: 0 }, allocation: &a }];
        let before = s.to_draw_multi_instanced();
        assert!(capture(&s, &owners).is_err()); assert_eq!(s.to_draw_multi_instanced(), before);
        s.writer(&unknown).clear(); s.hide(&a); s.writer(&a).push(&BrickTransformSsbo::default()).unwrap();
        let before = s.to_draw_multi_instanced();
        assert!(capture(&s, &owners).is_err()); assert_eq!(s.to_draw_multi_instanced(), before);
    }
    #[test]
    fn retained_and_ever_published_draw_wrapper_refuse_before_clear() {
        use country_core::asset_libraries::ssbo_library::Ssbo;
        let mut s = storage(); let a = s.new_allocation(); fill(&mut s, &a, 1, &[1]);
        let owners = [Owner { key: OwnerKey { wall: 1, lane: 0 }, allocation: &a }];
        let snapshot = s.snapshot();
        assert!(capture(&s, &owners).is_err());
        drop(snapshot);
        assert!(capture(&s, &owners).is_err(), "a previous publication cannot be declared fresh by dropping its last wrapper");
    }
    #[test]
    fn full_chunk_signature_includes_unused_tail_bytes() {
        let mut images = vec![OwnerImage { key: OwnerKey { wall: 1, lane: 0 }, visible: true,
            segments: vec![Segment { used: 1, data: vec![BrickTransformSsbo::default(); 32] }] }];
        let used = image_hash(&images, false); let full = image_hash(&images, true);
        images[0].segments[0].data[31].seed = 17;
        assert_eq!(image_hash(&images, false), used);
        assert_ne!(image_hash(&images, true), full);
    }
    #[test]
    fn original_borrowed_storage_reports_absent_optional_owner_without_fabricating_one() {
        let mut s = SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&country_core::render::test_device::TestDevice(1)).unwrap();
        let a = s.new_allocation(); fill(&mut s, &a, 1, &[1]);
        let refs = s.fixture_reference_state();
        assert!(!refs.owned_buffer_present);
        assert_eq!((refs.owned_buffer_strong, refs.owned_buffer_weak), (0, 0));
        let owners = [Owner { key: OwnerKey { wall: 1, lane: 0 }, allocation: &a }];
        assert!(capture(&s, &owners).is_ok());
        // This is only the sparse-side CPU proof. The public fixture entry
        // still requires the matching live BufferHost target and references.
    }
}
