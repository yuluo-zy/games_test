# Descriptor-binding framework consumer

Target SHA256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. Existing R7 and batch05 files were left unchanged. The current five-step module workflow is documented in `evidence/render-recovery/AGENT_EXECUTION_GUIDE.md`; the earlier R8 work package is a migration source, not a substitute for shared-render integration.

## 1. Module and caller evidence

The existing configured RE-MCP/Ghidra service was connected through the new official-SDK runner. Seven focused module/consumer functions were read together: sparse layout `0x1418c1a10`, bind specializations `0x1416a5380` / `0x1416a6010`, shared write constructor `0x140b06020`, persistent texture update mapper `0x140a96740`, persistent bind `0x140bee150`, and descriptor lookup `0x140bff400`. Catalog, actual connection, identity and successful close(save:false) are preserved in `mcp-report.json`. The exclusive session was released immediately; no global config or full-program auto-analysis changed.

Ghidra did not recover the shared write constructor switch and its sparse-layout pseudo-code reads adjacent code/data incorrectly. `pe-boundaries.json`, bounded disassembly and original jump tables are authoritative for these branches. Sparse layout PE entry/body is `0x1418c1a10..0x1418c20a9`; the write constructor is `0x140b06020..0x140b066b2`; compute sparse bind body is `0x1416a6010..0x1416a64ee`. Workgraph/PDB ranges include unwind fragments in some cases; the implementation does not treat them as primary body instructions. Saved database xrefs are incomplete and are supplemented by prior original direct-call evidence; negative xrefs do not prove missing callers.

The native compute specialization's PDB association identifies country_core compute_pass_builder dispatch_impl closures. Original instructions at `0x1416a6245..0x1416a628e` consume a `(BindingIndex, PassResourceSlot)` tuple and then enter actual membership, write, update and bind consumers. No catalog-to-ordinal adapter was used for this path. Actual shader selection/reflection creation upstream remains unknown.

## 2. Types, state and error semantics

Sparse layout consumes HashMap iterator entries with a 32-bit BindingIndex key and 40-byte reflection Binding value in a 48-byte bucket. The key is written directly to VkDescriptorSetLayoutBinding.binding; sparse holes remain holes. Set number is separate: it indexes 120-byte DescriptorSetState records, and is passed as firstSet to vkCmdBindDescriptorSets. A set index beyond pipeline set count returns without allocation, update or bind.

The compute mapper's tuple occupies eight bytes: BindingIndex u32 at +0, PassResourceSlot u8 at +4. It indexes a resolved 32-byte DescriptorSetBinding array. Out-of-range slots panic **before** membership filtering, so an undeclared binding key does not hide an invalid slot. The safe Rust API exposes that failure as SlotOutOfBounds; original panic unwind is not reconstructed. Duplicate tuples retain their order and can produce repeated writes. Duplicate sparse-layout bucket keys are only malformed raw-iterator fixtures: a valid HashMap producer cannot emit duplicate keys.

The shared write constructor consumes `{binding:u32, array_element:u32, resource_pointer}`. Resource tags differ from reflection tags: 0 sampled image, 1 storage image, 2 sampled image array, 3 storage image array, 4 combined image sampler, 5 storage buffer, 6 uniform buffer, 7 acceleration structure (pending), 8 dynamic uniform buffer, 9 dynamic storage buffer. Dynamic offsets append in consumer traversal order. Image payloads preserve sampler/image-view/layout; buffer payloads preserve buffer/offset/range. Arrays retain their caller-supplied descriptor count, including zero in raw fixtures. No Vulkan validity or caller reachability is implied for raw unusual values.

Membership checks only the declared key, not reflection type/count. A mismatched resource class is passed through to the API; the Rust consumer does not invent a private type-checking failure. Six class-mismatch fixtures carry an annotation describing the mismatch; native declaration class is not loaded by this membership consumer. Unsupported tag7 and missing payload errors are explicit safe-candidate boundaries, not claims of original panic behavior.

