# Timelapse lifecycle integration, 2026-10-19

Original B only. T13 feedback capture and its producer remain excluded.

## Source delivered

* `country-core/src/systems/timelapse_replay.rs`: canonical byte state, start
  and finish events, independent `Option<f32>` override, complete observed
  start/interrupt/finish/cleanup branches. Initial tag2 is the actual run_game
  writer, never an invented inactive default. Recording rejects the drained
  last start request with sound4; accepted requests preserve AppMode, append
  LoadSession(id,true) then WaitHistoryReplay, multiply the request by5, set0.
  Interrupt while0 appends LoadSession(id,false) exactly once, sets1 and waits
  for the real command/fence owner. Empty commander disables the override and
  emits its current byte; PostUpdate independently consumes Finished and sets2.
  The original unconditional changed-tick write before the finish state test is
  preserved.
* `country-core/src/systems/history/replay_mode.rs`: removed the erroneous
  `ReplayFastHoldInput` substitute for the override; canonical MainActionSet
  slots16/17 remain the input owner, and ReplaySpeedMultiplierOverride is the
  sole live speed owner. Other replay algorithm branches are retained.
* `toolbar/src/photomode/timelapse.rs`: actual last-Finished consumer from
  do_photomode@14234b653..7b0. Payload1 unhides photo UI, enables its screen and
  requests Photomode; payload0 does not alter that state. Its cursor is
  independent of cleanup's PostUpdate cursor. `photomode.rs` exports this module.

Central owner integrates the gameplay registration described in
`integration.patch`, replaces the old ReplayFastHoldInput initialization in
`country-core/src/systems/history.rs`, and inserts `restore_after_timelapse`
after photo transitions but before finish_photomode_frame in set8. This report
does not claim those central changes were already applied when written.

## Queue / async ownership review

The existing canonical MetaCommander already retains the original UUID,
ignore_snapshot and owned Arc fence across LoadSession retries. It waits for
AppMode13/14 before issuing Reload. `finish_replay` consumes the real
WaitHistoryReplay sentinel only when original history/input completion permits
it; `commander_update_system` never treats that sentinel as immediately done.
The two RT async cases read canonical structural/mesh dirty and worker idle;
300-frame initial wait and decremented bounded retry match original cases2/3.
No parallel queue, fake empty ReplayBuffer or permanently inactive state was
added. Other script variants8/9/12..16 still explicitly reject unsupported
owned payloads in the preexisting MetaCommander; their complete consumers are
outside this bounded lifecycle delivery and are not declared restored.

## Independent verification

`verify_original.py` directly reads the unchanged original RNE, validates its
whole SHA, five full function-range SHA values, the5.0 original constant and
seven raw-instruction branch assertions. Output:
`independent-byte-verification.json`. All completed successfully. Existing
retained C/asm, initializer and numeric-set caller evidence remain in this
directory. The newly bound Finished UI facts come from the complete original
`evidence/core-recovery-20261015/gui/0x14234b010.{c,asm}` and are independently
checked against the RNE bytes.

Three core lifecycle tests and one toolbar last-payload/cursor test are present
for central execution. This worker ran rustfmt, **did not run Cargo** and does
not label the source compile- or runtime-verified. Original-byte identity is
not original-process behavioral equivalence.

## Explicit remaining producer boundary

The original do_photomode signature includes StartTimelapseReplayCmd writer,
but its retained complete body shows no recovered f32 Start event append.
photomode_buttons' original complete signature has no Start writer. A writer
generic in the PDB proves neither a UI button nor an executed producer; this
worker does not invent one. The exact global Start producer / possible original
compiled-out UI remains unclosed. The actual Finished UI consumer is delivered.
Full scene loading/replay and UI interactions still require the central real
App verification, including interaction with Escape's original History
interrupt producer.
