# Borrowed nearest-curve seam for the Dormer geometry consumer

Scope: a necessary safe, allocation-free adapter to the existing canonical curve algorithms. This does not introduce a curve owner, change the stored wall geometry, or restore the entire Gothic consumer by itself. The only proposed source writer for utils::curve/mod.rs is owner01 under TG-9cab24dd1d18f9a94a; this TG73 package contains evidence and API design only. Existing frozen owner01 candidates remain unchanged. Root applies any later independently reviewed source delta.

## Actual caller and current boundary

The complete original get_gothic_hat_bones@1407eb500/1182B is called by the actual Dormer generator at1407cca74. It calls get_approx_u_from_pos@14126a040 at1407eb715,780,7e5, clamps the returned U and obtains its segment via get_coord_at_u@141269710. It then consumes the original wall curve's selected point pair. This is real geometry, not an optional diagnostic query.

Current PublicWallState.simplified_curve_2d() borrows the authoritative core::roofs::geometry::surface::Curve2, storing points:Vec<[f32;2]>, points_u:Vec<f32>, length:f32. The older GothicHatWallView instead requires an owned utils::curve::Curve2 whose points are Vec<glam::Vec2>. Copying the curve into a second owned type on each query, casting Vec headers, recomputing its cached U/length, or implementing another nearest-curve algorithm in the business source is unnecessary and not accepted by this design. The proposed adapter borrows both actual slices and projects one point to Vec2 by value.

This seam proves only nearest coordinate / approximate U. Other shape, WallSpace and sampling inputs must use their own native contracts; a similarly named method in the core curve is not automatically original-equivalent. Owner01 is already restoring the canonical WallSpace contract. The rest of the Gothic consumer remains a separate design/implementation gate.

## Original behavior to share

The two complete B functions are195B and572B, not the different747B inline instance. The195B caller directly calls the572B implementation at14126a053. Every instruction and both continuous spans were checked against RNE f507a6ce with matching PDB2d9686b8. PDB records1506:17048 and1506:22120 bind these exact generic Vec2 functions. Full original functions, selected metadata and constants are retained in EVIDENCE.json and curve-borrowed-seam-pack-v1.

Nearest coordinate:

1. Enumerate all consecutive point pairs in stored order, without allocation. Default result is segment0,t0, including empty/single-point input to this coordinate function. The separate U function rejects fewer than two points.
2. Initial best distance is f32::MAX, exact bits7f7fffff at142925980. No improvement on an equal or unordered distance, so ties retain the first segment.
3. For a,b and query p, let delta=b-a. Compute dot(p-a,delta) and dot(delta,delta) in native f32 order, using the two XY lanes. Denominator is MAXSS(d2,epsilon), with epsilon bits322bcc77 (1e-8) at14295a0d8. Its NaN handling follows those operand roles.
4. Divide to obtain t, then MAXSS(0,t), MINSS(1,t). This sequence propagates a NaN t. It does not use the current global clamp01 helper, which maps NaN to0.
5. Project b*t+a*(1-t), compute Euclidean distance from p, and update only when distance<best. Immediately before a successful update the original also asserts0<=t<=1; retain the failure contract rather than silently accepting an invalid coordinate. An unordered distance does not enter that update.

Approximate U:

1. Require points.len()>1 before calling nearest coordinate (14126a049..04e).
2. Use the returned segment's two real cached points_u elements. Bounds failures panic; no fallback to normalized index, recalculated distance or zero U is allowed.
3. Compute (1-t)*u[segment]+t*u[segment+1] (14126a078..093). Explicitly reject NaN and infinity before returning (14126a097..0aa). The current ownership/cache contract otherwise remains unchanged.

## Existing source counterexample

Current owned get_approx_coord_from_pos calls clamp01, whose ordered comparisons turn NaN into0. A concrete static IEEE-f32 example is points=[(-1e20,0),(0,0),(1e22,0)] and p=(1e18,0). In both original segments, numerator and denominator overflow to infinity and their division is NaN; each projected distance is then NaN and skipped, leaving default(segment0,t0). In current source the second t becomes0, its projected point is(0,0), and distance1e18 is finite, so it selects(segment1,t0). This is a bounded source/CFG inference, not an executed original-target or runtime differential result.

Therefore the seam must restore this specific nearest-function clamp sequence while factoring the shared implementation. A mechanical extraction of the current owned function cannot be described as original-equivalent. Do not change global clamp01, since its unrelated consumers are outside this scoped counterexample. Preserve ordinary finite behavior, tie order, cached-U indexing and failure order.

## Proposed public interface

Names may follow owner01's local style, but the typed contract is:

    pub fn approx_curve_coord<P>(
        points: &[P], point: glam::Vec2,
        project: impl Fn(&P) -> glam::Vec2,
    ) -> CurveInternalCoord;

    pub fn approx_curve_u<P>(
        points: &[P], points_u: &[f32], point: glam::Vec2,
        project: impl Fn(&P) -> glam::Vec2,
    ) -> f32;

One implementation owns each algorithm. The existing generic Curve<T,S> methods delegate to these same functions using CurvePoint::projected. The Gothic borrowed adapter delegates using Vec2::from_array(*point), with the authoritative current wall slices. No new Vec, cache, resource, registry, guessed curve length or unsafe cast is needed. Coordinate and U result types remain the existing CurveInternalCoord and f32; no fallible default-success adapter is added.

## Meaningful required checks, not executed here

- Run the actual shared function and existing owned method on the overflow counterexample; both must produce the original default coordinate. Bind a cached U table that distinguishes segment0 from1 to check the actual approximate-U result too.
- Compare the same owned and borrowed functions for an ordinary interior projection, a repeated zero-length segment, equal-distance ties, and endpoint cases; retain cached points_u rather than reconstructing them.
- Check native failure boundaries: U requires at least two points, selected cached-U indices must exist, and NaN/infinite final U must fail. Coordinate-only empty/single input retains its original default.
- Keep the current owned point types/callers and owner01 raw WallSpace delta coherent. No new dependency or Cargo change is required. Actual gethat→half profile→Dormer tessellation and App integration remain separate gates.
