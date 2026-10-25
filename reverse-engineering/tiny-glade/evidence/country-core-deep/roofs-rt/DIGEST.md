# country-core roofs-rt 域 DIGEST（roofs / rt / render / tree_storage / mod / time / shape_intersections）

日期：2026-10-14。数据库：tg_rt1（`workspace/input/tiny-glade.exe`，B patched，独立持久库）。
范围：`resources/roofs/**`（10 文件）+ `resources/rt/**`（3）+ `resources/render/**`（3）+ `tree_storage` + `shape_intersections` + `resources/mod` + `time`，共 20 个章节文件全填充。
反编译：`roofs-rt/decomp/`（63/89 批处理成功 + 26 个 Ghidra NotFound 小函数以 rne-read 原字节手解 + 12 个既有 evidence/roof-* .c 拷贝）。
对账与既有证据：`reconstruction/roof/`（ridge.rs/surface.rs/roof_state.rs/tiles.rs/parameters.rs）与 `evidence/roof-ridge|roof-surface|roof-tiles|render-passes`。

## 1. 模块清单（20 章，与 runner/source-map 对齐）

| 模块 | 文件作用 | 业务/内联/胶水 |
|---|---|---|
| resources/mod.rs | 顶层 App 资源注册（rendering/SSBO/业务） | 3/1/0 |
| resources/shape_intersections.rs | ShapeIntersections 资源（默认胶水） | 0/0/1 |
| resources/time.rs | GameTime 门面（已由前域取证，本域补 default VA） | 0/0/1 |
| resources/tree_storage.rs | TreeStorage 哈希表容器 + InteractableForestPlacements/BirdHaven | 7/2/2 |
| resources/render/mesh_atlas_library.rs | 世界子集/名称→网格图集 + rt 注册 | 11/1/11 |
| resources/render/mod.rs | Affine3Packed 打包仿射 | 4/1/0 |
| resources/render/spatial_resolve_offsets.rs | 空间解析偏移常量表 | 1/0/0 |
| resources/roofs/commands.rs | 屋顶命令枚举（fmt 胶水） | 0/0/2 |
| resources/roofs/dormer_tiles.rs | DormerTiles 资源（default 胶水） | 0/0/1 |
| resources/roofs/roof_animation.rs | RoofAnimation 组件 | 5/0/0 |
| resources/roofs/roof_noise.rs | 屋顶噪声（gpunoise 2D value noise + apply/revert） | 2/6/0 |
| resources/roofs/roof_ridge.rs | 脊线尺寸/几何（全部已由 ridge.rs 恢复） | 6/1/1 |
| resources/roofs/roof_shape.rs | Roof/RoofShapeParams/风格枚举权威 | 28/4/29 |
| resources/roofs/roof_tiles.rs | RoofTile 组件声明（生成逻辑在 tiles.rs） | 2/0/0 |
| resources/roofs/roof_tile_sim.rs | RoofTileSimulation 组件 | 2/0/1 |
| resources/roofs/roof_trigger_volume.rs | 屋顶触发体（三角/法线/uv + mesh-uv→位置） | 7/1/3 |
| resources/roofs/roof_visual_shape.rs | 世界位置→height01/截面 + RoofBoundsDims | 6/0/0 |
| resources/rt/bvh.rs | GPU 线性 BVH 构建 | 1/0/4 |
| resources/rt/bvh_builder.rs | RtBvhBuilderGlue 后台 worker | 1/1/4 |
| resources/rt/mod.rs | RtWorld/RtMesh/DirtyCow + update/upload 系统 | 11/12/8 |

## 2. 布局表（实测）

