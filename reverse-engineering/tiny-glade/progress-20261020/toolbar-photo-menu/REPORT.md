# Photomode slider parameter restoration — 2026-10-20

Reset-default claims below are superseded by the native-verified reset erratum
in ../phase2-toolbar-photo/REPORT.md: kind 5 is .015, kind 7 is exp2(-2.5).

Production scope: `toolbar/src/photomode.rs` and its new `photomode/slider_math.rs`.
Original oracle is B RNE `workspace/input/tiny-glade.exe`, SHA256
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

## Restored and corrected

- `slider_val_01` is the full 500-byte range 142349de0..142349fd4, not the
  stale 28-byte Ghidra function body. It returns normalized positions. Original
  13-way dispatch and 13 lower/upper endpoint tables are in slider-tables.json.
- Formation sliders address +1c,+20,+24,+30,+38,+34, not the former +18-based
  sequence. None of these six getters clamps its input.
- Six canonical camera bodies use the original focus table 142aabb68 and lens
  table 142aabb98. FOV and aperture map through each actual lens's own bounds;
  focus uses log2(distance)+3, clamp [-2,2], then the range normalization.
- Camera writes preserve `(1-t)*lower + t*upper` ordering. Changed focus target
  transitions states 0/1 to 3 and copies the original 8-byte start-pose payload.
- Reset 0..2 reads immutable config 0, preserving contrast/saturation defaults;
  reset 6/8 uses per-lens defaults and 7 restores 1/8 with the force transition.
- UiSliderReset itself carries the byte kind. A reset entity requires no
  UiSlider component. Original reset kind 9 is deliberately a no-write branch.
- Independent review corrected the state owner: original PDB parameter 15 is
  AppMode, not EscMenuState. Production ui_slider now follows AppMode::Photomode,
  including when the escape menu is Closed.
- Original PDB parameter 13 is EraserSizeMultiplier. Kind 9 now writes the
  canonical shared resource used by deletion/brush previews, with original
  [0.3,2.0] clamp/NaN behavior; it does not overwrite a Path/Water/Garden brush.
  Types and canonical owners were reused without new shadow resources.

## Verification

`probe_getter.py` executes the real original getter and camera-write blocks in
Unicorn. Only original library log2f/exp2f dependencies are hooked. Calls to
game-owned current configuration, camera selection and inverse_lerp execute
their original instructions. All original input and output bytes are retained.

- Getter: 1,638 cases (13 kinds × 6 cameras × 21 inputs), exact f32 bits.
- Camera writes: 360 cases (3 kinds × 6 cameras × 4 focus modes × 5 inputs),
  full 0x4e0-byte camera output comparison, exact bytes.
- Both actual production function slices and the actual slider_math module
  were compiled with direct rustc and passed against these native fixtures.
  The probe's raw camera/config containers replace dependency storage only;
  unrelated brush write branches panic if reached in the camera-only probe.
  The final getter probe invokes the unified real-owner dispatch, including
  EraserSizeMultiplier for kind 9; all 13 getter branches are therefore covered.
- Three production canonical-owner/ECS regression tests are added for central
  Cargo execution. This agent did not run Cargo or claim those tests passed.
- Independent reviewer toolbar_layout checked original dispatch, parameter
  tables, six focus/lens owners and transitions, and identified the gate fix.

Reproduction: run probe_getter.py, build_getter_probe.py and
build_camera_write_probe.py, compile the two generated *_actual_source_probe.rs
files with rustc --edition 2021, then execute both binaries.

## Still open; not counted as complete

The entire original ui_slider additionally draws/recycles slider image entities,
tracks grab offset and two mouse buttons, consumes navigation and emits focus/
sound/category changes. The present source's UiDragEvent bridge does not restore
that entire lifecycle. do_photomode and setup_photomode_toolbar also remain
incomplete. No exact original host stage was newly established by this work;
registering this partial bridge alone must not be reported as full closure.

The shared LogSlider::value_to_param method is missing subtraction of log2_base;
central was supplied the narrow correction. This agent did not edit shared types.
No menu/text/license claim or progress increment is made in this batch.
