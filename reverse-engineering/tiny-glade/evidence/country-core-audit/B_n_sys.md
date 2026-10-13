# B_n_sys — render/nani/{systems,cull_shaders,draw_ext}.rs 报告

范围：`reconstruction/country-core/src/render/nani/{systems.rs, cull_shaders.rs, draw_ext.rs}`（mod.rs 未改）。
地址均为 B patched EXE（cf048e27…）；PDB 过程表 `procs.tsv`。标注 O=机器码观测 / I=推断 / U=未知。
所有 `todo!`/`unimplemented!` 已清除（三文件 grep 为空）。未 commit。

## 1. 恢复的函数（地址 → Rust 项）

### systems.rs（2944 行，含测试）
| 地址 | 大小 | Rust | kind(`mov dl,K`) | 数据 |
|---|---|---|---|---|
| 0x14206f300 | 6068 | `nani_maintain_instances`（+`resolve_mesh_part`/`resolve_changed_mesh_part`/`compact_key_for`/`warn_once`） | — | — |
| 0x141565950 | 16 | closure$1 = `extend_no_data` 回调（把 12B 记录存入 `Option`） | — | — |
| 0x142070ac0 | 2754 | `nani_update_solid_vertex_color` | 3 | 0x40 |
| 0x142071590 | 2673 | `nani_update_solid_vertex_color_window` | 4 | 0x40 |
| 0x142072010 | 2874 | `nani_update_solid_albedo_map` | 0xf | 0x40 |
| 0x142072b50 | 2874 | `nani_update_solid_albedo_map_emissive`（机器码与上同体） | 0x10 | 0x40 |
| 0x142073690 | 4258 | `nani_update_preplaced_flower` | 0xe | **0x30** |
| 0x142074740 | 2338 | `nani_update_plant` | 0xa | 0x30 |
| 0x142075070 | 4529 | `nani_update_solid_vertex_color_window_glass` | 5 | 0x40 |
| 0x142076230 | 3084 | `nani_update_solid_vertex_color_bricks` | 6 | 0x40 |
| 0x142076e40 | 3794 | `nani_update_gothic_window_bricks` | 7 | 0x70 |
| 0x142077d20 | 2649 | `nani_update_curved_halftimber`（`&mut` 直写） | 0x18 | 0x40 |
| 0x142078780 | 4140 | `nani_update_stone_floor` | 9 | 0x40 |
| 0x1420797b0 | 3386 | `nani_update_wren` | 0x11 | 0x70 |
| 0x14207a4f0 | 3474 | `nani_update_lantern_emissive` | 0x12 | 0x40 |
| 0x14207b290 | 3588 | `nani_update_plaster` | 0x13 | 0x30 |
| 0x14207c0a0 | 3253 | `nani_update_cloth` | 0x14 | 0x40 |
| 0x14207cd60 | 2562 | `nani_update_smoke` | 0x15 | 0x40 |
| 0x14207d770 | 2673 | `nani_update_balcony_door` | 0x16 | 0x40 |
| 0x14207e1f0 | 3684 | `nani_update_uber` | 0x17 | 0x40 |
| 0x14207f060 | 3162 | `nani_update_velocity_only` | 0x19 | 0x60 |
| 0x14207fcc0 | 2430 | `nani_update_stairs_floor` | 0x1a | 0x30 |
| 0x142080640 | 1053 | `nani_flush_instance_updates` | — | — |

每个 `nani_update_*` 有纯函数 `build_<name>(…) -> InstanceUpdate<N>`（逐位复刻打包）+ ECS 系统（PDB 精确参数）+ 测试。
`bevy_ecs::system::function_system`/`schedule_system` 等 FunctionSystem 单态化（procs 里 ~60 条）是 bevy 库实例，不恢复。

