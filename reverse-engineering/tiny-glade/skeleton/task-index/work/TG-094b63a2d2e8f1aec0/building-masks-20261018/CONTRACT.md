# B canonical mapped mask constructor repair

Candidate only. Target B despite the shared index's old C default for two file families; no C address, layout or validation is imported. Legitimate owner building-masks-20261018 holds Water TG-094b63a2d2e8f1aec0, exclusion TG-a0fb89a9f7cc9770b9 and Garden TG-91fe8570b8ea4df799. Root alone integrates production and runs Cargo. No JIT/native execution attempted.

Original B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 / matching PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Reused complete identity-bound constructor evidence from startup worker; independently checked raw PE hashes and disassembled every instruction (normalizing trailing whitespace only), then copied the evidence into this claim. Three1203-byte mapped constructors and three627-byte add_new wrappers total5490B/1272 instructions. Original source/hash metadata, copied asm and raw binary are retained in evidence/native-inputs.json and its referenced files.

| Current canonical type | Constructor | NAME pointer / length | add_new key |
| --- | --- | --- | --- |
| WaterMaskSsbo | 14161d570 | 142c7f8ae /9 WaterMask |4652455297dab420 at1412c2eaa |
| WaterExcludeClutterMaskSsbo |14161c080 |142c7fe38 /23 WaterExcludeClutterMask |8dbc70d368e43c84 at1412c182a |
| GardenMaskSsbo |14161f660 |142c7fa28 /10 GardenMask |6fd8113d23595a99 at1412c56da |

All three original constructors: Some initial input checks byte length divisible4, pointer aligned4, then rejects length>=0x8004. None allocates0x8000 bytes aligned4, fills all bytes zero and supplies that complete initial payload to create_buffer. Buffer descriptor is size0x8000, usage0x22, memory4, alignment4. They then invoke actual mapped_slice_mut (Water14161d716, corresponding offsets in the other two), require a non-null map with bytes>=descriptor size, validate map pointer/length alignment, and divide the returned byte length by4. The returned mapped span may exceed8192 words if the device returns a larger map, as in the existing generic wrapper; it is not silently truncated to the descriptor capacity.

Current bugs: Water's Gap is not a defined u32 element and its compatibility pass hash is0. All three names have an incorrect Ssbo suffix and all three capacities are0x8000 elements instead of bytes. The incremental patch corrects Water to repr(transparent)u32+Zeroable+Pod, changes the existing canonical names and BUFFER_LEN to0x2000, keeps their existing MappedStorageBuffer storage factories and sets compatibility pass hashes from the same canonical NameHash as SsboId. Exclusion/Garden receive that small trait adapter without a second type. No generic buffer or new registration engine is introduced.

## Real consumer trace

The current production TerrainReadbackSsbos already selects these exact three resource types. read_terrain_data::read_water_mask writes Water and exclusion in one /water/read_water_mask.cs.hlsl pass; read_garden_mask writes Garden. Both dispatch0x2000 threads, matching512*512 cells packed32 bits per u32. Their calls use LibrarySsbo and AssetSsboLibrary::get_rg_buffer, so corrected names/storage immediately flow through the existing canonical library. The old compatibility PassSsboType is not mistaken for this real library identity; both now agree on the original keys.

A workspace search found no second actual CPU reader of these typed mapped buffers or consumer-created default0x8000-word span. terrain_hover_idx's TerrainWaterSample commentary mentioning Gap is stale: its real CPU water input is WaterRaster; it does not read WaterMaskSsbo. No other owner or its hand-written sampler is rewritten here. Actual GPU submission/completion and frame-to-CPU world mask acceptance remain separate root/host work; this constructor patch does not pretend that merely declaring passes executes GPU readback.

## Regression source and validation

The two tests in Water's module exercise all three existing typed library factories. None checks original names, descriptors, complete zero payload, typed length8192, and actual device map pointer identity; it mutates the last u32 through the canonical typed storage. Some preserves explicit supplied bytes and the fixed mapped capacity. Original immediate add_new keys validate both actual SsboId and compatibility pass hash. The shared MockDevice's initial-data recording and map are separate test observations; the test does not claim a simulated GPU upload copies into that map.

The second test registers these canonical types and calls the actual read_water_mask/read_garden_mask consumers via the existing pass test environment, checking8192-thread dispatches and CPU mapped write access0x32. The shared recording mock returns the same buffer id7 for every allocation, so this fixture explicitly assigns distinct observed handles101/102/103 in the existing Buffer fields before graph import. This is test-only handle setup, not production state or a new engine. It does not run a graphics backend.

All three candidate files parse/format with rustfmt. Combined owned-files.patch passes git apply --check. No Cargo, Rust test execution or new native differential run occurred. Root must execute these regressions after independent review/integration. Existing broader WaterMask constants/shaders, world raster algorithms, library and mapped-buffer implementation stay authoritative.
