# Logo → gameplay loading gate review

Date: 2026-10-21. Scope: genuine loading completion, graph warmup, prepared
SubApp publication and Logo handoff. T13 is excluded. Existing rhapsody
ownership and presentation contracts are retained.

## Production changes

`tiny-glade/crates/tiny-glade/src/run_game.rs` now records bounded startup
milestones under `tiny_glade::startup`: first update begin/return, observation
of the real LoadingGameComplete event and update count, GPU warmup begin/end,
successful publication/receipt, the actual Logo wait-stage release and handoff
begin/end. The existing native gates and update loop are unchanged by these
logs. The release log checks the old wait value so it does not repeat every
frame while the final stage animates. Failures are logged before save/World
disposal so secondary shutdown errors do not obscure the original phase.

`GameplayPreparation::poll` now rejects a missing LogoAnimation owner instead
of silently bypassing the finished-stage gate. A received gameplay SubApp is
disposed through the existing save-worker shutdown boundary before the error
returns; no ready event or fallback Logo is manufactured. A new regression,
`missing_logo_refuses_handoff_and_stops_received_save_owner`, covers both queued
and already received Apps, verifies the loading owner remains unmoved, and
checks the save worker stops before World disposal. The existing later-handoff
failure fixture now supplies its genuine completed Logo, preserving its
original assertion about failure after that gate.

These are diagnostics and a missing-producer correctness fix. They do not
establish the cause of the user's previous Logo failure.

## Original gates and observed current source

- Original async loader calls SubApp update, then reads both actual
  LoadingGameComplete event buffers (native `1400d7922..792a` ORs the two
  lengths). Current spawn uses the same event identity and `!events.is_empty()`.
- Warmup runs only after that event. Publication is after successful real graph
  declaration, compilation, execution, export and GPU completion. It is not a
  replacement ready flag. Asset failures are preserved.
- Original main loop checks waiting at `1404d9832`, finishes the wait at
  `1404d9a5e`, checks finished at `1404d9ad7`, then moves the six owners, emits
  StartupRevealStarted, releases Vermeer hold and moves Egui before replacing
  the SubApp. Existing source preserves this sequence. Early receipt/error
  observation is a documented host adaptation and does not release the gate.
- LoadingGameComplete is not delayed behind full history replay, a converter
  flag or StartupReveal. None of those fabricated conditions was added.

## Warmup 32 × 32 extent: proven static discrepancy, not changed

The original B binary identity is checked by `probe_warmup_extent.py` against
SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
It decodes four complete PDB-bounded functions, without executing the game or
altering an analysis database. Body/disassembly identities are in
`bounded-native-manifest.json`. The available ReVa project inventory was empty;
no import or global analysis was started. Retained Ghidra C was used as a
navigation aid, and the conclusions below were checked against original
instructions rather than its placeholder signatures.

1. Async caller `1400d7a51` puts `0x2000000020` in RCX and calls the original
   render function at `1400d7a65`. There is no swapchain; mode is World.
2. Render `1404e807b` saves incoming RCX in R12, `1404e84b9` stores it at
   `[rbp+af8]`, `1404e8885` copies it to `[rbp+1000]`, and `1404e88be` publishes
   its address as closure field `+18` before calling compile-main.
3. Compile-main `140161e06` reads field `+18` into RDI; `140161f9a` moves RDI
   to R14. `140162302` and `140162880` read `[R14]` into RDX for
   `TextureDesc::new_2d`, creating the depth and world color targets.
4. The complete 49-byte new_2d leaf writes EDX to descriptor width `+c`, shifts
   RDX by 32, and writes the remaining EDX to height `+10`. This proves 32 × 32
   dimensions, rather than interpreting an arbitrary magic constant.
5. `140162a8d..140162adc` also passes this extent as the render_world stack
   `+20` argument. Without a swapchain, `140162f65` retains this extent in R9 for
   final_post. The swapchain-present branch can choose its own display extent.

Current `production_renderer::warm_up_gameplay` instead selects the real
WindowSize/FrameRenderConfig extents, and build_full_world_stage independently
selects the display extent. That differs from the original warmup algorithm.
Production warmup remains unchanged in this delivery because central froze
implementation pending the first actual runtime failure. A future correction
must thread the original explicit extent through world and tail construction;
mutating WindowSize or omitting passes would not restore the original contract.
The full Rust/PDB source prototype is not claimed recovered by this bounded
register-flow proof.

## Verification boundary and runtime result

This worker ran no Cargo command, GUI or game process. `git diff --check` passed
for its production edits. Central owns execution; relevant filters are:

- `run_game::startup_handoff_integration`
- `run_game::gameplay_preparation_shutdown_tests`
- `run_game::save_shutdown_preparation_tests`

The shutdown tests use Rust's default test panic behavior. The real main
installs its original aborting panic hook, so catch_unwind does not establish
recovery or cleanup after a true main-process panic. Result failures and
ordinary cancellation still use the explicit ownership paths. No claim of
production panic recovery is made.

Central's first actual Windows launch stopped before Logo because the Intel
driver does not expose the swapchainMaintenance1 capability required by the
current strict presentation contract. This worker did not reproduce or bypass
that failure. Consequently neither Logo progress, LoadingGameComplete,
completed warmup nor the actual App switch has been observed on that device.
The earlier Logo root cause remains unknown; milestone logs are preparation
for a supported-device run, not proof that the game now starts.
