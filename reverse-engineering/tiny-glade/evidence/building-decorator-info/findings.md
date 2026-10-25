# B DecoratorInfo72 → wall opening inputs — UNVERIFIED

All work in this parallel extension is static analysis and implementation. No Cargo, tests or new native oracle was executed. The earlier opening/subtype/asset cores retain their own scoped proof; that proof does not accept this new Info decoder, source producer or runtime wiring.

The original private modules are `country_core/resources/walls/decorator.rs`, `wall_attachment.rs` and `decorator_storage.rs`. Symbol provenance is saved in `accessor-three-version-symbols.json`; original excerpts are `accessor-static-slices.asm.txt`, `B-accessors` and `B-producers`. Std/hashbrown/Serde/glam are reused. PDB TPI precise search returning no matching Rust type is not evidence that fields are absent.

B `dst_as_wall140833260` and `try_dst_as_wall1408333a0` accept byte+22<2 and return the Info base. Roof2, terrain3 and stair4 accessors test the same byte and return base. Actual WallAttachment serialization140a4b2a0 names two fields `coord` (first8B) and `anchor` (starts+8). Anchor serializer1416fa690 and its embedded field strings prove:

| B Info offset | Actual field |
|---|---|
| 00/04 | WallSpaceCoord two f32 fields |
| 08 | anchor.platform, WallId u64 |
| 10 | anchor.wall_height_relative_y, f32 |
| 14/18/1c | anchor.anchor_ws, Vec3 |
| 20 | anchor.corner, WallCornerAttachment |
| 21 | anchor.is_bottom_door, bool |
| 22 | anchor.side, WallAttachmentSide0/1; also destination enum niche |
| 2c | DecoratorId copied from placement request and used as actual insertion key |
| 44 | rank |
| 45 | placement source stores DEV_OVERRIDE_IS_DEMO_MODE false5/true0 |
| 46 | DecoratorType; window cottage38/gothic39 in the real subtype dispatch |
| 47 | exact caller R8 byte; its meaning remains unresolved |

Info+28 and +30..43 are preserved as observed stores without inferred render meanings. The two corner values are not invented: `WallCornerAttachment::is_attached140b256b0` tests its byte!=0. The subtype's former "door" intent is actually **platform WallId::is_valid**:140814277 calls140b34f20 on the anchor beginning at Info+8, which tests u64!=0. It does not test is_bottom_door. Cottage derives corner3/platform4/half1/plain0; gothic6/9/7/6. Arrow-slits3a and non-wall destinations remain separate consumers.

Three versions have materially different layouts: A destination tag+21 and wall base return, B tag+22 and base return, C tag+32 and return base+10. Shared source symbols do not authorize applying B offsets to C. This implementation is explicitly B-only.

`decorator_source_from_place` reconstructs actual apply_decorator_place140a20c70 field stores from the observed88B request, named dev demo state and exact caller byte, while guarding original readonly storage panic. It does not invent an ID generator. Request address16B is passed to get_or_insert_default; next40B goes to Info destination, request50→Info28, request4c→Info2c/key, request38..4c→Info30..44, request56→rank and55→dtype. Std map insertion stays in the storage owner; source construction alone is not a renderer/bundle recovery.

The new runtime `decorator_info_binding.rs` reads actual tracked storage and actual global-ID→Wall address. It requires an explicit native ID→OpeningId identity map, checks the embedded actual producer ID, uses real wall-space coordinate directly, derives subtype from the actual platform/corner/current Raw/Prepared/Public/Roof context, retrieves the real selected subtype/rank collision asset and writes `OpeningAuthority`. This feeds actual opening/hole/wall geometry consumers without a position/family sidecar or a world→wall roundtrip. Missing source/path/assets, unsupported destination/type/corner and inconsistent identity stay pending. Removed live IDs remove their known derived opening. The resolver observes state every update so original storage changes and host/roof/cache changes propagate.

The former sidecar bridge skips native-owned IDs, including its stale-removal path. Parent needs the corresponding old placement opt-out and shared core/module registration. Scheduling is explicit CPU adapter: native identities before Authoring; native Info projection after Placement/before Openings. Original fork ordering, anchor adjustment producer, corner renderer, special-destination renderer and unknown Info render metadata remain unclaimed.

## Actual source insertion extension (unverified)

B14155afe0 `Decorators::insert<DecoMoveToStorage>` removes the ID from removed, adds it to added, inserts/replaces Info, then updates the global address. If no previous record existed, global insert must return None (tag3) or the original panics; if a previous record existed, game helper `DecoMoveToStorage::update140a1b5a0` must find that ID in the live index and overwrite its address, otherwise panics. This helper does not infer or remove another wall storage. The new core `insert_decorator_source` preserves these normal business branches and exposes original invalid-index partial-mutation domains as precommit typed errors. It does not fake storage moves by erasing unrelated records.

New runtime events are `ObserveDecoratorStorageReadonly(bool)` and `ApplyObservedDecoratorPlace { request:DecoratorPlaceRequestObserved([u8;88]), observed_caller_byte:u8 }`. Register initializes `NativeDecoratorPlaceState`; source construction/insertion runs after Authoring/before CollectRaw, then direct Info projection runs after Placement/before Openings. Actual DEV demo owner, readonly flag and initialized storage are required; missing/error cases preserve the full request queue. Successful sources modify the real tracking/index resource and immediately reach native Info-based openings in the same adapter update. Prior Info is retained as `previous_records` for its real history owner, not manufactured into an unproven decorator history variant. Upstream UI ID generation remains caller-owned.
