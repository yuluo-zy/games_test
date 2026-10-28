# W23 / W25 / W30 / W34 / W43 render state follow-up

2026-10-21. Scope: original draw order, attachment identity/load policy, depth/stencil state, and the main terrain/grass/wall/roof callbacks. T13 remains explicitly excluded. This is a continuation of `render-order-state-audit.md`, not a claim that every W leaf or gameplay scene is verified.

## Result and validation boundary

**One new P1 found and narrowly repaired: W34 Late Forward wrote GBuffer instead of the original scene/HDR color.** The original fourth argument had been removed across the production call chain. The first Late Deferred pass correctly uses GBuffer; the second must use the distinct HDR target with LOAD. The old comment claiming that native r9 was unused was false.

W23, W25, W30 and W43 attachment declarations and explicit state transformations below match the bounded original instructions. Eight principal leaf callbacks plus `nani_draw` were examined for subsequent state overwrites. No additional confirmed P1/P2 emerged in this bounded set. This does not certify all 82 callbacks, their resource producers, shaders, or final scene pixels.

No Cargo, GPU, original executable, or window launch was performed by this agent. A regression assertion was added to the existing recording-host test, but its execution is delegated to central coordination. At the time of this report, Git metadata was unavailable from both D:/game and D:/game/tiny-glade; `git diff --check` could not run. Earlier completed Loading GPU validation belongs to the first report and is not evidence for W34.

## Original evidence provenance

- Original: `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`.
- SHA-256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
- Function boundaries: `D:/game/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv`.
- Reproducible bounded decoder: `render-state-evidence/world-followup-probe.py`; uses the existing stdlib PE section mapper and Capstone, never executes original code.
- Evidence: `render-state-evidence/world-followup/{shadow,main,main-oc2,late,deferred,nani-draw,grass-lod0,grass-lod1,roof-tile,roof-spire,roof-beam,rocky-terrain,bricks-lod0,bricks-lod1}.asm`.
- `world-followup/manifest.json` preserves all 14 PDB names/sizes, original body hashes, assembly hashes and the decisive W34 instruction bytes. The prior 15-function manifest is untouched.
- Original outer caller remains `render-state-evidence/render-world.asm`.

## P1: W34 original fourth target argument was dropped

Complete pointer lineage, not a pass-name inference:

| Original address | Instruction / meaning |
|---|---|
| `0x1404ec56b` | Caller loads r15 from original scene-color argument `[rbp+0x4f8]`; same r15 is passed to W25, W30 and W34. |
| `0x1404ec648..64b` | `mov r9,r15; call 0x1408373b0`. Bytes `4d89f9e860ad3400`. |
| `0x1408373d6` | `mov r14,r9`, bytes `4d89ce`: preserve scene target. |
| `0x1408373e6..3f7` | Empty Late Deferred list jumps directly to `0x140837753`, retaining r14 = original r9. |
| `0x140837400` | Save r14 to `[rbp+0xa80]`, bytes `4c89b5800a0000`. |
| `0x140837416` | `mov r14,r8`, bytes `4d89c6`: temporarily use GBuffer pointer for Deferred. |
| `0x14083741e/42f/489/496` | Deferred color tag0, pointer r14, r9 points to that ColorSpec, `new_impl`. |
| `0x14083773e` | Restore r14 from `[rbp+0xa80]`, bytes `4c8bb5800a0000`; original r9 is live again. |
| `0x140837753..75d` | Independently skip Forward if its list is empty. |
| `0x14083777d/787` | Forward color tag0 + pointer r14. Bytes `c785a8040000000000004c89b5b0040000`. |
| `0x1408377da/7e7` | r9 = actual color-spec address, then `new_impl`. Bytes `4c8d8da804000041b80c000000e884010300`. |

