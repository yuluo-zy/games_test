# B full visual blueprint consumer: body design v1

Status: complete native body inspected; source implementation is gated on the canonical interfaces and equality gaps listed below. This document does not claim full scene integration, typecheck, tests, or runtime success. It is a continuation of root's existing TG-69cc2f973d7c14cb28 delegation; no other source or claim was changed.

## Identity and exact scope

Original B `instantiate_blueprints` is VA `0x140827910`, exactly 7,991 bytes, PDB module 471 / record 37388. Every instruction in the retained complete assembly has now been checked against the identity-bound PE again: bytes SHA-256 `c696022b75ff7d18559393d6fe56ce30ab67348cd1ea763126461f692474c0d5`; assembly SHA-256 `479f551ae2f5e3be8e6519978d94282ec1ac6565e9574507cfc502d6ead812f5`. `evidence/CORE_EQ_NATIVE_RANGES.json` records this and four exact option-equality procedures. `evidence/BUILDER_EQ_NATIVE_RANGE.json` additionally binds the complete 2,178-byte builder-slice equality. The earlier misaligned `instantiate_build_loop-bounded.asm` is not used.

The two original typed callers at `0x1403d4c21` and `0x140468909` bind these fourteen parameter types, in order:

1. `ResMut<DecoratorVisualState>`
2. `ResMut<WallHoles>`
3. `Commands`
4. `ResMut<RaycastWorld>`
5. `ResMut<DecoratorPhysicsColliders>`
6. `ResMut<WindowAutoClutterCandidates>`
7. `ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>`
8. `ResMut<WallAttachedDecoTracker<DoorStairsAssemblyParams>>`
9. `Res<DecoratorBlueprints>`
10. `Res<PrevDecoratorBlueprints>`
11. `Res<AssetMeshLibrary>`
12. `Res<AssetShaderLibrary>`
13. `Res<AssetTextureLibrary>`
14. `EventReader<OnWallDeleted>`

The exact original Rust argument names and declaration syntax are not recovered. These are typed caller identities, not a recreated ABI declaration. The texture library remains a real scheduling parameter even though the current builder's proven body does not read its original fifth library argument.

## Native business order

The source must preserve this order, including failure after earlier side effects. It must not prevalidate all blueprints, build a parallel entity registry, or respawn every stable frame.

