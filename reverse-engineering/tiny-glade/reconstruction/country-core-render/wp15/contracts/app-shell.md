# WP15 Contract — app shell (`app_loading_state` / `game_config` / `game_profile` / `game_key_bindings` / `meta_commander`)

Source: `skeleton/B/crates/country-core/src/{app_loading_state,game_config,game_profile,game_key_bindings,meta_commander}.rs`
Evidence: Ghidra decompiles in `analysis/b2_meta/`, `b3_profile/`.

## Observed (O)
- **`GameConfig`** (ron 0.8.1 persisted): `GameConfig::upgrade` @0x14090df90 — migration reads int @+0x80, float @+0x84, flag byte @+0x88; when flag clear it zeroes int@0x80 and, if `int@0x80!=0 && float@0x84 > 0.125 && !=1.0`, sets flag@0x88=1 and copies float@0x84 → float@0x8c (a scale-override migration).
  - **Glade bounds** `is_within_glade_shape` @0x14090dfe0 (inlines `clamp_to_glade_shape` + `clamp_to_circle`): non-demo clamps X/Y to a rounded box with half-extent `f32 0x42c28200 ≈ 97.2538`; demo (`_DEV_OVERRIDE_IS_DEMO_MODE`) clamps to a circle radius 65 (`4225 = 65²`). `clamp_to_rounded_glade_shape` also used by the camera rig.
  - `developer_hotkeys_enabled` @0x14090e070, `DevSettings::live_link` @0x14090df40 — tiny accessors (PDB `void()` placeholder; ABI U).
  - Named entities: `DevSettings`, `GameConfig`, `GameSettings`, `ScreenshotFormat`.
- **`GameProfile`** = **0x208 B** (ron persisted): `load_game_profile` @0x140916480 — `paths::config().join("profile.ron")`, `read_to_string` + `ron::Options::from_str_seed<GameProfile>`; on failure logs `"Failed deserializing profile.ron. Falling back to defaults. The error was: ..."` and builds the default (`Uuid::new_v4()` player id + default key bindings), then `write_profile_via_tmp()` and `Commands::insert_resource<GameProfile>` (0x210 B command).
  - `save_game_profile` @0x140917230 → `write_profile_via_tmp` @0x140917360 (tempfile `NamedTempFile` + `ron::Options::to_string_pretty` + `persist`; atomic). `profile_file_path` @0x140916440.
  - Default key-binding sequence (O, from the default build): 26 key slots `{7,8,9,10,0xb,0xc,0xd,0xe,0xf,0x23,0x27,0x2d,0x2b,0xc3,0x38,0xab,0xa5,0x1b,0x49,0x3a,0x51,0x52,0x50,0x53,0x4f,0x4e}` (`0xc3` = unset), mouse buttons `{1,2,1}`, then one more `0xc3`.
  - Named entities: `ExpBackOff`, `GameProfile`, `PlayerId`, `SaveGameProfileCmd`, `TutorialCompletionState`, `TutorialExponentialBackOff`.
- **`KeyBindings`** (profile-persisted): `assign_mouse_button` @0x14089b280 writes `{key u16, mouse u16}` at `self+0x1bc + action*4` ⇒ action slots 4 B below 0x1bc; `assign_key` @0x14089b2a0 (1700-line conflict-resolution body U); `KeyBinding::get_keycode` @0x14089d300 / `action_desc` @0x14089cb60; `keycode_to_str` @0x14089d490 (jump table over a u8 range `(byte+0x7b)<0xf4`), `mouse_button_to_string` @0x14089c970 (u16 jump table); `KeyBindingDesc::{text,icon}` (inline). Named entities: `KeyBinding`, `KeyBindingDesc`, `KeyBindings`, `MouseBinding`.
- **`MetaCommander`** = 80 B `{commands: VecDeque<MetaCommand>, profiler_sinks: HashMap<String,(FrameSinkId,Arc<Mutex<ProfilerSink>>)> + RandomState}`; `new` @0x14091eff0, `peek` @0x14091f0c0, `consume` @0x14091f100 (pops front; `MetaCommand` stride **0x38**; empty sentinel `0x8000000000000014`), `undo_consume`, `is_profiling` @0x14091f0f0, `is_empty`/`extend_script` (inline into `finish_timelapse`).
  - **`commander_update`** @0x14091f180 dispatches MetaCommand tags: 0 = session load (`NextSession` + `MetaCommandFence` + `is_session_ok_to_load`), 1 = re-queue, 2 = `insert_resource<TimeUpdateStrategy>` (RT-BVH idle gate), 3 = `insert_resource<InputRecordReplayState>`, 6 = manual time (`from_secs_f32`), 7 = default replay state, 8 = push `LastKnownGraphicsDevice` (**GPU selection list**), 9 = create save dirs, 0xc = start profiling (`puffin::set_scopes_on` + add sink), 0xd+ = stop profiling / write profile (`FrameView::save_to_writer`), 0xe = write GPU performance info, 0xf = write memory log (`MemoryLog` JSON), 0x10 = despawn, 0x11 = `WriteSettingsFileCmd`, 0x12 = `insert_resource<ShutdownGame>`, 0x13 = `std::process::exit()`.
  - `MetaCommandFence::new_pair_blocked` @0x14091ef90 — `Arc<Mutex<(bool,bool)>>` both set; `is_blocked`/`unblock`; attached via `NextSession::append_on_load` so a load stays blocked until done.
  - Named entities: `MetaCommand`, `MetaCommandFence`, `MetaCommander`, `ProfilerSink`, `commander_update::MemoryLog`.
- **`AppLoadingState`** (O named entity) — a bevy `State` enum; only hash/eq/fmt impls visible; **variants U**.

## Inferred (I)
- `GameConfig`/`GameSettings`/`DevSettings` field layouts (beyond upgrade offsets).
- `KeyBinding` slot ordering / action-id table.
- `MetaCommand` variant *names* (labels map tags → ops; see source).
- `AppLoadingState` variant set.

## Unknown (U / typed gaps)
- `GameConfig`/`GameSettings`/`DevSettings` layouts; `ScreenshotFormat` discriminant order.
- `GameProfile` remaining ~0x100 B; `Tutorial*` layouts; `ExpBackOff` math.
- `keycode_to_str`/`mouse_button_to_string` string tables.
- `assign_key` conflict-resolution detail.

Reused deps: ron, serde, uuid (PlayerId default; **not** pulled into the standalone crate — typed gap), tempfile (atomic profile write), puffin, rhapsody (GPU queries), bevy_state (AppLoadingState).
