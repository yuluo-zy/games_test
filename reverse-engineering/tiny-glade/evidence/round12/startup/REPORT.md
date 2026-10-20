# Round12 startup / camera / audio

Correction: the audio completion channel claims below (zero capacity,
send/recv timeout 1 second) were disproven by original constructor and callee
branches. See [audio-completion/REPORT.md](../audio-completion/REPORT.md): original
is std sync_channel(1), blocking send/recv with deadline=None, consumed once.
That bounded correction is implemented; central validation remains required.

Source frozen for root's single-batch validation. This agent did not run Cargo.
T13 feedback capture and its state producer are deliberately excluded.

## Implemented chain

`run_game_thread(CliArgs)` produces `AudioStartupOptions` from the actual
`fmod_asynchronous` CLI flag. `audio_host::install` creates the original locked
libfmod2.222.6 Studio, loads the five boot banks, retains the real Studio owner,
and starts the original two-pass metadata/sample loader. Its zero-capacity
completion promise is consumed before gameplay Startup. Boot and gameplay Apps
share the same native Studio through `share_core`, with events/VCA retaining the
Arc until their final release. Root integrates the boot/App handover.

The Windows import libraries are linker metadata generated from the original
shipped DLL C exports; no DLL or SDK algorithm is reimplemented. Original DLLs
are copied only into Cargo target profile and deps directories because Windows
resolves static imports before main. Provenance and exact lock checksum are in
`native-audio-provenance.json`. No dependency version was upgraded.

Audio startup/state consumers now include building, clutter, decorator,
elevation supports, forest, menu, path, stairs, replay, roof, sheep, timberframe,
tree, wall, wall UI, weather, trigger boxes, music, loading and global listener/
VCA update. This is a registration/owner claim, not an assertion that all legacy
domain algorithms or their gameplay event writers are fully verified. Time is
the real `bevy_time::Time`, weather is canonical `resources::weather::Weather`,
listener/water position reads canonical MainCamera. Menu's pause snapshot reads
EscMenuState, not the unrelated AppMode enum (PDB run_unsafe@140b27720).

`load_session_audio` now stops old tracks, resets new-glade time, selects tracks
using SessionMeta mutators (Halloween2024 or2025), replaces/stops/starts ambience,
starts and releases optional stinger, writes Winter richness/target richness
(.25, otherwise0), and sets GladeType/IsHalloween independently. Production uses
this owner-aware loader. GladeAudio has the actual Option<String> stinger and
original fallback MainMenu track/Ambience/None, not empty event strings.

The original photo command producer and state/controller are connected in set8,
before camera9/ray10; GUI is independently set39. Actual button clicks0..3 and
next-camera5 publish canonical PhotoModeCmd. Entering Free matches the real
Normal pose once; entering Walk initializes the original authored spawn using
the real collision/terrain owners and teleports Dolly using original look_at.
Mode transitions preserve original Replay exceptions and window focus events.
Photo audio parameters execute immediately after photo state production in set8.

## Original evidence and rejected hypotheses

* `audio-async-caller.asm`: run_game@140593bed reads CliArgs+149; +148 is
  skip_splash. Captured byte at closure+10 flows through audio_init@1400d824c
  into AudioCore::new@140a54a40. `cli-parser-fields.asm` maps original field
  name at1429c3cb4 (17B) through stack+d6 to CliArgs+149. Thus default false
  means Studio SYNCHRONOUS_UPDATE4, explicit --fmod-asynchronous true means0.
  The previous convenience hardcodedtrue is removed.
* `audio-new-prefix.asm`, `audio-delayed-closure.asm`: initial five-bank and
  later all-bank two-pass policies. Eager loading every theme was incorrect.
* `load-glade-audio.asm`, `glade-audio-default.asm`: entire original3927B
  session loader and402B fallback; SessionMeta wins over directory spelling.
* `walk-teleport.asm`: Dolly look_at/rotation/euler body. `raycast-simple.asm`:
  R9=0 at caller is no optional exclusion; the leaf hardcodes solid=true.
  The earlier interpretation solid=false was explicitly rejected.
* `photo-plugin.asm`, `photo-tuple-pdb.txt`: set8 tuple is original do_photomode,
  photomode_buttons,ui_slider, hiddenui. `gui-set39-pdb.txt`: a different tuple.
  No false photo39 or photo before38 scheduling remains.
* `trigger-audio-core.asm`: camera segment/AABB trigger path and original
  six-lens flag table. Its d8 override is written by rollercoaster_update
  (PDB run_unsafe@140377d30), not ordinary forest growth.

## Validation limits and next exact patches

`fmod-api-probe.json` is a real ctypes probe of shipped native DLLs: create,
initialize,19 metadata banks, five named event lookups and release all returned
FMOD_OK. This does not prove Rust audio scheduling, the complete gameplay frame,
or audible output. Root owns current unified Rust/real App validation.

While frozen, original MusicPlayer::load_tracks@1408b6b10 was checked again:
it clears/releases old tracks/names before creating replacements and does not
stop playback a second time. The current old helper's transaction-like order
and redundant stop are queued in `pending-music-load-ownership.patch`, not yet
applied by this agent. `session_audio_regression.rs` provides a meaningful test
for release-before-create, one-stop, stinger lifecycle and metadata-based
Halloween selection. It is pending integration, not a passed test.

EscapeMenuJustOpened is still not claimed complete. The true producer is
do_gui_escape_menu@1403cf9b0, called by wrapper@1400ae1b0 in GUI39. Input uses
digital27 main_menu (+d8), not digital18 ui_cancel. The condition whose ICF label
says DragState::is_some actually reads TimelapseState (param14) and tests!=2.
No fabricated idle resource or false menu-transition adapter was inserted.
Opening sets Main, preserves Photo mode, exits Replay via History[0,3,0,3] plus
RestoreAppTo(default), or requests default mode; it then emits Opened, sound10,
SaveGame, SaveProfile in order. Closing sound11 then NextClosed does not emit
Opened. The exact recovered policy is staged as `escape_menu_policy.rs`; its
actual Timelapse owner plus original MEI menu/exit/wishlist/settings closures
remain unresolved. Root independently fixed Node::used from original11B
(state_bits&0x30!=0); the prior source's no-op claim was false.

Other open audio chains include original full first-person terrain/rock-mask
sampling and canonical movement owner; signifier sounds; water-raster/ambience
startup; terrain/garden editing audio registration; domain-specific material/
path rasters; and coaster override lifecycle. None were replaced by defaults,
empty events, or synthetic editing-speed resources. Photo button branches other
than camera selection/next remain explicitly incomplete.
