# BirdSpot return and Wren state checkpoint

Original `1422f86e0`, 378 bytes, executed unchanged in 19 fixture cases. `original-recycle-fixtures.json` records original SHA and full source range. No game callback replaced by a host hook. Profiling dependency boundaries and the explicit original ECS fixture storage are recorded by the probe.

Queue owner/Drop belongs to wall-gpu candidate `bird_spots.rs`. This patch depends on it, does not replace it. The callback checks `Option<Entity>`, live generation and the actual Query membership, then writes only Wren state 0 -> 1, setting Changed only for that write. It never compares the BirdSpot id. Query requires Transform and Wren and excludes SittableForBirds. Queue is drained every update, including no-event frames; manager keeps the lock across callbacks and preserves capacity.

The former separately registered WrenSpotState had no spawn writer and prevented a real actor callback. State now belongs to canonical core Wren, both boid births create original state 2, and update_wrens reads this same owner. This is a known semantic projection, not native 80B ABI reconstruction or a claim that broader Wren AI is complete. Existing position/boid/audio algorithms are retained. Native original spawn records at `142326a80` write state 2 in all three tuples; audio is still the current separate host component.

Roof BirdSpots are inserted on the roof entity, rather than spawning unrelated entities. Decorator construction now uses the actual record/id and manager lifetime. The sampling branch is deliberately still labeled unresolved in this checkpoint and will be the next checkpoint; this does not claim the roof producer complete. Current old RecalculateRoofCmd payload adapter and decorator retention cleanup also require independent original call-site verification, not covered by recycle fixtures.

Integration tests use real Bevy World, actor generation, actual Query/filter, changed ticks, Drop and queue drain. They have been prepared, not compiled or executed. Source/Cargo untouched. `components.rs` is only the Wren reexport narrow hunk, and must be merged with wall's own Bird/PrevTransform changes.
