# B snap_balcony_door: bounded consumer contract and design v1

Status: proposed design only. No source candidate, Cargo, native execution or App acceptance. Scope is B original3277B at 14127f0e0, the actual location caller and its necessary finite simple-ray boundary. Claim TG-60234a3eb4a6f6e4d2 belongs to rhapsody-correctness-20261019. Root alone integrates production. Historical A/C task presence is not B/C equivalence.

## Evidence and actual source gap

The original full function has715 verified instructions, no overhang; full nine-support-function pack and two publisher-binding snippets use the same original RNE/PDB identity. Retained full publisher supports context, not a second provider or target execution. Current intended source SHA is da0e2a17575a0e00c27d3900b45b1bf5bfeb1b76574110f36d7dca467ab6b3d8. It always returns None and has no current executable callers anywhere in the workspace. Its old type0x1a/circle/ray comments are wrong: info.ty0x38 and0x39 select variants4/9;0x1a belongs to EMPTY library key. Incoming actual location producer14080f470 calls at14081090d and interprets output tag4 as Reason6{wall}; caller adaptation belongs to01.

Native output is16B: WallId@0, f32height@8, tag@12. Tags0/1/2 initialize the payload,3 and4 only write tag. No native ABI is introduced into Rust. Semantic enum proposal:

- AdjacentFlatRoof { wall: WallId, height: f32 } =>0
- WallOutline { wall: WallId, height: f32 } =>1
- ColliderSource13 { wall: WallId, height: f32 } =>2 (do not invent nominal source13 name)
- Unavailable =>3 (unsupported dtype or finite-ray result other than source13)
- RejectedUpperClearance =>4

`Unavailable` is not a fake completed placement. Caller must preserve its original continuation. A tag4-only caller may match precisely RejectedUpperClearance; it must not lose or manufacture the other outputs for a different actual consumer.

## Narrow canonical interface

Replace only the stub plus necessary private helpers/result type in the owned ui_systems/decorator_interaction_intent.rs. Keep calculate_decorator_interaction_intent and corner stub outside this implementation. Proposed signature:

```rust
pub fn snap_balcony_door(
    require_upper_clearance: bool,
    info: &DecoratorInfo,
    world_pos: Vec3,
    probe_direction_xz: Vec2,
    wall_distance: f32,
    wall_id: WallId,
    wall: &PublicWallState,
    roofs: &Query<&Roof>,
    meshes: &DecoratorMeshLibrary,
    prefer_wall_outline: bool,
    world: &RaycastWorld,
) -> BalconyDoorSnap
```

The two bool names describe observed branches, not recovered author spelling. Preserve actual values: location caller passes true/true. Preserve direction components exactly; do not normalize, swap, clamp or change their sign inside this function. The caller owns the existing normalized source vector. Native unused additional wall-coordinate Y and UiSignifierStream parameters need no new local dependency: all715 instructions have no reads of their argument slots+1d8/+208. The actual parent/cottage feedback stream remains with01; this does not eliminate parent side effects. Immutable Bevy Query<&Roof> is borrowed twice as the original function does. Reuse actual same-World Roof/mesh/world owners; no extra World, fake collection, default mesh, injected query result or backend.

## Exact control flow and numbers

Use f32 in instruction order. All constants have raw original bytes in constants-readonly.json. Keep strict/inclusive comparisons and unordered behavior. A local max-number helper may express original MAXSS+NaN-select: if right isNaN choose left; else if left>right choose left; else right (right wins ties). Do not conflate this with simple ordered maximum or clamp.

