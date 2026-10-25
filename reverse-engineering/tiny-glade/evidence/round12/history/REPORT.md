# Round12 history delivery and current acceptance boundary

Later independent correction: the native9/16 attribution below was wrong. Native9 is typed ChangeStyle, native16 is bool ChangeSkirt; native10 plaster and12 supports wire are now restored. See [HISTORY-WIRE-CORRECTION.md](HISTORY-WIRE-CORRECTION.md) and the central validation log for current acceptance. Earlier claims below are retained as historical delivery context, not an override of the correction.

This batch preserves the latest rhapsody and existing utility algorithms. No Cargo process was started by this agent; root owns target-round12 checks/tests. Source syntax was checked with rustfmt on the changed files. Rustfmt success is not compile/test acceptance. Root's final logs determine validation; no ignored external diagnostic is counted as a pass.

## Implemented

- Saved MultiEdit now preserves entry-owned decorator/stairs auxiliary diffs, including original vectorized decorator addresses and named states and original stairs edge tuple/hash fields. DecoratorV5 retains its real32B marker owner; it is distinct from the auxiliary diff. Terrain and all6 raster outer variants use their original wire shape; Max/Add opacity are not confused with world raster pixel blend policy.
- Roof Create/Delete/Visual/Ridge/Rim codec is typed with real named fields and proven missing-field defaults; tag4 is ChangeFlatRoofRimStyle with before/after Rim/NoRim, not from/to. Roof CPU create/edit/direction/rim/deletion/association/gable lifecycle is registered. Color restore and user color events reach the actual typed GPU color buffers when the device exists.
- All12 observed inner saved Wall variants have typed ownership and save/reload codecs. Native4 is discrete IsEnclosed; native5 is premade geometry morph. Native8 modifies source+64 arch terrain policy, native12 source+67 supports. Native9 is deprecated ChangeSkirt, not stairs. Native16 has save-name ChangeStyle and legacy Debug-name ChangeSkirt. Evidence: original17-way process tables, original Debug table, actual updater byte stores.
- Wall replay now executes native5 rectangle/circle morph using existing shape/curve utilities, native6/7 scalar mutation, native8/11 terrain policies, native12 support bool and native16 style. Native13 MorphFreehand uses original game wrapper140952570: t==0 clones before with exact cached u/length; other factors invoke existing utils Curve::crashy_lerp and write the authoritative curve. The only new generic utility API is from_cached_parts: an ownership constructor, no algorithm rewrite. No fake generic morph implementation was added.
- Timed expansion now includes real Height/MorphPremade/Elevation/ArchHeight/MorphFreehand paths instead of accidentally timing IsEnclosed. Source publication notifications and original command-group boundaries are retained. Malformed/unrestored deprecated payloads still produce errors or the original deprecated panic; they are not empty edits.
- Tree bend and tree animation use the canonical48B PrefabInstance, propagate actual bend/packed animation values and finish deletion before prefab despawn. Existing forest algorithm gaps remain separate.
- ArchSegments is now an owned map of full MaterializedWallSegment records, matching native104B three-Vec plus metadata owner. Existing verified regular and freehand arch algorithms return their complete profiled segments; PublicWallState executes them against actual prepared positions/U/path values, real arch caches and independent third height array. The wall-constructor wrapper writes the map rather than dropping the segments. Segmentation wrapper likewise uses the actual prepared path, replacing its synthetic XY/zero data.
- ConstructGateCmd now owns the complete materialized segment plus WallId/ordinal, instead of pretending native Vec headers are an inline120B point packet. Consumer uses the current prepared curve and actual terrain sampler mode. Existing construct_gate_cpu feeds real Mesh4attributes/indices, MeshLibrary, exact shader key, DrawableMeshBundle and TransientMesh. Full leaf metadata is kept in WoodenGateState; the existing render view receives actual WallId/seed. Every Nani projection becomes a real typed NaniBundle and is tracked by Gates. Rebuild streams drain/despawn in order and preserve empty entries; wall/roof deletion removes the entry. Invalidation precedes construction. Removed the old empty-entity spawn and invalid24B<-12B payload copy.

## Meaningful regressions added, awaiting root execution

1. original_mixed_multi_edit_aux_survives_save_reload_and_replay_expansion:18 unmodified original MultiEdits across13 outer domains, with real auxiliary diffs.
2. all_observed_original_wall_variants_preserve_saved_payloads: one unmodified original object for each of12 observed inner variants, exact new named payloads and native8 identity, plus Roof rim.
3. original_freehand_curve_and_discrete_policy_replay_redo_then_undo: real original freehand points/cache and independent arch/support bytes.
4. production_roof_create_edit_delete_and_undo_restore_actual_entities: actual canonical walls and stage19 publication, real roof lifecycle through edit/delete and undo,24frames per operation.
5. gate_rebuild_and_roof_delete_despawn_tracked_entities_in_real_world: actual Bevy Entity cleanup and empty-vs-removed map behavior.

Full external original corpus diagnostic must be explicitly run, not counted ignored:
`TG_INSTALL_ROOT=D:/game/ljxsj_92385/Tiny Glade cargo test -p country-core --lib all_original_starting_build_histories_decode_save_reload -- --ignored --nocapture`.
Expected33 history.json files /2120 MultiEdit entries. This checks decode/save/reload, not by itself whole scene replay or GPU correctness. Parent owns actual command invocation and logs.

## Still partial / unverified

- Root host does not yet have complete wall constructor/GPU instance production/flush integration. Arch masks and profiles now have correct authority inputs, but whole MaterialStats/style branches and finite sparse-pool warning policy must be audited before installing the entire library registration wholesale. Generated support96B records still enter WallConstructorOutput; a CPU record vector is not a GPU upload.
- assemble_roof_tiles still needs the actual ArchSegments-derived special mode, Roof+c..24 rectangle frame, sparse allocation writer and genuine RoofTiles component production. Its current old wrapper uses None/false and cannot be used as evidence that pitched roofs render. Root has been notified.
- Actual original create bundle includes RoofTiles, RoofSpire, RoofSupportBeams and StoneFloor NaniBundle; current reduced CPU creation bundle does not produce the full render owner. RoofGeometryChanged and RecalculateRoofCmd are currently distinct ECS event identities; a real producer bridge/identity correction is still required for the visual consumers. Trigger volume and stone-floor generators contain unresolved wrappers and cannot be scheduled as no-ops.
- Canonical core WoodenGate component unification with the render view remains architectural work; complete leaf semantic data is presently owned in the system crate and projected into the existing renderer identity. No fields are dropped, but other future gate gameplay consumers need the same owner.
- Saved legacy/deprecated wall operations9/14/15 and unobserved optional payloads are not claimed restored. This corpus only observes12 variants; preserved failure boundaries must remain explicit. Rectangular/premade malformed/NaN domains need native parity checks beyond the current accepted geometry corpus.
- Timelapse initialization is now directly proven tag2 and relayed to startup; full start/finish MetaCommander chain is not implemented here. See TIMELAPSE-PRODUCER.md.
- T13 feedback capture is explicitly excluded by user and is not a blocker for this round. No excluded capture producer was added.

## Reused original evidence

building-structures/B-core/14075f680.{c,asm.txt}; building-gates/findings.md and frozen whole-owner/native lifecycle matrices; roof-wire-original.asm; wall-process-tables.json; wall-debug-tags.json; morph-curve-original.asm; retained camera-time/ui-input decomp. Standard library, Bevy/glam and current rhapsody algorithms were not reconstructed.
