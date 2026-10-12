# G6 取证：场景数据 / 程序化生成 compute pass

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 方法：`run-lanes --lanes 2`（database `ccscene`）。
- 产物：`job-g6-scene-passes.json` + `REPORT-g6.json`（30 个 pass）、`job-g6-lib.json` + `REPORT-g6-lib.json`（14 个底层访问方法）、`decomp/`（29 份）、`decomp-lib/`（14 份）、`REPORT-g6-lane2.json`、`REPORT-g6-lib-lane1.json`。
- **注意**：lib job 复用 database `ccscene`，**覆盖了 lane1 报告**；合并报告 `REPORT-g6.json` 仍含全部结果。
- 仅取证，未改任何代码。

## 0. ⚠️ 父级复核：G6 的**证据**不在保留文件中，但其**结论**已由 G8 独立佐证

G6 报告称"c1 表记「write→0x37」，本次直测**普通 write=0x32**、并发 write=0x37"，并把它当作对 C1 的**更正**。

**第一步核查（属实）**：该说法在 G6 自己的保留证据中**找不到支撑**。

- `g6-scene-passes/decomp-lib/` 中 `add_resource_access`(`0x1409dca90`) 的**全部**出现，只有 `0x140905260.c:132`/`:152` 两处，且都是**透传**：`FUN_1409dca90(param_2 + 0xc, resid, param_4, 1|0)` —— 码来自 `param_4`，**不是立即数 `0x32`**。
- `REPORT-g6*.json` 中 `grep "1409dca90("` 结果同上，**没有任何 `0x32` 立即数**。
- G6 声称承载该结论的两个函数 **`0x140975320` / `0x140975e40` 根本没有被反编译**（不在 `decomp-lib/` 清单中）。

**第二步核查（父级更正自己）**：**G8 用 PE 扫描 167 个 `add_resource_access` 调用点，独立得出"访问器：读 = `0x1a`/`0x1f`、写 = `0x32`、并发写 = `0x37`"。**

⇒ **G6 的实质结论（写 = 0x32）成立**；G6 错的是**框架** —— 它把这个当成"对 C1 的更正"。实际是**两套不同 API**：

| API | 码 |
|---|---|
| builder `write_rg_storage_buffer`(`0x14096dd60`) | **`0x37`**（保留伪代码，C1/A2 正确） |
| accessor `write` | **`0x32`**（G8 独立扫描佐证） |
| accessor `write_concurrent` | **`0x37`** |

**不构成矛盾。** 详见 `../g8-semantics/findings.md` 第 4 节。

## 1. 访问机制（G6 实测，见 `decomp-lib/`）

pass 登记 = `ComputePass::new`（7 参 `0x1416032a0` / 6 参 `0x1405876a0`）→ 若干 `read_storage_buffer` / `write_storage_buffer(_concurrent)` / `bind_texture_to_compute_pass` / `UniformBinder::write_uniform` → `dispatch_2d` / `dispatch_with` → `PassBuilder::render`(`0x1409dd4d0`)。资源用 **TypeId** 经 `Components::get_resource_id`(`0x1400dd6f0`) 解析。

**G6 声称的访问码映射（`0x1a`/`0x32`/`0x37`/`0x34`/`0x19`）中，只有落在保留伪代码里的部分可采信**；`0x32` 项见第 0 节。

（`dispatch_2d` / `dispatch_with` 只有分派、**无资源访问码** —— 此项与既有结论一致。）

## 2. `upload_raster` × 6

`upload_world_raster::upload_raster_staging_to_texture` 的 **6 个单态**。每个查**唯一** TypeId、`new`（6 参/单名）+ `read_storage_buffer` + `dispatch_2d`。

| # | TypeId | 地址 |
|---|---|---|
| 1 | `0x85c987d23d58d2de` | `0x1403D8BD0` |
| 2 | `0x6dc90dcfd23786fa` | `0x1403D90F0` |
| 3 | `0x1ad182936286a0ee` | `0x1403D9610` |
| 4 | `0x979ffa6288aa259a` | `0x1403D9B30` |
| 5 | `0xbb8761bcc7907109` | `0x1403DA050` |
| 6 | `0x86486f3e82eb6bbe` | `0x1403DA570` |

后两组 TypeId 与 `water_rasterize_from_cpu` / `garden_rasterize_from_cpu` **相同** ⇒ 分别对应 water / garden 栅格（观察）。
结合源码共 **6 个 `MappedStorage *RasterStagingSsbo`**（Path / Cobblestone1/2/3 / Water / Garden）⇒ **推断**六次上传即这 6 张 CPU 栅格 staging → GPU 纹理（**逐索引命名属推断**）。

## 3. 地形链

- `compute_path_masks`(`0x1409014E0`)：读 **5 张**栅格（`0x85c987…`/`0xbb8761…`/`0x86486f…`/`0x6dc90d…`/`0xcda6af…`），注册 **5 个** pass 产路网掩码；
- `rasterize_terrain_stroke`(`0x1408C7120`)：走另一路——查 `PassResources(0x47c94cb78283a72e)` + `0x1e38c16726d0fb8d` + `0xcda6af2f6afb130e`，注册 **3 个** pass（含 `GaussBlur`；另有大量 **CPU 端**高度/路径计算）；
- `displace_rocky_terrain`(`0x140A3EE80`)：读计数资源 `0xfa44538b95ae76c6`、写 2 buffer、绑 5 张地形纹理、uniform = `count/3`、**单 pass**。

