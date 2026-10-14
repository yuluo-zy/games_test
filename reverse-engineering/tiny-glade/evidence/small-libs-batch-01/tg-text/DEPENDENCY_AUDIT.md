# tg-text dependency audit

| Runtime dependency | Original version/source | Actual use | Reimplemented? |
|---|---|---|---|
| serde | =1.0.200 registry | FontTable/entry/family/shape/scaling Deserialize derivation, String deserialization | No |
| ron | PounceLight/ron 0.8.1 rev2787d8c resolved2787d8c05250c4dd706dfb358a2e9f16969d2968 | Original FontTable and ShapeStream RON format parsing | No |
| std Vec/HashMap/String | Original Rust compiler1.86 family; current build uses installed Rust | Ordered shape storage/raw words/font lookup/strings | No |
| serde_json (dev only) | =1.0.85 registry | Deserialize real native-oracle fixture for differential tests | No |

Original Cargo/PDB baseline: tracking/original-Cargo.lock, original font_table.rs and shape_stream.rs namespace ownership annotations. Parent integration audits all22 external packages in the3 new crates against original version/source/checksum; this crate uses shared lock without changing it.

Font metrics multiplication and word bit packing are game-private rules, not substitute font or container libraries. HashMap machine-code probing/SipHash/Vec allocator internals excluded. Raw u32 packing is the observed game-consumer protocol, not a new generic binary codec. RON/serde implementation and original generic monomorphizations are dependencies; only game schema/variant/default policy is analyzed.

Deferred: original cosmic-text0.11.2 fork commit3da2a14ea3d063875fdc55695431e74405302b0f, etagere0.2.10, fontdb0.16.2, rhapsody GPU resources. They are not currently required for this CPU scope and are not replaced with handmade shaping, rasterizing, atlas allocation or rendering. Follow-on Fragment/Renderer work must use them and recover only the private consumer/caching policies.

Tools: native_oracle.py uses Python stdlib ctypes/struct/hashlib only to extract/map original PE instruction evidence. It is analysis code outside the Rust crate and is excluded from business progress. No new provider installation or global MCP changes.
