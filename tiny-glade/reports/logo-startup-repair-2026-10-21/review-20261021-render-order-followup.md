# Independent render-order/state follow-up review

Date2026-10-21. Read-only review of loading MEI shadow flag, T11 LOAD and T12 CLEAR through current tiny-glade host and existing rhapsody attachment lowering. No Cargo/GPU execution, no source edits, no business module changes, no new bottom-library implementation.

## Conclusion

No must-fix P1/P2 found in the three changes. They match the retained original B evidence and reach the existing graph-access and native attachment paths. This establishes static/source closure only: the current release's real saved-background/960-window run is still central runtime acceptance. It does not assert that T11 or T12 was exercised merely because normal startup produced gameplay frames.

## Evidence independently rebound

Input original `reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe` SHA256 matches f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. Independently parsed PE sections and rehashed all15 address/size bodies from `tiny-glade/reports/logo-startup-repair-2026-10-21/render-state-evidence/manifest.json`:15/15 match exactly. The16 bytes at VA0x142a9cc30 are exactly `01000000ec51383eec51383eec51383e`, matching the constant manifest.

All15 assembly files were checked. Raw file hashes differ from the recorded hashes because this Windows Git checkout uses CRLF; normalizing CRLF→LF produces15/15 exact recorded assembly hashes. No binary-byte normalization used. Rebinding details/current raw hashes: `D:/game/tmp/review-20261021-render-order-followup-sha.json`.

## 1. Loading MEI has no shadow combine

- Original compile-loading.asm @140165347 is `xor r9d,r9d` immediately before call1409c6700 @14016534a. Callee mei.asm @1409c6726 copies r9d to r12d; @1409c6aff tests r12b and branches zero to1409c6bc8. Thus this exact caller supplies false for the actual shadow flag.
- Current loading_graph.rs363–366 calls `ui_2d_pass::mei_pass_with_shadows(...target,None,extent,text)` and no longer creates an unused "mei temp" graph texture. No loading EscMenuState-derived shadow policy is substituted.
- Core ui_2d_pass.rs331–392 chooses target tag0 when tempNone, declares real MEI texture/buffer reads, renders text directly, then returns before the shadow shader lookup/temp sampling/combine. Existing gameplay caller still derives its own actual EscMenuState shadow flag; it was not globally disabled.
- Direct target tag0 uses LOAD and preserving access, so preceding Vermeer composition remains a real content dependency. No missing temporary owner or stale texture slot is introduced.

## 2. T11 session previews LOAD prior target contents

- Original session-previews.asm @1408f83b0 stores tag0 at rbp+7c0, @83ba stores target reference, @1408f8404 passes this descriptor in r9 to RasterPass::new_impl @1408f8411. Nearby stack7f00 tag2 is the separate depth-none representation and must not be confused with the color descriptor.
- Current session_preview_host.rs519–528 passes `[ColorSpec::tag0(target)]`, no depth, same target viewport; existing thumbnail pool/owned texture bindings/consumer mode gates remain unchanged.
- DrawPassCtx::raster_pass(d_main_support.rs155–183) maps Tag0 to `ColorAttachment::Texture`. RasterPass::new(draw_stream.rs593–620) emits access0x36, group1, color tag0 with original sample count.
- RasterHost raster_attachment_accesses(raster_host.rs2305–2332) independently verifies/reconstructs tag0→0x36/load=true and exact view. Projection1250–1330 rejects mismatched access/target/group, computes current physical view range, then sets preserving_attachments when an actual overlapping previous producer exists. Submission1460–1481 calls `add_preserving_resource_access_range` for that load/write. Thus preview quads retain the preceding final_post/menu target producer in the graph's content chain; no “LOAD only in Vulkan but write-only in RG” gap observed.
- Record callback1515–1533 creates actual Attachment::color(no clear), sets load from tag0 (Vulkan LOAD), retains current managed texture/view. The normal final_post→menu→T11 target already has a producer; a genuinely undefined prior target remains a graph/content validation issue, not initialized by a fake background.

## 3. T12 LUT gallery CLEAR RGB0.18/alpha1 reaches native GPU attachment

- Original lut-gallery.asm @1408b0b13 loads xmmword from RIP+21ec115 = VA142a9cc30. It stores tag1 and RGB bits0x3e3851ec at rbp+790..79c; @1408b0b23 writes alpha0x3f800000 at rbp+7a0; target reference at+7a8. This is a color-clear payload, not a subresource view or unused numeric metadata.
- Current world_postprocess.rs960–975 deliberately bypasses ColorSpec::tag1's defaultzero payload and uses canonical `RasterPass::new` + `ColorAttachment::View { view:[0x3e3851ec;RGB],alpha0x3f800000,texture:target }`, no depth, original target viewport. Every bit is retained; no approximation0.2/gamma-converted fill or shader replacement.
- ColorAttachment::View tag=1, access0x2e; RasterPass::new copies clear words into RasterPassColorAttachment.view; pass.render sends them even if no draw packets. LUT name stream always includes initial config; empty positions also do not suppress the actual pass/clear.
- RasterHost marks tag1 load=false/access0x2e, so this is an independent write. It intentionally does not keep old target contents or force T11/final_post producer as a LOAD dependency; this matches original CLEAR replacing the backdrop.
- At actual record(raster_host.rs1515), `(c.tag==1)` converts the preserved words with f32::from_bits and passes Some(clear) to existing rhapsody `Attachment::color`. It sets load `AttachmentLoadOp::from_raw(1)` = CLEAR. Native Attachment owns real texture Arc and actual first-mip view/sample count. `prepare_for_rendering`~1578 validates/retains these owners and `RhRenderPassApi::begin_prepared`~1597 issues actual attachment rendering, including zero-draw clear.
- Existing rhapsody attachments.rs60–82 stores VkClearValue float32 with desc.clear_input; dynamic-rendering lowering~455 uses attachment.desc.load and exact attachment.clear in VkRenderingAttachmentInfo. The current rewritten PreparedAttachments enum has only Dynamic; this review does not add or assume a legacy render-pass implementation. This is source evidence of end-to-end CLEAR plumbing, not a newly measured GPU pixel result.
- run_lut_gallery's actual gates remain demo=false, EscMenuState::Photomode(true), filled real optional preview slot, actual loaded configurations/LUTs. Missing preview still returns before creating the pass, as original. This change does not make an absent producer look present.

## Menu background and ordering

Menu-bg original @1408c9d3b stores tag2 before new_impl @1408c9d92; current menu_background_host.rs249 remains ColorSpec::tag2. No inappropriate global conversion to LOAD/CLEAR was made.

World tail still performs final_post→menu background→painterly→Vermeer→actual reveal/UI gate→T11→T12→UI2D/MEI→loading list/T21. These local attachment changes neither move a pass nor bypass reveal/demo/preview policy. Per-pass resources remain owned by the same enclosing CoreGraphFrame and current App; the modifications create no additional cross-frame graph IDs or owner handoff.

## Limits

No runtime log/pixel result is attributed to this read-only review. A normal startup run can exercise loading MEI, but T11 requires its actual menu/session branch and T12 requires real gallery preview state; source completion of those branches is not proof of runtime traversal, LUT thumbnail correctness or full game visual equivalence. Existing render-host attachment tests were inspected as contracts and not rerun here.

