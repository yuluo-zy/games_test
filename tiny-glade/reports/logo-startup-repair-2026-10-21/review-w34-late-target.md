# W34 late target independent review — 2026-10-21

Scope: read-only original-evidence / current-source audit. No Cargo, GPU run, or production edits. Existing startup UI capture predates this repair and does not cover it.

Result: no remaining P1/P2 defect found in the narrow target repair as currently written. The previous implementation wrote both late lists into GBuffer; the repaired core / SceneRasterHost / world_graph now carry the distinct forward scene-color attachment through all three layers.

Original identity was rebound independently: original PE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; all 14 world-followup PDB-bounded function bodies match their manifest hashes, including late 0x1408373b0/2177 bytes, body SHA 3bc1d9c13cc6c655be35f89ae374791047c6f9c70b7044fa6d5e95b7604ca335. Assembly hashes match after CRLF→LF normalization (not raw checkout hashes). Details: review-w34-late-target-sha.json.

## Original argument and attachment proof

- render-world @1404ec56b loads r15 from caller stack argument [rbp+4f8]. Main @1404ec582, OC2 @1404ec601, and late @1404ec648 all pass that same r15 as r9. It is a different parameter from MainPassTargets r8=r12; the initial prologue r15 held dimensions but was replaced at 1404ec56b. Thus the target identity is not inferred from the prologue register name.
- late @1408373d6 saves incoming r9 to r14. If Deferred is empty, @1408373f7 jumps to Forward with that value intact.
- Nonempty Deferred saves r14 to [rbp+a80] @140837400, temporarily reuses r14 for r8 @140837416, and places that GBuffer pointer into tag0 descriptor @14083742f. Descriptor reaches new_impl @140837496.
- Following Deferred rendering, @14083773e restores the incoming r9 value and @140837745 restores r8. Forward @14083777d emits tag0 and @140837787 writes restored r14 into the attachment descriptor, consumed by new_impl @1408377e7. This proves Forward must use the fourth argument in both empty/nonempty Deferred branches.
- Both paths use depth r8+0x24, layer0 / no-clear descriptor; extent comes from r8+0x28. Both require stencil_ref from list item +0x64/+0x65 and apply state.byte1=(byte1&0x8e)|0x21 after bind_common. No evidence supports changing depth/stencil while repairing the color target.

## Current three-layer repair and dependency path

- country-core main_pass.rs:221 onward now takes scene_color. Deferred uses tag0 targets.gbuffer; Forward uses tag0 *scene_color. Empty-list guards remain; depth_write_load(targets), depth viewport, flag=false, ItemMode::Stencil, variant0 and extra=None are unchanged.
- tiny-glade scene_raster.rs:110 forwards scene_color without substituting a target. world_graph.rs:144 supplies attachment(color), the same color supplied to main/OC2 and subsequently passed into world_lighting. GBuffer remains a distinct texture created at world_graph.rs:125.
- d_main_support.rs:155–183 maps tag0 to ColorAttachment::Texture, and DepthSpec::Write to writable depth/layer0/Load. draw_stream RasterPass produces tag0 color access 0x36 and writable depth 0x2f, group1.
- raster_host.rs:1319 recognizes the LOAD attachment's prior defined producer; 1461–1465 uses add_preserving_resource_access_range. This is a preserving write edge retaining previous contents, not a write-only overwrite that could discard earlier Forward scene color.
- raster_host.rs:1515 onward resolves actual texture handles. Color tag0 is Attachment::color(..., None), depth has clear=None plus ReadWrite; existing attachments.prepare_for_rendering uses the current rhapsody Dynamic attachment path. Native attachment owner retention and existing graph-range/sync handling remain in place. The repair changes only which actual texture ID participates; no raw resource/owner implementation was added.
- Stencil packet requirements and with_stencil_write remain in d_main_support.rs:271–297. No depth/stencil source changes are present in the W34 diff.

## Verification limits

The amended existing CPU fixture checks distinct GBuffer vs scene-color IDs for Forward-only and Deferred+Forward cases, both tag0 LOAD, and no depth clear; I inspected but did not execute it. Host source proves the intended dependency and native attachment mapping. It does not prove GPU validation, image output, or nonempty LateForward / LateDeferred business producers at runtime. A real branch-covered run or focused GPU fixture is still needed before calling W34 empirically closed.
