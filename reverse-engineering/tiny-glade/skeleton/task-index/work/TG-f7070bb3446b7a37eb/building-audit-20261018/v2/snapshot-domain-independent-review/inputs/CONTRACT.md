# B snapshot domain serialization and the existing Motomoto file writer

This is a bounded candidate/handoff for the existing `systems/save/snapshot.rs`. That original family TG-3adf041a3108edb59a remains legally claimed by `main-chain-boot-20261018`; this agent has not taken its claim, written its work directory, production, Cargo, or the save-thread owner's source. Root requested this independent incremental candidate under our already delegated TG-57a33750ac4a866a76 work. The task index defaults to C; every native fact and candidate behavior here explicitly targets B. No C parity is implied.

The candidate retains SnapshotDomainData, SnapshotSaveData's current Arc API, existing domain types, current public Tree entry structure, current HistoryState, and the existing Motomoto crate. It does not create a second snapshot ledger, engine, process-memory dump, domain owner, or decorator collection. Original SnapshotSaveData ownership is Box, while current Arc is a permitted architecture adapter; the old Arc/0xc0-header claims are not file-format evidence.

## Confirmed bugs and exact native identity

The old `to_section_values` only serialized nonempty Trees and caller-injected Values, silently discarding actual walls, roofs, decorators, colors, terrain editing, AutoClutter IDs, stairs and removed supports. The old Trees entry derive also emitted an object rather than the original two-element tuple. `capture_game_snapshot` and save_thread's current capture path wrote bare JSON, whereas the native file contains Motomoto chunks. Native Version is 2, separate from the containing game save's Version 27. The previous nine-section list was inferred from a partial load/log path and is not the complete native schema.

Identity is original B binary SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 and matching PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. `verify_evidence.py` verifies both whole identities, all 19 saved ranges (34,113 bytes / 8,142 instructions), every raw byte and Capstone text, 56 actual ClutterMeshTy table/string bindings and isolated exact two-patch application. Seventeen ranges are complete source/PDB-bound procedures; `support-field-bounded` and `corner-field-bounded` are explicitly 110/19-byte dependency snippets, not complete-function claims.

The original capture procedure @1417a1d00 is **20,581 B**, module1722 record19228, 4,935 instructions; @1417a0430 capture-save-data is **6,343 B**. Neither the older 6,947-B extraction nor the Ghidra text that falls through into another procedure defines the full capture boundary. `native-ranges.json` contains exact procedure records, aliases and source mappings.

## The fourteen JSON fields

Native capture inserts every field, including legitimate empty domain collections. Calls below are inside the complete captured procedure. JSON object key order follows serde_json's map implementation and is not a different file section engine.

| Field | Original serializer/call | Current owned input |
|---|---|---|
| Version | native constant2 | SNAPSHOT_FORMAT_VERSION |
| Walls | call1417a211d ->141590a30 | Vec<(WallId, WallState)>, canonical WallState Serialize |
| Roofs | call1417a2252 ->14158faf0 | Vec<Roof>, canonical Roof Serialize dependency |
| Decorators | call1417a2390 ->141560e90 | canonical address buckets, true DecoratorId plus HistoryState |
| Raster | call1417a2579 ->1415905e0 | vector of raster **names**, not byte payload objects |
| WallColors | call1417a2678 ->14158f8b0 | existing full index + ColorId pairs |
| GateColors | call1417a283e ->same helper | existing pairs |
| HalftimberColors | call1417a2975 ->same helper | existing pairs |
| Terrain | call1417a2aaa ->1408e6b70 | strokes and next_stroke_id; no runtime rasterized flag |
| TerrainHeightRange | call1417a2be4 ->140b30f30 | actual Range<f32> resource, {start,end} |
| Trees | call1417a2d19 ->141590160 | (TreeStorageIndex, existing six-field TreeEditData) tuples |
| AutoClutter | call1417a2e58 ->14158f050 | DeletedAutoClutter IDs, sorted numeric u64 |
| Stairs | call1417a2f8c ->14157d810 | {edges: pairs, checksum: Option<u64>} |
| StairsRemovedSupports | call1417a30ce ->140b3bad0 | {edges: sequence of full edge pairs} |

