# Original roof spire producer — current B

Full native ranges, hashes and complete PDB aliases are in `native-manifest.json`. The original B RNE identity is `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

`place_roof_spires@142196e70`1943B has3 ECS inputs, independently established by `run_unsafe@1419ee460`532B: `Res<GraphicsDeviceResource>`, `Query<(&Roof,&RoofAnimation),With<system_roof::utils::components::RoofSpire>>`, `Res<AssetSsboLibrary>`. Current safe library `get_mut` requires `ResMut` as a host API adaptation. The previous PublicWalls / RoofSpireRecords resource inputs were unsupported substitutions.

The original library lookup precedes collecting query records (@1421970d5..148). Missing buffer or failed storage downcast follows a warning/return branch. RNE142ff4208 contains the45-byte original warning `ssbo_library doesn't have RoofSpireSsbo ready`;142ff4260 contains the original log target. The implementation uses that message and the module log target. It never creates a substitute buffer.

The original iterator is `filter_map` (`Vec::from_iter@140dce380`778B; query iterator driver@140dcdb00 705B), with actual business closure@1419ee9e0 337B:

1. Read RoofAnimation second f32 channel at+4.
2. Reject a non-pitched Roof through Roof::ty@1408e3250.
3. Read RoofRidgeDims::from@1408e2690. Compute both reciprocal ratios and the numeric max with the observed SSE NaN policy. Accept only ratio<`f32::from_bits(0x3f8147ae)` (1.01f32); `ucomiss threshold,ratio;jbe` also rejects unordered results.
4. Compute pivot+tip_offset_x0y+heightY using the existing recovered Roof methods and the original f32 evaluation order. The height*0 products and final w=0 are retained.
5. Emit the real32B RoofSpireSsbo: position float4, second animation channel, Roof profile@2c, radius0.25, pad1=0.

At1421971f6 the empty vector skips upload. A nonempty vector is bounded by the actual buffer byte size, then uploaded with a single full range at offset0 (@1421971fb..25a). Capacity or backend failure unwrap-panics before the separate count store at142197260. Empty success writes count0. The adapter reuses `TypedStorageBuffer::gpu_write` and the current real device contract, retaining this timing and failure behavior.

The previous96B position-only record and disconnected RoofSpireRecords resource were removed. The original query marker identity, existing schedule consumer, current renderer/device ownership and rhapsody architecture are retained.

Tests are explicit CPU fixtures through the actual producer body/library/factory: `native_spire_producer_filters_roofs_and_uploads_real_records` checks circle vs flat/long-ridge/unordered filtering, the real32B payload, second animation channel, w=0 and empty upload skip. `native_spire_count_is_written_only_after_successful_upload` injects a real trait error and over-capacity records, observes panic and preserved prior count, and checks missing-library return. They do not substitute fake GPU or session startup success. Parent compilation/GPU validation is separate; source hashes and filters are in `applied-source-manifest.json`.
