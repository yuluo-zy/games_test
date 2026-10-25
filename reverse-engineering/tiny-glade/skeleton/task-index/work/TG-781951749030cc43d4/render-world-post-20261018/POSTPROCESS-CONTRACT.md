# Original B world postprocessing, candidate contract

This delivery is a bounded original-first correction and current-host adapter. It is not full-world readiness, an A/B/C equivalence claim, or a GPU pass result. Production sources were not edited and Cargo/GPU were not run by this worker. All nine current source bases matched at handoff creation; see `candidate-manifest.json`.

## Original sources and bounded integrity

Oracle B is `ljxsj_92385/Tiny Glade/tiny-glade.rne`, SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`, with matching PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`. `native-ranges.json` records complete PDB extents, exact PE-code SHA256 and LLVM disassembly SHA256 for nine bounded original functions. No Ghidra database was opened or modified.

| Original boundary | Observed behavior | Candidate consequence |
|---|---|---|
| `calc_dof@140a5a570`, 12099B | Global14362c080 independently selects fused/multires | Host takes explicit `multires_dof` control; AppMode does not replace it |
| `140a5affd..5b035`, `5b094`, `5b0b6` | Copy seven prefix words; append zero until length multiple4; allocate/copy32B | Upload exact eight words via real DynamicConstants, not old28B/resource0 |
| `140a5b298` | Store slot count7 in4B push payload | Populate actual init shader push bytes, independent of dispatch thread count |
| `140a5bee9..5bf96`, `140a5bfe0`, `140a5c27e..c28a` | Fullres variants are declared lod0→lod1→full with slot bytes1,2,0, multiplied16 | Preserve declaration order, correct enum identity and indirect offsets10,20,00 |
| `generate_mips@1400fb890`, 2202B | Mip count comes from actual texture description+1c | Remove bogus caller `mip_count8`; native8 at render_world1404eccd0 is string length of `dof_mips` |
| `apply_volumetrics@1408a44c0`, 2009B | Uses real MainCamera mode and original color/depth | Host reads actual owner and keeps exact pass; in-place color content dependency is explicit |
| `calculate_taa@140a3b2f0`, 1891B / `calculate_tgsr@140a3ba60`, 6211B | Real FrameRenderConfig lookup, bypass byte+20, sharpen float | No zero camera/config default; f32 sharpen replaces old opaqueu32 field |

`fullres-native-slot-map.json` independently decodes exact original names/vtables/slot bytes. The packaged fullres shaders agree: full reads offsets[0], lod0[1], lod1[2]. The earlier handwritten order was wrong.

`original-spv/manifest.json` binds nineteen full packaged programs to extracted SPIR-V SHA256/instruction walk. `taa-original-spv/manifest.json` adds six actual TAA/TGSR programs. Every extracted program was walked to its declared word count and terminal OpFunctionEnd; the full disassembly and Vulkan GLSL translation are retained. GLSL is navigation, not a replacement shader.

## Real uniform ownership within the existing architecture

`ExternalSlot::UniformBytes(Arc<[u8]>)` is an additive **source host adapter**, not a claimed native enum variant. CPU declaration has immutable original bytes. `GraphPasses::add_compute` uploads on first use using its existing `DynamicConstants::push_slice`, checks reflected binding kind/device/usage/range, imports that actual allocation with the enclosing current session and binds its true range. The original `Other` branch continues to fail.

A small bounded64-entry `Arc::ptr_eq` declaration cache retains each exact source Arc and its actual chunk. The same source table is physicalized once per GraphPasses and reused across its reading passes. This is not a second allocator, descriptor heap, resource graph or persistent resource cache. DoF has one table. Physical lifetime follows existing graph imports, BindlessParameters and command ResourceOwners retention. There is no raw object-only retain substitute.

No new RecordedComputePass field is introduced. No existing struct literal migration is required. GraphPasses has one constructor literal, updated to initialize this cache; its public constructor signature is unchanged. UniformBytes is currently produced only by the original DoF initializer. Existing external Buffer/Image/Other users retain their paths.

## Content intent, kept narrow

These annotations express original shader work in the current strict graph. Legacy access37 remains pure-write by default, not a universal RMW inference.

