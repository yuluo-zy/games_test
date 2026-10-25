// Original B gpu_generate_draw_lists@140987dd0 and its complete callback family.
// This is source-host integration over the current core PassEnv / graph APIs.
// The caller provides the real GpuWorld, same-device buffer owner and optional
// HZB. It neither manufactures a scene nor substitutes CPU counts for GPU args.

use crate::render::nani::{buffers as nani_buffers, gpu_world::{GpuWorld,
    GpuWorldSubset, GpuWorldSubsetKind}, cull_shaders::NaniCullAndBucketFamily};
use crate::render::pass_builder::compute_pass_builder::{ComputePass,
    ComputeDispatchIndirectApi};
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;

/// The old `FilteredInstance` name is a binding role, not an original SSBO
/// type. Native 141608260/14160c630 both name NaniInstanceIndirectionSsbo.
pub struct GameNaniSsbos;
impl NaniSsbos for GameNaniSsbos {
    type DrawListOffsets = nani_buffers::NaniDrawListOffsetDefaults;
    type DrawListLength = nani_buffers::NaniDrawListLengthSsbo;
    type InstanceIndirectionAllocator = nani_buffers::NaniInstanceIndirectionAllocatorSsbo;
    type FilteredInstanceAllocator = nani_buffers::NaniFilteredInstanceAllocatorSsbo;
    type DrawList = nani_buffers::NaniDrawListSsbo;
    type InstanceIndirection = nani_buffers::NaniInstanceIndirectionSsbo;
    type FilteredInstance = nani_buffers::NaniInstanceIndirectionSsbo;
    type PrevVisibility = nani_buffers::NaniPrevVisibilityBitmaskSsbo;
    type NextVisibility = nani_buffers::NaniNextVisibilityBitmaskSsbo;
    type DrawBuffer = super::super::indirect_params::DrawIndexedIndirectBuffer;
}

#[derive(Clone, Copy, Debug)]
pub struct NaniHzb {
    pub texture: RgTexture,
    /// Native HZB tuple prefix, not the padded backing image extent.
    pub inner_dims: [u32; 2],
}

/// Snapshot of one actual RenderSublistsIter item. All offsets come from the
/// same GpuWorld NaniDrawListParams; no second allocator/cache is introduced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaniSublist {
    pub kind: GpuWorldSubsetKind,
    pub list_index: u32,
    pub instance_offset: u32,
    pub command_offset: u32,
    pub histogram_offset: u32,
    pub histogram_end: u32,
    pub subtotal_offset: u32,
    pub subtotal_end: u32,
}

pub fn render_sublists(
    params: &nani_buffers::NaniDrawListParams,
    fine: bool,
    range: core::ops::Range<u8>,
) -> Vec<NaniSublist> {
    assert!(range.start <= range.end && range.end <= 4);
    let mut result = Vec::new();
    for kind in GpuWorldSubsetKind::all() {
        if kind.use_fine_grained_sorting() != fine { continue; }
        for lane in range.clone() {
            let index = kind as usize * 4 + lane as usize;
            result.push(NaniSublist {
                kind,
                list_index: index as u32,
                instance_offset: params.cmd_offsets[index],
                command_offset: params.max_instance_offsets[index],
                histogram_offset: params.fine_grained_offsets[index],
                histogram_end: params.fine_grained_offsets.get(index + 1)
                    .copied().unwrap_or(params.total_fine_grained),
                subtotal_offset: params.batch_offsets[index],
                subtotal_end: params.batch_offsets.get(index + 1)
                    .copied().unwrap_or(params.total_batches),
            });
        }
    }
    result
}

fn word_bytes(words: &[u32]) -> Vec<u8> {
    words.iter().flat_map(|v| v.to_le_bytes()).collect()
}

/// A dispatch-argument plane has 108 aligned uint4 records. Native callbacks
/// select plane 0/1/2/3 for 64/32/32768/subtotal work, respectively.
pub fn nani_dispatch_offset(plane: u32, list_index: u32) -> u64 {
    assert!(plane < 4 && list_index < 108);
    u64::from(plane * 108 + list_index) * 16
}

fn indirect_commands(
    pass: ComputePass,
    args: &RgBuffer,
    commands: Vec<(Vec<u8>, u64)>,
) -> RecordedComputePass {
    pass.dispatch_indirect_with(args, move |api: &mut dyn ComputeDispatchIndirectApi| {
        for (constants, offset) in commands {
            api.push_constants(&constants);
            api.dispatch_indirect(offset);
        }
    })
}

