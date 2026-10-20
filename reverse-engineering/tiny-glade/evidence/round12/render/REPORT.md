# Round 12: render producer/consumer integration

Identity: original RNE `workspace/input/tiny-glade.exe`, SHA256
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
The PDB address/name/size ledger is `country-core-audit/data/procs.tsv`.
No rhapsody library sources were changed. The adapter uses the current physical
TextureHost, render graph imports, draw streams and original image dependency.

## T11

Consumer/file/GPU host implemented in `session_preview_host.rs`: actual four
shader startup, six512x512 sRGB pool textures, explicit compiled texture assets,
canonical EscMenuState/LoadMenuState and SavedSessionList, original core pool
assignment/range/refresh/retry/error-once logic, actual image0.24 decode to RGBA,
unchanged decoded dimensions, real FILETIME on Windows and actual texture update.
Recorded native draw commands are replayed through the actual DrawPassCtx and
current host. Per-App pool owners release their namespace entries at teardown.

The saved-card flag writer is closed: load_saved_session_list closure141328260
filters UUID directories with regular history.json and successful history-file
modified time. It calls is_session_ok_to_load140a9fa80 and stores the result at
SavedSession+18 (141328610). Card rendering tests that byte against0 at1408f8d90;
it is readiness, not screenshot_exists. Optional AutoSaveState participates in
refresh. Original controls retain the demo card restriction.

Evidence: saved-session-writer.asm, saved-session-flag.asm, session-card-flag.asm,
thumbnail-decode.asm. Direct locked xxhash-rust name matching recovered
menu/craft_paper, menu/new_glade_tile, cloth_ferris, brush/2. menu/back is explicitly
loaded for the newly restored LUT back control as well.

Interaction status: **still partial**. The shared ThumbnailScrollContent is now
implemented, but the original full load::ui141e1fe80 (15632B) and its contextual
load/delete/backup/warning controls are not yet restored or registered. A Load
state with never-produced metrics must not be called a complete load-menu path.

## T12

Actual shared capture slot and physical ownership API added. Publication binds
to the original Arc, discards/releases stale captures after closing/reopening,
and avoids releasing a same-ID owner when republishing. The gallery imports the
same physical texture into the current frame and binds its graph handle, so a
same-frame capture write and thumbnail read have a declared dependency. Parent
integration owns gather_screenshot_requests and shared-texture capture.

Corrected a substantial old consumer error: original iteration is
once(config0.name,config0.grading_strength).chain(metas.map(name,1.0)), not just
metas. Original UI creates metas.len()+1 cells. Default first-cell strength is
preserved; all other cells use1.0. iter_non_default_mut now skips configuration0,
as proved by the original enumerate/filter callers141ec0890/141ec09bc.

Restored `toolbar::photomode::lut_gallery::ui` and actual retained GUI tree:
fullscreen("lut browser") -> thumbnail_scroll("luts", len+1, same metrics Arc),
two rows, exact cell/gap bit patterns, navigation speed2; hover sound0/click2;
click0 restores initial name/strength, later clicks select meta[index-1] with
strength1 into both editable configurations. Closing writes NextEscMenuState.
The original safe-frame menu/back template and real PillButtonContent emit
background+image primitives and handle keyboard cancel18. RON template keys use
the original relative path without extension, preserving menu/back rather than
collapsing every path to file_stem. Parent registered the system in gameplay.

Registration is proved: MenuPlugin::build14068de50 calls the LUT UI's exact
eight-parameter run_if specialization140ff9df0 at14068df2e; group registration
passes set0x24 at14068e29e. run_if uses AppLoadingState tag1 (Game). The added
GameConfig source parameter maps the process demo flag; it is not a native ABI
claim. See menu-build.asm and production-system-signatures.tsv.

Shared widget algorithm141e1f4c0 includes wheel input, drag delta/inertia,
overscroll constraints, keyboard selection and all four navigation directions,
click-vs-drag threshold, cell/grid coordinates and shared metrics publication.
The original capture/bubble phase state and truncation prefix were restored in
MEI action dispatch (thumbnail-action-dispatch.asm).

