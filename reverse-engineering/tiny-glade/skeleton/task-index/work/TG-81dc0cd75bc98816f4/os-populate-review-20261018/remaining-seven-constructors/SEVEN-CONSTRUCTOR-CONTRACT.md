# Remaining seven constructor contracts — current B

Read-only original evidence, not a production registration or completed CPU producer. `native-manifest.json` retains14 complete RNE ranges and aliases, bound to B SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. Actual descriptor NAME bytes, addresses and add_new immediate hashes are in `native-names-and-hashes.json`. The shader observations below come directly from8 installed original binaries, not hand-written HLSL; member types/offsets and source hashes are retained in `original-shader-layout.json`.

| Canonical element contract | Original constructor | Element bytes / capacity | Buffer bytes | Align | NAME / hash | Stored owner |
|---|---|---|---|---|---|---|
| spawn_boats::BoatSsbo |14161a020,451B |64 /4096 |0x40000 |16 |`Boats` /4f69c51e83e1d670 |Instance48,count0 |
| stairs::StairsTransformSsbo |141612cb0,579B |48 /41000 |0x1e0780 |4 |`Stairs transforms` /c405372d6efc32d3 |Typed40 |
| compute_water::LilypadSsbo |141615d80,451B |64 /50625 |0x317040 |16 |`Lilypads` /321bc3dfb10d5554 |Instance48,count0 |
| compute_water::LilypadFlowerSsbo |14161a1f0,451B |64 /50625 |0x317040 |16 |`Lilypad Flowers` /ae37cbcafbe88da4 |Instance48,count0 |
| compute_water::ReedSsbo |141617be0,451B |48 /22500 |0x107ac0 |4 |`Reeds` /d6d72e989e1b8586 |Instance48,count0 |
| compute_garden::Flower03Ssbo |141618fd0,451B |16 /160000 |0x271000 |4 |`Small lowpoly flowers` /f7c8c9c2eb9e16fb |Instance48,count0 |
| compute_garden::GardenStoneSsbo |141619ab0,451B |64 /32400 |0x1fa400 |16 |`Garden Stone` /45a92b71a96e360f |Instance48,count0 |

All descriptors use usage0x22 and memory1. Every original None branch forwards no payload and performs no zero initialization. The6 typed constructors with typed slice inputs accept count<=capacity. Stairs accepts bytes, checks len%48=0 and pointer%4=0, rejects len>=0x1e07b0, and has no independent InstanceBuffer count; its add_new allocates0x28 rather than0x30. All wrap/propagate create-buffer failure through original unwrap panic. No default content is implied by a count0 or by a Rust Default implementation.

The instruction sequences proving element sizes must be retained: shl6 for Boat/Lilypad/Flower/GardenStone; shl4 for Flower03; shl4 followed by lea*3 for Reed. Stairs divides byte length by48 and uses the original fixed size separately. Native capacities are constructor element counts, not draw counts or buffer ids.

Original GPU schemas are separate facts. BoatXform and LilypadXform use dense48B Affine3Packed in original compute shaders. Their native CPU buffer constructor uses64B allocation elements; this is an original capacity difference, not permission to shrink the allocation to48B or infer a fourth meaningful SIMD column. The current host can preserve64B opaque creation rows and the actual descriptor while executing the real shader's dense48B accesses. Reed and Stair both have48B `xform`/`transform` compact affine records, and Stairs compute and raster agree. Flower03Xform is position float4@0. GardenStone is transform48@0 plus color float4@48. CPU meaning of unconsumed Boat/Lilypad allocation words remains unknown.

Ownership: Boat/Stairs were legally claimed by frameinputs-20261018 with the root's current full-scene/72-constructor authorization recorded for deferred scope. Water/Garden are owned by render-water-live/render-garden-live; their evidence is only handed to that owner, no file or claim is taken over. Current source has zero-sized carriers and missing library factories for these water types, and one-element macro carriers for Boat/Stairs/Flower03/GardenStone. A correct factory is necessary for complete init_populate, but does not establish those passes' full producer/consumer scheduling. No partial subset is labelled the full72 producer.
