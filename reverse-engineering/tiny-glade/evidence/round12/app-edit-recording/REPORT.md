# Main App wall edit and history recording audit (frozen candidate)

Original B is tiny-glade.rne SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. Every helper in native-registration.json, native-helpers.json and native-history-extra.json has freshly extracted machine-code bytes, SHA256 and original PDB generic symbol. The source was not changed and Cargo was not run. Native `.pdata` function ranges may exclude outlined cold handlers; all control-flow registration decisions quoted here are in these actual hot bodies.

## Correct numbered order

- set6 real hover/update selects a previous-frame registered widget and retains its Arc. The UI owner is still restoring this producer; no hand-written interaction fixture proves real picking.
- set14 history_start_recording: native 1409438ce passes 0x0e to helper141738630. History is the existing `History<MaybePendingSnapshotId>`, not `History<()>`. Native params: ResMut History, Res UiState, Res AppMode. Existing Rust representation reorders them and makes UI optional; installed App supplies the real owner.
- set17 ui_arch_controls is separately registered by add_wall_systems14049314e.
- set19 `(main18.chain_ignore_deferred(), publication16.chain_ignore_deferred()).chain_ignore_deferred()`; complete members/parameters in registration-contract.json. The requested “16 items” is the SECOND tuple; the first tuple has 18, including splitter and conditional bulldozer before the 16 conversions. Original 140493246 assigns stage0x13. UI draw and continue therefore run AFTER start14; local CPU adapters that put them before start are not the original main schedule.
- set34 (`0x22`, native14049343e) history_record_adjust_wall_height -> history_record_wall_changed -> history_record_merge_walls -> history_record_auto_clutter. All complete before set35. Existing typed RecordWallHistory adaptation additionally needs its actual `record_wall_history::<MaybePendingSnapshotId>` event drain in this stage; simply registering the deferred producer leaves History.add disconnected.
- set35 (native1409438e1 `0x23`) history_finish_recording -> history_process -> history_clear_previous_state -> replay_mode -> finish_replay -> signifier_replay_active. This is chain_ignore_deferred. Finish before process is required so a release frame commits its pending group before undo/replay commands inspect it.
- set40 replay consumers and set41 Pop remain installed exactly once by the central owner. Never call the old whole `add_history_cpu_systems` to recover the missing prefix.
- set42 (`0x2a`, native14049316a) is the actual `system_decorator::restore_archived_decorators`, confirmed by helper140322d40 name and original full8 SystemParams. It clears immutable then handles mode, archive/collision/stair lifecycle. The unique direct call of DecoratorStorage::mark_mutable140a1c2b0 in the whole original .text is at140817a9f inside this system140817a40; mark-mutable-xrefs.json records the independently scanned result. A synthetic frame-prefix mark_mutable must not substitute for full restoration.

## Available integration candidate

available-registration.rs installs the actual canonical start14, complete currently implemented draw/continue/snapping slice19, existing pending-wall and typed-event drains34, finish35 before the EXISTING process function. It does not claim to be full native wall registration. It preserves original relative order of available leaves and runs before the already installed publication/height chain. Original resource constructors are preserved by `register_wall_drawing_resources`/`register_recording_events`; GameTime, GameConfig, MainCamera, WindowSize, real cursor ray, GladeBorder, canonical mode/UI, typed History, and actual asset/subtype owners remain required from startup. It creates no placeholder ray or UI state. It is an uncompiled integration candidate only. No duplicate Replay/Pop or bogus empty missing system is registered.

## Concrete gaps, separate from registration

1. Main18 wall_splitter and merge_walls have no production source module. The current bulldozer producer is a partial wall-only proxy; its automatic full-wall action on a ray hit is not the original UI jump table, and it must not be declared complete or used as a real interaction acceptance result.
2. finish_wall explicitly rejects snap mode>1 and continued premade shape routes. draw_wall also reports SnappedTopology. Registration cannot restore these topology algorithms.
3. set34 height/merge recording are not standalone canonical consumers yet. The current rectangle `record_color_history` drains without converting to History; this remains an actual interaction/history gap.
4. Native set35 signifier_replay_active has no production system module. Finish leaf1409dd790 has 11 original params including MusicAudioState, but machine code never accesses that stack argument; adding a guessed musical side effect would be incorrect.
5. set42 source is the previous reduced adapter: missing original AppMode/ModeChanged gates, collision/interactions and tail diff flush. Next assigned task is its full reconstruction alongside canonical archive/collision owner.
6. Main App test through the real picker must wait for the actual picker/update state machine. Existing low-level tests using UiState::new/replace_interaction test consumers, not the original input producer. No such fixture is presented as whole-chain validation here.

## Efficient reproducible method

Read the actual registration call-site stage byte, then chain helper body, then exact per-leaf name string + PDB generic SystemParams. Do not infer member names from a generic parameter signature alone. Record full signatures in evidence files and only print concise names; thousands of ICF aliases at shared helper addresses are engine implementation, not game business. Compare current named functions and owners only after the native tuple is fixed. Native direct History.add may be represented by a real typed event bridge, but both producer and drain must be explicitly scheduled before commit. Keep compiler/testing centralized while shared source is frozen.
