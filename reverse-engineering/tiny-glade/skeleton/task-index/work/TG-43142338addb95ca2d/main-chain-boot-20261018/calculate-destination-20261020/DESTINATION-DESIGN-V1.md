# Canonical DecoratorDst payload and actual intent consumer

Scope: root delegated this existing main-chain-boot-20261018 work directory without changing claim TG-43142338addb95ca2d. Only necessary DecoratorDst payload/access/Info codec and its actual consumers/tests may be proposed here. No source candidate is written before independent design review. Root alone integrates production and runs Cargo. The five systems tasks were separately live-showed NULL and normally claimed by rhapsody-correctness-20261019; their claim receipts are in TG602/calculate-closure-next-20261020.

This is the existing canonical core destination, not another engine, a second destination enum, an opaque96B record or a forced native ABI. It is also **not** the separate24B InstaCreateDeco creation context recovered by03. That context has its own tag0/2 and fields native04/Vec2/WallId; original placement explicitly converts it. Never memcpy it into this destination or infer that its WallId equals the coordinate bits below.

## Direct original evidence and current counterexample

Original calculate_move_dst1412801f0/512B, PDB1523:3180, is fully frozen in TG602/.../calculate-contract-20261020/boundary-callees-pack and included in seven instruction-by-instruction original PE/PDB checks. Original B f507a6ce and matching PDB2d9686b8 identity are already locked there. Its four-entry table142c248b0 selects the **intent** tag0Roof/1Wall/2Terrain/3Stair. Output core destination stores byte+22 as2Roof/0or1Wall/3Terrain/4Stair. These are three distinct tag domains: hit4-case, intent4-case and storage destination5-case.

Original place7ac6bd..7ac6d2 passes the accepted0x60B intent and actual PublicWalls to this512B conversion. Current move_ops::calculate_move_dst(&DecoratorInfo)->MoveDstKind cannot receive that value and discards coordinates, rotation and wall attachment. Current core DecoratorDst::Wall(WallId)/Roof/Terrain/Stair similarly has no fields for the actual output. A simple concrete counterexample is a roof intent with coordinates(0.25,0.8) and yaw0.7: original512B copies both coordinates and yaw and sets tag2; current enum can represent only Roof, making two distinct accepted placements indistinguishable before the actual apply operation.

Full workspace Rust search found no production value constructors/readers of core DecoratorDst outside its own declaration/methods/tests. Snapshot.rs contains only a diagnostic string about the native niche, not an enum use. Therefore all existing enum consumers/tests can be migrated coherently in this source; no old constructor compatibility wrapper should manufacture missing fields. Existing DecoratorInfo remains its current canonical raw field owner; this packet does not redesign its extra-info schema or history representation.

## Exact payload, one canonical type

Keep the existing DecoratorDst enum and native tag() domain, but give its variants typed payloads. Names below are reconstruction names, not recovered original Rust type names. The two u32 stair key components remain explicitly named first_id/second_id until their author type names are proven; they must be the real pair, not a synthesized index.

```
WallDecoratorDst {
    coord: WallSpaceCoord,       // absolute longitudinal distance and Y
    platform: WallId,
    relative_y: f32,
    position: Vec3,
    corner_attachment: u8,
    bottom_as_door: bool,
}
RoofDecoratorDst { coord: Vec2, yaw: DecoratorRotationXZ }
TerrainDecoratorDst { position_xz: Vec2, yaw: DecoratorRotationXZ }
StairDecoratorDst {
    first_id: u32, second_id: u32,
    coordinate: Vec2,           // signed lateral, longitudinal U
    yaw: DecoratorRotationXZ,
}
DecoratorDst = Wall(WallDecoratorDst) | WallCompound(WallDecoratorDst)
             | Roof(RoofDecoratorDst) | Terrain(TerrainDecoratorDst)
             | Stair(StairDecoratorDst)
```

Use existing WallSpaceCoord, WallId, Vec2/Vec3 and DecoratorRotationXZ. No current canonical WallCornerAttachment Rust type was found in the workspace, so preserve the native attachment byte in the one canonical payload rather than inventing another foreign module/enum. Its actual predicate/constructor can be reused as a local operation with independently bound6B/44B evidence when the parent calculates it. Do not infer its value from geometric proximity without the actual corner decision.

The five tags and fields are directly supported by original output writes:

| Core tag | Original defined writes | Canonical meaning |
|---|---|---|
|0/1 Wall|coord2f32@00, platformu64@08, relativeYf32@10, worldpos3f32@14, cornerbyte@20, doorbyte@21, tag@22|same target wall is supplied by resolved DecoratorStorageAddr, not encoded in coord bits|
|2 Roof|coord2f32@00, yawf32@08, tag@22|actual roof coordinates and relative yaw|
|3 Terrain|XZ2f32@00, yawf32@08, tag@22|actual terrain position and yaw|
|4 Stair|twoIDs@00/+04, coordinate2f32@08, yawf32@10, tag@22|actual stair key and normalized lateral/longitudinal placement|

Do not assert Rust enum size/alignment equals native0x28B. Derive Clone/Copy/Debug/PartialEq as appropriate; f32 values do not justify Eq/Hash with altered numeric semantics. Preserve exact f32 bits at codec boundaries, without a yaw-normalizing constructor or finite/clamp changes not present in that codec.

`tag()` remains0..4; `is_wall()` includes0/1. `as_wall()`/`as_wall_mut()` return the actual WallDecoratorDst payload and retain required nonwall failure; `as_roof()` returns RoofDecoratorDst with required failure. The old `wall_id()` accessor must not reinterpret coord bits or return platform as the authority owner. It has no actual caller outside its old test; remove that misleading API and migrate the test to explicit resolved storage address versus optional platform. Actual wall ownership comes from DecoratorStorageAddr in both original calculate and move conversion.

## Necessary Info codec, no other field rewrite

Add a single checked `DecoratorInfo::destination() -> DecoratorDst` and `set_destination(&mut self, DecoratorDst)` pair in the same canonical owner. Decoder dispatches only the real `dst_tag()` byte+22; unknown tags keep an explicit failure. The Wall boolean must have the original valid0/1 representation; malformed raw values must not be normalized into a successful different payload. All fields are encoded/decoded through existing raw integer fields and f32::from_bits/to_bits, not unsafe transmute or a second packed object.

Mapping to current raw fields is exact:

- coord or XZ / twoIDs at+00..07 is current dst_wall. Its name does not make those float bits an authority WallId.
- +08..0f is current dst_secondary. Wall stores platform; Roof/Terrain write/read only its low32 yaw and preserve unused high32 on a setter; Stair uses both coordinate floats.
- +10..17 is current dst_payload. Wall uses low32 relativeY and high32 position.x; Stair uses only low32 yaw and preserves unused high32. Roof/Terrain leave this field unchanged.
- +18..1f is current u18; only Wall stores position.y/z.
- +20..23 is current u20. Wall writes byte0 corner, byte1 actual boolean0/1 and byte2 tag while preserving byte3; other destinations update only byte2 tag and leave unrelated/undefined bytes untouched.

Every setter preserves current Info fields outside the variant's actually defined output bytes, including u24/u28/id/u30/u38/u3c/u40/rank/u45/type/state. It does not invoke existing set_extra, reinterpret its known legacy schema deviations, clear the rest of Info or install guessed defaults. Decoder reads only defined bytes for its selected variant; native padding never becomes a gameplay field.

This is a necessary real producer/consumer boundary: native calculate_move_dst produces a full destination payload; actual place/move op construction copies this payload into its current Info/operation. Systems must use this one codec when applying the computed value. Keep external creation-context24B→Info conversion explicit and separately proven by03; a context alone does not bypass this codec's required computed wall relativeY/world position.

## Owned intent carrier and calculate_move_dst migration

Replace the existing guessed owned DecoratorInteractionIntent struct with the actual logical sum. Its accepted record is a distinct native computation stage: it holds the two real storage addresses, an absolute wall candidate or canonical roof/terrain/stair payload, original/current ID and rank, input world position and an optional MergeSelection{target_id,resulting_rank}. Rejected terrain has position only; other rejection has quaternion and position only. No Rust ABI mimicry is required.

An owned four-case placement candidate in this pre-conversion intent is **not a parallel core storage destination**: wall does not yet have relativeY/world position, and its native tag domain is0Roof/1Wall/2Terrain/3Stair. Its Roof/Terrain/Stair cases reuse the corresponding canonical payload structs above rather than duplicate those layouts. The Wall candidate stores only the original absolute coord/platform/corner/door/core-wall-variant known at this stage. Keep original input rank distinct from selected merge rank (rank erratum81c369ac/3807c9ba); the original+5c is input rank, optional+48 is resulting merged rank.

