# R5C temporal CPU schema/state

Target SHA256: f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. Four new decompilations/xrefs and original instruction ranges are retained here; parity/export and compile consumers reuse batch01/02 evidence. RE-MCP connected to tg_render_arch, completed the four queries, and closed successfully with save:false. Target was not launched. PDB void() is not a recovered ABI.

## Observed

- Keys are ustr::Ustr, not a custom 64-bit hash: public symbols identify 0x140c0f640 as From<&str>, 0x140c0f650 as From<String>, and 0x140c0f6f0 as Ustr Display. Both String/&str texture instantiations intern names before lookup. This dependency is reused at original version 1.0.0.
- Temporal record stride is 0x68: key +0, discriminant +8, active logical payload +0xc, retained physical resource +0x30 (0x38 bytes). Raw texture=0, raw buffer=1, exported=2. Exported access byte is +0xc. Physical texture kind byte at record+0x66 differs from buffer sentinel7.
- Missing buffer/texture calls create and imports physical handle with access0; inserts raw variant only on success; result byte0 means newly created, byte1 reused, byte2 error. Single buffer result byte at +0x18, texture at +0x24.
- Hit must be exported2. Raw0/1 means same-frame duplicate and returns error before any create/import. Exported physical kind must match texture versus buffer, otherwise returns error.
- Buffer hit imports physical resource with stored access; no buffer descriptor comparison appears. Texture hit compares extent bytes descriptor+0xc..+0x17 first. Changed extent creates a replacement (including informational logging); failure leaves old exported record unchanged. Equal extent requires equality for words +0/+4/+8/+0x18 and bytes +0x1c/+0x1d/+0x1e, else panic. Descriptor+0x1f padding is not compared.
- Success imports existing exported physical resource and writes raw0/1 logical payload, leaving retained physical resource untouched. New texture replacement inserts only after create/import and drops old physical value via managed reference/drop helpers.
- export_temporal iterates all map records: raw0 calls export_texture 0x140bd6380; raw1 export_buffer 0x140bd5120; access0x37 for both. It writes exported2/access/payload, retains physical block, and skips exported records. No parity flip, no wait, no physical recreation occurs here.
- Pair interns the base name, formats exactly base:0 and base:1 (binary string slices at 0x142ab5e88/0x142ab5eb0), and calls texture single constructor twice with the same descriptor. First error short-circuits; second error preserves first mutation. Both success then flip base-key parity. Pair byte+0x4c is first.reused AND second.reused. Result discriminant+0=0 success/1 error; success payloads at +4 and +0x28, each0x24; parity1 swaps them.
- Parity state+0x30 is a separate Ustr/u32 map, bucket stride0x10. Missing stores/returns0; hit stores/returns1u32.wrapping_sub(old). Separate pair keys do not affect each other; export does not reset parity.
- Compile 0x140166510 takes graph0x90 and marks unavailable, takes temporal at persistent state+0x90, zeroes the temporal-present field, passes it through closure recorder, then restores temporal +0x90..+0xef before registry calculate/populate. The populate-failure graph restore is separate from temporal restoration; this is not a temporal transactional rollback.

## Inferred reconstruction

Rust candidate uses original Ustr1.0.0 and matched original-lock registry versions/checksums. TextureDesc names fixed_words/extent/fixed_word_18/fixed_bytes deliberately preserve known compare groups without inventing Vulkan semantics. Physical and logical handle operations remain an external Backend boundary; no allocator, hash/intern implementation, Vulkan, or country-core PassResources creation closure is reproduced.

Five texture frames and six pair frames are exercised by CPU oracle tests. Other tests cover buffer no-comparison, duplicate request error, resize replacement/failure preservation, mismatch panic, kind mismatch, missing failure and partial pair failure. These are candidate semantic regression tests, not full original-entry/GPU equivalence.

## Unknowns / limits

Exact physical wrapper ABI and descriptor semantic field names, pair/reset API beyond the observed path, temporal clear/drop consumers, unused-key eviction, and compile closure-error unwind remain unresolved. A buffer-pair generic API is not established by this texture-pair monomorph; no buffer pair is invented. No cross-frame Vulkan or dynamic scene trace was performed.
