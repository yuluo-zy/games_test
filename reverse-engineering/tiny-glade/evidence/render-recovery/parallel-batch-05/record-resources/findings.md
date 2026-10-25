# R7 nonempty record/resources joint path

Target: original `tiny-glade.exe`, SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

## Observed original instructions

The full sequential `record_batch` entry `0x1402c1d40`, private helper `0x1402c3240`, pre-barrier entry `0x1402c3d00`, and managed tracking helper `0x140bd6a70` execute under Unicorn. No candidate replaces these entries. Original `resource`/kind resolution, concurrency and last-access helpers also execute. Original `submit_pending_cbs` receives the exact command object bytes produced by original recording in the chain cases.

- Pending length is reset before initial validation. Initial transitions accumulate against the pre-existing command (fixture handle 99), emit once after the loop, then reset. A later invalid resource preserves earlier transition events and stops before command creation.
- The helper creates/begins a fresh command (fixture handle 100), then resolves each pass's resource references and runs original managed tracking before calculating pre-pass barriers. Tracking updates the resource sequence from 2 to the injected command sequence 7, and silently skips a stale-generation handle; subsequent non-concurrent barrier validation can still fail.
- A no-predecessor, non-reused access has no pre-barrier because initial handling owns its transition. Reused accesses skip initial handling, require zero initial access in their first pre-barrier, and emit zero to current. This reused marker remains a synthetic consumer fixture; producer reachability is not established.
- Equal access codes skip pre-barriers for read/read, or for writes in the same nonzero group. Different codes and ungrouped writes produce the transition. Each pass emits its nonempty scope and resets it before taking/invoking its callback.
- Callback payload is consumed before invocation. Callback failure or absence prevents appending the new command and ending the old command, while preceding tracker/barrier events remain. No separate post-pass barrier call or scope exists in this sequential helper; the subsequent pass's pre-barrier or final export follows successful callbacks.
- Successful recording appends the fresh command, ends the old command, leaves the fresh last command open, then performs final-batch exports against the fresh command. Export uses the registry last access; with no pass accesses it falls back to initial access. Image export aspect uses declaration format 91; initial/pre image aspect uses managed format 37 (the actual mapping is a fixture).
- Actual original call arguments observed at command begin and queue-pool selection are tag 0. The queue-family index returned by the selector is separately a fixture 0. The injected command sequence 7 is deliberately unrelated to initialization evidence and is not a queue timeline claim.
- Submit takes/reset its executor command vector, ends only the last command, then delegates the whole vector to the queue boundary. An end failure keeps that command open in the taken storage and produces no receipt. The public Rust projection preserves empty executor state and separately exposes taken command states.

## Delivery and limits

The independent Rust crate composes the previously validated private pre-barrier rules from `barrier-record-r5`; it does not modify that crate or reactivate/upgrade original dependencies. New recording control flow, callback ownership, tracker ordering, initial/export scopes, and submit transfer state are in `record-resources-r7/src/lib.rs`. Command constructor results and queue receipts are explicit injected inputs, rather than library/device reimplementations.

The oracle covers 1/2/4 logical resources, alternating image/buffer kinds, 0/1/2/4 passes, read/write and group variants, reused consumers, initial and per-pass invalid device/kind/generation cases, callback failure/absence, final exports, and submit end errors. All new cases contain logical resources; zero-pass cases specifically validate nonempty exports without a last access. The prior 108 empty-resource cases are not counted again. Exact totals, candidate/native hashes and checks are in `validation.json`.

No new Ghidra/MCP session was required: existing original pseudocode, assembly and machine-code oracle were sufficient. This avoided opening the shared database while the pipeline agent owned it. The existing PE hash gate and native harness are reused; no analysis result is saved over prior evidence.

Parallel recording, full queue submission construction, original device-created sequence values, live Vulkan/GPU, natural reused production and unwinding remain unknown. Synchronization/device/profiling/allocator/queue boundaries remain fixtures. This is source/CPU behavior recovery, not GPU or complete rhapsody acceptance.
