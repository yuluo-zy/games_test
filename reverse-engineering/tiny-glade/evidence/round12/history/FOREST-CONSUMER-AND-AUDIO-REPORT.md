# Forest canonical consumers and audio pool

Production writes:0. Cargo/rustc: not run. All candidate Rust files parsed; both patches pass git apply --check against their recorded bases. The central owner applies together with round12/clearing's producer/types and the independent audio owner's canonical instantiate_loop correction.

## Candidates

* `forest-consumer-candidate.patch`, SHA25616547f3d7b088195799bb6ba675c0f1afe4a9477c63d179a1db542f5f2e762a9: manage_tree_prefabs.rs, only forest_tree_animation in tree_animation.rs, new native_forest_fixtures.rs. The handplaced-tree function is retained verbatim.
* `forest-audio-pool-candidate.patch`, SHA2565aae319e5fd94deae76d8048a402595bd0f0a070d8ba0fd0714b5743b3980ded: full canonical forest_audio.rs. The independent audio-core owner explicitly handed off this entire file; no overlapping single-loop initializer is retained.
* Corresponding `*-manifest.json` files bind every current-source and candidate hash. `build-forest-consumer-candidate.py` and `build-forest-audio-candidate.py` reproduce candidates without production writes.

## Native consumer141bf16c0/5342B

The producer owner recovered separate ForestTree80B records and InteractableForestTree24B components. The consumer uses the same canonical fields and does not convert records into components or put entity bits into an unrelated id field.

Phase1 retains the six samplers and their order; the four path contributions are three ordered additions, followed by MAXSS0/source and MINSS1/source semantics, preserving NaN/-0. Water/garden exclude first. The mandatory same mapped TerrainRockMask SSBO is resolved once with its original missing/downcast failure policy, rather than silently reporting no rock. ConstructTimberframeCmd is now the real fixed32B payload; EventReader drains through the last actual shape, and circle/rectangle branches preserve the original strict comparison.

Critical correction: original wall specialization1422785d0 does not enumerate nearest segments or compare their midpoint to the tree. It scans circle-overlapping spatial cells, expands the radius by half-cell diagonal, and clears keep at the first occupied cell (14227883b..876). It is the same algorithm family already recovered for convert_preplaced_trees1425defd0, so the candidate reuses the existing canonical has_occupied_cells_in_circle method. The old segment callback was fabricated behavior.

Native phase2 seeds a local generator from GameTime.frame_count+2c at141bf1e48..53 via the existing verified utils::rng::init_rng. Every candidate advances the local generator after processed=true, including rejected/existing entities. The random lerp(.3,.75) is a newly grown tree's delay, not its amplitude/scale. First pass uses saved placement.scale as initial scale, progress1, delay0; subsequent spawn uses scale0, progress0, random delay. The component's amplitude is always the original saved scale.

Commands use actual spawn_empty entities, warn removal of TreeMarkedForDeletion, panic insertion, and canonical record.entity writeback. Standard Bevy0.16 EntityCommands::remove is the exact warn policy (verified registry source), while try_remove would incorrectly suppress it. Original queue handlers142278cf0/142278f00 independently identify warn/panic.

The prefab event is fully populated: same PrefabName Arc, actual entity, original position with bounded native height sampling, initial scale, rotation/id, custom0 from record.billboard_expansion_scale, custom1 unchanged, packed animation(initial_scale,1), both bend floats and pad0/mode unchanged. There is no default empty event. TreeAmbience is inserted only inside -65<=x,z<65; the original outside placement keeps saved y and has no ambience component. Native zero multiplications and additions in ambience coordinates are retained for NaN/-0 behavior.

## Native animation14227a0d0

The animation now accesses placement_index/amplitude/progress/delay by their actual meanings. The previous positional proxy is removed. Fresh original14227a68a..69f uses MAXSS/MINSS and retains NaN/-0; the old reconstruction's branch clamp incorrectly replaced NaN with0. The candidate preserves native operation order and writes eased progress×amplitude into the same48B prefab component.

Marked trees at ordered progress==0 emit the real DespawnPrefabCmd and release precisely placements.records[placement_index].entity. Native clears generation@record+44; the safe owner uses Option<Entity>::None. Bounds still panic. This closes disappearance→slot available→regrow; the old implementation did not release any record.

## Native audio140aa18f0/1543B and140aa1f00/4295B

The old single-loop implementation was a major business simplification. Native initialization allocates50×56B AudioLoops, each desc(name=event:/SFX/ForestGrow,count3,threshold50), an empty HashMap<usize,usize,RandomState>, and an explicit free VecDeque0..49. The independent canonical constructor owner proves count3 is three additional instances, so each loop has4 and the whole forest has200 actual instances.

The exact query is (&InteractableForestTree,&PrefabInstanceData). Only menu tag2 or tag10/flag0 runs allocation. Other menu states request stop for all50 players without discarding maps or queue ownership. Each queried placement index is recorded, including inactive/duplicate IDs. Ordered0<progress<1 starts/reuses its mapped slot and sets the current instance's position from the actual prefab placement; no free slot means no allocation or stealing. Inactive progress removes the mapping and appends its slot to the free FIFO. After traversal, clone active keys, reclaim keys absent from the real query, then clone the free deque and request stop for each free player. No sorting changes native iteration/queue order. All start/stop errors follow the original shared-helper contract; indexing still panics.

## Evidence and verification boundary

`verify-reveal-forest-original.py` retains original RNE identity,20 complete function ranges with original bytes/hashes/all PDB aliases, and native disassembly in `reveal-forest-original.{json,asm}`. The consumer, wall specialization, component queue wrappers, native animation, audio init and complete audio consumer are included. Profiling and dependency container internals are evidence only, not reimplemented.

`verify-forest-native.py` actually executed the original475B rectangle projection plus726B native edge selector in Unicorn for84 cases, and original14227a60c..6a5 animation arithmetic for280 inputs covering±0,NaN,Inf,positive/negative delta,marked/unmarked and delay. Native outputs and function hashes are retained in `forest-native-rectangle-fixtures.json`; generated Rust constants are in native_forest_fixtures.rs. These are native-only oracles; candidate Rust has not yet been executed.

Candidate nonempty regressions exercise the actual deferred ECS phase2 and actual forest_tree_animation: full prefab payload with nonzero channels, bounded terrain sample and ambience, existing-entity duplicate suppression, marker reversal, deletion and record release, regrow, and outside-boundary placement. The Rust rectangle test compares against84 original executions; real ECS animation compares280 original bit fixtures. Audio regressions run the actual canonical query with recording Studio/Event backend:50 loops/200 instances, per-tree positions, FIFO reuse, disappearance, preserved menu mappings,51-tree saturation and NaN exclusion.

Remaining acceptance: central Cargo/type validation and first+second complete sessions, actual GPU prefab consumer execution, real FMOD pool lifetime and host query schedules. The handplaced canopy resource/local proxy and its unrelated audio-box policies remain outside these narrow changes. No claim that all forest business or all startup rendering has passed is made before those boundaries run.
