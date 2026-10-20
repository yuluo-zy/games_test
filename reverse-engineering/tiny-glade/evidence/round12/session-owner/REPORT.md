# Parsed session ownership handoff — round12

Production sources remain frozen. This worker changed only this evidence directory and ran no Cargo command. Review patch: `ownership-v27-review.patch`; seven full candidates are review material, **not files to copy over the shared tree**. `source-baseline.json` binds each hunk to its source baseline. Root owns integration and validation.

## Scope and result

The patch restores the actual Load UI parsed-owner route through NextSession3 → startup_session → LoadGameManager → the real loading consumer. It fixes the native parser's missing GladeSettings input and preserves the four separate wrapper flags. It provides an original-evidence-supported version27 route; original older-version migration and its metadata-dependent downstream consumers remain explicitly unfinished. This is not full Load UI, all-save compatibility, or a three-flow completion claim. T13 is excluded.

The canonical constructor for the UI worker is:

```rust
let data = parse_saved_game_data(&selected_history_or_backup_path, &selected_glade_settings)?;
commands.insert_resource(NextSession::from_parsed(selected_session_id, data));
```

The surrounding original UI transition writes native byte2, canonical **AppLoadingState::Reload**. No new Loading variant is justified. This patch does not modify the UI transition owner.

`NextSessionKind` keeps its existing Copy/Clone/Eq discriminator API, adding kind3. The actual owner is a private `Option<Box<ParsedGameData>>` in NextSession; the raw template words never contain an allocation address. The constructor sets empty on_load and ignore_snapshot=false. Cancellation/replacement uses ordinary Rust drops. No clone was inserted to make the consuming path compile.

startup_session case3 bypasses current-history readiness/fallback and its `is_file` check: successful backup parsing must remain usable when current history is corrupt or absent. The selected SessionId still determines metadata and the canonical snapshots directory. It prepares those before taking the owner; the final fallible take checks the invariant before publishing active state. The loader takes LoadGameManager.source once and moves out `*Box`, rather than cloning the history graph or reparsing current history.json. Failure takes the existing explicit Failed terminal route and drops the consumed owner; no retry/success claim is synthesized.

## Original evidence

