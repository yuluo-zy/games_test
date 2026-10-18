# Roof secondary buffers — current B

Applied under the parent integration authorization after v29. Original RNE SHA256 is `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`; the four complete function ranges and hashes are in `native-manifest.json`. Original shader member offsets/types are independently saved in `original-shader-layout.json`.

| Canonical original type | Typed ctor | Element / capacity | Actual descriptor | NAME | None |
|---|---|---|---|---|---|
| `resources::roofs::roof_buffers::RoofSupportBeamSsbo` | `14161a590`,451B | 80B / 131072 | size `0xa00000`,usage `0x22`,memory1,align4 | `RoofSupportBeams` | no upload |
| `resources::roof_tile_ssbo::RoofSpireSsbo` | `141616860`,451B | 32B /1000 | size32000,usage `0x22`,memory1,align16 | `RoofSpearTransforms` | no upload |

Their original `add_new` functions `1412c2650` / `1412bdd70` prepend an independent zero count and allocate a48B InstanceBuffer. Neither original None edge means initialized zero elements. Some data is bounded by the respective fixed capacity, with count derived from data length; the current byte API also checks whole elements and pointer alignment before forwarding the original byte payload. No rhapsody architecture changes are involved.

Support shader fields are transform48B@0, circle_normal float4@48, roof_profile f32@64, pad0 i32@68. The final8 bytes are opaque words because the original80B constructor proves their stride but their CPU meaning is not established. The descriptor alignment remains4 even though the current Vec4 representation aligns the host record to16. Explicit field coverage prevents uninitialized Rust padding in the Pod representation.

Spire shader fields are float4 position@0, f32 roof_animation_t@16, f32 roof_profile@20, f32 radius@24, i32 pad1@28. Its original CPU producer is independently recovered in task `TG-6cc17f0d3d4c72a243`, including filtering and real GPU upload.

`RoofTileSsbo` was moved to its original canonical `resources::roof_tile_ssbo` path. The previous `resources::roofs::roof_tile_ssbo` file now only reexports that same identity; existing fields/algorithms were preserved. The shared full type-name check now sees the original path, and a regression checks alias TypeId equality. This path correction does not claim independent revalidation of every RoofTile CPU producer.

The current B PDB contains a `place_roof_support_beams::closure$1` declaration at source line95 but the available extraction has no compiled procedure or exact inline host for it. The similarly named `system_wall_constructor::construct_lintels::place_support_beams` is a different business domain. No active roof Support CPU producer was invented from that name. This remains unresolved code attribution, rather than a recovered zero-output system.

Validation: rustfmt only at delivery. `native_roof_instance_constructors_preserve_empty_counts_and_no_upload` uses an explicit CPU recording device through the actual library/factories, checks count0/None descriptors/NAME hashes/size and the canonical alias identity. The parent owns the subsequent compilation and GPU runs. `v30-integration-manifest.json` records all7 changed sources and SHA256 hashes. The complete72 registration is still separate work; these constructors alone are not that producer.
