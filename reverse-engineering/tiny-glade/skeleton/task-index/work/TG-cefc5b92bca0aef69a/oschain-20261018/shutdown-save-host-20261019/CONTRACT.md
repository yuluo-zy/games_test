# Save-thread shutdown and real OS close contract (proposal)

## Scope and identities

Base: production game_loop.rs cb10929a53efe31129bb31b75928c53912aa5900723879cc02676665b45a293c; run_game.rs 5ce22c7f1d0878113f3e1cdf9e4cce879eba50008b571f435ec4fba0d2091d7c. Original evidence is B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03, not C equivalence. Reuse oschain-20261018/141720680.asm (full 5,139-byte save_thread) and original main_loop.asm zero-size/frame branches.

Observed B/current facts:
- The native outer receive niche11 carries Empty/Disconnected, not a save command. Disconnected forces deferred sections and sync, then returns. Current save_thread implements this; final failure is warn-only and does not publish an error reply. Do not send tag10 when there may be no active file.
- Current production App assembly creates SaveThreadHandle immediately, attaches its one cloned sender to AutoSaveState, and retains the same unsaved_files Arc. No other production command-sender clone was found. Deferred init_save_thread is a separate malformed/partially applied path and cannot be called joined without a handle.
- Normal nonzero ShutdownGameCmd is already consumed by save43 for real synchronous saving before PostUpdate produces the marker. This candidate does not replace that path or invent a new completion event.
- Current preparation drops error/cancel Worlds inside the worker; pending, queued, send-failed and spawn-failed closure Worlds also require cleanup before their App is destroyed.

## Host helper contract

`save_thread::shutdown_save_thread(world: &mut World) -> anyhow::Result<()>` is allowed only when this World's schedule is stopped and will never resume.

1. Read whether the actual AutoSaveState sender is attached. Call a narrow `AutoSaveState::detach_for_shutdown` accessor which drops that sender and moves out the actual prior anyhow error. Keep AutoSaveState and its original unsaved_files Arc in the same World; do not reset sections, file/session identity or unsaved history.
2. Remove the actual SaveThreadHandle; drop its sender after AutoSaveState's sender. Then join its actual JoinHandle. A prior auto error never prevents disconnect/join. No marker, ack, Flush/CloseFile command or substitute worker is sent.
3. After join, drain this handle's real AutosaveErrorState channel and check the retained unsaved_files list. A nonempty unsaved list is a currently observable failure. Report prior auto error first; preserve/log thread panic and actual worker-reported I/O errors as subsequent failures. Attached AutoSaveState with no Handle returns an explicit error; it cannot report joined. No Auto/Handle is a valid loading App/no-worker no-op.
4. Ok means any owned save worker was joined and no currently observable error was found. It does NOT mean durable save completion: the original disconnected final flush is warn-only and remains unobserved by this API. A second call with no attached sender/owned handle is a no-op, not a new join claim; the first error remains owned by the exiting host Result.

No new worker I/O result channel or terminal result is proposed in this batch. If a future durability observation is desired, it must be a separately reviewed host API wired to the actual final I/O result.

## Lifetime coverage and error order

- Current active App: stop/join preparation; shutdown its current World's save worker; then GPU shutdown. Every cleanup is attempted; first failure is returned, later cleanup failures are logged with full chains.
- Worker error, cancellation, update panic, warmup failure: keep the exact SubApp outside catch_unwind, borrow it during update/warmup, and invoke save cleanup before it is dropped. A local ownership guard covers closure spawn failure and any otherwise early World drop. It owns the same SubApp and does not run a parallel App.
- Worker success: transfer the same SubApp through the existing channel. If receiver has disappeared, recover the send error payload and cleanup that World before drop.
- Pending and queued success on cancellation: join preparation first, explicitly save-clean both possible Worlds before drop. If handoff resource transfer or preparation join fails after receiving a success, clean that local World before returning the original failure. The old loading App is also save-cleaned before replacement (normally no save owner).
- Typed cancellation is normal only if save cleanup succeeded. A save cleanup failure must be reported even when the primary worker result was cancellation. Load/warmup/panic remains primary over cleanup failures.

## Zero-size Close adaptation

Preserve the original either-zero WindowSize gate: no simulation, no App.update, no rendering, no fabricated nonzero size. Only an actual queued OsWindowEvent::CloseRequested in this captured batch may request host exit. Dispatch still emits the canonical ShutdownGameCmd into the current App. If final size is zero and that real close was present, return false to the OS owner and perform the cleanup above. Do not insert ShutdownGame or LoadingGameComplete. Nonzero close still runs the normal full Update/save43/PostUpdate marker path. This zero-size real-close escape is explicitly a current main-thread host adaptation, not an original native CFG assertion.

## Minimal verification to submit to root (not run by author)

1. A real save worker with a real temporary OpenFile + deferred ChangeSection writes that section during disconnect, joins, and retains unsaved Arc identity. Read the actual resulting file, not an ack or task count. Also test startup/no-active-file shutdown without tag10 panic.
2. A bounded test worker confirms both actual senders disconnected before join; cover already queued error diff, preexisting auto error, worker panic, and attached-auto/no-handle explicit failure. No failure can skip join.
3. Preparation current/pending/queued/error/cancel/panic/send-failure ownership tests assert save worker termination before WorldDropFlag. No GPU substitute assertions.
4. Original close decision: nonzero+close still one full update, zero+no-close stays alive and updates zero times, zero+real-close exits with updates/render zero. Existing captured-batch FIFO/cursor confirmation remains unchanged.

Performance: changes only run at App disposal/handoff/shutdown, plus one boolean per captured OS batch. No new per-frame allocation, lock or GPU wait. Joining waits for actual thread termination and may wait on the real file operations/channel queue; no I/O deadline is promised and no timeout is invented. Execution and durability conclusions remain unverified until root runs the tests.

Design review: 07 passed_scoped_design before implementation, with nonempty unsaved_files failure and no finite I/O-deadline claims added. Handle without canonical AutoSaveState is joined but reports unknown unsaved ownership as an error.
