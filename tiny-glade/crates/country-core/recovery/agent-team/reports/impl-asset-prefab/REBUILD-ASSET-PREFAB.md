# REBUILD-ASSET-PREFAB — asset_libraries prefab 类型落地 + prefab/grading/startup 分批收口

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类：宏胶水 vs 真业务）
依据: reports/impl-unimplemented/REBUILD-UNIMPLEMENTED.md（批2 grading 5 / 批3 prefab 16）+
      reports/interaction/ui_mode/LOGIC-UI-REMAINDER.md（prefab 16 / grading 4 登记项）。
证据: planning/impl-asset-prefab/results/{a1,b1,b2,c1,c2,d1,e1}-*.result.json（本次 fresh 2026-10-17，
      ghidra shared 只读；`get_functions fields=signature,decompiled_code,callees` + `read_memory` +
      `disassemble_function`）。
红线: 只以 fresh 反编译为准；缺类型不编造；不改他模块；asset_libraries 文件已有实现只补 prefab 相关。

## 批 1 — asset_libraries 层 prefab 类型落地

**先核结论（任务第 1 步）**：
- `chapters/asset_libraries/` **无 prefab 模块**（11 个模块全核对）→ 原版 prefab 资产类型**不在**
  asset_libraries；`asset_libraries/mod.rs` 的 `Handle/Asset` 通用层 + 9 个 `*_library` 均为既有实现。
- `PrefabName`/`PrefabNameRef` = 外部 `asset::prefab`（skeleton B `crates/asset/src/prefab.rs`），
  **库内建模已在** `resources/render/mesh_atlas_library.rs`（A3 res_render 按机器码重写，O：
  `PrefabName(Arc<str>)` / `PrefabNameRef(str)` + `as_name_ref` 0x141556130）→ 不改动。
- `PrefabLibrary` 锚定 `resources::prefab`（`chapters/resources/prefab.rs.md` + skeleton A
  `resources/prefab.rs` + `HashMap<PrefabName, Prefab>`）→ 按 PDB/章节落地到**现有**
  `src/resources/prefab.rs`（在 crate 已注册 `resources::prefab`，非新增 asset_libraries 文件）。
- **落地类型**（resources/prefab.rs）：`Prefab`（parts: Vec<PrefabPart>）、`PrefabPart`（typed gap，
  原生 0x98 B 布局 U，spawn_prefabs 0x140840610 part 遍历 stride 0x13 qword）、`PrefabLibrary`
  （hashbrown `HashMap<PrefabName, Prefab>`）、`PrefabsModifiedCmd`、`PrefabWatchPlugin`、
  `impl Equivalent<PrefabName> for PrefabNameRef`（按 str 内容比较，O：has 0x141552ff0）。

**实现（fresh 逐函数判等）**：

| VA | 函数 | fresh 证据 | 实现 |
|---|---|---|---|
| 0x1408e2540 | `add(name, prefab)` | `insert<PrefabName,Prefab,RandomState,Global>` + 被替换旧值逐 `PrefabPart` drop | `self.inner.insert(name, prefab)` |
| 0x141552d70 | `get<&PrefabName>` | 空表直 miss；`as_name_ref`→`hash_one<&str>` SIMD probe + memcmp；miss → `from_iter<Keys>` 收 `Vec<&PrefabName>` + `panic_fmt`（read_memory 0x142c6c7a0/0x142c6c7c0 文案 `No such prefab {:?}; available: {:?}`） | `inner.get(name.as_name_ref())`，miss panic（含 keys） |
| 0x141552ff0 | `has<&PrefabNameRef>` | `contains_key`（str 哈希/比较）→ bool | `inner.contains_key(name)` |
| inline | `iter` | `inner.iter()`（PDB 内联宿主 load_mesh_atlases） | `inner.iter()` |

编译/测试：`cargo check -p country-core --lib` PASS；`resources::prefab::tests` 4 个 PASS
（add/get/has/iter + 覆盖替换 + miss panic 文案 + 空表直 miss）。

## 批 2 — prefab 16（systems::prefab）

fresh 反编译分类（b1/b2）：

