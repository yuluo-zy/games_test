# Freehand arches complete CPU consumer

## Frozen candidate and acceptance boundary

`reconstruction/building/wall_arches_freehand.rs` provides `construct_freehand_wall_arches(FreehandWallArchesInput, &mut arch, &mut third) -> WallArchesOutput`. It shares the accepted typed segmentation/materializer and the regular arch 96B emitter; it consumes actual processed masks, prepared XYZ/U, path interaction values, Public284/288/294/295, raw source length, WallId and actual roof relationship. No captured trim, profile, proposal, walker points, seeds or instance metadata enters the candidate.

Source ownership uses `../building-structures/three-version-source-provenance.json` and its actual B native CALL map. A/B/C original `construct_arches/{mod.rs,profile.rs}` and `arch_walker.rs` paths establish game ownership; they do not prove A/C behavior. The candidate reuses existing Curve2, glam, std allocation/iteration and typed wall source conversion. Private Curve2 slice exports in `../building-structures/B-arches-freehand/` confirm exact adjoining endpoint omission, and are used only as geometry policy around the already recovered Curve2 owner. No generic Vec/itertools/ECS implementation is ported.

`B-freehand-arches-verification.json`: **270 whole CPU cases, 9,691 ordered96B records, 363 game ConstructGateCmd120 projections**, all meaningful bytes and complete arch/third arrays match. 64 cases obtain masks from actual original calc/platform producers;96 are explicit open segmentation ABI cases;110 cover start/end/both/interior and lengths immediately around .1875/.375/1/2. 54 cases cross native SparseWriter pages, maximum905 records. Every full original profile/walker/default/emitter/writer call must return; every composed original caller block must reach its explicit edge. No GPU upload or original outer ECS shell is claimed. Parent owns independent review and actual World lifecycle integration.

```sh
cd reconstruction
cargo build --release --bin wall-arches-freehand-probe
cd ..
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_arches_freehand.py
```

## Actual freehand branches

The actual arches event consumer skips Public296!=0; this producer applies to the complementary regular/special guard with shape2. It invokes original104B materializer with regular=false/closed=false. Highest selection and has_roof && this mask materializes exactly one segment cloning match the regular owner; height clipping differs.

`1407645f0` checks LoopRange start0 or end==prepared_count. It records original segment XYZ length for the one-sided blend. If XZ length>.1875, it trims .1875/length, capped.95; both-end trimming additionally caps at bits3ef33333 (.475). Trimmed XZ is resampled at unchanged local ordinal count, retaining pointY and U. Original private slice omits a bordering interior point when its full XY equals the sampled endpoint; both Curve2 construction and sampling reuse the existing recovered geometry owner.

The profile retains clamp(oldHeight/Public284)*.5 and cubic center weighting, but endpoint weighting blends a one-sided quadratic with cubic using clamp(inverse_lerp(2,1,originalXYZlength),0,1). Side is end-only? (1-t)^2 : t^2. It does not apply the regular global arch-height cap. Target/lower and exact arithmetic order are native-tested.

ArchWalker uses open WallSpace linear U arithmetic, actual .56 step/.1 tail rules and the shared actual 96B closure/default/packer/writer. Seed is global materialized-segment ordinal plus emitted local ordinal; BrickSourceId comes from WallId. Source-distance metadata still uses actual original callback's endU unwrap.

For non-[0,1] trim, the actual caller third consumer remaps original ordinal into the trim interval, clamps, selects Curve2 coordinate from profile-mutated XZ, and difference-interpolates current profile heights. It writes maxnum to the true prepared LoopRange index, rather than copying local heights. With identity trim it uses direct heights.

The separate freehand cache owner runs only if ClutterVersion::is_initial(Public295==0) and Public288+bits3f6b851f (.9200000167)>Public284. It clones arch, raises every affected range to max(uncut preparedY+profileHeight)+.92, applies15 exact Jacobi .5 neighbor smoothing passes with unchanged endpoints, and max-merges into original arch. Shape2 bypasses both closed first->last copies. Normal wall geometry must consume the resulting arch/third after this owner, with source cache rebuilding before recompute.

## Native oracle and retained failure