### cull_shaders.rs
9 个 `AsComputeShaderSource` 实现（impl$10/14/18/22/26/30/34/38/42 的 `as_compute_pipeline` 0x141623120/0x141623590/0x141623a10/0x141623e90/0x141624300/0x1416247a0/0x141624c40/0x1416250f0/0x141625590；`source_hash` 0x141624770/0x141624c10/0x1416250c0/0x141625560）。
O：全部同构 `ComputePipelineDesc{path, entry:"cs_main", defines:[("PERM_oc_pass", i32.to_string())]}`；`source_hash = calculate_hash(&("<path>:cs_main", &i32))`（SipHash-1-3 零密钥，写串+0xff+4 字节；`0x141459e20` 实读）。路径长度（29/39/37/34/42/47/37/35/42）来自 `mov ecx,N` 的 alloc 大小。
旧文件的 `cull_and_bucket_for_kind` / `group_draw_cmd_prefix_sum` 在 PDB 无对应符号，删除（I：改为 `NaniCullAndBucketFamily::for_kind`）。`ComputePass::new<…>`、`dispatch_with`、`create_cull_and_bucket_pass`、`cull_and_bucket_external` 属 `gpu_generate_draw_lists`/pass_builder，不在本文件。

### draw_ext.rs
`nani_draw` 0x140904ae0（1428 B）。O 流程见文件头文档（GpuWorld 取 subset 读锁 → `list_index = kind*4 | [0,0,1,2,3][pass_variant]`（立即数 0x302010000）→ `list_index_to_cmd_offset` → 取两个 RG buffer（DrawListLength/DrawList SSBO）→ `read_storage_buffer<NaniInstanceIndirectionSsbo>` +（有句柄则）`read_rg_storage_buffer(0x2106412d631790c6)` → 偏移 `cmd_slot*20`（步长 0x14）→ `count = FORCE_FINE(0x143634d9c) || (kind<=1 && !FORCE_NO_FINE(0x143634d9d))` 选 `draw_indexed_indirect_count`（`max_draw_count = TABLE[kind]`，`.rdata 0x142aab288` 27×u64 实读）或 `_raw`）。
纯决策函数 `nani_draw_plan` / `nani_list_index` / `uses_indirect_count` 均可测。

## 2. maintain 关键 O 发现（旧文档多处错误）
- 顺序：**先删除、后新建、最后变更**。
- 删除：generation==0 的无效实体跳过；`local.remove_entry` 缺失 → `Option::unwrap` panic（src 行 51:41）。
- 新建：`NaniInstanceDesc` 24B：`mesh`（bit0 判别：1=AtlasPart(u32 @+4)，0=Named(NameHash u64 @+8)）+ `subset_kind u8 @+0x10`。Named 经 `get_by_name` 线性找 `part.subset==subset_kind`，找不到 → panic（行 63:21，文案 "NaniInstanceDesc requested mesh {} and shader {:?},\nbut this mesh is not registered for that shader in nani_meshes.ron"）。
- **紧凑 key 16B = `{Vec3 translation, f16(max|scale|), u16 mesh_part}`**（不是旧文档的 `{chunk_id u64, instance_index u32, sort_key f32}`）。
- `extend_no_data(subset, kind, &[key], 1, closure)`；回调给出 12B 记录 `{u32,u32,kind u8,[u8;3]}`，存入局部 Option。解锁后 `gpu_world.total_instance_count(+0x1440) += 1`（**无条件**）。记录存在 → Local `HashMap<Entity,12B,RandomState>::insert` + `Commands` 压 0x28B `Insert<NaniActiveInstance>`；缺失（上限）→ `warn_once("Instance limit reached while trying to spawn {desc:?}")`（`static Mutex<BTreeMap<String,()>>` 去重；键 `"module_path! ()" + "$crate :: Level :: Warn" + 消息`；`expect("Mutex was poisonned")` 行 98:13）。
- 变更：`Changed<NaniInstanceDesc>` 查询 PDB 是 `(&desc, &NaniActiveInstance)`（**无 Entity**）；AtlasPart 直用，Named 要求 `parts.len()==1`（`assert_eq!` 行 106:17）且 kind 相等（行 108:17）；调 `update_instance_mesh(gpu_world, &active, part)`。
- flush：27 个槽（`0x1440/0xc0`，旧文档的 30 错），Owned 槽（判别 ≥ 0x8000000000000002）调 `ShadowedBuffer<GpuInstance>::gpu_flush(device)` + 数据缓冲 vtable+0x30；最后 `+0x1448 = +0x1440; +0x1440 = 0`。

