# Music recovery — 2026-10-15

## Implemented

- `systems/audio/music/music_player.rs`: owned player/instances, all five native state variants; track loading, callback rejection, shuffle/refill/pop, pause, immediate stop, photo snapshot stop, timeline callback and diagnostics globals.
- `music_system.rs`: actual MusicAudioState and initialization; richness accumulation and exponential interpolation; NewGladeTime; menu/preview/photo snapshot policy. Typed ECS consumers use AppMode, EscMenuState, DebugGui, GameTime, Time<()>, PhotomodeScreen and PhotomodeState. `configure_music_consumers` pipes a real session theme producer into update then debug GUI.
- `music_debug_gui.rs`: optional egui context, original DJ station controls, current track/state/beat/bar/tempo/signature labels, exact force/immediate switch side-effect order.
- `native.rs`: optional native-audio adapter to original libfmod 2.222.6, using Studio/EventInstance directly; real callback registration, parameter writes, pause/start/stop, timeline reads, and release-on-drop. No simulated playback or smoothed=target substitution.

## Primary evidence and corrections

Original B executable SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. PDB identities are in procedures.json; native instruction excerpts in matching VA.asm files. `parameter-types.json` contains the actual FunctionSystem tuple; the menu gate is EscMenuState, not another AppMode argument.

| Evidence | Restored behavior |
|---|---|
| 1408b5500; jump tables 142a9fa24/34 | SwitchTrack → Playing → PlayingLoopedOnce → SwitchAtNextOpportunity → Switching. First loop resets elapsed/ticks. Wait-for-opportunity does not accumulate dt. |
| 1408b5b52..5bba | When elapsed > integer ticks, set ticks=ceil(elapsed) as u64 and sample fastrand::f32; transition if (elapsed/120)^2 > random. |
| 1408b59b0..59fa | Start switching snapshot when bar_count divisible by four and beat=1. A timeline rollback resets bar_count to zero. |
| 1408b5826..5922 | Strict elapsed thresholds: >18 stops current track immediately; >18.5 stops switching snapshot immediately; >19 sets SwitchTrack. Preserve actual command order: switching snapshot, state assignment, fade-in snapshot, track. |
| 1408b70d0 | Refill 0..len, fastrand shuffle; compare first element against previous, swap with len/2; pop the end. Do not reinterpret as sequential playlist or change first-element check. |
| 1408b6b10 | Clear tracks then names; instantiate failures panic; disabled and callback-registration-failed instances are dropped, not retained. Existing playlist/current index are not reset. |
| 1408b7620 | Signed i32 payloads sign-extended to u64; changed bar increments counter, unsigned maximum with bar. Per-field atomics replace the original unsynchronized globals without fabricating musical time. |
| 140a72810 + 140c904e0 | GameTime.delta controls richness and NewGladeTime; Time<()> delta controls player. smooth=current+(target-current)*(1-exp(-game_delta)); threshold snap after smoothing. |
| 140a72ca3 | Menu `(tag | 8)==10` admits Closed and Photomode regardless of photo-menu payload flag. |
| 140a72ced | Camera byte 2 starts snapshot at +70 (PhotoMode) and stops +a0 (FirstPerson); all other photo camera modes reverse these. Names/order preserved even if unintuitive. |
| 14083b480 via vtable 142a93400 | Force switch sets state=3; immediate switch stops switching snapshot then current track, sets state=0, retaining current index. |
| 140a54440 + 14089a280 | Instances default stop-on-drop=None, then release and unwrap. Adapter preserves that lifetime behavior. |

Constants 1.5, 18, 18.5, 19, 120 and preview table [0,3,1,2,4] were checked in the original PE. Callback globals live in the zero-filled virtual tail of .data; raw file bytes past SizeOfRawData must not be read as initial values.

The downloaded official libfmod crate has SHA256 75defbe5991c324e74ac5021dece895eabe5cb5b8976bb33c7e9a13c478826ad, exactly original Cargo.lock checksum. Only public API/enum definitions inspected; library algorithms are reused. StopMode 0=AllowFadeout, 1=Immediate; prior chapters reversed them.

## Review and verification

- Parent independently checked native switch_track, callback signed/unsigned operations, and interpolation helper: no discrepancy reported.
- Parent reports final incremental all-targets `native-audio` check passed (process 85702 exit 0).
- One deterministic Rust test `native_transition_thresholds_preserve_command_order` covers first-loop transition, sample timing, beat scheduling, strict boundary comparisons and call order. Added at parent's request; central execution passed (1/1).
- Claude helper completed one bounded load_tracks analysis (claude-load.*). Its phrase that callback errors abort loading was rejected: original 1408b6d99 branches to drop/continue on callback failure. The implementation uses original instructions as authority.

## Remaining integration boundaries — not claimed complete

- Music now reads the authoritative `Res<ActiveSession>` metadata theme directly. `configure_music_consumers` orders music and its debug GUI; caller supplies the actual shared resources and backend.
- Full AudioCore/device/bank startup, its logging deduplication, GladeAudio file loading/theme transition producer remain outside this music restoration. Native backend requires a real initialized Studio with loaded banks; missing runtime/import libraries have not been exercised. `MusicPlayer::load_tracks` is implemented but the full GladeAudio loading consumer has not yet been restored.
- Global parameter failures warn and continue, preserving the audio control flow; original AudioCore's screen-log/dedup infrastructure remains external.
- No audio playback or live FMOD integration run is claimed. No overall country-core completion is claimed.