Before repair, `main_pass.rs::late_forward_and_deferred_pass(ctx,targets,lists)` allocated `[ColorSpec::tag0(targets.gbuffer)]` outside the two-pass loop. `SceneRasterHost` and `world_graph` did not carry scene color. Late Forward could therefore modify encoded GBuffer instead of the visible HDR surface; later deferred lighting would consume corrupted material data. That impact follows the graph dependencies; the exact visible artifact remains GPU-unverified.

Authorized repair only:

- `crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs`: add `scene_color:&AttachmentTexture`; Deferred selects targets.gbuffer, Forward selects scene_color; both retain tag0, original empty-list gate, viewport, depth writable LOAD and stencil mode.
- `crates/tiny-glade/src/scene_raster.rs`: carry the missing target argument through the adapter.
- `crates/tiny-glade/src/world_graph.rs`: pass `attachment(color)`, identical scene-color owner used by W25/W30.
- `crates/country-core/src/systems/render_loop/draw_passes/d_main_tests.rs::late_passes_are_skipped_when_their_list_is_empty`: distinct GBuffer id10 / HDR id13, equal 640x360 dimensions. Assert Forward-only branch writes13/tag0, dual branch writes10 then13/tag0, depth remains uncleared, and original pass ordering/empty gates remain. This test would fail on the old attachment selection; equal ids cannot conceal the bug.

## Attachment and ordering matrix

`G` = GBuffer; `C` = external scene/HDR color; `V` = velocity; `D` = scene depth. Color tag0 = LOAD, tag1 = CLEAR, tag2 = DONT_CARE. Depth tag0 = writable, tag1 = read-only. Clear-option presence is separate from attachment tag. The concrete backend tag translation was audited in the preceding report.

| Stage and original new_impl address | Original declaration | Current implementation | Status |
|---|---|---|---|
| W23 Shadows `0x1409cc62b` | No colors; three cascade layers0,1,2 of preset0x11; writable depth; clear1.0 per layer. `5cc..5e2` packs layer and `0x3f80000000000001`; `5b0` bounds to3. | `shadow_pass.rs`, loop0..3, Write{layer,clear:Some(1.0)}, no colors. | O/source match; per-layer GPU contents not checked here. |
| W25 G-buffer `0x140833ef8` | G tag2; D writable clear0. At `e7d` color2, `e8e` depth0, `ea7` clear-option qword1 (flag1/value0). | `main_pass.rs::main_pass`, first unconditional pass. | O/source match. |
| W25 G-buffer emissive `0x140834266` | G LOAD + C CLEAR RGBA0, D writable LOAD. `1e1` tag1, `1eb..1ee` zeroRGBA. | First write of C cleared; material draws use both targets. | O/source match. |
| W25 G-buffer velocity `0x1408345ca` | G LOAD + V CLEAR RGBA0, D writable LOAD. `541` target+0x48, `545..552` tag1/zeroRGBA. | First V clear0. | O/source match. |
| W25 Forward `0x140834996` | C LOAD, D writable LOAD. | Original `ShaderRefactoringState.disabled()` gate retained. | O/source match. |
| W25 ForwardVelocity `0x140834cea` | C LOAD + V LOAD, D writable LOAD. | Same gate as Forward. | O/source match. |
| W25 Velocity `0x140835039` | V LOAD, D read-only LOAD; always after optional Forward pair. `4fcf` depth tag1. | `DepthSpec::ReadOnly`. | O/source match. |
| W30 G-buffer OC/2 `0x140835d78` | G LOAD, D writable LOAD. | `main_pass_oc2`. | O/source match. |
| W30 G-buffer emissive OC/2 `0x1408360dc` | G + C LOAD, D writable LOAD. | Same. | O/source match. |
| W30 G-buffer velocity OC/2 `0x140836440` | G + V LOAD, D writable LOAD. | Same. | O/source match. |
| W30 Forward OC/2 `0x140836784` | C LOAD, D writable LOAD; no W25 refactoring gate. | Same. | O/source match. |
| W30 Velocity OC/2 `0x140836ac4` | V LOAD, D read-only LOAD; `a61` depth tag1. | Same. | O/source match. |
| W34 Late Deferred `0x140837496` | Nonempty list12 only; G LOAD, D writable LOAD. | Same after narrow fix. | O/source match. |
| W34 Late Forward `0x1408377e7` | Nonempty list13 only; **C LOAD**, D writable LOAD. | Missing C restored through production adapters. | Confirmed P1 repaired; CPU/GPU validation pending central. |
| W43 Apply deferred lighting `0x1409d2298` | C LOAD; read-only D = arg3+0x24; viewport from D. `2216` color0, `2220..2227` pointer originalr9, `222e` depth1. | `deferred_raster.rs` uses i.depth (historical field name, actual C) and i.lighting[1] for D. | O/source match; misleading field name already documented, not renamed. |

