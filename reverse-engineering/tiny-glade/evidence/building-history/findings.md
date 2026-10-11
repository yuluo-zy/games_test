# History transaction boundary

The implemented B boundary is `start_input_recording → add(Rim mutation) → finish_input_recording → cursor selection → original Rim replay event`. `reconstruction/history/transaction.rs` supplies normal Rust owners and explicit observed-ledger import. It intentionally has no startup `Default<History>`. `after_observed_future_clear` represents the proven negative-cursor reset, not original application startup. The parent Bevy integration must explicitly import or reset the ledger before recording.

## Original business policy

| Owner / original B entry | Recovered behavior |
| --- | --- |
| `History::start_input_recording`, `140a86270` | Reject Replay tag 13; open empty group; explicit start replaces prior pending group. |
| `try_start_input_recording`, `140a86510` | False during Replay or already recording; otherwise opens empty group. |
| `add`, `140a84ec0` | Requires a pending group. First edit, with cursor behind tail, invokes `nuke_future_states` before appending. Opening and cancelling an empty group preserves redo. |
| `nuke_future_states`, `140a84c50` | Nonnegative cursor retains chain through cursor, clamps unknown count at +58, retains snapshots whose index ≤ cursor in order. Negative cursor clears chain/snapshots and sets cursor/anchor -1/count zero. Fresh fastrand nonce at +60. |
| `finish_input_recording`, `140a86730` | Takes pending; None panics. Checks edits and three auxiliary counts. Empty group returns false. Nonempty group appends one 40-byte entry, refreshes nonce, increments cursor and asserts committed tail. Emits backend `InputReplayCommand` observed tag 1; not `HistorySaved`. |
| `history_process`, numeric region `140af605c` | Consumes cursor commands but ignores them while recording. Undo selects old cursor then decrements, stopping at anchor. Redo selects cursor+1 then increments; cursor below -1 panics. |
| `replay_edit`, `140af6170..140af62bb` | Expands edits forward using `14143fb50`, then reverses the entire generated 48-byte ReplayCmd array for undo. |
| `RoofReplayCmd::process`, `1421b48e0` | Rim RoofEdit tag 4 produces `SetFlatRoofRimStyle`: wall ID, selected from/to style, from=to and record_history=0. Replay does not recursively record. |
| `finish_potential_mutation`, `140a86980` | Active token clears resolved snapshots and refreshes nonce; inactive token changes nothing. Consumes active flag. Does not commit the recording. |
| `HistoryMutationToken` Drop, `14088f8b0` | Active forgotten token, when this thread is not unwinding, panics: “History was mutated, but wasn't told about it”. Never commits. |

The Rust recording API returns structured errors at original panic boundaries. It preserves already performed state transitions; it does not silently invent missing recording state. `cursor_step` is deliberately available only for `RoofRimHistoryMutation`: one Rim edit expands to one replay command. Reversing raw generic edits would be incorrect for owners that expand one edit to multiple commands. Nonzero auxiliary counts return `AuxiliaryReplayOwnerRequired` rather than dropping edits.

## Executed verification

`scripts/multi-version/verify_history_transaction.py` compares the actual workspace release `history-transaction-probe` with untouched B machine instructions. **462 scenario sequences pass** over ledger lengths 0–5, all valid cursor positions, two anchors and eleven action sequences. Sequences include None add/finish, empty cancellation, Replay start/try-start, replacement of pending group, ignored cursor commands during recording, two/three Rim groups, undo/redo in both instant modes, branch truncation and inactive/active mutation invalidation. Snapshot order, nonce calls, pending state, cursor, anchor, unknown count and replay event order/selected style are compared after every action.

The oracle actually calls native `replay_edit`, native expansion, native command-list reversal and native `RoofReplayCmd::process`. Expected Rim commands are not supplied by a hook. Entry addresses are read from the live native Vec pointer, including after its reset/reallocation. Three additional original-only token guard executions confirm inactive return, forgotten active panic and already-unwinding return, with the ledger unchanged. These three guard runs are not represented as Rust panic-unwinding differential coverage.

The allocator, Vec reserve, Arc aggregate destruction, RandomState hash seed and fastrand TLS calls are explicit bridges listed by address in `B-transaction-verification.json`. Game nonce consumption and game selection/filter predicates execute original instructions. The token Drop is not bridged. Native Bevy events use preallocated original queue layouts; full ECS scheduling is exercised by the parent Rust World tests, not by this instruction oracle. `instant=false` verifies flag/payload selection only; original timed replay scheduling remains unresolved.

The Ghidra `B-replay-expand/14143fb50` export contains only a jumptable dispatcher. It cannot support branch claims alone. The oracle maps the entire original PE and follows the real branch outside that export. B loader bytes are checked against the baseline SHA before execution. Candidate binary/source/Cargo provenance is recorded by actual runs; final shared-source changes require the parent to rerun.

## Three versions and dependency filtering

`version-comparison.json` binds original addresses and PDB game-source checksums for all three versions. `compare_history_versions.py` regenerates source ownership and reads C's exact static Replay guard. A uses 32-byte MultiEdit entries and cursor +50; B/C use 40-byte entries and cursor +68. A checks two emptiness counts; B/C check four. A inspected finish has no B/C nonce write. C's AppMode representation and Replay tag changed to byte tag **9**, from B dword tag **13**. C transaction policy is an instruction comparison, not execution acceptance; A Rim expansion has not been matched.

All four selected history source checksums differ in A/B/C. These are private `crates/country-core/src/resources/history/*` / `systems/history/*` business owners, so they remain core reconstruction targets. Repeated generic names or compiler-generated `core::ptr::drop_in_place` aliases do not turn game Drop policy into standard library policy. Conversely, Bevy EventWriter storage, Vec/Arc/HashMap, allocator and TLS RNG internals are excluded and reused through actual dependencies. The Rust nonce uses `fastrand::u64(..)`, not a guessed clock, zero or authored default.

## Applied reconstruction guidance

The native-rust-reconstruction dependency/module mapping rule changed the implementation boundary: original private history policy is kept, while known Cargo/stdlib container and RNG code is bridged or reused. Its instruction-verification rule changed the undo proof: the oracle now executes expansion/reversal/process rather than assuming reversing recovered input edits proves every replay owner. REA broader investigation step 6 provided the independent history investigation split while the parent restored holes/decorator consumers.

Remaining boundaries: original startup/snapshot importer; auxiliary decorator/stairs edits; full producer graph for active mutation tokens; complete VisualEdit/other expansion owners; original timed replay scheduler; C runtime transplant. None are replaced by a fabricated empty owner or reported as a complete history implementation.
