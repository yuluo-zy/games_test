# Three precise constructor corrections

Work candidate only. No production edits, Cargo or GPU run. Original RNE SHA and complete native function ranges were rehashed against the retained pack; original-evidence.json carries those ranges plus the independent actual constant read.

- DofAutofocusSsbo: None enters14161025c, allocates0x200B; movaps141610283 reads128-bit constant142ac7e60 = four0x41f00000 words, then32 stores through14161034a fill the complete512B. All128 word bits are30.0f32. Preserve Some bytes, native size/usage22/memory1/align4 and existing byte-input guards. Do not change Default for this struct's other consumers.
- ShadowMaskHistogramSsbo: None enters1416121dc, allocates0x400B; xorps141612203 and64 stores through1416123a6 produce1024 zero bytes. ShadowMaskMeanSsbo is deliberately unchanged: native141613720 None carries no payload. Never apply one None policy to both.
- PebblePlacementSsbo: native141610ddd branches directly to141610e39 for None, with no default allocation or upload; fixed0x6c00B buffer remains. Current factory incorrectly used TypedStorageBuffer::new(None), which uploads zeros for capacity>1. Route all byte inputs through the already-correct fixed-byte constructor instead. Some byte-input behavior is retained.

The two existing constructor tests with contrary None assumptions are corrected to original data evidence. One focused Pebble test covers both absent and supplied data. Relevant filters:

- systems::render_loop::compute_passes::dof_autofocus::ssbo_layout_tests
- systems::render_loop::compute_passes::apply_screen_shadows::ssbo_layout_tests
- native_pebble_none_stays_uninitialized_and_some_preserves_input

Rustfmt parsed/formatted candidate files. Nothing in this candidate modifies generic storage defaults, sizes, graph access or gameplay algorithms. Root must merge narrow hunks in coordination with the separate systems/resources owner and run the above scoped validation; there is no prefilled PASS.