Time-source correction: B14068459f copies Time+58 to MeiContext+270; the old
source mislabeled it elapsed_secs_wrapped. It is delta_secs. A bounded probe
linked against the existing locked bevy_time0.16 rlib reports offset58=0.125
while elapsed is1.359; raw original instructions independently prove the read.
No Bevy implementation was reconstructed. Existing public source field name is
retained to avoid unrelated API churn. See mei-time-delta.asm and time-layout*.

Interaction status: GUI producer/consumer/capture chain is connected. Full
production GPU/real event acceptance remains the parent integration run's job.
The wider photomode slider/toolbar system has independent unrecovered branches;
this report does not claim the entire photomode subsystem is complete.

## T16

Restored the entire color_button_animations1422979b0 (1594B) and deletion
142297ff0 (618B) against original instructions. CustomizationIcon is the real
0x24B owner: size, destination position, initial position, appearance progress,
hover progress and flags. Marked and unmarked branches retain their distinct
position/size/order behavior. Appearance advances by dt*4 or dt/-0.15; hover
advances by +/-0.15 **per frame**. Existing BackOut/CubicOut easing code is reused.
Only completed marked icons despawn. Native GameTime signature is retained in
production-system-signatures.tsv.

Interaction status: **still partial**, actual icon/menu producer not closed.
Existing show_color_options14229a7f0 is74991B, with cases0..38, entity-specific
wall/roof/decorator/stair selection, demo behavior and multiple event writers.
The existing source substitutes camera eye position for screen menu position,
radius1, any-wall/roof tests, demo=false and incomplete options. Those existing
approximations are not counted as recovered behavior or expanded in this batch.
hide_color_menu1422adcf0 has thirteen submission/event branches; replacing that
with a despawn loop would omit required side effects.

Additional original evidence obtained: ColorRadialMenu::create is5531B, not the
old decomp's1947B. The old decomp jumps into rdata because of a broken switch and
void ABI. Full original assembly and both12-entry tables are now retained in
radial-menu-create.asm/radial-menu-jump-tables.json. Its original argument
projection, selected-state32B, menu owner160B and canonical icon payloads must
be restored together before wiring create into the incomplete show producer.

## T13

`excluded_by_user`. Removed production feedback capture/report consumer call.
Historical recovered functions/evidence remain; no T13 producer was pursued.

## Validation

rustfmt succeeded on edited Rust sources. Added CPU regression tests for real
PNG decode and FILETIME, canonical metrics projection, retained LUT GUI click
and shared Arc/config writes, and appearance/exit/deletion lifecycle. Existing
LUT tests now include default first-cell strength and shortened zip behavior.
No Cargo command was started by this render worker; root runs unified checks
and production validation in its independent target. Compiler, tests and GPU
outcomes must be reported from those actual results, not inferred here.

The work tree was shared with other active reconstruction work. Edits were
incremental; no old complete file or rhapsody implementation was restored.

## Reusable integration lessons

* Audit a preserved decompiler export against the original PDB function size
  before using it. In create,1947B was a truncated/bad-switch export of5531B;
  targets into rdata and halt_baddata were analysis damage, not game behavior.
  Recover each bounded original jump table and real targets before transcribing.
* One shader/draw/animation consumer is insufficient evidence of an interaction
  closure. A create leaf also does not close show_color_options74991B or hide's
  thirteen event-submit branches. Track consumer completeness and player-trigger
  completeness separately, and preserve source-size/control-flow residuals.
* A shared metrics layout can close two GUI paths, but publish through the same
  retained Arc; do not synthesize a default grid to satisfy a renderer assert.
* Retain the existing numeric utility's endpoint rounding. Native BackOut at0
  can leave a tiny sin(pi) residue; regression tolerances must not motivate an
  unsolicited clamp in the game algorithm or rewritten utility.
