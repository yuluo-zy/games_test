# Actual cloth consumer numerical/cache projection notes v1

These are additional bounded B observations, not a source implementation approval. C parity and whole-App reachability are unverified. The accepted input/output contract9315de88 remains unchanged. This note does not replace the frozen original evidence or its prior manifests.

## Corrections to old descriptions

The existing source's manual modified-flag anchors, adjacent-window loop and discarded quads do not implement this native consumer. The native first input is the integrated global VisualState.cloth_infos. There is no new persistent insertion in the inspected main; actual output registration is the same global nested transient map.

The earlier PROJECTION_EVIDENCE_V1.json bytes at142a891e0 are correct, but its role label “endpoint beam local offset” is imprecise. That constant is `(0,0,0.2)` used later in garment/thread attachment computation. Beam output separately removes the rotated `(0,0,0.385)` offset. Preserve both original byte ranges and their hashes; do not relabel one value as the other.

The existing clothline helper's claimed round-based segment count is wrong. Its call142923e60 is an import thunk to CRT **ceilf**. The garment-distribution call142923fa0 is a different thunk to **roundf**. Neither is a recovered dependency algorithm; use the corresponding locked Rust floating-point operation and preserve subsequent conversion/arithmetic order.

## First input projection and graph

Typed fold141293730/434B keeps quaternion, scale, wall, both float words and DecoratorId. It changes translation by `q*Vec3(0,0,0.385)` plus the second dimension's vertical shift `dimensions[1] * (-0.5) * 0.6`, in that order. In particular native Y is `(old_y + shift) + rotated_y`; do not collapse the two multiplications into -0.3. Native scalar flags/payload at44..4d are explicitly zero except tag4d=1 (8-byte store at44 and word0100 at4c). This is a bounded Cloth projection, not the declaration of the more general native CandidateInfo type or proof of other flag producers.

greedily_connect1407e1430/4312B sorts candidates stably by unsigned DecoratorId first (16fc and1ff3 path, witnessed comparator). It scans each source's later candidates only: initial target skip is i+1 (17ad..17e2,1939..196e); source tag4d must be nonzero and target tag must equal1. It reads wall at30, not the old source comment's0c. Distances use the original strict squared windows. The cloth caller supplies degree1, flag1,9.0/6.0/0.0; the old flag caller's degree2/10.0/5.5/2.0 defaults cannot be reused.

The flag argument permits same-wall connections; when enabled, only a same-wall neighbor weight receives +3.0 (18c8..18f6). Each source has its own neighbor list, sorted stably by partial_cmp(weight).unwrap(), not one globally sorted edge list. Keep insertion/tie order and original dependency sort semantics; no third-party sort reconstruction is required.

The second scan uses dense per-candidate **index** degrees and canonical index pairs, not a HashMap keyed by DecoratorId. Capacity is checked at both ends before testing the accepted-edge set. It rejects too-steep edges, requires distance>0.02, then tests a normalized finite ray with max_toi=distance-0.01 and mask30db. Only accepted edges are appended and inserted into the set (1db7..1e07), then both dense degrees increment. Never mark a pair used before collision rejection. Keep index identity when IDs repeat.

An engineering interface may return a stable sort permutation with original-input index pairs/degrees to avoid copying unknown CandidateInfo fields. It must preserve native traversal, tie order and duplicate-ID behavior, and clearly distinguish this projection from an original ABI declaration. This is a proposed adapter shape, not yet source code.

## Cache and event transitions

The actual Local<CacheClothes> is a persistent system-local state. Its first field is a Vec<DecoratorId> (4-byte stride): main compares the current projected candidates' IDs against prior IDs and emits a new-candidate event when absent (5530..5601). It then replaces that previous-ID Vec before running the graph (56b8..56ea). The payload position is the projected candidate position; the event's trailing word is0010, meaning the current canonical fields kind16/context0.

The local also contains two HashSet<(DecoratorId,DecoratorId)> values. At57bb..5811 the current set is cleared, preserving its owner; accepted graph edge IDs are normalized to unsigned(min,max) and inserted at596a..59a9. Membership in the previous set controls the new-connection event; absence emits kind16/context0 at the midpoint of the two projected candidate positions, each endpoint multiplied by0.5 before addition (5ada..5b34). At635d..6383 current set content and RandomState are cloned into previous set before the geometry-output loop. This is not reinitialization every frame. Reuse std/hashbrown dependency behavior, not a handwritten hash implementation or an empty cache placeholder.

The actual system's original Default initializes these containers empty; it does not default a ClothInfo or garment payload. Cache event ordering and disappearance/reappearance behavior must be tested with several real system invocations, including duplicate IDs.

## Connection plans

The original local plan Vec stores semantic `(DecoratorId,DecoratorId,Transform,Transform,u32)` values, native112-byte stride. Do not transmute its layout. For connected nodes it considers signed half-dimension endpoint combinations in order(--),(-+),(+-),(++). Same-wall plans select the farthest strictly greater distance from initial0; different-wall plans select the closest strictly smaller distance from initial10000. Ties retain the earlier selected pair. If none is selected, native retains the real center translations with their real quaternion/scale, not default Transforms. The constant-sign and multiplication-order projection remains an explicit final implementation gate; finite arithmetic equivalence alone is not a claim about nonfinite/signed-zero inputs.

