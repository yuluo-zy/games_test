# Independent B compile_main caller/tail review

Scope: original B only; T13 excluded. No production edits, Cargo, GPU runtime or source acceptance performed. This review uses the local shipped RNE directly and historical decompilation only as an aid. It does not use graph_chain's conclusions as evidence.

## Identity and complete code boundary

`original-identity-range.json` records SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. PE `.pdata` gives primary body [0x140161cd0, 0x1401640c4), 9204 bytes, SHA 8fa611afe469518afb6b389282e3b7ecf4b28f06f9e33b6ef66a965bd4fdc4b3. `original-full.asm` freshly disassembles that complete interval with LLVM. The historical compile-main.json requested 10961 bytes through 0x1401647a1 and explicitly did not verify the boundary; all 2159 historical instruction byte sequences match the local RNE, but its requested range must not be called one function's body. Additional unwind funclets may exist outside the primary range; this review does not identify them as the primary function.

The original symbol is a `RenderGraphState::compile<ResourceExportHandle<Texture>, PassResources, compile_main_render_graph::closure_env_0>` monomorphization. The historical `void()` prototype is not its ABI. Machine code has an output-result pointer, moved graph state, closure captures and stack arguments. `prior-compile-main-decoded.c` retains the earlier parameterized decompilation with unresolved aliases; it is not treated as a verified Rust signature.

## Concrete defaults rejected

| Host input before this round | Direct original evidence | Verdict |
| --- | --- | --- |
| flag_0x1ac0=true | 0x140162874 writes zero to rbp+1a80; 0x140162856, 2b98, 2bde write low byte 1. Same byte feeds final_post and wireframe/UI gate. | Dynamic producer; constant true skips original idle path. |
| bvar43=None -> Keep | At 0x140162c70 a locked vector scan checks request+0x96==6; first match's +0x4c supplies DIL. Exhaustion writes DIL=7 at 0x140162cd7. | Actual tail value is screenshot-selection state. No request yields 7, which selects Custom grading, not Keep. Earlier render_world-result reuse is overwritten before LUT/logo decisions. |
| painterly.enabled=false | 0x140162546 calls 0x14093fbc0 and stores result; fallback stores 0 at 0x1401627b4. 0x140163120 copies this byte as stack sixth arg to painterly. | Dynamic EscMenu policy, not a load-time constant. |
| painterly auxiliary textures unknown | 0x140163144 puts &rbp17e0 in r9; stack+20 receives &rbp17b0 at 0x140163131. Both are dataflow copies of actual render_world +0 depth/+0x24 CoC or the original dummies. | Recoverable caller producers, not arbitrary missing inputs. |
| Vermeer skip_generate=painterly.enabled | 0x140163250 reads a different resource +0x20 and feeds stack+20 for call 0x140163277. | Existing shared bool alias is incorrect. The resource identity needs its separate positive mapping. |
| logo_overlay=false | 0x140163286 reads demo-global pointer then checks byte==1. Demo branch accepts purpose3/4; otherwise purpose3 checks captured config+f3. UiSettings serializer @0x140a46b96 takes self+0xb with literal game_logo. GameConfig.ui is +e8. | Config.ui.game_logo is the live normal branch producer; false cannot replace it. |
| cursor_visible=false | custom_cursor_pass @0x1409d1a8a/94 queries GameConfig TypeId3d7b56e787d47c40/67b0b10a8d43bfc7 and @0x1409d1af5 checks +f4!=0. UiSettings serialize @0x140a46bb6 takes self+0xc with literal custom_cursor. | Live Config.ui.custom_cursor. Default happens to be false but must remain mutable. |

Exact UiSettings strings were read from original RNE: 0x142aceca9 length9 = `game_logo`, 0x142acecb2 length13 = `custom_cursor`. `ui-settings-ser.asm` and `cursor-producer.asm` retain the instructions. The cursor shader/window/resource lookup occurs before visibility early return: a host must not suppress the whole call to avoid its mandatory lookups.

## Before-render_world gate contract

Use address names for unresolved resources; do not invent ownership. Let M be required EscMenuState, A be required resource (35385c45b6f578e9,f1623d81d5dd0c9f), P be MetaCommander::is_profiling at 0x14091f0f0, R be FrameConstants helper 0x140976880. The original main color (format0x7a, dimensions from closure, descriptor transform helper 0x140bd5460 arg6) is allocated before this gate.

1. If A.byte0 != 1 OR !M.method93fc20(): painterly_enabled=false; select `P`.
2. Else painterly_enabled=M.method93fbc0(). If M.method93fbe0(): select true.
3. Else if painterly_enabled && !PainterlyState.has_target(): optional trace log `painterly needs a target`; select true. The PainterlyState lookup is required only here.
4. Else if R(): optional trace log `temporal needs reset`; select true.
5. Else if M.method93fc60(): select `P`, retaining the previously computed painterly_enabled.
6. Else optional trace log `!can_use_static_world_capture()`; select true.

Selected false: flag=false; no render_world and no primary_rt_debug pass; synthesize depth and CoC below. Selected true: if process byte143634b90==1, flag=true and call original primary_rt_debug 0x1409beca0 on main color, without render_world; synthesize depth and CoC. Otherwise call render_world with the original world inputs, flag=true, and consume its depth/CoC/TAA. Trace logs are gated on original log level, not different behavior branches.

There are two independent missing/niche checks after render_world: result byte+0x22 (at 0x140162b9e) ==7 selects depth dummy; result byte+0x46 (0x140162be4) ==7 selects CoC dummy. Other bytes are copied, not reconstructed from a default. The exact Rust enum name for these niche bytes is not proved here.

Original dummy construction is positive evidence:

- depth: new_2d(world dimensions, format0x82); descriptor helper0x140bd6ce0; allocate name `main depth` (0x14295d798, len10).
- CoC: new_2d([1,1], format9); descriptor helper0x140bd6ce0; allocate name `coc` (0x14295d8a8, len3).
- No-world branch keeps the already-declared main-color texture as the postprocess input. It does not fabricate a TAA pass/output.
- Actual render_world @0x140162af9 uses sret rbp1810, with returned blocks +0x00 depth, +0x24 CoC, +0x48 final color. The 36-byte blocks are copied in full before the niche checks.

Leaf methods independently disassembled in esc-controls.asm: 93fbc0 returns false for tag2, flag+8 for tag10, true otherwise. 93fbe0 returns true for tag2, !flag+8 for tag10, false otherwise. 93fc20's whole body was not captured by that short leaf slice; use full previous source or extend the exact leaf, do not infer its remaining bitmask result from this partial slice.

## Failure and acceptance boundary

Original required world-resource lookups have explicit panic edges (including A, M, conditionally PainterlyState, FrameConstants and MetaCommander). Original graph compile takes/moves state and has its own compile-failure sentinel and drop branches after declaration. A missing-resource error can expose an unresolved reconstruction boundary; it must not become an invented default or an empty pass. This review validates static producer dataflow only, not GPU output or integrated failure unwinding.

The independent Ghidra worker attempted names-job.json twice and failed at launch because Java LaunchSupport could not write C:/Users/liyu/AppData/Roaming/ghidra/ghidra_12.1.4_PUBLIC/java_home.save. Retained backend-launch-failure.txt. No config change, escalation, install, worker kill or production modification was made. Existing original byte evidence and fresh LLVM disassembly remain usable independently of that failed tool call.