1. f14e..f17b: unsupported dtype returns Unavailable before touching any dependency.0x38 uses library subtypes4 then0;0x39 uses9 then6. Variant is actual info.rank@44. Original DecoratorLibKey::new5B swaps input registers into result fields; use existing canonical `DecoratorLibKey::new(subtype,rank)`, not rank as subtype. Two actual collision BBX lookups occur before roof iteration. Missing first/second key panics with key context, not Unavailable. Both extract maxY-minY. First extent is multiplied by0.5 at f4b4 => half_height; second remains baseline_full_height for the later threshold.
2. ProbeXZ = worldXZ + directionXZ*0.2. Initial support threshold = worldY-half_height-0.4. Iterate actual Roof query order and take first qualifying other-wall flat roof (`Roof::is_flat`16B, not is_active/raycast).
3. Other roof supportY = roof.floor_y()-0.79. Require worldY>supportY (strict, NaN rejects) and supportY>=max_number(wall_elevation-0.4, initial_threshold) (inclusive, unordered rejects).
4. f6c2..f76d footprint uses the existing raw Roof enclosure at+8: odd tag is rectangle; even circle. Reuse utils Rectangle2d::contains_point_padding and Circle2d::signed_distance; decode only the existing Roof.observed fields, with no constructor's extra validation. Rectangle requires both currentXZ and ProbeXZ inside inclusive padding Vec2::splat(0.8). Circle requires current signedDistance<0.4 AND probe signedDistance<0.0, both strict;0.8 is not a circle radius. Do not call Roof::shape_at since that changes profile/eave geometry.
5. First qualifying roof produces tag0, wall=roof.wall_id(), height=max_number(wall_elevation,supportY)+half_height. Carry into step9. Upper-clearance-disabled early return does not access wall curves/heights.
6. If no roof qualifies, original mapped wall outline+90 must have at least2 points (0 asserts;1 unwrap-panics). Traverse projected XZ arc length from coordinate0 using maxss(wall_distance,0) (NaN and negative choose positive0). First interpolate outline Y as a+(b-a)*t and maxss against0. Preserve source segment and t for independently sampled upper heights later; no normalized-U rescaling, raw curve substitution or fabricated points.
7. `prefer_wall_outline=true`: select wall path only if sampled_outline_y > (worldY-half_height-0.1). `false`: first compute ray_base_y=worldY+baseline_full_height*(-0.5), then select wall path only if sampled_outline_y>ray_base_y. Strict greater; NaN takes ray branch. Wall path tag1/currentWall/height=max_number(half_height,sampled_outline_y+half_height+0.05).
8. Ray path: originXZ=worldXZ+directionXZ*0.5; originY = branch-selected baseY+0.5 (true baseY=worldY-half_height; false baseY=worldY-baseline_full_height*0.5). Direction exactly(-0.0,-1.0,-0.0), original max_toi1.0, mask0x1000. Accept only actual nearest simple-query source tag13; all other tags including miss17 =>Unavailable. Tag2/currentWall/height=max_number(half_height,actual_hit_position.y+half_height+0.05). No hit injection, infinite-ray substitute or source filtering after skipping a nearer in-mask hit.
9. fA47..fcea: require_upper_clearance=false returns selected output. If true, traverse the SAME actual mapped XZ outline again at maxss(wall_distance,0), interpolate current upper/arch-height vector+e0 at exact selected segment/t; array bounds failure stays panic. Iterate all actual Roofs; any flat roof on current wall sets margin=-0.79, otherwise0. Reject tag4 iff worldY+half_height >= interpolated_ceiling+margin; the compare is current input position, not newly selected output height. Equality rejects; unordered does not take the rejection branch. Otherwise return selected payload/mode. No source mutation or tracking cleanup.

## Publisher scalar ownership

Native f683/f77e read Public+280, independent of cached maxY+10c. The retained full original publisher and new snippets prove source-end stored at a8cada, end-70 is RawInnerWall source; a8d825 loads end-18=source+58, a8d904 stores scratchPublicBase500+280. This is the existing canonical `wall.authority.elevation()`/`source().terrain_floor` scalar. Use it verbatim, not minimum_y, constrained_extra, max_y, flat_roof_y, nor positive floor clamp. No publication-field extension is needed. Existing `wall.height_outline()` and `wall.arch_segment_heights()` remain the actual canonical geometry/height owners; absent upper cache must fail rather than substitute zeros.

