# Explicit original undefined-history reads in the current graph architecture

Candidate only. All thirteen candidate files parse; all thirteen patches pass a nonmutating applicability check against the shared source. This worker did not edit production, invoke Cargo, or run the GPU. Syntax/applicability checks do not establish compilation or behavior. Root must compile and run the attached CPU and real-GPU regressions.

## Original facts and the present blocker

The six complete packaged TAA/TGSR shader programs, their original package/stage SHA256 and full SPIR-V are retained in `TG-781951749030cc43d4/render-world-post-20261018/taa-original-spv/manifest.json`. This delivery reuses those exact programs. In original TAA, the five history samples `%198/%211/%222/%233/%245` execute before the `history_valid && FrameConstants.frame_index != 0` selection; that selection chooses current color on the first frame. TGSR reproject likewise performs five real history samples before multiplying by validity. Undefined/NaN multiplied by zero is not proof of an inactive read. Calling either binding inactive would falsely remove actual memory reads and existing producers.

The four complete native ranges and code/assembly hashes in `../first-history/manifest.json` prove the actual allocation boundary. The 610-byte pair function at1404f7450 calls the String temporal creator at1404f6850 twice and chooses parity; it has no initialization call. The parallel reference creator at1404f6e50 exposes the same allocation shape: at1404f714d, TextureInitialData is tag0 (None), then create_texture is called at1404f7187. In complete create_texture2794B@1405ba5f0 the tag is retained, checked at1405bab96, and tag0 jumps past the unique init_texture_data call at1405babe0. The String instantiation is retained too, so the pair's true callee is covered. No hidden history clear has been inferred.

Thus the original program permits a real read of undefined newly allocated history texels. This differs both from a known-disabled shader read and from a defined-content producer. Current strict graph initialization correctly rejects ordinary uninitialized reads but needs an explicit, bounded caller contract to represent this original behavior. It should not invent a black history, mark an import initialized, or remain permanently Unsupported at the first real frame.

## Additive capability and invariants

`AccessDeclaration.allow_undefined_contents` defaults false. `PassBuilder::add_undefined_content_read_range` admits only a nonwriting access with a valid same-graph handle and valid exact range. It retains the original access code, synchronization group, range, read version and resource owner.

`reads_contents()` remains true. Liveness/dependency analysis, conflict detection, barriers, usage derivation, resource lifetimes, physical binding and command owner retention therefore retain their existing behavior. An available full or partial writer remains a required dependency. This flag only removes the requirement that every read cell already have defined contents. It does not write, publish, or initialize those cells. A subsequent ordinary read or export is still rejected unless a real writer has defined the requested range.

Both initial resolution and dynamic boundary revalidation on a cached definition honor the same flag. The independent plan checker separately permits missing content cells only for this explicit real-read flag, while collecting all available writer provenance. It still rejects forged writing, preserving or inactive declarations carrying the flag. Its ordinary source check remains strict. Subgraph input/output boundaries remain unchanged: a real input must still be declared through the ordinary port interface.

The twelve-byte original access ledger has no such bit; this is current-architecture host metadata, not an original ABI claim. Alias merge treats an ordinary active binding as requiring defined contents even when another alias permits undefined contents. An inactive alias imposes no content requirement. A write upgrade clears the permission. Derive Eq/Hash and copied normalized accesses retain the bit; compiler schema version advances3→4. There is no alternate backend, tracker, allocator, global validation toggle or default bypass.

## Game consumer boundary

The existing core ComputePass stores exact image-binding indices in `undefined_texture_bindings`. `allow_undefined_texture_contents(hash)` resolves an actual nonwriting ImageView, checks that it is not inactive, and carries that metadata into RecordedComputePass. GraphPasses maps the existing ImageView/access occurrence to the real graph resource and exact subresource range, then calls the new graph API. No placeholder resource ID or copied history image is added.

