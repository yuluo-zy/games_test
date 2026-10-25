# Ivy CPU recovery, 2026-10-15

Implemented in country-core resources/ivy and systems/ivy:

- Owned IvyStorage / point deque, root copy, ID allocation, original Perchance spawn ordering and both per-ivy random ranges.
- IvyLeafStorage, endpoint-ID lookup/removal and per-ivy clear (map keys remain).
- WallCoord with arc-length curve coordinate and original double-sided offset.
- Branch thickness and delayed leaf growth/thickness consumers; real unit growth event.
- RemoveIvyCmd producer and consumer: strict box containment; delete inclusive suffix at first hit, delete affected leaves, clear growth interpolation.
- IvyDirectionProposer, IvySpawnNoise and IvyLeafRng use original public dependencies. No replacement noise/RNG algorithm.
- Segment leaf generation: capacity gate, storage RNG selection, noise gate, endpoint lookup, disc offset, six normal samples, quaternion rotation, droop, size and growth delay.
- CPU registration and an ECS create/point/leaf/prune/animation test. Parent reported both this fixture and `generated_leaf_follows_original_disc_normal_and_rotation_order` passed centrally, together with the native-audio all-target compilation check. These are local fixtures; they do not establish whole-module parity.

Primary evidence is the original B instructions in this directory and existing collision-eco decomp. Important corrections from older drafts: point +20 is animated thickness; IDs are not deque indices; delete_all_leaves clears one ivy's Vec; leaf frame is u32; branch thickness uses count rather than dt; direction uses Perchance (not fastrand), a normalized noise vector, and a full ±80° rotation (±40° is its quaternion half-angle). The two Vec3 fields at Ivy +6c/+78 were checked at 14094b1e8..b24e and 14094bf85..bfad, establishing +68 pending growth and +84 progress.

Claude assistance: spawn result and direction result are saved alongside the bounded prompts. Direction response confirms +Z and full angle 160*n degrees before saturation. Its claim that swapping sin/cos makes a non-unit quaternion is false and was rejected; the imported CRT names and lane shuffles are the actual evidence. Spawn response's inclusive range and overlapping-field claims were rejected against native calls/stores.

## Remaining boundaries

- Full grower/root spawner, wall-change point-validity pruner, root/branch deletion policy, UI/ActionSet gates, wall jump/hole constraints and GPU upload/render are not yet restored.
- `ivy_leaf_spawner` consumes `In<bool>` from the real GladeSettings +1c flag; no competing setting owner/default was created. Caller must pipe that flag into the system and maintain events.
- Segment completion events have a real leaf consumer, but their full original growth producer is still part of the unfinished grower.
- source_60/source_18 and RemoveIvyCmd's first two u64s retain observed values without unsupported semantic names.

This is an implemented subset, not completion of the Ivy module.