## 3. update_instance_data 的真实签名（O，0x14156fff0 系）
`update_instance_data<T>(subset, key: &mut{u32 chunk,u32 index}, position: &Vec3, sort_key: f32, data: &T)`：fast-path `compact[index].… == key.chunk`；否则 HashMap 查找并回写 `key.index`；写紧凑 key（Vec3 + f16 sort_key）与 `size_of::<T>()` 字节；CAS min/max。
各系统的 `position/sort_key`：多数 = `translation` / `max(|sx|,|sy|,|sz|)`；plaster = `(0,0,0)/1.0`；stairs_floor = `0/1.0`；curved_halftimber = `data[0..3]/1.0`；velocity_only 取当前 Transform。

## 4. 各系统 O 要点（完整打包见源码文档与测试）
- 通用 12 字仿射：`Mat3::from_quat(rot)` 各列乘 `scale.{x,y,z}`（列主序 9 字）+ 平移 3 字。
- solid_vertex_color：w12=SemiTransparent 在场, w13=FlatLighting 在场。window：w12=Semi, w13=ColorId.0。albedo_map/emissive：w12=`[0,1,8,2,3,4,5,6,7][NaniSolidTexture byte]`（表 0x142fd38c8）。glass：`ColorId|semi<<31`（ColorId bit31 → unwrap 失败 "value out of bounds for field WindowGlassFlags.color_id"）、窗光 f32、AlbedoMap 在场、bindless 下标（缺 name → `Option::unwrap`）。bricks：`pack_2x16f_uint(f2,f3)` 等（`utils::pack_2x16f_uint` 0x140c90df0 = `f16(a)|f16(b)<<16`）。
- preplaced_flower（旧文档错）：载荷 **0x30**；先取 `TerrainRockMaskSsbo`（断言 name，缺失 unwrap 失败），`u/v=min(511,trunc(clamp01(c/130+0.5)*512))`，掩码位置 1 → 隐藏；否则水采样（`bilinear_sample_at_world_xz(...,130.0)`）>0.01 → 隐藏；否则 4 个路径栅格 clamp01 之和 >0.1 → 隐藏。隐藏载荷 = 单位旋转、scale 1、平移 `(-0.0,-100.0,-0.0)`，sort_key 0（不是 scale=0）。
- curved_halftimber（旧文档错）：**无 cap 掩码**；只是选色：`direct!=0` → `HalftimberColorIds[key]`（越界 panic），否则 PublicWalls.roofs → Roof(`byte@+0x40==2` → `u32@+0x44`)，否则 ids；写入 `data[3]`（组件 `&mut`，触发 change tick），复制整个组件。
- stone_floor：Roof(0x58B)/Floor(0x28B)/RoofAnimation(8B，缺省 1.0)；Roof 与 Floor 都缺 → unwrap 失败；载荷/切线见源码 `build_stone_floor`。
- wren（0x70）、gothic（0x70）、lantern、plaster（断言 `w.simplified_curve_2d.is_valid()` 文本 O）、cloth、smoke、balcony、uber（flags `albedo*1+semi*2+4`）、velocity_only（当前+上一帧两个仿射）、stairs_floor（9 字拷自 chunk+8 + 3 零字）：详见各 `build_*` 文档。

