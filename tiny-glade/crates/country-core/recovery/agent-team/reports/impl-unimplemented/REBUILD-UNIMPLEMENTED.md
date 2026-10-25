# REBUILD-UNIMPLEMENTED — country-core 未实现(无 body)分批收口（impl-unimplemented）

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类：宏胶水 vs 真业务）
依据: reports/interaction/collision/{LOGIC,REBUILD,REBUILD-DEEP,REBUILD-REMAINDER}.md、
      reports/eco/walls/REVIEW-DEEP-WALLS.md、reports/state/history/REBUILD-REMAINDER.md、
      reports/interaction/ui_mode/LOGIC-UI-REMAINDER.md、reports/interaction/render_bridge/REBUILD-REMAINDER.md。
红线: 只以 fresh 反编译为准；缺类型不编造；不改他模块；宏胶水标记不重建。
证据: planning/impl-unimpl/results/b1-update-rapier.result.json（本次 fresh，2026-10-17 12:00）。

## 方法

每函数 fresh 反编译（ghidra `--worker shared` 只读，每批 ≤20）→ 分类（宏胶水 / 真业务 /
存疑）→ 域内类型已具备则实现，缺类型则登记（登记缺类型清单，不编造）。产出本文件分批。
编译：`cargo check -p country-core --lib` 全树被并行 rhapsody WIP 阻塞（51 E ，
非本模块）；本批实现文件用独立探针复验（见各节）。

---

## 批 1 — collision monolith 余（world.rs + intersect_meshes + visualize + resources/rt）

### 1.1 fresh 反编译：update_rapier_colliders（0x1408d0490）

本次 fresh（ghidra shared，`get_functions fields=signature,decompiled_code,callees`）：
**`decompile_failed=True`**，`decompile_error = "Recoverable Error: Unable to find unique hash
for varnode"`（与 half_timbering 同款硬点，AI_REVERSE_TOOLS.md §7 已登记）。函数 22401B、
41 callees（callees 已落盘）。**callee 面（结构佐证）**：
`construct_arch_geo`(0x1408d7ed0)、`new_indexed`/`new_unindexed`(0x1408cee40/0x1408cf7d0)、
`calculate_hash<[UVec3;3]…>`(0x14145a540)、`upsert_collider_entry<Entity,(Vec3,Quat)>`
(0x140b40b50)、`dispose_collider_mesh_instance`(0x1408d9590)、`get_collision_geo`(0x1408e8d70)、
`floor_mesh::closure_env$0`(from_iter)、`as_points2/3`(circle)、`StoneColumn` Vec reserve、
`roof_trigger_volume::triangles`。
**结论**：真业务 monolith（collider 生命周期：取碰撞几何 → 建 RaycastMeshData → arch 几何 →
upsert/dispose → 地板/石柱/屋顶触发体积）。**登记为 decompile-hardpoint**，恢复路径：
专属 write worker 修 PDB `void()` 占位签名后定点重取（同 half_timbering 已攻克路径）；
分段落盘逐分支重建。

### 1.2 分类与登记（本轮，全部真业务）

