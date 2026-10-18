# Independent review: typed terrain history candidate

Target B. Reviewed frozen-inputs.json: owned-files.patch SHA256 59641f78bfcbfa0019a3fe1fdcdd10bf728f3484542480240ff15ed18a40b2e0; integration-owner.patch SHA256 a19422e5a8bceda93b02d20418763c95830dd76cfe0a8b42ca95f7f94ba43f33. Every listed frozen source matched its declared SHA at review time. No production source edits, Rust builds or tests by this reviewer.

## Required changes

1. **P1: ui_draw_terrain candidate does not resolve its new types.** The frozen ui_draw_terrain.rs introduces PendingTerrainHistory / TerrainEdit / StairsPlacementBucket / StairsPlacementEntry, but the full file imports none of these and does not qualify them. owned-files.patch also lacks these imports. Add the existing sibling-owner imports and refreeze; these are ordinary compiler errors, not an ABI concern.

2. **P1: production terrain tool adapter lacks the ordering used by the candidate test.** Current add_terrain_tool_systems registers hover, draw, edit, focus, history_record_terrain and replay_terrain in an unordered tuple. The candidate changes only owned leaf sources and two history dispatch files, so this remains true after applying the submitted patches. The release regression manually chains ui_draw_terrain -> history_record_terrain; the real adapter does not provide that edge. A deferred-first schedule records the release in a later frame, potentially after that input's history transaction closes. Likewise replay_terrain has no membership/edge against HistoryCpuStage::Cursor/Pop or TerrainCpuStage::Edit, so its typed EditTerrainStrokeCmd / SetTerrainStrokePtsCmd and final completion side effects can be consumed after Pop/finish. Connect the actual registration owner to the existing stage architecture. At minimum: current-frame resolved input -> tool producers -> deferred history recording; History Cursor -> terrain replay -> terrain edit/change consumers -> History Pop/finish; history recording before Input Finish. Validate the resulting combined schedule, not just an artificial two-system test. These are local adapter dependencies, not a claim about original global native scheduling.

The integration owner has been informed of both findings. No other blocking deviation was established for ordinary finite inputs in the reviewed algorithmic chain.

## Original behavior independently confirmed

- Default pending is empty and both tool producers share the same typed owner. A compatibility type alias is suitable; the native niche layout need not become Rust state.
- Original ui_edit capture/release order is height/falloff capture followed by typed index, previous height, new height, previous falloff, new falloff. Direct RNE bytes 0x140aefc86..0x140aefcf1 match that mapping.
- New stroke undo at progress >=0.99 writes zero to the mapped height field, preserves ID/points and publishes the source ID as changed. Earlier removal behavior was incorrect.
- update_terrain_curve uses the existing stroke's count guard: zero asserts, one retains the old point and returns CubicInOut(progress). For larger old strokes, it removes adjacent duplicates from the source; invalid Curve3 keeps old points. Valid curves use the full XYZ metric, minimum visible length0.1, end=(1-eased)*clamp(0.1/length,0,1)+eased and height factor CubicInOut(clamp(progress*4,0,1)). The candidate uses the shared current Curve3/easing implementation.
- New replay feedback XZ comes from the final source point and Y from resulting height. Original reapply 0x140859564..0x140859583 explicitly forwards that height to xvy; this is not the source point's original Y.
- Edit replay reuses the existing tg_terrain::replay arithmetic and endpoint-derived operation flags. Move replay uses CubicOut, original expression order, and output middle point as feedback position.
- Original replay first forward New at elapsed==0 inserts the complete source stroke with falloff0.4, preserved height, and max(next_index,id+1) before applying the curve animation. The candidate preserves that.
- Duration is length-derived only for New (including zero/one source segments: length0 -> duration0.4); other variants use0.3. Outer animation threshold is0.001. Original 0x1409953b0 decrements initial counter2 and queues removal only when negative, hence three completed applications. Elapsed increments even on completed applications.
- Pending is taken before History.add and shared payload-free DebugHistoryMsg is emitted even when add returns an error. Root separately owns the remaining shared debug consumer and save/schema compatibility edits.

## Evidence and limits

Independent local extraction in this directory:
- terrain-review-static-identity.json
- terrain-review-140858d30.asm
- terrain-review-1408592d0.asm
- terrain-review-140aef1f0.asm

RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. All three bounded code ranges were independently compared against the current B patched PE and were byte-identical. 371/302/1213 Capstone instruction records are retained with original bytes. These ranges include unwind/alignment neighbors where appropriate; only explicit function/branch addresses above are used for conclusions.

The fresh read-only Ghidra get_functions request for these three functions failed with Corrupted BufferMgr state after matching metadata; raw failure retained in terrain-review-fresh-assembly.json. No database changes or retry were made. Review therefore relies on retained RNE update/replay disassembly plus independent local PE bytes and the previously collected original decompilation.

The author's 24 original curve fixtures exercise original native business instructions with resolved one-bucket hash and allocation/CRT bridges; three release fixtures exercise an original branch range with supplied last-stroke and preallocated queues. They are useful bounded evidence, not full ECS/GPU/native process validation. Root owns compilation and candidate regression execution. Extreme-coordinate differences intentionally present in current shared Curve3 were not reimplemented or expanded here. ui_draw starting-height SSBO policy and the already documented incomplete ui_edit command production/response switch remain outside this typed-history candidate.

