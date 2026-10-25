# GUI resource and consumer increment — 2026-10-15

Status: partial recovery, not a claim of complete gui.rs, toolbar, photomode, UI, or country-core. The previous README lagged the actual source: the move wrapper, two photomode flags, ring logger and producer queue were already implemented before this increment. Those are reviewed/reused, not counted as new restoration.

## Implemented

- `resources/gui.rs`: recovered the owned DebugGui controls proven by original widgets/defaults. B +0xf0 debug enabled; +f1 spatial grid; +f2 trigger proxies; +f3 threshold curve; +f4 lazy grid; +f5 sheep; +f6 music; +f7 Show 2D UI; +f8 human reference; +f9 terrain highlight; +fa brush previews; +fd wall snapping; +fe eraser highlight; +100 music richness; +101 experimental window; +102 proxy mode; +103 CameraMode; +104 raycast geometry; +e8 normal length=1; +ec snapshot interpolation speed=1.5. All recovered bool defaults are false except Show 2D UI=true. Field names describe labels rather than claiming recovered author spellings.
- Corrected old `show_screen_log` field name: the original +f7 is the shared Show 2D UI flag, not a separate logging flag. Existing draw callbacks retain the same `DebugGui.suppress` storage for the +fe eraser suppression control.
- Added the original map ownership policy for per-TOD-entry state: u64 TempUiId key, one `TodSnapshotGuiState { expanded: bool }`, false for new keys, preserved on revisits. Uses std HashMap/RandomState rather than rebuilding hashbrown. It does not invent a cleanup policy.
- Added recovered egui controls including original labels, enum order and two proven slider bounds: normal length 0.2..=4, snapshot speed 0..=10. These are resource-owned controls, presented as a subset; original full panel layout and cross-domain controls remain incomplete.
- `systems/backend/gui.rs`: active pending-context transfer; original-shaped screen-log system using `ResMut<DebugGui>, Option<Res<EguiContextResource>>, Res<GameTime>`; debug-control consumer; explicit usable local schedule chain pending -> flush -> controls -> log. Renderer remains egui frame owner. This local integration order is not asserted to be the recovered full game schedule.
- PhotomodeScreen now exposes its proven enabled edge and frame-end copy. The full `do_photomode` consumer still requires AppMode, camera, image-formation, UI, autofocus and audio domains.

## Evidence / review

