//! Final-frame diagnostic only: retain exact receipt owners, then read after
//! the fixture's existing GPU drain. No copy commands, usage changes or added wait budget.
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType},
    render::nani::{
        buffers::{NaniDrawListLengthSsbo, NaniDrawListSsbo, NaniInstanceIndirectionSsbo},
        draw_ext::{
            nani_draw_plan, NaniIndirectMode, NANI_FORCE_FINE_GRAINED_SORTING,
            NANI_FORCE_NO_FINE_GRAINED_SORTING,
        },
        gpu_world::{GpuWorld, GpuWorldSubset, GpuWorldSubsetKind},
    },
};
use rhapsody::{
    resource::Buffer,
    rg::{GpuResource, SessionFrameReceipt},
};
use serde_json::{json, Value};
use std::sync::Arc;
use tiny_glade::render_bridge::CoreRenderBridge;

type ObservedOwner = Result<(Arc<Buffer>, Vec<u32>), String>;
const HEADER_WORDS: usize = 108;
const READ_TIMEOUT_NS: u64 = 0; // Already drained. Pending use means Unavailable.
const MAX_READ_BYTES: usize = 1 << 20;

struct FineList {
    kind: &'static str,
    list: usize,
    command_start: u64,
    command_end: u64,
    instance_start: u64,
    instance_end: u64,
    native_max_count: u64,
}

pub struct FinalNaniObservation {
    header: ObservedOwner,
    commands: ObservedOwner,
    instances: ObservedOwner,
    fine_lists: Result<Vec<FineList>, String>,
    initializer_retained: bool,
    generator_retained: bool,
    receipt_frame: u64,
}

fn retain_owner<T: SsboType>(
    world: &World,
    bridge: &mut CoreRenderBridge,
    receipt: &SessionFrameReceipt,
) -> ObservedOwner {
    let handle = *world
        .get_resource::<AssetSsboLibrary>()
        .ok_or("missing actual SSBO library")?
        .get_buffer::<T>()
        .ok_or("typed SSBO owner missing")?;
    let owner = bridge
        .hosts()
        .buffers
        .managed_buffer(&handle)
        .ok_or("typed handle no longer resolves through the live generation")?
        .shared();
    if owner.size != handle.byte_size {
        return Err("typed byte size differs from retained native owner".into());
    }
    // Resolve native tokens only within this real receipt; never assume a core
    // handle, resource index or numeric token is a Vulkan object identity.
    let logicals = bridge.with_session(|session| {
        receipt
            .snapshot
            .physical_mapping
            .iter()
            .filter_map(
                |mapping| match session.backend.resource_clone(mapping.token?) {
                    Some(GpuResource::Buffer(actual)) if Arc::ptr_eq(&actual, &owner) => {
                        Some(mapping.logical)
                    }
                    _ => None,
                },
            )
            .collect::<Vec<_>>()
    });
    if logicals.is_empty() {
        return Err("current typed owner is absent from final receipt mapping".into());
    }
    Ok((owner, logicals))
}

fn fine_lists(world: &World) -> Result<Vec<FineList>, String> {
    let gpu = world
        .get_resource::<GpuWorld>()
        .ok_or("actual GpuWorld missing")?;
    let parameters = gpu.draw_list_params();
    let mut lists = Vec::new();
    // Restrict this minimal observer to the two default fine-count families.
    // Raw/other/custom families have different bounds and remain unobserved.
    for (kind, name) in [
        (GpuWorldSubsetKind::Canopy, "Canopy"),
        (GpuWorldSubsetKind::TreeTrunk, "TreeTrunk"),
    ] {
        let slot = gpu.get_subset(kind);
        let Some(GpuWorldSubset::Owned(owned)) = slot.as_ref() else {
            continue;
        };
        let bound = owned.main_cull_command_bound();
        for variant in [0, 2, 3, 4] {
            let list =
                country_core::render::nani::draw_ext::nani_list_index(kind, variant) as usize;
            let plan = nani_draw_plan(
                kind,
                variant,
                gpu.list_index_to_cmd_offset(list),
                NANI_FORCE_FINE_GRAINED_SORTING,
                NANI_FORCE_NO_FINE_GRAINED_SORTING,
            );
            if plan.mode != NaniIndirectMode::Count {
                return Err("selected family is not the real count-consumer path".into());
            }
            lists.push(FineList {
                kind: name,
                list,
                command_start: plan.args_offset_bytes,
                command_end: u64::from(
                    parameters
                        .max_instance_offsets
                        .get(list + 1)
                        .copied()
                        .unwrap_or(parameters.total_max_instances),
                ) * 20,
                instance_start: u64::from(parameters.cmd_offsets[list]),
                instance_end: u64::from(
                    parameters
                        .cmd_offsets
                        .get(list + 1)
                        .copied()
                        .unwrap_or(parameters.total_cmd_offset),
                ),
                native_max_count: plan
                    .max_draw_count
                    .ok_or("count path missing actual limit")?
                    .min(bound),
            });
        }
    }
    Ok(lists)
}

