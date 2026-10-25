# Nani current-host content intent, original consumer evidence

Status: candidate only for the current systems owner. No raw access-code policy change and no manual `side_effect` pinning. The v35 strict real-GPU test stopped before GPU work because its actual compiled snapshot had only 9 passes and omitted original `Nani clear histograms`, unsorted fill/prefix and other initializers. Log: `/private/tmp/tg-original-nani-v35-gpu.log`.

`nani-consumer-preserving.patch` adds only explicit preserving metadata to real master consumers. It depends on `ComputePass::preserve_storage_buffer_contents` and the GraphPasses adapter delivered separately. That adapter must decide preserving before the first native registration, and only on a writing access; raw 0x37 alone still does not imply previous-content reads. All original synchronization groups, native access codes and resource/view ranges remain intact. The added metadata is a present-day graph contract, not an original twelve-byte ledger field.

## Original shader identity

`../nani-gpu-proof/original-master-shaders/manifest.json` retains all 33 bounded actual packaged SPIR-V stages: the 17 master stages and all 16 custom/external cull permutations. Every entry includes installed package SHA256, extracted complete stage SHA256, exact preceding-u32 word count and an instruction walk ending at `OpFunctionEnd`. `.glsl` is SPIRV-Cross's inspection projection of those exact stages; `.spvasm` is the primary instruction evidence. No old lifted shader source was used as authority.

## Exact preserving sites

| Real master consumer | Binding | Original contents use and producer that must survive |
|---|---|---|
| all cull families/permutations | `filtered_instance_allocator_buffer` | Device SSBO `OpAtomicIAdd`; allocators were initialized by original clear-draw-lists. |
| all cull families/permutations | `output_histogram` | Device SSBO `OpAtomicIAdd`; original clear-histogram is required before the first cull. |
| all cull families/permutations | `next_visibility_bitmask` | Device SSBO `OpAtomicOr`; original clear of Next (first round) or Prev (HZB round) must survive. Binding name remains `next_visibility_bitmask` after roles swap. |
| second and later custom/external cull passes | `filtered_instance_buffer`, `filtered_instance_meta_buffer` | Appends only the current subset's range at values returned by its per-list allocator. Earlier generic/custom/external payloads occupy disjoint list-offset regions. The first generic cull is deliberately not preserving the freshly created Meta buffer. |
| histogram-prefix | `inout_buf` | Original storage `OpLoad` and `OpStore` scan in place; consumes cull's histogram counts. |
| unsorted and sorted fill | `histogram` | Device `OpAtomicIAdd` on list*1024+mesh histogram positions; must consume prefix results, and sorted fill must keep unsorted fill's positions. |
| unsorted fill | `draw_list_buffer` | Device `OpAtomicIAdd` to `DrawIndexedIndirectCommand.instance_count`; preserve original prepare-unsorted's index count/first index/zero instance count. |
| sorted fill after unsorted fill | `sorted_instances_buffer` | Store index is atomic result + `list_instance_offset`. Fine and non-fine RenderSublists occupy different regions of the same physical buffer; pure write of fine regions must keep unsorted payloads. |
| unsorted-atlas-prefix | `draw_list_buffer` | Loads index_count/instance_count/first_index/vertex_offset then writes the command with computed first_instance. This is actual in-place update, not discard. |
| allocate-draws | `thread_group_draw_cmd_histogram` | Only dispatched live groups write `histogram_offset + gl_WorkGroupID.x`. Dispatch plane1 is ceil(actual thread_count/32); the following prefix reads through the fixed maximum `range_end`. Therefore original clear-histogram supplies the unwritten padding and cannot be culled. |
| group draw-cmd prefix | `draw_list_length_buffer` | Device `OpAtomicIAdd` to list counter; consumes the original second clear-draw-lists. |
| group draw-cmd prefix | `inout_buf` | Original loads and stores the histogram scan in place. |
| group draw-cmd prefix | `subtotals_buf` | Writes only `subtotals_start + group + 1` when inside bounds. It intentionally does not write the leading subtotal and inactive tail; original clear-subtotals supplies them. |
| subtotal-prefix | `inout_buf` | Original loads/stores subtotals in place; preserve the group-prefix and leading zero. |
| generate-draws | `instance_indirection_allocator_buffer` | Device `OpAtomicIAdd`; consumes the original second allocator clear. Workgroup shared atomics are not confused with device-buffer dependencies. |
| generate-draws | `draw_list_buffer` | Writes only fine list ranges at `list_draw_cmd_offset + group_draw_offset`; non-fine original prepare/fill/prefix commands must remain. |
| fill-unsorted-indirection | `unsorted_instance_atlas_entry_prefix_buffer` | Device `OpAtomicIAdd`; consumes unsorted-atlas-prefix. |
| fill-unsorted-indirection | `instance_indirection_buffer` | Store addresses come from the non-fine list's original prefix and count. Fine output already occupies its own list regions; retaining those earlier writes is required even though this shader's buffer instruction itself is a store. |

No preserving flag is added to original clear passes, full dispatch-argument producers, first fresh sorted output, or first fresh Meta output. Final fine indirection starts a new phase after its temporary filtered-index use; only the subsequent non-fine append preserves the new fine phase. No new clear, empty draw, fake initialized import or blanket `reads_previous` inference is introduced.

## Validation boundary

The patch must first make the real compiled snapshot retain original initializer and partition producers. Then the existing one-real-instance test must execute original shader work and read back its indexed command, true instance-indirection value and new visibility bit. A pass-order match alone is not a GPU result. The 600-storage-descriptor host budget is separately bounded by actual device limits; it is not part of this original consumer contract and two-round full-game requirements remain to be measured.