| VA | 函数 | 分类 | 处置 | 缺类型/登记原因 |
|---|---|---|---|---|
| 0x1408d8710 | `RaycastWorld::on_level_reload`（1993B） | 真业务 | 登记 | fresh 已解：drop 旧 `MeshInstanceSources`@+0x3dc8（0xe0B）→ 17 槽 `default<RapierSubWorld>` 重建 → 发布器复位 → +0x10/+0x1c 计数器写。`MeshInstanceSources` 类型缺 + 精确偏移未锚定（`RaycastWorld` 现为语义模型） |
| 0x140a97b70 | `find_mesh_mesh_intersections`（13359B） | 真业务 | 登记 | 遍历 mesh 对 + 建图边；依赖 collider 存储 / `RaycastMeshData` 布局 / `MeshIntersectionGraph` 装配 |
| 0x1408f2850 | `merge_segments_into_polylines`（6057B） | 真业务 | 登记 | fresh 全量已解：`HashMap<IVec3,VertexId>` 端点查表 + `Vec<Polyline>`（0x28B/条）+ `VecDeque<PolylinePoint>`（0x18B）+ 端点中点平均 + 跨 polyline 拼接/闭环。缺 `Polyline`/`VertexId`/`PolylinePoint` 类型 + `from_iter<PolylinePoint,2>` 初建语义 |
| 0x1408d0390/0x140b26a80/0x140b26bd0 | `to_offset_collider_builder` impl$23/21/22 | 真业务 | 登记 | `IsoTransform::from_translation_rotation`/`nested_in` + shape Arc(+0x20) 增计数 + `ColliderBuilder::new` 装配。缺 `OffsetColliderTransform`/`IsoTransform` 类型；impl$41/impl$42 输出偏移**不一致**（+0x1c/+0x24 vs +0xc/+0x14），布局未锚定，不编造 |
| 0x14155d340/0x1408d98c0/0x1408d9960/0x140b26d20 | `into_transformed_shape` impl$41/42/43/45 | 真业务 | 登记 | Rectangle3d 版：`extent()`→半边长 + `Quat::from_rotation_arc` + xvy/x0y swizzle；impl$42：`ColliderSet::get_mut_internal` 拷贝 shape Arc。同上缺类型 |
| 0x1408d9990 | `as_collider_shape` impl$47（26B `return;`） | 访问器 | 登记 | 取 OffsetColliderTransform 的 shape；类型缺 |
| 0x14129b780/0x14129bad0/0x140254890/0x140254aa0 | `iter_shape_overlaps` ×4 | 真业务 | 登记 | fresh 已解：`into_transformed_shape` + `Rapier::subworlds` + mask 位门 + `Qbvh::traverse_depth_first_node` + `intersections_with_shape` closure。依赖上两族类型 + 泛型 closure |
| 0x142612d10 | `shape_overlaps_with<Rectangle3d>` | 真业务 | 登记 | fresh 已解：`into_transformed_shape` + subworld mask + `QueryPipeline::intersection_with_shape`。依赖同族 |
| 0x14129be00 | `decorator_overlaps_with_others` | 真业务 | 登记 | 依赖 `OffsetColliderTransform` + `DecoratorRank` |
| 0x141326370 | `visualize_colliders`（932B） | 真业务 | 登记 | fresh 已解：17 槽 collider 遍历 + 注册表 (index,gen) 校验 + DebugGui +0x104 kind 过滤（case 0/2/3/4/5/6/8/0xd/0xe/0xf/0x10）+ `visualize_shape`。依赖 `DebugLine::show`/`DebugSphere::show` ECS 胶水（debug_viz.rs 已登记缺 show） |
| 0x141326930 | `visualize_shape`（1747B） | 真业务 | 登记 | fresh 已解：as_compound(递归+复合变换)/as_trimesh(closure_0 线框)/as_convex_polyhedron(to_trimesh+closure_1)/as_cuboid(box_trigger_volume)/as_ball(DebugSphere::show)/不支持 panic。同上缺 show 胶水 + `box_trigger_volume`（wall 域） |
| resources/rt（13） | mod.rs 业务 11 + bvh 1 + bvh_builder 1 | 真业务 | 登记 | fresh 已解（b8-closures-visualize-rt.json：upload_rt_world 0x1408bcff0 / update_rt_world 0x1408bc650 / build_gpu_bvh 0x140b0da50 / bvh_builder_worker 0x140b24d50 / DirtyCow::get_mut 0x1415626f0 等）。缺 `RtWorld`/`RtMesh`/`DirtyCow`/`RtBvhBuilderGlue`/`RtMeshHandle`/`GpuRtTriangle`/`RtBvhSsbo` 等类型族 |

### 1.3 批 1 汇总

- 实现：0
- 登记：33（表中 32 个业务函数 + `update_rapier_colliders` 单独登记 decompile-hardpoint）
- 缺类型清单（批 1）：`OffsetColliderTransform`、`IsoTransform`（utils）、`MeshInstanceSources`、
  `Polyline`/`VertexId`/`PolylinePoint`（intersect_meshes）、`RtWorld`/`RtMesh`/`DirtyCow`/
  `RtBvhBuilderGlue`/`RtMeshHandle`/`GpuRtTriangle`/`RtBvhSsbo` 族；`DebugLine::show`/`DebugSphere::show`
  胶水；`MeshIntersectionGraph` 装配。

