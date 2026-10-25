# Second independent migration review

Scope: read-only production review; no Cargo, no candidate codec roundtrip.
Original RNE identity is independently checked and raw code/assembly preserved
in original-results.json. The complete migration range was extended past the
usual 5215-byte arithmetic window: 141471d3c..1414732b3 is5495B and includes
finish_potential_mutation and its log/control exits. Complete15561B load_game
was extracted independently from the original installation as well.

## Observed from original execution and instructions

-32 unchanged-RNE executions of legacy15 (circle/rectangle tags, both is_valid,
  randomized24B payload and64B wall id) retain the full ShapeParameters. True
  creates before=No(tag2), after=original shape; false reverses those sides.
  Native operation niche changes800000000000000e ->8000000000000003. Inactive
  union padding is not a serialized business field and is not asserted.
-65 original window-record cases (tags55..59 ×13 colors) and65 Arrow cases,
  then the combined order, execute actual separate native record loops.
  Window visits56/57/58 and changes the low color word+30, retaining999.
  Arrow visits only58 and writes999.13 GateColors entries execute the same
  original color table. The cold window decorator block is14146f6d8..f79e;
  the window flag branch14146ec9c reaches it through the earlier GateColors
  loop14146ed83..ee0a. Absence in an adjacent slice is not absence of this write.
-8 wrapper executions cover every Window/Arrow/pre19 flag combination. Native
  writer offsets+d/+e/+f become bits40/48/56 of wrapper second word respectively.
  Bit48 Arrow executes before bit40 Window in the actual load consumer.
-Pre19 bit56 at14146deb4 appends an8B unit event through the tuple writer,
  not a corrupted-file warning. Name/ABI evidence and canonical producer match
  SpawnPreplacedTrees; its capacity growth140562870 is ordinary event storage.
-Original History identity path: parser body+48 ->load temporary+8b8 ->canonical
  History+60. Native fallback basename hash ->(hash,metadata length)->(hash,
  modified time), returning the accumulated prefix on metadata/modified error.
  It has no current-clock call or second Identity parser. Four untouched native
  pair-hash outputs are in original-results.json for a canonical utility check.
-Selected snapshot wrapper firsti32 reaches canonical History+68; signed replay
  delta is target minus this selected snapshot cursor. No-snapshot branch writes
  -1. This is a history-chain index, not the array ordinal of the reference.

## Inferred safe integration

The current migration owner retains canonical typed ShapeParameters/IsEnclosed,
entry-owned decorator diffs and the same History ledger identity field. The
observed control/payload contracts agree with the reviewed code. Clear parser
snapshot refs only when the potential-mutation flag ran; Window/Arrow flags
alone are independent. Selected snapshot Window conversion subsequently clears
that loaded history's refs. Do not interchange those two invalidation sites.

The native probe is not a complete parser/App runtime oracle: allocator/serde/
filesystem input was not substituted with a miniature parser. It executes only
the actual business regions listed above; full original range extraction gives
control-flow coverage for review, not proof that all errors were executed.

## Still unknown / required complete-App regression

The migration writer alone does not close pre19 conversion. Current production
registers SpawnPreplacedTrees via world.init_resource, but the real conversion
consumer is absent from history_runtime/run_game registration; the OnEnter
startup asset routine is a different path. Its complete event-driven startup
and convert chain is now assigned to save_migration_full. Candidate has explicit
rock-mask/candidate-owner gaps and must not be registered as complete unchanged.

Original full JSON parser malformed-input errors, filesystem metadata failure
on each fallback stage, snapshot selection retry/duplicate-id handling, snapshot
future cursor with a negative replay delta, and full async save identity after
replay are not dynamically accepted by this review. Fresh empty-Glade identity
constructor was not re-established here; active loaded identity is independently
proven. Retain those boundaries rather than promoting local tests to full load.

Executable regression targets: native original_probe.py (Python313 with existing
Unicorn/pefile); canonical migration tests should compare its32 exact shape
payloads,65 per-record outputs and8 wrapper cases; utility hash tests can use its
four exact pair values. Full-App tests must cover v15/16/17/18/19/20/21/26/27,
valid/invalid/open-failed snapshots, target before/after snapshot, event maintenance,
preplaced tree typed prefab creation+delayed conversion, and save identity owner.

No production change, no Cargo, no T13 work.