| VA | 函数 | 分类 | 处置 | 缺类型/登记原因 |
|---|---|---|---|---|
| 0x140840430 | `PrefabPlugin::build` | 胶水 | 登记 | Bevy 插件注册：`register_event<SpawnPrefabCmd>`+`<DespawnPrefabCmd>`+`insert_resource<PrefabsToDelete>`+`add_systems`（4 系统链）。需 bevy_app 宿主 + 系统已挂 |
| 0x140840610 | `spawn_prefabs` | 真业务 | 登记 | 消费 SpawnPrefabCmd → `PrefabLibrary::get` → 遍历 `prefab.parts`（**stride 0x13 qword=0x98 B**，`@+0x20` 名字 / `@+0x28` kind 判别 `0x65657254=="Tree"` panic `Unknown audio trigger kind in pr…`）→ create_prefab_* + `insert<(PrefabInstance, PrefabInstanceData)>`。需 `PrefabPart` 0x98 B 布局（asset 域） |
| 0x140841550 | `create_prefab_rt` | 真业务 | 登记 | `Mesh::positions`→Aabb3 extent/center + `sinf/cosf` 旋转 → `RtMesh` 库 `get_mut<RtMeshHandle>`。需 `resources::rt`（RtMeshHandle/DirtyCow<RtMesh>） |
| 0x140841b00 | `create_prefab_gpu_instances` | 真业务 | 登记 | `MeshAtlasLibrary::get_by_name<&PrefabNameRef>` → `GpuWorld::create_instances<PlacementDataSsbo,closure_env$0>`（create_instances 已在 render/nani/gpu_world.rs） |
| 0x140841d60 | `update_prefabs` | 真业务 | 登记 | `get_changed_ticks_slice_for` 改检 → `sinf/cosf` 旋转 → `RtMesh` `owned_subset_mut` + `Aabb3::scaled/translated` → `update_instance_data<PlacementDataSsbo>`。需 resources::rt |
| 0x140842d60 | `despawn_prefabs` | 真业务 | 登记 | 消费 DespawnPrefabCmd（event 槽 0x10 B）→ 每实体 `despawn_prefab_contents` + `EntityCommands::despawn`。依赖下项 |
| 0x140843390 | `despawn_prefab_contents` | 真业务 | 登记 | `PrefabInstanceData` 内 3 个 Vec：GpuWorld `remove_instance` 组（stride 0xc）+ `RtMesh` `remove_entry`/`drop_slow<RtMesh>` 组 + audio 组。需 PrefabInstanceData 全布局 + resources::rt |
| 0x140843740 | `hot_reload_prefabs` | 真业务 | 登记 | `PrefabLibrary::get` + `create_prefab_rt`/`create_prefab_gpu_instances` + `despawn_prefab_contents`；`HashSet<PrefabName>` + `HashSet<GpuWorldSubsetKind>` 变更追踪。需 PrefabWatcher（notify）+ resources::rt |
| 0x1408446b0 | `impl$3::register_required_components` | 胶水 | 标记 glue-legitimate | Bevy 组件注册（register/insert ComponentInfo 内联，无业务体） |
| 0x1408e5d10 | `impl$7::register_required_components` | 胶水 | 标记 glue-legitimate | 同上（PrefabInstance/PrefabInstanceData 注册） |
| 0x140002370 | `impl$3::clone_behavior` | 宏胶水 | 标记不重建 | 3B return |
| inline | `create_prefab_gpu_instances::closure$0` / `hot_reload_prefabs::closure$0` | 胶水 | 标记不重建 | closure |

**0x1409e6210 识别**（任务指定 fresh 区）：fresh 反编译实为 **`country_core::resources::history::tree_edit::missing_prefab`**
（109 B，`__rust_alloc` + `handle_error`），写 String "MISSING_TREE_PREFAB_NAME"（24 B，三个 qword 逐字核对）。
属 **history/tree_edit 域**（impl-history-deep 范围），不在 prefab 域——登记跨界识别，交 history 域接线。

## 批 3 — grading_lut 4

