# Integration handoff

1. Root may apply `candidate.patch` to the legally held roof_shape.rs family. Its base and resulting SHA are in source-map.json. Existing runtime APIs, geometry algorithms, component identity, and raw field owner remain intact.
2. Root handles `integration-owner.patch` on the current manual valid_enclosure/mod.rs, which has no separate indexed task family. It is required so the reused existing ShapeParameters serializes nonzero Circle rotation correctly. Its captured base/hash is separate and this patch does not edit utils.
3. history_audit owns the real capture adapter: serialize the current Vec<Roof> with serde_json::to_value, keeping the snapshot key `Roofs` and actual query identities/order. No new registration or second component collection is necessary. There is no Roof Deserialize addition in this patch.
4. Run the six new tests through root's unified Cargo validation. They are presently source-only. Existing Gable public identifiers stay Matching/Override but their JSON names become Stepped/Overhang; old wrong JSON names are rejected by the existing derive Deserialize.
5. The original B payload is 88 bytes, but no repr(C), transmute, unsafe pointer ownership, native enum Rust layout declaration or raw-array JSON was added.

The existing pattern_id()->u8 observer remains a compatibility surface over the material byte. Serialization correctly uses u32@+8; do not describe this patch as acceptance of that legacy API or of the full Roof module. No original execution was run or retried.
