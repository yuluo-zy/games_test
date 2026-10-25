# Strict original f32 lengths for the actual Gothic profile Curve3 consumer

This package is evidence and design only. The single current utils::curve source owner is owner01 under TG-9cab24dd1d18f9a94a. No source change, Cargo, Rust test or original-target execution is included. This is separate from the already frozen nearest-query packages fc474fc0 / fe388d8f and must not expand their scope silently. Root explicitly requires preservation of the existing public f64 extreme-magnitude improvement.

## Actual consumer and original boundary

The actual half-profile callback1407e7dd0 calls Curve::try_new_from_points<Vec3,CurveCoordSemantic3D>@141269ce0 at1407e810c with reject_duplicates=false. Failure is unwrapped at1407e8113..8203; the second half is not computed after a first-half failure. This is not a diagnostic-only constructor. PDB1506:13576 identifies the complete702B function; all instructions and its continuous span are independently checked against original B RNE. The caller's complete1364B function is included as well.

Original constructor behavior:

1. Fewer than2 points returns TooFewPoints(count as u8), consuming/dropping the input Vec. Otherwise allocate the cumulative-U buffer of one f32 per point. There is no preliminary per-point finite scan.
2. Start previous=points[0], length=+0. For each current point, including the first, subtract previous-current component-wise in f32. Compute segment=sqrt(dz*dz + (dy*dy + dx*dx)), in that operation grouping. Add segment to cumulative length in f32, store it, then advance previous. No f64 fallback, scaling normalization or robust-length substitution is present in141269d90..dd0.
3. Ordered length==0 returns NonFiniteLength. NaN does not enter that zero branch. Normalize the cached U values sequentially by dividing each by final length. Reject the first value whose sign-cleared bits exceed0x7f7fffff. Infinity/NaN are thereby errors; no repaired cache or normalized-index fallback is returned.
4. Only after successful normalization, if reject_duplicates, check adjacent XYZ ordered equality in order and report the first segment index. Signed zeros compare equal; NaN is non-equal (normally already rejected by step3). Otherwise return the original points, real cached U, and final f32 length.

Current source uses CurvePoint::metric_length, whose Vec3 implementation converts to f64 for non-normal f32 squared lengths; it also scans point finiteness before constructing U. Those policies are intentional current shared improvements and are not to be globally removed for this game consumer.

## Concrete bounded counterexamples (static IEEE-f32 analysis)

- points=[Vec3::ZERO,Vec3(1e20,0,0)], reject_duplicates=false: original squared distance overflows, final length=+Inf and last U=Inf/Inf=NaN, so it returns Err. Current f64 fallback yields finite length1e20 and valid U=[0,1].
- points=[Vec3::ZERO,Vec3(1e-30,0,0)]: original squared distance underflows to0 and returns Err; current fallback can retain the finite nonzero distance and return Ok.

These are source/native-CFG predictions, not executed original or Rust results. No evidence currently bounds all real PublicWallState coordinates away from them. Therefore a full-domain equivalence claim for the half-profile cannot rely on the default current constructor.

## Proposed narrow same-constructor strategy

Owner01/root should choose the local method name, but expose a strict original-f32 Vec3/3D constructor entry on the same Curve type. It must delegate to a single existing parameter-construction loop with a private length/validation strategy. The ordinary public constructor and Vec3::metric_length retain their current f64 behavior. Do not introduce another Curve type, clone the constructor algorithm, change global metric_length, or add a registry.

The strategy selects both the strict segment arithmetic and the original validation sequence (no early finite-point scan, zero check then sequential normalized-U finite check), so merely supplying a length closure to the current prechecks is not a claim of exact failure order. Existing default validation stays unchanged. This small API is a separate authorization/review step; this document does not authorize owner01 to edit it as part of nearest-query work.

The restored Gothic half-profile should use that exact entry and unwrap; other unrelated Curve users keep their default policy. Required tests should call the real shared default and strict entry, prove the two extreme counterexamples remain intentionally distinct, check ordinary f32 cumulative values/XYZ grouping, repeated points/duplicate error index and nonfinite/short input. No tests are run in this package. Native allocator implementation and arbitrary custom CurvePoint types are outside this scope.
