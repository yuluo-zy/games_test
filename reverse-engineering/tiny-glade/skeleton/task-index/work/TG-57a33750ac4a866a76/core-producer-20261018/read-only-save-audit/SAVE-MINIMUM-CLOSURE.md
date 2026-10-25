# Actual save/shutdown minimum closure — read-only handoff

This complements SNAPSHOT-PAYLOAD-CONTRACT.md. Preserve current hand-written architecture and existing actual worker/load/host numeric stages. Current-source-manifest.json is a point-in-time checkpoint; owners are concurrently editing. No production edit, new claim, compilation or original execution was performed.

## Direct original facts

The actual files140a3f470.asm(save17300B),141721aa0.asm(execute38878B),14171f7f0.asm(flush1934B) match direct B bytes/instructions throughout (existing-asm-verification.json). There is no located literal fullsave.asm. Complete PDB bounds are those retained in oschain host-edit-save-audit manifests; no summary is substituted for original code.

Save prologue binds EventReader<SaveGameCmd> in R8 and ShutdownGameCmd reader in R9; PDB SystemParam tuple binds SavingGameComplete writer and live sheep/domain queries. Both readers are read independently and exhausted @140a3fd42..fdf3; the later AND of both-empty flags gates the explicit save/flush/completion branch @140a4170e. Autosave snapshot/backup/metronome logic occurs independently before this gate, so run_if(on_event) over the whole function would drop normal autosave behavior. SaveGameCmd is an original ZST event, not a bundle of preassembled GameSaveData.

Explicit branch @140a419a9 creates a capacity1 Result<(),anyhow> reply channel, sends AsyncSaveCmd tag10 containing its sender, waits for the receiver, and emits completion only on success. Do not label constant0x3b9aca00 a1s timeout merely from its bytes: its surrounding receive/Instant enum has not been rederived here. Worker case10 calls CurrentSaveFile::flush_deferred_and_sync @14172347c and sends its Ok/Err through typed Result sender1408da1b0 @1417234cd. No-active case goes to14172718e→panic_fmt1417271ca, not warning-and-success. This is separate from tag11 shutdown's existing bool protocol; no new tag11 acknowledgement is proved.

After successful tag10 reply, caller invokes SessionId::save_folder_path @140a41b73. SavingGameComplete event instance is40 bytes: owned PathBuf32 + EventId8. Its payload is the session directory, not history.json and not unit. Failure must not emit a successful completion event.

Original save registration1409443b0 uses ActiveSession-exists condition and Update numeric set43; load remains its separately registered set12. Existing host inserts ShutdownGame in PostUpdate; game loop observes it after update and exits. The actual save43 must consume the same ShutdownGameCmd independently and finish its synchronous flush before that quit marker can stop the loop. Do not duplicate the existing load or worker startup by wholesale installation of the alternative local core helper.

## Current concrete gaps and minimum adapters

At checkpoint SaveGameCmd is a non-Event struct with data; SavingGameComplete is Event unit; AsyncSaveCmd::Flush=10 has no reply. save_game_system reads neither request, writes no completion, and flushes/metadata-writes each invocation. Its SaveGameSyncData has sections=None by default and no actual host writer; SnapshotCaptureState has an independent Vec/cursor and no producer synchronizing History. Merely registering these partial pieces does not capture persisted game state.

1. Restore canonical event schemas in their existing modules: SaveGameCmd Event ZST and SavingGameComplete owned PathBuf; current session refresh ignores event data and can reuse the new type. Restore tag10 reply in the existing enum and execute branch, using current crossbeam bounded(1)/Result pattern already used for ManualBackup. Retain no-active panic/failed-flush error and send/receive failure semantics based on original.
2. Within the existing save system, retain normal ActiveSession autosave behavior, read both request streams without short-circuit, and on explicit save-or-close assemble sections from real current History, MainCamera/cache, TOD, Colors, live Sheep query and actual typed edit ledger. Reuse assemble_game_save_sections and SerializedJson; do not fabricate missing domains or inject a successful default mirror. Snapshot references come from canonical shared pending-ID ledger, never SnapshotCaptureState copies or cursor IDs.
3. The legal host owner registers save43 only when ActiveSession exists and required real resources are ready, before initiate_game_shutdown. Keep existing worker creation and native load12. Add the actual supported user-save/commander producer through its original owner; event registration alone does not make Save reachable. Close already has a real ShutdownGameCmd producer.
4. Only after actual worker flush Ok emit completion with save_folder_path. Metadata, deferred edits/sections and errors must follow the verified original branches. Unsupported snapshot streams/domain application must remain explicit errors; save capture should not certify missing payloads.

Required validation is current typed edit→committed History→save/close request→actual worker→flush reply→completion path→disk reopen, with both requests consumed, no-completion on error, and close marker after flush. Narrow event-only or manually injected sections tests do not establish that closure. Native GPU/OS session switching and all snapshot domains are separate boundaries.

## Legal ownership

claims-at-review.json captures current owners. Save_game and save_thread are oschain-20261018; history/mod, load_game, snapshot and save/mod remain main-chain-boot-20261018. Root does not modify these claimed production files. Shared host/shutdown/event registration is integration-owner work, not permission to overwrite those source families. Session resource TG-a4422b4c9680d489d4 had no claim in this checkpoint; no new claim was made here. Full save/input implementation proceeds with the actual os_chain worker, as main-chain coordination directed.
