# Startup/session lifecycle audit — original B, 2026-10-19

Production source is frozen by central-owner request. This delivery contains **candidates only**, not a Cargo pass or completed real-App/GPU acceptance. It excludes all T13 capture/producers.

## Reproducible evidence

- `verify-startup-lifecycle.py` reads the original RNE, verifies SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`, retains 23 complete function ranges, byte hashes, exact PDB generic signatures, original f32 constant bytes, and current source hashes.
- Results: `startup-session-lifecycle-evidence.json`, `startup-session-lifecycle-original.asm`.
- `build-lifecycle-candidate.py` creates only evidence-folder mirrors and `startup-unload-owner-candidate.patch`; it never writes production source. Its manifest records exact source bases and candidate hashes.
- Patch application check: `git apply --check --ignore-whitespace` passed. Rustfmt parsed every complete candidate with output discarded, preserving unrelated source formatting. Cargo was deliberately not run by this agent.
- `init-photomode-session-reset.patch` is a separate ready candidate. It also passed application check; central integration must retain concurrent source updates.

## Actual scheduling, correcting the older report

The complete builder `add_game_startup_systems140941110/134B` plus actual PDB generic helpers establish:

| Batch | Actual schedule | Evidence |
|---|---|---|
| 1 | PreStartup | helper140b67620/608B |
| 2 | OnEnter(AppLoadingState::Game) | builder141452870, helper140b66830/396B |
| 3 | OnExit(AppLoadingState::Game) | helper140b66480/610B |
| 4 | Startup | helper140b67b50/316B |

The older interpretation that batch3 and batch4 share OnEnter because DL=1 was incorrect. PDB's real generic schedule type is decisive; standalone void() procedure placeholders are not ABI evidence.

Unload builder140b3f550/951B has the actual outer `.chain()` type `(closure_env$1, (uninit_clearing, despawn_glade_entities))`. Its nested pair is built by `into_configs140a11930/839B`, with no inner `.chain()` established. The candidate keeps `(begin, (uninit, despawn)).chain()` and does not invent a uninit-before-despawn edge. Bevy owns deferred command application.

## Narrow complete candidate: photo session reset

Current first-session free-camera consumers and real Dolly owners are present. The missing reset is per-session `PhotomodeState`, currently inserted only once by `run_game`.

Original run_unsafe1408397a0/323B inlines `init_photomode_state`: +0 scalar3.0/full or1.0/demo; +4 indexffffffff; +8/+9/+a/+b zero; +c false; +d true. Builder140839b80's first descriptor140839c98 places it in the actual OnEnter tuple. `on_enable_photomode` intentionally preserves selected camera mode and free initialization, so using that selective reset as the session reset is wrong.

The separate patch adds the canonical owner reset and OnEnter registration after the existing session loader. Real acceptance must mutate previous photo free-initialization=true, load a second session with a nondefault normal camera, enter Photo/Free, and assert the actual Free Dolly matches the new normal pose. First-session default poses cannot expose this bug.

## Complete observed unload contract and candidate owner operations

The original first closure run_unsafe140b3c930/494B has `EventWriter<ClearCanvasCmd>, ResMut<AutoSaveState>`. It writes false at140b3ca2b, then calls the real `AutoSaveState::close_file140a43840` at140b3ca42. Current production only defines `close_file`; no caller was found in current Rust. The candidate calls the existing implementation, preserving its real sender, session identity, send failure policy, and save-cache/metronome reset.

Original uninit_clearing140a7a5b0/1527B and the typed builder prove all eight owners:

| Owner | Observed operation | Candidate |
|---|---|---|
| AssetPlacementDataLibrary | clear buffer table140b79d30, then path table140b79ad0 | `clear()` in same order; retain allocations |
| AudioTriggerLibrary | drain actual SlotMap via141559330 | reuse current `clear()`; old generation keys invalidate |
| RtWorld | clear actual mesh table140b794c0; dirty+38=true | reuse existing `clear_meshes()`; preserve pending upload and next prefab counter |
| GpuWorld | visit27×c0 slots; clear owned instances/count only | reuse existing `clear_owned_instances()`; retain buffers, dirty ranges, mesh part counts, next ID, external/None slots, global counters |
| Option WaterAudioGrid | retain cells/base positions; zero each +8u64 and +10u32 | canonical cell visibility bits and two i16 sums; `clear_activity()` |
| InteractableForestPlacements | clear/drop0x50B records, set processed+18=false | canonical `processed` flag; move current consumer off Local cache; clear existing records then flag |
| AudioCore | read only; instantiate Loading only if snapshot empty | retain the current real engine owner |
| GameLoadingAudioState | preserve nonempty snapshot; otherwise instantiate/start; loading=true,timer=0 | `Option<AudioInstance>` distinguishes empty from retained Disabled; preserves active owner/release semantics |

Original despawn1408b3510/386B alone has a void() placeholder. Builder140a04980/421B and140b3f550 independently prove `Query<Entity, Without<KeepOnLevelUnload>>, Commands`. The candidate uses this exact canonical marker and Bevy despawn, with no fabricated whitelist.

Current `app_systems.rs` has the full wall-unload consumer and reload state function; run_game already registers walls_on_glade_unload. The candidate retains that complete separate consumer. It does not overwrite either implementation or revert current RT/Nani algorithms.

## Water owner/algorithm corrections independently checked against original bytes

Original init1409ebf20/3955B allocates100 cells and constructs10×10 bases: `t=(axis+0.5)/10`, coordinate=`t*65-(1-t)*65`. Current8×8 was a mistaken inference from64 visibility bits.

Each native24B cell owns base x/z at0, bits u64 at8, and two signed wrapping i16 jitter sums at10. There is no cell.state. Native producer141baefe0/2477B writes only bits/sums. The single frame counter remains Local, so unload resets canonical rolling state while preserving frame progression. Candidate removes parallel Local windows/sums and uses each real base position. Original sample-window immediate43020000 is130.0, not the old annotation260.0.

The actual native volume guard1409ed321 compares the selected **AudioInstance** discriminant to1. Current cell.state was a wrong owner attribution. Source position is base + (sums/active_count)*0.8125 on x/z; y=0. Native distance includes camera x,y,z.

Native1409ed2b8 multiplies count by1/64 before the numerator and attenuation. Complete arithmetic1409ed355..36d is:

```
fraction = count * f32(0x3c800000)             // 1/64
attenuation = fraction * f32(0x42290000)      // 42.25
volume = sqrt(fraction * (1 / (dist_sq / attenuation + 1)))
```

The old source used42.5 and unnormalized count in the numerator, so the old zero-distance amplitude was8× too large. Candidate retains the native multiply/divide/add/reciprocal/multiply/sqrt sequence. Central owner is independently verifying these bytes/math; this agent's source-derived Python references are not described as independent runtime confirmation.

## Loading audio: consumer recovered; real reveal producer still missing

Original loading_audio140b1b1b0/1707B and typed wrapper140b61d30/1065B have exactly `(AudioCore, GameLoadingAudioState, EventReader<StartupRevealStarted>, EventReader<StartupRevealFinished>, GameTime)`.

- Drain all Started events, create/start/release exactly one UI/Loading one-shot for that frame.
- Drain all Finished events; take snapshot to empty before stopping/releasing old active instance.
- Only if loading=true, add canonical GameTime.delta(+28) to timer. At strictly timer>1.0 take/stop/drop, clear flag, **retain timer**.
- init_game_loading_audio140b1ae70 replaces snapshot only; it does not reset existing timer/loading.
- Unload creates snapshot only when empty, then sets loading=true,timer=0. Disabled is retained distinctly from empty. Errors preserve original unwrap/panic policy.

Current production instead emits a UI sound each frame while loading, uses bevy_time::Time, stops repeatedly outside loading, and resets timer after expiry. Candidate fixes these actual consumer branches without fabricating Started/Finished events. Current whole-source search found canonical definitions/registrations only, no writers. Original reveal production and its render/load boundary are therefore explicitly **unclosed**.

## Fresh first-session producer audit and remaining acceptance

**Meadow:** current OnEnter installer runs real init_meadow1408639f0 after actual session load, loads48B placement data, mesh/shader, and emits identity drawable + placement handle. Actual frame setup publishes instance buffers/counts; mesh VAO rebuild owner exists before draw; env5 draw1409b68e0 consumes the same handle. Do not repeat the stale claim that this first-session mesh→VAO edge is missing. Original Flowery has66 placements in1 batch; Summer has197 in4 batches(80,66,21,30); Autumn/Olden/Winter are empty. Default Autumn smoke success cannot verify this chain. Before unload candidate, second load of the same nonempty theme panics on duplicate path because placement library had no clear caller.

**Grading:** current OnEnter load_glade_grading1408bf9c0 follows session load; all LUTs are loaded and published by initialize_grading_assets/GradingLuts::load140b01df0; current().flattened140b00b20 reaches the real final postprocess. Slot0 defaults, saved override1/2, Halloween2024 branch, selected kind, names and actual resource formats are present. Whole-App nonempty saved selection/kind switching and real GPU colors/reload remain unverified here, not absent static producers.

**Largest residual startup gap:** full init_clearing140a78aa0 still only has mesh-loading projection. Original forest0x50B record fields/prefab ownership/entity writeback and actual clearing spawns/emitter population remain incompletely represented; current forest still constructs default prefab commands and retains several known approximations. Canonical processed reset fixes lifecycle ownership, not those missing algorithms. Candidate does not claim full forest or init_clearing reconstruction. Auditing original KeepOnLevelUnload writers for Startup UI/reference drawables is also required before accepting complete two-session visuals.

**Additional concrete missing edge:** canonical ClearCanvasCmd has producers but no current Rust EventReader consumer found. Original clear_canvas140a7abd0/6525B has the full canvas/history/raster/wall reset parameter surface; this is separate from the required OnExit false event and must not be replaced with arbitrary defaults. Parent central owner has been notified.

## Candidate regressions and execution limits

Four new meaningful tests are present in candidate files (not run):

1. Full original outer OnExit chain with a real channel-backed AutoSaveState, nonempty placement file/trigger/RT mesh/owned Nani subset/water cell/forest records/active audio instance, and persistent + temporary ECS entities. Check CloseFile exact session, ClearCanvas false, actual removals, retained RT pending upload/counter, Nani next ID/parts/dirty range/global count, retained water bases and reset flags; reload the same nonempty placement path with a fresh handle.
2. Loading audio actual ECS consumer with recording Studio/EventInstance: three start events yield one one-shot/start/release; reread yields none; finish stops/releases once; unload recreates only empty snapshot; timer=1 retains,1.125 stops; retained timer; no synthesized reveal events.
3. Native water cell layout/100-grid bases, nonempty bit/sums/centroid, duplicate-bit idempotence, unload retains allocation/base and clears true owner; continued producer writes do not restore stale Local state.
4. Actual RNE attenuation constant and normalized volume.

Required central acceptance remains Cargo/type-check of candidate domains, actual second-session photo regression, nonempty Summer/Flowery real App draw/reload, and independent original math verification. These are not inferred from Rustfmt or successful patch checks.