`native_arches_freehand.py` imports the frozen shared allocation/CRT/dependency bridges and original profile/clone setup. It invokes actual game machine code for all numerical policies. Freehand walker setup changes only true WallSpace closed input to0. Third/cache fixtures reconstruct valid owned104B views from actual original profile outputs and execute `140761288..140761770` followed by `140761770..1407629b1`; these are composed caller blocks, not host replacement rules.

The first fixture failed after repeated mixed block runs: Unicorn reused a translated block across a previous different `until` stop and crossed the requested caller edge. `B-oracle-fixture-failure.json` preserves that failure. New `run_to_edge` installs a temporary instruction hook **only at the requested stop address**, stops before that instruction and asserts RIP equals it; no original game function is intercepted or replaced. Frozen regular fixtures are unchanged. Complete formal proof pins this implementation and all imported oracle code.

## Gate command owner and integration

The actual120B game command contains a profile-mutated104B WallPathSegment, WallId64 and wall-global segment ordinal64. The128B native buffer element includes an8B Bevy EventId; allocator pointers/capacities, unproved4B segment padding and dependency EventId are excluded from game projections. `WallArchesOutput` returns `instances`, `gate_commands`, `segment_count`. The native fixture still executes the actual event send and asserts its event-count increment, but that native-only dependency state never enters the candidate.

Creation/source edit/roof change/mask change/height change must rebuild cache and replace emitted instances and send actual ConstructGateCmd events; the next gate EventReader owns game creation and removal. Old maxima must not survive a lower current profile. Parent owns true EventWriter registration, World chaining and lifecycle review. No custom Events storage or allocator counter is introduced.

## Superseding event-owner correction

`B-gate-event-owner-contract.json` now closes the owner question above: entry+40 is Win64 **argument8**, and actual full signature binds it to EventWriter<ConstructGateCmd>. The alleged resource is **Bevy Events**;128B record=120B game command+8B dependency EventId; counter+40 is event_count. Production now emits owned profile/WallId/ordinal via `ConstructGateCmd120Projection`, and removes every counter input/field. Event IDs, buffering/default/clear/update are delegated to Bevy. The new formal proof is reexecuted and excludes EventId8 from candidate game bytes while retaining native-only event observations. The next original game owner is the gate EventReader consumer.

## Actual World integration and native proof

`crates/tg-building-runtime/src/wall_arches.rs` consumes actual current prepared path and processed masks. Parent adds a structural_arch snapshot immediately after source cache/gable finalization; this snapshot and base_third are the inputs to every rebuild, rather than feeding freehand smoothing/previous maxima into itself. Arches execute after special lintel rebuild/withdrawal and before normal wall geometry. The runtime writes final arch/third, stores all96B instances in an actual Bevy entity/component map, and writes120B ConstructGateCmd payloads through actual `EventWriter::write`. No custom event buffer, parsed proposal Vec or event counter is present. Source/path/mask/roof/cache changes rebuild; idle preserves generation; pending/removal/special switching remove stale CPU entities. Switching to special preserves the preceding lintel third owner.

`B-arches-world-verification.json`: **12 actual frames /22 arches wall frames /318 ordered96B arch records /4 Bevy gate commands /3,654 subsequent96B wall bricks**, zero differences. `scripts/multi-version/verify_arches_wall_runtime.py` verifies actual native preparation/cache/calc/platform masks, whole profile/walker/cache outputs and subsequent whole from_curve. Full final arch/third and every game command payload are compared; EventId is intentionally dependency-owned. Source inputs are actual raw publication, real terrain/raster resources and real Roof creation. The fixture contains no captured masks/proposals/contexts. Roof/source/path changes, idle, deletion/recreation and special switching are represented. Actual current-update Bevy events are compared only on the producer's real rebuild frames.

`wall_arches_chain.rs` has2 actual World tests, including fresh low-source replay equivalence, idle/no feedback, path clear, delete/recreate, special->lintel ownership and actual Roof/strip-path gate emission and no idle resend. They pass. Parent owns independent review and shared registration/stage wiring; the game gate EventReader and door geometry remain the next separate business owner.

The mixed regular/freehand World oracle exposed the same translated-block `until` issue in the previously frozen regular fixture. `run_to_edge` is now shared in `../building-arches/native_arches.py` for all composed caller edges. Empty masks execute the real CMP/JZ (no host empty shortcut), regular closed copies run, and freehand initial-clutter tail runs when its real guard permits. Both standalone formal reports are being reexecuted with the repaired oracle hash before final phase acceptance.
