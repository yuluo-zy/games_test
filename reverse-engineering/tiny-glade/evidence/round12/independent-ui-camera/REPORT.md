# Independent runtime and camera wiring review

This reviewer did not author brush-runtime-full. Read production source and
fresh original B RNE bytes; no Cargo/rustc, production edits or extra LLVM work.
Original SHA and the reviewed source hashes are frozen in
`independent-original-evidence.json`. The reproducible independent extraction
is `../sheep-control-runtime/verify_second_batch.py`.

## Four restored algorithms

Observed independently, without evaluating an author's model:

- Parsed actual original PE import descriptors and RIP-relative thunk bytes:
  142923fd0 → IAT1431931a8 → **sinf**;
  142923eb0 → IAT143193128 → **exp2f**;
  142924060 → IAT1431931d0 → **truncf**. All come from the original CRT math API
  DLL. Replay's raw body has two sinf, two exp2f and one truncf call sites.
  Current source uses sin/exp2/trunc, not the discarded exp/floor hypothesis.
- Decoded Human's actual five-entry relative jump table142a9f3b0:
  [1408b2ea0,1408b2e07,1408b2e90,1408b2e07,1408b2e07]. Phase0 triggers;
  phase2 compares zero with timer, and jb rejects positive or unordered timer.
  Current canonical DigitalActionState::just_pressed implements exactly
  phase0 OR (phase2 AND timer<=0), including NaN rejection. Debug+f8 is the
  gate. Current transform insertion retains height*0 and half-height, including
  NaN behavior; existing transforms are removed rather than overwritten.
- UI2D's current actual Query writes only extent.pos from cursor, preserving
  size/unmarked entities. Its unit marker matches original ZST descriptor.
- Mesh's original indexed collect loads actual u32 index order, checks position
  bounds and calls the original terrain sample1409e9640 for x/z. Current source
  uses real Handle<Mesh>/AssetMeshLibrary, indexed sampling and set_attribute,
  not a Transform shortcut. Current get_mut produces the owning modified_meshes
  receipt and set_attribute releases/replaces only the position attribute.
- Replay preserves None early return after mutable elapsed access, elapsed
  advance with empty queries, maxss-style period, trunc fractional phase,
  empty-buffer cursor translation, original up-vector signs, real look_at,
  shared phase mutation across rows, index1 and other-index offsets, exp2/sin
  decay and existing scale. The source uses the existing ReplayBuffer twice as
  the native signature does, and reexports the original elapsed owner.

No additional algorithm defect was found in these four source bodies during
this static review. The author's40/4/7/90 native corpora and the central Rust
tests remain separate evidence; this reviewer does not count them as new
independent executions or claim the newly compiled source has passed them.

## Actual camera assembly

Observed source paths now exist and should not be re-added from stale reports:

- The application creates a real MainCamera with six original bodies and Dolly
  rigs. It enables country-core native-physics and installs real terrain,
  RaycastWorld, PathRaster, WindowSize, MainCamera/CameraSnapshot, DebugGui,
  session, depth, cursor, Vermeer and camera events.
- Set8 runs photo visibility/transitions and camera command consumption. Free
  entry matches the current Normal body once; Walk entry uses authored x/z,
  actual terrain height and collision ray, then teleports the existing rig.
- Set9 installs reset before snapshot, then active main/paused camera branches.
  Main camera uses real input/modifiers/cursor events, selected translation and
  rotation, all six lenses, current matrices, terrain/collision constraints and
  ground-source publication. Free and Walk use current Dolly owners, not a
  no-op camera Resource. Set10 runs camera ray, actual terrain hit, capture and
  depth update. Paused/active and session flags remain the original gates.

These are source/registration observations, not a real GPU/physical-input pass.

## True remaining entry gaps found

1. Current actual game App invokes toolbar::configure_app only. It does **not**
   install ToolbarPlugin or directly register render_taffy_ui_to_widgets. The
   normal UI click producer is consequently absent from the actual schedule.
2. PhotomodeButton has definition/registration/consumers but no actual entity
   producer anywhere in the source tree. PhotoModeCmd writers are the button
   consumers of UiClickEvent. Therefore source camera algorithms and command
   consumers being present do not close physical UI→Free/Walk mode entry.
   Mounting an empty Query or seeding PhotoModeCmd would not repair that gap.
   The complete original photomode UI producer/layout, real widget factories,
   and toolbar interaction publication must be restored and mounted together.
3. At the reviewed audio_host source hash, the actual App Update list lacks
   firstperson_steps, garden_audio, terrain_audio, water_audio and water_ambience.
   The broader core add_audio_systems helper contains them but is not invoked
   by the actual App. PreStartup load_audio's four resource commands initialize
   owners; they do not run these consumers. This was confirmed with that
   initializer's owner and reported centrally. It requires original set38
   wiring, rather than redoing the recovered footstep/material algorithms.

The central owner has assigned the real photomode producer/toolbar chain as
the next restoration task. Current findings do not authorize replacing the
large original producer with a handful of invented static buttons. T13 remains
excluded; rhapsody and the optimized shared libraries remain untouched.
