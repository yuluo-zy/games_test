# Independent startup review — 2026-10-19

Scope: review of other authors' wall/sky/roof complete startup producers,
constructor stage wiring/canonical event types, and default TOD startup wrapper.
No production source edits, Cargo runs, GPU acceptance, or T13 work are included.

## Verdicts

1. **P1: event maintenance was skipped by resources-only helper ordering.**
   Locked `bevy_app 0.16.0/src/sub_app.rs::add_event` only calls
   `EventRegistry::register_event` if `Events<T>` is absent. Existing helpers
   eagerly `init_resource::<Events<T>>()`; later `App::add_event` therefore
   registers no frame maintenance. This affects more than the constructor's
   four wall notifications: nested wall drawing/height/shape/history, roof,
   color/tree, toolbar, decorator ExternalDecoratorCmd, and DeleteSessionCmd.
   Events can remain indefinitely and condition-based consumers can repeatedly
   act on stale retained events. `EventRegistry::register_event` itself blindly
   pushes; calling it without deduplication can update one event container twice
   per frame. No engine implementation should be copied or replaced.

   `event-maintenance-erratum.patch` creates an explicit 95-type canonical
   `App::add_event` table and invokes it immediately after the gameplay
   `App::new`, before any resource helpers. Existing later `App::add_event` calls
   and `init_resource` calls preserve the live container. The roof
   `RecalculateRoofCmd` / `RoofGeometryChanged` alias and constructor/roof
   `RecalculateWallBricks` alias each have one canonical registration. Current
   input/save World helpers already use guarded EventRegistry registration;
   they remain unchanged. The separate loading App has only the correctly
   guarded input helper and is not given unnecessary construction events.

2. **Wall/sky/roof original startup policy: no additional omission found in the
   reviewed final candidates.** Independently extracted all original function
   bytes using the original RNE SHA and exact PDB bounds, inspected complete
   disassembly, and re-executed each complete function using the author's
   explicit recording harness. Boundary implementations were inspected; these
   runs are independent original-function executions, not an independent new
   emulator. They do not prove real shader compilation/Bevy command application
   or GPU execution.

   - Wall `140a92670 / 2320B`: two mesh additions, two shaders, seven queued
     spawns and exact ordering match. `half_cube` genuinely has empty attributes
     with 18 indices; filling vertices would change the original policy.
     Four spawn bundles have OcclusionCulled and three do not.
   - Sky `140864430 / 636B`: one mesh/shader/spawn. Translation bit patterns are
     `80000000 c0000000 80000000` (`-0,-2,-0`), matching the corrected candidate.
   - Roof `1414547d0 / 1680B`: four mesh additions interleaved with two shader
     registrations/two identity-transform spawns, matching the candidate.
   - Original `load_mesh_into_library 140b1aca0 / 456B` watches the mesh before
     library addition, matching candidates. None of these ten global drawables
     is KeepOnLevelUnload; their original spawn executor identities were checked.

3. **Constructor schedule candidate: reviewed subset matches original stage
   contracts; the complete constructor flow remains partial.** Original
   `add_wall_systems 140493120 / 1413B`, set22 chain `1405389b0 / 1399B`, set28
   chain `14033def0 / 2586B`, and set44 assignment `1400f50e0 / 971B` were extracted
   independently. The candidate extends the existing set19 publication chain
   rather than duplicating it. Its set22 and available set28 producers use the
   original relative order and chain semantics. The canonical wall/roof brick
   recalculation alias is necessary: roof producers and wall consumers must not
   use parallel nominal event types. Original set28 platform stairs/pillars/
   floors and other unclosed producers are not restored by this wiring patch;
   do not report the entire construction path complete. Do not also mount the
   legacy aggregate constructor registration alongside these numbered stages.