1. Clear only the visual state's first cloth-output Vec while retaining capacity (`0x140827bc0..bca`). This is before the real collider-helper call at `0x140827bfc`. The first Vec is absent from the current visual-state source and belongs to 04's canonical ClothInfo design.
2. Invoke the existing canonical `instantiate_decorator_colliders` on the same current/previous blueprint and two physics-map owners. This is a real second invocation in the independent full-visual phase; the earlier preview/validation collider phase does not authorize removing it.
3. Build the removed-ID work set from **previous blueprint keys**, not from persistent-state keys and not from the collider live maps (`0x140827c01..ceb`). The collider-only helper has a different, already recovered removal-set contract.
4. For every current blueprint, look up the previous blueprint and record whether their Arc allocation identities differ. The same-Arc edge is `0x140827f9e -> 0x140828474 -> 0x140827fb1`; it does not skip the following output accumulation. Remove the current ID from the removed-ID work set (`0x140827fb1..fdb`).
5. Append the blueprint's wall holes (`0x140827fe0..0x14082803c`). A nonempty list first requires `storage_addr.try_as_wall()`; a non-wall source logs and panics at `0x1408292fb...`. This check is conditional on actual hole output, not a new unconditional wall-only gate on clutter.
6. Append all window auto-clutter candidates to the **same** canonical parameter-6 resource (`0x140828041..148`). The observed source is the current map's stored Arc pointer plus `0x110`/`0x118` for data pointer/length. The element stride is `0x50`. The resource is not cleared by this function. Its real drain consumer owns that lifetime. Same-Arc and different-Arc/value-equal blueprints still append.
7. If the blueprint has its original boxed ClothInfo, append a cloned value to the visual state's first Vec (`0x14082814c..209`). The source Box is at the stored Arc pointer plus `0x120`. The body copies three 16-byte blocks, qword `+0x30`, qword `+0x38`, and dword `+0x40` into a `0x50`-stride output. Semantic schema/Clone/Eq comes from 04; no padding comparison or opaque 80-byte surrogate is proposed.
8. Only now inspect the Arc-change flag (`0x1408282ed`). Same Arc keeps all persistent entities and tracker state. A different Arc performs the complete field comparison below. Value equality likewise keeps the persistent entities and tracker state; neither optimization removes steps 5–7.
9. For a changed blueprint, remove its old persistent entry and enqueue despawn of its stored entities (`0x140828860..964`). The authoritative entity identities remain in `DecoratorVisualState::persistent`; no second entity map is introduced.
10. Clone the current builder Vec and consume that clone in original order, calling the actual persistent builder for the **map's current DecoratorId**, then seal that ID (`0x140828964..a7d`). The current source can use `builder.build(commands, id, meshes, shaders, &mut visual_state.persistent)`. The original call supplies the outer ID explicitly; a host-only `builder.decorator_id` must not replace that authority. Original Vec iterator sentinels are compiler representation, not a business filter on valid builders or entities.
11. Check the outline/transform option only **after** building and sealing (`0x140828a82...`, failure through `0x14082946c...`). The original log says the blueprint did not specify an outline transform. Moving this failure before despawn/build would change real side effects. The native option equality at `0x1408251f0` binds this field to `Option<utils::components::transform::Transform>`.
12. If a previous blueprint exists and had chimney/door-stair assembly output, remove those entries using its **previous storage address** and this ID. Then add current outputs using the **current storage address**, this ID and the current parameters (`0x140828a8f..be2`). Chimney copies 40 bytes; door stairs copies 32. 05's strict add/remove state machine must be used, not ordinary map insert/remove.
13. After all current IDs, for each remaining previous ID remove its persistent entry and despawn its entities. Look up the previous blueprint; absence is an invariant failure. Remove its old chimney/door entries when present (`0x140828bf3..f33`). This is not permission to delete current/previous authority maps.
14. Finally consume actual `OnWallDeleted` events and remove each wall's state from both assembly trackers, including pending/dirty wall state (`0x140828f5c..0x14082921a`). The exact tracker cleanup API is 05's dependency contract. Fullvisual itself does not snapshot previous blueprints or clear decorator modification tracking.

## Complete blueprint equality

The stored current map pointer is compared to a previous pointer adjusted to its data. The observations below therefore name load sites rather than advertising a guessed Rust field layout. Two positioned-mesh slots already have recovered Arc-pointer-plus-ordered-pose equality.

| Native range/call | Compared output |
| --- | --- |
| `0x140828351..380` | exact DecoratorStorageAddr discriminant; wall payload only for its actual payload-bearing case |
| `0x140828380..3aa -> 0x1412a92a0` | entire builder list in order, using actual native field equality |
| `0x1408283b0..46f` | hole list length, four ordered f32 values and kind byte per 20-byte item |
| `0x140828483..5e1` | second list length; per 48-byte item, ordered f32 at offsets 0/4/8, word at 12, ordered f32 at 16/20/24, word at 28, ordered f32 at 32/36/40/44 |
| `0x140828614..77b` | collision/interaction three-state tags; Mesh compares Arc pointer, ordered position and ordered quaternion |
| `0x14082877b..7ac -> 0x1412baca0` | canonical WindowAutoClutterCandidate list |
| `0x1408287b2..7ce -> 0x140825090` | Option<Box<ClothInfo>>, by pointed-to value, not Box address |
| `0x1408287d4..7f0 -> 0x140824fe0` | Option<Box<ChimneyAssemblyParams>>, by native value equality |
| `0x1408287f2..80e -> 0x140825150` | Option<Box<DoorStairsAssemblyParams>>, by native value equality |
| `0x140828810..821 -> 0x1408251f0` | optional outline Transform, ordered rotation/translation/scale |
| `0x140828823..84d` | both actual flag bytes observed at the current stored Arc pointer +0x138/+0x139 |