**帧序**：`rasterize_terrain_stroke` → `displace_rocky_terrain` → … → `compute_path_masks` → … → `path_pebbles`（后者读路网掩码产 `PebblePlacementSsbo`）。**数据依赖**：栅格(staging) → 地形/mask → path mask → pebbles。

## 4. `rasterize_buildings` vs `gpu_generate_draw_lists`

- `rasterize_buildings`(`0x140A32770`)：查 `0x9b126e795fb22adf` + `PassResources 0x8226cc786e679355` + `0x2ed189581b92decf`，注册 4 pass，产 `GroundedBuildingsSsbo`(`InstanceBuffer`) —— **产实例数据**；
- `gpu_generate_draw_lists`(`0x140987dd0`，C1 已解剖)：做 cull/bucket、prefix-sum、间接参数 —— **消费实例 → 绘制列表**。

⇒ **分工 = 生产 vs 组织提交。**

## 5. 植被链

| pass | 结论 |
|---|---|
| `water_rasterize_from_cpu` | 4 pass，来自上传的 water 栅格 |
| `garden_rasterize_from_cpu` | 2 pass |
| `water_spawn_vegetation` | 2 pass，11 纹理，`0x74430dfcc6d8719b` |
| `spawn_vegetation_around_buildings` | 2 pass，12 纹理，2 并发写 |
| `garden_spawn_vegetation` | 4 pass，15 纹理，4 并发写 |
| `read_bush_positions_for_cpu`(`0x140866540`) | 单 pass，**回读** bush 位置到 CPU（`BushCpuSsbo`，Mapped） |
| `leaf_particles`(`0x1408F4D00`) | 粒子发射器（`0x3dce36237fb50dff` + `0x4259a350202a056c`） |

## 6. `roof_tiles_gravity`(`0x140A29B10`)

遍历 ECS 屋瓦表（魔数 `0x65cfda6a55a841ac`），注册**单个** compute pass（名长 10/27，写 `0xc3483cd70105de2f`、uniform `0x601a45ce43af912f`）。

着色器 `_roof_tiles_gravity.cs` 消费 `RWStructuredBuffer<RoofShingle>`（**stride 64 B**），`PushConstants{first_instance, instance_count, wall_height_delta, roof_pivot_y, animation_t}` ⇒ 更新 64 字节记录的 `current_height` / `velocity__rotation` / `animation_t`。

> **与既有证据对上**：这正是 `evidence/roof-tiles/` 里那个 **64 字节 `RoofShingle`** 记录（偏移表已由该证据的 SPIR-V 反射给出）。**独立来源互证。**

## 7. `stairs` —— 与 `construct_door_stairs` **无关**

- `compute_passes::stairs::stairs`(`0x140A68A50`) 注册 **2 个** compute pass（名长 0x17/0x24、0xe/0x2c），绑 5 张地形纹理 + 读/写 buffer。
- `construct_door_stairs`(`0x14200C7F0`, 4668 B, `system_decorator::decorator_visual::door_stairs_assemble_bricks`)：签名 `(f32×5, …)`，只调 `random_splits_into`(`0x140c90350`)、`0x14128cdd0` 碰撞等，**无任何 pass-builder 调用** ⇒ **纯 CPU 砖块装配**。

⇒ **stairs 链是「CPU 装配 → GPU stairs pass 逐帧处理」**，两件事。

## 8. `GpuWorld` 数据模型

- `GpuWorld::import_to_rg`(`0x1409921C0`, 100 B)：遍历固定 **21 项**子对象数组（**stride 0xc0 = 192 B**，偏移 `0xa0..0x14e0`），对每项调其虚表 `+0x40` 的 import（传 render graph）—— 即 `OwnedGpuWorldSubset` 家族按实例类型各自导入。
- **`TypedStorageBuffer<T>` 共 87 个单态**：raster staging(6)、`RoofTileSsbo`、`Bush`/`BushCpu`/`WaterMask`/`WaterExcludeClutterMask`/`TerrainHeight`/`VisualTerrainHeight`/`TerrainRockMask`/`PathRaster`/`GardenMask`/`GardenRaster`、`GroundedBuildings`(`InstanceBuffer`)、`LeafParticle`/`ParticleEmitter`、`PebblePlacement`、`MouseTerrainRaycast`、`NaniDrawList*`/`NaniInstance*`/`NaniMeshAtlasPartInfo`、`RtBvh`/`RtTriangles` 等。

## 9. 观察 / 推断 / 未知

**观察**：地址、注册结构、TypeId、读/写/纹理计数、**落在保留伪代码中的**访问码字面量、gravity shader 的 64 B `RoofShingle`、`import_to_rg` 的 21 子对象循环。

**推断**：6 次上传的具体栅格名（靠共享 TypeId + 6 个 staging 类型）；`read_terrain_data`(`0x14098C680`) 为 **GPU→CPU 回读**（4 draw + 4 pass + 清 pending 标志，**强推断**）。

**未知**：**TypeId → 类型名（不可反推）**；shader 名串；各 buffer 的具体字段；`PassResources` 7 槽位名；upload 是纹理拷贝还是 compute 写。

**PDB 签名多为 `void()` 占位，伪代码 ≠ 作者原文。未宣称构图闭包闭合。**
