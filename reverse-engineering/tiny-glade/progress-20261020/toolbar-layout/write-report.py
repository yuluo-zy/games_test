from pathlib import Path
import json,hashlib
root=Path('D:/game');ev=root/'reverse-engineering/tiny-glade/progress-20261020/toolbar-layout'
files=['tiny-glade/crates/systems/toolbar/src/toolbar_systems.rs','tiny-glade/crates/systems/toolbar/src/ui_interaction.rs','tiny-glade/crates/systems/toolbar/tests/tb_f_auto_hide.rs','tiny-glade/crates/systems/toolbar/tests/tb_f_locked_animation.rs']
manifest={'modified_files':{f:hashlib.sha256((root/f).read_bytes()).hexdigest() for f in files},'validation':{'original_binary_sha256':'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03','geometry_actual_source':{'passed':1600,'bit_comparisons':6400,'sound_boundary_cases':5},'locked_native_clicks':{'events':6,'expected_commands':3,'cursor_after':6},'cargo':'not run by agent; central owner validates','ecs_tests':'added, not run by agent'}}
(ev/'delivery.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
pdb=root/'reverse-engineering/tiny-glade/skeleton/B/crates/systems/toolbar/src/ui_interaction.rs';lines=pdb.read_text(encoding='utf-8').splitlines();i=next(i for i,l in enumerate(lines) if l=='// 函数: toolbar::ui_interaction::ui_locked_animation');(ev/'ui_locked_animation-pdb-signature.txt').write_text('\n'.join(lines[i:i+7]),encoding='utf-8')
report='''# Toolbar layout and interaction recovery, 2026-10-20

## Restored production behavior

1. `toolbar_auto_hide` (B 0x142343a10): corrected the actual bottom/left/right/top centers. Previous implementation reversed all four edges and used toolbar length instead of thickness for left/right center. Restored native f32 arithmetic order, per-axis scale-dependent activation margins, and actual writes to every `DebugToolbarActivationZone` extent before the cursor AABB test. Disabled auto-hide leaves debug extents unchanged and resets the selected toolbar progress. Existing normal/photomode filtering, keep bit, click-held and menu-closed branches, easing, sound edges and anchored offset remain. Sound edges now preserve unordered-NaN comparisons.

2. `ui_locked_animation` (B 0x14233c8b0): restored exact PDB five-system-parameter shape and complete click producer. Only a live entity found by canonical `Query<&UiLocked>` receives amplitude .1 / duration .3 / progress 0 through deferred Commands. Existing animation query runs before queued click inserts are applied, preserving restart behavior and event order/repeats. Completion removal uses Bevy's original warning handler. Original inline sinc performs sinf/divss without a zero guard; removed earlier unsupported `sinc(0)=1` behavior and retained NaN.

No rhapsody, generic dependency implementation, host schedule, menu or photomode source was modified.

## Binary evidence

Actual oracle: `workspace/input/tiny-glade.exe`, SHA-256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. No patched database or another version's address is used.

- Original geometry instructions 0x142343f2f..0x142343ffc; four-target jump table 0x14304e6b8; vectors 0x14304e310 = (40,80), 0x14304e300 = (80,40). `toolbar_auto_hide-original.asm` retains the full original function range.
- Native geometry six buttons, scale 1, window 1000x600: bottom center (500,583.5), size (790,193); left center (16.5,300), size (193,790); right center (983.5,300), same size; top center (500,16.5), size (790,193).
- Click producer 0x14233c8b0..0x14233caf1, original command 0x140fdf820, panic handler 0x141e4f860, record 40 bytes. `locked-click-native-receipt.json` records actual native writes.
- Animation sinf/divss 0x14233cd34..0x14233cd39; completion uses warning handler. `ui_locked_animation-original.asm` and retained original C distinguish authoritative instructions from older pseudocode assumptions.
- PDB exact system parameter spelling is retained in `ui_locked_animation-pdb-signature.txt`, source information da21103e06ec109a0ec604e3.

## Validation actually executed

- Compiled unchanged cached dependency glam 0.29.3 (std) with standalone rustc. No Cargo or core/Bevy code generation.
- Extracted actual production `activation_rect`, `toolbar_activation_geometry`, `auto_hide_sound_kind` into `actual-source-geometry.rs`. Original instructions executed through Unicorn, all four alignments, eight button counts including zero, ten scales including signed zero, five windows. **1600 cases / 6400 f32 bit comparisons passed**, plus five sound boundary/NaN cases. Manifest hashes bind the production source and native fixture file.
- Executed original click producer with six events: live locked, unlocked, stale generation, dead, second live locked, repeat. Correct original order produced three insert commands and reader cursor 6. This is a native memory fixture, not a Rust ECS runtime pass.
- Added `tb_f_auto_hide.rs` and `tb_f_locked_animation.rs` real-system regression tests for canonical owner selection, padded hit zone, debug writes, mode-local progress/offset, sound, disabled policy, delayed click restart, reader consumption and completion. **Not run by this agent; central Cargo owner verifies.**

## Still open

This closes two algorithm/producer gaps, not the entire toolbar chain. Host schedule and whole-App end-to-end runtime remain central integration work. `ui_sounds`, full `add_toolbar_item` kind handlers, undo/redo layout producer, some focus/signifier lifecycles and complete slider input/draw/navigation remain explicitly unknown. They are not replaced with fabricated defaults or counted as complete.

## Independent slider review

Read-only review of the peer's slider implementation found actual setter gate stack+0x2c8 checks tag 0x13. PDB ui_slider signature's fifteenth parameter is Res<AppMode>, and does not contain EscMenuState. Current EscMenuState-based gate was reported to its owner and central integration owner for correction. Kind 9's scalar is independently confirmed by PDB as canonical EraserSizeMultiplier, distinct from BrushSizes. Native getter/setter fixture claims and arithmetic have been inspected, but this reviewer has not independently rerun all peer tests. Peer is preparing the final owner/gate correction.
'''
(ev/'REPORT.md').write_text(report,encoding='utf-8');print(json.dumps(manifest,indent=2))
