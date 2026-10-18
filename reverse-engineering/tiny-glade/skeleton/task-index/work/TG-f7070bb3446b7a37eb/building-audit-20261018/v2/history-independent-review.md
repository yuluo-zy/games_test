# Independent review: history replay mode / finish / local adapter

Target: B, review date 2026-10-18. Read the original RNE disassembly for 0x140a7c550 and 0x1408b36a0, then current production replay_mode.rs, finish_replay.rs and history.rs. This is a review, not a production patch or execution result.

## Findings

1. **P1, existing unresolved owner gap: registered Pop drops auxiliary restoration.** process_history_cursor_commands attaches decorator/stairs diffs to ReplayBucketAux, but the actually registered pop_empty_replay_bucket destructures and drops _aux. The recovered replay_pop_bucket helper is not registered. Original B 0x140b38250 calls restore_decorators (0x14155ba30), and, when stairs aux exists, restore_backup / apply_diff / graph iteration / clears / make_backup; it then extracts the archivist diff and flushes the raycast world. Thus typed command replay can finish while decorator/stairs side effects remain absent. This is independent of the two newly recovered leaf control flows. Resolving it needs the actual storage/collider/archivist owners; replacing it with a no-op adapter would not close the business chain.

2. **P2, explicit data-order edges were missing and are now present in the inspected current production source.** History Cursor now follows InputSystemSet::Derive, History Record precedes InputSystemSet::Finish, and finish_replay follows pop_empty_replay_bucket. Input sets currently chain Prepare -> Control -> Clock -> Translate -> Resolve -> RecordReplay -> Derive -> Consume -> Finish. History sets chain Cursor -> Replay -> Apply -> Pop -> Record. Existing wall/garden ordering adds no path back to Derive or from Finish to Cursor; static source inspection found no cycle introduced by these edges. They are local adapter dependencies, not recovered global native scheduling.

3. **Remaining consumer boundaries:** CmdUnfreezeSheep is produced/registered but has no actual EventReader consumer in current workspace. ClearCanvasCmd is likewise produced with no actual EventReader consumer found. Original replay_mode directly clears UiState each replay frame; an event records the requested host operation but does not demonstrate that side effect. The current source now initializes Events<ClearCanvasCmd> in register_history_events (the earlier missing registration has been fixed). Do not describe either unfreeze or UI clearing as a completed end-to-end effect.

## Confirmed behavior

- MainActionSet phases are the existing manual design (JustPressed0, Held1, Repeating2, JustReleased3, Idle4). The two leaves' primary/secondary pressed/released predicates match that design and use slots16/17. No original memory layout is imposed.
- AppModeChanged and HistoryCommand are the real current mode-manager event types; independent EventReaders preserve native per-consumer cursor behavior. The original reader still scans events after a primary value >=3 before checking the secondary/natural-exit condition. EventReader::any is therefore correct and stops at the first matching second byte3.
- replay_mode restoration mapping, elapsed/speed/hold reset, override early return, >0.2 hold gate, fast replay command words and queue-based minimum speed agree with the original leaf control flow inspected.
- finish_replay's one-pending-byte0==1 unfreeze emission, interruption predicate, natural exit only when pending/replay empty and both actions released, full RestoreAppTo payload and toolbar choice, and final sentinel consumption agree with the original leaf control flow inspected.
- No extra reset of elapsed on leaving Replay is justified by the original leaf.
- The shared pending queue is persistent and separate from Bevy event transport. Events emitted after Cursor are available to the next Cursor invocation; claiming a specific native global frame order would require separate evidence. Event lifecycle updates remain the host's responsibility.

## Evidence and scope

Original leaf files:
- TG-3d5f906471eb59f319/systems-history-audit-20261018/140a7c550-rne.asm
- TG-3d5f906471eb59f319/systems-history-audit-20261018/1408b36a0-rne.asm
Auxiliary source evidence:
- evidence/country-core-deep/history/decomp/decomp-0x140b38250.txt
- evidence/building-integrate/B-roof-switch/140b38250.asm.txt

Source review and resource/producer-consumer searches only. No Rust compilation, tests, native run or full Bevy schedule execution was performed by this reviewer. Root owns integration and validation. No further blocking finding in the new replay_mode / finish_replay leaf implementations was established.

