# Roof visual chain: original evidence and reviewable candidate

Production files were **not modified** and Cargo was **not run**. `roof-visual.patch` and `proposed/` contain seven candidate source changes; `patch-manifest.json` binds the actual base bytes and LF-normalized proposed text. This candidate is not compile, GPU, or whole-chain acceptance. Root is the shared-source integration owner.

## Confirmed corrections

1. The native `RecalculateRoofCmd` has a 32-byte `EventInstance` stride, with entity at event+8. `assemble@141a36700` and `stone-floor@1421a6d40` perform direct Query entity lookup and unwrap; they do not re-resolve through PublicWalls. Current `RoofGeometryChanged` already owns entity, wall and kind. The candidate re-exports this single event identity as `RecalculateRoofCmd`; all current production readers have been adapted. Whole `crates/**/*.rs` search found no tuple construction other than the superseded definition. Resource registration in core/critters remains idempotent initialization of the same Events TypeId.
2. Assemble retrieves the canonical `RoofTileSsbo` storage before entity lookup, lazily allocates on the real RoofTiles component, calls `writer.clear()` **before** the flat gate, then calls the existing original tile algorithms. For rectangles it passes the full roof bytes `[0x0c..0x24]`, not None. `special_mode` is the exact nonempty `ArchSegments[roof.wall_id]` predicate, not an arbitrary false. The map now owns full104-byte materialized segments from the already restored wall algorithms.
3. Each generated64B record is decoded field-for-field into `RoofTileSsbo`, then pushed through the current sparse writer. Capacity exhaustion keeps the written prefix and continues generation. Rectangular closure1421b2ca0 and circular closure1421b4510 each have an independent warn-once expansion table and the original message `Out of roof tile instances`. The candidate retains two tables. No old rhapsody or buffer backend is introduced.
4. `create@1421aaea0` native spawn tuple includes Roof, simulation, trigger, animation, RoofTiles, system-roof RoofSpire/SupportBeams, and `NaniBundle<StoneFloorInstanceData>`. Candidate restores the complete tuple with original AtlasPart0/RoofFloor9/identity transform, instead of empty markers on separate entities or a CPU-only roof. Despawn drops this actual allocation/descriptor owner.
5. Stone-floor generation is a real descriptor write. Rectangles use part0. Circles use `log2(radius*3)` bounded to1..5 then saturated/truncating u32 conversion. Native MAXSS and MINSS choose the second operand on unordered values, so NaN remains NaN through both operations and becomes0 on Rust `as u32`. Existing blanket `max/min` numeric semantics would be wrong. This replaces the current empty Command. The whole native839B range and its constants are freshly extracted from original RNE.

64B record offsets are0:Vec3,12:f32,16:2u32,24:2u32,32:2u32,40:u32,44:u32,48:f32,52:i32,56:i32,60:i32. Explicit byte decode preserves every integer/float bit pattern without unaligned pointer casts. `roof_tile_ssbo.rs` and original `RoofShingle` shader are the layout authorities; native push141a3c070 copies exactly8qwords. No legacy algorithm is recreated.

## Actual original registration, not inferred local stages

`add_roof_systems@14062fcb0` retained decomp is `evidence/round10/tools/dec-14062fcb0.c`. Each helper below was newly checked against original RNE and its embedded exact system name. `original-evidence.json` and `register-*.original.asm.txt` preserve the lookup. Adjacent exception/next-function disassembly is retained in files, while extracted constructor names stop at the first main return to avoid misattribution.

| Original set | Original group / helper | Confirmed ordering |
|---|---|---|
|17|140ddbf90: ui_edit_roof, ui_edit_roof_ridge_dir@140de4980, ui_edit_roof_ridge_length@141a20600|3-item tuple, unchained |
|18|140ddba10: do_ui_edit_roof, focused ridge/height@141a1e290, do_ui_edit_roof_ridge_dir@140de47d0, do_ui_edit_roof_ridge_length@141a20450, switch_roof_ridge_direction@140de4b30|chain_ignore_deferred |
|20|140ddda40 outer group, detailed below|nested chain with deferred boundaries |
|21|140ddf400: delete_roofs|single system |
|29|140ddffb0: assemble_roof_tiles, stone-floor@141a37900, place_spires@1419ee830|3-item tuple, unchained |
|34|140de19b0 roof_record_history;140de4d60 record_switch_roof_ridge_history|separate configurations |
|40|140de10b0 replay_roof|single system |
|44|140de55d0 gpu_flush_buffers|single system |

Set20 is exactly the following structural composition. There is no registration proof for the present local WallCpuStage names:

```rust
(
    (
        (
            mark_roof_for_deletion,
            update_roof_spatial,
            wall_height_dt,
            roof_animation,
        ).chain_ignore_deferred(),
        create_roof,
        edit_roof,
    ).chain(),
    simulate_roof_tile_for_audio,
    highlight_roof,
    switch_roof_type,
    calculate_roof_trigger_volumes,
).chain().in_set(GameSystemSet(20))
```

