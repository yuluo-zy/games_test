# tg-audio notes

Docs: A = `evidence/multi-baseline/audio-events-analysis.md`, B = `audio-extra-analysis.md`.
Run: `CARGO_TARGET_DIR=/tmp/tg-standalone-target cargo test -p tg-audio` and
`... cargo run -p tg-audio --example audio_demo`. 97 tests, all driven through a real `bevy_app::App`
(pure-table/math tests excepted); coverage not measured with a tool.

## Dependencies (baseline B, `skeleton/B/Cargo.lock`; versions pinned in the workspace root)
- `bevy_ecs =0.16.0` (features `std`, `bevy_reflect`): `#[derive(Event)]`, `Resource`, `SystemSet`, `Commands`,
  `Res`/`ResMut`/`EventReader`, `run_if(resource_exists)`. Used as-is; no hand-written event/system/app layer.
- `bevy_app =0.16.0` (feature `std`): `App`, `Plugin`, `Startup`/`Update`, `add_event`. `AudioPlugin` registers
  the 16 events and the systems; tests use `App::new().add_plugins(..)`, `world.send_event`, `app.update()`.
- `glam =0.29.3`: `Vec3` in event payloads / backend calls.
- Not used (not needed): `lerp`, `log`. B's country-core audio also uses puffin/anyhow (profiling/error plumbing; no behavior).
- NOT a dependency: FMOD (libfmod / FMOD Studio) is a closed native library that cannot be reused here. The FMOD side stays
  behind `AudioBackend` and lives in the `AudioBackendRes` resource (`Box<dyn AudioBackend + Send + Sync>`). Tests install
  `RecordingBackend` (cheap clone sharing one log) and assert on it.

## Implemented (module -> source)
- `registry.rs`: `add_audio_events(&mut App)` = 16 `add_event` calls in documented order + register VAs table (A 1a); 3 systems names of `add_audio_systems` (A 1b). Order test compares `Events<T>` ComponentId ascending.
- `plugin.rs`: `AudioPlugin` = events + sub-plugins; `AudioInit`/`AudioFrame` system sets chained in module order (real order UNKNOWN, A 1b).
- `events.rs`: the 16 `#[derive(Event)]` types; the 9 without recovered payload are unit structs (UNKNOWN).
- `resources.rs`: `AppMode`, `WallVolumeSource` inputs. Each subsystem file also holds its own `*AudioPlugin`, state `Resource` and init system (Startup).
- `wall.rs`/`wall_system.rs`: wall init (10 loops, count 3 / max 50), kind 0-23 table with jump-table VAs, params WallHeight/FenceLength/HasBricks, loop vs one-shot, frame-tail start/stop + flag clear, editing_speed = source/60 (mode 13 -> 1.0) (A 3, B 1).
- `building.rs`: 7 loops init order; event kinds 6/7/8 -> Shapes/Detach|Attach|PeakReached with editing_speed + BuildingSize; kinds other than 6-8 skipped; tail kind->loop table (data only) (A 4, B 2).
- `stairs.rs`: 6 loops, kind 0-18 table (16 ignored, 17/18 = support), loops stopped at frame start (A 5, B 3a).
- `menu.rs`, `sheep.rs`, `clutter.rs`: kind tables + dispatch + AppMode snapshot gate (B 3d-3f).
- `music.rs`: MusicRichness / NewGladeTime formula, snap-off, mode gates (B 4); init instances (A 6a).
- `update.rs`: 5 VCA volumes, runtime (Update) vs boot (Startup) listener (A 2).
- `backend.rs`: `AudioBackend` trait, `AudioBackendRes` resource, `RecordingBackend`.
- Host-supplied inputs (systems idle while absent): `AudioSettings`, `ListenerPose` (update), `MusicInputs` (music). Defaults inited: `AppMode(0)`, `WallVolumeSource(0)`, empty `BuildingTable`/`ClutterNames`, identity `MusicLerp`.

## UNKNOWN (marked `UNKNOWN(evidence: ...)` in code, no behavior built on it)
- Menu kinds 3/5/6/10, sheep kind 2 name, sheep kind 4 meaning, clutter kind->name (39 kinds; default table inert).
- Stairs parameters (stair_width, size, ... value sources) - names recorded only; PlayStairsAudioCmd extra fields.
- Building tail-path trigger; BuildingSize inverse_lerp range (`BuildingTable` holds the normalised value); behavior for missing building id (we omit BuildingSize).
- `time_continuous_lerp` (music smoothing; `MusicLerp` resource, default identity), MusicPlayer::update state machine, third music init instance, which instance is the MusicPlayer event.
- Camera->listener math, boot default listener pose; wall `[state+0x280]` powf modulation; wall volume-source type.
- tree_audio mapping (PlayTreeSoundCmd only declared), terrain_stairs / water ambience / decorator / other systems (constants only in docs).
- Initial `event:/SFX/Stairs/` prefix on stair one-shots and `event:/UI/` names are inferred from string-length/address spacing (tests confirm all lengths).

## Test -> evidence map
- wall `k0..k23` macro tests + `table_is_indexed_by_kind`, `*_lengths_match_doc`: B 1 rows; `param_values`: B 1 params; `editing_speed_source`: B 1 tail/A 3b.
- `init_creates_ten_loops_in_order`: A 3a. building `kind6/7/8_*`, `other_kinds_skipped`, `tail_table_is_consistent`: B 2.
- stairs `s0..s18`, `table_indexed_and_lengths_match`: B 3a. menu `named_kinds_play`, `mode_gate`, spacing: B 3d. sheep: B 3e. clutter: B 3f.
- music tests: B 4 (snap boundaries, /120, 0x13, flag, 0.8 cap, mode gate). update tests: A 2a/2b. registry tests: A 1a.
