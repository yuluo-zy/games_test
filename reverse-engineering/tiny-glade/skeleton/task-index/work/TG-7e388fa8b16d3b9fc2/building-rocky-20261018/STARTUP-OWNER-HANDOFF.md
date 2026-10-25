# Rocky startup input handoff (B, existing startup owner only)

This is a necessary bounded integration contract, not a second startup implementation or permission to replace the manual startup module. The resource candidate changes only its claimed canonical module. Startup/main-chain remain under their current owner.

## Positive original registration: Startup, not PreStartup

`add_game_startup_systems@140941110` is full PDB134B. It registers several different schedules. Distinguish these exact branches:

1. `140941125 -> 140a97110` builds a four-element outer **chain**; `140941130 -> 140b67620` is PDB-typed `SubApp::add_systems<..., bevy_app::main_schedule::PreStartup,...>`. Its nested plain ten-tuple `140a2bf40` includes a Device+SsboLibrary+ConfigWatch+Commands function. That function is **init_color_library**, whose into_configs helper `1413fb780` names it. It is not startup_terrain. An earlier shape-based reading of that tuple as terrain was incorrect.
2. Actual startup_terrain name `country_core::startup::startup_terrain::startup_terrain` is the 55-byte plain name at `142cd7db1`, loaded by LEA `14176a423` inside `into_configs@14176a370` (PDB1708:124236). The earlier same text occurrence inside a FunctionSystem type-name is not the registered name.
3. The actual eight-tuple builder `14099b8b0` calls that helper at `14099baad`; this is **slot4, zero-based**. Its other member configs are siblings; the plain tuple itself is not a chain.
4. `SubApp::add_systems@140b67b50` has exact matching PDB generic `bevy_app::main_schedule::Startup`; it calls `14099b8b0` at `140b67bda`. `add_game_startup_systems` calls it at **14094117f**. Thus this loader's original stage is **Startup**.

The separate conditional OnEnter<AppLoadingState> branch (`14094115d -> 140b66830`) loads a saved session and does not reclassify startup_terrain. These full ranges, matching PDB names and name-reference bytes are frozen. No full-PDB scan or original execution was needed.

## Real asset/library/count contract

Actual startup_terrain has seven inputs: `Res<GraphicsDeviceResource>, Res<AssetMeshLibrary>, Res<MeshWatch>, Res<AssetShaderLibrary>, Res<AssetSsboLibrary>, Res<ShaderWatch>, Commands`. The rocky subsection uses the same canonical device/library as the rest of startup; do not create a private AssetSsboLibrary or second typed buffer identity.

At `140927060` it loads the exact 30-byte slice **assets/data/rocky_terrain.json** from VA142aaefd1, sets len0x1e at 140927067 and calls load1408bbf50 at14092706c. This is relative to the original game's asset root/current working directory. Production integration must use the host's resolved real asset root, not this review workspace's absolute ljxsj_92385 path. The current resource loader already accepts an explicit Path; preserve that useful manual API.

Load succeeds with the real vertices.len() in RDX. `140927071/73` rejects Err into 140927c22 (the original expect/panic path, not a zero-count default). On success, startup stores the FnOnce thunk1412d7aa0 and RDX at command record+8 (`14092709e/0a9`). The thunk advances its payload by eight at1412d7aa5, loads that qword at1412d7ab1 and calls exact typed command apply14205e980. That command's original Resource TypeId immediates are fa44538b95ae76c6 / 12146c4ea814069b. The logical resource is `RockyTerrainMeshVertCount`, not current RockyTerrainMesh's CPU Vec layout.

Minimal existing-architecture insertion at the owner-controlled Rocky subsection:

```rust
use crate::resources::rocky_terrain::RockyTerrainMesh;
// rocky_path comes from the host's existing actual asset-root resolution.
let rocky_count = RockyTerrainMesh::load(&rocky_path, &library, device.device())
    .expect("failed to load rocky terrain")
    .into_vertex_count();
commands.insert_resource(rocky_count);
```

This is an API usage hunk; no proposed new parallel Resource/root resolver or replacement startup system. Keep the host's existing current architecture. Original full startup_terrain is 5382B and also initializes other mesh/UI/shader state; this bounded resource fix does not certify that entire function. The current file presently implements grid helpers only, so the owner must explicitly add/register the missing loader subsection rather than assuming an existing startup_terrain ECS system already calls it. Running a separate partial helper in PreStartup is a current scheduling choice, **not original stage evidence**; avoid that choice when the normal Startup phase is available.

## W4/W5 canonical consumer handoff

Both consumers must use `resources::rocky_terrain::RockyTerrainMeshVertCount` from this one successful load:

- W4 displacement uses full usize count /3 then low u32 for its original push/dispatch. Do not pass triangle_groups into a second divide-by-three.
- W5 rocky draw uses the same count's low u32 for draw_no_index_buffer vertex_count.
- `RockyTerrainVertexSsbo` and `RockyTerrainTriangleSsbo` are the canonical AssetSsboLibrary owners. Replace foreign-carrier/tag placeholders with these exact existing types (reexport/trait impl in current graph architecture), not second structs with the same NAME. The library verifies full Rust type names in addition to short-name hash.
- A borrowed current typed count is allowed; layout-casting RockyTerrainMesh or copying count into an independent mirrored world Resource is not.

Required owner-side integration verification: actual resolved path → real library creates exactly these two buffers with data → deferred count resource inserted by Startup → both real W4/W5 consumers borrow it. Distinguish mock graph/source checks from a completed GPU frame. Keep failures visible; no default count or fake zero asset.
