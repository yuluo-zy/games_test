# tg-history

`tg-history` parses a Tiny Glade save's history document and exposes its stored edit commands, entry sidecars, roof command fields, cursor metadata, and snapshot references. It is a format/schema crate. It does not execute edits against the game world.

The `tg-history` package is a reconstruction workspace split; it does not assert that the original game shipped a standalone Cargo crate with this package name. Original history resources and systems belong to `country-core`.

## What the original data shows

The B packaged sample audit covered 33 `assets/starting-builds/**/history.json` files: 2,120 edit-chain entries and 2,774 outer commands across 13 observed command tags. The sampled history `Version` values are 16, 17, 18, 20, and 27; this is a document field and must not be read as the Tiny Glade product release number.

The command families observed in those samples are `WallNewSystem`, `Wall`, `Roof`, `DecoratorV5`, `Terrain`, `Tree`, `Color`, `Stairs`, `Path`, `Water`, `Garden`, `Cobblestone1`, and `Cobblestone2`. `WallNewSystem` is present in the older sampled documents; `Wall` is present in the Version 27 documents. These names do not establish a migration mapping. The implementation classifies these outer tags while retaining each payload as JSON, including unknown future tags.

Entries can also carry `decorators` and `stairs` sidecars alongside `edits`. Decorator diffs are owner-scoped before/after maps; the stairs sidecar stores edge arrays and hashes. They are exposed separately because they are not outer edit commands. Wall operations themselves are grouped arrays, and observed old/new wall families have different operation sets and payload layouts. Their nested payloads stay raw until the original consumers establish a safe typed contract.

Roof commands have a narrower typed projection: Create/Delete presence, VisualEdit before/after fields, and ridge-direction changes. The projection preserves additional fields with `serde(flatten)`. Roof history uses `wall_index` in the observed samples; the crate does not claim that this proves the internal ECS entity encoding.

## Cursor and snapshots

`History.chain_index` is represented as the original signed 32-bit cursor. The parser also reports how many entries lie after that index; it does not discard them or call them the active state. Six sampled documents contain one to three entries after the cursor.

Snapshot references expose signed 32-bit `chain_index` and the 64-bit `snapshot_id`. Original B `History::closest_resolved_snapshot` filters runtime entries through `ResolvedHistoryChainSnapshot::try_from`, then chooses the nearest chain index to the cursor; equal-distance ties choose the later index. The `try_from` machine code accepts a snapshot only when its Arc strong count is at least two. The new `closest_resolved_snapshot` method mirrors the selection and tie rule, with a caller predicate because serialized JSON cannot reveal that runtime ownership state. `current_snapshot_reference` returns a reference only at the exact cursor index, matching the original getter. The parser still does not open snapshot files. In the B resource audit, 32 of 54 referenced snapshot files were present beside their history documents, while 22 references had no same-directory file; external loading and validation remain caller responsibilities.

`history_start` is returned as its original JSON value. Its loading and migration meaning remains outside this crate's proven contract.

## API and preservation rules

- `HistoryDocument::parse` validates the required document version, history cursor, edit-chain shape, singleton command tags, and snapshot-reference fields.
- `commands()` returns every outer command in stored order with entry/edit indices, a known-or-unknown tag classification, and the original JSON payload.
- `history_entries()` returns those commands together with the entry's raw value and optional decorator/stairs sidecars.
- `roof_commands()` decodes only the four observed roof variants. Unknown roof variants remain in `raw()` and are listed by `summary()`.
- `raw()` and `to_json()` retain unknown top-level data, command payloads, extensions, and large integer IDs. Typed `f32` roof projections do not replace the source JSON values.
- `snapshot_references()` reports references only; it does not imply that the corresponding binary assets exist. `closest_resolved_snapshot()` requires the caller to supply the original runtime-resolution predicate; it does not inspect assets or infer resolution from file presence.

Serde and serde_json are reused at the versions in the original B lock file. No serializer, ECS, allocator, or snapshot codec is reimplemented here.

## What is established elsewhere

The original country-core history resource and systems own recording, branch truncation, cursor-based undo/redo selection, replay expansion, and event consumption. A separate B transaction reconstruction has an instruction oracle for bounded recording/cursor scenarios, and Roof replay/recording adapters have their own evidence. Those features are not implementations inside `tg-history`; this crate does not claim full transaction or world replay support.

Three-version format checks cover 98 packaged histories and 733 roof commands. They validate parsing and roof fields, not A/C binary behavior or cross-version migration. No runtime save/load or scene replay is implied by successful decoding.

## Remaining reverse-engineering boundaries

- Full typed payload contracts for all 13 command families, especially grouped wall edits and version migration.
- Snapshot binary schema, asset loading/validation, and missing-reference policy. Nearest resolved-reference selection and exact-current-snapshot lookup are implemented from B instruction evidence.
- Startup history construction, complete auxiliary edit owners, and scheduler/world consumers.
- A/C runtime transaction and ABI acceptance; static layout comparisons do not establish runtime equivalence.

## Evidence

- [Observed sample schema and findings](../../../reverse-engineering/tiny-glade/reconstruction/history/findings.md)
- [Raw schema observation](../../../reverse-engineering/tiny-glade/reconstruction/history/observed-history-schema.json)
- [33-file sample verification](../../../reverse-engineering/tiny-glade/reconstruction/history/sample-verification.json)
- [Three-version format verification](../../../reverse-engineering/tiny-glade/evidence/multi-version/history-rust-verification.json)
- [Original history transaction findings](../../../reverse-engineering/tiny-glade/evidence/building-history/findings.md)
- [This implementation pass](../../../reverse-engineering/tiny-glade/evidence/tg-history-completion-2026-10-14/REPORT.md)

The B original executable SHA256 is `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`; A and C identities and limits are recorded in `evidence/building-history/version-comparison.json`.

## Validation

From the `tiny-glade` workspace root:

```powershell
cargo test -p tg-history --all-targets --offline --locked
cargo clippy -p tg-history --all-targets --offline --locked -- -D warnings
cargo fmt -p tg-history -- --check
```
