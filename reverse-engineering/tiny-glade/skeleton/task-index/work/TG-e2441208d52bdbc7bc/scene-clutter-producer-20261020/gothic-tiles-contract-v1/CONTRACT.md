# Actual Gothic Dormer tile consumer: two curves to the canonical64B sparse record

This is a bounded B-native business design for the existing TG-e2441208d52bdbc7bc source. It does not change source/Cargo, run tests, execute the original, register a system or claim a nonempty scene. Complete5713B141290b30 and its real caller14207B1407cb1a0 are retained and directly rechecked; required projection helpers and736B actual push are retained too. Current old Vec<GothicDormerTile{2xf16}> is not the original output and is not reusable as tile geometry. C reuse is unproven.

## Owner and complete calling boundary

At1407cd000..01c the generator passes RCX=&[Curve3;2], RDX=&actual Roof, R8=same UI stream (unused inside this helper), R9=&actual CachedDecoratorTransforms, stack20=current DecoratorId, stack28=emit closure borrowing the current SparseInstanceBufferWriter. The original helper does not return a Vec of coordinates; it calls emit with a complete64B record at1412911c3 and141291cbc. The actual closure14128d0e0 copies that record to the same canonical RoofTileSsbo allocation, and the original callers ignore its NoFreeChunks return. No cancellation, stop, warning, retry, replacement buffer or RNG rollback may be added for that condition.

Proposed semantic interface keeps those real inputs and an emitter borrowing each canonical RoofTileSsbo:

    assemble_gothic_dormer_tiles(profiles: &[Curve3;2], roof: &Roof,
        ui: &UiSignifierStream, cached: &CachedDecoratorTransforms,
        decorator_id: DecoratorId, emit: &mut impl FnMut(&RoofTileSsbo))

The exact current cache method accepts rawu32; pass decorator_id.0 at that existing API boundary, not a new cache or default transform. Original UI argument is not used by this machine-code instance. The enclosing real system still has its actual required UI resource; omitting reads here does not authorize creating a fake UI owner.

Current generate_dormer_roofs is an unregistered placeholder with one call using the old wrong helper signature. Therefore a changed helper cannot be integrated alone without a coherent real assembler companion. Do not keep a fake old overload returning empty coordinates or silently make the old caller succeed. Root integrates coherent reviewed sources only; actual Cottage tessellation/cache/generator remains a separate source gate.

## Initial projection, failure and single RNG owner

1. Require first profile.points.first(), otherwise original panic. Do not prevalidate the second curve or all subsequent samples.
2. Let P be the first point of profile0. Query actual Roof::roof_shape_at_intersection_y(P.y). OutOfBounds returns immediately before cache lookup, RNG creation or emission. For a true circle/rectangle section, find the exact closest perimeter point to P.xz and lift it with P.y to Q. RawSection at y==floor must decode the same actual field values: circle center/radius/rotation or rectangle axis/center/dimensions; do not synthesize zero orientation or substitute new bounds.
3. Fetch cached.transform(decorator_id). Missing transform remains required failure. Seed one pinned fastrand1.8 Rng from saturating-u64 of sqrt(z10²+(y10²+x10²)), where translation components were first multiplied by10. Keep native grouping/conversion, including NaN→0/positive overflow→u64MAX. No hash seed, extra draw, new RNG per half or hand-written Wyrand is added.
4. L=sqrt((P.z-Q.z)²+((P.y-Q.y)²+(P.x-Q.x)²)). Keep this original L, before extension. Set extended P component-wise to P+(P-Q)*.05. No zero-distance early return: the original still forms raw normalizations and writes according to the following steps.

Dependencies must retain original semantics. The84B circle perimeter helper does raw d*(1/sqrt(dy²+dx²))*radius+center. Current public Circle2d::closest_point_on_perimeter uses normalize_or(X), changing center and overflow/underflow cases. A strict same-canonical geometry entry/strategy is required before source acceptance; preserve the improved default shared API and do not create another Circle/geometry owner. The original section752B and rectangle475B are included for independent reuse audit; their names alone do not prove current API equivalence. In particular original section computes cap before testing y==floor, while current helper copies RawSection first; invalid cap/floor failure-domain compatibility remains a shared dependency gate until native valid Roof domain or a narrow strict entry is established. This design does not silently repair a section into valid geometry.

## Stage1: ridge records, before body splits

Call shared original random_splits with count=max2(roundf(L/.46875) as usize), jitter=.09375/L and the one RNG. The original thunk142923fa0 imports roundf; using ceil here changes counts. The stored routine returns normalized boundary points including0/1 and consumes only internal-point draws. Standard shared implementation is reused, not rewritten.

Let d=(extendedP-Q) raw-normalized with f32 length, r=(d cross Y) raw-normalized; ridge quaternion is pinned glam's from_mat3(cols(r,d,Y)), preserving its original branch/sign choice. No atan2, from_rotation_arc, normalize_or_zero or final quaternion normalization substitutes for it. The finite geometric basis and native two-branch simplification are consistent; native NaN/degenerate calculations must not be changed into a fallback basis.

For each consecutive pair(a,b) of these boundaries in order:

- midpoint=(a+b)*.5;
- position=Q*midpoint+extendedP*(1-midpoint), preserving this grouping;
- dimensions=(.1,(b-a)*L*1.2,.2);
- half-pack quaternion and dimensions, then draw rng.i32(-100..100), then emit the complete64B record.

