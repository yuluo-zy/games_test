# R3 GPU lifecycle

Status: observed private leaf semantics verified; disposal/submission lifecycle statically closed; candidate event oracle compiled and checked. GPU execution and complete renderer equivalence remain unknown.

## Identity and method

Original RNE/workspace PE SHA256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. Input plan: `../contract.json`. Both MCP query reports retain metadata, decompilation, references and successful `close_database(save:false)`. No function creation, annotation, database save, auto-analysis or dependency reconstruction occurred. PDB `void()` signatures are placeholders. Ghidra C is recovered pseudocode, never author source.

Correction: `0x140bd6a70` = RawManagedHandle track, `0x140bd6b00` = clone, `0x140be24b0` = drop. Clone was absent as a Ghidra function; original instructions were disassembled and executed without changing the database.

## Observed ownership and identity

Handle projection is 24 bytes: owner pointer +0, control/resource pointer +8, generation u32 +16, device ID u16 +20, kind u8 +22, untouched padding +23. Control prefix is max command sequence u64 +0, references u32 +8, generation u32 +12. Track compares generations first. Mismatch silently skips even an invalid command state. Match requires command tag 1 (recording), then atomic unsigned max(control.sequence, command.sequence). CommandBufferSeqTracker track requires the same recording tag without the generation check.

Clone atomically increments u32 references, copying only handle bytes 0..22. No overflow guard or generation check is observed. Drop rejects a moved/null control pointer; non-last drop decrements references and clears its handle pointer. Last drop appends a 16-byte control pointer/generation/device/kind record to the owner mutex Vec, then clears the handle pointer. It performs no GPU destruction and does not free the control block at this point.

ManagedResources take_out_dropped_waste swaps the global waste Vec out while locked, releases the lock, dispatches each resource kind and reads the control's tracked command sequence. dispose_cb_resource(ManagedBuffer) looks up that sequence in disposal buckets. Found: dispose closure targets the bucket, storing deferred resource data. Absent: dispose closure targets an immediate disposal bucket. This permits untracked resources or resources dropped after their last command bucket was removed to dispose immediately.

DisposableResourceSlot(ManagedBuffer) dispose (`0x1418af800`) validates device, kind 1, and exact generation before disposal. It calls the private resource/backend disposal path; then asserts references == 0, increments generation with u32 wrapping, and links the slot into the managed free list. A stale generation silently skips track but fails actual disposal. The reference assertion occurs after the backend disposal call, so the candidate preserves this event order.

## Observed command/queue chain

Command new initializes tag 0; begin accepts tags 0/3, calls external reset/begin, gets old fetch-add sequence from device sequence-counter slot 0 or auxiliary-index+1, sets tag 1, sequence, queue identity and inserts an empty disposal bucket. End requires tag 1, calls external end, and sets tag 2. Submission requires tag 2 and matching queue identity, sets tag 3, and records the command together with a separate per-queue timeline value. Command-buffer sequence is not the submission timeline value.

Queue update_pending_submissions (`0x140bf0b90`, supplied by R4) invokes the external get-semaphore-counter callback, stores its u64 result, and drops the pending submission timeline prefix <= that value. Queue reclaim_command_buffer_resources separately consumes pending command records whose timeline threshold is complete. It validates tag 3 and queue identity, calls SequentialDisposal dispose_in_order(command.sequence) and returns command-buffer resources to their pools.

Sequential disposal rejects sequence <= disposed prefix and duplicate out-of-order sequence markers. Later completed sequences are retained with zero gap slots. Only receiving prefix+1 drains that bucket and all consecutively ready later slots. Remove of an absent bucket fails; insert of a duplicate bucket fails. immediate_empty_bucket runs the external destroy/recycle operations for an extracted bucket.

wait_idle_and_recycle_resources: device idle callback -> take dropped waste -> update all queue completion counters -> reclaim all queues. wait_submit_done and timed wait: semaphore wait -> take dropped waste -> reclaim default/aux queues. queue_submit also updates pending counters before building submissions and extracts waste/reclaims afterward. Large Vulkan functions have static/event contracts only.

## R4 integration

R4 observed ExecutedRenderGraph retire has no GPU wait or token check. CPU resource pool return can happen immediately after execute. That CPU ownership transition does not remove raw managed slot protection. Last handle drop queues the control pointer; tracking has already recorded its last command sequence. Waste attaches to the existing last-use bucket, or disposes immediately when that bucket no longer exists. GPU completion drains command records and then the sequential bucket prefix; only that path performs deferred raw GPU destruction and slot generation increment/free-list return.

## Candidate and validation

Candidate: `reconstruction/render/work-packages/gpu-lifecycle-r3/`. Standard atomics/Vec/maps are directly reused; no public dependency implementation is recovered. Leaf ABI projections implement exact observed wrapping and generation behavior. EventOracle requires callers to provide CommandKey domain -> queue/timeline mapping. It refuses a single handle crossing unknown sequence domains instead of inventing a Vulkan synchronization policy.

leaf-validation.json: 29,692 untouched original entry cases (track 7,268; sequence tracker 15,405; clone 3,007; non-last/moved drop 4,012), zero mismatches. Panic entries observed without unwind. Last-reference append remains static-only. Six Rust tests pass, including CPU pool return before completion, two queues completing out of order with sequential reclaim, out-of-order CPU drops, clone delay, stale generation, empty submission, duplicate bucket, late drop and disposal generation failure/order. event-oracle.json is structured candidate output, not original GPU execution.

## Residuals and workspace incident

Unknown full domain counter initialization/global bucket key uniqueness, same-resource cross-domain mapping, complete typed disposal payloads, final-reference append runtime, allocator errors/concurrent races, shutdown paths and actual Vulkan execution. Empty submission timeline advancement is an oracle input, not inferred runtime behavior.

An early candidate was mistakenly written to production reconstruction/render/gpu_lifecycle.rs and lib.rs before relocation. The new production file was deleted and added module declaration removed. No pre-operation byte hash was captured; the four original lib.rs lines match the tool read before the edit, but byte-identical restoration is not proven. Restored hash: b863da8439a6d9050c0fa2d74306f4e55dab17a1364e30904f9da302e3c34c13. Temporary candidate code/binaries under work-packages/gpu-lifecycle were removed; final code is only in assigned gpu-lifecycle-r3. Shared memory/tracking and other agent artifacts were not edited.
