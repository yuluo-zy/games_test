# Complete original calculate boundary and migration contract

This is a read-only contract and dependency audit for the owned TG-60234a3eb4a6f6e4d2 file. It is not a new large implementation or acceptance of the current UI systems. Root alone writes production and runs Cargo/GPU. The separate Snap candidate 929c4ca757c027dc62eec2d140b453690558d15a9e5d86efe3669042209927cd remains frozen; its independent scoped source review does not establish this parent function.

Original B is f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 with matching PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911 / GUID 09F1A02A-49F2-F8C8-4C4C-44205044422E age 1. The reused calculate body 14127cf70 is **8546 bytes**, PDB 1522:1440; 48KB was an old decompiler-text comment, not native size. The new complete move caller is 1407a4850/12790B, PDB 420:2336. The existing place caller is 1407a9c10/15522B, PDB 422:7120. All seven selected complete ranges in native-range-verification.json match original PE bytes instruction by instruction, have continuous coverage, and match their PDB code sizes. This verifies identity and static evidence, not target execution.

## Corrected output extent and source reachability

**The old 0x98B output claim is false.** Callee saves hidden RCX output into RSI at 14127cfe7, writes at most byte +5c on ordinary returns, and returns that same pointer at ea57. Place passes &stack340 at 7ac4eb, calls at 7ac507, and copies six consecutive 16-byte blocks covering +00..+5f at 7ac569..7ac5b6. Move passes &stack6a0 at 7a6c0d, calls at 7a6c25, and the final 16-byte copy at 7a6c84 covers +50..+5f. The observed transfer extent is **0x60 bytes**, with alignment/padding as compiled in original B. No public Rust type should mimic this native ABI merely to reproduce size.

The first word is not an always-valid bit. Place 7ac50c..7ac535 and move 7a6d82..7a6da1 distinguish first-word values 3 and 4 before using a placement payload. On success the first two words are copied from the actual caller's two-word context (stack arguments +58/+60 at callsites), whose domain is the three-case DecoratorStorageAddr. Success is therefore distinguished by the context values 0/1/2; the rejected forms use 3 or 4. This is compatible with a niche-optimized sum; the original author enum name and Rust layout are not present in the stripped TPI and are not claimed recovered.

The current workspace has **no actual Rust invocation** of calculate_decorator_interaction_intent or consumption of DecoratorInteractionIntent outside the owned file; ui_place_decorator only mentions it in documentation. ui_move_decorator is a phase stub. Current calculate dispatches on info.dst_tag, ignores the hit, passes an empty merge candidate list, discards merge/corner outputs, and sets valid=1 for broad tags. These are source-proven omissions; do not count a future corrected leaf as an active restored UI entry until both real callers are migrated.

## Native input and output contract

After the prologue, RBP = original entry RSP -278. The actual register/stack roles below are evidence boundaries, not a proposed Win64 API for new Rust:

| Original location | Observed role |
|---|---|
| RCX | hidden output pointer |
| RDX | actual PublicWalls; required get, roof_entity and later world-to-wall operations |
| R8 | actual Query<&Roof> owner/state; retained and queried with real entity generation |
| R9 | actual StairsState containing the 0x150B assembly records |
| RBP+2a0 / caller stack+20 | hit/proposed-transform record: quaternion +00..0f, RaycastDecoratorDst at +10, world position +20/+24/+28; roof coordinate optional payload at +2c..34 |
| RBP+2a8 / stack+28 | current/proposed DecoratorId word; propagated to output+4c |
| RBP+2b0 / stack+30 | actual DecoratorInfo |
| RBP+2b8/+2c0 / stack+38/+40 | existing exclusion slice pointer/length, used by the wall merge proposal |
| RBP+2c8 / stack+48 | actual DecoratorStorage |
| RBP+2d0 / stack+50 | canonical derived resource references, used for immediate candidate subtype and collision lookup |
| RBP+2d8/+2e0 / stack+58/+60 | caller's actual input storage address, copied into successful output+00/+08 and passed into immediate subtype derivation |
| RBP+2e8 / stack+68 | boolean enabling merge search and the later Snap application override |
| RBP+2f0 / stack+70 | actual caller-owned roof collision/merge callback input; passed intact into the roof intersection closure. Its semantic Rust name must be resolved at the caller before implementation, not replaced with zero/default |
| RBP+2f8 / stack+78 | boolean used by the roof flat-only gate and wall destination-variant selection; preserve both branches until its source-level name is resolved |
| RBP+300 / stack+80 | real UiSignifierStream, used by original roof ridge debug feedback |
| RBP+308 / stack+88 | actual same-World RaycastWorld, passed to Snap |