4. **Default TOD wrapper: original parsing, ownership replacement and startup
   phase independently verified.** `native_tod_wrapper.py` independently executes
   complete `14089d6f0 / 448B` plus original serde parser `1414051e0`, using only
   allocator/CRT bridges and a filesystem read of the shipped JSON. All 15
   80-byte entries, temp IDs and all 17 f32 bit patterns per entry match the
   actual asset (SHA `e3bd28fb2fd29354de9c8af2d396ca8b45f4eb2b693cccac88a3e647a7a4ab0d`).
   A seeded three-entry old Vec is deallocated as 240 bytes/alignment8, and the
   resource changed tick becomes81. Native Startup registration batch
   `140b67b50 / 316B` and PDB signature confirm this is a Startup TodSequence
   producer, not a TimeOfDay value assignment or per-session OnEnter producer.

   One additional alternate-entry defect is corrected by
   `tod-core-registration-erratum.patch`: the CPU core assembler did not
   construct required TodSequence and still put this wrapper in its session
   schedule. The candidate initializes the real owner and moves the wrapper to
   that assembler's startup table with its existing run-once condition. The
   actual gameplay App already initializes TodSequence and uses Startup.

   Numerical consumer boundary: the old `snapshot_at_tod` periodic wrap algorithm
   is incorrect. Its author has supplied a separate `tod-sequence-candidate.patch`
   and native original-function fixture: 189 total cases, 21 empty-sequence
   panics and 168 successful outputs ×17 = **2856** compared float components.
   Those fixtures are the author's execution evidence, not this reviewer's
   independent numerical verification. Apply and validate that correction before
   claiming the complete TOD sampling chain.

5. **Brush path/Look final candidates independently reviewed and original
   functions re-executed.** Original path140b1c010/608B preserves only native
   modes7/8 and requires exactly one entity. Native branches140b1c1c0 and208
   reject a first archetype containing multiple rows as well as any later
   nonempty matching archetype; `single_mut()` correctly expresses both cases.
   All64 original cardinality/mode/float fixtures re-executed identically. Scale
   x/z are path×1.25, y is SSE MAXSS with source1.0 (NaN selects1). Original
   Look140873de0/1476B traverses every matching row, calls the maintained camera
   eye getter, then canonical Transform.look_at with world Y up. All4 original
   row-cardinality fixtures re-executed identically. The candidate preserves
   translation/scale through the existing modern Transform helper. The oracle's
   camera getter/math destinations remain explicit recording boundaries; no
   independent numerical Look math claim is made. Exact ranges/hashes and scope
   are in `brush-runtime-independent-execution.json`.

## Evidence and validation boundaries

- `native-review-identity.json`: original PE identity and 11 complete byte ranges.
- `reviewed-source-hashes.json`: exact reviewed candidate source identities.
- `wall-startup-native-policy.json`, `sky-startup-native-policy.json`,
  `roof-startup-native-policy.json`: this review's original executions.
- `original-tod-wrapper-execution.json`: independent native wrapper/parser result.
- `event_registration_probe.rs`: actual locked Bevy App reproduction **passed**.
  Bad ordering retains one event after four frames. Early App registration,
  subsequent helper init and two further App registrations preserve one event
  after one frame and expire it after four frames. Bare late EventRegistry
  registration alone cannot clear already unchanged old data; it starts updating
  after a new write. `event-registration-execution.txt` records actual output.
  Standalone rustc reused the existing locked Bevy rlibs and windows-targets
  import library, with lld-link; no Cargo run or dependency reimplementation.
- Both erratum patches passed `git apply --check` against the production source
  at generation. Event module passed rustfmt syntax parsing. Full integrated
  compilation and real gameplay session lifecycle remain the central owner's
  validation responsibility.

An attempted whole95-type standalone metadata compile mixed existing feature-
distinct Bevy rlibs and placed a pub(super) module at crate root; it is invalid
as an integrated typecheck and is excluded from positive validation evidence.
The real App lifecycle probe uses one consistent existing App/ECS artifact pair.
