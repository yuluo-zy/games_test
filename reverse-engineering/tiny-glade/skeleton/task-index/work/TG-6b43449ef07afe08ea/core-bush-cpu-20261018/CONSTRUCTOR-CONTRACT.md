# Bush CPU canonical mapped storage — B bounded constructor

Original 0x14161bd30, 835 bytes (includes cleanup tails) is identity-bound in original-inputs.json. 0x140eebaa0 is the existing upper consumer entry. No new original instruction runtime was executed for this delivery.

The current Gap / one-element TypedStorageBuffer cannot feed the existing CPU consumer. Keep the same canonical type and library owner, with repr-transparent [u32;2] (8-byte size, 4-byte alignment), 9601 rows, usage0x22/memory4, alignment4 and exact name `Bush positions for CPU` (22bytes). The first row is metadata; following rows store XZ f32 raw bits. Constructor None passes None through without a default allocation/upload. Other MappedStorageBuffer types keep their existing initial policy.

Some validates divisibility by8, pointer alignment4, then byte length<0x12c10. Device errors and missing map panic. Mapping validates len>=returned Buffer.byte_size (not requested size), then divisibility8/alignment4; retain the complete returned typed length. Use the current device-owned persistent mapping and AssetSsboLibrary, no second resource or CPU mirror.

Two Rust regressions exercise actual canonical AssetSsboLibrary/type key/map identity/no-upload and exact Some bytes plus early validation failures. Not run yet. GPU submission, waiting, coherency and actual upper readback consumer remain separate integration requirements.