Entry requires all three position scalars finite (d00e..d046) and all four quaternion scalars finite (d067..d087 via is_finite_mask). There is no added normalization or fallback quaternion. The caller passes a valid four-case RaycastDecoratorDst; Invalid tag4 is excluded before this call. The exact 4-entry table at 142c24718 dispatches **0 Wall, 1 Roof, 2 Terrain, 3 StairTop**. It is a different domain from DecoratorInfo's 0/1 Wall,2 Roof,3 Terrain,4 Stair. A fifth table entry must not be invented from adjacent data.

The accepted record's observed fields and actual consumer are:

| Native offset | Contract |
|---|---|
| +00..0f | original input storage address, preserved from caller context |
| +10..1f | resolved placement storage address; calculate_move_dst passes +10 to DecoratorStorageAddr::try_as_wall for wall placement |
| +20 u32 | destination union: 0 Roof, 1 Wall, 2 Terrain, 3 Stair; independently confirmed by 142c248b0 in the full 512B consumer |
| +24..3a | destination-specific payload below; padding is not a fabricated public field |
| +40 u32 | optional merge flag, tested by actual place caller at 7ac6ee |
| +44 u32 / +48 u8 | selected merge target DecoratorId / resulting rank, read at 7ac6fb/7ac701; read only when merge flag is set |
| +4c u32 | proposed/current DecoratorId from input+2a8 |
| +50..5b | position Vec3, the branch's actual stored world position; wall/terrain/stair accepted paths copy the input world position. Do not synthesize it from final wall coordinates |
| +5c u8 | resulting rank, normally info.rank, or a selected merged rank from the real proposal |

Wall destination payload: +28/+2c actual longitudinal distance/Y, +30 platform WallId, +38 WallCornerAttachment, +39 applied outline-Snap boolean, +3a actual core Wall destination variant 0 or1. +24 and +3b..3f have no established independent semantic field in this boundary; do not read native padding or manufacture opaque gameplay fields. Roof payload: +24 relative DecoratorRotationXZ, +28/+2c real roof coordinates. Terrain: +24 yaw, +28/+2c X/Z. Stair: +24 rotation relative to the actual path orientation, +28/+2c actual stair IDs, +30 signed lateral normalized coordinate, +34 longitudinal curve U. Resolved storage is Wall for wall/roof, Terrain for terrain and Stair for stair.

Rejected form3 writes only tag3 + position Vec3 at+08..13; it is produced when terrain placement is forbidden. The move caller can convert this into its genuine removal interaction, while place shows the actual icon and does not apply placement. Rejected form4 writes tag4 + original quaternion at+10..1f + input position at+20..2b; it is used for unsupported roof/stair placement and a failed final wall projection. Both callers take a failure path before placement decoding. Never read merge/rank/placement payloads of either rejected form.

A narrow semantic Rust value should consequently be an enum with Accepted(PlacementIntent), TerrainUnsupported{position}, and Rejected{rotation,position}, using reconstruction names with native tag facts documented. PlacementIntent should use canonical DecoratorStorageAddr, DecoratorId, WallId, WallSpaceCoord, WallCornerAttachment and the actual destination value, plus optional typed MergeSelection. It must not retain current f80/u88/f98/rank-at-bc guesses or an always-valid u64. It must not cast a raw byte array into DecoratorInfo. Exact Rust layout need not match the native 0x60B sum.

## Complete ordered business branches

