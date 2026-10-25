# Nani owned/custom cull BDA lifetime and exact-range design

Scope: current single-App/single-graph consumer of B recovered Nani culling. Root authorized continued candidate work under TG-5f034e6af1260241e4 and TG-11d607188d6724d9f3 old render-nani-main claims; locks and old deliverables remain unchanged. This document is a pre-implementation contract for independent design review, not a runtime failure or full-world equivalence claim. Initial broad custom-position read wording was corrected after inspecting both original CurvedHalfTimber SPV permutations; no blanket custom dependency will be implemented.

## Actual path and missing declaration

world_graph.rs already calls scene_compute::generate_nani_draw_lists twice (None then Some(HZB)); adding duplicate calls would be wrong. scene_compute borrows the current World GpuWorld by resource_scope and passes exactly frame.hosts().buffers to the core cull builder. OwnedGpuWorldSubset has separate positions: ShadowedBuffer<InstanceCullData> and boxed instance data. InstanceCullData is16 bytes. positions owns its actual GpuInstance buffer created with usage0x20022 and full initial bytes. Native/current active count is the dense instance_count, independent of full capacity. Flush updates actual positions/data before culling and preserves sparse dirty updates.

The generic owned_cull_batch entry reads active count and passes device_address(&subset.cull_buffer().buffer) in its20-byte push payload; the custom Owned branch does the same with the original custom word order. For both, the numeric address is obtained from a generation-checked BufferHost lookup, but that temporary ManagedBuffer clone is dropped when device_address returns. The captured callback contains the numeric pointer/count only; its RecordedComputePass.device_address_reads is empty.

GpuWorld::import_to_rg imports Owned instance data only. It does not import positions. The current FrameBufferGraph forwards actual explicitly imported resources to FrameDeclarer/session, and session imports deduplicate by Arc physical identity. No global walk imports every BufferHost slot. Other bindless/VAO data imports therefore cannot establish a positions-buffer read. The current commands retain graph-declared physical resources, not every object in the host slot table.

## Existing producer protection, and its limit

BufferHost::update_buffer_bytes promises completion on return. GPU-only updates use the bounded transfer manager and wait for that update's actual points; host-visible access paths use the existing range-state submission gate and completion snapshot. Thus creation/flush is already a real producer, and this fix must not add upload, clear, wait-idle, fake initialization or a CPU readback. Those writes can only observe previous GPU reads that were registered in last_use/range-state. A cull BDA absent from graph/command use cannot acquire that read receipt merely because its upload was completed.

Current sources prove absent cull use metadata, not a measured crash. The existing update/remove methods write positions and mark dirty ranges, and flush uploads them to the same allocation. Without registering an earlier cull read, the existing range-state update protocol cannot account for that read. Actual OS Edit/Reload schedules have not been run here; the newer7-type ECS producer remains a separate candidate, and current clear_owned_instances has not been proved to be called by Reload. Reuse/release impacts remain validation risks, not reproduced user-action races or crashes. No new Edit/Reload producer is invented.

## Proposed narrow existing-API adaptation

Modify only core gpu_generate_draw_lists.rs and its sole production scene_compute.rs call. Keep GraphicsDevice/GpuWorld/allocator APIs and all original algorithm/count/dispatch/push packing unchanged. Add a mandatory owner resolver argument to gpu_generate_draw_lists:

`retain_cull: &dyn Fn(&Buffer, u64) -> rhapsody::Result<DynamicConstantsChunk>`

The second argument is the exact readable active-prefix byte count (checked count*size_of::<InstanceCullData>(), with zero-count subsets still skipped). The builder returns rhapsody::Result<NaniGraphOutputs> so a missing/stale owner or invalid range remains a typed failure. No default owner/success fallback exists.

scene_compute's resolver uses the same frame BufferHost.managed_buffer(full id+generation handle), then owner.shared(), offset0,size=active_bytes. Reject stale handle or range beyond actual allocation. Core verifies that returned offset/size match the requested prefix and returned owner.device_address equals the already-used device.device_address. Reject mismatched owner/address rather than retaining an unrelated buffer. This is current-host admission validation, not a changed native cull algorithm.

owned_cull_batch returns its original batches plus retained actual chunks; attach chunks with existing RecordedComputePass::with_device_address_reads. The custom Owned Plaster/Smoke/StairsFloor passes attach their one exact chunk through the same API. CurvedHalfTimber keeps its existing descriptor access and original numeric payload but does not attach a positions read: both original permutations have no physical pointer load and derive culling data from instance_buffer instead. External RoofTile/WoodenPlank/Flower/Bush paths retain their existing descriptors, indirect count and graph-owned storage; no change to them.

The existing GraphPasses::record consumes device_address_reads, validates nonempty in-bounds/BDA-capable chunks, imports each actual Arc via the originating session with access0x1a, and adds its exact Buffer range in synchronization group1. The session deduplicates repeated imports by Arc physical identity. Existing command recording retains physical resources and registers submitted uses through GPU completion; declaration leases retire on cancel/retirement. Cancellation/error paths remain the same frame/session mechanisms.

## Exact range and original evidence

Retained original master shaders already exist under nani-gpu-proof/original-master-shaders with a manifest. Main and Plaster/Smoke/StairsFloor custom shaders first return when invocation index>=input_instance_count, then load the pointer at base+16*index. Both occlusion permutations share this bounded record access. CurvedHalfTimber has the count guard and the address field in its push layout, but no physical load from that address in either permutation; a shared PhysicalStorageBuffer capability declaration alone does not prove a read. The candidate will retain0..count*16, not full capacity, zero bytes, or mesh/data buffers in place of positions. Packaged SPV/control-flow snippets and native callback manifests are retained and will be referenced in the implementation manifest; no full PDB or shader rescan is needed.

## Cost and proposed verification

No new GPU operation, readback, upload, lock-held wait or allocator. New metadata uses one existing DynamicConstantsChunk per active owned subset per cull stage; BufferHost resolution takes its existing short slot lock and clones the actual Arc. Generic batches keep one bounded list (at most27 subsets) and custom paths one entry. Actual performance is unmeasured.

Preserve original push/dispatch expectations. Add a targeted actual BufferHost test which resolves a real cull handle/range, rejects stale/mismatched/out-of-bounds requests, and verifies cancellation plus GPU command completion keep the physical owner after host handle release. Reuse existing GraphPasses device_address_reads exact-range/cancel/submit coverage as mechanism evidence, while adding a Nani-path check so the real generic/custom builders actually attach metadata. Root owns Cargo/GPU execution. Pre-implementation review requested; no production or candidate source has yet changed.
