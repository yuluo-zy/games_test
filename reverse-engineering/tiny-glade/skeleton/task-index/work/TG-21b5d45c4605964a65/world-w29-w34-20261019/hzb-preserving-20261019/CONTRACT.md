# W26 SPD counter preserving-content correction

B-only evidence; no C-equivalence or complete-world acceptance claim. Root delegated continued work under the stopped child agent's original claim; the lock and original owner remain unchanged. No production source, Cargo or GPU operation was performed by this author.

## Original facts and present adaptation

The retained original shader package `_generate_hzb.cs.hlsl_cs_main_0.bin` SHA-256 30811cb17132c7da1a9b9e7eabb10bb8f95b2fa597c103af76b8c1184c9ebaff contains stage0 at byte23, 15072 u32 words /60288 bytes. Stage0 SHA-256 232d44e7cbc423304888ebf633614023f2e271771a6fddc42d05a0d091843e2d matches the retained shader catalog. It was extracted from the original BIN, not recompiled HLSL. An initial extraction interpreted its vector length as bytes, spirv-dis rejected it as truncated, and that result was not accepted; the corrected full-module extraction disassembled successfully.

`atomic-cfg-bounded.spvasm.txt` retains exact disassembly line numbers. `spdGlobalAtomic` is %19, set0/binding3, its uint counter is within a 24-byte record. %93 loads the first push-constant member `mips`. At original lines9189..9196, mips<=6 branches to %100 and returns. Otherwise local invocation0 executes OpAtomicIAdd on the counter at lines9204..9206; the returned old value decides the final workgroup. That workgroup resets the counter to0 at lines9230..9232 and performs remaining reductions. Thus the binding reads existing contents only on the mips>6 path.

The B PDB procedure record33612 binds generate_hzb@0x1409286f0 to a complete2666-byte range, ending exclusively0x14092915a. `native-manifest.json` binds full RNE/PDB identities, complete decoded range and raw byte SHA. Original native0x140928c1b loads binding hash0x068d4413b4de4f24. The current host's legacy synchronization code0x37 and shader dispatch remain intact; the new preserving flag expresses actual content dependency in the user's current graph architecture.

## Real producer to consumer

world_graph::build_world creates main depth through the same CoreGraphFrame and calls SceneComputeHost::generate_hzb after main_pass; the returned HZB feeds W27/W28/grass and later visibility. Current generate_hzb calls existing spd_atomic_buffer -> PassEnv temporal_buffer -> render_host::GraphPasses::get_or_create_temporal_buffer -> current session.named_buffer_zeroed. That session creates and uploads an actual zero slice once when the named24-byte allocation is created/resized. Its real resource owner records `initial_contents_defined`; temporal declaration applies it separately from confirmed history. No new zero producer, default resource or fabricated initialized import is introduced.

The candidate binds the same atomic RgBuffer, then calls existing preserve_storage_buffer_contents(HASH_ATOMIC) iff the exact pushed mips-minus-one value exceeds6. RecordedComputePass.preserving_buffers propagates through render_host's declared access branch to add_preserving_resource_access_range. That branch retains the same graph resource, access0x37 and range and therefore preserves its true prior initialization/producer dependency. Missing or revoked physical contents remain subject to the existing strict graph guard.

## Lifecycle, errors and cost

Name/reuse,24-byte owner, history publication, resizing and cancellation remain owned by the existing named-buffer/session implementation. Unsubmitted cancellation preserves actual uploaded initialization without confirming history; reset/partial submission revokes the existing initialization fact. This patch neither clears history nor invents recovery contents. Existing errors propagate to the already repaired render_frame Result boundary; no OOM policy changes are included.

The change adds one existing per-pass content-intent vector entry only for the atomic-consuming path and one comparison at declaration. It introduces no GPU pass, full-buffer upload, CPU readback, locks, waits or extra temporal allocation. Actual performance has not been measured by the author.

## Verification boundary

The candidate contains one narrow CPU declaration test across push values0/6/7/10, checking only the counter is preserving above6, that the original access sequence remains0x1a/0x19/0x37/0x37/0x37, and that one pass/one actual named-buffer request remains. Existing tests retain exact dispatch/push/view expectations. Author did not run Cargo/GPU. Root should run the affected existing core compute-pass tests once source is integrated; any full nonempty-world two-frame GPU acceptance remains a separate task. The prior generate_mips large-SPD GPU evidence validates the shared owner mechanism, not this unexecuted HZB consumer correction.