| VA | 函数 | 分类 | 处置 |
|---|---|---|---|
| 0x140b00bc0 | `GradingLut::load_unwrapped` | 真业务 | **实现**（见下） |
| 0x140b01df0 | `GradingLuts::load` | 真业务 | 登记（闭包 0x140ae5bb0 已 fresh：`DirEntry::path`→`extension`→`file_stem`→分派 "cube"(len 4)/"png"(len 3)，Err → `log::error!("country_core::resources::grading_lutFailed parsing LUT : …")` 0x142ae32b0，未知扩展 log+跳过；输出**双 Vec stride 0x28/0x38**，元素语义 U——不编造） |
| inline | `lut_name_to_asset_name` | inline | **实现**（helper `format!("grading_lut:{name}")`，O 前缀 0x142ae6c70 侧；PDB inline 依赖 `format!`+`map_or_else`） |
| inline | `asset_name` | inline | 登记（inline 宿主 lut_gallery@78；`map_or_else(lut_name_to_asset_name, …)`，签名 I；消费方 final_post 已自带 `grading_lut:` 前缀） |

**实现 — `load_unwrapped`**（fresh 0x140b00bc0 + disasm）：
- `image::io::Reader::open_impl`（0x140ba3a10）→ `Reader::decode<BufReader<File>>`（0x14091bbb0）→
  `DynamicImage::to_rgba8`（0x140bad8d0，drop 原图 0x140b00010）。
- `TextureDesc::new_3d([16;3], 0x25)`（disasm `MOV R14,0x1000000010`+`[RBP+0x58]=0x10` → extent 三轴 0x10；
  `MOV R8D,0x25` = `VK_FORMAT_R8G8B8A8_UNORM`）→ `.sampled()` → `create_texture("grading lut", …)`（名长 0xb）→
  Err `unwrap` panic（原版不转 Err）。
- 依赖：`image = "=0.25.1"`（workspace 原依赖，B Cargo.lock 锁定；country-core 补 `image.workspace = true`）。
- 测试 3 个 PASS（PNG→16³ 纹理 extent/format/数据长、缺文件 Err、前缀 helper）。

## 批 4 — startup 资产绑定类

| VA | 函数 | 分类 | 处置 | 缺类型/登记原因 |
|---|---|---|---|---|
| 0x1412a1a60 | `reload_prefabs` | 真业务 | 登记 | Mutex 守卫（lock_contended/is_zero_slow_path）+ 重入检查 → `load_prefabs_impl`。需 PrefabWatcher（notify） |
| 0x1412a1df0 | `load_prefabs_impl` | 真业务 | 登记 | 大 monolith：`assets/prefabs`（0xe）→ RON `asset::prefab::config::PrefabConfig` → `load_prefab_mesh`（appear_pos/prim_center/prim_normals/Vertex_* 提取 + `MeshProcessorConfig` panic "Unknown mesh code processor"/"Sholud not exist"）+ `filter_mesh_by_vertex_color`×2 + `PrefabLibrary::add` + 依赖图 `HashMap<PathBuf, Vec<PrefabName>>`（add_file_to_prefab_dependency）+ `recalculate_canopy_quad_attribs`（HashMap<[u32;3],Quad>）。需 asset::prefab::config 接线（asset crate 已有 config.rs）+ Mesh 属性管线 |
| 0x1412a8e00 | `PrefabWatcher::new` | 真业务 | 登记 | `notify::new_with_custom_delay`（100 ms）+ `Mutex<HashSet<GpuWorldSubsetKind>>`；**原版用 notify crate**（非 reconstruction 的 hotwatch 猜测）——依赖决定待 host |
| 0x1408e4b00 | `PrefabWatchPlugin::build` | 胶水 | 登记 | `insert_resource<PrefabWatcher>` + `register_event<PrefabsModifiedCmd>`+`<MeshAtlasesModifiedCmd>` + add_systems |
| 0x140862270 | `spawn_preplaced_trees` | 真业务 | 登记 | `from_str<Vec<PlacementConfigEntry>>`/`from_str<Vec<PlacementDataSsbo>>`（trees.json）+ spawn `(ConvertedPreplacedTree, AutoClutterObject)`。需 `startup::clearing::PlacementConfigEntry` + components::ConvertedPreplacedTree |
| 0x140aab0c0 | `load_shaders` | 真业务 | 登记 | 13× `load_shader_into_library<&str>`（helper 已在 startup/mod.rs）；需完整 shader 名单 + 每着色器 `RasterPipelineDesc`（render 域） |
| 0x140aaaf80 | `load_ui_shaders` | 真业务 | 登记 | 6× `load_shader_into_library<&str>`，同上 |
| 0x14129f400 | `load_mesh_atlases` | 真业务 | 登记 | procedural `generate_roof_floor_meshes`/`generate_curved_halftimber_meshes`（FlatMap Range 点阵 + grid_gen_ccw/cw + set_attribute）+ `MeshAtlasLibrary::add<&str>/add<PrefabName>` + `load_nani_meshes<closure_env$0>`（`sinf`） |
| 0x1412a0820 | `reload_mesh_atlases` | 真业务 | 登记 | `clear_atlas_by_name<&PrefabName>`/`clear_world_subset_atlas` + 重载；需 MeshAtlasLibrary（库内已有）+ PrefabWatcher |
| 0x1415f5b60 | `load_nani_meshes<closure_env$1>` | 真业务 | 登记 | `from_reader_seed<Vec<NaniWorldSubsetConfig>>`（RON）+ `load_json_as_mesh` + `watch_atlas` + MeshAttribConfig。需 NaniWorldSubsetConfig/MeshAttribConfig 类型 |

