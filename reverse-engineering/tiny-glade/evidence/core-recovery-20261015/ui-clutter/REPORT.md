# Automatic clutter deletion/history subset, 2026-10-15

Implemented: one live `DeletedAutoClutter(HashSet<u64>)`, resolved-ID deletion command and real `RecordAutoClutterHistory` producer/consumer, shared `EditType::AutoClutter` tag 13 and Arc payload, native tag-9 replay construction, front mixed-bucket replay and make-instant integration. The existing History transaction ledger is reused.

Original evidence:

- `record.asm`, B 0x1409a9160: consumes every unread ID and adds an Arc-owned history edit; it does not start a recording session.
- `expand.asm`, B 0x14143fc1e: duration 0.1, timed=!instant, elapsed=0, undo byte, command tag 9. Jump table at 0x142c5f284 selects this branch for history tag 13.
- `replay.asm`, B 0x140aa4a60: one table of 8-byte IDs, apply insert/remove when elapsed==0; timed completion is old elapsed > maxss(duration/speed,0.05), then elapsed+=dt; untimed completion does not add dt. Minimum bytes at 0x142930d70 are cdcc4c3d in the original PE raw-backed rdata section.
- Original HashSet default is PDB ui_clutter/mod.rs:20. Old drafts' wall/pair deletion sets inferred from folded generic aliases were rejected.

Central validation reported by parent: both `autoclutter_` tests passed, including actual ECS delete→commit→undo→redo, strict time boundary, repeated zero-dt application, NaN duration clamp, instant behavior, and front-bucket isolation. Existing shared history tests also passed. Parent independently reviewed the shared ledger/expand/register connection and replay flow. These are local checks, not whole-domain completion.

Claude timing assistance completed using inherited deepseek-flash configuration. The timing/sentinel explanation matched the instructions. Its claim that negative infinity is left unclamped was rejected: MAXSS selects 0.05. Its suggested dt*speed equivalence also misses the fixed minimum-duration bound; implementation uses the original division and unscaled dt.

## Integration boundaries

- `DeleteAutoClutterCmd` is an explicitly resolved-ID adapter requiring an already-open real recording session, with visible HistoryRecordingFailed on missing session. It is not the original RMB handler. The caller finishes that session after record consumption.
- Full original RMB picking, parent/roof transform resolution, auto-to-decorator/stair conversion and UI interaction remain unimplemented. No fake conversion event or placeholder entity is emitted.
- The generation/mesh owner that consumes deletion IDs to suppress rendered clutter is not yet restored. This change applies the authoritative deletion/history state, and does not claim visible scene removal.
- Registration supplies a runnable local edit/record/replay order, not the original complete engine schedule.

Shared files changed: `resources/history/mod.rs`, `resources/history/replay_cmd_buffer.rs`, `systems/history.rs`; new canonical-facing `resources/history/auto_clutter_edit.rs` and `systems/history/replay_auto_clutter.rs`. Existing `recovered_systems/clutter.rs` remains the sole serde edit definition, with its stale header corrected.