All *.c files in this folder are extracted verbatim from existing `skeleton/task-index/work/TG-cfac1f498c427933d2/core-gui-20261015` MCP responses. `inspect_native.py` emits bounded original assembly and verifies input SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03` before reading. No analysis DB mutation.

- 0x14093fce0: actual DebugGui constructor defaults and original HashMap initialization. 0x1405f4180: five owned strings and one map drop, no hidden owning string/Vec inferred.
- 0x1400bb890: music flags; 0x1400b7da0: terrain/wall/eraser/brush flags; 0x1400bcfc0: Show 2D UI and sheep controls; 0x1400bdc80: proxy fields, normal length, enum reads, exact slider bounds; 0x140148cb0: snapshot interpolation speed and its bounds.
- 0x1400c48b0 original fmt tables at 0x142935590/0x1429355c0 yield Normal/TiltShift/Free/Walk/TopDown/Butterfly. Therefore DebugGui +103 is CameraMode. `do_photomode` writes PhotoModeCameraMode::from output (+1) on entering/changing and 0 on exit; claiming this field itself is PhotoModeCameraMode would be wrong.
- 0x1400bff20: inserts one false byte by TempUiId and passes its address. 0x140144590 renders `...` and toggles expansion. 0x140144ec0 gates editor controls on that bool and invokes **double_clicked_by** for `Delete (2-click)`; this is not a two-step confirmation state.
- 0x140160410: checks any occupied slot, context present and +f7 before opening log window. 0x14015e1c0 renders physical slots, then subtracts `[GameTime+0x28]`, dropping only on `<0`. The PDB-generated gui.rs generic system signature explicitly identifies `Res<GameTime>`; same time_update at 0x140922c30 populates GameClock.delta. Parent unified `resources::time::GameTime` as its sole resource alias.
- Existing 0x14093fe50 log inserts without severity gating, initial lifetime 5 seconds, `(next+1)%5`; 0x1408c0d90 drain preserves order and mutex poisoning. No queue algorithm rewrite.

Claude was actually invoked with tools disabled and existing provider configuration. `claude-snapshot-analysis.txt` is advisory output only. It correctly identified one bool map value; its proposed confirmation/deletion interpretation remained ambiguous. Original adjacent closure and assembly disproved treating the bool as deletion confirmation. Final state name is expansion, and no Claude inference is primary evidence. CLI emitted an unrecognized-model diagnostic for `deepseek-flash[1M]` but completed with content; model/provider price and speed were not measured.

## Verification scope

Added four focused tests: defaults plus stable per-entry map state; six insertions and strict zero expiry; producer -> active context -> egui shape output with GameTime aging; hidden/absent-context lifetime preservation. No full repository fmt; only edited source files formatted. Parent runs compilation/tests and records actual results. This report does not claim tests passed before that run.

## Remaining

- DebugGui snapshot-operation payload at +0 and flags +fb/+fc/+ff: defaults observed but consumers/semantics not yet closed; omitted rather than fabricated.
- Full TOD editor/schema producer, camera-snapshot queue and interpolation, experimental panel, cross-domain debug panels, full photomode effects, and original global scheduling.
- Canonical TempUiId owner is not yet in current time_of_day.rs; u64 map API preserves observed identity without inventing its allocator.
- AppMode, toolbar tables and mode_manager input/transition implementation now have their separate `../mode/REPORT.md`; parent reports six mode tests and two toolbar tests passed. Full toolbar widget consumers remain unimplemented.

No directory/module is marked complete by this report. User request for complete behavior remains active.

## Later implementation and original-instruction review

- `backend/gui.rs::show_debug_controls` now consumes the real MainActionSet slots 28 and 33. The context guard precedes both; slot 33 runs after the main window even when that window is closed (0x140148017..0x1401482d4). MainActionSet must be supplied by the caller.
- `resources/photo_mode.rs` restores four photo camera modes. Conversion 0x14090e500 adds one; full cycle 0x140b59720 is `[1,2,3,0]`; demo table 0x142af1278 is `[1,2,0,panic]`.
- `PhotomodeState` now has all observed fields through +0xd. Its old zero-derived default was wrong: 0x1408705b0 initializes scalar to 3 (demo 1), index MAX, camera TiltShift, all flags false except +0xd true. The qword store uses ECX, whose write zero-extends RCX; bytes +8..+0xb are zero. `on_enable_photomode` deliberately preserves +9/+b/+d, resetting only scalar/index/+8/+a/+c.
- `backend/photomode.rs::update_photomode_visibility` restores the prefix at 0x14234b190..0x14234b1f7 using actual AppMode/EscMenuState/MainActionSet resources. Open menu resets flags first; a same-frame photo hide action may then toggle them. Replay retains flags only when the menu is closed. This is a bounded consumer, not the entire do_photomode system. Full camera initialization, audio, focus readback, snapshot owners and original scheduling remain open.
- Added bounded cases for debug action edges/context gate, photo enum cycles, constructor/reset preservation, and visibility ordering. Parent owns central compilation/testing; no pass is claimed here before its result.

`claude-photo.result.json` records successful helper invocation using inherited configuration. Its answer was rejected on two material details: it incorrectly sign-extended `mov ecx,0xffffffff`, and confused AL with the separately toggled CL. Original instructions prove zeroed upper RCX and +0xd receiving the **old** hide flag. No Claude claim replaced primary evidence.

Further readonly evidence is saved for the next owner: `toolbar-buttons-result.json`, `button-parameters.json`, `select-animation-result.json`, `select-animation-parameters.json`. These prove real UI-click request forwarding and selection-animation insertion but do not count as implementation. Parent reprioritized toward lowest-implementation modules before these consumers were added. No new toolbar component or system was created for this exploration.
