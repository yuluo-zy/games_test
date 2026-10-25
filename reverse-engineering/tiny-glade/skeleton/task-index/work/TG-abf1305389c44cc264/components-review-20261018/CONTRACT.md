# Components review/recovery contract — B, 2026-10-18

Scope: existing `country-core/src/components/` types, their three `_gap` fields,
module organization/documentation, and direct field producers/consumers. The task
index locks A/B/C families; this delivery targets B only, following current core
recovery policy. No C equivalence is asserted.

## Observed contracts

| Type | B payload | Writer/reader evidence |
|---|---|---|
| TreeAnimation | 16 B, align 4; f32 duration@0, progress@4, canopy visibility@8, byte hidden@12; trailing padding | register 0x140846ae0; process_tree_cmds 0x142267430; original handplaced instructions 0x142279caf–0x142279deb; despawn 0x1422686d0 reads +4 |
| TreeMoveBend | 8 B, align 4; f32 world x@0/z@4 | register 0x1408471d0; process_tree_cmds; tree_move_bend 0x1422689c0 |
| TreeComponent | 16 B, align 8; u64 tree index@0, bool preplaced@8 | register 0x140845d00; process_tree_cmds command payload; spawn_preplaced_trees 0x140862270 writes preplaced=1 |
| ConvertedPreplacedTree | 16 B, align 8; owned prefab name | register 0x1408463f0 has drop; spawn_preplaced_trees clones Arc; convert_preplaced_trees 0x14226be70 calls PrefabName::as_name_ref on second query component |
| Ui2dImage | 32 B, align 8; String storage@0..24, u32 layer@24 | register 0x14084f5d0; inline new in decorator_cursor_icon 0x1407adda7–0x1407addb6 writes cap/pointer/length and layer=0; UI pass reads pointer@8/len@16/layer@24 |
| UiDrawOrder | u64, 8 B/align 8 | register 0x140850aa0; draw pass comparator + color_button_animations 0x1422979b0 |

Field names are semantic recovery names, not author identifier recovery. Use
existing `asset::prefab::PrefabName` and standard String/Bevy; do not reconstruct
their internals. Do not expose compiler padding as fake unknown data.

## Behavior and review corrections

- Keep animation progress on each entity. Query-order-indexed `Local<Vec<f32>>`
  attaches progress to the wrong tree after removal/archetype changes.
- Deletion reads progress@4, never duration@0. Strict `<0.01`, NaN excluded.
- Preserve the temporary hidden byte until the animation consumes/clears it;
  visibility and growth progress are separate quantities.
- Inserted toolbar images retain their label and layer zero.
- Component Clone behavior must remain B Default for types in this scope. Bevy
  derive automatically chooses Custom when Clone is added; remove unintended
  Clone/Copy derives rather than rewriting the dependency.
- Registry is reverse-engineering metadata, not runtime registration logic.
- Nani bundle's observed copy extent 0x48 is not its Rust allocated size; local
  16-byte alignment rounds allocation to 0x50. Native x64/ARM ABI not equated.

## Boundary

Three existing `_gap` payloads are fully described by the retained B writer and
reader evidence. Prefab placement GPU writes (+0x20/+0x24/+0x28), global canopy
state, unrelated events/history fields, complete original schedule/GPU behavior,
and the other registry entries' absent type definitions are separate outstanding
work. Do not certify these by compilation or by this schema review. Relevant old
reports describing the former component shapes are historical after integration.

Verification: focused layout/Component descriptor and tree deletion/query-stability
checks plus compilation of country-core and directly changed consumers. No new
dependencies, source lock changes, raw ABI transmutation, or native execution
equivalence claim.
