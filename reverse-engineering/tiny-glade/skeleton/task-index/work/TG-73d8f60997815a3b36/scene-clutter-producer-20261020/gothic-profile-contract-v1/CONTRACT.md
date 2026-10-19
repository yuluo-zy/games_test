# Gothic hat and half-profile business chain, with canonical borrowed wall input

This is a B-original evidence and implementation design package for the necessary helpers in TG-73d8f60997815a3b36. No source, Cargo, Rust tests, original execution or production files are changed. Root remains the production writer. Current sources have known hand-written placeholders; their doc labels are not original evidence. C checksum differences remain unknown and no C equivalence is claimed.

## Actual inputs, output and consumers

The complete Dormer roof generator1407cb1a0 calls get_gothic_hat_bones1407eb500 at1407cca74, then passes those returned values plus its owned profile into calc_gothic_half_dormer_profile_ws1407eb160 at1407ccb83. It consumes both returned Curve3 values in real tile geometry. Complete function bodies/PE checks and exact caller transfer are retained. The BP48B hole list is a separate output branch and is not this helper's input.

Proposed public semantic signatures in the existing Gothic source are:

    get_gothic_hat_bones(wall: &PublicWallState, info: &DecoratorInfo,
        u: f32, y: f32, half_base: f32) -> GothicHatBones
    calc_gothic_half_dormer_profile_ws(rank: u8, subtype: DecoratorSubtype,
        profile: Vec<Vec2>, bones: &GothicHatBones,
        wall: &PublicWallState) -> [Curve3; 2]

The same existing GothicHatBones fields are side:[f32;4], first:Vec3, center:Vec3, second:Vec3, edge:[f32;2]. This is a semantic value, not a native repr/size guarantee. Existing fake wall view/params are unnecessary; the current whole-crate text references of their names and both helpers are only in this file. The generator itself remains unimplemented and will not be registered by this helper batch. Required ordinary crate typechecking is not claimed.

## One authoritative curve, borrowed without allocation

PublicWallState.simplified_curve_2d() borrows the actual authority source's core Curve2 with points Vec<[f32;2]>, cached points_u and length. A private CurveSampling adapter in this business source borrows that Curve2. It must not clone or recalculate either slice, cast Vec headers, create a parallel curve owner, or cache replacement data.

- length and point_u read the actual source cache.
- coordinate sampling uses that core Curve2::coord_at_u after comparing it to the complete original694B141269710. Both finite-check before endpoints, use right-biased binary search, assert selected cached indices and unequal endpoints, inverse-lerp and native MAXSS(0,t)/MINSS(1,t). The native uses a u32 segment; arbitrary host arrays exceeding its representable count are not asserted equivalent.
- position reads the selected actual pair and computes b*t+a*(1-t), component-wise f32. raw_tangent_at_u reads the actual pair difference; no normalization or fallback is substituted. tangent uses raw difference times1/sqrt(dy²+dx²).
- approximate_u delegates to the single shared borrowed seam designed in fc474fc0 and family addendumfe388d8f, written only by owner01. That companion corrects nearest's NaN-clamp/f32::MAX policy. The current owned implementation alone is not accepted as equivalent.
- WallSpace used for project_clamp is the semantic {is_loop:wall.shape().tag()!=2,length:wall.wall_space_length()}. The complete21B140b2a740 establishes both fields. The current Option<f32> facade must not be unwrapped or guessed into that struct. For world conversion the actual curve's own length is distinct and preserved; do not substitute wall_space_length.
- Call the existing independently recovered WallSpaceCoord::to_world_space, including the raw selected endpoint tangent override. Its current source owner is01; no duplicate world-conversion algorithm is introduced here.
- PublicWallState::is_circle is a true Some only for shape tag0, with the four shape floats. The complete36B140b2a7d0 matches the current semantic read. Circle radius is the third shape float (original wall+264), not rank, wall length or a default radius.

## gethat complete branch and operation order

Logical arguments are out, wall, info, u, y, half_base. Rank is info.rank at original+44. Keep the branch-specific failure order; do not eagerly compute center or all curve segments.

