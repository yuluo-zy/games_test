# Save recovery — B, 2026-10-15

## Implemented and reviewed

- `ShadowFile`: original in-memory shadow plus dirty 4096-byte blocks, adjacent-run writes, truncate, retry after opening failure, sync and same-directory random temporary-file rewrite/persist. `+0x78` is an optional I/O error, not a flushed-block counter. Original create `0x1409184d0`, flush `0x1409193d0`, sync `0x14091a230`.
- Save JSON envelope, all ten section names/ranges/padding, history edit pre-separator offsets; in-place section resize shifts trailing ranges/offsets, edit replacement restores footer and clips the dirty bitmap when truncating. Original `0x141750c90`, `0x141751c90`, `0x14171ece0`, ReplaceEdits machine code `0x14172667c..0x141726b96`.
- Backup metronome: original intervals 300/600/3600/7200, budget clamp 0..8, exact counter advancement and hierarchical replacement protection. Negative/NaN time asserts; NaN budget is preserved. Largest tier can replace counter zero. Original `0x140a3fbf0`, `metronome-a.asm`, `metronome-b.asm`; independently reviewed by parent against instructions. Claude assisted only with initial decompiler hypotheses.
- `SaveBackupList`: actual filesystem listing, regular `.json`/`.bak` filtering, modification times, native prefix classification, maximum auto counter plus one. Original `0x140ae8330`, `0x140ae8470`, filter `0x14166df80` (local assembly saved).
- `snapshot_gc`: selective `History.snapshots` reader using serde, original parser-error suppression and first-match capture; read each locked backup, send its path to release channel, abort before deleting snapshots if receiver disconnects. Original invalid numeric `.snapshot` stems reach deletion; other extensions are retained. CurrentSaveFile owns the single locked-backup set and release receiver, with real start/poll consumers. Original `0x141286730`, `0x1413e4830`, `0x1413e4990`, `0x1414186d0`, `0x14142ce00`, worker `0x14172b280`, main loop `0x141720680`.

## Verification

Short instruction checks for GC:

- Signed 32-bit chain indices: `history-index.asm` (original `History::current_snapshot`) reads snapshot index at `+8`, compares with `cmpl` at `0x140a84691`, and uses signed `cmovlq` at `0x140a84695`. Existing `tg-history-continuation-2026-10-14/tryfrom-original-rne.asm` reads the same field with `movl` at `0x140a84263`. This agrees with the established `HistorySnapshotReference.chain_index: i32`; GC's 16-byte JSON-reference entries retain this type.
- Invalid numeric filenames: `14172b280.asm` branches directly to `0x14172bb94` for empty stem (`0x14172ba3f`), lone plus/minus (`0x14172ba51/60`), non-digit (`0x14172baa1`, `0x14172bad6`), multiplication overflow (`0x14172baaa`) or addition carry (`0x14172babd`). That block logs, then calls `remove_file` at `0x14172bcb3`; only successful numeric lookup can branch to retain at `0x14172ba00`.
- First-match capture: outer `0x14142d13d..144` writes captured Vec then jumps to end-object processing at `0x14142d170`, not the scan loop. Inner `0x141418a14..21` does the same. Wrappers return `{0,8,0}` only when the capture remains the MIN sentinel; otherwise they retain the Vec after dropping parser errors. Claude's analysis confirmed error destruction but incorrectly described the default as unconditional; that claim was discarded in favor of the wrapper conditional and instructions.

Parent central fixture `systems::save::tests::create_grow_replace_shrink_retry_and_sync_actual_shadow_file` passed **1/1**, exit 0, 2026-10-15 (session 66681). It covers real temporary-file creation failure/retry, section grow/shrink, edit replacement, read-back JSON, dirty-block truncation, and metronome. Subsequent `backup_scan_and_snapshot_gc_release_cancel_and_delete_actual_files` also passed **1/1**, exit 0 (1.3s): real backup filtering/counter, partial selective parsing, lock release, referenced-file retention, invalid-name deletion and disconnected-channel cancellation. Both only use test-owned temporary directories. These tests complement the original instruction evidence above, and do not establish full save-domain completion.

## Remaining — this is not completion of the save domain

- Complete AsyncSaveCmd variants, deferred commands, main background save loop/unsaved-file retry reporting and scheduling are not implemented.
- Game-thread capture/serialization of every private domain and snapshot binary capture/load are not connected. The codec requires callers to supply all real serialized section values.
- Automatic/manual backup production, original fallback to shadow memory on sync failure, retention deletion queue and its interaction with locks remain to be connected.
- Session switching/opening does not yet automatically construct/run the recovered GC slice. No global schedule or full save-session lifecycle is claimed.
- Direct ReVa was unavailable in this Windows tool session (empty project); existing persistent decompilation and original RNE instructions were reused. The original `.data` virtual tail is treated as loader-zeroed storage, never as later raw file bytes.
