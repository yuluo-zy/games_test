# tg-history format/API completion

## Scope and target identity

This pass completes the crate's observed history-format API and documents where the crate ends. The target is the B Tiny Glade executable/RNE identity already used by the history instruction evidence: SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. Packaged JSON evidence was read-only; this pass did not start the game or claim runtime save/load coverage.

## Original evidence and findings

- `reconstruction/history/findings.md` and `observed-history-schema.json` bind the static sample audit to all 33 B `history.json` files. They record 2,120 edit-chain entries, 2,774 outer commands, 13 observed outer tags, 689 normalized field paths, and 31 nested tag paths.
- The observed outer command families are `WallNewSystem`, `Wall`, `Roof`, `DecoratorV5`, `Terrain`, `Tree`, `Color`, `Stairs`, `Path`, `Water`, `Garden`, `Cobblestone1`, and `Cobblestone2`. The tag set remains open; names and sample presence do not establish a closed enum or a migration mapping.
- Wall/WallNewSystem payloads contain ordered groups of operation enums and vary by document `Version`. Decorator diffs and stairs edge/hash values are entry sidecars, not additional entries in `edits`.
- Six sample documents have entries after `chain_index`; the document cursor is therefore not used here to truncate the stored array. B history consumers separately establish runtime cursor/undo behavior.
- The sample audit found 54 snapshot references: 32 matching same-directory files and 22 without such a file. The parser returns references and does not infer the missing-file policy.
- Roof commands expose four typed variants. `wall_index` is an observed serialized field; it is not treated as proof of the internal entity ABI.
- The original B history resource/system evidence is summarized in `evidence/building-history/findings.md` and `B-transaction-verification.json`. It covers bounded B transaction/control-flow policy; it does not turn this format crate into the original country-core resource or scheduler.
- A/B/C package format verification previously covered 98 histories and 733 roof command payloads (`evidence/multi-version/history-rust-verification.json`). This is format/field validation, not A/C runtime transaction acceptance.

## Implementation changes

In `tiny-glade/crates/tg-history/src/lib.rs`:

- Added `HistoryCommandKind` for all 13 observed outer tags with an `Unknown(String)` forward-compatible case.
- Added `commands()` for ordered, located commands while retaining payloads as raw JSON.
- Added `history_entries()` to expose each entry's commands and optional `decorators`/`stairs` sidecars.
- Added `snapshot_references()` with 64-bit IDs and `history_start()` as an unmodified JSON value.
- Extended summaries with unknown outer tags, entries after the serialized cursor, and snapshot-reference count.
- Preserved the existing typed Roof projection and raw document roundtrip behavior.

The parser intentionally does not type every family payload. Versioned wall, terrain, decorator, tree, path, and water payload internals remain raw until their original consumers establish field semantics. It also does not execute undo/redo, migrate versions, decode snapshot binaries, or resolve world IDs.

## Verification

- `cargo test -p tg-history --all-targets --offline --locked`: 5 passed.
- A temporary host-only Rust integration probe ran the completed API over the B original resource directory: 33 documents, 2,774 commands, 241 roof commands, and 54 snapshot references; all totals matched the retained sample audit. The probe was removed after the run and is not a package test dependency.
- `cargo clippy -p tg-history --all-targets --offline --locked -- -D warnings`: passed.
- `cargo fmt -p tg-history -- --check`: passed.
- `reconstruction/history/verify_observed_samples.py`: passed again for 33 files and 1,348 recorded source pointers. This validates the source sample audit, not Rust dynamic equivalence.

## Open boundaries

The crate now has a complete query path over every observed outer command and preserves all raw payloads. Full nested command typing remains an API/schema extension task. The separate game-history implementation still needs the original startup/snapshot importer, complete auxiliary command owners and scheduler/world integration; A/C transaction behavior remains distinct from B evidence.
