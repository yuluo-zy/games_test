# Round12 wall constructor and command integration

This delivery uses the current country-core and rhapsody resource ownership. No Cargo command was run by this agent. Root owns workspace compilation and full App/GPU validation.

## Implemented

- `country_core::systems::wall::set_wall_style::set_wall_style`: the actual three command streams from canonical color-menu events. Original order is Style, Plaster, Freehand. Authority updates use the existing InnerWalls setters. Each recorded command creates its original concurrent edit group; Freehand creates two ordered groups, native11 and8, and writes its `to` value to both freehand/arch authority. The independent style record/sound flags are respected; silent preview restore does not produce duplicate history or audio. The sound is `PlayWallUiSound`, proven by PDB, and uses the original mapped Curve3 midpoint (u=.5), with the native absent-public-wall origin branch. The old chapter's RequestWallMaterialAudio and helper association are wrong.
- MaterialStats is typed instead of Gap + empty methods. Original current reset, seven budgets, eight counters, seed, pillar arrays, max movement and material selector restored. init_this_update does not implicitly advance previous. The argmax preserves total_cmp, last-wins ties, plain-.01, sewer-before-stairs, and UCOMISS/JAE's unordered gate. Its previous/current publication ownership is still not connected: original advancement must be recovered before audio claims.
- `system_wall_constructor::construct_walls` reexports the canonical core producer; the previous duplicate used point Z as height and dropped all outputs. `from_curve_resolved` now requires full PlainWallProductionInput + WallProductionBranch and delegates the restored production algorithm. It no longer returns an empty vector from invented inputs.
- `WallConstructorOutput.gpu` owns per-(WallId,ConstructorPart) complete revisions and real sparse allocations. Arches and gates retain typed96B output; multiple gate commands for one wall form one complete revision. Gate rebuild/removal clears only its gate part. Lintels use actual prepared positions/U/segment values, actual arch cache, shape!=2 closed policy and real independent third-height writeback; they produce64B records in the same physical WoodenPlankSsbo. Fence boards use actual prepared data, and leaving fence style clears their previous part. Fence post Nani creation remains incomplete.
- `flush_constructor_output` writes through current SparseInstanceBuffer::replace_all and try_gpu_flush, retaining capacity/storage failures and physical device errors. Dropping wall owners queues actual chunk reclamation; deletion never clears unrelated main-wall allocations. AssetLibraryRuntime exposes a read-only buffer_device() getter for this shared physical resource, not a second device.

## Independent verification

`probe_material_stats.py` verifies original B RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03, maps its PE sections unchanged and invokes only original pure entries140a887d0/140a88280. No EXE entry is run, no helper is replaced. Captured520 selector calls +10 initialization calls, including NaN/infinity/ties/-zero. `material-stats-native-cases.json` contains original outputs.

Standalone optimized rustc compilation of the actual production MaterialStats source (no Cargo/dependencies) compared all530 outputs: **530/530 passed**, exit0. `material-stats-check.rs` is the reproducible comparison driver. This proves those two method scopes only, not the game or material audio chain.

Rustfmt parsed changed source (syntax only). Pending root tests: original_native_material_stats_calls_match_all_bits_and_selectors; canonical_commands_mutate_authority_record_groups_and_do_not_rerecord_hidden_restore; style_audio_uses_mapped_curve_midpoint; source_replacement_deletion_and_foreign_storage_keep_real_sparse_ownership; plank_projection_preserves_all_sixty_four_output_bytes. No pending workspace test is counted passed.

## Central integration / remaining boundaries

1. Install set_wall_style after its actual color/UI producer and before publication/history recording. Register the three commands and RecordWallHistory / PlayWallUiSound through canonical resources.
2. Initialize WallConstructorOutput. Run source constructors only once after canonical prepared/cache/mask producers; do not schedule the whole old crate registration. It still contains incomplete platform stairs/pillars/plaster/half-timber wrappers. Core main producer is already independently registered and must not run twice through its new reexport.
3. delete_constructor_output before flush; flush_constructor_output after all selected constructor writers and before W21/W27/Nani. Check output.gpu.failures and last_device_error before considering the frame validated. Main WallBrickUploadStatus alone cannot cover these additional owners.
4. Need actual nonempty authority→segments→gate/arch/lintel→flush→GPU DrawLists/readback acceptance. Tests added here use CPU sparse shadow; they are not GPU acceptance. Root should use strict physical device and compare actual draw counts, plus edit/rebuild/delete/reuse.
5. MaterialStats previous/current advancement and constructor writers, style==2 arch branch, exact finite sparse-pool native partial-output/log policy, fence-post Nani, full corners/skirts/signifiers and other constructor wrappers remain unresolved. Current batch's retained old-geometry capacity policy comes from the existing user sparse API; original finite exhaustion semantics are not newly claimed.
6. T13 remains excluded by user.

Original evidence: retained building-constructor/building-structures/building-gates matrices; fresh `set-wall-style-original.asm`, `material-stats-original.asm`; retained original decomp copied compactly to `set-wall-style-original.c` and PDB FunctionSystem identifies final PlayWallUiSound. No rhapsody algorithm or library ABI was restored.

## Subsequent source audit and correction

The earlier `fence post Nani` gap above is withdrawn: full original141238d80
emits both posts and rails into the64B WoodenPlankSsbo. Current core instances
already include those posts. The real remaining boundary is exact shared finite
pool push/failure policy and statistics, not an invented Nani producer.

See `SOURCE_READINESS.md` for freshly verified original19/22/28/44 registration,
actual current producer/resource gaps and the12-file review-only candidate.
Production remains frozen during central builds; the only authorized intervening
source edit repaired the cross-crate cfg(test) device dependency in gpu_output.
That local test device rejects GPU upload, so CPU shadow tests cannot masquerade
as successful device validation.
