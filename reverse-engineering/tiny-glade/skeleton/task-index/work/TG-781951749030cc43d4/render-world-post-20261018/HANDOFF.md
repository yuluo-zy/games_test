# B original world-post candidate handoff

The minimal W51 actual-uniform closure is ready for root integration and strict GPU validation. No production file was changed by this delivery. All native contracts are in `POSTPROCESS-CONTRACT.md`; hashes/base comparisons are in `candidate-manifest.json`. All patch paths are now normalized to the tiny-glade workspace root (`patch -p1`); these hashes supersede the earlier unnormalized headers.

Apply the minimal independently testable group in this order:

1. `candidate/uniform-bytes-carrier.patch` — root compute builder scope TG-c7c4edf038d495b5b1.
2. `candidate/uniform-bytes-physical-host.patch` — root current-host integration scope, existing allocator/session only.
3. `candidate/dof-real-uniform-and-partitions.patch` — owned TG-781951749030cc43d4; exact padded32B, slot-count push7, corrected native indirect order, narrow content dependencies, true typed three-buffer result.
4. `candidate/scene-post-compute-host.patch` — actual W48/W50/W51 adapters, no fake world.
5. `candidate/dof-tile-offsets-strict-gpu.patch` — append-only existing GPU target; root should run test `original_multires_dof_reads_uploaded_tile_offsets_and_dispatches_its_real_partitions` with the same strict Vulkan/layer/loader settings as v37.

The broader caller group is separate:

- TG-40c65c5fc31e4c20ff/render-world-post-20261018/`volumetric-in-place-color.patch` preserves actual old color.
- TG-6561ac1c9e7d6ac76a/render-world-post-20261018/`spd-exact-content-intent.patch` preserves actual source mip0 and the conditional atomic contents. First atomic producer still unresolved; do not fake it.
- `candidate/world-postprocess-module.patch` creates the complete typed W47..53 caller. Add `pub mod world_postprocess;` to the shared tiny-glade lib when integrating. Caller owns main FrameSetup and real W1..46 outputs.
- `candidate/world-post-typed-parameters.patch` fixes old render_world caller semantics. Only current FramePasses implementor is its CPU Rec; both signatures/calls/Rec tests are migrated together. It removes the non-existent mip-count argument8, uses f32 sharpen, and names photomode separately from the global multires switch. No unknown opaque payload is fabricated.

The full production world entry is not enabled by these patches. Necessary remaining boundaries include the preceding actual geometry/transparency chain, W52 first-history handling (negative SPIR-V evidence, not a false history_valid inactive exemption), W54 debug/capture behavior, and the actual M3 indirect-count capability/command-tail contract. The current fullworld Unsupported must be removed only when those necessary paths have true producers/consumers.

Current source candidates were syntax-parsed with rustfmt. No Cargo/GPU was run. Root must check compilation and the actual GPU fixture before reporting a successful closure. In particular test assertions must not be weakened to accept zero output or missing partitions.
