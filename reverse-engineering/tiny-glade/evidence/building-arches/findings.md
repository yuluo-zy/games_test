# G05 actual arches CPU owner

## Original owner and three-version filtering

Source ownership reuses `../building-structures/three-version-source-provenance.json`: A/B/C private game sources include `crates/systems/wall-constructor/src/construct_arches/{mod.rs,profile.rs}`, `arch_walker.rs`, and `segmentation/wall_path_segment.rs`; matching paths/checksums establish ownership, not A/C behavioral equivalence. B exact native call map is `../building-structures/B-structures-native-call-map.json`.

Native original exports are `../building-structures/B-arches/` and `B-arches-writers/`. `141237ea0` is Cargo itertools tuple-windows iteration; candidate reuses std `windows(2)`. `141202d50` is std Vec::from_iter allocation around game profile closure `1412083a0`; only the game height policy is restored, not Vec internals. `1408587c0` is actual game BrickTransformSsbo::default; native writes deformation0, surface[-100;3], flags0. These values are proved constructor values, not fabricated fixture defaults. `14089d4d0` packs Mat3 + translation into48B; generic vector/matrix operations reuse glam. Original 96B writer `14120c120` executes, including true free-pool/count/payload/dirty writes; only pinned Cargo single-thread TLS deadlock observer bookkeeping is bridged in the shared environment.

## True caller guards and raw input

B `construct_arches14075f680` is its own wall event consumer, not an internal call of `construct_walls`. Public+296/source+65 **nonzero skips** arches' main mask loop at14075fff5; it is complementary to the accepted special-wall lintel consumer. Caller reads processed136B masks and calls the accepted104B materializer14122ff90 at140760288.

`RegularWallArchesInput` has processed masks, prepared positions/U, path interaction values, shape_tag0/1, Public284 constrained_extra, observed Public294 byte, Public28c original Curve2 source length, WallId and actual validated PublicWalls::has_roof relationship. Public294 comes from Raw64 (independently confirmed by512-copy proof in `../building-production-authority/B-publication-source-fields.json`); authored semantic/default remain unknown, so the API keeps its observed byte.

When Pub294!=0, caller140760628..6f5 selects shared highest(height + groundY) and corresponding groundY across this mask's materialized segments, starting at0/0; otherwise profile selects each segment's first maximum. `create_stone_arch_profile1407645f0` uses Pub284, old segment heights and clipped points to generate actual profile. Weight uses `clamp(oldHeight/Pub284)*.5`, then cubic distance from ordinal center; cubing occurs **before** multiplication by the weight (141208490..4bf). Target lower height is `max(highestTotal-3.5,selectedGroundY)-1-groundY`, floored0. Blend is then capped by true global arch height minus groundY for shape0/1.

## Complete regular outputs and cache effects

`construct_regular_wall_arches(input,&mut arch,&mut third)` returns all ordered96B `WallBrickInstance` records, `ConstructGateCmd120Projection` game payloads and segment_count. Parent owns real eligibility/dirty/delete and sharedlib export.

ArchWalker141232bb0 uses true step bits3f0f5c29 (≈.56), tail ratio.1, profileHeight+groundY points and real WallSpace/U interpolation. Actual closure141231af0 emits96B. Seed is wall-global arch segment ordinal plus persegment emitted index; material field is the already proved BrickSourceId::from(WallId), not a guessed render material. Wall distance pair is sourceU × Pub28c, with actual endU+1 seam unwrapping. Default deformation0/surface[-100;3]/flags0 are taken from the real default ctor. Matrix uses tangent×length, rotated up×bits3ed70a3e, side×.7 and actual segment center.

For has_roof && this mask materializes exactly one segment, original1407607ea..b34 clones the **profile-mutated**104B segment into a120B ConstructGateCmd game payload, then sends it into the Bevy Events128B element (120payload+8EventId). Safe projection preserves owned Vec contents/counts, source ordinal pair/LoopRange, WallId and wall-global ordinal. Last4 padding of104B, allocator addresses/caps and dependency EventId are omitted. The candidate returns the command, never accepts or maintains Bevy event_count. The native full clone/send body executes; the observer checks true send bookkeeping but leaves it outside the game API.

Caller140761288..1770 accumulates current profile heights into third by true LoopRange index and maxnum. Caller140762953..29b1 then copies first→last in **both** third and arch arrays for closed regular outlines. No lintel -.1125 is added here. The independent arches writer must run before normal wall geometry consumes final third/cache authority; any source/height/path/holes/roof/processed-mask change requires proper cache initialization and recomputation, and removal clears old CPU records/associations. Capacity-growth return from original SparseWriter is not a gameplay dirty event.