1. If wall shape tag is nonzero: rank1 keeps half_base unchanged (1407eb586 directly→643); rank2 adds-.05; rank3 adds-.1; other ranks panic. Current source incorrectly groups rank1 with rank2. If shape tag is0, first compute inverse_lerp(.95,1.6,circle.radius), even for rank1; then rank1 adds-.05. For ranks2/3 clamp that result via native MAXSS(0,t),MINSS(1,t), and subtract respectively ((1-c)*.21 + 0*c)+.1 or ((1-c)*.158 + 0*c)+.15. Keep the zero-product and original f32 grouping; invalid rank panics at its actual branch.
2. Multiply adjusted half by.5. Compute first projected U=project_clamp(u-half), then second projected U=project_clamp(u+half), before either world conversion. Convert first, convert second.
3. For first: actual approximate U→native clamp→actual coord→bounds-check segment+1→read pair. Do the same for second. Only then convert the unprojected center(u,y) to world space, and approximate/clamp/select its pair. This order preserves invalid-cache/world conversion failure placement.
4. For each selected delta(dx,dy), inverse length is1/sqrt(dy*dy+dx*dx). Stored side=[-dy_first*inv_first,dx_first*inv_first,-dy_center*inv_center,dx_center*inv_center]; edge=[-dy_second*inv_second,dx_second*inv_second]. Both side normals are complete independent normals. SIMD_SIDE_TRACE.json documents the actual MOVSHDUP/SHUFPS chain. Current source's purported -dy_first*inv_center lane is an old symbolic-register mistake, not original behavior.
5. first/second output Y is input y via xvy, while center preserves the actual world-conversion result. No normalized_or_zero, robust/f64 length, guessed normal, or early geometry fallback is inserted.

A bent real point sequence[(0,0),(1,0),(1,1)], open shape tag2, length2, rank1, u1.5, half_base2 selects first delta(1,0) and center delta(0,1): original side[2]=-1, current cross-mixed expression gives0. This is a static selected-segment example, not an executed fixture. It is distinct from the noncircle rank1 width error.

## half-profile point transform (625B1412ac9b0)

The complete923B wrapper builds output half0 then half1 and returns exactly two Curve3 values, consuming/dropping the input profile. It does not fetch a default profile internally. Caller profile/rank/subtype and complete hat values are real inputs. Subtype's current semantic wrapper is DecoratorSubtype(u8); the old name shape_kind must not make it a WallShape tag.

For every lazily requested profile point(x,h): call the actual wall.is_circle. Radius=.38 except Some(circle) AND subtype==7, when require rank1..3 and use[.25,.3,.29][rank-1]. Do not silently default invalid rank in that active branch. Then require profile.last(), and alpha=inverse_lerp(0,last.x,x), unclamped. Original checks last lazily inside mapping, not as an eager success/default guard for empty input.

Half0 interpolates center→second and center side normal→edge; half1 interpolates center→first and center side normal→first side normal. Compute B*alpha+A*(1-alpha) component-wise. Add h to its Y, interpolate the two normal components with the same grouping, normalize with1/sqrt(ny²+nx²), lift to(x,0,z), and add radius*normal. Do not use robust normalization, zero fallback, x-only geometry, or multiply h by wall.max_y. Zero/interpolated-opposite normals retain NaN behavior.

## Clip/curve construction (1364B1407e7dd0)

Transform points lazily in the original iteration order. Form consecutive pairs. A first pair must exist before pushing the first point; then push that first point unconditionally, even if below max_y. For each pair(prev,next), call wall.max_y and:

- if max_y>next.y is false (including unordered), push next and continue;
- if true, call max_y again, compute t=(max_y-prev.y)/(next.y-prev.y), append prev+(next-prev)*t for all three components, and break. Do not force its Y to the scalar maximum, filter the whole curve, continue after the first clip, or evaluate unconsumed future profile points.

Original max_y failure happens in the pair loop after first-point push; do not query height before mapping/pair availability. Empty/one-point input produces an empty clipped list and then constructor failure. There is no empty-success curve. Build with reject_duplicates=false and unwrap, append the completed first curve, then process the second. A first error prevents all second-half effects.

The actual current Curve3 constructor has a separate strict-f32 policy gap (profile-curve-constructor-contract-v1), and must receive a narrow same-constructor policy entry by its source owner before claiming full-domain equivalence. Preserve the default public f64 extreme support. This helper candidate can be written only with an explicit unresolved dependency, never represented as runnable complete by a fake method/body.

## Reuse and scope gates

Direct PE reads also verify current gothic_dormer_profile_curve's6/7/9 point literal tables bit-for-bit against original393B1407c7500 including final(.35,0)/(.588,0)/(.76,0). Reuse that table; no new profile owner or edit to its generator is needed. The supplied half-profile curve has its own producer, not assumed equivalent because its name sounds related.

Before candidate: independent review of this design, shared nearest API, and strict-constructor companion scope. Before App hookup: real cache/dormer generator, full Gothic/Cottage tessellators, seven BP record producers, actual registration and ready SSBO owner. This helper does not permit registering the current empty geometry/legacy generators. Keep original push-exhaustion behavior in the later tile consumer; it is outside this helper.

Required later tests must exercise the real canonical borrowed wall, noncircle rank1/2/3 and circle-rank radius branches, bent normals, endpoint/invalid cache failures, same actual sample ownership, point transforms for both halves, first-point behavior, exact first crossing followed by an unconsumed poison point, height-not-ready placement, empty/one profile failure, and first-half failure preventing second construction. No test/default fake wall or output can count as an original target or sameWorld App acceptance.
