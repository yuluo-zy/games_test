# WP15 Contract — `input` (input map / keybindings / cursor state / replay stream)

Source: `skeleton/B/crates/country-core/src/input/{mod,action_set,input_stream,panic_repro_state}.rs`
Evidence: Ghidra B-patched-EXE decompiles in `analysis/b1_input/`, PDB named entities below.

## Modules & named entities
| module | O named entities |
|---|---|
| `input/mod.rs` | `ActiveInputMode`, `ActiveInputModeTracker` |
| `input/action_set.rs` | `ActionSetAction`, `ActionSetStates`, `ActionSetValues`, `DigitalActionSetActionType`, `DigitalActionState`, `MainActionSet` |
| `input/input_stream.rs` | `InputFrame`, `InputRecordReplayMode`, `InputRecordReplayState`, `InputReplayInitialState`, `InputStream`, `InputStreamSyncPoint`, `InputTranslationState` |
| `input/panic_repro_state.rs` | `PanicReproState` |

## Observed (O)
- **`ActiveInputModeTracker`** = 32 B `{mode u32 @0, Instant+Duration @0x08..}` — `new_default_controller` @0x140958eb0 / `new_default_mouse_keyboard` @0x140958ef0 build it with `std::time::Instant::now()` + duration math; `mode` @0x140958f30; `on_controller_used` / `on_mouse_keyboard_used` (O inline into `systems::steam::update_steam_inputs`); `ActiveInputMode::is_controller` @0x1408c5680 (31 callers; folds with `StructureType::is_fence`).
- **`DigitalActionState`** accessors `pressed` @0x14093fa70 / `released` / `just_pressed` / `just_released`; `just_pressed` dispatches on a u32 discriminant (jumptable) ⇒ 4-state enum.
- **`ActionSetAction`** tag `0`=Analog (8 B payload @+0x04), tag `1`=Digital (byte @+0x01). `just_pressed`/`pressed` check `tag==1 && inner-state`; `as_analog` @0x140948800 returns `Some(Vec2)` iff tag 0.
- **`MainActionSet<A,D>`** generic over analog type (`<Analog,Digital>`, `<Vec2,DigitalActionState>` instantiations); `deconstruct` @0x140946b80 writes per-action `(action-type u16, value)` triples (0xc B; tags 0x100/0x200/…/0x1001 and 0x101/0x201/…); `reconstruct` @0x140947c80; serde_json (de)serialize.
- **`InputStream`** = 64 B `{sync_points: VecDeque<InputStreamSyncPoint>, frames: VecDeque<InputFrame>}`; `InputFrame` stride **0xd0**, `InputStreamSyncPoint` stride **0x60** (snapshot of `InputTranslationState` 88 B + frame index).
  - `add_frame` @0x140949190: recording && frame_count ≥ 9999 → trim (frames ≥ 50000 & sync<2 → drop oldest frame; else drop oldest sync point); then push frame.
  - `on_history_sync` @0x1409494c0: !record → truncate both; sync≥9 || frames≥10000 → drop oldest sync point; push snapshot.
  - `drop_oldest_sync_point` @0x140949660: pop front, prune frames up to its frame index, re-base remaining offsets.
  - `sync_point_count` / `frame_count` / `last_frame_mut` / `seek_to_sync_point` (O).
- **`InputRecordReplayState`** = 80 B `{stream: InputStream, mode: u64 @0x40 (=0x8000000000000002 on load), index: i64 @0x48 (= -1)}`; `try_load` @0x1409499c0 = `read_to_string` + `serde_json::from_str<InputStream>`.
- **`PanicReproState::is_inactive`** @0x140833230 (17 callers).

## Inferred (I)
- `DigitalActionState` variant *names/order* (`Released/Pressed/JustPressed/JustReleased`).
- `ActiveInputMode` discriminants (`MouseKeyboard=0`, `Controller=1`).
- `DigitalActionSetActionType` = u16 action-type id used as the per-action hash key.

## Unknown (U / typed gaps)
- `InputFrame` / `InputTranslationState` / `InputReplayInitialState` / `ActionSetStates` / `ActionSetValues` field layouts.
- `MainActionSet` field layout and the exact `deconstruct`/`reconstruct` packing (tags observed; payload semantics U).
- `PanicReproState` layout.
- The default key-binding *semantics* (codes) live in `game_key_bindings` (see app-shell contract).

Reused deps: `std::time`, serde_json, bevy (states consumed by `systems::backend::input`). No fabrication.
