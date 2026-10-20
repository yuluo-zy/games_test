# Independent history wire correction

Original B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. This correction supersedes the earlier round12 claim that native9 is deprecated and native16 is typed ChangeStyle. No Cargo command was run by this agent; central validation logs determine compile/runtime acceptance.

## Independent attribution

Both complete serializer functions140314dc0 and141400220 have17-way tables, respectively1429bb2fc and142c5badc. Reading their signed-relative entries and the selected escaped-string arguments gives:

| Native operation | Save variant | Payload | Instant consumer |
| --- | --- | --- | --- |
| 9 | ChangeStyle | wall_index, before/after WallStyle |140ac0699 calls140aaa350, writes source+68 |
| 10 | ChangePlasterPeelAmount | wall_index, before/after f32 |140ac0635 calls140aaa530, writes source+60 |
| 12 | ChangeSupports | wall_index, before_removed_supports/after_removed_supports bool |140ac0224 calls140aa9800, writes source+67 |
| 16 | ChangeSkirt | wall_index, before/after bool |140ac05e3 calls140aaa350, writes source+68 |

The same source+68 updater is ICF-shared between typed style and deprecated bool skirt. Sharing this consumer does not make their variant numbers or payload types interchangeable. The old Debug names were insufficient: both9 and16 printed ChangeSkirt. Concrete serializer callee types distinguish them:141583920 is SerializeMap::serialize_entry<WallStyle>;141588150 is its bool instantiation. Plaster uses the f32 instance141585160. Tag12's exact field strings are read at142c5b69b/23bytes and142c5b6b2/22bytes.

Original outer EditType writer140316660/table1429bb340 also proves13 AutoClutter and11 DecoratorOld. The previous SavedEdit/encoder omitted AutoClutter, so legitimate future clutter deletion histories could not save even though their live replay consumer existed. AutoClutter now reuses its existing Delete{id:u64} codec and existing expansion; no replacement algorithm was introduced. DecoratorOld now writes its actual saved name, retaining original visitor aliases Decorator/Decorator4/Decorator5; the deprecated replay panic is preserved.

`verify-wire-original.py` reproduces the bounded raw audit and hashes the complete serializer, process and three updater functions. Results: `wire-independent-original.json` and full disassembly `wire-independent-original.asm`. No database, original file, generic dependency or rhapsody implementation was modified.

## Implemented chain and verification boundary

- Live typed domain now assigns9 to ChangeStyle and16 to bool ChangeSkirt. Tag10 owns actual f32 values and writes their exact selected bits. Tag12 has a dedicated saved field projection; a generic before/after serializer would be incorrect.
- Existing actual replay_wall writes each instantaneous complete WallAuthoritySnapshot to the canonical InnerWalls and its dirty notification owner. This retains source+60/+67 and style; no extra shared history_runtime special case or parallel wall authority is needed.
- `original_history_wire` adds four small linked integration tests. The first executes wire→SavedMultiEdit→ReplayBucket→actual Bevy replay_wall→InnerWalls for9/10/12/16, redo then undo, preserving curve/cache and unrelated identity metadata. The payload values are branch fixtures transcribed from proven schema, not claimed captured saves. Others cover max-u64 AutoClutter wire plus direction/timing, deprecated decorator save spelling, and missing-vs-explicit-null decorator float fields.
- Shared DecoratorInfoJson's three `Option<Value>` floats incorrectly collapsed explicit null to None, then restored0/1 defaults; comments promised original null→NaN. A field deserializer now preserves present null as Some(Null), while missing fields still take their original default. The fourth test checks raw input→typed state bits→saved null; prior state→JSON tests alone did not cover this loss. The migration owner confirmed it does not edit snapshot.rs.
- The external33-history test now normalizes only /decorators/diffs, the confirmed vectorize<HashMap> representation. Duplicate addresses and malformed pairs are rejected by the comparison helper. Ordered edit groups, curves, points, stairs vectors and every other array remain strict. Business HashMap iteration was not changed.
- For the11 Version27 histories it additionally compares original fields against the first encoded output, with only documented aliases/default omission normalized. This detects a decoder/encoder pair that silently agrees after dropping original nondefault fields. The22 older histories require the independent migration owner; they are not falsely accepted as v27 preservation.

Expected corpus totals33/2120/408/63 are independent Python input inventory, not a Rust passing result. The new tests and added comparison have not been counted passed here. Full original process behavior, non-finite geometry, all cross-crate effects, complete GPU construction, legacy14/15 migration and entire scene replay remain outside this small branch acceptance. T13 remains excluded.

## Reusable efficiency/correctness lesson

Reuse full original byte ranges and exact jump-table entries. Follow variant→saved string→field-specific serialize callee→actual mutation before assigning an ABI identity. Debug formatting and ICF-shared consumers can hide distinct business variants. Test both original-input field retention and live canonical-owner writes: roundtrip self-consistency alone can miss a mutually wrong encoder/decoder. Keep compiler ownership central; an analysis agent's rustfmt success is only a syntax check.
