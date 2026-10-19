# Actual proposed-position subtype, complete original1893B

Owned task TG-a790a7eb5fc820c629 was live-showed NULL and normally claimed by rhapsody-correctness-20261019 under root's five-task authorization. This design covers original singular derive_decorator_subtype140813d00/1893B (PDB456:13968), its necessary shared half-dormer kernel998B and two attachment leaves6B/44B. It does not replace the entire per-frame subtype producer, rewrite mesh loading, or call a cached old ID subtype for a new proposed position. No source candidate, Cargo or original target execution is included before independent design review.

Original B/PDB are the same f507a6ce/2d9686b8 identity. The complete physical1893B is451 instructions and its primary table142a8f990 has eight exact entries. Existing old decompiler comments that its jump table is unrecoverable are superseded by this finite PE table read. Source before remains the actual current file, not an old reconstruction snapshot. Existing numerical code and canonical resource owners are reused; there is no new asset catalog or World.

## Necessary API and shared owners

Add the singular callable in this owned file:

```
derive_decorator_subtype(
    info: &DecoratorInfo,
    storage: DecoratorStorageAddr,
    derived: DerivedDecoratorInfoView<'_>,
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
) -> DecoratorSubtype
```

The original call uses an input pair{&Info, actual16B storage address}, actual mesh library, actual PublicWalls and actual Roof query. The new canonical View7506ddc7 already borrows the real subtype and mesh resources. This singular computation uses `derived.mesh_library()` for the actual half-dormer collision; it never asks `derived.subtype(info.id)` to decide the proposed position. It returns the actual subtype only, without cloning an Arc/creating a legacy DerivedDecoratorInfo bbox record or mutating the per-ID map. Parent calculate then combines that real subtype with actual info.rank for its own required collision entry, exactly as original.

The storage WallId is independent from Info+00's packed coordinate floats and Info+08's optional platform. Required wall ownership uses storage.as_wall, not either of those payloads. Only branches that originally touch ownership/geometry do so; fixed subtype returns do not acquire a missing wall, cache or mesh dependency.

## Complete dispatch and failures

Primary index is `(ty-0x38) as u8` when<8, else6. Index6 and all ty outside38..3f return24. Secondary destination index is `(dst_tag-1)` for dst2/3/4 and0 otherwise. Thus wall cases0/1 (and native fallback tags outside2..4) share the wall branch. Do not dispatch on a new assumption that every type is a window.

| Type | Wall0/1 | Roof2 | Terrain3 | Stair4 |
|---|---|---|---|---|
|38 cottage|corner3; else valid platform4; else half1/plain0|flat5, otherwise2|original panic|original panic|
|39 gothic|corner6; else valid platform9; else half7/plain6|flat10, otherwise8|original panic|original panic|
|3a arrow-slit|required wall; corner22; else half23/plain22|original panic|original panic|original panic|
|3b|19|21|original panic|original panic|
|3c|16|18|17|18|
|3d|attached12, else11|flat14, otherwise13|15|14|
|3e / every other type outside38..3f|24|24|24|24|
|3f|25|25|25|25|

For38/39 wall and3a wall, require storage.as_wall and actual PublicWalls.get, preserving original failure on a nonwall address or missing wall. Resolve the actual optional roof via PublicWalls.roof_entity and the genuine Query generation/presence checks; a missing/invalid optional roof becomes None here, rather than a default roof or a required-query failure. This optional query occurs before corner/platform decisions, just as the full original body does.

Wall corner attachment is Info byte+20, and the original6B is_attached is exactly `byte!=0`. The original44B constructor takes rectangle index<4 and returns index+1; otherwise panic. Existing current byte interpretation happens to be correct for corner, but it must not be conflated with platform validity.

**Door selection is Info+08 platform WallId::is_valid, after corner priority. It is not Info+21 bottom_as_door.** Original140814277..4295 passes the copied platform qword to140b34f20. Actual zero platform is invalid. The current private derive_one→attachment_flags_of→derive_wall_subtype path uses byte+21 and therefore cannot serve this exact singular API. A concrete fixture with valid platform and bottom_as_door=false must return4/9; invalid platform and bottom_as_door=true must continue to the half/plain calculation. Same-World current storage remains unchanged in both fixtures.

Roof38/39 and3d require actual storage.as_wall, then only optional roof lookup (no invented required PublicWalls.get). Missing/invalid roof is nonflat:2/8/13. Actual flat roof yields5/10/14. The fixed3b/3c/3e/3f branches require no wall or mesh. Arrow-slit requires a wall destination and panics for every dst>=2. Keep all original unsupported-destination panics rather than returning UnsupportedDecoratorType or a default subtype.

## Shared998B half-dormer kernel, lazy required data

