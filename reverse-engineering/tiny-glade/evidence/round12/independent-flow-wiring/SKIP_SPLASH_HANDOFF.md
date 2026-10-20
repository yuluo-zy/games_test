# Independent skip-splash and loading handoff audit

Frozen evidence-only candidate: `skip-splash-candidate.patch` and `skip-splash-manifest.json`. Production untouched. Original B executable SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

The original `skip_splash=true` branch **does bypass** the separate loading App, asynchronous preparation, LoadingGameComplete observation, and warmup. It must not be repaired by adding a synchronous load/warmup wait. The current bypass itself agrees with the original.

Direct complete-function Capstone decoding (`skip-splash-original.asm`, 68,544-byte `run_game_thread` body, SHA256 `2471990a592afec11565ccca52690d85a44f40443ebbe76dac25b5762ee8d88f`) proves:

- 14059f0a9 calls `App::insert_resource<PendingEguiContextResource>`140127e40. That insertion contains the exact pending GUI TypeId pair, independently checked against original bytes; its source Context is the game-thread argument.
- 1405a2bf8 reads GameConfig+E4, 1405a2bff saves it at stack+107. Reads1405a3ead/3ebc restore this into R13B. The conditional1405a4168 tests R13B, and1405a416b jumps to the loading App path when zero.
- Nonzero skip branch1405a4194 looks up pending GUI TypeId `(aed7b5777ef68ba3,5834a199b0692b72)`, removes the owner through BlobVec, calls identity conversion14067dd80, and calls `App::insert_resource<EguiContextResource>`14013c9f0 at1405a4288. ICF aliases at14067dd80 are explicitly distinguished; its original body is `mov rax,rcx;ret`.
- 1405a42b0/42ba looks up the **same** StartupRevealFinished TypeId independently proved by the event registration/writer review. 1405a4375/437c/4380 appends one unit event and increments sequence count. No Started event is sent in this branch. GameLoop::new/run follow1405a614e/617d.
- The other branch constructs the loading App and spawns async load before the separate new/run calls1405a5f88/5fa9. Handoff in original main_loop moves its six resource owners, then sends StartupRevealStarted, then consumes Vermeer with hold disabled, then moves active Egui. This is separately backed by `reveal-independent-review.json`.

The candidate adds the missing skip branch GUI move and immediate Finished writer. The public host creates its GUI context before selecting the branch and passes the same identity to gameplay Pending and loading Active resources. Context creation and a Result diagnostic for absent mandatory Pending are explicit public-host adapters; no native source field layout is invented. No loading gate, replay shortcut, duplicate event, T13 feature, or warmup is added.

The patch touches only two current production-relative paths, preserving their mixed line endings. Its `run_game.rs` base SHA256 is `3a5d7a38b45c05c1c9e905d91576f029ac4c8e6445f73651f9e055ce5e319913`; `game_loop.rs` base is `f10b67b5fbc7842d1e880f476632e725aacc07e958e9f8d06f96dcc9330fe245`. Apply narrowly if another owner edits those files.

Three candidate tests use actual resource types and actual production methods:

1. `ordinary_startup_waits_for_logo_then_moves_real_owners_and_starts_reveal`: publishes a real prepared SubApp through the actual receiver, exercises `GameplayPreparation::poll`, verifies logo Waiting then Finished, all six nonempty payloads including owned allocation identity, exactly one Started, preserved nontrivial Vermeer state with hold released, shared GUI data identity, and the separate gameplay ShaderLibrary Arc.
2. `skipped_splash_moves_pending_context_and_finishes_without_started_or_loading_wait`: executes the actual skip helper without any loading manager/event/Logo and verifies the GUI move and Finished identity.
3. `started_is_published_after_all_six_moves_before_vermeer_and_egui_move`: forces the real missing-Vermeer failure point and observes all six owner moves, already-published Started, and Egui still retained by the source. This tests the native ordering boundary, not an atomic transaction invented by the host.

`git apply --check --directory=tiny-glade --ignore-whitespace` passed. Both complete candidate Rust source files parsed successfully with rustfmt, edition2024, skip_children=true. **No Cargo, Rust execution, window/device, async load producer, or full gameplay verification was run by this agent.** The tests establish only the handoff owner/event contract when central execution passes; they do not prove the still-partial gameplay scene or full first frame.