## 5. U（typed gap）清单
- 组件均为 `crate::Gap` 单元结构（`BrickColor`、`NaniSolidTexture`、`WindowLight`、`AlbedoMap`、`PrevTransform`、`Wren`、`GothicWindowBrickDeformation`、`CurvedHalfTimber`、`Roof`、`RoofAnimation`、`Floor`、`LanternLightParams`、`Plaster`、`ClothlineCloth`、`NormalMap`、`Smoke`、`StairsFloorMeshChunk`）：系统经 `gap_words::<_,N>()` 读（返回 0），**每处注释写了机器码读取的偏移**；待组件所有者给出真实布局后替换。
- `terrain_rock_mask_words`：`AssetSsboLibrary` 无取 CPU 影子字的访问器 → 返回 None，**`nani_update_preplaced_flower` 在有行时会走原版"SSBO 未注册"的 unwrap panic**（需 ssbo owner 补访问器）。
- `circle_tangent_u0`（stone_floor 的 `Circle2d::get_tangent_at_u(0.0)`，0x140c97660）返回 0；`animated_peel_lookup`（AnimatedPlasterPeel 为 Gap）返回 None；`window_light_value`（GlobalWindowLightsControl 为 Gap）返回 0.0。
- `nani_flush_instance_updates` 的参数顺序/第二个参数类型（I：`Res<GraphicsDeviceResource>`）；`NaniActiveInstance` 12B 第 3 个 u32 的语义（I：kind u8 + 3B）。
- External subset 的 `draw_count` 来源、Owned subset 的实例间接缓冲 RG 句柄（draw_ext 中 `OwnedIndirectionHandle` 默认返回 None）。
- `NaniCullAndBucket*` 的 4 字节字段名 `oc_pass`（I；类型 i32 由 Display/Hash 确认）。

## 6. 依赖/stub
- **Cargo.toml 需追加 `half = "=2.7.1"`**（离线缓存已有；原锁 2.4.1）。systems.rs 用 `half::f16::from_f32`（0x1408b9f20）与 `pack_2x16f_uint`。未改 lib.rs/rhapsody_types.rs。
- 本地实现 `warn_once`（原版宏内联，非库）、`pack_2x16f_uint`（0x140c90df0 是游戏自有 utils）。

## 7. 对公共 API 的改动 / 缺口（由集成者合并）
systems.rs 通过 `systems::gw` 适配层按**机器码调用约定**写，gpu_world.rs/drawable.rs 现有占位签名不符，需 n_world 提供（旧→新）：
1. `OwnedGpuWorldSubset::update_instance_data(key: InstanceKey, data)` → `(&self, key: &NaniActiveInstance, position: Vec3, sort_key: f32, data: T)`。
2. `OwnedGpuWorldSubset::extend_no_data(keys: &[InstanceKey], kind, cb: FnMut(u64))` → `(&self, kind, keys: &[NaniCompactKey], cb: FnMut(NaniActiveInstance))`（`NaniCompactKey` 在 systems.rs 定义）。回调收 12B 记录，不是 u64；达上限不调回调。
3. `GpuWorld::update_instance_mesh(entity)` → `(&self, &NaniActiveInstance, mesh_part: u32)`；`GpuWorld::remove_instance(InstanceKey)` → `(&self, &NaniActiveInstance)`。
4. `GpuWorld` 需 `bump_instance_count()`（`+0x1440 += 1`）、`roll_instance_counts()`（`+0x1448 = +0x1440; +0x1440 = 0`）、`flush_instance_buffers(&GraphicsDeviceResource)`（27 槽，Owned 才 flush）。
5. `InstanceKey{chunk_id:u64,instance_index,sort_key:f32}` 与机器码不符：紧凑 key 是 `{Vec3, f16, u16 mesh_part}`；`NaniActiveInstance` 是 12B `{u32,u32,kind u8,[u8;3]}`（drawable.rs 现为 13B 有效/16B，且 `from_key/into_key` 为 `todo!`；Local map 值也是这 12B，不是 `InstanceHandle{kind,index}`）。
6. 机器码 `Transform` = `rotation Quat @0, translation Vec3 @0x10, scale Vec3 @0x1c`，stride 0x30；crate `utils/components/transform.rs` 是 translation@0/rotation@0xC。systems.rs 全部按字段名访问，不受影响。
7. `instance_data.rs`：`PreplacedFlowerInstanceData` 应为 12 字（现 16；测试 `preplaced_flower_is_12_words` 已 `#[ignore]` 标注）；`CurvedHalfTimberInstanceData` repr(C) 因 u64 字段不对齐为 72B（应 0x40；`curved_halftimber_is_16_words` 已 ignore；curved 系统直接复制组件本体，故尺寸修好即对）。`pack_instance` 在运行期断言 `size_of::<T>()==4N`，其余 19 个类型尺寸已验证一致。
8. `GpuWorldSubsetKind` 的命名与各系统的 K 字节不一致（plaster=0x13/smoke=0x15/flower=0xe/curved=0x18 …，枚举里 Plaster=4/Smoke=6/Flowers=3/CurvedHalftimber=2）。系统里 kind 一律按数值 `kind_from_u8(K)`；`GpuWorld.main_subsets`（+0x1450）在 nani_draw 里是 `list_index→cmd_slot` 的 u32 表（`list_index_to_cmd_offset`），不是 7 个锁。
9. A4 `pass_builder`（按当前公共 API 写，列缺口）：
   - `DrawBuilder` 无 World/`context` 可取 `GpuWorld`/`AssetSsboLibrary`：`NaniDrawExt::nani_draw(self, kind)` → `nani_draw(self, kind, &GpuWorld, &AssetSsboLibrary, &mut RenderGraph)`（唯一调用方 clearing_shaders.rs 里只有文档注释，无实际调用）。
   - `draw_indexed_indirect_raw(self, indirect_index, offset, count)` / `draw_indexed_indirect_count(self, indirect_index, count_index, offset, count_offset)` 缺机器码里的 `stride(0x14)` 与 `max_draw_count`（count 路径）参数；我用 `ImportedBuffer.resource as u32` 作 buffer 下标。
   - `read_storage_buffer<T: SsboType>` 需要 `NaniInstanceIndirectionSsbo: SsboType`（buffers.rs 当前为无 impl 的单元结构）；DrawBuilder 字段 `variant`（现 +0xb0）与机器码 `[self+0xf0]` 的"pass 变体"含义需 A4 确认（我用 `self.variant`，I）。
   - `AsComputeShaderSource` 现返回 `ComputeShaderSource`，而 9 个 cull shader 的机器码返回 `ComputePipelineDesc`（`NaniCullShader` 自有 trait 暂代，A4 定稿后改 impl）。