- Volumetrics loads old color before writing fog color: preserve only the reflected color binding.
- DoF analyze atomically adds to the seven initialized dispatch counts: preserve that buffer, retaining the real init producer.
- Halfres variants write disjoint tile partitions after the real clear-halfres pass; preserve previous partitions. Fullres declaration order remains lod0→lod1→full. The first declares new output; later variants and upsample preserve predecessor partitions. No artificial clear is inserted.
- SPD imgSrc is readonly storage despite legacy synchronization37: preserve exact mip0 view. Its inter-workgroup atomic path is entered only when the original push `(mip_levels-1)>6`. Preserve that existing atomic content only then. **No first-frame atomic initializer was found in this helper; no fake zero/import-initialized is added.** Above this threshold an absent actual producer must remain a strict graph error.

## Continuous typed caller, W47..W53

`world_postprocess.rs.candidate` is a complete callable declaration segment using one `CoreGraphFrame`, the actual App World, the actual DrawLists and all typed WorldLightingOutputs. It performs original shadow-mean → volumetrics → DoF prepare → optional SPD/whole DoF → TAA/TGSR → post-TAA transparency, returning the three real native outputs `{depth, coc, taa}`. It does not submit, install owners, nest FrameSetup, make empty draw lists, or set gameplay readiness.

Required real owners are GameConfig.video, AppMode, ImageFormationConfigs, FrameRenderConfig, MainCamera; the six existing FrameWorldAccess libraries/real device/backend; FrameConstants and frame setup globals; canonical ShadowMaskMean/Depth/DoF autofocus/SpatialResolveOffset/ColorLibrary buffers; and the original registered raster shader and real disk blue_noise. The enclosing world graph must supply real geometry, reprojection, color-history pair and W44..W46 results on that same frame. Some library inputs are lazily imported by the core consumers, not pre-imported wholesale.

Type identities are confirmed by fresh original producer extents: ImageFormationConfigs insert_resource140122c30 / register_resource140222cb0 contains4a142e09642375b9 +43f6154cfa8c847f; AppMode register_resource140224bc0 containsc2b8e941c91a6428 +59f3c6d1d3da43a0. Photomode tag0x13 chooses `photomode_dof_enable` and the shader quality byte. Bokeh is the original active configuration's float+30 (`current().params_30[0]`), not an invented GladeTheme parameter. FrameRenderConfig's actual display extent8 feeds TGSR; actual flag20 feeds bypass clamp. Two process debug controls have no Default and must come from the real host state.

This segment is not yet a proved first-frame execution closure. See the following counterevidence. W54 debug/capture consumer follows it and is not silently declared recovered. Full compile_world_graph must continue to reject missing actual consumers/owners rather than running a prefix and pretending ready.

## W52 first-history counterevidence — do not apply a false inactive exemption

The current TAA helper binds a newly allocated previous history with read19. In the actual TAA SPIR-V, five history texture samples occur **before** the history_valid/frame_index decision. Its false branch discards their value, but it does not prevent the samples. TGSR reproject similarly samples history first, then multiplies by the valid flag; undefined/NaN contents cannot be assumed harmless (`NaN*0` may remain NaN). The four TGSR main permutations do guard the aux_tex fetch with history_valid and frame_index!=0, but that proves nothing about the preceding reproject pass.

Accordingly this delivery adds no TAA/TGSR false inactive metadata or history clear. Their first-history producer/undefined-allocation contract remains an actual integration question. The existing narrowly proved reprojection inactive path must not be generalized to these shaders. This worker legally claimed TG-24952c5dd502f4529d for this bounded audit but intentionally provides no unsupported leaf patch.

## Strict GPU fixture, queued for root

`dof-tile-offsets-strict-gpu.patch` only appends helpers and one test to the existing original_scene_compute_gpu target. It uses the existing real Fixture/CoreRenderBridge and packaged blue_noise, canonical SpatialResolveOffsets default producer, real FrameConstants, and explicit diagnostic inputs64×32 with CoC0. It then records the whole original nine-pass multires branch on the normal frame/session.

Before execute, all nine names must survive compilation. Actual GPU readbacks assert offsets `[0,32,64,96,104,112,120,0]`, seven dispatch vectors (slot2=32, slot6=8, all other x0; y/z1,w0), slot2's32 distinct8×4 coordinates and slot6's8 distinct4×2 coordinates, and the original resolve output of constant linear RGB(.25,.5,.75). Only written coordinate subranges are read. This verifies GPU consumption of the actual uploaded tile offsets and correct indirect slot identity, not a CPU Vec comparison. It is a bounded diagnostic case, not a recovered game scene.

Only rustfmt syntax parsing and non-mutating patch applicability were requested/performed by this worker. Root owns compilation, strict GPU execution and integration; record those results separately before claiming closure.
