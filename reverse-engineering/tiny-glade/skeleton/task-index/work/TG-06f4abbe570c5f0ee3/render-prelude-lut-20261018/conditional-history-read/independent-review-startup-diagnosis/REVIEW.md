# Independent read-only review: inactive contents capability

Reviewer: startup_diagnosis. Scope: current RG candidate, companion host mapping, retained original Reprojection shader. No production edits, Cargo/GPU execution, shared DB mutation, or country-core systems/resources edits.

## Findings

1. **Build finding in the initially reviewed patch, subsequently corrected by its owner:** integration test `inactive_contents.rs` called `GraphDefinition::matches_declaration`, which is `pub(crate)` in `rg/plan.rs`. This would fail privacy checking. Root independently identified it too. The current RG patch SHA `de3c9aa0684a6f0d8827873423bbd238864021d36a3b358bbc0239490458948c` removes that inaccessible test call; no API widening is needed. No compile has been run by this reviewer.
2. **No additional semantic blocker found by source inspection in the scoped capability.** Runtime acceptance remains pending root validation. The highest-value remaining counterexample is the host's mapping of active/inactive bindings to different views of one resource, described below; existing candidate tests cover RG Whole-range aliases but not that host occurrence case.

## Original gate independently checked

The installed shader package hash is `a3d324137aa9635ad63d6e752ae958cc5ffce89297ef143be64bb373a654b6c4`. Its exact extracted 17,860-byte SPIR-V at byte offset 23 hashes to `11486fe55c9db966a3bea31db51bd5b3ea554e57b426ecf2f07b8d717eb3f6d2`. An independent instruction walk consumed all 4,465 words and 717 instructions and found one OpImageGather (result241, word3604). The retained disassembly identifies prev_depth_tex as set0/binding15. Globals.member2 is loaded into212; zero takes the false branch to214, whose Phi220 is false on this edge; that skips222, the block containing the sole Gather241. The separate frame-index gate remains conservative. Hence `prev.valid==0` establishes no previous-depth texel read without removing its binding.

## Resource lifetime, usage and synchronization

The patch only changes contents provenance and producer-liveness conditions. ResourceInfo.calculate/extract_pass_resource_access still consume every retained declaration with its original code/group: usage flags, first/last access, pass lifetime and next-access chains remain. RangeProgram.compile still passes resource/range/code for every retained access into range_use and native-range combination; contents_unused is never used to omit a native use. Native layout restrictions, layout-change conflicts, read/write conflicts, multi-queue family admission and physical resource preparation remain on the original access stream. A retained inactive binding therefore still requires a real live resource/view and proper Vulkan access/layout state.

Content-only dead writers can be culled. This is intentional and safe for an inactive reader; if a later active reader consumes that content, it adds the original data dependency and retains the writer. Surviving accesses retain hazard ordering. The supplied dead-writer test specifically preserves a subsequent native read→write edge.

## Aliases and versions

For exact resource/range/version aliases, same-code/group dedup ANDs contents_unused. Either active alias restores the content requirement, independent of insertion order. The two existing read-code merge cases also AND the flag. Different ranges remain distinct, so an inactive view cannot suppress validation of an active view.

The original read19/read1a→write37 upgrade explicitly resets contents_unused=false and reads_previous=true on both resulting entries. This retains its prior conservative preserving semantics. It can reject an otherwise safe inactive-read→pure-write combination, but does not accidentally permit an uninitialized preserving write. The capability does not claim to relax that legacy case.

Inactive accesses do not update initialized ranges or write versions. Ordinary active later reads and exports retain their existing initialization requirements. Inactive-write or inactive+reads_previous forged metadata is rejected by both contract resolution and the independent plan checker.

## Cache and subgraph consistency

AccessDeclaration derives Eq/Hash with the new field. GraphDefinition clones full accesses, compares full access records in matches_declaration and bumps compiler version. Its dynamic import initialization check uses reads_contents and continues to check later active accesses and exports. Subgraph local-key construction clones full passes/accesses, then only normalizes ranges/indices/versions, retaining contents_unused. Local/global composition uses reads_contents for provenance while leaving storage-conflict history unchanged. Subgraph boundary validation exempts only the inactive content requirement; explicit version checks and output-port/lifetime requirements remain.

## Host occurrence mapping

ComputePass::bind_texture_with_view appends native resource access and ImageView together. RecordingPassBuilder simply pushes accesses and does not deduplicate. Host matching by (resource,write) with first-unmatched ImageView therefore preserves the order for values produced by this builder. The new marker resolves a shader binding via AccessRange and stores the actual binding-table index rather than a whole-resource flag. With normal append order, same resource + different hash/view can be individually inactive. Alias dedup later remains conservative.

A manually forged RecordedComputePass can break the assumed parallel order; that was already part of the current adapter's validation boundary, not a new trusted-input guarantee. No broader architecture change is proposed here.

## One meaningful missing counterexample

Build a real recorded ComputePass binding the same two-mip texture under two shader names: inactive mip0 and active mip1, with an unrelated buffer binding between them. Repeat in the opposite image-binding order. Without initializing mip1, declaration resolution must reject the active mip1 read even though mip0 is inactive. Initialize only mip1 and it must accept; the retained native range program must still contain both image views, SAMPLED usage and the full pass lifetime. This catches accidentally marking the resource, taking the first matching hash/range, or shifting occurrence indices through nonimage bindings. It is more diagnostic than adding another Whole-range RG alias test.

Acceptance: source-reviewed candidate only. Root must run the scoped CPU tests and actual first-frame original Reprojection/Copy-depth GPU readback; this review does not prefill a pass.
