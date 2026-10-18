# B wall/history host registration contract

Read-only original registration audit, 2026-10-18. This is a handoff to the existing `main-chain-boot-20261018` host owner, not a production patch. It does not reinstall or replace the current hand-written modules. No Cargo, native execution, MCP mutation or whole-PDB scan occurred.

## Identity and method

`native-ranges.json` binds every retained PE span to B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. Matched PDB SHA256 is `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`; the existing B address-alias map selects exact module/record IDs and only those module records are read. The `.bin` files are direct PE reads and `.asm` files decode those bytes. The PDB span of `add_wall_systems` is 1413 bytes, including unwind code; its ordinary return/tail dispatch is within the previously retained 1131-byte main-body extract. This is a boundary clarification, not different native behavior.

`native-name-links.json` records actual RIP-relative UTF-8 function-name bytes, the instruction carrying their length, and the specialized helper. `typed-tuples.json` preserves each original tuple in order. `set-vtable-identity.json` proves the dynamically interned 1-byte sets are `country_core::systems::GameSystemSet`, using the vtable's `SystemSet::system_type<GameSystemSet>` method. Thus the labels below are not guesses based on integer literals alone.

## Positive wall registration evidence

Entry: `tiny_glade::app_systems::impl$0::add_wall_systems`, `0x140493120`, PDB module 186 record 72644. All listed update registrations reach `SubApp::add_systems<..., bevy_app::main_schedule::Update, ...>`; the exact Update helper records are retained. Numeric discriminants establish membership. This audit by itself does not prove the global ordering of all GameSystemSet values.

| Native site | Set | Registered consumer/configuration |
|---|---:|---|
| `0x14049314e/3150` | 17 | Arch UI-controls producer, typed `IntoScheduleConfigs::in_set` at `0x140112d30` |
| `0x14049316a/316c` | 42 | Mode-change archival/collision/stairs consumer, typed `in_set` at `0x140322d40` |
| `0x140493241` then `0x140493246..326e`, submitted `0x1404932c7` | 19 | The 18-entry edit chain and 16-entry height/publication chain, wrapped in an outer two-config `chain_ignore_deferred` |
| `0x140493304..3329` | 24 | Elevation-support construction tuple at `0x14030fb40` |
| `0x140493393..33b8` | 28 | Gate construction tuple at `0x14033def0` |
| `0x140493423/3425` | 36 | Rectangle-support update (`0x1403104a0`) |
| `0x14049343e..3463` | 34 | Adjust/deferred wall/clutter history recording tuple at `0x1400cbd40` |
| `0x1404934ce/34d0` | 44 | Instanced-wall GPU upload + shared SSBO update (`0x1400f50e0`) |
| `0x1404934e9..350e` | 32 | Plaster create/recalculate/delete chain (`0x140272c20`) |

The set19 edit tuple at `0x14050d290`, slots **zero based**, is:

0. wall_splitter;
1. the prebuilt configured eraser/bulldoze system (its complete original SystemParam/condition type is retained; this audit does not reconstruct its condition);
2. wall_eraser;
3. animate_wall_bulldoze;
4. **ui_draw_wall** (into-config helper `0x140307300`, call `0x14050d536`);
5. **suppress_continue_wall** (`0x1405b4430`, `0x14050d542`);
6. **ui_continue_wall** (`0x1405b45f0`, `0x14050d54e`);
7. snapping_init;
8. snapping_to_ends;
9. snapping_to_segments;
10. check_for_disable_snapping;
11. snapping_audio;
12. snapping_signifiers;
13. snapping_maintain;
14. **draw_wall** (`0x1403146f0`, `0x14050d5ab`);
15. **finish_wall** (`0x1404e4a70`, `0x14050d5b4`);
16. merge_walls;
17. telegraph_walls_are_merged.

The native UI/draw/finish functions therefore **do not have separate numeric sets**. The function names above are linked through real native `FunctionSystem` name constants, not just matching parameter shapes. The typed tuple contains exact canonical `RecordMouseInputIntoWall`, `FinishWall`, `CurveNursery`, `InProgressWallResource`, SnappingMode, ContinueWallOriginalState and color/audio identities.