#[derive(Clone, Copy)]
struct NaniCullSource {
    family: NaniCullAndBucketFamily,
    oc_pass: i32,
}
impl AsComputeShaderSource for NaniCullSource {
    fn source_hash(&self) -> u64 {
        crate::render::utils::calculate_hash(&(self.family.source_uri().as_str(), &self.oc_pass))
    }
    fn as_compute_pipeline(&self) -> crate::render::pass_builder::shader_uri::ComputeShaderSource<'_> {
        use crate::render::pass_builder::shader_uri::{ComputeShaderSource, ShaderDefValue, ShaderUri};
        ComputeShaderSource { uri: ShaderUri::parse(self.family.path()), defs: ShaderDefValue::default() }
    }
    fn pipeline_defines(&self) -> Vec<(String, String)> {
        vec![("PERM_oc_pass".to_owned(), self.oc_pass.to_string())]
    }
}

fn is_special_cull(kind: GpuWorldSubsetKind) -> bool {
    matches!(kind, GpuWorldSubsetKind::RoofTile | GpuWorldSubsetKind::Flower |
        GpuWorldSubsetKind::Bush | GpuWorldSubsetKind::WoodenPlank |
        GpuWorldSubsetKind::Plaster | GpuWorldSubsetKind::Smoke |
        GpuWorldSubsetKind::CurvedHalfTimber | GpuWorldSubsetKind::StairsFloor)
}

fn owned_cull_batch(
    gpu_world: &GpuWorld,
    device: &dyn GraphicsDevice,
) -> Vec<(PushConstants, u32)> {
    let mut result = Vec::new();
    for kind in GpuWorldSubsetKind::all() {
        if is_special_cull(kind) { continue; }
        let slot = gpu_world.get_subset(kind);
        let Some(GpuWorldSubset::Owned(subset)) = &*slot else {
            panic!("Nani generic cull subset {kind:?} is not owned");
        };
        let count = subset.instance_count() as u32;
        if count == 0 { continue; }
        let address = device.device_address(&subset.cull_buffer().buffer);
        assert_ne!(address, 0, "Nani cull input has no GPU device address");
        // Multiunzip@140b2b8f5 writes address@0, kind@8, count@c, list@10.
        result.push((PushConstants([address as u32, (address >> 32) as u32,
            kind as u32, count, kind as u32 * 4]), count));
    }
    result
}

fn record_empty_unsorted(
    env: &mut PassEnv<'_>,
    atlas: &ExternalBufferHandle,
    lists: &[NaniSublist],
) {
    let constants: Vec<_> = lists.iter().map(|s|
        word_bytes(&[s.kind as u32, s.command_offset])).collect();
    let pass = env.new_pass("Nani prepare unsorted draw lists",
        &"/nani/prepare_unsorted_draw_lists.cs.hlsl")
        .bind_external_storage_buffer(H_ATLAS_PART_INFO, atlas)
        .write_storage_buffer::<nani_buffers::NaniDrawListSsbo>(H_DRAW_LIST_BUFFER)
        .dispatch_with(move |api| {
            for constants in constants {
                api.push_constants(&constants);
                api.dispatch_1d(64);
            }
        });
    env.record(pass);
}

fn record_sparse_cull<T: crate::render::sparse_instance_buffer::SparseSsbo + LibrarySsbo<
    Storage = crate::render::sparse_instance_buffer::SparseInstanceBuffer<T>>>(
    env: &mut PassEnv<'_>,
    source: NaniCullSource,
    res: &CullBucketResources<'_>,
    kind: GpuWorldSubsetKind,
) {
    let resources = env.pass_resources();
    let sparse = resources.ssbo_library().get::<T>()
        .expect("Nani external sparse instance buffer is missing");
    let ranges = sparse.to_draw_multi_instanced();
    let pass = create_cull_and_bucket_pass::<_, GameNaniSsbos>(env, &source, res)
        .read_storage_buffer::<T>(H_INSTANCE_BUFFER)
        .dispatch_with(move |api| {
            for (first, count) in ranges {
                // Original 1415f9630: exactly 16 B, not the main 20 B payload.
                api.push_constants(&word_bytes(&[kind as u32, first, count, kind as u32 * 4]));
                api.dispatch_1d(count);
            }
        });
    env.record(pass);
}