W25 Forward gate original `0x1408348ff cmp dword[rax],0; 4902 jne 0x140834fab`: nonzero skips both Forward passes, but still runs Velocity. Current bool is explicitly the enum's disabled()==true path, so no inversion was found. W25/W30 create their empty subpasses because they initialize/preserve attachments; W34 alone skips whole empty subpasses. These were checked separately.

## Per-draw state and callback sequence

Native default state is requested for each individual item, not inherited from a preceding draw: e.g. W23 `0x1409cc72f`; W25 `0x14083400f/436f/46df/4a9f/4dff/0x14083514f`; W30 `0x140835e8f/0x1408361ef/654f/688f/6bcf`; W34 `0x14083758f/78ef` all call `0x140be6750`. Current `d_main_support.rs::run_list_pass` obtains `host.default_raster_state()` inside the item loop. Production `raster_host.rs` supplies original `0x1ffc00d0` (triangle list, no cull, reverse-Z GREATER depth test/write, stencil off, blend off). The recording host's test default is an intentionally different value and is not used as proof of native defaults.

| Path | Original state edits and ordering | Source correspondence / conclusion |
|---|---|---|
| W23 | Builder variant1 at `0x1409cc811`; bind_common `830`; clear depth comparison bits with `835 and byte[state],0xcf`; callback `861`. Cascade flag is2..4 separately from shader variant1. | `run_list_pass` Shadow sets CommonDrawFlag(cascade+2), builds variant1, binds common, then `state &= !0x30`, then callback. Depth becomes LESS; write/test preserved. |
| W25/W30 stencil material paths | Copy item stencil value(+0x65) after checking option(+0x64), then bind_common, then `(state.byte1 & 0x8e)|0x21`, then item callback(+0x70). Representative W25 `0x1408340f7..417c`; remaining masks at44a9,4820,4bd7,4f37. W30 masks at5fc9,6329,6689,69c9. | Current Stencil mode preserves unwrap semantics, writes unknown_b1 before common binding, changes stencil state after binding and before callback. No blanket state replacement after callback. |
| W25/W30 velocity-only | `(default & 0xffffff8f)|0x20` **before** builder and common binding: `0x14083522c/22f`, `0x140836cac/caf`. This selects EQUAL and disables depth writes while preserving depth-test bit. | `ItemMode::Velocity` rewrites state before item.to_draw_packet/DrawBuilder, uses read-only depth, does not require stencil option. |
| W34 both lists | Same Stencil path. Native masks `0x1408376c6/6c8`, `0x140837a24/a26`, after common binding, before callbacks. | Fix did not alter any of these state choices. |
| W43 five fullscreen entries | Native entry sequence at `0x1409d23d0..241e`: material1,3,0,0,2; only fourth flag1. All disable depth-write (`25f4 &0xbf`). Fourth stencil ref material|0x80 (`2507`), additive blend (`2556 &0x9f`, `2558 |0x20`). After binding, `2e4c..2e5b` disables depth-test, enables stencil, sets compare/mask bytes while preserving high blend bits; `2e8e` draw3. | `ENTRIES`, `entry_state`, `final_state` mirror explicit bytes. W43's final_state preserves additive blend from the fourth entry. No per-item entity callback follows this final state. |

