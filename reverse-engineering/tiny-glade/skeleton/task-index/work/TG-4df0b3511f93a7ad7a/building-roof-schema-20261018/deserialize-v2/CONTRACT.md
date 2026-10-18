# B Roof snapshot reader: bounded incremental closure

Target is current **B**, explicitly overriding the index's default C. PE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`, matching PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`.

Owned group TG-4df0b3511f93a7ad7a remains `building-roof-schema-20261018`. New work is wholly in `deserialize-v2/`; the already reviewed 139-input writer freeze is unchanged. `base.rs` is that freeze's **candidate.rs**, SHA256 `218d62d0e86c94bd40396254489667fb82ee13bf13e742847807109b3a43bf4e`. No production source, Cargo file, schedule or geometry is changed here.

## Native direction and minimum API

The writer closure established Roof[88], RoofShapeParams[28], FlatRoofStyle[12] and ShapeParameters[28] as the current canonical owners. This candidate adds only `Deserialize` implementations to the existing first three types. Private derived `Saved*` structs/enums exist only during a Serde call, then write the same existing byte owners through current Roof::new/ShapeParameters::from_shape. They are not ECS components, Resources or duplicated runtime state. Existing public methods, GableRoofStyle variant identifiers, shape geometry and serializer logic are retained.

The reader's named/sequence schema is independently recovered here; serializer emission does not imply required fields. Native uses serde_json::Value then its generated visitors. The candidate remains generic Serde while the JSON Value case is the proven wire boundary. Error messages and error allocation layout are not asserted ABI identities.

## Roof fields, defaults and construction

Exact PDB 1630:350932, `visit_object` **0x1415c5460/2960B**, and 1630:43388, `visit_array` **0x1415a9230/1360B**, are Roof readers, identified by typed child calls, native field visitor 1673:43896 **0x1416ea920/288B**, and final output writes. The private ambiguous `_::impl$0` symbol names alone are insufficient; these dependencies prove the type.

Sequence order is `id, params, wall_shape, pivot_y, color_id, gable_style, flat_roof_style`. Map field tokens 0..6 follow the same order; unknown token7 is `IgnoredAny`. Recognized duplicate fields branch to duplicate_field; trailing sequence elements are rejected by the enclosing array visitor.

| Field | Required/default | Runtime destination | Native evidence |
|---|---|---|---|
| id | required u64 | +0..8 | parse 1415c5638; missing 1415c59f9; write 1415c5b34 |
| params | required RoofShapeParams | +24..40 hex | parse 1415c58a1; missing 1415c59dd; write 1415c5b46..5b68 |
| wall_shape | required ShapeParameters | +8..24 hex | parse 1415c5766; missing field deserializer 1415c5a73; write 1415c5b37/5b42/5b03 |
| pivot_y | required f32 | +4c | parse 1415c57df; missing 1415c5bcb; write 1415c5b6c |
| color_id | required u32 | +50 | parse 1415c56fd; missing 1415c5c9d; write 1415c5b77 |
| gable_style | default Stepped (byte0) | +54 | option sentinel2 init 1415c54c4; AND1 1415c5b81; sequence absent EDI0 1415a956a |
| flat_roof_style | default MatchingStoneOnly (tag1) | +40..4c | sentinel3 init 1415c54cb; CMOV tag1 1415c5ac3..5acd; sequence 1415a95a9..95cb |

The raw return writes prove the same 88B fields as the already frozen serializer. They call no roof geometry or height/profile normalization. Current Roof::new only copies these fields and is reused. Padding and inactive union bytes are zero initialized for host safety; the candidate makes no claim that original uninitialized padding is zero or that a new production birth/default has been recovered.

## RoofShapeParams aliases/defaults

Exact PDB 1630:376028 **0x1415c7ed0/2944B** map; 1630:70016 **0x1415ab620/1189B** sequence; field visitor1673:46820 **0x1416eadf0/428B**.

| Sequence field | Map accepted names | Rule | Runtime relative offset |
|---|---|---|---|
| profile_01 | profile_01, profile | required f32 | +8 |
| height_01 | height_01, height | required f32 | +c |
| tip_offset_01 | tip_offset_01, tip_offset | required Vec2 | +0/+4 |
| ridge_length_01 | ridge_length_01 | default f32 bits3ecccccd (0.4) | +10 |
| ridge_dir | ridge_dir | default Width byte0 | +18 |
| eave_length_01 | eave_length_01 | default f32 bits3e4ccccd (0.2) | +14 |
| flat_rim_style | flat_rim_style | default Rim byte0 | +19 |