All Walls/Roofs/Trees/outer-address rows and AutoClutter IDs use the native existing sort keys. Canonical decorator sort also sorts true IDs within each bucket; borrowed views preserve that ordering without copying the large raster/terrain buffers. Sorting does not synthesize IDs, curves or missing domains.

Terrain @1408e6b70, full348 B, explicitly binds `strokes` and `next_stroke_id`; its next ID serializer is usize. Current handwritten terrain counter is u32, so encoding widens that existing value to usize and does not invent a second counter. Decode of counters beyond current u32 remains a manual-type boundary. Raw height buffer capture is separate: @1417a0e6e..0e9d loads the actual height owner, multiplies its element count by four, copies the bytes and takes range at resource+0x50. The encoder accepts the resulting owned bytes unchanged; it does not guess conversion from the current u16 height model or derive a range from strokes.

StairsSnapshot metadata must not call compute_checksum while serializing: the original field is Option<u64>. The necessary owner hunk adds only `cached_checksum(&self) -> Option<u64>` on the existing authoritative type. Removed supports currently use HashMap<StairGraphEdge,()> as a set; the private borrowed projection emits the original HashSet sequence, using each canonical edge's nodes(). Native StairGraphEdge is a two-element tuple, not the current named a/b derive object. No second graph or support state is created. Hash set order is unspecified; no unproven edge ordering is introduced.

## Decorator schema dependency and precise projection

The canonical bucket/schema dependency is terrain worker's TG-889aa76ca030a2e3cd/terrain-curve-20261018 frozen69 manifest SHA af133443bd03bf895633b6e22e0d297f09fef346d3c20cb58653216c4eaa6144. It fixes take_snapshot, original type-directed restore conversion and canonical DecoratorOpEvent forwarding. Its original State value serializer1417c0cf0/1144 B and stream serializer1417c0960/900 B govern this candidate's borrowed JSON adapter. Storage address value141560650/262 B is {"Wall":u64}, "Terrain", or **"Stairs"** (plural unit). Canonical Rust variant spelling Stair is retained.

Original outer serializer141560e90 uses key **states**, typed Vec<(DecoratorStorageAddr,DecoratorAddrSnapshot)>; inner141560fc0 uses key **state**, typed Vec<(DecoratorId,DecoratorHistoryState)>. Author's older pack filenames bucket/snapshot were inverted labels; dispatch is determined by actual key/type, not those labels. This gives {states:[[addr,{state:[[real_u32_id,HistoryState],...]}],...]}, preserving Some(empty) buckets. No rotate/XOR ID encoding is used.

HistoryState projection uses the existing owned to_bytes() getter, never process addresses. Mandatory fields are dst(first40 B), ty@4a, rank@48 as {x:u8}, seed u32@2c. Conditional fields are color_id u32@28 !=0, clv byte49 !=0, h_offset f32@30 !=0, allow_connection byte4c when false, window_light_color u32@34 !=0, lantern_brightness f32@38 !=1, chimney_smoke byte4d when false, stair_width f32@3c !=1, ladder_color u32@40 !=0, railing byte4b !=0, stairs_supports byte4e !=0, magic_color u32@44 !=0. ±0 compares equal; NaN includes the field and serde_json writes null, matching the original Value serializer. Default nonzero bool bytes are omitted rather than clamped or exposed as a raw integer.

DecoratorDst's niche byte22: 0/1 Wall; 2 Roof; 3 Terrain; 4 Stair. Wall coordinate is {x,y}. Wall anchor stores platform u64@8, relativeY f32@10, anchorXYZ@14..1c, corner byte20 always, bottom-door byte21 when nonzero, Inner side for byte22=0 only; Outer=1 is omitted. Roof is {mesh_uv:[f32;2], optional relative_yaw_radians}; Terrain is {pos_xz, optional yaw_radians}; Stair is {graph_edge:[u32;2],uv:[f32;2],optional yaw_radians}. Current simplified runtime destination enums are not used to erase attachment data.

