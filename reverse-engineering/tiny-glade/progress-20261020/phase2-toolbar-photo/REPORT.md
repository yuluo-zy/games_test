# Phase 2: original ui_slider consumer — 2026-10-20

Source re-read after upstream main a6ef0b5 synchronization. Production scope:
`toolbar/src/photomode/ui_slider.rs`, the parent module export, slider_math's
original SSE clamp, and `tests/tb_g_slider_lifecycle.rs`. No rhapsody, engine,
generic dependency algorithm, factory, or host schedule was replaced.

## Actual restored consumer

The public function keeps the original sixteen SystemParams. Canonical resources
are GameTime, ActionSetStates, UiNavState/UiFocus, ImageFormationConfigs,
MainCamera, EraserSizeMultiplier, BrushSizes and AppMode. Commands, PlayUiSound
and RequestToolbarCategoryChange use their real Bevy owners. Three original
system locals retain grab offset, navigation arming and previous focus.

- Every matching slider retires its previous knob; even a zero/NaN extent
  queues the old despawn before skipping new drawing. Valid extents spawn the
  original Ui2dExtent + Ui2dImage("toolbar/slider_widget") + UiDrawOrder(1).
- Original travel/thickness arithmetic, per-frame hover +/- GameTime.delta/.2,
  static CubicOut easing (native tag 6), and 1..1.3 scale are preserved.
- Navigation is digital slots 20/21 (ui_left/ui_right), not two mouse buttons.
  Focus identity and prior focus, pressed/released/repeat states, arming, exact
  +/- GameTime.delta, right-over-left order and category activation are retained.
- All drag bytes preserve the original branches: phase 3 starts grab offset;
  phases below 2 record completion; any phase uses the retained offset when
  present. Completion does not discard the final cursor write or clear the
  original persistent offset. Event order wins over prior navigation candidates.
- Only the final selected candidate writes a parameter. Nonfinite values skip
  sound/parameter/previous-position writes but do not suppress a completion's
  category command. Original requery failure boundaries are not silently ignored.
- Fourteen-bin sound transitions emit actual PlayUiSound kind 0x1c once for the
  final value. Canonical eraser/path/water/garden completion emits the actual
  toolbar-category command (4/1/8/9), with the original AppMode duplicate guards.
- Previously verified 13 getter/parameter owners are reused. The shared canonical
  UiSlider remains the existing 24-byte component; central applied only the
  semantic accessors in ui-slider-accessors.patch, preserving type identity.

## Evidence and executed verification

Oracle: original B RNE workspace/input/tiny-glade.exe, SHA256
f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
Original full assembly is retained in the prior toolbar-photo-menu/ui_slider.asm;
PDB FunctionSystem's sixteen-parameter spelling is in skeleton/B/.../photomode/
ui_slider.rs. Old truncated pseudocode's fictitious 0xd+ cases mix adjacent
function bytes; the real getter/parameter switches have 13 original variants.

`native_slider_probe.py` executes original instructions; `build_actual_source_probe.py`
extracts the actual production helpers and drag business block into a small
standalone Rust harness. It also uses actual dependency source for Aabb2,
EasingFunction and DigitalActionState. glam is the existing pinned compiled
dependency. Native fixture files and production/dependency SHA manifests are retained.

Executed comparison, all passed with exact f32 bits and state outcomes:

| Scope | Cases | Original range |
|---|---:|---|
| Drawing geometry/hover/scale | 972 | 142348b15..142348c57 |
| Navigation/focus/arming | 1200 | 142348cfc..142348ef9 |
| Drag phases/offset/NaN | 540 | 1423490fd..1423492df |
| Actual category guard/payload | 273 | 142349a85..142349b40 |

Visual/drag probes hook only the already separately verified parameter getter's
fraction result; visual allocation receives test storage. Original AABB/easing
instructions execute directly. Navigation/category have no dependency hooks.
The drag business block is verbatim from production, with dependency containers
replaced only at the harness boundary. This does not claim native Bevy query or
command-queue execution was compared to the Rust whole-system runtime.

Six real canonical Bevy system tests are supplied: repeated knob entity
generation/recycling, zero extent retirement, grab+finish+cancel, last-event-only
write/sound, canonical eraser/category dispatch, navigation focus/GameTime pause,
and nonfinite completion semantics. Central actually executed Cargo: all six
new tests passed and all four existing tb_e_config_slider tests passed. Log:
`D:/game/tmp/phase2-toolbar-slider-tests.log`. This is a real ECS system pass,
distinct from the native bit comparisons above and from a full-App host pass.
After that run, this agent removed only two newly unused parent imports;
the consumer and all tested algorithms/tests remained byte-for-byte unchanged.

## Frozen source identifiers

- photomode.rs: 066efeb31b685104a244e9b71f0b71542bd848eb31f7350e6ff4d042e8f9c8c0
- photomode/slider_math.rs: 5b66994a670be3d2efdec301774abf5b9aa9b3f4fcb44e1628634b5cd680df52
- photomode/ui_slider.rs: 67199873cabbbc249296d04277d9e7b42518d5fa04760017c2edd2db809b7f31
- tests/tb_g_slider_lifecycle.rs: 0de22b6667b34da7eca979b036088b39391e785099b1c9054a0a0db363041c78

## Remaining toolbar chain boundary

This restores this consumer, not all photomode. Actual slider/reset entity
factory branches, photomode toolbar assembly, full do_photomode orchestration,
and the exact host stage/ordering remain separate work. No test-created entity
is counted as a recovered production factory and no whole-game pass is claimed.

## Reset-default erratum found during the factory follow-up

Two earlier reset constants were assumptions that the earlier tests did not
cover. Rechecking original table 14304ed58 and executing all thirteen original
reset parameter branches exposed them: kind 5 resets chromatic aberration to
0.015 (bits 3c75c28f), and kind 7 uses the parameter default 0.5 before
LogSlider(base=-3), producing exp2(-2.5), bits 3e3504f3. It does not use the
normalized midpoint as parameter zero. Both production constants are corrected;
the original forced focus-state/payload transition remains intact.

`native_slider_probe.py` now also records 312 reset cases (13 kinds × 6 camera
bodies × 4 focus modes), executing original range 14234824a through the actual
dispatch branches. Only the original exp2f library dependency is hooked.
`build_reset_probe.py` compiles the actual production reset/focus methods and
actual LogSlider/BrushSizes methods against dependency storage containers.
All 312 cases pass, comparing the full 0x4e0 camera, all eighteen relevant
formation fields and all three brush fields. Kind 9 remains a no-write reset.

The seventh tb_g regression uses real UiSliderReset entities/clicks with no
UiSlider and asserts these exact native bits plus the forced focus payload.
Its central Cargo execution is pending; the six prior consumer tests remain
the actual executed ECS result documented above. Frozen parent/test hashes
above predate this reset correction and will be refreshed at delivery.
