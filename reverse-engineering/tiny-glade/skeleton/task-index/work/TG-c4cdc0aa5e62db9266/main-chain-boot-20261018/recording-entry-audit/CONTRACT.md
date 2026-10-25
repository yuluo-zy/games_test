# B History recording entry boundary

Original oracle: release RNE SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`; no Ghidra decompile used for this conclusion. Full `history-process-rne.asm` and PDB signature are in the parent work directory.

## Observed owner and iterator

- Original `History::is_recording@140a86970`, all 10 bytes: `xor eax,eax; cmp rax,[rcx+30]; setno al; ret`. This is the same predicate as `history_process@140af5cf1..5cf7`: zero minus the pending-record niche, then JNO. `History::finish_input_recording@140a86750..765` takes this exact owner and writes the None sentinel `0x8000000000000000` back to +30. `start_input_recording@140a8639e..3de` writes the pending group and its two auxiliary Arcs at +30/+48/+50. All four complete producer/consumer procedures are retained here.
- PDB arg4 is EventReader<HistoryCmd>. At 140af5b65 `[r9]` is its cursor; @5bd0 the cursor is saved to rsp+b0. Iteration consumes each current event by incrementing that cursor @5ce9 BEFORE testing recording @5cf3.
- If recording is active, @5ea8 takes an optional Warn log branch. It then goes directly to @5f9c return, both when logging is disabled (@5eb6) and enabled (@5f7f). It does not return to the event loop. Only the first current event is consumed; later events remain unread until their ordinary event-buffer lifetime expires.
- Warning string table @142ae5290 contains pointer142ae5250/length59: `Can't process History while recording a new edit is active!`; log target @142ae52b8 is `country_core::systems::history::history_process`.
- Recording return occurs before command enqueue/clear/instant, all three output events and @5f81 replay/pending selection. No new event means iteration exits to @5f81 directly: there is NO recording check in the existing pending-command path @5f81..6134.
- Arg5 DebugHistoryMsg is named by the complete PDB signature, but its stack slot is not read anywhere in this complete native body. Captured 16B EventInstance +8/+9/+b are this single HistoryCmd transport, not a second original stream.

## Current implementation

`history_process_system<S>` now owns one typed entry-event iterator and the queue selection in one ECS function. Both this system and the existing isolated cursor adapter call the same `advance_history_cursor` body. `cursor_step_expanding` keeps its public signature and removes the invented recording gate; no alternate cursor algorithm was added. `run_game` set35 and `add_history_cpu_systems` use the unified system. Existing lower transport adapters remain callable for captured-data/isolated tests but are not automatically installed in production. The original unused DebugHistoryMsg parameter remains unread.

Bevy 0.16 event iteration was read directly in the locked local source: `EventIteratorWithId::new` sets the baseline to event_count minus unread, `next` advances last_event_count exactly once, and there is no Drop advancing the remainder. Returning from the actual loop preserves the original consumption boundary.

## Verification boundary

Four production/test files applied; rustfmt only. Parent owns Cargo and actual GPU execution. Two new tests cover (a) first event consumed, second left unread and old pending not popped; recording completion then processes only the second event, (b) no new event allows a queued replay despite recording. Existing quiet/new-replay and clear/audio/interrupt tests now use the same runtime function. Their execution is pending, not claimed passed. Reviewable exact diff and before/after SHA are `applied.patch` / `manifest.json`.

Later actual validation: parent v28 ran the complete systems::history::tests filter, 14 passed / 0 failed, including both recording boundary fixtures. See original-core-v28-11.log in the root report. The separate Tiny Glade app/intro and GPU integration remain pending.
