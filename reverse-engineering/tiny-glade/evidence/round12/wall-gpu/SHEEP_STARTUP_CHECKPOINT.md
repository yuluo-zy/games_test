# Sheep startup / canonical owners checkpoint

Candidate only: `candidates-sheep/manifest.json` + `candidate.patch` (19 Rust files).
Central adapter: `SHEEP_APP_ADAPTER.rs`. No production edits or Cargo/codegen performed.
All candidate Rust parsed through rustfmt. This is not a type-check or a test-pass claim.

## Original evidence

- B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
- PDB author startup_sheep140a093e0..140a0b33f, 8031B. Whole existing-mesh birth
  policy executed in `probe_sheep_spawn.py`, output `sheep-spawn-native-policy.json`.
- Original RNG with raw seed0x16e5c executes, six draws. Original BirdSpotManager
  create_spots and SheepControl::new execute. File loader, shader library/compiler,
  Bevy command executor and current cube geometry are explicit host boundaries.
  Disabled audio case executes; active FMOD and GPU are not claimed by this fixture.
- PDB Commands::spawn<SheepControl>1412e23e0/756B; new1416f0ea0/759B.
  FunctionSystem140f94930 and load_game state140883f00 both confirm Query<&mut
  SheepControl>, not a Resource.
- cache_prev14082bd80/628B and PDB140b51320 confirm unfiltered
  Query<(&Transform,&mut PrevTransform)>; restored leaf is not assigned a guessed set.

## Atomic integration contract

1. Apply already frozen `sheep-startup-assets` six files (root has done so).
2. Apply this candidate's canonical PrevTransform/SheepWalkRecord and actual
   Env2 reader. Retain the independently merged wall-proxy aliases in the same
   views file and CritterControl aliases in callback_b/components.
3. Apply `birdspot-recycle` candidate's canonical core Wren + recycle consumer;
   this candidate already includes its Wren reexport hunk. Add independent
   verifier's canonical Critter marker. Do not copy an old whole components file.
4. Apply independent UI owner's complete SheepControl component/runtime/new.
   This candidate calls `SheepControl::new(wireframe,solid)` and writes its
   typed `snapshot`. Do not keep a same-named Resource or silently default the
   second original Option<Vec> payload.
5. Add the exact adapter in GameplaySessionStartup::Sheep. It uses the actual
   shared shader Arc, SSBO library, physical buffer device and NativeStudio audio.
   Err audio creation remains fatal; Disabled stays Disabled. Remove bare mesh
   startup: registering frame1 before the full loader poisons its cache gate.
6. load_game now installs an incoming snapshot in the unique live component;
   before its birth it stores an explicit PendingSheepSnapshot DTO. Startup
   moves this DTO to the live control and removes the DTO resource. More than
   one control owner is an error. There is never a live Resource shadow.

Original birth order is size -> instantiate Baah/Pitch -> two RNG draws ->
gen_id -> position arithmetic -> owned bird points -> full sheep command,
repeated three times, then control cube -> shader lookup/wire tuple -> second
lookup/solid tuple -> control component. Debug tuples have no Transform at birth.

BirdSpot is the original24B Option<Entity>/Vec3/u32 ID. The manager owns an Arc
of a mutex return queue, point sets hold Box<[BirdSpot]>+Weak. Drop upgrades
Weak and pushes each entire record in original order. IDs increment u32 by1
with wrapping; no old +8 ID reuse pool remains. recycle drains under the mutex
and delegates the exact live-generation/query/state predicates to the independent
consumer. Manager destruction before point destruction is safe.

## Validation still required centrally

- Run `tiny-glade --test original_sheep_birth`: native golden positions/scales,
  three complete tuples, audio parameter calls, actual Env2 component reads,
  GBUFFER_VELOCITY3+SHADOW3 CPU list admission, missing debug Transform, Drop
  queue and fatal audio prefix. Its metadata backend and synthetic VAO handles
  are explicitly CPU admission boundaries, not a GPU success claim.
- Existing `country-core --test sheep_snapshot_native` now constructs real
  control components. Run it with UI owner's updated runtime system parameters.
- Actual intro/v27 saves contain three nonempty sheep rows: before the first
  accepted gameplay drawing, real controls runtime must apply the loaded
  position/goal/move_state to those same three born sheep. Check query state
  and no remaining Pending DTO. A successful empty warmup is insufficient.
- Physical acceptance must observe the real992B animation rows uploaded for
  every actual mesh vertex, actual VAOs for the sheep mesh, three Env2 callback
  invocations reading the canonical live animation state/previous Transform,
  submitted work/readback, and valid reload destruction/rebirth.
- Complete controls picking/second Option<Vec> payload and original cache_prev
  registration are independent evidence dependencies; do not mark them complete
  merely because this birth candidate exists. T13 remains excluded.
