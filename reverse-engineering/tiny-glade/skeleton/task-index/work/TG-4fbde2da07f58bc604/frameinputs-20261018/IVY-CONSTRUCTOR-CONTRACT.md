# Ivy constructor and identity correction — current B

This is a two-file candidate, not yet applied during v30/v31 source freeze. Both file families were unclaimed, then legally claimed by frameinputs-20261018: leaf/flower `TG-4fbde2da07f58bc604`, segment `TG-f76c1e9798c4ffc62f`. The complete original6 functions, aliases and byte hashes are in `native-constructors-manifest.json`, bound to B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

| Original canonical type | Add-new / Typed ctor | Element / capacity | Descriptor size | Actual NAME |
|---|---|---|---|---|
| `systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo` |1412bc970 /141616120 |48B /10000 |480000 |`Ivy Leaf Buffer`15B@142c7f8e0 |
| `systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo` |1412bf920 /141615f50 |48B /1000 |48000 |`Ivy Flower Buffer`17B@142c7f8cf |
| `systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo` |1412c28d0 /14161a3c0 |48B /40000 |1920000 |`Ivy Segments Buffer`19B@142c8006d |

All3 Typed constructors are451B; each passes usage0x22, memory1, alignment4 and no initial payload on None. Their add-new wrappers are627B and prepend independent instance_count0 to the40B typed buffer before storing the48B InstanceBuffer. Some data allows exactly the declared capacity (`setb count,cap+1`); the byte API validates whole48B rows/pointer alignment4 and forwards Some(empty) distinctly from None.

Current errors were independently verified in source: all3 NAMEs were full Rust type paths instead of the descriptor names; actual definition spellings were Ivy*Gpu, failing the original canonical type identity; Flower used10000 rows instead of1000, Segment used dynamic BUFFER_LEN0 instead of40000, and generic fixed constructors synthesized zero uploads on None. The existing NAME-hash “gap” comments were therefore stale implementation errors, not unresolved hashing behavior.

The candidate defines original Ivy*Ssbo structs, reexports Ivy*Gpu as the same identity for compatibility, supplies original NAME/capacity constants and a shared safe byte-boundary factory through current APIs. Original shader binaries are read directly from the installed compiled-shaders directory, with hashes and SPIR-V members/offsets retained in `original-shader-layout.json`: Leaf/Flower0/12/16/28/32/44; Segment0/12/16/28/32/44, all48B. Pod derives cover those explicit fields without hidden padding. The schema correction does not independently prove CPU fields are filled correctly.

The existing CPU `pack_leaf`, `pack_flower`, random selection and segment normal implementation are retained. Their inferred seed and selection logic still require the original master/closures; no current algorithm is certified by these constructor tests. This candidate also does not implement the full72 init_populate producer or invent an ECS registration.

Recording-device tests call the real library and factories, verify independent count0 vs buffer id987, None descriptors and original type names/alias identity. Boundary tests use actual typed rows, verify inclusive capacities and Some(empty), reject excess before device creation. Existing constant-only tests are replaced by actual SsboId hash comparisons to original immediates. Candidate SHA256s are in `candidate-manifest.json`. Only rustfmt was run; parent owns compilation/GPU validation.