After successful selection, vkUpdateDescriptorSets is called even for zero writes; the device update counter wraps/increments once. vkCmdBindDescriptorSets follows with the actual command handle, pipeline bind point, pipeline layout, set index, allocated descriptor set and collected dynamic offsets. Original per-command DescriptorSetState allocation is an injected private boundary in entry verification and remains unresolved. The candidate owns image/buffer backing vectors through update, then releases them after callbacks return. API callbacks panic normally; no recovery/rollback is synthesized. Full command/managed-handle GPU ownership is outside this module.

Layout type/count/array flags, `_dyn` names, immutable sampler provider and runtime count rules reuse frozen R7's verified private logic and original ash. Pipeline push extent remains an explicit selected-interface field, used as COMPUTE/offset0 optional range; it is not guessed from descriptor types.

## 3. Whole Rust module

`reconstruction/render/work-packages/descriptor-binding-r8/src/lib.rs` provides a single consumer preparation API:

`prepare_compute_pass(interface, target, tuples, resources, device_limit) -> PreparedComputePass`.

ComputePipelineInterface carries descriptor sets with sparse BindingIndex/type/count/name/sampler requirements and push_constant_bytes. ComputeBindTarget carries actual command/pipeline-layout/descriptor-set handles and selected set. The entry derives set_count and declared keys from the selected interface, produces sparse layout and persistent pool plans, maps real slot resources into writes, and returns bind_plan plus context. Consumers invoke `prepared.bind_plan.execute(&prepared.context, counter, update, bind)`. The layout/pool plans expose original ash create-info callbacks; ComputePipelineInterface.with_pipeline_layout exposes the COMPUTE push-range create info.

`compute_pass_bind_plan` is also available when the pipeline/interface has already been prepared. No fixed binding ordinal, fixed sequence, manually generated VkWriteDescriptorSet or shader inventory adapter is required by these consumer APIs. Typed shader-interface selection, resolved resource handles/layouts/ranges, sampler cache, DescriptorSetState allocation and actual Vulkan remain explicit inputs/providers.

## 4. Wiring and concentrated validation

The root integration owner owns shared `render/lib.rs`, runtime resource types and record callback. This module supplies its consumer API for that callback and does not edit shared files concurrently. Its independent crate checks are not reported as completed shared integration. Shared callback execution and any chain acceptance are reported by the owner separately.

Existing concentrated original CPU tests are frozen at 3,785 cases, all zero differences: sparse layout 1,975; actual write constructor 864; enumerated bind entry 300; native compute sparse tuple mapper through write/update/bind 640; key-only mismatch behavior 6. CPU emulation enters complete native bodies, but supplies public allocator/hash/BumpVec/TLS arena boundary mocks, external sampler and private descriptor-set allocation callbacks; real Vulkan calls are captured only. Slot errors stop at the original panic entry; secondary persistent chained bindings, allocation failure/unwind and GPU are excluded.

Both native and Rust verification read actual API pointer structures and nested image/buffer/offset data; plan JSON alone is not the oracle. The 1,975 sparse fixtures use real raw-iterator key records, not catalog enumeration. Source/harness/dependency/oracle/input/target hashes are retained in checksums.json. All 21 registry dependencies source/version/checksum match the original Cargo.lock. R7 is reused as a frozen source dependency. Two unit tests, locked offline build, fmt and strict clippy pass. The prepare_compute_pass adapter composition is candidate framework glue; its shared-callback acceptance belongs to root's integration, not to the native-entry count.

## 5. Current blocker

Actual compiled shader/interface **selection producer** and its binding-to-game-pass association are not recovered. Next evidence must follow the shader-library lookup/compiled shader passed to country_core dispatch_impl into these descriptor-set maps, including actual immutable sampler descriptions and graph-resource handle producers. The recovered compute tuple mapper closes the sparse consumer; it does not prove that any of the 638 shipped declaration records belong to a particular live pass. Shader_pipeline/Spirq generic internals are not reverse engineered. Full shader module/compute/raster pipeline creation, acceleration structures, persistent chained bind input, managed descriptor lifetimes, real Vulkan and game-frame equivalence remain pending. No duration or throughput estimate is invented.
