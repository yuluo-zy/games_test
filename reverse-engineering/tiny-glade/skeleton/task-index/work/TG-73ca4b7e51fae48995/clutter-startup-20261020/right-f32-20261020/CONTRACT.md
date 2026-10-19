# Original f32 right extension, bounded current API repair

Authorization: root normally claimed TG-73ca4b7e51fae48995 for clutter-startup-20261020 and delegated only the existing right method. Current source before SHA256 is b3a32495557b92a2de06fb19b64e28fa5dce97f8cf2aba2ae5c94c7f19636ff1. This work writes candidates here; central alone integrates production. No left extension, measurement, constructors, other curve algorithms, public API addition or test deletion is authorized by this packet.

## Original identity and actual consumer

B f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 with matching PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911, GUID09F1A02A-49F2-F8C8-4C4C-44205044422E age1. Complete 1412898e0..141289c79, 921B, PDB module1538 records660/3628 share the body (Semantic3D and SemanticThresholded3D source Vec3 remapped to Vec2). The retained report has `function_boundary_verified:false`; length and procedure identity come from the independently retained matching PDB, not that report flag. External pack fingerprints are explicit in native-references.json. C/B source checksum navigation matches the file family; this packet makes only the demonstrated B function behavior claim, not a complete C equivalence claim.

Actual original location14080f470 calls this exact helper at14080f7f8,14080f88f,14080f92d for currentU, currentU-half collision width, currentU+half collision width on the actual prepared XYZ wall path remapped to XZ. It borrows points; there is no point-vector allocation. Actual current lexical callers and source identities are in curve-current-callers.json: same-file extend_right wrapper and Curve::get_coord_at_2d_ws_distance, then get_coord_at_wall_space. New location recovery will reuse this existing API. Inventory is lexical, not a dynamic reachability proof.

## Exact right method

Keep `extend_right_until(&mut self,distance:f32,stop:impl FnMut(&usize)->bool)` and `extend_right` wrapper. Keep `distance>=0.0` assertion: negative and NaN panic, positive infinity is accepted. Do not early return for zero. Keep the existing borrowed index range, take_until stopping-vertex inclusion, remapping and tuple-window iterator; do not add extra predicate invocations or eagerly sample the whole path.

Get the first actual pair; if absent leave the endpoint unchanged. Compute `length=(b-a).metric_squared().sqrt()` with f32 subtraction, squared terms, sum and sqrt, explicitly avoiding metric_length/metric_distance_f64 robust fallbacks. Original concrete remapped Vec2 sums dy²+dx² (the concrete finite two-term sum is commutative); the generic current API keeps its existing CurvePoint metric_squared extension contract for other custom point types. No claim of independently proved arbitrary custom or non-remapped 3D machine-code instantiations is made.

Compute `residual=((1.0-old_t)*length)-distance` as those separate f32 operations. If ordered residual>=0, write `(segment,1.0-residual/length)` and return. Otherwise immediately write `(segment,1.0)`, then for each subsequent actual pair add its f32 length to residual. On the first ordered residual>=0 write `(segment,1.0-residual/length)` and return; otherwise keep writing completed endpoints. On exhaustion retain the last completed endpoint. A NaN residual fails the ordered test and proceeds/exhausts. No clamp, zero-length skip, finite guard, saturating fallback or compensated subtraction is original here.

Zero-length first edge at distance0 writes NaN t (0/0); positive distance over underflowed squared length moves to the endpoint; overflowed length can produce inf/inf and NaN t. These are observable original arithmetic outcomes. Original constructor checks do not imply that an extension output remains finite. This packet does not repair subsequent callers by fabricating a valid coordinate.

## Current numerical enhancement conflicts, retained explicitly

`compare_f32_lift.py` was actually executed with the system Python and standard-library struct rounding at each f32 operation. numerical-observations.json records exact input/output bits. This is a bounded scalar source/assembly lift, **not execution of Rust, the original binary, a GPU test, or a stopping-predicate test**. No new dependency was installed.

The current fixture source contains260 right calls. The lift includes the actual LoopCurvePoints logical -1..period mapping. All260 original lift results match saved expected coordinates/panics exactly; old source lift also passes all260 under the fixture's existing 8*EPSILON absolute tolerance. There are26 bit differences between old and original lifts (including zero-distance one-ULP drift), so “260 pass” would not imply arithmetic equivalence of the old method. The fixture file has historical comments/modifications for left behavior; this packet binds its present bytes, does not claim an immutable pristine oracle, and does not alter it.

Existing `curve_properties::slice_lengths_and_extensions_handle_tiny_and_extreme_segments` asserts enhanced f64 behavior beyond original f32 domain. On its actual test inputs, the executed lifts yield:

| Input | Old source/right expected | Original right result |
|---|---|---|
| length1e-30, distance=length*.5 | segment0,t=.5 | segment0,t=1 |
| f32::MIN_POSITIVE, half distance | segment0,t=.5 | segment0,t=1 |
| length1e20, half distance | segment0,t=.5 | segment0,t=NaN |
| lengthf32::MAX, half distance | segment0,t=.5 | segment0,t=NaN |
| endpoints -MAX,+MAX, distanceMAX | segment0,t=.5 | segment0,t=NaN |
| [0,(1e-30,0),(1e-30,1)], distance5e-31 | segment0,t=.5 | segment1,t=0 |

Its right-endpoint and right-derived measurement assertions will therefore fail after original restoration; left and direct curve length assertions remain outside this change. `curve_review::zero_length_slice_edges_do_not_introduce_nan_coordinates` also expects right0 to preserve the initial coordinate on the repeated-zero edge; original writes NaN, so that old assertion is incompatible. These tests remain present and unchanged. No new opt-in enhanced API is proposed. Central must make a separately explicit test-owner decision; this packet will not delete, ignore or quietly weaken conflicting assertions. Ordinary finite nonzero segments still have rounding differences from f64 cancellation removal; only fixture comparisons and bounded selected inputs are evidenced here, not a universal equivalence theorem.

## Candidate and verification boundary

After independent design approval, replace only the existing right method body; add a small same-file independent cfg(test) module to assert original zero-edge, finite zero-distance one-ULP drift, underflow/overflow and inclusive stop behavior. Tests must be top-level module functions, not impl-associated #[test]. Keep existing fixtures and external enhanced tests unchanged. Preserve source outside the method and added test module byte-for-byte. No Cargo execution is authorized to this worker; central can run the bounded native fixture and focused tests, recording the known incompatible external assertions separately. No whole-App or visual claim follows from this numerical leaf.