`migration-handoff.json` binds the original RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`, parser procedure size14275B, bounded native migration range,30 callsites,14 version/flag instructions, and original default-color constants. Existing exact evidence was reused; no MCP overlay or new database was modified.

* Ordinary UI: `../render/load/load-ui.asm`141e20bd8 parses;141e20bdd tests return byte+f against2 (Err);141e20d5c copies the entire successful16B value to NextSession+30;141e20dc6..dd0 writes kind3;141e20e1a inserts. Backup and theme writers use the same complete owner. Their detailed locations are in `../render/load/HANDOFF.md`.
* startup consumer: `skeleton/task-index/work/TG-c459c7f73345e24709/main-chain-boot-20261018/0x14166e7d0-fresh.c`, case3 copies the two owner qwords; it builds the selected session path without reparsing that file. No raw-pointer payload or file-reopen substitute is justified.
* Parser settings input: native141470d75 saves r9 into r14;14147191a reads settings+14 building color,141471924 settings+18 roof,14147192a settings+10 freeform. Original fallback stairs100003 comes from vector constant142c63d10 `[0,0,100003,0]`; brightness1.0 from142925904; building_wall_style1 from141471944→141472e53. Remaining zero fields come from the original zeroed registers/stores. Both missing Colors and failed typed Colors use these settings-aware defaults.
* Parser success141472e59 allocates240B,141472e71 moves its complete body,141472e97 writes the pointer,141472ea3 writes the target chain_index, and141472ea7..eb3 writes four independent bytes. The low i32 is **chain_index, not version**: load_game subtracts current History.cursor at14146's counter publication. Version is retained in the allocated body and not lost into the wrapper flags.

## Wrapper flags and their consumers

| Native field | Producer | Consumer and status |
|---|---|---|
| +c / second-qword bit32 | source version!=27 OR actual compatibility rewrite flag at141472e76..e7d | Path source upgrade backup gate at load_game native pre14146ea49; skipped for new_session. Patch applies the correct known metadata gate and propagates backup errors. Older metadata remains unknown. |
| +d / bit40 | compatibility loop flag atstack2f0 →141472eab | **WINDOW** snapshot-color upgrade and, after snapshot initialization, clearing snapshot references. Existing LoadUpgradeFlags names had bits40/48 swapped; patch corrects the helpers. Original actual snapshot upgrade/invalidation still needs full migration integration. |
| +e / bit48 | compatibility loop flag atstack304 →141472eaf | **ARROW SLIT** snapshot-color upgrade. Mapping leaf and record applicability already exist, but the complete typed snapshot consumer must be connected by migration owner. |
| +f / bit56 | original source version<19 at141472f5e..f65 →141472eb3 | load_game's legacy notice event branch before closest-snapshot load. Exact canonical event identity remains to be established. Value2 is Err niche, not a successful true flag. |

Native version27 shortcut141471bfe..1c23 initializes migration flags0 and jumps around the old-version rewriting path. Final version==27 comparison makes+c false; version<19 is false. Consequently the patch can assign the explicit evidence-bound `ParsedLoadMetadata::VERSION_27` constant. It does **not** assign false metadata to older saves or infer all flags just from the version number.

For older saves `migration_metadata=None` preserves the explicit uncertainty. Newly prepared Parsed UI loads with unknown metadata fail with a concrete unrecovered-migration error instead of claiming compatibility. Existing Path's prior support scope stays in place with an explicit warning and its prior best-effort backup policy; this old incomplete implementation is not being reclassified as correct. This interim boundary must be replaced by the complete original migration, not made permanent or advertised as completed interaction support.

## Precise migration handoff

The next worker should start at native **141471d3c..14147319b (5215B)**, SHA256 `962ad824f93a9a91a9b3a103a215a8848dd60b7f5ea30271c49f8ede3ea23180`. The full function is14275B and includes cleanup beyond this migration core. Retained `../startup/parse-saved-body.asm` is the original full-range assembly; the final wrapper writer and additional pre19/pre17 branches occur around141472f5e..14147319b. `migration-handoff.json` lists direct callees and the precise version thresholds (16,17,19,20,21,27) and flag writes. ICF alias lists are not callee semantic proof.

Migration work must restore the actual edits/aux/maps, original warning/log/identity side effects, and all four flags together. It must then restore snapshot window/arrow color rewrites before domain application, snapshot-reference invalidation, and the canonical legacy notice event. Do not simply set `requires_upgrade_backup = version!=27` or fill unknown flags with false. Preserve the target chain_index independently of migrated History cursor.

The related leaf **derive_fallback_save_file_identity@141673140** is an additional concrete algorithm gap: the current source explicitly admits a modified-seconds XOR nanos approximation with native field order unknown. Parser141472bc1 and backup identity de-duplication both use the original leaf. Restore its real file metadata composition/hash and failure policy once, in the canonical owner; do not add a second UI approximation.

## Source/API audit and validation

The seven patch files:

1. country-core/src/resources/session.rs — kind3, private owning payload, constructor and checked move.
2. country-core/src/startup/startup_session.rs — exact kind3 selection and source transfer.
3. country-core/src/systems/save/load_game.rs — wrapper model, settings-aware parser, move consumer, pure helper moves, known backup policy and explicit remaining boundary.
4. country-core/src/resources/save_file.rs — correct native WINDOW bit40 / ARROW bit48 helpers.
5. country-core/src/systems/save/tests.rs — all parser/helper callsites and a real owner-allocation/metadata/no-file-reparse regression.
6. country-core/src/startup/tests.rs — existing pure-helper call updated for mutable ownership/settings.
7. tiny-glade/src/run_game.rs — **only one intro-test parser callsite** supplies explicit settings; integrate its hunk, never the full candidate file.

All-crate searches are retained in `all-crates-api-callsite-audit.txt` and `api-callsite-audit.txt`. The only exhaustive NextSessionKind match is updated. Existing MetaCommander constructor and template comparisons retain their discriminator API. No removed Copy/Clone/Eq requires copied()/cloned() substitutions. The added mutable pure-helper arguments are covered in both existing test modules. There are no other qualified calls to the pure load_game_session helper in the current crate sources.

`rustfmt --check` parsed the five initially generated candidates without syntax errors; it returned1 solely for formatting diffs (including pre-existing source formatting). Two later callsite-only candidates are included. No Cargo build, unit-test pass, original-process behavior, GPU, or full ownership-chain runtime validation is asserted. Root must compile and run meaningful startup/actual-App acceptance after hunk integration; the added test is staged, not passed.

One useful regression preserves the original edits Vec allocation pointer while moving from parsed data → request → pure consuming loader, carries all flags unchanged, points at a deliberately nonexistent file, and verifies that the same owner cannot be consumed twice. It checks ownership mechanics, not old-version migration or complete original gameplay behavior.
