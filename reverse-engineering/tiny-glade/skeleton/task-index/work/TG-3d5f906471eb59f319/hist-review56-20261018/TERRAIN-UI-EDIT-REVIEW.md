# Independent fifth-batch ui_edit review

Reviewed frozen TG-72f0eb6920f270f8a6/terrain-curve-20261018/real-edit-producer
(17 fingerprints passed). Target is B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
No production source edits, Rust compilation, game process or GPU execution.

## Original evidence, read before candidate

The four exact relative jump-table entries at0x142ae404c resolve to idle/begin/
continue/release. ADD AL,-2 followed by unsigned min3 means subtype2 idle,3 begin,
4 continue, and0/1/5..255 release. Independent execution of all256 inputs matched
the frozen native observations. The candidate's wrapping_sub(2).min(3) implements
this original rule. The removed alleged grading controls belong to an adjacent
grading_lut serde field visitor: retained original matching PDB/LLVM source
samples bind0xaf01f0 and0xaf02f0 to that separate source. The collector's broad
range is explicitly not a verified single function boundary; no such claim is
needed for this four-entry dispatch result.

Original0x140aef4d4..0x140aef51f computes sqrt(dy²+dx²), both absolute components,
dx*0.002 and dy*0.02/20. Case2 publishes the unit notification before its positive
length gate. At equal absolute deltas horizontal wins. Original horizontal command
uses falloff Delta(dx*0.002), height Keep. Vertical reads the selected stroke's
height and computes sqrt(clamp((height/20)²-dy*0.02/20,0,1))*20-height, uses falloff
Keep/height Delta. Source-biased SSE max/min preserve unordered and equal-source
values, consistent with the candidate's ordered comparisons and unchanged NaN.

## Independent checks and current consumer

Independently replayed18 original case2 fixtures with exact command count,
operation tags, active f32 value bits, cached XYZ and ID. Keep value bytes are
uninitialized/unused and intentionally excluded. Additional probes cover NaN in
each delta axis, signed-zero delta and NaN original height: zero/unordered delta
emits no parameter command but keeps the unit notification; NaN height emits
the original NaN vertical Delta. Exact reports are retained in
terrain-ui-edit-independent-review.json and can be rerun using review_ui_edit.py
without executing the author's report-writing footer.

Independently verified every instruction byte in both retained LLVM packs against
the original RNE and patched B input, and re-read the jump-table bytes and targets.
The public FunctionSystem signature has EventWriter<EditTerrainStrokeCmd>, matching
the native40-byte EventInstance. Its layout is falloff operation/value@0/4, height
operation/value@8/12, position@16..24, ID@28 and eventID@32. Existing EditCommand/
Adjustment already express these rules, so the candidate adds no ABI replica.

The production edit_terrain_stroke consumer reads EditTerrainStrokeCmd and calls
the existing instruction-backed apply_resolved, updates the live stroke and calls
ScheduledTerrainReadback::on_terrain_modified. register_terrain_events registers
the command and add_terrain_tool_systems orders ui_edit before TerrainCpuStage::Edit.
The candidate's current-world command-consumer/readback test is therefore a real
existing consumer path. Root must compile and execute it; this reviewer did not.

## Disposition and boundaries

Pass for the frozen dispatch and typed parameter-command producer. Its native
unit/other notification identities remain wrong in this intermediate patch, as
the contract explicitly acknowledges; sixth-batch correction is required. The
full click/MainCameraMoved/DecoratorArchivist state and original native hash/grow/
failure paths are outside these resolved probes. Existing begin/cache projection
and release-history checks are not independently reaccepted by this report.

The corresponding edit consumer still routes audio through a historically wrong
stairs bucket in the baseline. That producer-to-audio endpoint is being corrected
separately; it is not implicitly accepted by the valid EditTerrainStrokeCmd arm.