Unconnected degree-zero candidates are visited in the sorted candidate order and produce their own +X then-X half-dimension endpoints (the negative branch has distinct -0.0 multiplication stores). Both plan IDs are the actual same DecoratorId. The final u32 includes the actual storage DecoratorInfo+2c value (currently canonical decorator_id), not the rank byte at44. Reads use the actual optional DecoratorStorage lookup; native missing lookup contributes zero in this specific scalar computation, which is not permission to create a default decorator, Transform or ClothInfo.

Each plan initializes the actual RNG state with init_rng(id_a.wrapping_mul(id_b)). The locked fastrand1.8 owner then uses with_seed on that already mixed state once. Shared utils::random_splits and xz_rotation_bad are now root-integrated and their exact-source7-test kernel harness passed; that result does not test this consumer.

## Garment values

From the actual attachment endpoint positions, let d be the original full3D distance. Main6ae3..6b65 computes roundf(d/0.7), Rust-style saturating f32-to-usize conversion, then the machine's wrapping `n+1+(n==0)` before calling shared random_splits with jitter0.2/d. Keep the overflow behavior explicit; do not insert a saturating point count or use an arbitrary maximum. Random points are normalized boundaries with exact0 and1 endpoints.

For each adjacent boundary pair, m=(u0+u1)*0.5; span=(u1-u0)*f32::from_bits(0x3f36db6e). Edge samples are m±span*0.5. Their relative vertical values use the original parabola with factor-0.08*d plus the linear endpoint-height difference; their average is subtracted from both and used in the garment translation. Preserve every f32 multiplication/addition order from6c76..6d41. The two resulting bit patterns and the plan's final u32 plus garment index become the actual12-byte ClothlineCloth value at6f16..28.

The horizontal normal passed to the shared xz_rotation_bad is Vec2(-delta_z,delta_x) divided by its horizontal length (6b93..6bdd,6dce..6dd9); do not insert an unproved local fallback before that existing helper. Translation uses actual XZ interpolation and average curve height.

Every garment consumes one native u32 RNG draw followed by two f32 draws. The first selects `_cloth_2` when `draw & 0xc0000000 == 0`, otherwise `_cloth`; the same condition selects NormalMap hashes0xe21f7890eabecf74 versus0xb9fbcd499788b2f1 (6d46..6da7). These hashes are opaque original name hashes until separately mapped, never descriptor indices. Mesh names and actual Cloth atlas kinds are in frozen ASSET_INPUTS_V1.json.

The two subsequent f32 draws produce the actual scale override, not a default Vec3. The shared width factor is abs(d*span)*2.5 (6cf0..6cf8). Native first stored override component is min((1-r2)*0.8+r2*1.1,0.8)*factor; second is min(((1-r1)*0.6+r1*1.2)*factor,1.0); third is1.0. The store order is builder+34,+38,+3c at6f04..6f0e. Do not interchange the pair or “simplify” the caps. Native main then calls real build<Transient> at7006 with the actual first plan DecoratorId; the scale1 constant is a proven write, not a substitute for missing input.

## Thread segment geometry

The same main calls original clothline2604B with exact mesh string, original endpoint Transforms, stretch=true, width1, sag0 and absent style. The helper uses ceilf(distance/spacing), saturating conversion, then minimum segment count2. Its iterator emits consecutive pairs through endpoint n, so there are n segments (2a0b..2aae,29ec..2a05), not a returned quad list. Caller spacing is exactly f32 bits0x3e99999a=0.3, read at7121 from1429258d8, now bound in NUMERIC_WITNESSES_V1.json. The sag ratio uses native maxss/minss ordering, retaining NaN; do not reuse the current manual conditional helper which turns that NaN into0.

For each sampled segment, use actual sample0 translation, forward=(sample1-sample0)/segment_length, right=Y.cross(forward).normalize(), and up=forward.cross(right).normalize(). The existing locked Quat::from_mat3 of these columns reproduces the dependency branch, including sign choices; do not handwrite atan2 or a replacement quaternion algorithm. Scale is(width,width,segment_length) for this true stretch caller. Native does not silently normalize_or an invented fallback. With style absent it uses nani_solid_vertex_color and real builder/transient registration (29c8..29e7); no ClothlineCloth/NormalMap is attached to the thread itself. The styled flag branch remains a separate unsupported recovery domain, not a successful None adaptation.

## Remaining bounded work before source candidate

Finish the exact signed-zero/nonfinite multiplication and thread sample floating-point expression-order projection; retain the now bounded four-endpoint order, actual storage-scalar addition, midpoint sound event and spacing. Then submit a complete numeric/cache implementation design to07. Actual generate_clothline and flag_connections candidates must replace the live manual-anchor/quad-discard path, emit nonempty actual Commands output into the same global transient owner and consume actual assets. Canonical NaniCloth binding is a separate design under the Nani claim; maintain must continue rejecting this kind until approved implementation supplies all canonical inputs. Real upstream Box<ClothInfo> window producers, whole-App registration and GPU execution remain separate facts.
