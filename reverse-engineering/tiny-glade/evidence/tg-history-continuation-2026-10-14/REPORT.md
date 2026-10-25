# `tg-history` continuation: snapshot cursor selection

Target identity: original B `tiny-glade.rne`, SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

## Original behavior recovered

- `ResolvedHistoryChainSnapshot::try_from` at `0x140a84260` reads the snapshot's chain index at `+0x08`, follows its Arc pointer, and checks the Arc strong count at inner `+0x10`. It returns `None` below two references and returns the resolved pointer plus chain index at count two or greater. See `tryfrom-original-rne.asm` for the direct instruction listing from the original RNE.
- `History::closest_resolved_snapshot` at `0x140a84a50` first applies that conversion as a `filter_map`, lower-bounds by chain index, and compares the adjacent candidates' absolute distance from the cursor. The comparison chooses the later index on equal distance. It can therefore select a snapshot after the cursor.
- `History::current_snapshot` at `0x140a84650` finds the nearest raw snapshot and returns it only if its chain index exactly equals the current cursor.
- `History::set_current_snapshot` at `0x140a84760` inserts by chain index and asserts if an entry already exists at that index. This preserves the ordered, unique index invariant expected by the binary search.

## Implementation

`tg-history` now stores cursor and snapshot indices as `i32`, implements `closest_resolved_snapshot` with the original nearest/tie rule, and implements exact-cursor `current_snapshot_reference`. The JSON parser cannot observe an Arc strong count, so the closest-snapshot method takes a predicate from its caller to mark IDs as resolved. It does not substitute file existence for the original runtime ownership check.

This implements the history reference-selection policy. It does not decode snapshot bytes or reproduce the Arc ownership lifecycle / game-state loader.