## Bounded projected-distance helper, not a new curve subsystem

Original support CurveSlice specialization921B+callback10B is fully frozen. Current utils::CurveSlice owns Vec3 references, whereas current PublicWallState owns [[f32;3]]; it also intentionally uses f64 accumulation, skips zero-length segments and returns early for distance0. Native selected specialization computes f32 segment sqrt(dz²+dx²), f32 residual balance and `t=1-balance/segment_length`, including zero/NaN behavior. Materializing a Vec<Vec3>, unsafe array casts or silently using changed numeric semantics are unnecessary.

Propose a private bounded `snap_curve_coord(&[[f32;3]], distance)->(usize,f32)` for this sole Snap dependency: assert >=2 points; distance already maxss-clamped; start segment0,t0, balance=first_length-distance; when balance>=0 return(0,1-balance/length); otherwise mark passed segment endpoint(t1), add each next length to balance in f32; first balance>=0 returns corresponding segment and1-balance/length; exhausting returns last segment,t1. Preserve native unordered comparisons and endpoint tie behavior. This is the original game utility specialization required by the selected function, with no public curve API or geometry owner changes. Private interpolation reads existing arrays directly; cannot index an invented/prepared replacement.

## Necessary finite simple-ray API (independent world delta)

Current production world SHA15993d59d77ed971adbee9df8c274a0a30941ccf8cb7d3686140bfad21459a24 already includes accepted overlap + bounded intersects_any. Do not replay older candidates. Original Snap passes max1 in XMM3; world-raycast-simple39B explicitly transfers XMM3 to Rapier's stack bound, zeroes excluded Option; Rapier697B loads bound intoXMM6 at ce1cc, submits it at ce2dd and tightens it from every hit at ce31f. Current simple query hardcodesMAX, a real consumer gap.

Propose native-physics-only `RaycastWorld::raycast_simple_with_max_toi(origin,dir,max_toi,mask)->RaycastSimpleResult`, delegate to same existing Rapier helper after adding explicit initial bound parameter. Existing raycast_simple and raycast_simple_exclude_wall_id continue explicitMAX semantics; no exclusion semantics change here. Preserve17slot ascending walk, mask, solid=true, actual registry/source projection, nearest tightening, equal-distance later-slot replacement, actualposition andTOI. No new nonnative miss/false success, no sanitizing/clamp/direction scaling or extra flush. This source is root-owned and requires separate authorized candidate/review against current actual SHA before implementation; do not hide this dependency in the Snap stub.

## Acceptance obligations (root executes)

Static reviewer must independently compare full body/branches/args/constants, source owners, first-match order, key order, mode4 current-position comparison, no unneeded array reads on early return and bounded simple-query propagation. Tests should use real Bevy sameWorld Query<&Roof>/actual mesh records and real registered/flushed Rapier colliders; no fake outputs.

CPU Snap controls: unsupported dtype returns before missing deps; key(4,rank)/(0,rank) and9/6 both use actual bbox; missing keys panic; rectangle inclusive padding vs circle strict boundaries; samewall/nonflat skip and query-order first qualifying roof; support thresholds and both bool branches; current elevation differs from maxY; mapped distance differs from normalizedU/fullXYZ; zero/negative/NaN/over-end distance and zero projected segments; real source13 inside1unit vs beyond1 and nearer wrongtag; independent upper heights/flat-current-roof margin; exactly equal upper clearance rejects and candidate outputheight does not replace currentY; cache absent/mismatched ranges panic. Test old public MAX behavior as compatibility control. Real finite simple-ray tests must verify same canonical source24B,position,TOI; nearest tightening/equal-later-slot, masks/slot16, nonunit direction, solid inside0, miss outside bound. Keep prior overlap/intersects_any regressions. Source-only design/static review is not runtime acceptance or entire UI/validate integration.
