# R7 pipeline / descriptor CPU contracts

Target: original Tiny Glade `tiny-glade.rne`, SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. Implementation: `reconstruction/render/work-packages/pipeline-descriptors-r7/src/lib.rs`.

## Recovered and verified

The private persistent-layout specialization `create_descriptor_set_layout<Enumerate<SliceIter<Binding>>>` at `0x1418c2150` now has executable Rust types and control flow. The original complete CPU entry runs through its Vulkan callback and returns, using injected public allocator/free/memset, externally resolved sampler and Vulkan callbacks. 4,438 original-entry fixtures match the Rust callback's actual ash create info, binding flags, descriptor fields and immutable sampler values; four injected Vulkan errors reach the original panic entry. Panic unwinding and allocation failure are excluded.

The original reflection ABI supplied to this entry has 40-byte binding records. Count enum tag at +24 is 0 for fixed and 1 for runtime; fixed count is +28, descriptor tag is +32. Runtime allocation count is `min(max(device_scalar - 32, 0), 8192)`, from device +0x658 -> +0x680. Its precise Vulkan property name remains unknown. Zero effective count panics before creating the layout. More than one descriptor sets PARTIALLY_BOUND (4), including fixed arrays. This flag is not VARIABLE_DESCRIPTOR_COUNT (8).

Six original descriptor tags map to Vulkan types: 0 storage buffer, 1 uniform buffer, 2 combined image sampler, 3 storage image, 4 sampled image, 5 immutable sampler. Exact `_dyn` suffix changes buffer types to their dynamic variants. Runtime sampled image sets binding flags 15 (UPDATE_AFTER_BIND, UPDATE_UNUSED_WHILE_PENDING, PARTIALLY_BOUND, VARIABLE_DESCRIPTOR_COUNT), and layout UPDATE_AFTER_BIND_POOL (2). Immutable sampler bindings always emit one descriptor even if input count exceeds one; get_sampler remains an external provider and its cache/descriptor is not reconstructed.

The remainder of `create_persistent_descriptor_set` at `0x140bfef70` has a typed pool/allocate contract: descriptor counts accumulate with wrapping u32 by type in first-observed order, pool flags UPDATE_AFTER_BIND (2), max_sets 1; allocation always attaches one variable count computed with the same scalar rule. 486 complete entry fixtures match actual pool and allocation callback structures, after injecting the separately validated descriptor layout result and external registry/lifetime callbacks. This is not a validation of managed registry allocation or disposal.

The compute consumer at `0x1416a4b20` creates its pipeline layout with offset 0, COMPUTE (32), and one push range only for a nonzero byte extent read at reflection +0x148. Rust callback construction matches 256 original CPU slice fixtures from `0x1416a4b99` through the actual create-pipeline-layout call at `0x1416a4c5a`. Descriptor-layout production is injected for that slice; shader module and pipeline creation after it remain pending.

## Reflection linking level

The prior catalog supplies 638 module-entry declarations. All 638 were used as candidate adapters into the **enumerated persistent-layout** specialization, with original reflection class/name/count semantics, constant stage COMPUTE and externally supplied sampler handle. This validates CPU conversion of those declaration shapes; it does **not** prove that the game actually sends these objects to this specialization. Set indices, sparse shader binding indices, real shader-to-pass selection, visibility, immutable sampler descriptions and original shader_pipeline reflection producer are not closed. Native sparse-map specialization `0x1418c1a10` was inspected but has not been implemented or entry-validated. No shader algorithm or real GPU/pass binding is claimed.

## Tools and evidence boundaries

Eight focused decompile/xref pairs were obtained from the existing `tg_render_arch` database using the configured RE-MCP server and official MCP client SDK. Actual catalog and connection results are retained. Auto-analysis was disabled; close_database(save:false) succeeded and the exclusive window was immediately released. No global tool config changed.

Ghidra misrecovered the `0x1418c1a10` jump table and wandered past its PE function range into adjacent function/data. Its pseudo-code is retained as fallible evidence, not executable truth. The recovered six-branch conversion uses bounded original disassembly and the original jump table at `0x142d063b0`. `persistent-layout.asm.txt` includes the enumerated specialization and its unwind fragments only.

Saved-database xrefs are incomplete with auto-analysis disabled. `call-evidence.json` retains workgraph direct-call and indirect-call evidence for the selected functions; a missing Ghidra xref is not an absence-of-consumer claim. PDB void signatures remain placeholders. Static observation does not establish runtime execution.

Dependencies directly reuse original ash 0.38.0+1.3.281, serde 1.0.200 and serde_json 1.0.85; all 21 registry packages, source URLs and checksums match the original lock. Unknown local/fork shader_pipeline origin is not fabricated. No public library was reimplemented.

## Reproduce / remaining

Run `python evidence/render-recovery/parallel-batch-05/pipeline-descriptors/verify.py` from the project root. It builds locked/offline Rust binaries and replays original CPU instructions in Unicorn without launching the game or Vulkan. `cargo test --locked --offline`, fmt and strict clippy pass.

Descriptor writes, full sparse map layout producer, complete create_compute_pipeline/create_raster_pipeline, shader module/specialization/cache state, descriptor lifetime/image layouts, descriptor feature limits, full managed disposal and real Vulkan/GPU/frame acceptance remain pending. M12 remains verified_partial. CPU contract verification is reported separately from reflection adapter coverage and candidate Rust tests.