All ridge records are emitted before allocating body splits. Emission order/RNG remain unchanged if the sparse push fails. Do not replace with uniform rows or skip the first/last span.

## Stage2: two profile bodies and their loop order

After stage1 drops its split Vec, make the shared longitudinal boundaries with count=max2(ceilf(L/.3) as usize), jitter=.03/L. For profile0 then profile1:

- create its profile boundaries with count=max2(ceilf(profile.length/f32bits3ea3d70b) as usize), jitter=.032/profile.length. The divisor is the exact .320000022649765 literal, not rounded .32 bits. This allocation/draw occurs once for each half, after previous emissions.
- choose side=cached.rotation*Vec3(0,0,+1) for half0 and cached.rotation*Vec3(-0,-0,-1) for half1. The signed zeros are observed in the native sign-mask preparation. Reuse pinned glam quaternion-vector multiplication, not yaw-only or world-Z logic.
- loop consecutive longitudinal pairs outermost, profile-boundary pairs innermost. For a longitudinal pair(s,t), width=(t-s)*L and offset=(-(s+t)*.5*L)*(cached.rotation*Z), retaining the original scalar grouping as ((1-mid)*0-mid*L). This offset is common to both halves, not multiplied by side's sign.
- For profile pair(u,v), require each finite and in[0,1] in original first-u then second-v order. Use the actual current Curve3 cached U/selected points. Sample A=profile(u), B=profile(v) as b*t+a*(1-t). Preserve bounds/zero-knot failures; do not resample from approximate curve indices or recalculate cache.
- D=(B-A) raw-normalized with1/sqrt(dz²+(dy²+dx²)); C=D cross side, with no additional C normalization. Base orientation is from_mat3(cols(side,C,D)); native1412917b5 tests D.z as m22 and the following m11−m00/m11+m00 branches identify these columns. Do not swap C/D merely to make dimensions visually intuitive.
- Draw r0=rng.f32(), then r1=rng.f32(). angle_half=(((1-r0)+(1-r0)+r0*5)*f32bits3c8efa35)*.5. Build local-X quaternion(sin(angle_half),0*sin(angle_half),0*sin(angle_half),cos(angle_half)); call order is sinf then cosf. Multiply base*randomX, then multiply that result by the exact constant quat(.707106769,0,0,.707106769), whose nonzero bits are3f3504f3. Do not collapse the two multiplications or use a single angle to change rounding/RNG behavior.
- overlap=(1-r1)*.1+r1*.2. Dimensions=(width,length(B-A)+overlap,f32bits3da3d70b). Note the exact thickness is .08000000566244125, not the adjacent .08 literal.
- Position=(A*.5+B*.5)+(D*overlap)*.5+(C*negative_thickness)*.5+offset. The original operations add midpoint, then extension, then normal offset, then longitudinal offset; preserve each component's grouping rather than one f64/vector fused expression.
- Half-pack orientation/dimensions, draw rng.i32(-100..100), emit. No extra random sign/angle/draw is present. This third draw happens even when the eventual push fails.

The constructor strict-f32 policy in TG73/profile-curve-constructor-contract-v1 is required for original profile caches. The current default Curve3 extreme-length improvement must remain available; merely returning default curves or accepting all invalid caches would hide a real dependency. Normal, successfully constructed strict caches have finite monotonic U, so current coordinate query reuse is limited to that actual domain rather than arbitrary public field mutation.

## Canonical64B output for both stages

Use the existing canonical country_core::resources::roof_tile_ssbo::RoofTileSsbo, never a mirror or a two-half coordinate record:

- base_position from the stage above; current_height=+0;
- clipping_plane exact words[0x3c000000,0x7bff0000] (the observed half4 constant);
- scale_t packs three dimension components via the existing Halver/half dependency, zero fourth u16;
- rotation_quat packs the four quaternion components;
- roof_id is the low32 bits read directly from the actual Roof WallId, with no float conversion or decorator-id substitution;
- velocity_rotation=0, animation_t=1, seed is the stage's actual random i32, is_circle=0, is_main_roof_body=0.

Reuse existing IEEE half packing and pinned glam/fastrand dependencies. Cross-platform math-call/NaN-payload bit identity is not certified by source-only comparison; original DLL import bindings and all math constant bytes are retained, not silently treated as an executed differential test. Source assembly/correct owner and actual output are separate from GPU flush/present acceptance.

## Meaningful verification and remaining chain

Later tests should call this exact function with real cached transforms and real successful Curve3 inputs, collect complete records, and assert stage/half/row/profile order, nonempty ordinary geometry, RNG-state equality to the same pinned stream including deliberately rejected pushes, round-vs-ceil count cases, half/thickness/constant fields, required-cache and invalid-sample failure order, OutOfBounds-before-cache behavior and degenerate raw normalization semantics. Do not only test packing helpers or use constant geometry.

Current prerequisites remain strict circle/section reuse bounds, Gothic hat/profile companion, actual assembler/cache/Cottage algorithm and their registration. Ownership/delete and uploader are already separate real canonical source integrations; they do not prove this geometry has been produced. Required sameWorld+actualRoofTile storage allocation/retirement and GPU upload checks remain root-run gates. No test has been executed here, and no source candidate is yet frozen.