/// Call once for frame30 after its successful execute, before owners can be
/// recycled. This retains no new GPU object and performs no native memory read.
pub fn retain_final(
    world: &World,
    bridge: &mut CoreRenderBridge,
    receipt: &SessionFrameReceipt,
) -> FinalNaniObservation {
    FinalNaniObservation {
        header: retain_owner::<NaniDrawListLengthSsbo>(world, bridge, receipt),
        commands: retain_owner::<NaniDrawListSsbo>(world, bridge, receipt),
        instances: retain_owner::<NaniInstanceIndirectionSsbo>(world, bridge, receipt),
        fine_lists: fine_lists(world),
        initializer_retained: receipt
            .snapshot
            .pass_order
            .iter()
            .any(|name| name == "Nani clear draw lists"),
        generator_retained: receipt
            .snapshot
            .pass_order
            .iter()
            .any(|name| name == "Nani generate draws"),
        receipt_frame: receipt.frame,
    }
}

fn read(owner: &ObservedOwner, offset: u64, length: usize) -> Result<Vec<u8>, String> {
    let (buffer, _) = owner.as_ref().map_err(Clone::clone)?;
    if !buffer
        .memory_properties()
        .contains(ash::vk::MemoryPropertyFlags::HOST_VISIBLE)
    {
        return Err("actual backing memory is not HOST_VISIBLE".into());
    }
    if length > MAX_READ_BYTES
        || offset
            .checked_add(length as u64)
            .is_none_or(|end| end > buffer.size)
    {
        return Err("diagnostic range exceeds native owner or fixed read cap".into());
    }
    let mut data = vec![0; length];
    // The existing range-tracked API checks ownership/completion, rejects
    // shared transient allocations, and invalidates non-coherent memory.
    // Zero timeout never extends the fixture's existing GPU waiting policy.
    buffer
        .read_bytes_timeout(offset, &mut data, READ_TIMEOUT_NS)
        .map_err(|error| format!("tracked native read unavailable: {error}"))?;
    Ok(data)
}

fn owner_metadata(owner: &ObservedOwner) -> Value {
    match owner {
        Ok((buffer, logicals)) => json!({"state":"RetainedFromActualFinalReceipt",
            "receipt_logical_resources":logicals,"bytes":buffer.size,
            "actual_memory_property_bits":buffer.memory_properties().as_raw(),
            "host_visible":buffer.memory_properties().contains(ash::vk::MemoryPropertyFlags::HOST_VISIBLE)}),
        Err(reason) => json!({"state":"Unavailable","reason":reason}),
    }
}

fn words(data: &[u8]) -> Vec<u32> {
    data.chunks_exact(4)
        .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
        .collect()
}

fn command_range(list: &FineList, reported: u32) -> Result<(u64, usize, u64), String> {
    let count = u64::from(reported).min(list.native_max_count);
    let bytes = count.checked_mul(20).ok_or("command byte count overflow")?;
    let end = list
        .command_start
        .checked_add(bytes)
        .ok_or("command range overflow")?;
    if end > list.command_end || bytes > MAX_READ_BYTES as u64 {
        return Err("effective native count exceeds its actual command-list slice".into());
    }
    Ok((list.command_start, bytes as usize, count))
}

fn instance_ranges(list: &FineList, commands: &[[u32; 5]]) -> Result<Vec<(u64, u64)>, String> {
    let mut ranges = Vec::new();
    for command in commands {
        if command[0] == 0 || command[1] == 0 {
            continue;
        }
        let first = u64::from(command[4]);
        let end = first
            .checked_add(u64::from(command[1]))
            .ok_or("instance range overflow")?;
        if first < list.instance_start || end > list.instance_end {
            return Err("draw references outside its actual instance-indirection slice".into());
        }
        ranges.push((first, end));
    }
    // Coalesce only CPU reads of already referenced storage. The output keeps
    // every command's original five words/order; no GPU draw is reordered.
    ranges.sort_unstable();
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }
    Ok(merged)
}

