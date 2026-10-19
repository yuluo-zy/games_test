# Full original location result and geometry closure

Scope: B is_decorator_location_valid14080f470/8143B, same-file ridge1203B and corner200B helpers, required existing API callers only. This is not a bool flip and not recovery of the remaining culling/removal subscription systems. Task TG-719dcf28a173a4b9b1 belongs to clutter-startup-20261020. Central alone writes production. Current cull source before SHA098ca8e51102164ffdefbf49f40e094f87ba1e46316e75c354bf9a6e1325a24c is archived whole. Do not write foreign Storage, blueprint, collisionWorld or Snap source.

## Identities, producer, consumer and failures

Original B f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 / PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911, matching GUID09F1A02A-49F2-F8C8-4C4C-44205044422E age1. Main complete8143B is in the existing validate-investigation/location-outline-native-pack; no reacquisition is necessary. The complete parent1412b84f0/8161B plus original539B validate wrapper is a separate independently passed outer/interface packet. Additional local full helpers: ridge140811440/1203B, corner140811900/200B, three-sample directions14129d600/517B, right distance1412898e0/921B, profile1407c7500/393B. Explicit native references and input-table fingerprints accompany this packet. C file navigation is not proof of C behavior equivalence.

Actual parent validate passes real ID, info, locator storage address, PublicWalls, Roof query, canonical derived resources, canonical mesh library, current cached transform and same-World RaycastWorld. Location itself produces typed geometry only; parent consumes all invalid payloads as actual feedback, or proceeds to original overlap query on Valid. UiSignifierStream is a trailing native argument through Bounds/Snap, but those complete helper bodies do not read it, and location geometry does not append effects. Omit unused stream from the narrow Rust location API; do not remove the parent's real feedback stream.

Canonical callable shape: `is_decorator_location_valid(id:DecoratorId,info:&DecoratorInfo,dst:DecoratorStorageAddr,walls:&PublicWalls,roofs:&Query<&Roof>,derived:&DerivedDecoratorSubtypes,meshes:&DecoratorMeshLibrary,cache:&CachedDecoratorTransforms,raycast:&RaycastWorld)->DecoratorLocationCheckResult`. The canonical two-reference DerivedDecoratorInfo view will wrap those same resources, not another subtype or opening table. Native current-ID and same-World ownership remain explicit for Snap and later overlap. Info's first qword is packed wall/roof U,Y floats, not an authoritative WallId; WallId comes from actual storage address. Type/rank and subtype are genuine current info/derived lookups.

Missing required mesh/subtype/cache/wall and invalid required roof query keep original observable lookup/assertion failures; they never fabricate zero transforms/dimensions or an invalid result to mask resource errors. Branches that originally return MissingRoof or Valid on optional roof misses retain those specific outcomes below. There is no allocation of remapped prepared-path points and no clone of a whole World/library per check.

## Result schema, all eleven reasons

Use a typed public enum, `Valid` corresponding to native tag11, and Invalid carrying one of these real payloads. Exact Rust layout need not counterfeit the native tagged-union ABI. Preserve input/output values and tag mapping; semantic names are reconstruction names, not recovered author names.

| Native tag | Proposed reason | Native payload / actual consumer |
|---|---|---|
| 0 | WallCutoutBoundary | WallId + CurveInternalCoord(segment u32 at+10,t f32 at+14); wall-coordinate feedback |
| 1 | RoofBounds | position Vec3 at+4, direction Vec3 at+10, WallId at+20; parent draws position→position-2*direction |
| 2 | WallEnds | left_oob bool at+1,right_oob bool at+2,WallId at+8 |
| 3 | SharpWallTurn | WallId + actual center CurveInternalCoord |
| 4 | WallTop | padding f32 at+4,WallId at+8 |
| 5 | WallCorner | WallId at+8,actual Rectangle2d24B at+10,cornerIndex usize at+28 |
| 6 | BalconyUpperClearance | WallId |
| 7 | FlatRoofForbidden | no payload |
| 8 | GableBandForbidden | no payload |
| 9 | MissingRoof | no payload |
| 10 | WallLowerBoundary | WallId |
| 11 | Valid | no invalid reason |

Parent result1 direction is not dimensions: full parent1412b97db..9838 subtracts two times that vector to form a line. Current struct `{valid:bool,position,Option<Rectangle2d>}` and claimed tag1Valid are incorrect simplifications and will be replaced. `is_valid()` matches only Valid; `get_invalid_reason()` yields the typed reason without losing it. Parent blueprint owner applies schema changes and feedback; this source candidate does not write that foreign file.

## Shared real geometry inputs