Original `calculate_taa` and `calculate_tgsr` retain their shader, uniform, binding and dispatch order. TAA history and TGSR-reproject history receive the permission only when the actual temporal `HistoryTexture.valid` is false. Subsequent valid-history reads use the ordinary strict API. The historical `TemporalTexturePair.fresh` field already means `history_valid()`, not allocation freshness; it is not inverted.

All four TGSR-main permutations actually gate `aux_tex` sampling by their computed history-valid uniform (and frame index). When that uniform is zero, this separate aux binding is marked inactive through the already existing capability. TGSR main's optimized-out history binding is not invented. First-frame TAA/TGSR history remains a real read; it is never marked inactive.

This candidate covers these explicit W52 readers. It does not apply a blanket exemption to SSGI/ReSTIR, SPD atomic counters, arbitrary temporal images or ordinary resource reads. Any additional original undefined read requires its own complete producer/consumer evidence.

## Integration and all literal migrations

Apply numbered patches from `games_test/tiny-glade` with `-p1`, reviewing shared-source drift rather than overwriting a whole candidate file:

- 01..05: current RG declaration, ledger policy/alias merge, PassBuilder API, both initialization paths, compiler version plus borrowed-cache regression and the one explicit all-fields AccessDeclaration literal.
- 06: core ComputePass new/state/finish and the additive binding-index method.
- 07: current GraphPasses image occurrence/range mapping only. This patch does not touch root's UniformBytes write_lease fix or the independently delivered count adapter.
- 08: one RecordedComputePass literal in compute_frame.
- 09: all four RecordedComputePass literals in tiny-glade/tests/render_host (including the existing inactive-alias and UniformBytes regressions).
- 10: the two actual original AA consumers and the separately proven TGSR aux inactive intent. This file is claimed as TG-24952c5dd502f4529d/render-world-post-20261018; root integrates the reviewed delivery and coordinates any external core owner.
- 11: independent plan-checker provenance and mode validation.
- 12: five current-architecture integration tests, not new game logic.
- 13: one strict original-shader GPU fixture appended to the existing original_scene_compute_gpu harness.

A full bounded search found no other RecordedComputePass literals carrying inactive_texture_bindings. Default-derived AccessDeclaration constructions need no source mutation; the one explicit exhaustive initializer in plan.rs receives false. Source-before SHA256 and each patch/candidate SHA256 are in candidate-manifest.json. validation.json records parsing/applicability outcomes, not build/GPU success.

## Validation requested from the sole runner

CPU integration target `rhapsody --test undefined_contents` covers: ordinary rejection versus explicit admission for both created and external uninitialized textures; unchanged sampled usage/lifetime; no publication/export/later-read exemption; partial-writer retention and read→write hazards; both alias orders; forged conflicts; full Eq/Hash and normalized scope-cache distinction. The additional library test `undefined_read_permission_does_not_bypass_cached_ordinary_boundary_validation` exercises the actual private borrowed match from its owning unit module, so no API visibility is expanded for a test. It proves both cached and owned validation distinguish ordinary versus permitted access after a defined import changes to initial_access0.

GPU test `original_taa_and_tgsr_read_first_undefined_history_then_use_real_previous_output` uses the existing fixture's same CoreRenderBridge/device/libraries/FrameSetup. For TAA and then TGSR it executes two frames with actual untouched temporal pairs, asserting false→true validity. It only initializes explicit diagnostic current color/reprojection/depth inputs, never history. It checks actual history remains bound/read, has initial_access0 on the first frame, and only that frame uses the new permission. It requires all original AA passes retained, actual submit receipts and center-pixel original-resolve readback of constant red on both frames. The expectation allows only original packed-float/UNORM rounding, not arbitrary pixels. It exercises the existing pipeline/prewarm/execute/wait/resource-owner path. Root must preserve the full strict Vulkan output and report any real first-use format or shader defect rather than clearing history to hide it.

No whole-world success or game-ready state is asserted by this bounded fixture. The preceding production world consumers and following W54/compile_main connection remain independently required.
