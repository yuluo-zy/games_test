# REVIEW — asset-prefab 收口(REBUILD-ASSET-PREFAB 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: impl-asset-prefab REBUILD-ASSET-PREFAB.md;`cargo check --lib` PASS、相关测试 11 PASS
(协调器基线)。编译注:现全仓 5 error 均在 `systems/collision/intersect_meshes.rs`
(VecDeque::reverse E0599 + E0308,**collision worker 并发在途**,非 asset-prefab 文件);
asset-prefab 文件(prefab.rs/grading_lut.rs/Cargo.toml image)不在错误列表。

## 逐项复验

### ① 类型落地 — PASS
- `PrefabLibrary`(hashbrown HashMap<PrefabName, Prefab>)、`Prefab`(parts: Vec<PrefabPart>)、
  `PrefabPart`(typed gap,原生 0x98 B 布局 U)、`PrefabWatchPlugin`、`PrefabsModifiedCmd`、
  `impl Equivalent<PrefabName> for PrefabNameRef`(按 str 内容哈希/memcmp,O has
  0x141552ff0)——落位现有 resources/prefab.rs(PDB/章节锚定)✓;PrefabName/PrefabNameRef 确认
  已在 resources/render/mesh_atlas_library.rs(不动)✓。
- grading_lut:`UNWRAPPED_SIZE=16`、`UNWRAPPED_TEXTURE_FORMAT=0x25`(VK_FORMAT_R8G8B8A8_UNORM)✓。
- `image.workspace = true`(=0.25.1,B Cargo.lock 锁定)✓。

### ② 实现 6 — PASS
- `PrefabLibrary::add`(insert + 被替换旧值逐 part drop)/`get`(miss → **panic 文案
  "No such prefab {:?}; available: {:?}" 逐字** + keys,read_memory 0x142c6c7a0/7c0)/`has`
  (contains_key)/`iter`(inner.iter)✓。
- `GradingLut::load_unwrapped`:Reader::open_impl→decode→to_rgba8→
  `TextureDesc::new_3d([16;3], 0x25).sampled()`→create_texture("grading lut"),extent/format
  disasm 核对(Err → unwrap panic,原版不转 Err)✓。
- `lut_name_to_asset_name`:helper `format!("grading_lut:{name}")`(O 前缀)✓。

### ③ 登记 21 诚实 — PASS
- systems::prefab 8 业务(spawn_prefabs stride 0x13 qword=0x98B/create_prefab_rt/
  create_prefab_gpu_instances/update_prefabs/despawn_prefabs/despawn_prefab_contents/
  hot_reload_prefabs/reload_prefabs_impl)——需 PrefabPart 0x98B + resources::rt + bevy_app,
  全登记 ✓。
- startup 10(reload_prefabs/load_prefabs_impl/PrefabWatcher/spawn_preplaced_trees/load_shaders/
  load_ui_shaders/load_mesh_atlases/reload_mesh_atlases/load_nani_meshes/…)——需
  asset::prefab::config + PrefabWatcher(notify)+ mesh_atlases×3 + NaniWorldSubsetConfig,登记 ✓。
- grading 2(GradingLuts::load 双 Vec stride 0x28/0x38 U、asset_name inline)——登记不编造 ✓。
- **0x1409e6210 跨界识别**:fresh 实为 history::tree_edit::missing_prefab(109B,
  "MISSING_TREE_PREFAB_NAME" 24B 逐字),归 history/tree_edit 域,交 impl-history-deep——识别
  诚实,不冒充 prefab ✓。

### ④ 胶水 5 不重建 — PASS
register_required_components×2(Bevy 注册)/clone_behavior(3B return)/closure×2——标记
glue-legitimate 不重建 ✓。

## 结论与 DISPATCH 建议

- asset-prefab 收口复验通过:**6 实现(4+2)+ 类型落地 + 21 登记诚实 + 5 胶水不重建**,协调器
  基线 cargo check PASS + 测试 11 PASS。
- 建议:asset-prefab → 闭合(实现层面);登记项(systems::prefab 8/startup 10 需
  PrefabPart 0x98B + resources::rt + asset::prefab::config + PrefabWatcher)归后续卡;
  missing_prefab 交 history 域。现 intersect_meshes.rs 断点系 collision worker 并发在途,
  不属本批。
