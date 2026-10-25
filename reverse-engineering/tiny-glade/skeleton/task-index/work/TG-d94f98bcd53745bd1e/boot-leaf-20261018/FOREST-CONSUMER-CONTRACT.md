# Forest consumer B contract

Status: work candidates only; original retained RNE bound, no Cargo or GPU run. Original RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. All seven before hashes match current source at delivery; `git apply --check` succeeds for all. Manifest lists exact file and patch hashes. Root owns integration.

## Proven producer/consumer chain

`init_clearing@140a78aa0` loads actual PlacementData48 and PrefabName, filters out central points, leaves real entity None, and resets one shared InteractableForestPlacements.initialized=false. The separate entity component is placement index + amplitude + progress + delay, not a second position/scale record. No fabricated Entity or prefab name/default.

`manage_tree_prefabs@141bf16c0/5342B` clears Local candidates every update, borrows actual mapped rock bits, reads/consumes the last unread ConstructTimberframeCmd, evaluates four actual paths, water/garden/rock, optional shape and actual occupied wall cells. Out-of-bound trees remain allowed. Its second stage seeds original RNG from actual GameTime.frame_count. Every candidate computes first-update or later random data before keep branching. It uses Commands-generated Entity, complete Placement48 including custom0 expansion bits, exact packed anim_t, actual height only inside the half-open world bounds, and retains outside original Y. It removes/inserts real deletion marker and finishes initialized=true even for an empty collection.

`forest_tree_animation@14227a0d0/2228B` writes actual PrefabInstance48.scale from cubic-out progress, consumes true per-entity index/amplitude/delay, sends Entity-only Despawn on completion and clears the exact placement entity owner. `handplaced_tree_animation@142279500/3017B` uses original AppMode/EraserLayer branches, actual camera/audio-trigger boxes, shared canonical two-f32 canopy state, and writes actual instance.anim_t_packed. MAXSS/MINSS preserve unordered progress, no invented NaN-to-zero.

`forest::build@140650c50/243B` proves Update set36 and tuple `manage_tree_prefabs -> forest_tree_animation -> handplaced_tree_animation` using `chain_ignore_deferred`. The former ordering and `chain()` inserted unintended deferred flushes. The candidate fixes the single existing public builder and does not invent a second algorithm/schedule.

## Existing shared algorithms corrected

`WallSpatialHash::query_with_circle_callback<forest>@1422785d0/837B` queries live occupied cells and circle-vs-cell center using actual half diagonal, not wall segment midpoint or nearest segment. New narrow specialization reads the same authority and its cells; existing generic APIs are retained for independently contracted consumers.

`Rectangle2d::get_closest_edge_id@140c9acb0` is only a five-byte jump to `get_closest_edge_pt_indices@140c9a9d0/726B`. Original initializes best distance **1000.0** (rdata14295b630), uses projected closest-point Euclidean distance and first strict improvement; the old infinite bound/squared distance loop is not the same algorithm. Candidate directly refactors this shared method through the restored single selector, adds the original `closest_point_on_perimeter@140c9ae20/475B`, and keeps its direct recomputed point instead of lossy u conversion. Actual guard is area>0; epsilon bits1e-20. One bounded test captures first-edge ties and native finite bound. These are algorithm facts, not execution validation.

## Original evidence

- This family: full manage RNE `141bf16c0-full-B-rne.asm`, actual occupied-cell callback `1422785d0-full-B-rne.asm`, constants and native manifest.
- TG-a433e0de1f7930df66/boot-leaf-20261018: full two animation bodies and canopy mode jump table.
- TG-35af5617310f31b9df/boot-leaf-20261018: full build body and named chain helper PDB.
- TG-6e76d82e8a953a9a45/boot-leaf-20261018: full rectangle selector/perimeter/tail-jump, RIP bytes and SHA.
- Clearing TG-025d04787e7e6b72cd/boot-leaf-20261018: full init6913B, filter500B and emitter closure521B, serde Config5827B.
- Canopy initial zero values from native run_game_thread@14059f41d..426 (same boot original RNE artifact).

## Hard dependencies / not complete yet

1. Foreign core systems/prefab owns canonical event migration: Spawn payload must PrefabName16+Entity8+complete Placement48, not one/flags/mode; Despawn payload Entity8 only. Current candidates intentionally require that corrected API. Do not preserve wrong fields or reinterpret memory.
2. Existing draw_callbacks_b legacy canopy view must project actual canonical smoothstep@+4 (native draw callback must be independently checked); do not create a second resource.
3. Complete clearing including Winter ice, Flowery lavender+meadow and Olden rectangle/autoclutter producers is in preparation, not claimed ready. True preplaced-tree event conversion is separate from GPU emitter generation.
4. Root must register this actual chain under its original set36 in the same gameplay App with actual resources, producers and canonical prefab consumers. No resource_exists or fabricated resource gate.

No original-process equivalence, full scene readiness or W18 nonzero GPU validation is claimed here.
