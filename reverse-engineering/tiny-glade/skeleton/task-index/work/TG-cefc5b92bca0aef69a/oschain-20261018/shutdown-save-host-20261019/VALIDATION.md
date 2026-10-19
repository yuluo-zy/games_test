# Requested centralized validation

Author ran syntax parsing only; root owns all build/test/GPU operations.

After applying this four-source patch (including the separately authorized save_game accessor):

- country-core library filter `shutdown_save_host_tests`: five tests, includes actual deferred section I/O through the same worker loop and the no-active-file disconnect path.
- tiny-glade library filter `save_shutdown_preparation_tests`: five tests, covers pending/queued owner, validation error, update panic, warmup failure, cancel, discarded unstarted closure, send failure, failed handoff; asserts worker stopped before World drop.
- Existing `gameplay_preparation_shutdown_tests`: four tests should remain passing.
- Shared real host fixture: zero dimensions/no OS close must not update/render or exit; a real CloseRequested while zero must preserve zero dimensions and zero update/render count, run actual save join and GPU shutdown; nonzero Close must still run actual save43/PostUpdate shutdown marker path. This GPU/OS runtime test is still pending and not replaced by the CPU cases.

The isolated-file test supplies a real initial CurrentSaveFile to the exact shared worker loop instead of mutating process-global save-account paths; it is not a full original OpenFile/session acceptance. A healthy concrete file readback does not close the warn-only terminal-flush error observability gap.