## 汇总

| 批 | 范围 | 实现 | 登记 | 胶水标记 |
|---|---|---|---|---|
| 1 | asset_libraries prefab 类型落地 | 4（PrefabLibrary add/get/has/iter） | 0 | 0 |
| 3 | grading_lut | 2（load_unwrapped、lut_name_to_asset_name） | 2（GradingLuts::load、asset_name） | 0 |
| 2 | prefab 16 | 0 | 8（build/spawn_prefabs/create_prefab_rt/create_prefab_gpu_instances/update_prefabs/despawn_prefabs/despawn_prefab_contents/hot_reload_prefabs）+1（missing_prefab 跨界识别） | 5（register_required_components×2、clone_behavior、closure×2） |
| 4 | startup 资产绑定 | 0 | 10 | 0 |
| **合计** | | **6** | **21** | **5** |

**类型落地**：`PrefabLibrary`/`Prefab`/`PrefabPart`/`PrefabsModifiedCmd`/`PrefabWatchPlugin` +
`Equivalent<PrefabName> for PrefabNameRef`（resources/prefab.rs）；`UNWRAPPED_SIZE`/`UNWRAPPED_TEXTURE_FORMAT` +
`load_unwrapped` + `lut_name_to_asset_name`（resources/grading_lut.rs）。`PrefabName`/`PrefabNameRef`
确认已在 resources/render/mesh_atlas_library.rs（不动）。

**关键缺类型清单（登记项 → 实现条件）**：
1. `resources::rt` 族（RtMeshHandle/DirtyCow<RtMesh>/RtMesh）→ create_prefab_rt/update_prefabs/despawn_prefab_contents/hot_reload_prefabs。
2. `PrefabPart` 0x98 B 布局（asset::prefab 域）→ spawn_prefabs。
3. `PrefabWatcher`（notify crate）+ bevy_app 宿主 → reload_prefabs/load_prefabs_impl/hot_reload_prefabs + PrefabWatchPlugin::build。
4. `GradingLuts` 双 Vec（stride 0x28/0x38）元素语义 → GradingLuts::load。
5. `startup::clearing::PlacementConfigEntry`/`ConvertedPreplacedTree` → spawn_preplaced_trees。
6. load_shaders 全名单 + RasterPipelineDesc、NaniWorldSubsetConfig/MeshAttribConfig → load_shaders/load_mesh_atlases 族。

## 编译状态

- `cargo check -p country-core --lib`：**PASS**（0 errors；9 warnings 均为既有他模块警告）。
  并行 rhapsody WIP 阻塞已消；历史 decorator_edit 2 错为并行 history agent WIP，已随其落库消解，非本模块。
- 测试：`cargo test -p country-core --lib -- prefab:: grading_lut::` **11 PASS / 0 FAIL**
  （prefab 4：库/替换/miss 文案/空表；grading 7：load_cube×4 + load_unwrapped×2 + lut_name_to_asset_name）。
- 新增依赖：`image = { workspace = true }`（=0.25.1，png/jpeg/tiff，workspace 已锁定）只进 country-core Cargo.toml。