### Wall hit, d0b3..d3a1 then dd16..e8fd

1. Resolve actual hit WallId with PublicWalls::get; missing required wall takes original unwrap failure. Get its real WallSpace. Build the candidate derivation context from actual info plus input storage, immediately call derive_decorator_subtype, combine returned subtype and actual rank through canonical DecoratorLibKey, and require that collision entry. Missing mesh keys remain visible failures, not default dimensions. Width is collision+30 and full vertical extent is maxY-minY (+3c-+38).
2. Obtain approximate U from the actual 2D wall curve and hit X/Z, clamp through native MAXSS(0)/MINSS(1), and multiply by actual curve length. Preserve initial candidate height. Initialize attachment/platform defaults by the actual constructors; folded aliases do not justify selecting a random symbol name.
3. If merge boolean is true, get the actual wall decorators, collect actual overlapping IDs using the original wall intersection predicate/closure and current candidate geometry, then call find_best_wall_decorator_merge_position with current ID/info, real exclusion slice, WallSpace, candidate U/Y, collected IDs, actual bucket and derived resources. No bucket or proposal-state2 falls through to corner/Snap. An accepted proposal replaces coordinates, wall/platform/attachment/variant/rank/merge selection and jumps directly to final projection; it skips the ordinary corner and Snap path. The candidate vector is released on every path. An empty hardcoded list is not this computation.
4. Otherwise, only real rectangle wall shape tag1 offers corners. Visit indices0,1,2,3 in that order. Compute actual Rectangle2d::get_point_u(index)*wall length and WallSpace.project_clamp. The first three are inlined repetitions of the same strict width threshold; index3 invokes the complete 200B should_snap_to_corner helper. Offer only when the type predicate holds and abs(WallSpace.subtract(candidateCornerU,currentU)) < width*.5 + (actual style>=2 ? .2 : .46). Use original loop/open policy and strict comparisons. Select the first accepted corner, not a minimum-distance corner. Convert using actual WallCornerAttachment::from_rectangle_corner_id, use that projected U and clear platform as originally done. With no corner retain the original candidate U and attachment state.
5. Recompute real approximate curve U and its actual position for the Snap input; rotate the original quaternion basis exactly to form the XZ probe, without an added normalize/fallback. Call the independently frozen five-state Snap with **false/false**. For tags3/4 retain previous platform/Y and do not read their undefined payload. For0/1/2, apply selected WallId/Y only when current info is a wall destination with valid existing platform or bottom-door flag, OR the merge/application input boolean is true. +39 is true only when applied and tag==1; it is not equivalent to any successful Snap. Preserve the native Snap tag separately where the actual merge/destination consumer needs it.
6. Final WallSpace.project of chosen longitudinal coordinate determines the accepted placement. Success writes resolved Wall storage, Wall destination coordinates/platform/attachment/variant, optional actual merge data, current ID/rank and input world position. Failed projection yields rejected form4. No fake clearance/zeroing can replace this rejection.

### Roof hit, d9b1..e141 and e368/e44b..ea57

1. Require type.can_place_on_roof. Resolve real WallId→roof_entity and genuine Query<&Roof>; missing/invalid required entity/query is an original observable failure, not an empty default roof. If input flag at2f8 is true and the actual roof is not flat, return rejected form4.
2. Compute the type-dependent original rotation from the proposed quaternion (or the original zero branch); require the hit's roof-coordinate optional payload (+2c lowbit). Absent required coordinates takes the original unwrap failure. Subtract actual roof rotation_yaw_radians for roof-local yaw. Carry input world position and rank.
3. Type0x3d follows its own ridge path. Flat roof goes directly to accepted roof placement. A nonflat roof calls snap_to_roof_ridge first with threshold **4.0**, preserves the original DebugSphere/DebugLine feedback into the actual UiSignifierStream, and then resolves the final call with threshold1.0. The Point/Segment/None cases produce different roof coordinate/rotation values; do not silently omit this dependency or use a default roof center. The present evidence maps this complete branch; no new ridge implementation is proposed here.
4. Other types reject if the second real roof coordinate is ordered >1 and roof is not flat. Immediately derive the candidate subtype and require its actual collision entry even when merge is disabled. When merging is disabled, roof is flat, or type.max_rank()<2, keep the base roof placement.
5. Otherwise collect overlapping roof decorator IDs using the actual candidate transform/dimensions and real callback input. Pass the actual bucket and captured info/coordinates/roof/canonical derived resources through the complete122B Option wrapper to find_best_roof_decorator_merge_position@142000e60. Native proposal-kind4 means no merge and retains base placement. A real proposal replaces destination payload, rank and optional merge selection before accepted return. The current reduced proposal helpers do not yet establish this complete payload contract.