## Frozen regular proof, pending independent review/World

`B-regular-arches-verification.json`:130 whole CPU cases,4159 ordered96B records,290 game ConstructGateCmd120 payloads, all meaningful bytes and complete arch/third arrays match. 38 cases cross real SparseWriter pages (max117 records). Cases include negative ground, cap/height changes, Pub294 bytes0/1/255, actual SourceIds including reserved values, roof/no-roof, clipped/split masks, empty masks and65/129/257 point inputs.80 cases obtain masks from actual original calc/platform producers; their original upstream inputs and typed masks are in the report. This proof consumes prepared/mask authority; it does not claim the original outer ECS reader/HashMap shell or upstream Rust publication was executed in this probe.

All native calls assert actual return or caller edge. For dense257-point circle tests, original upstream calc/platform5M emulator budget was insufficient; **only this new oracle invocation wrapper** resumes exact RIP/registers with a bounded extra30M instruction budget and then requires actual return. Accepted fixture files are untouched. No original game body is hooked or replaced by a rule bridge. Report pins this wrapper and each imported oracle source.

```sh
cd reconstruction
cargo build --release --bin wall-arches-probe
cd ..
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_arches.py
```

The gate-command semantic correction supersedes the initial reviewed API; all CPU formulas are unchanged, with formal reexecution binding the new counter-free API. No edits to prior accepted lintel or shared runtime/Cargo.

## Next shape2 gaps

Shape2 shares the same profile/walker/instance owner but has additional proven branches, not accepted as complete:

1. Profile endpoint condition: range touches global start0 or end prepared_count. Original trims XZ curve by .1875 distance (normalized fraction capped.95; both-end branch additionally capped.475 (literal bits3ef33333)), resamples XZ at same ordinal count, leaves clipped pointY and U projection distinct, and returns actual normalized trim interval for later third-height resampling. Need verify privateCurve2 slice_into1411f8130/crashy_slice1411f8640 against existingCurve2 operations, not substitute ad-hoc endpoints.
2. Endpoint height weight blends cubic center profile with one-side quadratic according to original inverse_lerp(segmentXYZlength,lowBound,1), clamped0..1. Literal lowBound2.0 is now bound by native14292e6f0; highBound1.0, no guessed scalar.
3. Third writer for non-[0,1] trim computes normalized ordinal over original point count, remaps into interval, samples profile heights by curve2 coordinate and clamps. Must restore this real consumer, not directly write local ordinal height.
4. If ClutterVersion::is_initial(Pub295==0) and Pub288+.92>Pub284, caller14076179b..2821 clones arch; for each segment it raises its range to max(public **uncut** preparedY + profileHeight)+.92, applies **15** exact .5 Jacobi neighbor smoothing passes, then merges max with original arch. Shape2 skips closed seam copying. Need whole original block and branch matrix proof.
5. C wooden_arch_frame source is an independent additional consumer; source path existence is not behavior proof. GPU/material upload, outer ECS transactions, source parameter creation defaults and A/C execution remain outside current regular CPU proof.

## Correction: alleged deferred resource is actual ConstructGateCmd event output

Subsequent exact Win64 argument mapping supersedes the resource interpretation above. RBP=entryRSP-3e8, so RBP+428 is entry+40, **argument8**, not7. The full native Bevy signature identifies argument8 as `EventWriter<ConstructGateCmd>`; neighboring actual loads identify PublicWalls/Masks/Assets/UiSignifier/ArchSegments/BuildingHadArchesLastFrame/roof events and Query consistently. The128B record is a120B game gate command plus8B Bevy EventId. Resource+20 is Events.events_b Vec and+40 is Events.event_count; the independent increment is precisely Bevy send bookkeeping. See `../building-arches-freehand/B-gate-event-owner-contract.json`, pinned native instructions and exact reused Bevy0.16 source.

Earlier `DeferredArchSegment128Projection`/`allocation_index` terminology was **incorrect** and has now been replaced with counter-free `ConstructGateCmd120Projection`/`WallArchesOutput.gate_commands`. Those measurements remain byte comparisons of original output, but the counter is excluded dependency machinery, not a missing game authority. Runtime should send the owned104B profile projection, WallId and ordinal via actual Bevy EventWriter<ConstructGateCmd>, and recover the gate EventReader consumer. No captured counter or custom Events implementation is needed. Events default0, update/clear retain monotonic event_count; reuse Bevy lifecycle. Gate creation/update/removal is a distinct original business consumer that must be implemented, not a generic “deferred allocation” association.
