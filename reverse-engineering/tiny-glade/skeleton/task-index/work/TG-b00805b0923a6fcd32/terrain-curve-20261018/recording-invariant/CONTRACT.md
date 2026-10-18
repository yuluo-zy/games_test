# Deferred terrain recording invariant

Current history_record_terrain consumes the typed pending edit before calling the shared History::add API, matching the original move order. Ignoring its Result, however, silently loses an edit when recording is missing. Normal lifecycle recovery does not justify hiding this invariant violation.

Original B History::add entry 0x140a84ec0 reads the pending niche at +0x30 and 0x140a84ef7 branches directly to 0x140a8525b when it is 0x8000000000000000 (no pending group). That block calls actual core::panicking::panic_fmt at 0x140a852a0. The independent original instruction probe executes this branch and stops at that actual panic entry, with native return address0x140a852a5. No formatting/runtime panic implementation is reproduced. The shared native harness binds the original B SHA.

The candidate keeps the current History Result API and adds expect at this recorder. It adds no error event or shared registration; the message is current host context rather than a claim to reproduce the original formatter text. It retains the original take-before-add and emits DebugHistoryMsg only after successful addition. A real World/Schedule regression verifies the missing-recording state fails rather than silently completing. The existing actual typed release/deferred regression already covers the successful path and pending clear, so no duplicate success test is introduced.

Root owns the new recording lifecycle, compilation and production integration. No Cargo or production edit was performed by this worker, and all older frozen typed-history input remains unchanged. This is a narrow invariant fix, not a reconstruction of the shared History module.
