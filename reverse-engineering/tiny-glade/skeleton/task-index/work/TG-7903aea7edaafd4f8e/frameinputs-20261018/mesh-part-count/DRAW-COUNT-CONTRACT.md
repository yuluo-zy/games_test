# Nani raw draw count — current B

The complete original `nani_draw@140904ae0`1428B, external constructor@1409912a0 21B, and Owned Placement constructor@1415673b0 1425B are saved here. `native-manifest.json` binds their bytes and PDB symbols to original B SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

The external constructor writes its R8 argument to payload+8. The owned constructor saves R8 at1415673d6, writes live instance count0 at1415677f4 (+a0), and writes the saved argument at1415677ff..806 (+a8). The master startup caller supplies the mesh-layout count, and the current GpuWorld owner now exposes that value independently through `mesh_part_count`.

`nani_draw@140904c4a..c5c` chooses these count fields according to the original variant and reads a u64. The raw branch forwards it as the fifth `draw_indexed_indirect_raw` argument. Live instance count controls the generation work elsewhere; substituting it here changes the number of mesh-part indirect commands. Returning0 for the external variant suppresses its raw draw entirely.

The production adapter now uses `owned.mesh_part_count()` and `external.mesh_part_count`; count-based sorting, offsets, stride20, and the existing RenderGraph/DrawBuilder APIs were retained. No GpuWorld or rhapsody internals were edited by this task.

The actual DrawBuilder regression uses explicit unequal fixture values (live3 / mesh7) and requires the raw packet count7. A second real packet checks external mesh count5. These CPU packet assertions are not GPU execution claims. See `applied-source-manifest.json` for the frozen source hash and test filters.
