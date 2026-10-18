Apply `owned-wall-history.patch` plus `integration-owner.patch` together. Both target the recorded production base paths/hashes in source-map.json; preserve other owners' current edits when shared base drifts. Shared hunk files are systems/wall/history_recording.rs (new explicit factory), resources/history/serialized.rs (independent saved height), systems/history/replay_wall.rs (one existing literal and one actual consumer test), and eraser/archivist.rs + eraser/bulldoze.rs (required new-field source-compatible literals only).

Continuation owner uses:

```rust
RecordWallHistory::deleted_wall_with_height(wall, &previous, current_color, desired_height)
```

The previous source remains intact. Do not assign desired_height to it. Delete Some undo must keep previous height; only Create's separate-height override exists in the current replay implementation.

Root should run the country-core test suite and joined wall lifecycle tests after incorporating the sibling continuation candidate. In particular validate the invalid continuation's old-height 3.9 / desired-height 7 payload and saved round trip, then undo old source 3.9 and redo absent wall.