10. 其它：`PathRastersResources` 机器码是 4×`Res<WorldRaster>` 的裸 SystemParam，crate 是普通结构；`HalftimberColorIds` 机器码是带边界检查的索引切片，crate 是 `HashMap<u64,ColorId>`；`bilinear_sample_at_world_xz`（utils/float_grid.rs）仍是 `todo!` 且缺第 4 个参数 130.0；`PublicWalls.roofs` 已用。

## 8. 测试
隔离台：`/tmp/h_n_sys`（`./go.sh`；用 stub 重现 crate 路径，真实文件挂 `real/render/nani/`，instance_data.rs 取自仓库）。
`cargo test --offline`：**70 passed, 0 failed, 2 ignored**（均为 7 的尺寸缺口）。包含：
- 21 个系统的纯函数测试（打包位、flag、分支、panic 文案：glass bit31、plaster 断言、flower 隐藏载荷等）；
- draw_ext：`0x302010000` 变体表、list_index、count/raw 分派、偏移与 `max_draw_count`、表值实读；
- cull_shaders：路径长度/URI/PERM_oc_pass/source_hash；
- ECS 级：`maintain` 创建/删除/变更全路径（stub GpuWorld）、缺 shader panic、plant 系统推送平移与 sort_key、`nani_draw` count/raw 两分支。
`rustfmt --check --edition 2024` 三文件通过。真实 crate 因 `dolly` 等缺失无法整体 `cargo check`，未做整体编译。

## 9. 遗留
- 各 gap 组件、AnimatedPlasterPeel、GlobalWindowLightsControl、TerrainRockMask 访问器、Circle2d 切线需所有者补齐后替换对应 `gap_words`/stub。
- 21 个 `nani_update_*` 的 kind 与 `GpuWorldSubsetKind` 命名映射需 n_world 澄清。
- `nani_flush_instance_updates` 参数顺序为 I。