The original half helper is complete998B/PDB456:20876. Its ordered operations matter:

1. Actual wall.shape.tag>1 returns false before any collision or height lookup.
2. Optional roof.is_flat returns false. If roof is gable, require its actual rectangle enclosure and gable sides, convert the actual proposed wall coordinate through the existing793B wall_coord_to_world, take the original closest edge, and return false if it equals either genuine gable side. There is no fallback rectangle or synthetic roof.
3. Convert actual coordinate to world again as in original; find the actual mapped outline coordinate using the existing thresholded XZ nearest helper. At this original read point borrow `wall.prepared_path.positions`, requiring genuine readiness; do not use `height_outline()`'s current raw-geometry fallback. Read only selected/next arch heights at native+e0/+e8 after genuine bounds checks. **Do not use independent third segment_heights(+f8).** Do not add a whole-array length-equality prerequisite or scan unused height elements on the new native path.
4. Only now require the exact mesh key `{subtype=(0x160600 >> ((ty-38)*8)) as u8, rank=info.rank}`: subtype0/6/22 for cottage/gothic/arrow. The B key constructor byte order is already established by5B140b25c90. There is no rank/mask fallback; absent exact key is the original visible failure.
5. Interpolate selected arch height `h0+(h1-h0)*t`. Read real collision BBX minimum/maximum Y and evaluate `((max-min)+(max+min)*.5)*.5 + actualWorldY > interpolatedHeight` with strict ordered comparison and original f32 operation order. Do not replace the actual collision with fixture dimensions or a metadata default.

The present public is_half_dormer accepts an already resolved slice/bounds and has legacy whole-array/finite validation. Eagerly computing those values for the new API would make a valid shape/flat/corner early return fail on missing mesh/heights, unlike original. The narrow implementation should share one private numerical/geometric kernel, with lazily called mapped-outline, selected-height and collision suppliers at the actual read points. Existing public API can wrap that kernel with its current supplied-slice/bounds validation to preserve current callers; the new exact singular path supplies actual prepared positions, real wall.arch_segment_heights and the exact canonical mesh lookup only when read. Keep the old public context/type usable; no second numerical algorithm or heap-remapped curve is necessary. Providers are borrowed closures, with no allocation or clone of a resource owner.

The existing old per-frame producer still calls its finite partial derive_one and is not claimed restored by this packet. Its current byte+21 door choice and wrong height input are explicitly recorded, not silently propagated into the new exact callable. Once actual producer scheduling/removed-ID behavior is recovered by its owner, it should call the same exact singular computation and publish the actual canonical subtype map; do not add a second cache now. This packet does not mark the producer or the full UI path accepted.

## Current consumer closure and candidate order

Parent calculate ownedTG602 must call this new API on both actual proposed Wall and Roof branches, using the real input storage address, actual unchanged info, canonical View7506, actual PublicWalls and actual Query. It then requires the corresponding actual collision key. Nonempty wall/roof merge helpers use the same View for existing selected ID subtype and its actual mesh entry; they are distinct from the proposed-position subtype computation. Original callback receipt/gen/cache owners remain canonical.

The typed destination/core codec and real512B move conversion design are a separate independently reviewed package (TG431/root old owner); this singular API neither introduces a parallel Dst nor writes foreign core files. Both actual UI systems still need the full parent and actual live values; a correct subtype leaf does not make their existing stubs active gameplay restoration.

Before implementation freeze current exact source SHA, canonical View7506 source hash, PublicWalls/Roof/WallId interfaces and every helper used. After design review, write a candidate only in this task work, without changing unrelated source/producer logic. Root integrates and compiles; do not use old core rlibs, new mock crates, invented resource defaults or a repeated complete build known to fail E0583.

## Focused tests required, not run

- Real ECS World Query and actual canonical PublicWalls: all finite primary/secondary table cases, including fixed types with an empty World/map so no spurious geometry dependency can hide.
- Cottage/gothic actual corner precedence, valid platform with false door flag, invalid platform with true door flag; two different proposed coords sharing an old cached ID subtype must compute from new position rather than that cache.
- Actual optional roof missing, stale/missing Query entity, real flat/nonflat roof and required wall address/generation cases; fixed subtypes must not require an owner they never read.
- Exact0/6/22+rank collision key, no fallback; missing mesh after actual required height reads fails, while shape>1/flat/corner/platform branches succeed without an unused mesh/height cache.
- Use distinct arch and third-height arrays so the old wrong array produces a different answer. Check strict equality threshold and actual selected segment, without full-array prerequisites on unused values.
- Keep existing finite legacy helper tests, plus one shared-kernel fixture proving the wrapper and exact actual-data path agree for the same legitimate geometry.

No tests were compiled or executed in this design. Full source/consumer review and central same-source validation are still required.
