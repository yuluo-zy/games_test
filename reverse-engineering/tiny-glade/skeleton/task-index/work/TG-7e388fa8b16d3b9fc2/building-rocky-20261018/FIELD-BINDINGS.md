# Direct JSON-key → source-offset contract

The primary load/constructors and all-positive actual asset alone do not bind JSON key identities to native source offsets. Independent review requested this direct evidence before freeze. The follow-up `evidence/field-bindings` contains 11 complete PDB-sized static ranges, including the actual from_str/Vec producer chain, not new custom reconstruction of serde.

## Actual call path

Original load1408bbfac calls from_str<RockyTerrainMesh>141405b00. This selects root deserialize_struct1414212b0, whose branches call the Vec<LoadedRockyTerrainVertex> and Vec<LoadedRockyTerrainTriangle> deserialize_seq141410d40/141410900. Those call VecVisitor visit_seq140965980/140965af0; the per-element next_element_seed14140bd70/14140b6d0 call14140be77/14140b7d8 and select deserialize_struct14142c2a0/141428ea0 respectively. PDB call aliases retained in each calls.json identify the Vec element and visitor interfaces. Root mesh fields `vertices`/`triangles` are compared by exact key text in1414212b0 and assigned their respective arrays; the field visitor below is therefore on the actual loader path.

## Vertex

Name visitor1417c3650, called at14142c474, compares literal bytes and produces tags `position=0, cell_bby=1, cell_id=2, dir_to_centroid=3, is_corner=4, unknown=5`. Its length-switch table at142ce251c has targets retained in field-binding-map.json. It does not depend on JSON key order.

The tag-switch at14142c487 loads table142c5dbe8 with six destinations:

| key/tag | branch | value parser | retained value | source payload offset |
|---|---|---|---|---|
|position/0|14142c497→c707|call14142c71c→141413820 ArrayVisitor<[f32;3]>|r13d + rbp-60 qword|+0..+8|
|cell_bby/1|14142c5f3→c740|call14142c755→14140cfd0 deserialize<f32>|xmm6|+0xc|
|cell_id/2|14142c545→c696|call14142c6ab→14140d270 deserialize<i32>|rbp-2c, loaded eax atc93c|+0x10|
|dir_to_centroid/3|14142c59c→c6cc|call14142c6e1→141413820 ArrayVisitor<[f32;3]>|rbp-28 plus rbp-58 qword|+0x14..+0x1c|
|is_corner/4|14142c4ee→c660|call14142c675→14140d270 deserialize<i32>|rbp-24, loaded r8d atc946|+0x20|

Success materializes Result payload through14142ca64..ca9c/cae3..cb3f: the temporary `[rbp+28..3c]` supplies source+0xc..0x20, and result+4 firstword/result+8 qword supply position. The Result discriminant occupies four bytes; do not interpret its returned +4 as the source struct's +4. The load iterator then reads those exact 36-byte records at1408bc090..0e2.

**Correction before freeze:** three integer fields in the earlier draft were assumed u32 from asset positivity. Native parsers directly prove `cell_id` and `is_corner` are **i32**, as is triangle `is_top`. Candidate restores these signed DTO fields and casts to raw u32 only during GPU packing. This makes negative valid JSON work and rejects out-of-i32 values. The native source/GPU byte mapping is unchanged for the shipped all-positive integer values. The candidate adds a signed JSON/overflow fixture independent of the shipped asset; it is written but not executed here.

## Triangle

In141428ea0, key length10 + two RIP-referenced literal pieces at141429054/905f recognize `neighbours`, then branch14142906e→9159 calls141413560 ArrayVisitor<[i32;3]> at916e, preserving the returned firstword/rbp-b0 qword. Key length6 at1414290c7/90d1 recognizes `is_top`, branch141429106→9197 calls14140d270 deserialize<i32> at91ac and retains r12d.

The success path1414292cd..2f1 and933d..9374 writes Result+4/+8 neighbours then Result+0x10 is_top; after the discriminant, these are source+0..8 and source+0xc. Load1408bc1b0..238 copies all four i32 words unchanged and adds four zero words.

## Recovery boundary

This evidence determines type/schema/field binding, not nine extra recovered game business functions. The candidate uses existing serde derives, fs and anyhow. No serde implementation, private visitor ABI, field-layout cast or separate parser was added. It does not claim exhaustive native malformed-JSON error text/line parity.