No missing premultiplied blend was inferred for these opaque/deferred passes: the native explicit state here is stencil/depth or additive. Applying the UI correction globally would incorrectly change them.

## Main callback state coverage

This table covers direct packet state modifications in the bounded callback bodies and current corresponding implementations; it does not promote all their resource producers or called binding helpers to fully independently verified.

| Callback original entry | Direct native state operation | Current implementation / coverage |
|---|---|---|
| Grass LOD0 `0x140b4af70`, LOD1 `0x140b4b1d0` | Both bind terrain presets9/10/8 and two buffers, write optional uniform, then different indirect terminators. No packet state edit in either bounded entry. | `draw_shaders/grass_shader.rs`: no depth/stencil/cull/blend rewrite, retains enclosing pass state. Current indirect buffer declarations remain real W29 outputs; contents were not GPU checked here. |
| Rocky terrain `0x1409cb4e0` | `0x1409cb510/512/514`: byte0 `(s &0xf3)|4`. | `rocky_terrain_draw.rs::rocky_state` masks only cull bits, preserving parent depth bits and other bytes. |
| Roof tile `0x1409d9070` | `0x1409d9127/9129/912b`: same cull-only mask/OR4. | `instanced_roof.rs::roof_state`, then `nani_draw(RoofTile)`. Parent stencil/depth preserved. |
| Roof spire `0x1409d96a0`, beam `0x1409d98a0` | No direct packet raster state modifications in bounded entries. | Both source functions bind their SSBOs and issue instanced draw without raster state replacement. |
| Wall brick LOD0 `0x1408a53d0` | Original pass-mode and proxy gates precede state update; `0x1408a54fc/54fe/5500` cull-only `(s &0xf3)|4`. | `draw_bricks.rs` preserves this gate/state ordering. `draw_callbacks_a.rs::bricks_cb` has an additional owner-readiness gate (`WallBrickUploadStatus.gpu_ready`); its asynchronous correctness is outside this state-only proof and remains a separately assigned boundary. |
| Wall brick LOD1 `0x1408a4ca0` | Original mode/proxy selection then `0x1408a4d92 and byte[packet+0xa8],0xf3` (cull off). | `draw_bricks.rs` uses cull_or0 after gate; does not restore default depth/stencil. |
| Nani terminal `0x140904ae0` | Pass variant determines list; resource bindings followed by count/raw indirect branch at `0x140904e24..ed4`, stride0x14. No direct raster state replacement. | `render/nani/draw_ext.rs` carries existing builder through binds and indirect terminator, no state_mut. Therefore the above cull and enclosing stencil/depth edits survive this layer in source. Full terminal Vulkan encoding is not newly re-proven here. |

## Remaining specific checks

1. Independently execute W34 differing-target regression, then capture a scene with a nonempty Late Forward list. Existing empty-world or same-target fixtures cannot validate this defect.
2. Confirm actual Vulkan image formats and pixel results with Late Forward material4 then deferred lighting, including stencil coverage. The original C/GBuf difference is proven; final visibility remains a separate integration gate.
3. Broaden callback evidence only along real nonempty draw lists. Transparent/overlay callbacks not in the bounded terrain/grass/wall/roof set remain separately covered only to the depth stated in the first audit, not globally certified.
4. W23 layer barriers and W30/W43 read/write layout execution have source-level graph declarations, but no new original-vs-restoration GPU trace was captured in this task. Do not equate matching LOAD declarations with verified GPU synchronization.

## Efficiency lesson from this defect

For a reused register, trace both assignments **and restores across each conditional branch**, then follow the pointer actually passed to `RasterPass::new_impl`. W34's r14 temporarily represented GBuffer but was restored to the saved original r9 before the second pass. Reviewing only the first attachment store or the function's retained comment hid a real wrong-target bug. Keep regression resources distinct, and preserve native address + raw bytes alongside the claim so another agent can independently reject or confirm it quickly.
