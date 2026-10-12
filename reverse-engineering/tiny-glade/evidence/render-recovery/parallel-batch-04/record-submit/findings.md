# R6B original record/submit event oracle

Target SHA256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

## Observed original instructions

The full sequential `record_batch` entry `0x1402c1d40` and its private pass-recording helper `0x1402c3240` execute under Unicorn. Resource-access lists are empty; allocator, command creation/begin/end, timing, and user callback boundaries are fixture captures. No function is substituted for either original private entry.

- A new command is created/begun even for a zero-pass batch. Each pass callback payload at pass `+0x28` is taken (zeroed) before invoking the vtable call at `+0x18`.
- The callback success tag is 11. A failure stops before appending the newly recorded command to the outer command vector. Already consumed callbacks remain consumed; the existing command remains in the vector with its original state.
- On success, the new command is appended. The pre-existing command and all new commands except the last are ended; the last remains open. The final-batch gate starts export debug/scope work but still does not end the last command.
- `submit_pending_cbs` `0x140bebdf0` takes/reset its command vector to `(capacity=0, pointer=8, length=0)`, takes the wait field and retirement payload before validating the last command. Empty command plus a nonzero wait field panics. Empty command and no wait performs cleanup without queue submission.
- A nonempty submission requires last-command state 1. Only that last command is ended. An end error prevents queue delegation and leaves its state 1 in the taken command storage. End success sets its state 2 and delegates the complete command vector to `queue_submit`; the returned 16-byte receipt is stored at executor `+0x108`.
- Original `queue_submit` tail `0x140c06151..0x140c0634f` acquires/releases queue locks around the indirect backend submission, invokes the private error-conversion boundary on a nonzero backend result, and writes a receipt only after conversion succeeds and post-submit boundaries run. Its source submit array/frame is injected, not built by the original prefix.

## Evidence strength

108 exact native/Rust event/state comparisons pass: 28 record entries, 48 submit entries, 20 injected queue tails, and 12 original-record-to-original-submit transfers. The transfers preserve the 176 bytes of two original-recorded command objects; they do not prove a full queue-entry differential.

The earlier R5 initial/pre/export decisions are not replaced or reinterpreted. R6 records empty resource lists and therefore leaves nonempty resource/record integration open. Public Rust/std/ash/vk-sync dependencies are reused conceptually as boundaries, not reimplemented. No real Vulkan or GPU operation runs.