---

## 批 2 — UI 域 grading_lut 5

| VA | 函数 | 分类 | 处置 |
|---|---|---|---|
| 0x140b00b20 | `ColorGradingState::flattened` | 真业务 | **实现**（见下） |
| 0x140b00bc0 | `GradingLut::load_unwrapped`（PNG 解码） | 真业务 | 登记（`image::Reader::open/decode→to_rgba8→TextureDesc::new_3d.sampled→GraphicsDevice::create_texture→unwrap`；需 image 0.25.1 已入 workspace 的真实接线 + rhapsody 设备，country-core 现用 `TextureDevice` 抽象，未接） |
| 0x140b01df0 | `GradingLuts::load`（readdir 扫描） | 真业务 | 登记（loader closure 0x140ae5bb0 未反编译） |
| inline | `lut_name_to_asset_name` / `asset_name` | inline | 登记（inline 宿主；消费方 `final_post` 已自带 `grading_lut:` 前缀） |

**实现 — `ImageFormationConfig::flattened`**（fresh 0x140b00b20，src/resources/image_formation.rs）：
- `flag_28==0`：克隆名 + 拷贝 +0x18..+0x24 四 f32；`flag_28!=0`：名 `"default"`（len/cap 7）+
  参数 `{1,0,1,1}`；两分支输出 +0x28=0（bypass 恒清）。
- 尾部（+0x30 镜头/+0x38/+0x3c/+0x3d）原生不写（属调用方栈未初始化区，非 ColorGradingState），
  保留 `self` 原值作语义模型（登记）。
- 测试 `flattened_resolves_bypass_to_concrete_params`（独立探针编译 + 单测 PASS）。
- `grading_lut.rs` 模块头已把 flattened 从未恢复清单移入。

批 2 汇总：实现 1、登记 4。

---

## 批 3 — UI 域 prefab 16

LOGIC-UI-REMAINDER.md 未实现(无 body)登记 16 项：`PrefabLibrary::{add,get,has,iter}`、
`impl$7::register_required_components`、`systems::prefab::impl$0::build`、`impl$3::clone_behavior`、
`impl$3::register_required_components`、`spawn_prefabs`、`create_prefab_rt`、
`create_prefab_audio_trigger`、`create_prefab_gpu_instances`、`update_prefabs`、`despawn_prefabs`、
`despawn_prefab_contents`、`hot_reload_prefabs`。

**处置：全部登记。** 依据：`src/resources/prefab.rs` + `src/systems/prefab.rs` 仅落类型合同
（`PrefabInstance`/`PrefabInstanceData`/`SpawnPrefabCmd`/`DespawnPrefabCmd`），资产加载/实例化/
回收宿主系统无 body；**country-core `asset_libraries/` 层无 `PrefabName`/`PrefabLibrary` 类型**
（已核对 ssbo/mesh/texture/vao/placement_data 等 9 模块，无 prefab 域）。任务规则
“若 asset_libraries 层类型已在 crate 则实现，缺则登记依赖”——缺，登记依赖
（`PrefabLibrary`/`PrefabName` 资产系统，forest/tree-tool 消费面已由别组接线）。
`impl$3::clone_behavior`（0x140002370，3B return）与 `create_prefab_gpu_instances::closure$0`/
`hot_reload_prefabs::closure$0` 为宏胶水/closure，标记不重建。

批 3 汇总：实现 0、登记 16（含胶水 3 标记）。

---

## 批 4 — history 跨模块 5

reports/state/history/REBUILD-REMAINDER.md 已 fresh 定案并登记（本轮复核维持）：

| 函数 | 处置 | 缺他域类型 |
|---|---|---|
| `replay_pop_bucket`（0x140b38250） | 登记 | `DecoratorArchivist::extract_diff` + `RaycastWorld::flush_colliders` + `ReplayBucket` aux diff Arc |
| `DecoratorHistoryState::from`（0x140b3f930） | 登记 | src `DecoratorInfo` 无 +0x41 style 字节字段；`DecoratorHistoryState` 需扩 ~0x4fB |
| `replay_edit` Color/Decorator（0x140af6170） | 登记 | `ReplayBucket::expand` 缺 Color(9)/Decorator(10/11) 命令类型 |
| `replay_wall` 17 操作（0x140951b80/0x140abfb30） | 登记 | `WallHistoryEdit` 枚举变体主体在 recovered_building |
| `ColorEdit` 互异载荷类型面（0x14003c5d0） | 登记 | WallId/TerrainStrokeId/FlatRoofStyle/StairRailingType/StairSupportType 各域持有 |