### Roof（roof_shape.rs，0x58 字节，roof_state.rs new_observed + 几何消费者）
| 偏移 | 大小 | 含义 | 证据 |
|---|---|---|---|
| +0x08 | 4 | shape tag（0=circle / 1=rect） | get_roof_shape/is_flat 等 |
| +0x0c/+0x10 | 4+4 | circle frame（cos/sin） | rotation_yaw atan2 源 |
| +0x14/+0x18 | 4+4 | circle center（xz） | shape_dims 加倍处 |
| +0x1c/+0x20 | 4+4 | rect dimensions | shape_dims |
| +0x24/+0x28 | 4+4 | tip_offset_01 | eave/profile 链 |
| +0x2c | 4 | profile_01 | surface.rs |
| +0x30 | 4 | height_01 | surface.rs/height |
| +0x34 | 4 | ridge_length_01 | ridge calc/from |
| +0x38 | 4 | eave_length_01 | surface.rs |
| +0x3c | 1 | ridge_dir bit | ridge calc/from |
| +0x3d | 1 | rim flag | is_flat_w_no_rim |
| +0x4c | 4 | floor 高度 | pivot_3d/get_height_01 |
| +0x50 | 4 | 动画初态 | new_observed |
| +0x54 | 1 | tag 字节 | new_observed |

### RoofShapeParams（28B）：`{tip_01[2]@0, profile@8, height@12, ridge@16, eave@20, ridge_dir@24}`（parameters.rs 已证 B 序列化）。
### RoofTriggerVolume：triangles {cap@0, ptr@8, len@0x10} stride 0x24；normals {cap@0x18, ptr@0x20, len@0x28} stride 0x24；uvs {cap@0x30, ptr@0x38, len@0x40} stride 0x18；三者长度断言相等。
### TreeStorage：哈希表 {base@0, mask@8, …, len@0x18}，16 字节条目（key@0, value@8），next_index@+0x30；iter 输出 {cur@0,next@8,end@0x10,occupied@0x18}。
### RtWorld/DirtyCow：RtWorld 哈希表同族；DirtyCow refcount 经 LOCK/INC/DEC，==1 时 get_mut 原地。
### RtBvhBuilderGlueResource：+0x10 busy refcount（cmpxchg 0→0x3fffffff 抢注）、+0x18 退出/停止字节、+0x68/+0x70 条目 Arc 对、+0x78..+0x9c 计数、+0x98 数据计数、+0xa0 state（2=空闲）、+0xa8 processing、+0xa1..0xa7 脏字节；全局 0x14362c420 停止标志。**【round3 更正：原"+0x14 state/+0x15 processing"为旧读，raw bytes 0x140b24d50 前导证实 +0xa0/+0xa8】**
### build_gpu_bvh：输入三角 stride 0x28，输出 BVH 节点 stride 0x30；空→3 节点退化根。
### GpuRtTriangle 上传三 SSBO：stride 0x50 / 0x18 / 4×u32（upload_rt_world）。

## 3. 逐函数结论汇总（O=已证实，I=推断，U=未知）

- **已证实 O**：roof_ridge 全部 6（ridge.rs 验证 14,776 案例）；roof_shape 28 中 24 个（surface/ridge/roof_state 已恢复 + 定点反编译 + 手解）；
  roof_trigger_volume 全部 7；roof_visual_shape 6；roof_noise 2；roof_animation 2（另 3 胶水）；roof_tile_sim 1+胶水；
  rt update/upload/get_mut/get/build_gpu_bvh/bvh_builder_worker 等 11；tree_storage 7；mesh_atlas 11；render/mod 4；spatial_resolve_offsets 1；mod.rs 3。
- **推断 I**：`impl$1::from`(rt, 0x1408bc460) 打包转换常量未全解；`FlatRoofStyle::is_wooden` 变体语义；
  `RtBvhBuilderGlueResource::is_idle`（小函数字节已存）。
- **未知 U**：无（所有业务函数至少 I 级，无臆造字段名）。

## 4. panic/日志串清单（原文+地址）