WallShapeSnapshot tag0circle/tag1rectangle/tag2open/freehand; native loop boolean is tag!=2, not universallyfalse and not merely premade. WallSpace length is `wall.geometry.original_curve_length`. Borrow actual `wall.prepared_path.positions` (native+90/+98), require preparation, never substitute raw geometry via a fallback. A borrow adapter may expose Vec3 using the pinned glam0.29.3 repr(C) three-f32 representation with compile-time size/alignment assertions and a precise immutable SAFETY comment, or use an existing genuinely borrowed CurvePoints adapter. No heap vector is needed.

The distinct native arrays must not be confused: +f8/+100 is current `wall.segment_heights`, the independent third array initialized from prepared segment-owner count; +e0/+e8 is current `wall.arch_segment_heights()` from real regular/freehand/gable heights. Index and next-index checks remain meaningful, not `.get().unwrap_or(0)`. Collision width is bbox.max.x-bbox.min.x and height is max.y-min.y, not the current simplified dims_wall_space's XZ mapping. Genuine canonical lookup uses subtype/rank; host constructor field names can stay subtype,variant without globally swapping foreign calls.

Prepare center, left and right curve coordinates by clamping projected wall-space U, starting an existing CurveSlice at the actual prepared first point, remapping XYZ→XZ, and extending right using the921B strictf32 method. Distances are currentU, currentU-halfwidth, currentU+halfwidth. The separate TG73ca contract restores that exact API, including finite rounding and degenerate outcomes; no location-private replacement sampler. Actual world-space 3D positions are sampled from the same resulting coordinates.

## Destination dispatch and roof branch

142a8eab0 sends tag2 to roof, tags3/4 directly Valid (native14080f542 writes11), and all other tags including default5+ to wall. Direct Valid occurs before cache/subtype lookups; do not add spurious resource failures to terrain/stair.

Roof branch resolves actual wall roof_entity; absent produces MissingRoof, but invalid required Query<&Roof> is an observable failure. For subtype2/8, require actual cached transform and collisionbbox. Subtype2 width is collision width. Subtype8 width is1.2 for rank2,1.54 rank3,.72 otherwise; full Bounds still enforces actual rank1..3. Call recovered full is_out_of_roof_bounds1561B. On out-of-bounds, return RoofBounds(position=(tx,ty+collisionheight,tz),direction=ZERO for subtype2; subtype8 direction is native normalized quaternion*Z vector). If subtype8 bounds accept, call the complete ridge helper using actual translation and normalized `(rotation*Z).xz()` probe, propagating any invalid result.

Then if real roof is flat and actual DecoratorType predicate forbids flat placement, return FlatRoofForbidden. If packed infoY>=1.9 or roof not gable, return Valid. Otherwise use genuine gable_sides and reject U in either half-open quarter band `[side*.25,(side+1)*.25)` with GableBandForbidden; all remaining values return Valid. Do not infer a roof from the catalog or transform type.

## Wall branch, complete ordered checks