### Terrain hit, d3a1..d415

Require type.can_place_on_terrain. If false return form3 with original position. Otherwise derive DecoratorRotationXZ from the actual quaternion and accept Terrain storage, Terrain destination with input X/Z and yaw, input position, original rank/current ID and no merge. Do not use info.dst_tag or set valid unconditionally.

### StairTop hit, d41a..d9b1 and dba2..dcf4/e15e..e297

Require the exact original can_place_on_roof predicate at d426; do not change it to a similarly named stair predicate without evidence. Find the actual 0x150B PlayermadeStairsAssemblyParams record by both IDs from the hit. Missing pair takes the original diagnostics/panic path (including actual assembly IDs), not None.

The complete native branch projects hit X/Z onto the actual sampled stair path/cross-section with four refinement rounds. Each round evaluates eight values with exact f32 1/7 at142c2415c, keeps the actual ordered minimum, and narrows to its neighboring interval. Partial comparisons reject NaN through original unwrap/panic. It then resamples path/derivative/width arrays with their independent bounds checks, preserves the reciprocal normalization/degenerate signed-zero fallback, calculates signed lateral coordinate using actual half-width, and offsets the input yaw by actual path orientation. It accepts Stair storage and Stair destination carrying the two IDs, lateral coordinate and longitudinal U. Current broad tag3 valid=1 has none of this business.

The 8-sample numerical routine is a real local business component in this body, not a generic closest-line replacement. Its full raw body is retained; implementation requires exact typed StairsState geometry access and all compared values, tie policy and panic conditions from this range. No new mocked stair registry or substitute two-point path is authorized.

## Actual next consumer, not just a kind enum

Original calculate_move_dst@1412801f0/512B receives output pointer, **&accepted intent**, and actual PublicWalls. It dispatches on intent+20 using four exact entries142c248b0. Roof/terrain/stair construct the real core DecoratorDst payload and write core discriminants2/3/4, respectively. Wall uses intent+10.try_as_wall (required), requires the actual wall from the same PublicWalls map, computes its base Y from the real wall coordinate, subtracts that from selected Y, calculates actual wall-space relative placement, and preserves platform/corner/applied-door/core-wall-variant. Missing owner/wall remains observable. This is not the current move_ops::calculate_move_dst(&DecoratorInfo)->MoveDstKind, which loses all computed payloads.

Original place only sends successful accepted payloads to this consumer (7ac6bd..7ac6d2), then uses the genuine merge bit/target/rank to choose the actual op path. Original move likewise converts the successful value and applies its actual operation. form3 and form4 have separate existing UI/backup/icon/removal behavior; do not flatten them to valid=false or silently treat form3 as a placement with no owner. The migration must retain actual deferred-op/backup handling owned by those UI callers; this packet does not reconstruct their entire 12790/15522B systems.

## Current dependency and ownership gates

current-source-ownership.json freezes every current source SHA and the actual on-disk claim snapshot. Absence of a claim at audit time is not authorization to write it. Only this owned intent file may host a future candidate after design review; root must coordinate other tasks. The three world queries already integrated through current919bfa69 and the separate Snap candidate must not be replayed from old baselines.

