# B ui_draw_terrain continuation recovery

This candidate closes the state2 continuation defects; state1 height setup and state3 history completion remain explicitly unresolved. Shared MCP was read-only, identified tiny-glade.exe at the known B path; original instruction/constant checks use SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. C exists in the task index but was not queried or verified.

Observed B code:

- ui_draw_terrain's known function body ends before the indirect jump, so its Ghidra decompilation does not cover the real tail. The previous round4 jump-table contract classified state3 only as “reads points” and missed the pending history producer.
- 0x140a9ff54..0x140a9ff84 resolves the full XYZ cursor position at rbp+0x48, or ZERO when tag&1==0. The Rust code used raw pos unconditionally and replaced Y with0 when constructing/updating strokes.
- 0x140aa0235..0x140aa024f gets the last stroke and combines its presence with BL. BL is the saved `GladeBorder::is_shape_inside` return at 0x140a9ffae, not a cursor-hit flag. Therefore blocked continuation must keep state2 but skip both mutation and changed notification.
- 0x140aa025b loads the spacing from original .rdata@0x1429258d8 = bytes9a99993e, float0.30000001192092896. Current source's1.0 was a guessed default. 0x140aa0266 passes the full resolved XYZ point and this0.3 to insert_or_morph_last_point.
- 0x140aa026b..0x140aa02be reads next_index@TerrainEditingData+0x38, asserts nonzero, subtracts1, and emits an event {event_id:u64,stroke_id:u32} through the changed EventWriter. This runs even when the point helper itself does nothing because of the0.001 threshold or1024 cap. Current source omitted the notification entirely.

Candidate changes: MORPH_SPACING0.3, use CursorTerrainRaycast::position() for all sampled/constructed stroke coordinates, preserve cursor Y, gate continuation on inside, emit EvTerrainStrokeChanged(next_index-1) after the helper call. Two current-world tests distinguish append from old1.0 morph and verify blocked continuation leaves points/events unchanged.

Original runtime evidence: native_terrain_extend_oracle.py executes original state2 range0x140aa0235..0x140aa076d and original insert_or_morph_last_point. It supplies the resolved last-stroke result, preallocates point/event buffers, and uses retained allocator/CRT bridges. Twelve cases cover0/1/2-point strokes, existing segment length0.2/0.3/0.4, inside/outside, near-duplicate and missing stroke. Original event counts are0,1,0,1,0,1,0,1,0,1,1,0. Hash lookup/ECS/world resolution, grow paths, state1 and state3 are excluded. First fixture attempt had a missing Bevy change-tick pointer and failed UC_ERR_WRITE_UNMAPPED; after correcting only fixture parameters, execution exited0. Candidate ECS tests remain for integration owner to run.

Remaining original tail:

- state1 calls actual new_stroke0x140924a10, then sets editing drag tag1/baseheight, checks the live MouseTerrainRaycastSsbo and TerrainRockMaskSsbo through AssetSsboLibrary. Base height is min(max(dragHeight[, +1.5 when no rock],0 or2.5 depending hit test),20); exact registration/missing-resource contract still needs owned recovery. It sets TerrainEditUiState active, emits changed and a tag5 placement record with original position/baseheight.
- state3 clones last stroke points plus height into the real PendingTerrainHistory enum (first qword0x8000000000000000), asserts previous sentinel0x8000000000000002, emits stairs placement tag0 and recalculate, clears editing drag tag. Current opaque PendingTerrainHistory and separate TerrainEditHistoryPending cannot hold the owning Vec payload safely. A typed shared history owner is required; no pointer bits are fabricated into fourqword adapters.