1. Resolve subtype. For subtype1 cottage-half-dormer, require actual wall roof_entity: absent MissingRoof, bad Query fails. Use real collisionbbox+cachedtransform and full Bounds1561B. Out-of-bounds returns RoofBounds at `(tx,ty+collisionheight,tz)`, directionZERO. Only acceptance continues to the remaining wall checks; do not make all wall-dormers legal by subtype.
2. Require current wall, collision width/height and actual prepared path; construct real WallSpace. Sample center/left/right coordinates as above. The center underlying path Y uses `path[c].y*(1-t)+path[c+1].y*t`. For subtype7 call full ridge helper first with actual wall-space world position and native normalized actual center segment XZ probe; propagate invalid.
3. Test original **unclamped** left/right WallSpace.project results. If either is outside, return WallEnds with both actual booleans, preserving both inputs even when the first fails.
4. Visit **left,center,right in that order**, not every intermediate path vertex. For each interpolate independent cut=`wall.segment_heights[s]+(next-current)*t`. Compute lower threshold by subtype:4/9 use `Y+(-.5*collisionheight)` if cut>0 else `Y+.2`;6 uses Y plus gothic_half_dormer_profile_curve(rank).last().y;11/12 use Y;16 uses Y+.3;25 uses Y+.2;default Y+(-.5*collisionheight). If centerUnderlyingPathY+cut is ordered greater than this threshold, return WallCutoutBoundary with the actual failing coordinate when cut>0, otherwise WallLowerBoundary. No cut-array/height-array alias or bbox approximation.
5. If subtype!=3 and shape.tag>=2, map those three coordinates to real3D positions. Their adjacent XZ differences are normalized using native reciprocal sqrt with no zero-vector fallback. Dot of the two normalized directions<.6 returns SharpWallTurn at the center coordinate. Full517B helper establishes actual take(3)/tuple_windows structure; do not inspect arbitrary later curve vertices.
6. Subtype4/9 calls06's exact five-state Snap with actual world position, center path derivative/probe value, wall distance and current info/current wall. Native call14081090d supplies both bools true; preserve them. Only RejectedUpperClearance(tag4) returns BalconyUpperClearance. Unavailable(tag3) is not a fake positive geometry input and simply continues where native does. The real bounded raycast max_toi1/mask1000 companion is required; do not use oldMAX query. Other tags retain original continuation.
7. If subtype cannot be on corner and real wall is rectangle, visit all four original corners in order. Corner U is actual rectangle.get_point_u(corner)*WallSpace length. For type.should_offer_snap_to_corner, strict absolute circular/open WallSpace.subtract between projected corner distance and currentU is compared to halfwidth plus (.2 when actual wall style>=2, else .46). First hit returns WallCorner with that actual Rectangle2d and index. The native200B helper proves U-distance only. Existing source comment3.0 and EuclideanXZ/always-open implementations are wrong;1429cd068 is .2, while unrelated feedback1429cd078 is3.0.
8. Top boundary: for premade shape0/1, an optional roof missing entity/query returns Valid here (different from required early branches). Subtype3 directly uses margin .79 on flat roof else.1. Other subtypes: if type59 or type63 or (type.is_window and !roof.flat), return WallTop(padding0) when Y+collisionheight*(-.25)>wall.max_y(), otherwise Valid. Remaining margin is .69 for flat else.1. Before this rule, actual gable roof with actual center point closest rectangle edge matching one of gable_sides falls through to the per-point arch-height rule; nonmatching edge uses this max rule. Subtype3 bypasses that gable-edge diversion. Offset is margin for subtype11/12, otherwise collisionheight*(-.5). Y-offset>wall.max_y()-margin gives WallTop(padding=margin); otherwise Valid.
9. Freehand, and the diverted gable-side branch, check left then right against genuine arch heights(+e0). Interpolate base+(next-base)*t. Padding=.25 for premade; for freehand .25 for type.is_window else.67. Vertical offset=-padding for subtype11/12 else collisionheight*.5. If Y+offset>interpolatedHeight-padding return WallTop(padding); otherwise Valid after the second actual sample. No default wall height on missing readiness.

## Full ridge helper and numeric dependencies

140811440/1203B applies only to subtype7/8, otherwise Valid. Resolve roof from actual wall and Query; here either missing yields MissingRoof. Circle enclosure immediately Valid. First profileY is gothic_half_dormer(rank) for7 and existing gothic_dormer_profile_curve(rank) for8 (393B full body reads info.rank; its argument is not zero-argument). Add profileY to actual world Y. Get real roof intersection at that Y; OutOfBounds returns RoofBounds. Obtain closest perimeter point from actual rectangle/circle section. TopY>roof.height()+floor-.1 rejects. Normalize actual XZ delta without finite/zero fallback; dot=(delta.z*probe.z)+(delta.x*probe.x). Native ucomiss/jbe accepts threshold .993<=dot **or unordered**; otherwise returns RoofBounds(position=top,direction=(probe.x,0,probe.y),wall). Preserve actual probe values; do not replace with default basis/quaternion.

Rectangle's recovered two-method packet v2 is a required coherent dependency. It preserves first-pair strict tie order, far1000 default, endpointU4=1 and original projection arithmetic. RoofBounds1561B's true rank/table/section/corner interval packet is required. No new Circle/Rectangle/Roof library is introduced.

## Current gaps, candidacy and integration boundaries

Existing current is_wall_decorator_location_valid is unconditionaltrue; roof true; terrain/stairfalse; result type wrong; missing cache silently returns invalidzero. These are actual simplified logic to replace. Current public geometry already supplies real prepared paths, third heights, arch heights and Roof section APIs; these are reusable when genuinely produced. Producer phase/readiness and cached wall/roof transform branches still need their respective owners, not invented current defaults.

Current same-file cull_one call must receive real new inputs and actual storage lookup, not derive WallId from packed info. Its broader current cull systems still synthesize a local DeferredDecoratorOpEvents::default and use partial archival/removal semantics. Do not mount those systems or claim the full culling consumer recovered in this numerical packet. Typed callable is ready for the foreign blueprint validate owner, which must supply original same-World resources and real locator IDs and retain cleanup/backup/UI/mute order from its independently passed outer contract.

Canonical subtype/mesh producer and actual CollisionOutline loader remain separate authorized packets. A synthetic test catalog must not be used as a genuine gameplay-ready catalog or to bypass Startup gates. Tests can create explicit geometry fixtures to verify this leaf, but must label them as fixtures, and distinguish current host helper coverage from original binary execution. Keep all existing unrelated same-file stairs/removal logic unchanged except signature forwarding necessary for the genuine callable. Independent review first, then candidates; root production/Cargo/GPU only.