Map required gates 1415c8366..8445; ridge/eave defaults 1415c83b6 and 1415c83a4 read **0x142925970** and **0x1429cd068**. Output 1415c83d2..83f2 copies fields without clamping. Both enum options initialize sentinel2 and AND1 produces0 when absent. Sequence defaults are the matching 1415ab83c/8c6/916/936 paths. Aliases map to the same token; providing alias plus canonical name is therefore a duplicate, not last wins. Unknown fields are ignored. Explicit null invokes the field's typed deserializer and is not a missing/default request.

## Strict enums and FlatRoofStyle

Complete enum Value readers are 1416e8680 Gable, 1416e91c0 FlatRoofStyle, 1416e99c0 material, 1416ea060 rim, 1416eafa0 ridge. Their exact variant-seed readers 1415d3f80/2eb0/26d0/19e0/23c0 and Flat field visitor1416e98c0 prove the names and discriminants:

- Gable Stepped0 / Overhang1. Existing Rust Matching/Override names remain, with the already frozen serde renames.
- Flat Matching0 / MatchingStoneOnly1 / Override2.
- Material Stone0 / Wood1; direction Width0 / Length1; rim Rim0 / NoRim1.

Unknown variant names fail. Unit variants accept native Serde string/unit-map forms; a non-unit payload fails. Flat `Override` reads its object with **0x1415d9440/2100B**: color_id u32 and material required (1415d9838..986e), pattern_id u32 defaults0 (1415d98da..98eb). Duplicate/unknown handling follows generated visitors; Value `Override` sequence payload is rejected by this backend (entry only accepts Value Object tag5). Full-width u32 color and pattern are preserved. No wrapper-entity or signed truncation is introduced.

## Necessary shared ShapeParameters compatibility adapter

`integration-owner.patch` is only a root/shared-owner handoff for the existing valid_enclosure module. The actual current `shape-base.rs` was captured after writer139 Circle{rotation:{val}} integration. It changes the existing reader, not an alternate type or second event/resource.

The exact native reader1673:65900 **0x1416ec620/1026B** buffers Content and first tries current enum141282e60→1416ecda0, then on *any* first error tries legacy141283000→1416ed020. Current shape children are original Circle2d (1412830a0) and Rectangle2d (141284050). Legacy CircleParams141283ad0 requires `origin:Vec2,radius:f32`; it passes fields through **Circle2d::new 0x140c97540/25B**, which copies center/radius and writes rotation+0, no normalization/guard. The old enum unit Rectangle is parsed, then native conversion asserts old result is Circle at1416ec880, with panic path1416ec90a. Candidate retains that failure rather than silently inventing dimensions.

Current Circle2d requires center and radius, optional rotation default+0; present rotation uses CircularF32 object `{val:f32}`, val required. 141283740 copies its f32 without wrap/new/set normalization. The frozen writer139 adapter already matches this. Current Rectangle2d requires orientation,width,length,center and directly writes decoded fields at141284561..4590; **there is no call to Rectangle2d::new or its geometric unit-axis/finite guards**. Existing source did call that constructor, making a non-unit decoded orientation panic even though the native reader copies it. Candidate uses the same existing Rectangle2d fields literal instead; no geometry operation changes. The private untagged input preserves current-first/legacy-second, including fallback when center exists but its rotation fails and old origin exists.

Native current shape enum content supports struct-variant sequence payloads because it replays ContentRefDeserializer; preserving that buffering also restores sequences in the existing reader. Roof/parameter ordinary arrays and Flat Override's Value-specific object restriction are kept separately by their respective existing Serde backends.

## Validation and boundary

34 exact full-PDB ranges: **32301B / 8116 decoded instructions**, original PE bytes and retained complete text, matching record sizes, defaults/key tables and semantic branches. This is static original evidence, not a new native execution/oracle. No JIT/probe is attempted after the earlier usage rejection. Ten new Rust tests are authored: six Roof/params/style tests (required/default/null/aliases/duplicates/sequence/widths plus actual World component), four existing-shape compatibility tests (including original failed legacy conversion). They have not been compiled/run by this worker. Root owns Cargo and actual integration verification.

This closes the bounded Roof JSON-value reader/type projection and necessary current ShapeParameters dependency. It does not close snapshot archive decompression, State wire loading, entity restoration, replay, geometry, total Roof module recovery, or save-thread/worker owner wiring. Those remain their actual owners' work. The writer139 freeze and its prior independent review are reused, unchanged.