fn record_external_gpu_cull<T: LibrarySsbo>(
    env: &mut PassEnv<'_>,
    source: NaniCullSource,
    res: &CullBucketResources<'_>,
    kind: GpuWorldSubsetKind,
    draw_index: u32,
) {
    let args = env.create_buffer("Dispatch args", 16, 0);
    let pass = env.new_pass("Draw -> dispatch args",
        &"/nani/draw_indexed_indirect_to_dispatch.cs.hlsl")
        .read_storage_buffer::<super::super::indirect_params::DrawIndexedIndirectBuffer>(H_DRAW_BUFFER)
        .write_rg_storage_buffer(H_ARGS_BUFFER, &args)
        .push_constants(&word_bytes(&[draw_index, 64]))
        .dispatch_1d(1);
    env.record(pass);
    let pass = create_cull_and_bucket_pass::<_, GameNaniSsbos>(env, &source, res)
        .push_constants(&word_bytes(&[kind as u32, kind as u32 * 4]))
        .read_rg_storage_buffer(binding_hash("dispatch_args_buffer"), &args)
        .read_storage_buffer::<T>(H_INSTANCE_BUFFER)
        .dispatch_indirect_raw(&args, 0);
    env.record(pass);
}

#[derive(Clone, Debug)]
pub struct NaniGraphOutputs {
    pub sorting_histogram: RgBuffer,
    pub filtered_instance_meta: RgBuffer,
    pub dispatch_args: RgBuffer,
    pub thread_count: RgBuffer,
    pub sorted_instances: RgBuffer,
    pub unsorted_atlas_prefix: RgBuffer,
    pub draw_cmd_histogram: RgBuffer,
    pub draw_cmd_subtotals: RgBuffer,
}

