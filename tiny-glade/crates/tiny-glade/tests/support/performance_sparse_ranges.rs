//! Diagnostic-only source inputs for the existing wall dispatch callbacks.
//! Keep this scan outside clean CPU measurements. It neither mutates the World
//! nor canonicalizes allocations: actual (first,count) boundaries are inputs.
use anyhow::{Context, Result};
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    geometry::instanced_wall::{BrickTransformSsbo, InstancedWall},
    render::{sparse_instance_buffer::DrawRange, ssbo::StorageElement},
    resources::walls::WallHighlight,
};

/// Store typed rows during the diagnostic; serialize only when reporting.
#[derive(serde::Serialize)]
pub struct WallDispatchSnapshot {
    pub schema: &'static str,
    pub phase: &'static str,
    pub source: &'static str,
    pub capacity_elements: usize,
    pub highlight_type: u8,
    pub highlighted_allocation_selected: bool,
    pub normal: Vec<[u32; 2]>,
    pub highlighted: Vec<[u32; 2]>,
    pub normal_range_count: usize,
    pub highlighted_range_count: usize,
    pub normal_instance_count: u64,
    pub highlighted_instance_count: u64,
    pub callback_dispatches_if_invoked: usize,
    pub native_dispatches_observed: bool,
    pub raw_ranges_blake3: [u8; 32],
    pub raw_ranges_blake3_encoding: &'static str,
    pub raw_encoding: &'static str,
    pub identity_scope: &'static str,
}

fn encode_ranges(normal: &[DrawRange], highlighted: &[DrawRange]) -> Vec<u8> {
    let mut bytes = b"wall-dispatch-source-ranges-v1\0".to_vec();
    for ranges in [normal, highlighted] {
        bytes.extend((ranges.len() as u64).to_le_bytes());
        for &(first, count) in ranges {
            bytes.extend(first.to_le_bytes());
            bytes.extend(count.to_le_bytes());
        }
    }
    bytes
}

fn range_summary(ranges: &[DrawRange], capacity: usize) -> Result<(u64, Vec<[u32; 2]>)> {
    let mut instances = 0u64;
    let mut records = Vec::with_capacity(ranges.len());
    for &(first, count) in ranges {
        let end = u64::from(first) + u64::from(count);
        anyhow::ensure!(end <= capacity as u64,
            "wall range [{first},{end}) exceeds actual typed capacity {capacity}");
        instances = instances.checked_add(u64::from(count))
            .context("wall range instance total overflow")?;
        // Retain even a zero-count range: the callback would still issue a
        // dispatch for it. Never filter, sort, merge, or renumber this evidence.
        records.push([first, count]);
    }
    Ok((instances, records))
}

/// Mirror world_walls::batches' read-only selection exactly. The returned
/// counts describe source ranges, not independently measured native dispatches.
/// Call immediately after App::update and before prepare; compare to the
/// separate successful per-pass native workload diagnostic before attribution.
pub fn wall_dispatch_snapshot(world: &World) -> Result<WallDispatchSnapshot> {
    let storage = world.get_resource::<AssetSsboLibrary>()
        .context("wall source observation: AssetSsboLibrary missing")?
        .sparse_draw_view::<BrickTransformSsbo>()
        .context("wall source observation: BrickTransformSsbo view missing")?;
    let highlight = world.get_resource::<WallHighlight>()
        .context("wall source observation: WallHighlight missing")?;
    let highlighted_wall = (highlight.ty != 0).then_some(highlight.wall_entity).flatten()
        .and_then(|entity| world.get::<InstancedWall<BrickTransformSsbo>>(entity));
    let selected_allocation = highlighted_wall.is_some();
    let (normal, highlighted) = if let Some(wall) = highlighted_wall {
        let allocation = wall.allocation()
            .context("wall source observation: highlighted wall allocation missing")?;
        // Keep the same call and ordering as world_walls::batches.
        let highlighted = storage.allocation_to_draw_multi_instanced(allocation);
        let normal = storage.to_draw_multi_instanced_excluding(allocation);
        (normal, highlighted)
    } else {
        (storage.to_draw_multi_instanced(), Vec::new())
    };
    let capacity = BrickTransformSsbo::BUFFER_LEN;
    let (normal_instances, normal_records) = range_summary(&normal, capacity)?;
    let (highlighted_instances, highlighted_records) = range_summary(&highlighted, capacity)?;
    let raw = encode_ranges(&normal, &highlighted);
    Ok(WallDispatchSnapshot {
        schema: "wall-dispatch-source-ranges-v1",
        phase: "after_App_update_before_prepare",
        source: "BrickTransformSsbo SparseDrawView and actual WallHighlight selection",
        capacity_elements: capacity,
        highlight_type: highlight.ty,
        highlighted_allocation_selected: selected_allocation,
        normal: normal_records, highlighted: highlighted_records,
        normal_range_count: normal.len(), highlighted_range_count: highlighted.len(),
        normal_instance_count: normal_instances, highlighted_instance_count: highlighted_instances,
        callback_dispatches_if_invoked: normal.len() + highlighted.len(),
        native_dispatches_observed: false,
        raw_ranges_blake3: *blake3::hash(&raw).as_bytes(),
        raw_ranges_blake3_encoding: "32 raw digest bytes; hex formatting is report-time only",
        raw_encoding: "domain NUL, normal length u64 LE and (first u32 LE,count u32 LE) in original order, then highlighted length/pairs",
        identity_scope: "typed GPU element indices are preserved; no Entity ID, buffer handle, address, process or allocator identity is included",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_range_signature_preserves_fragmentation_order_and_highlight_role() {
        let fragmented = encode_ranges(&[(0, 32), (64, 1)], &[]);
        assert_ne!(fragmented, encode_ranges(&[(0, 33)], &[]));
        assert_ne!(fragmented, encode_ranges(&[(64, 1), (0, 32)], &[]));
        assert_ne!(fragmented, encode_ranges(&[], &[(0, 32), (64, 1)]));
    }

    #[test]
    fn source_range_encoding_uses_fixed_width_little_endian_words() {
        let encoded = encode_ranges(&[(0x01020304, 0x05060708)], &[]);
        let prefix = b"wall-dispatch-source-ranges-v1\0";
        assert_eq!(&encoded[..prefix.len()], prefix);
        assert_eq!(&encoded[prefix.len()..], &[
            1,0,0,0,0,0,0,0, 4,3,2,1,8,7,6,5, 0,0,0,0,0,0,0,0,
        ]);
    }

    #[test]
    fn source_range_limits_preserve_zero_and_reject_out_of_bounds() {
        assert_eq!(range_summary(&[(0, 32), (64, 1), (65, 0)], 65).unwrap().0, 33);
        assert!(range_summary(&[(64, 2)], 65).is_err());
        assert!(range_summary(&[(u32::MAX, 1)], 65).is_err());
        assert_eq!(range_summary(&[(65, 0)], 65).unwrap().1, vec![[65, 0]]);
    }
}