| Required boundary | Current task / status | Concrete migration obligation |
|---|---|---|
| calculate + value enum + local stair branch | TG-60234a3eb4a6f6e4d2, this owner | Replace fixed guessed struct and approximation only after typed dependencies/callers are available; keep Snap frozen body |
| calculate_move_dst | TG-e1fa208f690e210d89, no claim observed | Real accepted-intent→DecoratorDst with same PublicWalls, payload and required failures; narrow root-coordinated source claim needed |
| actual move system | TG-9073c40e5ed5e52282, no claim observed | Supply real transformed hit, original address/exclusion/flags/owners and consume all three result classes; current phase stub has no calculate call |
| actual place system | TG-3ab1be37613b300ae5, owner clutter-startup-20261020 (01) | Same input/value contract; preserve original transient IDs, merge op, backup/UI state; current loop handles only init scaffolding |
| candidate hit conversion/transform | TG-919392431783aadd84 and grab-offset file, no claims observed | Actual RaycastDecoratorDst four-case + real quaternion/position/roof coordinate value; current RaycastIntersection guessed88B is not this 64B input proof |
| immediate candidate derive | TG-a790a7eb5fc820c629, no claim observed | Actual singular derive with candidate storage address and same resources; current private derive_one is only two window families and cannot be replaced by a stale ID cache entry |
| wall merge / intersection | TG-e6891d165c4bc44d47 / TG-e3145687c65378b3d0, no claims observed | Real IDs, actual exclusion slice, collision filtering and proposal payload; legacy public helper is None even for nonempty input and its full env path retains documented approximations |
| roof merge | TG-4f40224d71f85debf9, no claim observed | Actual roof predicate/optional proposal-kind4 and complete accepted destination/merge payload; current reduced roof proposal is not full equivalence |
| rectangle/corner | TG-6e76d82e8a953a9a45 foreign boot-leaf-20261018; TG-719dcf28a173a4b9b1 owner01 | Reuse real Rectangle2d and reviewed 200B strict U-distance helper, not old None corner; no foreign source write |
| current Curve API | TG-9cab24dd1d18f9a94a, no claim observed | Reuse actual wall/stair data and f32 operations; library reuse is not proof its complete closest-coordinate behavior matches every parent branch |
| roof ridge feedback | TG-fc452fca09cf0fa878, no claim observed | Existing real snap_to_roof_ridge + actual stream side effects; verify original branch payload, not replacement algorithm |
| canonical storage/publication/mesh/derived inputs | existing owners | Borrow same World and actual registry generations; keep missing actual DecoratorMeshLibrary population and broader derived producer coverage explicit; no default bbox/empty catalog |

In particular do not modify foreign Storage to make this parent compile. Parent input-storage and placement-storage are existing canonical values, not a new registry or changed authority-map schema. Mesh test fixtures verify a leaf only and are not a runtime catalog producer. Existing complete-core E0583 missing utils/sampling/world_frame remains a central build gate; there is no repeat build request in this packet.

## Review and validation sequence

1. Independently review native0x60 extent, four-case input, two storage addresses and destination payload/real consumers before approving a new value/API. Confirm remaining caller input2f0 and flag2f8 semantic roles through actual setup before fixing a final public signature. This contract deliberately does not claim those names recovered.
2. Root assigns narrow missing source owners/dependencies; same derived resources and mesh value must agree with01's actual location/validate input view. Do not restore a new large parent against two parallel derived catalogs.
3. Then prepare an owned source candidate plus meaningful tests: hit-vs-info dispatch mismatch; all accepted destination payloads and both rejected forms; context/resolved owner preservation; genuine nonempty merge shortcut bypassing corner/Snap; four corner first-match/strict threshold/open-loop wrap; flag-conditioned Snap application0/1/2 versus3/4; roof required query/flat/merge/ridge cases and real feedback; actual eight-sample stair refinement and missing/NaN/array failures; genuine calculate_move_dst roundtrip and caller merge selection. Tests must use actual canonical resources, current World/Rapier and actual dependency functions; no surrogate parent or mock catalog passed off as gameplay.
4. Root alone compiles/runs. No test in this audit was built or run. Snap8 and world14 tests remain separately unrun at this audit point. Static native/source review and body size/case count are not recovered runtime progress.