The second 48-byte list is bound to the original dormer equality through PDB inline records, but the current core `DormerHoleSsbo([u32;12])` does not supply this ordered-float equality or a recovered business record. It must not be silently dropped or compared as twelve integer words. A semantic schema/name and all native writer/consumer expectations remain a separate narrow interface gate.

Likewise, deriving PartialEq on the **current host builder** is insufficient. The native 2,178-byte helper compares GothicWindowBrickDeformation's 60 bytes as fifteen ordered f32 values, and several other packed fields mix floats with integer/tag values. For example, a NaN with the same bits must compare unequal and +0/-0 must compare equal in those float positions. Current `Option<[u8;60]>` byte equality has the opposite behavior for those counterexamples. Host-only preview/window_glass/shader/decorator_id fields must not be added to native equality without observed original fields. The exact projection will be designed separately from the already frozen builder source; no whole-file replacement is authorized here.

## Canonical source interfaces and companion gates

Production blueprint before SHA is `c86444c77352782c7572396e16886e55ff2820c56e16de7c2536d927a346b07b`. Current old scalar/byte compatibility fields are explicitly unrecovered and cannot masquerade as the typed outputs below.

* Existing storage address, Arc map, previous Arc map, builder Vec and both mesh slots are the actual owners to extend. No parallel renderer or direct generator spawn.
* Add canonical hole, 48-byte-record, WindowCandidate, boxed ClothInfo and typed chimney/door output fields only after their value contracts are accepted. Existing scalar `cloth: Option<u8>` and `chimney: Option<f32>` are not those typed outputs. Legacy setters/producers need explicit migration or must remain a visible unsupported scope; ignoring their state does not implement the original generic consumer.
* 03 owns WindowCandidate/resource schema and consumer/drain; 04 owns ClothInfo plus first Vec; 05 owns strict wall tracker state/operations and Params equality. Their source files are not changed by this design.
* `WallHoles::extend` currently returns its canonical HoleStorageError. The source must preserve its real failure at the call site rather than swallow it. No second hole algorithm is needed.
* The clear wrapper is separately missing original mutable storage access: `0x14082633e..351` marks the ResMut changed tick and calls actual Storage::iter_mut with the immutable guard. Any future blueprint source candidate should include its exact narrow interface correction after 06's accepted storage contract. Do not alter the frozen collider-only package.
* Scheduling remains 06's independent responsibility. The accepted original set25 order is clear/transient-clear (5) → generators (6) → collider-only (7) → flush (8) → validate (9), whose entry unconditionally clears modification tracking. Fullvisual and copy are a separate original registration chain. This document does not move cleanup to frame end or invent an Update registration.
* 01 has found a real existing PreStartup atlas path. CPU catalog loading does not require a second atlas or a moved startup phase. Five original-helper/current-sidecar subset mismatches remain a version-provenance acceptance boundary, independent of this body.

## Required behavioral verification after implementation

Tests must execute the actual consumer/real Commands queue and canonical owners, not a second diff algorithm: first build and seal; same Arc preserves entity identities while still collecting candidates/cloth/holes; different Arc with equal values preserves identities; changed builder/mesh/pose replaces old entities; old/new wall tracker state follows strict remove→add; removed current ID removes previous entities/tracker data; wall deletion clears both real trackers; missing outline fails after real build/seal side effects. Equality tests must cover packed-float NaN and +0/-0 counterexamples, both mesh slots' exact three-state tags, and Box-value versus Arc-identity behavior.

These tests have not been written or run for this unimplemented design. Root alone runs Cargo/typechecks/GPU after dependency source synchronization and review. A nonempty clutter blueprint is presently a real producer output; it is not yet a persistent Nani entity or a nonempty draw list.
