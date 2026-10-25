# B tree identity and original history schema — incremental contract

Root owns claims under core-wall-tree-adapter-20261018. Delegated history worker prepared only these candidates, never production/Cargo: resources/history/tree_edit TG-ab197f381ed7615ce6, replay_tree TG-ad24c4f393c2282e68, ui_place_tree TG-347c9d16cc440d611d, ui_move_tree TG-6da6686f3f5f072d5c, convert_preplaced_trees TG-14a1d4b6fb871ce8de, process_tree_cmds TG-3add2880f5e9bb64c3. Each has base.rs, source-base.json, candidate.rs, candidate.patch. Root separately handles hide_trees/ui_delete_tree/PrefabInstance/TreeStorage and uses the same constructor API. Shared integration-owner.patch only dispatches saved EditType::Tree and adapts one existing TreeEditData test literal.

Target B original RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 and PDB SHA256 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Fresh read-only LLVM pack status ok3 and raw PE key extraction bind the following exact facts. This work does not use C addresses or mutate a database.

## Distinct identities and layout

Original TreeData serializer14003d4e0 serializes position+18, scale+24, rotation_degrees+28, id(i32)+2c, String+0, is_preplaced+30. The nested id is placement id, the first24B of the actual48B PrefabInstance at placement+20. It is not a unique TreeStorage index. Original hide_trees Delete Arc0x58 has tag2@16, separate u64 storage_index@24, String@32, placement first24B@56, is_preplaced@80. Root retains that query ownership.

Original runtime replay141bc34f0 reads Place/Delete storage index from Arc+18 (hex, decimal24), while nested placement id is Arc+4c (hex, decimal76). Move index is Arc+30 (decimal48), from Vec3 is Arc+14 and to Vec3 Arc+20. These offsets exclude the two Arc header qwords only where explicitly named. The old id() widening nested i32 could delete/move a different tree; negative placement variants widened into huge fake indices.

## Original saved schema, including the outer index

It was incorrect to infer that six-field TreeData means no saved storage index. The separate original TreeEdit serializer14003d650 proves:

- Place: {"Place":{"index":u64,"tree":TreeData}}. index read from enum+8 at0x14003d800; key"index" atd80a, key"tree" atd82e and nested data enum+10.
- Delete: same index/tree shape. index read enum+8 at0x14003d91a; keys atd928/d948.
- Move: {"Move":{"index":u64,"from":Vec3,"to":Vec3}}. index enum+20 atd6f1; from enum+4 atd71f; to enum+10 atd743; keys d6ff/d723/d747.
- TreeData keeps its six entry sequence, names and types. serializer-key-bindings.json records original instruction bytes and exact referenced key text. LLVM identifies the serializer as the original tree_edit::TreeEdit/TreeData implementation.

Current public Place(TreeEditData)/Move(TreeEditData)/Delete(TreeEditData) constructors remain. TreeEditData adds storage_index:Option<TreeStorageIndex> #[serde(skip)] so a standalone TreeData decode has no fabricated identity. The outer custom Serialize errors for None, instead of widening placement id or defaulting index0. Outer Deserialize requires the actual index and binds Some. Move uses a small explicit for_move constructor: its unrelated current TreeEditData fields are inactive host representation and are not serialized as native TreeData. Existing previous_position remains the Move from owner. No original ABI struct or parallel tree model is introduced.

Factory signatures:
TreeEditData::from_placement(index:TreeStorageIndex, placement:&PlacementDataSsbo, prefab_name:String, is_preplaced:bool)
TreeEditData::for_move(index:TreeStorageIndex, from:Vec3, to:Vec3)

Root PrefabInstance is the existing module/type as tuple wrapper instance.0 over actual PlacementDataSsbo. It has no invented active-data Default. Root hide/ui_delete call the same from_placement factory with the actual PrefabInstanceData name and TreeComponent preplaced flag.

## Insert transport and existing consumer adapters

Original replay writer141bc3975..39c1 writes72B EventInstance: String0..18, position18..24, scale24, rotation28, placement id2c, is_preplaced30, storage index38, EventId40 (all offsets hex). The prior comment that EventId+40 was an extra height is false. This candidate adds only rotation_degrees, variant, is_preplaced to existing InsertTreeCmd; no height field, raw pointer, or fabricated default. Its old Default derive is removed because all active producer fields must be explicit. Original name clone is140d7bd90.

Current actual replay emits that unique storage identity for all six Place/Move/Delete direction cases, and retains rotation/placement variant/preplaced on reinsertion. Current process_tree_cmds sends those values through existing SpawnPrefabCmd rotation/variant and TreeComponent is_preplaced. It does not rebuild the prefab owner or rewrite the rest of process_tree_cmds.

Live producer API changes:
- place_tree uses Some(next_index) and the already computed tree_variant as nested i32 ID; the old code dropped variant and used unique index as variant. Actual Insert gets the same rotation/variant/preplaced values.
- ui_move_tree uses for_move with real storage index/cache.start/final_pos. It does not invent a placement variant for a Move-only payload.
- convert_preplaced keeps its existing candidate/filter logic, reads position/scale/rotation/variant from actual PrefabInstance placement, and uses the existing entity->TreeStorage reverse lookup. Missing lookup now fails explicitly instead of creating tree0. The function's unrelated unresolved filters/rock-mask/candidate-source semantics remain unresolved and are not accepted by this transport patch.
- hide_trees and ui_delete_tree are separate root candidates, not overwritten here.

SavedEdit::Tree integration is now possible because outer deserialize actually preserves index. Existing EditType native Tree tag12 is reused; no fake hydrate from placement variant is introduced. Auxiliary nonempty MultiEdit schema remains governed by its existing unsupported error.

## Validation and boundaries

All six source candidates parse/format via rustfmt. All six owned candidate patches plus shared integration patch pass git apply --check against their recorded bases. No compilation or Cargo mutation performed by this worker. Root performs actual combined compile/tests. Added tests cover positive full64-bit index vs negative placement variant, exact original outer Place/Delete/Move JSON, required index/no-bound save errors, unchanged nested six fields, and actual replay system six direction cases preserving unique index and full known Insert fields. Existing simplified tree biology/filter/render logic was retained, not presented as recovered.

Original instruction execution for new probes was not performed: an attempted unrelated finish-wall read-only Unicorn escalation could not pass automatic review because account usage was exhausted. No approval bypass was attempted. This tree delivery uses original bounded bytes/source binding and current Rust tests for root to run, with no claim of a newly executed native tree differential probe.
