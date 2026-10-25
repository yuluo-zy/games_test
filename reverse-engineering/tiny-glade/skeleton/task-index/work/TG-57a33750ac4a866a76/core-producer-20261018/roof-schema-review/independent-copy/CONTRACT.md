# B Roof capture JSON schema, bounded canonical adapter

Scope is Roof serialization for the real snapshot capture consumer. Task family TG-4df0b3511f93a7ad7a was empty and legally claimed by `building-roof-schema-20261018`; the index defaults to C, but this task explicitly targets B under the core recovery target. This is not acceptance of the whole roof module, roof loading, geometry, the game snapshot domain, or C parity.

Identity: original B `tiny-glade.rne` SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 and matching PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. `verify_evidence.py` checks both whole files, all 24 complete PDB procedure ranges (13,271 bytes / 3,726 instructions), raw PE bytes and full Capstone text, 27 RIP key/data references, the reused capture caller span, and exact candidate apply. This is static evidence, with no native execution, new JIT, Ghidra or Cargo.

## Producer and consumer

Original full capture is already held by history_audit under TG-57a33750ac4a866a76/core-producer-20261018/snapshot-domain-candidate/capture-original.asm. Its bounded 0x1417a2216..0x1417a22aa span writes the `Roofs` key and calls collect_seq at 0x1417a2252. We reuse and verify that span, without repeating the 20,581-byte whole capture analysis.

The PDB-bound collect_seq 0x14158faf0 (551 B, module 1628 record 29912) advances by 0x58 bytes and invokes element serializer 0x1416ea700. The element serializer (540 B, 1673:43160) emits all seven fields unconditionally:

| JSON key | Roof offset | Logical type | Original call |
| --- | --- | --- | --- |
| id | 0 | u64 | 141599600, qword read at 1415996c4 |
| params | 0x24 | RoofShapeParams, 28 B | 141594190 -> 1416eaa40 |
| wall_shape | 8 | ShapeParameters, 28 B | 141597950 |
| pivot_y | 0x4c | f32 | 141595a20 |
| color_id | 0x50 | u32 | 141599250, dword read 141599314 |
| gable_style | 0x54 | enum byte | 141599e60 |
| flat_roof_style | 0x40 | enum, 12 B | 141596b30 -> 1416e8f20 |

The 0x50 field is not animation. The old public constructor argument names remain for source compatibility and its documentation is corrected. No native repr/ABI is asserted for the handwritten Rust types.

RoofShapeParams serializer 0x1416eaa40 emits profile_01 f32@8, height_01 f32@12, tip_offset_01 Vec2@0, ridge_length_01 f32@16, ridge_dir byte24 (Width / Length), eave_length_01 f32@20, and flat_rim_style byte25 (Rim / NoRim). The last field cannot be discarded by projecting only the six typed tg_history::RoofParameters fields. Candidate implements Serialize on the existing authoritative RoofShapeParams itself, reuses tg_history::RidgeDirection and glam Vec2, and creates no second runtime parameter owner.

Gable byte0 is `Stepped`, nonzero `Overhang`; the current Rust variant identifiers Matching / Override are retained, and only their Serde names are corrected. The existing predicate algorithms are unchanged. Native strings are independently backed by 141599f4c's immediate `Overhang` and the checked `Step` + `pped` pieces at 141599f81 / 141599f78. This is unrelated to the flat-style `Matching` / `Override` names.

FlatRoofStyle is tag0 `Matching`, tag1 `MatchingStoneOnly`, tag2 `{"Override":{"color_id":u32@4,"material":Stone/Wood byte1,"pattern_id":u32@8}}`. Only the Override branch reads its payload. Original 1416e8f20 and 14158e230 / 14158e770 / 14158e0a0 prove the field offsets and integer widths. The candidate uses the existing ColorId projection. Native code reaches Override for every tag other than 0/1; the candidate follows that dispatch. This is not validation of otherwise invalid Rust enum bit patterns in the original.

The existing public `FlatRoofStyle::pattern_id()->u8` observes byte1, which is actually material. Its signature and behavior remain for compatibility; its misleading comment is corrected. The capture serializer uses the actual full u32 pattern at +8. A future public API cleanup must account for its compatibility, and is not silently declared completed by this capture closure.

## Existing shape dependency and confirmed shared bug

Candidate delegates to the single existing ShapeParameters Serde implementation. Original tag-bit dispatch 141597a1b selects Circle or Rectangle; 14157c5e0 shows Rectangle orientation Vec2@0, width@16, length@20, center Vec2@8. Original Circle 1416e7940 emits center Vec2@0 and radius@8, then omits rotation only when ordered equal to zero (both signed zeros). Its JNE / JP branches include nonzero and NaN rotations.

For a present rotation, original 1416e7a5b -> 141592910 -> CircularF32 serializer 140a44a00 (298 B) emits `{"val": f32}`, not a bare float. The key at 142ace498 is `val`, length3, and the call at 140a44a52 uses the same f32 serializer. This corrects a concrete bug in the current ShapeParameters saved adapter. `integration-owner.patch` is a narrow boundary fix on the existing handwritten module: retain runtime rotation:f32 and ±0 omission, use a private one-field Serde projection for the nested value, and keep the original value without normalization. It creates no second Shape/Circle component or curve owner and changes no geometry.

The shared deserialize side accepts the matching required `{val:f32}` field shape; its arithmetic is unchanged. Native deserialize business and the rest of snapshot loading are not dynamically accepted by this serializer evidence. Current utils::CircularF32's transparent Serde remains outside this scope; the shape adapter uses the demonstrated schema directly rather than changing the utility's runtime design or angle algorithm. Raw shape tags outside 0/1 are not valid source enum states and current ShapeParameters rejects them; no invalid-memory ABI claim is made.

Serde/glam/map/allocator machinery is reused, not reimplemented, and these 24 procedure ranges are dependency/schema evidence, not 24 recovered custom business functions. Numeric f32 -> JSON null for nonfinite values follows the original value serializer and current serde_json; CPU-specific NaN payload/sign are irrelevant to this JSON boundary.

## Candidate, verification and integration boundary

`candidate.patch` changes only legally owned roof_shape.rs. `integration-owner.patch` is delivered to root for the existing shared ShapeParameters boundary (task_queue has no separate `valid_enclosure` family); no production file has been written by this agent. The two patches apply independently and exactly to their captured bases. Rustfmt syntax/format checks pass. Five Roof tests (including 96 branch combinations and a real World/query -> Vec<Roof> -> JSON) plus one shared rotation test are written but not run. No test count is native behavioral validation.

history_audit's actual snapshot capture should call serde_json::to_value on the Vec<Roof> from the existing Roof query. Do not serialize Roof.observed() / byte arrays, substitute tg_history::RoofPresence (a distinct command schema), or build a shadow Roof collection. Roof Deserialize, source queries and the other snapshot-domain keys remain their original owner's bounded integration work. Root owns Cargo and integration.