| 字符串 | 地址（胖指针/C 串） | 位置 |
|---|---|---|
| `assertion failed: !self.triangles.is_empty()` | UNK_142aa5958 | roof_trigger_volume.rs triangles |
| `assertion failed: self.triangles.len() == self.normals.len()` | UNK_142aa59e0 | set_normals |
| （set_uvs 断言串） | UNK_142aa5a38 | set_uvs |
| `collider exists in the physics world but not its collider registry` | UNK_142aa3d50 | roof_shape roof_uv_and_point_at_ws_pos（源 systems/collision/world.rs） |
| `UpdateRtWorldEvent requested` | UNK_142aa09f8 | rt/mod.rs update_rt_world |
| `Updating mesh atlas for world subset ` | UNK_142ae3438 | mesh_atlas_library register_meshes |
| `Not registering atlas for world subset ` | UNK_142ae3518 | mesh_atlas_library register_meshes |
| 源路径 `crates/country-core/src/resources/rt/mod.rs` | UNK_142aa09c0/0a08/0a20/0a40 | rt |
| 源路径 `crates/country-core/src/resources/roofs/roof_shape.rs` | UNK_142aa5940 | roof_shape |
| 源路径 `crates/country-core/src/resources/roofs/roof_trigger_volume.rs` | UNK_142aa5a20/5a70/59c8 | trigger_volume |
| 源路径 `crates/country-core/src/resources/render/mesh_atlas_library.rs` | UNK_142ae3448/3498/3538 | mesh_atlas_library |
| 依赖 `crates/utils/src/curve/mod.rs`（Curve） | UNK_142c63340 | roof_shape profile_curve_ws |
| puffin-0.14.1 | /home/h3/.cargo/registry/...puffin-0.14.1/src/lib.rs | rt update/upload |

## 5. 与现有代码/合同的差异与纠错

- **0x140833230 / 0x1408c5680 PDB 名多归属**：GableRoofStyle::is_stepped/is_overhang 与 wall 域
  FreehandHeightType::is_follow_terrain / StructureType::is_fence 落在**同一地址**——两个 5 字节谓词体
  （`cmp[rcx],0;sete` 与 `movzx[rcx]`）是编译器跨类型去重结果。机器码为准：is_stepped=tag==0、is_overhang=tag!=0；
  wall 域此前引用的 `wall\decomp\decomp-0x140833230.txt` 内容是另一函数的 NotFound 副本，**弃用**。
- **0x140002370 clone_behavior**：audio-misc/camera-time/collision-eco 等的 decomp 文件全是 NotFound 副本；
  实际为 `xor eax,eax; ret` 空桩（返回 None），跨模块共享去重。Roof 的 clone_behavior 才是真实现（0x1408e6af0）。
- **0x140a1b100 TreeStorage::iter**：wall\decomp 同名文件实为 PublicWalls::wall_ids 的 NotFound 副本，已在本域重解（iter 迭代器布局）。
- **0x14067dd80 RtBvhBuilderGlueResource::new**：audio-misc 等 decomp 为 NotFound；实际 `mov rax,rcx; ret`（共享去重胶水）。
- **无合同写 X 而证据为 Y 的既有代码错误**：本域未发现 reconstruction 源码错误；reconstruction/roof 各 .rs 均已经原指令差分验证。
- 风格枚举 serde 变体串 = `"Matching"` / `"Override"`（8 字节 LE 常量 0x676e69686374614d / 0x656469727265764f）。

## 6. 跨领域线索

- roof_shape/visual_shape → collision 域：collider registry（FUN_1405b0d70 / systems/collision/world.rs）
- roof_shape → utils::curve（Curve2）、utils::name_hash（mesh_atlas）、utils::geometry::position_from_mesh_uvs（FUN_1416f34a0）
- rt → walls::WallId、render-passes 软件 BVH SSBO（rt_bl_bvh/rt_triangles/rt_triangle_meta）、puffin、render_ssbo（audio-misc）
- mod.rs → rt 构造 0x1408ba490/0x1408bab30、SSBO 库 FUN_1412c*（render_ssbo）
- roof_noise → roof_tiles_gravity compute、roof_visual_shape get_height_01
- tree_storage → critters/BirdHaven、树木系统
- mesh_atlas_library → session-save 域 PrefabName、audio-misc 域 NameHash

## 7. 错位/纠错登记（供中央索引）

1. GableRoofStyle 谓词与 wall 谓词共享去重地址（0x140833230/0x1408c5680）。
2. clone_behavior 空桩 0x140002370 跨 6+ 域复用；Roof 真实现 0x1408e6af0。
3. 0x140a1b100 与 0x14067dd80 的旧域 decomp 文件是 NotFound 副本（地址归属不同函数），本域已重解。
4. RoofShapeParams 字段名是 JSON 语义对应推断，字节偏移/数学行为已独立确认（沿用 roof-surface findings 的保留意见）。