/// Original top-level business orchestration. `None` is the first cull round;
/// `Some(HZB)` swaps the visibility roles and emits main-camera-only work.
pub fn gpu_generate_draw_lists(
    env: &mut PassEnv<'_>,
    gpu_world: &GpuWorld,
    device: &dyn GraphicsDevice,
    hzb: Option<NaniHzb>,
) -> NaniGraphOutputs {
    let params = gpu_world.draw_list_params();
    let range = 0..if hzb.is_some() { 1 } else { 4 };
    let unsorted = render_sublists(params, false, range.clone());
    let fine = render_sublists(params, true, range);
    let histogram = env.create_buffer("Nani list sorting histogram", 0x6c000, 0);
    let filtered_meta = env.create_buffer("Nani filtered instance meta", params.total_cmd_offset * 4, 0);
    let pass = clear_draw_lists::<GameNaniSsbos>(env); env.record(pass);
    let pass = clear_histogram(env, &histogram); env.record(pass);

    let resources = env.pass_resources();
    let mut graph = env.world.resource_graph();
    let visibility = if hzb.is_some() {
        resources.ssbo_library().get_rg_buffer::<nani_buffers::NaniPrevVisibilityBitmaskSsbo>(&mut graph)
    } else {
        resources.ssbo_library().get_rg_buffer::<nani_buffers::NaniNextVisibilityBitmaskSsbo>(&mut graph)
    };
    crate::render::rg_helpers::clear_rg_buffer(env.frame, u32::try_from(visibility.resource).expect("Nani graph resource exceeds u32"));

    // Host handles resolve only through the same device's live owner table.
    let atlas = crate::rhapsody_contract::ManagedHandle(gpu_world.mesh_atlas_info_buf().buffer.id);
    let draw_info = crate::rhapsody_contract::ManagedHandle(gpu_world.draw_list_info_buf().buffer.id);
    record_empty_unsorted(env, &atlas, &unsorted);
    let res = CullBucketResources { filtered_instance_meta: &filtered_meta,
        output_histogram: &histogram, atlas_part_info: &atlas,
        draw_list_info: &draw_info, hzb: hzb.as_ref().map(|v| &v.texture),
        hzb_dims: hzb.as_ref().map(|v| v.inner_dims).unwrap_or([0, 0]) };
    let oc_pass = i32::from(hzb.is_some());
    let main = owned_cull_batch(gpu_world, device);
    let source = NaniCullSource { family: NaniCullAndBucketFamily::Main, oc_pass };
    let pass = create_cull_and_bucket_pass::<_, GameNaniSsbos>(env, &source, &res)
        .dispatch_with(move |api| cull_and_bucket_dispatch(api, &main));
    env.record(pass);

    // Native second loop distinguishes absent, externally-owned and internally
    // owned/custom subsets. Unsupported ownership is a native panic, not skip.
    for kind in GpuWorldSubsetKind::all() {
        let slot = gpu_world.get_subset(kind);
        let Some(subset) = &*slot else { continue; };
        let source = NaniCullSource { family: NaniCullAndBucketFamily::for_kind(kind), oc_pass };
        match subset {
            GpuWorldSubset::External(_) => match kind {
                GpuWorldSubsetKind::RoofTile => record_sparse_cull::<
                    crate::resources::roofs::roof_tile_ssbo::RoofTileSsbo>(env, source, &res, kind),
                GpuWorldSubsetKind::WoodenPlank => record_sparse_cull::<
                    crate::resources::roofs::roof_buffers::WoodenPlankSsbo>(env, source, &res, kind),
                GpuWorldSubsetKind::Flower => record_external_gpu_cull::<
                    super::compute_garden::FlowerSsbo>(env, source, &res, kind, 11),
                GpuWorldSubsetKind::Bush => record_external_gpu_cull::<
                    super::compute_garden::BushSsbo>(env, source, &res, kind, 10),
                _ => panic!("Nani external subset {kind:?} has no original cull consumer"),
            },
            GpuWorldSubset::Owned(subset) => {
                if matches!(kind, GpuWorldSubsetKind::RoofTile | GpuWorldSubsetKind::WoodenPlank |
                    GpuWorldSubsetKind::Flower | GpuWorldSubsetKind::Bush) {
                    panic!("Nani subset {kind:?} must be external");
                }
                if !is_special_cull(kind) { continue; }
                let count = subset.instance_count() as u32;
                if count == 0 { continue; }
                let address = device.device_address(&subset.cull_buffer().buffer);
                assert_ne!(address, 0, "Nani custom cull input has no GPU device address");
                // Custom callback payload order differs from Main at words3/4.
                let constants = word_bytes(&[address as u32, (address >> 32) as u32,
                    kind as u32, kind as u32 * 4, count]);
                let imported = subset.data().rg_handle();
                let buffer = RgBuffer { resource: u32::try_from(imported.resource).expect("Nani graph resource exceeds u32") };
                let pass = create_cull_and_bucket_pass::<_, GameNaniSsbos>(env, &source, &res)
                    .push_constants(&constants)
                    .read_rg_storage_buffer(H_INSTANCE_BUFFER, &buffer)
                    .dispatch_1d(count);
                env.record(pass);
            }
        }
    }
    let pass = histogram_prefix_sum(env, &histogram); env.record(pass);

    // 4 argument planes * 108 uint4 commands; this allocation is not one
    // command per actual CPU instance or only one plane per active subset.
    let args = env.create_buffer("Nani dispatch args", 0x1b00, 0);
    let thread_count = env.create_buffer("Nani dispatch thread count", 0x1b0, 0);
    let pass = env.new_pass("Nani generate dispatch args", &"/nani/generate_dispatch_args.cs.hlsl")
        .read_storage_buffer::<nani_buffers::NaniDrawListOffsetDefaults>(H_INSTANCE_LIST_OFFSET)
        .read_storage_buffer::<nani_buffers::NaniFilteredInstanceAllocatorSsbo>(H_FILTERED_INSTANCE_ALLOCATOR)
        .write_rg_storage_buffer(binding_hash("dispatch_args_buffer"), &args)
        .write_rg_storage_buffer(H_THREAD_COUNT_BUFFER, &thread_count)
        .dispatch_1d(108);
    env.record(pass);
    let sorted = record_sorted_lists(env, params, &unsorted, &fine, &thread_count,
        &filtered_meta, &histogram, &args);
    // The temporary filtered-index role of the shared indirection buffer ends
    // here. Original code resets allocators again before emitting final draws.
    let pass = clear_draw_lists::<GameNaniSsbos>(env); env.record(pass);
    let histogram_words = params.total_fine_grained.max(1).div_ceil(64) * 64;
    let subtotals_words = params.total_batches.max(1);
    let cmd_histogram = env.create_buffer("Nani draw count per thread group", histogram_words * 4, 0x20);
    let subtotals = env.create_buffer("Nani group draw subtotals", subtotals_words * 4, 0x20);
    let pass = env.new_pass("Nani draw cmd histogram clear", &"/nani/clear_histogram.cs.hlsl")
        .write_rg_storage_buffer(H_HISTOGRAM, &cmd_histogram).dispatch_1d(histogram_words);
    env.record(pass);
    let pass = env.new_pass("Nani draw cmd subtotals clear", &"/nani/clear_subtotals.cs.hlsl")
        .write_rg_storage_buffer(binding_hash("subtotals"), &subtotals)
        .push_constants(&word_bytes(&[subtotals_words])).dispatch_1d(subtotals_words);
    env.record(pass);
    let prefix = record_unsorted_prefix(env, &unsorted);
    record_final_draw_commands(env, &atlas, &fine, &unsorted, &sorted, &thread_count,
        &args, &cmd_histogram, &subtotals, &prefix);
    NaniGraphOutputs { sorting_histogram: histogram, filtered_instance_meta: filtered_meta,
        dispatch_args: args, thread_count, sorted_instances: sorted,
        unsorted_atlas_prefix: prefix, draw_cmd_histogram: cmd_histogram,
        draw_cmd_subtotals: subtotals }
}