The subsequent set19 tuple at `0x140335b30` starts with height signifier animation, ui_adjust_wall_height_mode, then adjust_wall_height_impl (slot2). Later slots include wall_highlight(5), publish_public_wall_state(6), delete_wall(7), spatial_hash_update(8), spatial_hash_debug(9), calc_wall_bend(10), and remaining wall-change consumers. The complete 16 signatures are retained even where this audit does not name an unrelated body. The outer helper `0x1400d5080` receives the edit config first, then this height/publication config; main-entry copies at `0x1404931b4..323d` preserve that order. Therefore the original set19 has **finish_wall before adjust_wall_height_impl**.

`chain_ignore_deferred` is the actual pinned Bevy API used here. Replacing it with `chain` would additionally introduce deferred Commands barriers; this audit does not authorize that alteration as an original-schedule claim.

## Positive history registration evidence

Entry `country_core::systems::history::impl$0::add_history_systems`, `0x1409438a0` (full PDB span1390 bytes):

* `0x1409438ce` sets DL=`14`, calls `0x141738630`. This helper constructs the exact name `country_core::systems::history::history_recording::history_start_recording`, with `ResMut<History>, Res<UiState>, Res<AppMode>`, and sets GameSystemSet14.
* `0x1409438dc` constructs the tuple through `0x141737fb0`; `0x1409438e1` writes set35, then `0x1409438fd/390f` intern+attach it. Its **six-entry `chain_ignore_deferred`** is exactly:
  1. history_finish_recording;
  2. history_process;
  3. history_clear_previous_state;
  4. replay_mode;
  5. finish_replay;
  6. signifier_replay_active.
* `history_finish_recording`'s native tuple includes canonical `EventReader<InstaCreateDeco>`, `EventReader<InstaCreateStairPoint>`, `EventReader<FinishInputRecordingCmd>`, the actual UI/history/storage/archive/stairs state and input command writer. Its full signature is in `typed-tuples.json`, not replaced by unit/lookalike events.
* The replay raster/wall/clutter tuple is labeled40 (`0x140943981/3983`); finish/replay cleanup with conditional timelapse work is labeled41 (`0x140943a09/3a3d`). Their complete implementation is outside this audit.
* All four produced configs are submitted together through `0x140b669c0` at `0x140943baa`, whose matched PDB name proves Update schedule. The set35 entry must not omit its leading finish_recording simply because the existing host previously registered only the following four consumers.

The set34 wall-history tuple has four signatures: history_record_adjust_wall_height; record the existing PendingWallHistory; attach late PendingWallHistory data with InnerWalls; and deferred autoclutter history. `typed-tuples.json` retains these distinct inputs. This audit does not treat every ledger method as an independent missing native engine.

## Current host / local helper boundary

The current `crates/tiny-glade/src/run_game.rs::install_history_and_wall_runtime` contains a subset of original set19 (adjust/publication/spatial hash), a subset of35 (history_process/clear/replay_mode/finish_replay), and replay_wall40. It does **not** install the new ordinary UI/draw/finish chain or complete history lifecycle. This is a real integration gap, not evidence those leaves are absent.

The tested `country_core::systems::wall::registration::add_wall_drawing_cpu_systems` is a current-architecture helper. Its real empty-registry first-use regression deliberately arranges UI creation → start_recording → draw/finish → record → finish_recording, with Sync consumed by the next Control phase. It additionally places current finish_wall after current adjust_wall_height. Those are current integration semantics; they are not the original numeric set19 registration proven above. In particular, attaching that whole helper to host set19 while keeping its explicit `ui_draw_wall.before(history_start_recording)` and placing history_start in the host's numerically linear set14 can create contradictory edges. The host owner must reconcile the present UI-registry/phase API and explicit dependencies as a concrete adapter, not paste the helper into a numeric label and call it original.

Required handoff checks: initialize canonical resources through the existing helper/actual upstream owners; register exact existing event identities once; preserve original consumer ordering where the current data model permits it; document any justified current-architecture order adaptation; include a real empty UI registry first-use test and next-frame Sync consumption in the actual host graph; ensure repeated registration does not duplicate function SystemTypeSet constraints. Do not create a parallel wall state/event engine, fabricate missing Snapping/archival/bulldoze consumers, or claim complete original drawing based on the bounded fresh/snap≤1 implementation.

## Verification boundary

All collected bytes are static B PE/PDB evidence. This handoff ran no executable native target, no new differential fixture and no Cargo. Earlier ordinary-wall-r1 and masks-r1 results remain their own bounded current-code regression facts, not validation of the original global schedule. Full native draw entry remains unvalidated after fetch-unmapped and the approval block; it was not retried or bypassed.
