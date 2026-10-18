# B tree-wall callback: direct grid walk and exact distance

Read-only evidence for root's claimed hide-trees adapter, no production edit or Cargo. Bound B SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. Complete specialized callback 0x1425df320..f757 (1079B), with six real spatial/circle helper entries retained. Original MainQuery/Prefab identities remain root's source owner boundary; this report accepts only the callback leaf and grid filtering.

The generic current WallSpatialHash::query_with_circle sorts/deduplicates and uses <= plus a different closest-point operation. It cannot replace this specialized callback. Reusing current segments_in_cell and existing public grid conversion keeps the current authoritative segment owner while implementing the proven predicate in the existing tree-hide module.

## ABI and grid policy

RCX=&WallSpatialHash, RDX=&Circle2d{center Vec2,radius}, R8=closure pointers{&targetWallId,&treeVec3,&visibleBool}. On a hit, the callback writes false to visible, then immediately returns. Misses leave the supplied bool untouched. The upper hide system separately clears animation hidden and applies native visibility/update semantics.

The native spatial header supplies cell vector+08/len+10, dims u32x2+48, extent f32x2+50. Test fixtures use the established64x64 over130x130 world.

1. Compute circle AABB. Intersection lower is component max(world.min,query.min), upper=min(world.max,query.max), using glam's original explicit NaN mask: a NaN **query** coordinate selects the world bound. Assert resulting lower>=world.min and upper<=world.max. This is not a global rejection merely because query lies outside world.
2. Assert cell_size_x>0.5. Convert lower/upper with existing world_space_to_grid_coord_unclamped (libc floorf, saturating float->i32). Lower is clamped only upward to0, upper only downward to dims-1; an entirely outside query therefore has an empty range. Do not clamp both corners unconditionally to border cells.
3. Traverse row-major z outer, x inner, inclusive endpoints. No sorting, wall dedup or segment dedup. AABB clipping may visit cells whose circle-distance gate later fails.
4. Cell-radius factor executes two separate f32 multiplies: cell_size_x *0.5 (bits3f000000 @142925870), then *sqrt2(bits3fb504f3 @14292e6f4). Add circle.radius. Compute sqrt distance from native cell_center to query.center. Proceed only if radius_plus_cell_radius > distance; equality and NaN skip this bucket.
5. Iterate every segment in that cell's native order. Compare its WallId (+28 within56B Event/Bucket row, not+30 raw wall record) with target first; nonmatching rows perform no segment-distance calculation. Stop all loops at first distance hit.

Full original queries confirm four center0 cells, one corner cell at±65, no cells for center±100/radius.4, single cell for zero radius, empty negative-radius range, and NaN-axis AABB clipping spanning that axis. The latter does **not** hide anything because subsequent cell-center distance is NaN. These facts do not imply callers produce arbitrary circle radii; upper original tree query uses its own fixed radius.

## Exact segment arithmetic and threshold

A,B are2D segment endpoints; P is tree position XZ. Preserve separate f32 operations:

- q=P-A; d=B-A; dot=q.x*d.x+q.y*d.y; length2=d.x*d.x+d.y*d.y.
- MAXSS length2, 1e-20 (bits1e3ce508 @142a7baa0) returns length2 only when strictly greater; NaN/tie selects source1e-20.
- ratio=dot/denominator.
- MAXSS destination0, source ratio: `0 > ratio ? 0 : ratio`, preserves ratio NaN/-0.
- MINSS destination1, source previous: `1 < previous ? 1 : previous`, preserves source NaN/-0.
- nearest=(1-t)*A+t*B, **not A+(B-A)*t**.
- delta=P-nearest; distance=sqrt(delta.x²+delta.y²).
- Hide only if f32(.4) > distance (bits3ecccccd @142925970). Exact .4 is visible, one float below hides, one above visible. NaN/unordered does not hide.

Denominator floor changes tiny nonzero segments and is essential for degenerate segments. Existing generic util <= distance or a generic clamp with altered NaN/source semantics is not equivalent. No distance square comparison, fused operation or reordered weighted sum is accepted by this evidence.

## Executed oracle

native_tree_wall_probe.py executes full original callback and actual spatial helpers. Only imported libc floorf is bridged, plus shared allocator/memory hooks which these cases do not reach. Observation hooks log cell coordinates/accepted buckets/segment checks and do not change business decisions. A separate narrow original arithmetic invocation stops before the threshold branch to retain t/distance bits.

Passed84 invocations:10 narrow edge geometries +10 corresponding complete callback queries,3 ID/order/early-return queries,10 empty-grid/clipping queries and51 additional narrow finite/tiny/signed-zero arithmetic cases. Fifty-one finite outputs match an independently written f32-rounded arithmetic model exactly; edge cases include NaN/Inf and compare branch meaning without requiring cross-CPU arithmetic NaN payload identity. Original wallID rejection of a poisoned NaN segment occurs before arithmetic, and a first matching hit prevents examining the following rows.

This does not execute full ECS hidden-tick publication, height/area events, Prefab SSBO updates, GPU draw or the full game. Root's current-module adapter/tests must close those consumers separately. Native segment bucket representation is test harness setup only; the Rust candidate should retain current WallSpatialHash/Prefab authorities.
