# B W54 actual frame replacement and canonical host

This is a candidate delivery under TG-50b39e9844f539b049, owner render-world-post-20261018. Target index C remains unchanged; all observations and candidate behavior here are bounded to original B. No production edit, Cargo invocation or GPU run was performed by this worker.

`native-manifest.json` binds the complete PDB4231B range14098f9f0 and exact RNE-code SHA2569ed3a2dd5c4dda21708fb3e0bb9b6dbefc3ecfd8ed85b8b57e257b4c0003a0ac. The RNE is the already bound original B (SHAf507...7d03), not the patched analysis database. The native range includes its unwind tails. The two five-entry original jump tables are separately decoded in original-jump-tables.json; valid native discriminants are0..4. The previous handwritten commentary concerning out-of-range table values does not establish defined native behavior for an invalid discriminant, and is not used as a production readiness claim.

## Corrected actual output

Original14098fcf7 obtains the feedback temporal.14098fd09..29 saves its whole result's36B texture at `[rbp+10..33]`. For states3/4,1409905f0 loads its tail dword,1409905f3 writes `[frame+20]`,1409905f8/fd load the first two16B portions and140990602/609 copy them to `[frame+10]` and `[frame+0]`. This is **whole RgTexture replacement**, including actual logical resource ID and full description. It is not an unidentified description-tail edit.

The core candidate returns `replacement_frame: Option<RgTexture>` instead of the lossy bool `desc_tail_copied`. States3/4 return Some(feedback) on every phase, including phases with no capture/compare pass; states0/1/2 do not replace the frame. Original `inc eax` wraps payload, so the safe source explicitly uses wrapping_add. Native lookup/state transitions/pass order/names/binding access codes remain.

The strict CPU assertions now check the replacement's actual feedback ID and whole sampled/storage format/extent description, off-beat replacement, and payload wrap. All known bool consumers were local tests in this same source file and are migrated; the only new production consumer is the world_postprocess adapter in the companion final-world-post-candidate package. No second ECS state is installed.

## Reference-read dependency

The two packaged original shaders are retained in original-shaders/manifest.json with package/stage hashes, complete SPIR-V, instruction walks and offline spirv-val success. `capture` is a real source read and reference image write. `compare` uses OpImageRead on `reference_tex` and samples the rendered capture; it writes only feedback. Its formula is referenceRGB*.08*(0,1,1) plus red1 iff any absolute channel difference exceeds1e-5.

The original CPU code binds reference through legacy write32. Current source must retain that exact access code while expressing the actual contents dependency: only compare's reference binding receives preserve_storage_texture_contents. This uses the already integrated core→current GraphPasses→current PassBuilder metadata; it does not blanket-classify32/37 as RMW or rewrite rhapsody. Without it a previous capture writer can be pruned or newly created reference content can be falsely treated as a pure write.

## Current host and mutation timing

The companion public `world_postprocess::render_shader_refactoring` gets the canonical ShaderRefactoringState (same existing Resource alias). It errors on a missing owner. Native disc0 returns immediately after that mandatory lookup: no change tick, temporal allocation, shader request or extra PassResources lookup. Active states use the existing with_env/FrameWorldAccess/current CoreGraphFrame and actual original core entry; all returned passes are actually recorded.

The candidate commits the actual canonical state at graph-declaration completion before compile/submit, matching the native state-write location after pass declarations. A later compile/submit failure does not introduce a new rollback policy. Allocation/pipeline construction errors before the native commit retain the original state. The returned actual texture replaces `taa` before WorldPostOutputs is formed.

## Bounded GPU fixture and limits

`w54-strict-gpu.patch` appends `original_shader_refactoring_replaces_the_full_frame_and_keeps_real_reference` to the existing real scene GPU harness. It uses actual original capture/compare shaders on the same bridge/device/FrameSetup and actual single temporal owners. Phase0 checks disabled no allocations/unchanged frame; phase2/payload3 captures an actual GPU reference and keeps the real input output; phases3/payload3 compare same and changed red input against that persistent reference and require the full feedback resource/description as output. It requires both real pass names and reads actual center pixels through original resolve. The green/blue expected56/69 come from original .08 tint, packed R11/G11/B10 rounding and the original sRGB resolve; ±1 covers UNORM conversion only. It is an explicit diagnostic phase fixture, not a claimed gameplay state-machine shortcut.

There is a separate original undefined-output contract in active debug phases: states3/4 off-beat phases replace the frame with feedback even before the first compare has written it. The original B allocation has no invented black fill. The current strict graph may still reject a later exported/sampled undefined feedback in those early debug phases; the W52 explicit read policy is not silently extended to exports or raster bindings here. This does not affect the real startup/default disc0 path. The active-debug undefined output remains a separately identified current-architecture capability requirement, not evidence for clearing, dropping or substituting the frame. The attached fixture uses a real reference capture before comparing, so it does not manufacture that undefined branch's contents.

Syntax and applicability checks are recorded in ready-manifest.json. Actual compile/strict Vulkan/runtime results must be recorded by root before claiming this candidate validated. The whole game still needs all earlier scene stages and the original compile_main outer image/UI/final-post consumers.