impl FinalNaniObservation {
    /// Call only after the fixture's already-existing wait_submits_done.
    pub fn read_after_existing_drain(self) -> Value {
        let mut result = json!({"schema":"surface-final-nani-observation-v1",
            "diagnostic_only":true,"frame":30,"receipt_frame":self.receipt_frame,
            "phase":"after_existing_wait_submits_done",
            "read_timeout_ns":READ_TIMEOUT_NS,"additional_gpu_commands":0,
            "owners":{"counts":owner_metadata(&self.header),"commands":owner_metadata(&self.commands),
                      "instance_indirection":owner_metadata(&self.instances)},
            "scope":"final state of two fine-count families only; not all indirect GPU work or proof of pixel causality",
            "unobserved":["raw/custom families", "earlier frames and first cull-round overwritten state", "TAA and ice GPU texture contents"],
            "lists":[]});
        let captured = (|| -> Result<Value, String> {
            if !self.initializer_retained {
                return Err("final plan lacks the actual 108-word initializer".into());
            }
            let headers = words(&read(&self.header, 0, HEADER_WORDS * 4)?);
            let lists = self.fine_lists.as_ref().map_err(Clone::clone)?;
            if !self.generator_retained {
                return Err("final plan lacks the fine command generator".into());
            }
            let mut records = Vec::new();
            for list in lists {
                let observed = (|| -> Result<Value, String> {
                    let (offset, bytes, count) = command_range(list, headers[list.list])?;
                    let raw = if bytes == 0 {
                        Vec::new()
                    } else {
                        read(&self.commands, offset, bytes)?
                    };
                    let commands: Vec<[u32; 5]> = raw
                        .chunks_exact(20)
                        .map(|bytes| words(bytes).try_into().unwrap())
                        .collect();
                    let mut instance_reads = Vec::new();
                    for (first, end) in instance_ranges(list, &commands)? {
                        let byte_count = usize::try_from((end - first) * 4)
                            .map_err(|_| "instance read size overflow")?;
                        let raw = read(&self.instances, first * 4, byte_count)?;
                        instance_reads.push(json!({"first":first,"count":end-first,
                            "words":words(&raw),"raw_blake3":blake3::hash(&raw).to_hex().to_string()}));
                    }
                    Ok(json!({"state":"Available","effective_native_count_if_consumed":count,
                        "command_byte_offset":offset,"command_words":commands,
                        "command_raw_blake3":blake3::hash(&raw).to_hex().to_string(),
                        "instance_reads":instance_reads}))
                })();
                records.push(json!({"kind":list.kind,"list_index":list.list,
                    "reported_gpu_count":headers[list.list],"native_count_limit":list.native_max_count,
                    "native_draw_invocation_observed":false,
                    "observation":match observed { Ok(value) => value,
                        Err(reason) => json!({"state":"Unavailable","reason":reason}) }}));
            }
            let all_lists_available = records
                .iter()
                .all(|record| record["observation"]["state"] == "Available");
            Ok(
                json!({"state":if all_lists_available { "Available" } else { "Partial" },
                "all_selected_list_reads_available":all_lists_available,"header_words":headers,"lists":records}),
            )
        })();
        let observation =
            captured.unwrap_or_else(|reason| json!({"state":"Unavailable","reason":reason}));
        result["observation"] = observation;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list() -> FineList {
        FineList {
            kind: "Canopy",
            list: 0,
            command_start: 20,
            command_end: 100,
            instance_start: 10,
            instance_end: 20,
            native_max_count: 3,
        }
    }

    #[test]
    fn fine_count_reads_exact_effective_native_prefix_and_never_capacity_tail() {
        assert_eq!(command_range(&list(), 0).unwrap(), (20, 0, 0));
        assert_eq!(command_range(&list(), 2).unwrap(), (20, 40, 2));
        assert_eq!(command_range(&list(), u32::MAX).unwrap(), (20, 60, 3));
        let mut short = list();
        short.command_end = 60;
        assert!(command_range(&short, 3).is_err());
    }

    #[test]
    fn instance_reads_cover_only_real_draw_references_without_reordering_commands() {
        let commands = [[3, 2, 0, 0, 14], [3, 2, 0, 0, 10], [3, 2, 0, 0, 12]];
        assert_eq!(instance_ranges(&list(), &commands).unwrap(), vec![(10, 16)]);
        assert_eq!(
            commands[0][4], 14,
            "CPU read coalescing must not sort commands"
        );
        assert_eq!(
            instance_ranges(&list(), &[[0, 10, 0, 0, u32::MAX], [3, 0, 0, 0, u32::MAX]]).unwrap(),
            vec![]
        );
    }

    #[test]
    fn instance_slice_boundary_overrun_and_underflow_are_unavailable() {
        assert_eq!(
            instance_ranges(&list(), &[[3, 1, 0, 0, 19]]).unwrap(),
            vec![(19, 20)]
        );
        assert!(instance_ranges(&list(), &[[3, 2, 0, 0, 19]]).is_err());
        assert!(instance_ranges(&list(), &[[3, 1, 0, 0, 9]]).is_err());
        assert!(instance_ranges(&list(), &[[3, u32::MAX, 0, 0, u32::MAX]]).is_err());
    }
}
