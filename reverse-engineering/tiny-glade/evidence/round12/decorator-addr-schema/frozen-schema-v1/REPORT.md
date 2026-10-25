# Address history state schema — owner candidate, independent acceptance pending

Observed original facts:

- Game serializer1417c1490/248B is called by141582200. It writes `before`
  (pointer142ce1ba3,length6) from object+0 and `after` (142ce1ba9,length5)
  from object+0x30. Both call141586d70 whose PDB specialization is
  SerializeMap::serialize_entry<str,HashMap<DecoratorId,DecoratorHistoryState>>.
  This is a pair of two48B map owners; no extra journal set or `states` wrapper.
- Original drop140899100/137B has both names: drop DecoratorStorageHistoryDiff
  and drop tuple<AddrHistoryState,AddrHistoryState>. Original diff extraction
  returns96B before/after pair. Generic map/serde allocation remains reused.
- Full replay restore14155ba30/6407B consumes address→before/after maps. Existing
  corresponding source scans before.states for removals and after.states for
  additions (swapped on undo), and maintains live bucket.removed_ids, modified_ids
  and d2s. It does not read a history-state removed set.
- Raw independent corpus scan:33 original files,2120 entries,408 decorator aux,
  427 address diffs,668 nonempty side maps and842 total state entries. All427
  diff objects have exactly before/after; every side is a map from numeric IDs
  directly to state objects. No side has a states/removed wrapper. Original
  Version27 nonempty Stairs+CottageWindow fixture includes clv, allow_connection,
  stair_width, wall anchor/platform and unequal before/after population.

Source audit:

- All type references occur in5 source files (source-consumers.json). The extra
  `AddrHistoryState.removed` is constructed only in the old archivist projection
  and read only by two tests asserting that projection. Neither the current
  replay receiver nor original-shaped outer history codec consumes it.
- `TrackedDecorators.removed`, `Decorators.removed_ids` and storage.removed are
  distinct live journal owners. This patch preserves all3, their before buckets,
  clone/restore behavior, and the receiver's journal updates. Removing the false
  history field does not remove deletion behavior: before-only IDs drive removal,
  and the receiver adds them to the actual live removed_ids journal.
- The outer DecoratorHistoryEdit codec already emitted only its states map. The
  candidate gives AddrHistoryState canonical direct map serde with the existing
  snapshot state conversion and has the outer encoder reuse it. This prevents
  direct `DecoratorStorageHistoryDiff` serde from leaking the invented wrapper.

Candidate changes and compatibility:

1. History AddrHistoryState owns only its typed states HashMap. Manual serde uses
   the same original-shaped DecoratorInfoJson conversion as the established outer
   codec; no raw native byte slots or invented fields are serialized.
2. Archivist is based on frozen-v2, removes only the false history-state field
   and its projection/journal assertions; it keeps actual live journals. The
   two ambiguous expected-set expressions use explicit HashSet::from (v2 test
   type erratum found by metadata). Real diff remains per-address before/live
   after, semantic equal filtering, independent terrain/stair addresses.
3. Restore is based on frozen-v2 and adds the original missing-index panic before
   d2s update. Original140a1b5a0/337B calls panic_fmt on absent key. The current
   public update helper merely inserts/returns Option, so checking before the
   call preserves that failure edge without silently inserting a corrupt index.
   Its regression checks partial bucket write remains but missing d2s stays absent.

These3 deltas must follow frozen-v2; world.rs remains the frozen-v2 version.
Production has not been written and no Cargo/codegen/runtime test was run here.
Required central gates: independent original byte review; new nonempty direct
state serde/size fixture test; actual restore/diff owner tests; original gameplay
all_shipped_histories_keep_saved_aux_and_bidirectional_replay_commands with
TG_INSTALL_ROOT pointing to original B, retaining all Version27 fields and
ordered arrays. Formatting and a candidate are not execution/interaction proof.

The v2 metadata facade found the two test type ambiguities; its other errors are
cross-crate RaycastWorld/graph/intersection types caused by the facade. No success
is claimed for that test check. Actual central workspace testing is still needed.

Unknown/open:

- Original stripped PDB has no reliable game TPI field schema. The schema above
  relies on physical owner width, game serializer and actual original JSON,
  rather than a void() placeholder or type-name inference.
- Current legacy no-storage extract_diff helper remains a known approximation
  for unmigrated consumers. The restored8-parameter system uses the real new API.
- Original RaycastGeoSource payloads and full source equality remain unclosed,
  so the full collision domain still cannot be marked equivalent.