批 4 汇总：实现 0、登记 5（均需他域类型，缺则登记）。

---

## 批 5 — render_loop Env36

| VA | 函数 | 分类 | 处置 |
|---|---|---|---|
| 0x1409b7fa0 | closure_env$36（Env36） | 真业务 | **已实现**（见 [REBUILD-RENDER-SSBO.md](REBUILD-RENDER-SSBO.md)） |

现状（2026-10-17 impl-render-ssbo 收口）：`resources/sparkle.rs` 新增 `SparkleSsbo`
（32 B 元素，`SparseSsbo` CHUNK_COUNT=0x300、BUFFER_LEN=0x6000，`SsboType::Storage=SparseInstanceBuffer<Self>`；
fresh 0x1412e5f50/0x14120c710/0x141617a10）；`draw_callbacks_b.rs::env36` 删 `SparkleDrawRangesView` 视图，
改经 `AssetSsboLibrary::get::<SparkleSsbo>` → `SparseInstanceBuffer::to_draw_multi_instanced` 每帧现算 ranges
（反汇编 0x1409b7fa0 的 as_any 接收者即 Sparkle 缓冲；`<RoofTileSsbo>` 符号为 ICF 合并）。
验证：`cargo check --lib` 通过；集成测试 `tests/impl_render_ssbo.rs` 3/3 PASS。

批 5 汇总：实现 1、登记 1（RoofTileSsbo 存储形态归 roof/device 域；SparkleSsbo 写面属粒子域）。

---

## 汇总

| 批 | 范围 | 实现 | 登记 | 缺类型清单（新增） |
|---|---|---|---|---|
| 1 | collision monolith 余 | 0 | 33 | OffsetColliderTransform / IsoTransform / MeshInstanceSources / Polyline 族 / Rt 族；show 胶水 |
| 2 | grading_lut 5 | 1 | 4 | image 真实接线 + loader closure 0x140ae5bb0 |
| 3 | prefab 16 | 0 | 16 | PrefabLibrary / PrefabName（asset_libraries 层） |
| 4 | history 5 | 0 | 5 | DecoratorInfo+0x41 / extract_diff / ReplayBucket aux Arc / WallHistoryEdit 枚举 |
| 5 | render_loop Env36 | 1 | 1 | ~~SparkleSsbo 元素布局 + SparseSsbo 实现~~（已落地，见 REBUILD-RENDER-SSBO.md）；RoofTileSsbo 存储形态归 roof/device 域 |
| **合计** | | **2** | **59** | |

## 编译状态

- `cargo check -p country-core --lib`：全树被并行 agent 的 rhapsody WIP 阻塞（51 E，
  非本模块——历史报告同款，REBUILD-REMAINDER/REBUILD-DEEP 已记录）。grep country-core /
  image_formation 无错误。
- 本批实现（`ImageFormationConfig::flattened`）用独立探针 crate 编译 + 单测通过
  （/tmp/cc-verify-flattened.rs，1 test PASS）；未改他模块。

## 后续批次建议（登记项 → 实现条件）

1. `OffsetColliderTransform`/`IsoTransform` 布局锚定（需 fresh impl$41/42/43/45 + to_offset 全
   序列核对 → 接 to_offset/into_transformed/as_collider_shape/iter_shape_overlaps×4/
   shape_overlaps_with/decorator_overlaps_with_others 共 10 函数）。
2. `Polyline`/`VertexId`/`PolylinePoint` 类型 → 接 merge_segments_into_polylines。
3. `MeshInstanceSources` + RaycastWorld 精确布局 → 接 on_level_reload。
4. update_rapier_colliders：专属 write worker 修签名后定点重取（decompile-hardpoint）。
5. `SparkleSsbo` 元素布局 → Env36 每帧 ranges；`Rt*` 类型族 → resources/rt 13。