140ddeec0 constructs the first4-item group;140dddf00 places that configuration first then creates141a33c00/create and141a34a50/edit;140ddda40 places the passed configuration first followed by141a30fc0/simulate,141a31980/highlight,141a32920/switch,141a32ff0/trigger. Caller puts the result in20. Thus create-generated events reach trigger in20 and visual29 the same frame; replay40 normally enters the next frame's CPU20. Current history_runtime places most CPU work in40 and some dependent work in35: simply adding visual29 would introduce an unintended extra frame delay. Root must review the entire group and association/publication dependencies when integrating.

Device resources are already produced by the shared `CoreRenderBridge::install_asset_libraries` and actual gameplay PreStartup `init_populate_ssbo_library@140909e80`: RoofTile (registration11), Spire(12), support-beam(16), WoodenPlank(17). ArchSegments is populated by real wall construction; its presence with an empty map is the valid no-arch case, not permission to fabricate a nonempty segment. `gpu_flush_buffers@140de50a0` is a real sparse GPU upload in set44. It is currently absent from tiny-glade production registration. CPU push by this candidate is not GPU upload acceptance.

## Complete nonempty lifecycle acceptance plan

Extend the existing `history_runtime::tests::production_roof_create_edit_delete_and_undo_restore_actual_entities` through the same production installer, rather than using an isolated Roof byte packet as the integration oracle:

1. Use its actual InnerWalls rectangle/circle construction, original enclosure projection and stage19 PublicWall publication. Populate ArchSegments through `PublicWallState::construct_arches` with the **actual** segmentation masks, prepared path, regular/freehand arch caches and independent segment-height owner. Run both no-arch and nonempty-arch branches; never insert a fabricated MaterializedWallSegment merely to toggle the bool.
2. Create through the original saved Roof edit/replay path. Observe event TypeId equality and event entity/wall/kind, and assert all8 original bundle components are on the same spawned entity. Check actual RoofTiles allocation draw ranges are nonempty and all64B active records match the existing generator for the actual authoritative rectangle/mode. Check Spire instance_count against original ratio branch and RoofFloor9 actual Nani descriptor/active-key owner, not just marker existence.
3. Edit through real VisualEdit and change circle/rectangle/ridge/gable/flat branches. Assert original same-frame20→29 event delivery, old chunks cleared, and flat transition clears sparse ranges before early return. Assert circle floor parts follow1..5 boundaries and NaN conversion separately as numerical contracts.
4. Execute44 flush through the real bridge device and compare actual GPU buffer readback with the sparse CPU shadow over active64B records. Build the actual DrawLists, require the expected roof callbacks/ranges and render nonempty tiles/Spire/Nani in the production world graph. Exported graph success alone is insufficient.
5. Delete via saved edit, run full original animation lifetime, verify original entity gone and PublicWalls association removed. Run canonical Nani remove/maintain and sparse waste collection. Check all old tile chunks become free/inactive, Nani active key disappears, Spire count updates; undo creates a fresh entity/owner and re-uploads records. No manual despawn substitution.
6. Support beams require a proven nonempty native producer and physical upload assertion. A SupportBeams marker and preallocated SSBO are not evidence that this branch works.

This plan has **not run**. Existing CPU history test alone does not meet these GPU/lifecycle conditions.

## Remaining exact blockers and next bounded evidence

- `calculate_roof_trigger_volumes@1421a3220`,12048B, is still a no-output wrapper. Its complete rectangle/circle tessellation/UV/normals/noise algorithm is in retained `building-roof-trigger/B-producer/1421a3220.c` and newly bounded original assembly. Restore the whole producer before calling set20 complete; do not install a no-op and call it acceptance. Shape-specific bounds, gable/eaves, normalized/profile-world curves and original noise helpers already exist in core; arithmetic and branch equivalence remain to be transcribed and checked.
- RoofSupportBeams source module has no standalone public procedure, but the SSBO/marker/draw consumer exist. Do not substitute `system_wall_constructor::construct_lintels::place_support_beams` merely because of the same words: those emit WoodenPlank64B and are a different producer. Next investigation is PDB inline/line records for `visual/place_support_beams.rs`, whole original Spire/roof plugin factory tuple and RoofSupportBeamSsbo write sites. Current candidate intentionally does not invent a producer.
- Existing Vec-batch tile algorithm computes before the sparse pushes; its valid-input numerical sequence has prior native verification. Partial-panic behavior on malformed/NaN geometry could differ from original callback-as-generated writes. This exceptional side-effect equivalence remains unverified and must not be hidden by swallowing Curve errors.
- Reconstructed snapshot roof creation may have other bundle construction sites outside create_roof. Whole original App/snapshot acceptance must verify those sites also produce the same owner tuple; no completion claim from this seven-file candidate.

T13 feedback capture is explicitly excluded and has not been investigated or added.
