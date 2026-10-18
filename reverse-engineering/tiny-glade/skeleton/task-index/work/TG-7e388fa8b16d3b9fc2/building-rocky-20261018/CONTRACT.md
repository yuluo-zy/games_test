# Rocky canonical resource: bounded B recovery

Owner `building-rocky-20261018`, exclusively claimed `TG-7e388fa8b16d3b9fc2` (`resources/rocky_terrain.rs`). This batch edits only a candidate for that current module. It does not edit production, run Cargo, execute original code/JIT, replace the manual startup module, or claim W4/W5 GPU/runtime acceptance.

## Identity and evidence

The current core target is B, per the 2026-10-18 migration; the task index's historical C default is not the target of this work. B `.rne` SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`; matching PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`. `evidence/native-ranges.json` binds 18 primary full PDB-sized ranges and their raw bytes. `verify_evidence.py` checks PE equality, PDB size/name, complete Capstone decode, data references, typed calls and the source patch. This is static evidence, not original behavioral execution.

Primary business ranges: load `1408bbf50` 1292B, Vertex factory `141611ba0` 579B, Triangle factory `1416144a0` 531B. `add_new_with_data` specializations `1412bc1f0`/`1412bba70` are 627B each: they call those exact factories and register keys `1c5c4204bf6436bb`/`2a397285cdc5c9db`. Dependency glue is retained to establish identity, not counted as additional recovered custom business.

## Proven JSON and GPU records

Original file `ljxsj_92385/Tiny Glade/assets/data/rocky_terrain.json`, 30,998,513 bytes, SHA256 `0f63a7250df5a3382017dbdc3652748f33899750e3a65c5aef3727415b9183b7`. `inspect_actual_asset.py` actually checked all records and independently packed the full file using Python struct f32 conversion. Results are in `actual-asset-schema-and-packed-fixtures.json`.

The old DTO's `extra:[f32;6]` / `indices:[u32;4]` cannot deserialize this actual asset. The candidate restores logical fields, without requiring native DTO layout:

- 148,794 vertices: `position:[f32;3], cell_bby:f32, cell_id:i32, dir_to_centroid:[f32;3], is_corner:i32`.
- 49,598 triangles: `neighbours:[i32;3], is_top:i32`.

Original load `1408bc090..0d4` writes GPU words `[position.xyz bits, cell_bby bits, position.xyz bits AGAIN, cell_id, dir_to_centroid.xyz bits, is_corner]`. Integer fields are original signed i32, preserved as raw u32 words. The field-binding addendum directly binds JSON keys to the original source offsets and signed serde calls; all-positive asset values alone were insufficient proof. Original `1408bc1b0..238` writes `[neighbours as raw u32, is_top, 0,0,0,0]`; the last 16 bytes are explicitly zeroed with XORPS/MOVUPS, including the odd-record tail.

Full packed vertex hash: `d91d4d1d4422a7552974e21da127a7a671b2c64771c17cb0e2f58c8682e1499c` (7,142,112 bytes). Triangle hash: `dbbbb6f464fc08f71ed6cf013c4e5a25ea2b7b5d994b00b2e2a3703d87996a1d` (1,587,136 bytes). Selected beginning/middle/end and first negative-neighbour records are hardcoded into the proposed actual-asset Rust regression. The Python check is not a claim that that Rust test has run.

## Canonical factories and library

| current canonical type | NAME | records | record bytes | fixed bytes | usage / memory / alignment |
|---|---|---:|---:|---:|---|
| RockyTerrainVertexSsbo | Rocky Terrain Vertices |148794|48|0x6cfae0|0x22 / 1 / 4|
| RockyTerrainTriangleSsbo | Rocky Terrain Triangles |49598|32|0x1837c0|0x22 / 1 / 4|

For Some bytes, original validation order is length divisible by record size, pointer aligned to 4, then length strictly less than fixed bytes + one record. Thus exactly capacity is allowed. None forwards no initial upload. Creation failure keeps existing panic behavior. These are typed, nonmapped storage buffers; the old zero-capacity/readback constants are wrong.

Current `TypedStorageBuffer` fixed-capacity branch already implements that contract. The candidate uses it unchanged, preserves the current two canonical element types, their SsboType storage, Pod records, AssetSsboLibrary registration and full type-name checks. It adds no buffer engine or duplicate carrier. `evidence/dependencies` captures the current reused sources and their hashes. Library mutation remains vertex upload/register before constructing and uploading triangle output, matching load `1408bc0f8` before `1408bc101..241`.

## Count API and compatibility

Original successful load returns vertex count (`eax=0, rdx=rsi` at `1408bc2d0/2d2`); startup puts that value into one 8B Commands payload, then `1412d7aa0` advances eight bytes and calls `insert_resource<RockyTerrainMeshVertCount>` `14205e980`. The candidate adds a single canonical `Resource` `RockyTerrainMeshVertCount(pub usize)`, with no invented Default.

The current manual `RockyTerrainMesh::load -> anyhow::Result<Self>` and retained CPU vectors stay available. `vertex_count()` exposes their real length, and consuming `into_vertex_count()` yields the count while dropping those vectors. The existing `triangle_groups` compatibility field becomes vertices.len()/3; original W4 needs this derived value but the actual shared count remains full vertices.len(). Current repository search found no external callers/literals of the old DTO/mesh fields; necessary W4/W5 owner handoff is separate and uses this canonical count rather than a CPU Vec layout view.

IO/JSON parsing still uses current fs + serde + anyhow and propagates those errors. This batch does not reconstruct serde internals or assert every malformed/unknown-field visitor policy of the original compiler. See FIELD-BINDINGS.md for the 11-range derived schema evidence; no dependency visitor is manually reimplemented. See STARTUP-OWNER-HANDOFF.md for real schedule, relative asset path, typed library and outstanding integration.

## Validation and boundaries

Five new source regressions are proposed: signed i32 JSON fields and overflow rejection; raw packer bit preservation/negative neighbours/zero pad; actual typed factories with None and exact mock device descriptors/names/keys; invalid Some input before device invocation; and the complete original asset through real AssetSsboLibrary with selected original-record expectations and captured exact uploaded bytes. The last test intentionally requires the separately hash-bound B asset at the existing workspace location; it does not silently skip and does not embed a fabricated replacement file.

Executed here: full-asset Python schema/packing, original PE/PDB/ASM static verification, rustfmt syntax/check, base/candidate patch check. Not executed here: Cargo/Rust tests, native/JIT behavior, full Startup, full original startup_terrain, main-chain W4/W5 rendering or GPU completion. Parent owns integration and compilation. Do not release the claim before integrated verification.