Narrow conversion API in the already claimed move_ops file:

```
calculate_move_dst(placement: &DecoratorPlacementIntent,
                   walls: &PublicWalls) -> DecoratorDst
```

Only an accepted placement type can reach it; UI callers must first handle both rejection variants. For Roof/Terrain/Stair return the actual canonical payload without changed values. For Wall:

1. Require `placement.resolved_storage.try_as_wall()`; mismatched storage is a required error, not a fallback to the platform.
2. Require that actual WallId in the same PublicWalls owner. Preserve registry generations and observable missing owner/wall failure.
3. Use the actual wall.get_wall_attachment_height(coord.u) and preserve failure. Compute relativeY = coord.y - actual attachment height.
4. Reuse the existing public `crate::derive_decorator_subtype::wall_coord_to_world(actual_source_curve, coord)` in the systems crate; it already accepts the actual core `surface::Curve2` borrow and implements the recovered793B conversion, with visible required failure. Do not allocate/convert a new Curve2, add another numerical implementation, or pretend the core array-backed curve implements the separate utils::CurveSampling trait. Do not use a cached-old-ID transform or an unrelated/default curve.
5. Preserve coord, platform, corner byte and actual door bool. Select canonical Wall/WallCompound exactly from the original candidate wall variant0/1.

This source change does not touch existing remove signifier code or add a second MoveDstKind approximation. Full workspace inventory showed no executable caller of the current MoveDstKind helper; migrate its declaration and all future actual callers with the typed accepted intent in one reviewed integration. The parent itself must call exact singular candidate derive with the newly integrated DerivedDecoratorInfoView (same subtype/mesh owners), real cache and actual storage context, not old ID subtype lookup for the new position.

## Actual UI closure and explicit incomplete phases

Real place owner01 consumes this same accepted intent. It supplies actual transformed hit/RaycastDecoratorDst, transient/current ID, real candidate info/address, actual exclusion list and actual PublicWalls/Roof/Stairs/cache/derived/raycast/stream. On success it calls this conversion, writes the computed destination with the canonical codec, and preserves the actual merge target/rank operation path. On rejection it retains its original icon/backup/state transitions; it does not fabricate a destination.

Real move is now claimed TG9073. Its known original bounded segment7a6b6b..7a6fd3 supplies the same owners plus the actual current address/exclusion values; it takes the genuine rejection3 removal path and rejection4 no-placement/icon path before successful conversion/apply. Its whole12790B function is not yet implemented: initial hover/drag/release state, actual transformed hit production, camera/grab offset, backup/discard/apply semantics and operation/UI feedback remain required actual phases. They must be closed before mounting or claiming the full system. A dead helper or new synthetic CurrentMoveInput resource is not acceptance of that caller.

This carrier package is the shared contract that allows these owned consumers to converge; it does not authorize replacing the UI with an approximation. No foreign PlaceUI or core source candidate may be overwritten. Frozen before hashes and exact local scopes accompany the manifest.

## Meaningful validation, not yet executed

- Five destination variants including both Wall tags: exact field bit encode/decode; malformed tag rejection; setter preserves every undefined/other Info byte using a nonzero sentinel pattern. Use distinct coord bits and real owner WallId so the old float-as-owner bug cannot pass.
- Roof counterexample above, nonzero terrain yaw/XZ and distinct stair IDs/coordinates: all survive original intent conversion and Info codec. Tests must not only check tag.
- Actual PublicWalls wall with nonzero attachment height and published source curve: conversion produces actual relativeY/world position, preserves platform/corner/door and0/1 variant. Missing/wrong storage owner and missing height cache must fail; do not put a fake completion or empty owner into the test.
- A genuine nonempty merge selects a different resulting rank from input rank and actual target ID; both remain distinct, and the actual caller uses MergeSelection only when present.
- Owned actual move/place caller tests cover successful roundtrip plus both rejected classes with original backup/op/UI side effects. A pure destination test is not a claim that those full callers run.

No test/Cargo/target execution occurs in this design package. Existing complete-core missing-source E0583 and original slotmap-version difference remain explicit; do not retry the same blocked complete build or use an old core rlib. Independent design review precedes each candidate; central source integration and execution follow exact before hashes.
