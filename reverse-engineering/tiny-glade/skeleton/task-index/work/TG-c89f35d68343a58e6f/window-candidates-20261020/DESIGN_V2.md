# Window auto-clutter candidate value and resource contract (B only)

Status: source-read-only V2 design for independent review. Supersedes V1 collector gating; V1 is retained unchanged. No source candidate, compiler, CPU test, GPU, or target execution. Root delegated TG-c89f35d68343a58e6f / window-candidates-20261020. The existing claim and old artifacts are unchanged. Production integration remains root-owned.

## Concrete blocker and evidence identity

The actual 14-parameter `instantiate_blueprints@140827910/7991B` requires `ResMut<country_core::systems::collision::WindowAutoClutterCandidates>` as parameter 6. The current production source contains neither candidate type nor resource. The current clutter system takes a different `OnWindowInput`, containing manually assembled kind/host/raw fields and `Vec3` in the original quaternion position; it is not the original shared owner.

Original oracle is B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03, matching PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911, GUID 09F1A02A-49F2-F8C8-4C4C-44205044422E / age 1. `evidence/native-pack-v2/report.json` retains 16 full PDB procedure ranges, verified original instructions and LLVM symbolization. PDB boundaries are independently selected from the retained `procs.tsv`; the pack's own boundary flag stays false as documented. The 7991B consumer is reused from TG-b3ec56669fe9d97baf / scene-clutter-producer-20261020 / evidence / instantiate_blueprints-bounded.asm, not its misaligned similarly named build-loop file.

## Metadata is not an author struct definition

The retained B TPI/IPI archive contains 6,125 TPI plus 132,599 IPI records. Exact `WindowAutoClutterCandidate` search returns no type record; TPI does not supply its fields, derives, repr, or original ABI. Generic resource parameter names establish the canonical namespace used by original systems. The element equality's two inline records map to collision.rs lines 71 and 84, but this still does not expose an author declaration. `evidence/pdb-type-query.json` binds the actual archive and result. Proposed safe Rust semantic types below are recovered host representations, not author text or a promise to match native ABI/TypeId numerically.

`collision.rs` exists on this machine and is outside the finite MISSING_SOURCE_SNAPSHOT list. That is evidence it is not one of that snapshot's missing external modules; it cannot exclude newer uncommitted contents of the same existing file on the other machine. No new source is fabricated while waiting for sync.

## Element contract and ownership

The native element has size 0x50 and alignment 16. Its meaningful values are:

| Native offset | Recovered value and evidence |
| --- | --- |
| +00 | Six-case enum tag 0..5. No invented seventh valid case. +01/+04/+08 are tag-selected payloads below. |
| +10 | `Quat` rotation: four lanes compared by `cmpeqps`; actual consumer invokes the quaternion-vector path, with all four quaternion lanes. |
| +20,+24,+28 | `Vec3` world translation; actual on-window consumer copies these to the spawn position. |
| +2c,+30,+34 | `Vec3` scale copied from the original Transform scale and independently compared as three ordered floats. +38..3f are not semantic fields in equality. |
| +40 | Stable u64 key produced by `utils::calculate_hash<(DecoratorId, &str)>@14128cc30`; no owned string is stored in the element. |
| +48 | WallId u64 copied from the actual per-wall DecoratorStorage map key; no DecoratorId truncation and no pointer interpretation. |

Tag and payload contract follows both actual copy/equality tables and the real mesh selection:

| Tag | Proposed semantic case | Native payload | Actual ClutterMeshes source |
| --- | --- | --- | --- |
| 0 | WindowFlowerpot | none | original +f0; existing canonical `ClutterMeshTy::WindowFlowerpot` (27) |
| 1 | WindowFlowerbed | +08 usize mesh index | original Vec at +00/+08/+10; index checked through `.get().unwrap()`; existing `window_flowerbeds()` owner |
| 2 | WindowShutter | +01 bool, +04 u32 | original +f8; canonical WindowShutter (28) |
| 3 | DoorBell | none | original +1a8; canonical DoorBell (51) |
| 4 | DoorKrans | none | original +1b0; canonical DoorKrans (52) |
| 5 | DoorKransAutumn | none | original +1b8; canonical DoorKransAutumn (53) |

The u32 shutter payload is copied from DecoratorInfo +28. Its business name remains unknown, so a recovered representation must name it explicitly as `native_u32_04` (or a similarly honest offset projection), preserving all bits. It must not be relabeled DecoratorId or omitted just because the current source's `u28` is unknown. A bool carries the original 0/1 side value; no u32/byte padding joins equality.

This is **not an unknown-pointer raw 80B container**. Full resource drop@140149b70 only deallocates `capacity*80`, alignment16, and never iterates elements. Full `Drain<Candidate>` drop@140ed3da0 only restores/moves the tail, with no per-element destructor. The complete blueprints drop frees this Vec allocation directly; complete clone in instantiate_blueprints uses tag-selected scalar payload and value copies, with no refcount increment, string clone, allocation per element, or pointer dereference from the element. The key is a hash result, wall is an integer map key, and index is dereferenced only as an index into the existing resource. Thus the narrow representation contains no Arc/String/Vec/reference/raw pointer; reusing arbitrary raw80 with unknown pointer interpretation is unnecessary.

Proposed representation is one shared core enum, one shared core candidate value (`kind`, `rotation`, `translation`, `scale`, stable u64 key, WallId), and one core resource owning `Vec<Candidate>`. It may derive Clone/Copy/PartialEq for values if the resulting semantics are precisely the ordered float equality and tag-selected payload comparisons. No `Eq` or hash over floating fields; no raw 80B memcmp, no padding in equality, no authored layout claim.

## Original default and real owner

