# Original Nani count semantics and M3 current-architecture adapter

Candidate only. No production edit, Cargo invocation or GPU run by this worker. Offline glslang compilation and spirv-val passed; compilation/runtime validation remain with root.

## Original business contract

`nani_draw@140904ae0` complete1428B is retained with exact PE-code/assembly hash in `original-evidence-manifest.json`. Original instructions140904de8..e31 choose count for forceFine or (kind<=1 and !forceNoFine);140904e4c reads the original27-entry max table;140904e8e calls draw_indexed_indirect_count with source offset cmdSlot*20, stride20, count offset listIndex*4 and the actual maximum. Raw branch140904ed4 calls draw_indexed_indirect_raw with actual mesh-part count. Do not replace fine sorting with the raw business branch.

The existing nineteen-program original Nani evidence pack is reused by exact file hash, not regenerated or altered. Its relevant complete packaged SPIR-V shows:

- clear_draw_lists clears108 count words and initializes two allocators. It does **not** clear indirect command storage.
- prepare_unsorted_draw_lists writes64 zero-instance commands per unsorted mesh list. Fine lists do not receive that initialization.
- generate_draw_lists writes only the dense live prefix, using generated group command offsets and actual atlas info. The unused command tail can retain prior-frame commands.
- group_draw_cmd_prefix_sum atomically produces the actual per-list command count.

Thus fixed maximum raw drawing over the original buffer is wrong: stale nonzero tail commands would render. A count may shrink to1 or0 without rewriting that tail.

The official local capability probe `/private/tmp/tg-vulkan-capabilities-v58.log` reports AppleM3, device API1.4.357, multiDrawIndirect=true, drawIndirectCount=false. The log's SHA is recorded. Current device/init already enables drawIndirectCount whenever v12 reports it; no missing enable switch should be invented. Kind7 is ordinary indexed indirect and is distinct from kind8. Current source and v26's two count skips only concern mode8.

## Additive platform lowering within current rhapsody

The new `rg::indirect_count::IndexedCountAdapter` uses the existing bindless heap, AsyncPipelineCompiler, DynamicConstants pool, current RenderGraph and session-declared scratch Buffer. It introduces no separate allocator/backend/descriptor pool, changes no original Nani shader or command producer, and never reads the count on the CPU.

For each original count packet on a device that physically lacks the count feature, a same-graph compute pass reads the real counter and source commands. For i<min(actualCount,maxDrawCount), it copies all five20B command words exactly (including signed vertexOffset bits and firstInstance). For every remaining i<maxDrawCount it writes a fully zero command to fresh scratch. The raster packet then uses ordinary indexed indirect with tight stride20 and maxDrawCount on that scratch. Prefix DrawIndex values are unchanged; zero tail commands have no vertex/instance invocations. The original source and count remain intact. The actual source offset/stride and count offset are honored.

Native enabled-count devices keep their native kind8 path. A physically supported but unenabled count feature remains an explicit configuration error. Maximum0 records an ordinary zero-count draw and allocates no zero-size scratch. MultiDrawIndirect and actual device draw/dispatch/descriptor-range limits remain enforced.

Moving count snapshotting before the raster pass is equivalent for original Nani because its count and command buffers remain read-only during that pass. The host explicitly rejects fallback when that same raster submission declares a write to either source, rather than silently changing a more general in-pass mutation contract.

Every count packet receives a fresh, independent scratch. Its name uses the current raster scope and current graph resource index, checks the current declaration for an existing name, and advances that scope if necessary. No scratch is reused between packets; there is no secondary resource tracker.

Scratch is allocated solely by the existing GraphDeclarer→current VulkanGraphSession's ResourceSpec::Buffer. The adaptation pass declares shader reads of both inputs and a full shader write of scratch; the raster's final invariant registers scratch indirect read2/group1. Exact actual aligned descriptor ranges are declared, including the few conservative prefix bytes required by minStorageBufferOffsetAlignment. Shader push offsets are relative to those same ranges. BindlessParameters::bind_pass validates these ranges and existing command ResourceOwners retain all native Buffer leases, heap and pipeline through GPU completion. No ctx.retain_object-only memory escape is used.

The immutable platform shader is supplied as readable GLSL plus generated Rust SPIR-V words, with reproducible offline compiler command and hashes. It is explicitly labeled host adaptation, never original game shader evidence. ShaderHost caches this one pipeline per current App heap and includes its ticket in ordinary explicit prewarm; the frame callback uses try_get and never waits.

The frozen production descriptor budget is unchanged. This adds real scratch/parameter leases and can expose an actual budget error; it does not suppress one or blindly enlarge the heap.

## Integration files

Apply from `games_test/tiny-glade` with -p1, reviewing against current shared sources:

1. `indirect_count_spirv-module.patch`, `indirect_count-module.patch`, `rg-mod-export.patch` add the narrowly scoped current-rhapsody capability utility.
2. `render_host-count-adapter.patch` connects that utility to the same ShaderHost heap/compiler and prewarm ticket set. It touches only ShaderHost fields/new/method/prewarm, not GraphPasses or root's UniformBytes guard correction.
3. `raster_host-count-adapter.patch` lowers unsupported packets through the actual frame declarer before its ordinary draw admission and VAO preparation. Native count packets are unchanged.
4. `count-multiframe-strict-gpu.patch` extends the existing actual typed-VAO GPU test; no additional fake renderer or scene is created.

Original core nani/draw_ext and gpu_generate_draw_lists are read-only in this delivery. Shared source integration is root-owned. `candidate-manifest.json` records each patch hash and non-mutating applicability check.

## Strict real-GPU regression

The existing `actual_typed_vao_vertex_index_and_instance_count_change_gpu_pixels` now removes the mode8 skip. One persistent original command Buffer contains two live commands at nonzero offset16 and stride32 for all five count frames. Its tail deliberately remains nonzero. A same-graph **actual GPU fill** changes the same physical counter at offset8 through2→1→0→2→3 with capacity2. The first and second triangle's actual pixels must disappear/reappear exactly as count dictates;3 must clamp to2. A raw-max substitution fails the1/0 cases.

Each graph contains two actual count packets: the first consumes the changing GPU count at offset8, the second consumes a GPU-written zero at offset12. Both reference the same persistent source commands. On unsupported hardware the declaration must contain two distinct scratch resource IDs and names, and compiled pass order must contain exactly two adaptation passes while preserving the GPU counter producer. If scratch were reused, the zero packet would erase the first packet’s commands and the nonzero-count pixel assertions would fail. Native hardware must include no adaptation pass. Existing kind3/4/5/7 cases remain. The normal compile→execute→actual wait/readback path and ResourceOwner lifetime are used for every frame. Root must record compilation, strict Vulkan execution and validation output before claiming this adapter ready.