fn record_sorted_lists(
    env: &mut PassEnv<'_>, params: &nani_buffers::NaniDrawListParams,
    unsorted: &[NaniSublist], fine: &[NaniSublist], thread_count: &RgBuffer,
    filtered_meta: &RgBuffer, histogram: &RgBuffer, args: &RgBuffer,
) -> RgBuffer {
    let sorted = env.create_buffer("Nani sorted instances", params.total_cmd_offset * 4, 0x20);
    for (is_fine, lists) in [(false, unsorted), (true, fine)] {
        let (name, path) = if is_fine {
            ("Nani fill sorted instance list", "/nani/fill_sorted_instance_list.cs.hlsl")
        } else {
            ("Nani fill unsorted instance list", "/nani/fill_unsorted_instance_list.cs.hlsl")
        };
        let mut pass = env.new_pass(name, &path)
            .read_storage_buffer::<nani_buffers::NaniDrawListOffsetDefaults>(H_INSTANCE_LIST_OFFSET)
            .read_rg_storage_buffer(H_THREAD_COUNT_BUFFER, thread_count)
            .read_storage_buffer::<nani_buffers::NaniInstanceIndirectionSsbo>(H_FILTERED_INSTANCE_BUFFER)
            .read_rg_storage_buffer(H_FILTERED_INSTANCE_META_BUFFER, filtered_meta)
            .write_rg_storage_buffer_concurrent(H_HISTOGRAM, histogram, 1)
            .write_rg_storage_buffer_concurrent(H_SORTED_INSTANCES_BUFFER, &sorted, 1);
        if !is_fine {
            pass = pass.write_storage_buffer_concurrent::<nani_buffers::NaniDrawListSsbo>(H_DRAW_LIST_BUFFER, 1);
        }
        let commands = lists.iter().map(|s| {
            let mut words = vec![s.list_index, s.instance_offset];
            if !is_fine { words.push(s.command_offset); }
            (word_bytes(&words), nani_dispatch_offset(0, s.list_index))
        }).collect();
        let pass = indirect_commands(pass, args, commands); env.record(pass);
    }
    sorted
}

fn record_unsorted_prefix(env: &mut PassEnv<'_>, lists: &[NaniSublist]) -> RgBuffer {
    let prefix = env.create_buffer("Nani unsorted atlas idx prefix", 0x6c00, 0x20);
    let constants: Vec<_> = lists.iter().map(|s| word_bytes(&[s.list_index, s.command_offset])).collect();
    let pass = env.new_pass("Nani unsorted atlas idx prefix sum",
        &"/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl")
        .read_storage_buffer::<nani_buffers::NaniDrawListOffsetDefaults>(H_INSTANCE_LIST_OFFSET)
        .write_storage_buffer::<nani_buffers::NaniDrawListSsbo>(H_DRAW_LIST_BUFFER)
        .write_rg_storage_buffer(H_UNSORTED_ATLAS_PREFIX, &prefix)
        .dispatch_with(move |api| { for constants in constants {
            api.push_constants(&constants); api.dispatch_1d(64);
        }});
    env.record(pass);
    prefix
}