The actual registration is **system_clutter::build**, not CollisionPlugin. At 140652683..1406526ae it explicitly writes Vec{capacity=0, pointer=16, length=0} and calls `App::insert_resource<WindowAutoClutterCandidates>@140ee6ee0`. The original resource descriptor has size24/alignment8 and uses the exact resource drop above. This proves an initially empty vector is the real constructor contract, rather than evidence of an owning data dependency that must allocate default meshes.

That fact does not authorize an empty-resource bypass. A future implementation must share the **same Rust resource type** between the original blueprint collector and actual clutter consumer, register it through the actual clutter configuration, and retain the real producer data. It must not init a second resource in collision solely to satisfy parameter 6, keep `OnWindowInput` as a mirror, or declare successful window side effects from an unpopulated Vec.

## Real producer → blueprint → drain chain

The original producers write the blueprint's candidate Vec (data-relative +f8/+100/+108; add +10 for native Arc header when reading +110/+118 in instantiate_blueprints). They do not write the singleton resource directly.

The complete currently identified candidate push paths are:

- cottage wall windows@1407ebfd0/14303B: flowerbed tag1 push 1407ee480..1407ee4f3, shutters tag2/side0 and side1 at 1407ee820..1407ee87e and 1407ee984..1407ee9e2; original predicates, scale and positions remain producer-owned.
- cottage corner windows@1407f66b0/9983B: tag1 push 1407f8080..1407f80f9.
- cottage dormer@1407c7ac0/11257B: tag1 push 1407c994c..1407c99b3.
- cottage balcony doors@1407f3da0/8391B: tag3 push 1407f4fcf..1407f5016 and season-selected tag4/5 push ending 1407f5181.
- shared add_door_autoclutter@1407f5e70/789B: tag3 push 1407f611e..1407f6160 and season-selected tag4/5 push 1407f5fec..1407f602d. It preserves its actual external WallId, transform and DecoratorId-based key input.

The candidate-producing grow calls were separated from 80B aliases: flag_connections and circle_turret also call grow_one@14078ca00, but their complete write shapes are other value types (cloth info / turret records), not candidate enum writes. The fullvisual call 1408281b7 also grows BoxClothInfo, not candidates. Gothic wall window's complete function has no identified candidate push; its file name alone is not proof of one. Tag0 is consumed and compared, but no current retained producer writes it. We do not invent a WindowFlowerpot writer. `bounded-call-target-candidates.json` records a byte-scan over selected call targets as *candidates only*; callsites were checked against the complete retained function instructions. This finite call-target search is not a universal proof that no further inlined/static producer exists; an implementation must not claim every original window producer recovered on that basis.

`instantiate_blueprints` performs recurring collection before its visual differential/rebuild decision. The same-Arc comparison at 140827f9e branches to 140828474, writes rebuild flag0, then jumps back to 140827fb1. That path still removes the current key from the pending-removal set, contributes WallHoles, and appends candidate values at 140828050..140828148 to parameter6's Vec. Only at 1408282ed does flag0 skip full equality/entity rebuilding. Different-Arc/fullEq-equal blueprints also append before retaining their existing entity. Thus append must occur for every eligible current blueprint on each collector run, including unchanged shared Arcs; the flag controls visual rebuilding, not candidate output. It reserves the exact required count and copies only each tag's meaningful payload. At 14082879e the fullEq path calls the 80B comparator. The candidate field must join the current blueprint semantic representation and differential equality, but collecting it must remain outside the rebuilding gate. No fake resources, unconditional clears, or empty substitutions participate.

Actual `add_autoclutter_on_windows@1422cd7d0/1622B` drains the singleton resource by setting its length0 immediately. It resolves each tag using the same loaded ClutterMeshes resource and pushes an actual AutoClutterState spawn. It uses `rotation * Vec3::Z` as direction (Quat mul_vec3 metadata + full four-lane machine path); translation is copied unchanged. Non-shutter cases set source selector2/payload0, shutter uses `!side & 1` and preserves the u32. It copies the key and WallId as different values. The original tag6 branch terminates the drain loop; it is not evidence for a valid `Skip` element, nor permission to manufacture tag6 candidates. Index failure retains the original `.get().unwrap()` failure. The current source's manual OnWindowHost plus placeholder `"window"` prefab string is not this mesh-owner contract.

## Narrow integration design and acceptance boundary

This read-only design asks review of the shared value/ownership contract, not approval to replace all window generation or AutoClutterState algorithms. If implemented after root authorization, three independently owned changes must agree:

1. Core claim: add the semantic candidate and Vec resource in the canonical original namespace, using only proven value fields; no standalone resource construction that claims producer success.
2. Blueprint owner02: use `Vec<canonical Candidate>` in its blueprint, append through the recurring collection phase before the rebuild gate, and compare the exact semantic values for fullEq. Real window generators must populate it before window behavior can be accepted; clutter's own original blueprint naturally has no window candidates.
3. Clutter consumer owner/root: replace the separate OnWindowInput route with the actual shared ResMut drain and real canonical ClutterMeshes selection; register the resource where the actual clutter configuration runs. This requires the real AutoClutterState output representation and original selector/key/WallId propagation, not a new prefab mirror.

No source candidate is being produced under this read-only task. None of these integration points is currently declared complete. In particular, a value type plus an initially empty Vec would not establish the world/gameplay chain. Focused future acceptance must run actual constructor→blueprint append→real consumer on nonempty candidates (all six tags, side/payload, index failure), prove an immediate second drain is empty while a subsequent same-Arc collector run contributes its candidates again, and check NaN/+0/-0 and padding-independent equality. Those are proposed checks, **not tests executed**. Original B machine evidence does not establish original C equivalence, runtime scheduling, UI Edit/Reload behavior, or OS gameplay.
