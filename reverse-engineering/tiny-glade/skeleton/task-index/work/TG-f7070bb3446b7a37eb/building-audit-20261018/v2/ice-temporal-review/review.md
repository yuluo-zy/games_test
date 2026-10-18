# Ice history-valid failure: independent read-only review

Conclusion: stale test expectations, not a production inversion. Minimal patch changes only two expected resolve-valid values from1 to0 and corrects their comment. No production edits, compilation or tests by this reviewer.

The failing run's surface_tests::run constructs a new RecordingRg each call, with an empty temporal map. get_or_create_temporal_texture_pair sets legacy fresh=known.is_some(), so the newly allocated water refr temporal and ice refl ray len pairs both have invalid history. surface_ice correctly writes resolve_valid=(refr_temporal.fresh & ray_len.fresh)&1=0. The separate input history_valid still controls march/trace directly; their expected1/0 values are unchanged. The second run with input0 creates another empty RecordingRg and also expects resolve0.

The field name fresh is retained for API compatibility; its documented meaning is reusable valid history. Current game host and raster host project pair.history_valid() directly. Current rhapsody distinguishes allocation reuse from committed initialized contents; this review preserves that existing architecture and does not revert it to an allocation-only cache.

Original B independently checked:
- get_or_create_temporal cached descriptor match writes flag1 at0x1404f702e.
- new/resized allocation writes flag0 at0x1404f72d6.
- pair combines lookup flags at0x1404f7605..0x1404f760f, stores result+0x4c at0x1404f7696.
- ice_surface_pass loads first pair bit at0x1409c1f9e, ANDs second at0x1409c2052, masks bit0 at0x1409c279b, and writes the history uniform at0x1409c27be.
Original retained temporal-single/pair assembly and the ice decompilation were read; these specific instructions and the entire ice code range were independently extracted from original RNE and compared with current B patched PE. All five ranges match. static-identity.json and machine-byte assembly are retained here. RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.

Delivery: ice-history-valid-test.patch, surface_tests.rs, manifest.json. Patch check passed. No claim on the render file group was acquired; this candidate is for the existing integration owner. No Nani source touched. This corrects the fresh-start test expectation only; warm history, submission, resize and cancellation remain covered by their actual temporal-owner tests, not proved by this RecordingRg fixture.