fn record_final_draw_commands(
    env: &mut PassEnv<'_>, atlas: &ExternalBufferHandle,
    fine: &[NaniSublist], unsorted: &[NaniSublist], sorted: &RgBuffer,
    thread_count: &RgBuffer, args: &RgBuffer, histogram: &RgBuffer,
    subtotals: &RgBuffer, prefix: &RgBuffer,
) {
    let pass = env.new_pass("Nani allocate draws", &"/nani/allocate_draw_commands.cs.hlsl")
        .read_rg_storage_buffer(H_SORTED_INSTANCES_BUFFER, sorted)
        .read_storage_buffer::<nani_buffers::NaniDrawListOffsetDefaults>(H_INSTANCE_LIST_OFFSET)
        .write_rg_storage_buffer(H_THREAD_GROUP_DRAW_CMD_HISTOGRAM, histogram)
        .read_rg_storage_buffer(H_THREAD_COUNT_BUFFER, thread_count);
    let commands = fine.iter().map(|s| (word_bytes(&[s.list_index, s.histogram_offset]),
        nani_dispatch_offset(1, s.list_index))).collect();
    let pass = indirect_commands(pass, args, commands); env.record(pass);
    let pass = env.new_pass("Nani draw cmd histogram prefix", &"/nani/group_draw_cmd_prefix_sum.cs.hlsl")
        .write_storage_buffer::<nani_buffers::NaniDrawListLengthSsbo>(H_DRAW_LIST_LENGTH_BUFFER)
        .write_rg_storage_buffer(H_INOUT_BUF, histogram)
        .write_rg_storage_buffer(H_SUBTOTALS_BUF, subtotals);
    let commands = fine.iter().map(|s| (word_bytes(&[s.histogram_offset, s.histogram_end,
        s.subtotal_offset, s.subtotal_end, s.list_index]), nani_dispatch_offset(2, s.list_index))).collect();
    let pass = indirect_commands(pass, args, commands); env.record(pass);
    let pass = env.new_pass("Nani draw cmd subtotal prefix", &"/nani/subtotal_prefix_sum.cs.hlsl")
        .write_rg_storage_buffer(H_INOUT_BUF, subtotals);
    let commands = fine.iter().map(|s| (word_bytes(&[s.subtotal_offset, s.subtotal_end]),
        nani_dispatch_offset(3, s.list_index))).collect();
    let pass = indirect_commands(pass, args, commands); env.record(pass);
    let pass = env.new_pass("Nani generate draws", &"/nani/generate_draw_lists.cs.hlsl")
        .bind_external_storage_buffer(H_ATLAS_PART_INFO, atlas)
        .read_rg_storage_buffer(H_SORTED_INSTANCES_BUFFER, sorted)
        .read_storage_buffer::<nani_buffers::NaniDrawListOffsetDefaults>(H_INSTANCE_LIST_OFFSET)
        .write_storage_buffer_concurrent::<nani_buffers::NaniDrawListSsbo>(H_DRAW_LIST_BUFFER, 1)
        .write_storage_buffer_concurrent::<nani_buffers::NaniInstanceIndirectionSsbo>(H_INSTANCE_INDIRECTION_BUFFER, 1)
        .write_storage_buffer_concurrent::<nani_buffers::NaniInstanceIndirectionAllocatorSsbo>(H_INSTANCE_INDIRECTION_ALLOCATOR, 1)
        .read_rg_storage_buffer(H_THREAD_GROUP_DRAW_CMD_HISTOGRAM, histogram)
        .read_rg_storage_buffer(H_THREAD_GROUP_DRAW_CMD_SUBTOTALS, subtotals)
        .read_rg_storage_buffer(H_THREAD_COUNT_BUFFER, thread_count);
    let commands = fine.iter().map(|s| (word_bytes(&[s.kind as u32, s.list_index,
        s.command_offset, s.histogram_offset, s.subtotal_offset]), nani_dispatch_offset(1, s.list_index))).collect();
    let pass = indirect_commands(pass, args, commands); env.record(pass);
    let pass = env.new_pass("Nani fill unsorted instance indirection",
        &"/nani/fill_unsorted_instance_indirection.cs.hlsl")
        .read_rg_storage_buffer(H_SORTED_INSTANCES_BUFFER, sorted)
        .read_storage_buffer::<nani_buffers::NaniDrawListOffsetDefaults>(H_INSTANCE_LIST_OFFSET)
        .read_rg_storage_buffer(H_THREAD_COUNT_BUFFER, thread_count)
        .write_rg_storage_buffer_concurrent(H_UNSORTED_ATLAS_PREFIX, prefix, 1)
        .write_storage_buffer_concurrent::<nani_buffers::NaniInstanceIndirectionSsbo>(H_INSTANCE_INDIRECTION_BUFFER, 1);
    let commands = unsorted.iter().map(|s| (word_bytes(&[s.list_index]),
        nani_dispatch_offset(0, s.list_index))).collect();
    let pass = indirect_commands(pass, args, commands); env.record(pass);
}