Type56..61/63 strings are CottageWindow/GothicWindow/ArrowSlit/Chimney/Lantern/Flag/Stairs. Types0..55 emit {Clutter:{ty: original ClutterMeshTy name}}. `clutter-mesh-string-bindings.json` binds all56 jump entries, actual string pointers and lengths from the full981-B serializer14179a560. Railing1/2/3 are Low/High/Wooden; nonzero supports emits None, with Full default omitted. Invalid otherwise-unrepresentable niche/type/railing bytes return a host error rather than indexing an original jump table outside its valid enum. This is safe current-type adaptation, not equivalence for invalid native Rust enum bit patterns.

## File chunks and the existing engine

Native @1417a31e2 uses literal Snapshot len8 and adds its JSON chunk. Each raster @1417a35e7 calls tag_from_str_trunc, then writes **u64 little-endian raw byte length**, followed by raw Deflate at level6. @1417a3a38 uses literal Terrain len7 and writes level6 Deflate of actual raw height bytes **without** the raster length prefix. Existing Motomoto Writer handles 8-byte tags and length backfill, and all fallible operations propagate through `encode_game_snapshot(&SnapshotDomainData) -> anyhow::Result<Vec<u8>>`. SnapshotSaveData::encode delegates to it. Compatibility capture_game_snapshot delegates and expects the result. There is one archive encoder, not a second manually written framing or compression implementation.

Three values absent from the old collector signature are explicit Option captures: raw terrain bytes, actual height range and removed supports. `with_terrain_and_supports` supplies them; missing input returns Err. Some(empty) is a real supplied capture. The old optional-stairs compatibility signature now requires Some rather than silently substituting an empty snapshot. Existing unknown JSON extension keys are preserved for compatibility, but collision with any captured native key returns Err; injected Version27 cannot hide missing or overwritten native domains.

## Validation and limits

Rustfmt parsing/format verification and static verifier/isolated patch exactness pass. Five new Rust test sources cover fourteen nonempty actual owned domain values, full64 color identity, true bucket IDs/empty buckets, tuple Trees, original default/NaN wire rules, raw raster and Terrain chunk decoding through existing Motomoto Reader, missing inputs and duplicate-key failures. Existing source tests are adapted only where file output is now an archive and canonical buckets replace false flat records. **No Cargo, native/JIT execution or test pass count is claimed.** Root/actual owner must run integrated checks with the canonical Roof/shape and bucket dependencies.

Roof dependency is building worker's TG-4df0b3511f93a7ad7a/building-roof-schema-20261018 frozen139 SHA e690557ad3af91006228a88fa5182858a1a7d59974f1806bf77fdabdec0824ee. This candidate directly calls its canonical Roof Serialize and current WallState Serialize; no duplicate Roof DTO or shape engine is made. The shared ShapeParameters nonzero Circle rotation fix ({val:f32}, ±0 omitted) must also be included by its owner.

This bounded writer does not pretend the current JSON-only decoder or SnapshotPrefix memory projection is a complete native chunk loader. The old DecoratorInfo DTO discarded most state and produced Default DecoratorInfo for every ID; canonical bucket changes made that conversion invalid as well. Its compatibility conversion now returns Err and never fabricates objects. Remaining proper native State Deserialize/default-visitor and full load/world-domain restoration need canonical owner work; the old logging apply surface is not accepted as success. Legacy zero-record wall/decorator helpers are retained only as pre-existing compatibility scaffolds, are unused by this file writer, and are not verified or accepted by this contract. The current load_game valid-snapshot rejection remains an explicit known limitation until that chain is restored.

The save-game/thread owner must supply actual live collections instead of the old empty iterator placeholders and call encode's Result before writing/hash assignment. Its existing JSON-text write must not remain as the actual save path. Pending snapshot ID behavior and blob hash assignment are separately evidenced in read-only-save-audit; this candidate does not infer an ID from chain index or cursor. It neither changes nor claims complete autosave/shutdown scheduling.
