# B_i_nani — nani 层与 A4 / n_world 新 API 的集成

范围：`render/nani/*`、`render/pass_builder/draw_builder.rs`（仅 `bind_brick_erase_highlight` + 必需的 `UniformValue for Vec3`）、`Cargo.toml`、`utils/index_condenser.rs`（无改动）。
另改（任务 1/5 明确要求，非 nani 目录）：`components/drawable.rs`、`render/ssbo.rs`（追加）。标注 O/I/U 同前。

## 1. 对齐结果（n_sys 的 `systems::gw` 适配层已删除，直接调用 n_world 真实签名）
| 项 | 旧（n_sys 假设） | 新（与 gpu_world.rs 一致） |
|---|---|---|
| 取 subset | `owned_subset_mut(k).write()` + `if let Some(Owned)` | `let mut s = gpu_world.owned_subset_mut(k)`（`MappedRwLockWriteGuard<OwnedGpuWorldSubset>`；External/空槽按 O panic） |
| `NaniActiveInstance` | 16B 假结构（drawable.rs） | **就是 12B `InstanceKey`**：`InstanceKey` 现 `#[derive(Component)]`，`components::drawable::NaniActiveInstance` 改为 `pub use ... InstanceKey as NaniActiveInstance` |
| 紧凑 key `NaniCompactKey` | 自定义 `{Vec3,f16,u16}` | `pub type NaniCompactKey = InstanceCullData`（`radius`=f16 sort_key，`mesh_index`=mesh_part，布局相同） |
| `extend_no_data` | `(&self,kind,&[NaniCompactKey],cb(NaniActiveInstance))` | `subset.extend_no_data(kind, &[InstanceCullData], cb(InstanceKey))` |
| `update_instance_data<T>` | `(&self,&Active,pos,sort,T)` | `(&mut self,&mut InstanceKey,pos,radius,T: InstanceElement)`；查询只给 `&`，系统里拷一份 `let mut key=*active`（下标提示不回写进组件，只影响快路径） |
| `update_instance_mesh` / `remove_instance` | `(&Active, u32)` / `(&Active)` | `gpu_world.update_instance_mesh(&mut key, part as u16)` / `remove_instance(&mut key)` |
| `+0x1440 += 1` | `bump_instance_count()` | 直接 `gpu_world.total_instance_count += 1`（pub 字段，O：解锁后无条件） |
| flush | `flush_instance_buffers(&GraphicsDeviceResource)` | 新增 `GpuWorld::flush_instance_buffers(&dyn GraphicsDevice)`（Owned 才 `gpu_flush`）、`roll_instance_counts()`、`previous_instance_count()`；系统 `&**device` 取 `rhapsody_contract::GraphicsDevice` |
| `PathRastersResources` | crate 里是普通结构，不能做 SystemParam | 系统内自带 `NaniPathRasters`（`#[derive(SystemParam)]`，4×`Res<PathRaster/Cobblestone1..3Raster>`，O：机器码为 4×`Res`） |
| `CurvedHalfTimberInstanceData` 读取 | 旧字段名 `packed_a/source/...` | 按 n_world 新字段（`position,pad_0c,x0..w3c`）；颜色写入 `pad_0c`（= `data[3]`，O） |
| `GpuWorldSubsetKind` 旧名 | `Main/Bush/Flowers/Unknown12` | `Canopy/TreeTrunk/Flower/...`；cull_shaders `for_kind` 改用真名（`Flower→Flowers`，`CurvedHalfTimber→CurvedHalftimber`）|

`#[ignore]` 的两个测试（`preplaced_flower_is_12_words`、`curved_halftimber_is_16_words`）已取消 ignore 并通过（n_world 已修尺寸）。

## 2. draw_ext.rs → `DrawBuilder<'p>`
- `impl NaniDrawExt for DrawBuilder<'_>`；`nani_draw` 返回 `()`（`draw_*` 现为终结操作）。
- `read_storage_buffer::<NaniInstanceIndirectionSsbo>("NaniInstanceIndirectionSsbo", 0xcdf9c36a6a9c8c80)`（槽名 O：0x142c8151d）；`read_rg_storage_buffer(0x2106412d631790c6, rg_handle.resource as u32)`，句柄现在真取 `owned.data().rg_handle()`（原 `OwnedIndirectionHandle` 占位 trait 删除；External → `None`、draw_count 仍 U=0）。
- raw：`draw_indexed_indirect_raw(args_res, cmd_slot*20, 0x14, draw_count)`（O：`r8/r9/[rsp+0x20]`）；count：`draw_indexed_indirect_count(args_res, count_res, [cmd_slot*20, 0x14, list_index*4, MAX_DRAWS[kind]])`（O：偏移 `+0x50/+0x58/+0x60/+0x68` 对 `r8/r9/[rsp+0x28]/[rsp+0x30]`）。
- 缺口（A4）：`PassHost` 没有 World 资源访问器，`GpuWorld`/`AssetSsboLibrary`/`RenderGraph` 仍以显式参数传入。
- `buffers.rs` 的 7 个 SSBO 载体改用宏接入 `StorageElement` + 库 `SsboType` + `pass_resource_access::SsboType`（`rg_buffer_hash = SsboId::of::<T>()`，I），使 `get_rg_buffer::<NaniDrawList*Ssbo>`/`read_storage_buffer` 真的可编译。

### 机器码复核发现的两个 n_world 错误（已改，O）
1. **`nani_calculate_draw_list_params` 循环体在 0x1408bb0bc/1f6/336/475 展开 4 份**（只有第 4 份经跳表进下一个 kind）：每个 kind 向 4 个 Vec 各 push 4 次 → 每个 Vec 长 **108**（= `NaniDrawListInfo` 缓冲 0x1b0 B；`cmd_offsets` 与 `NaniDrawListOffsetDefaults::compute()` 的 108 项逐项相等，总和 3 788 800 互证）。旧实现只 push 27 次，`list_index = kind*4|变体` 一用就越界。新增常量 `LISTS_PER_KIND = 4`，测试已更新（总计 `(3788800, 71936, 2048, 8)`）。
2. **`list_index_to_cmd_offset` 读的是 params 的第二个 Vec**（`nani_draw` 0x140904c98：`[params+0x20]`/len `[params+0x28]` = `max_instance_offsets`，不是 `cmd_offsets`）。已改，越界仍是 `panic_bounds_check`。

## 3. `bind_brick_erase_highlight`（0x14090be10，仅此函数）
O：取 4 个 world 资源值（TypeId：A `0x6efcda1c77e106b4/0x7c999c10e7fa637f` 字节@+0xc；B `0x062dfe8f62b6fcc5/-0x09aff71ab79cd459` = `CursorTerrainRaycast`，标签 bit0 为 Some，Vec3@+4；C `0x0ae77a86a2c6cefc/-0x0aaf65a94cd17e30` f32@+0；D `0x48b321ef779dc301/-0x429a898e6b262238` 字节@+0xfe），缺失均 `Option::unwrap` panic（4 个位置）；然后三次 `write_uniform`：bool `0x203fd61c57064e54` = `!D && A`；Vec3 `0x06bdce4199980bf7` = 光标位置（None→0）；f32 `-0x4a7a31de4938d73c`(=0xb585ce21b6c728c4) = C。各自在常量块缺失时静默跳过。
实现：`bind_brick_erase_highlight(self, &BrickEraseHighlightInputs{enabled, cursor_position, radius, suppress})`（A4 缺口：无 World 访问器，值由调用方读出；A/C/D 的资源类型 U，B 与 `CursorTerrainRaycast{tag,pos}` 吻合）。附带 `impl UniformValue for glam::Vec3`（12 字节，O：`{u64,u32}` 拷贝）。2 个测试。

## 4. bytemuck / warn-once / BufferDeviceAddress
- `Cargo.toml`：`bytemuck = "=1.21.0"`（原 `reconstruction/Cargo.lock` 为 1.16.0，离线缓存没有；缓存只有 1.21.0/1.25.x，取最接近者）；`glam` 追加 `bytemuck` feature（`Vec3: Pod`）。**不带 `derive`**：`bytemuck_derive` 离线缓存没有，本 crate 也没用到 derive。
- trimesh：手写 `cast_slice_failed`/`positions_view` 的两项检查 → 真实 `bytemuck::cast_slice::<u8, Vec3>`（panic 文案 `cast_slice>…` 由库产生）；`attrib<T: bytemuck::NoUninit>` 取代 `unsafe from_raw_parts`；测试里的 `Aligned16` 改为 `[f32;4]` + `unsafe impl NoUninit`。
- warn-once：`log` 没有 warn-once 能力（游戏自有宏展开），所以保留逻辑但**只剩一份**：`nani::warn_once_in` + `warn_once_key` + `nani_warn_once!`（每展开点一张 `static Mutex<BTreeMap<String,()>>`，O），gpu_world 与 systems 共用；systems 里 `warn_once(String)->bool` 保留给测试。
- `BufferDeviceAddress` trait 删除 → `render/ssbo.rs::rhapsody_boundary::GraphicsDevice` 增加提供方法 `fn device_address(&self,&Buffer)->u64 { 0 }`（O 0x140bd4c00 的占位；`rhapsody_contract::GraphicsDevice` 经 ssbo.rs 的既有 impl 自动获得，其它实现者不受影响）。`Trimesh::build<D: GraphicsDevice>`。

## 5. 验证
harness：`/tmp/h_i_nani/run.sh`（基于 a4_compute harness；真实挂载 `render/nani/*`（含 mod.rs）、`render/pass_builder/*` 全部、ssbo/shadowed_buffer/asset_libraries/rhapsody_contract(由真 lib.rs 抽取)、`components/drawable.rs`、`resources/{color_id,critters,window_lights,world_raster,world_raster_defs,walls/{plaster,curved_halftimber,public_walls,public_wall_state}}`、`utils/{index_condenser,float_grid,components/transform}`、`resources/render/mesh_atlas_library.rs`；其余 components/Roof/StairsFloorMeshChunk/WallId/Floor 为同形桩）。
- `render::nani`：**131 passed / 0 failed / 0 ignored**；`render::pass_builder`：全过（含 draw_builder 44）。全 harness 508 项 505 过，3 个失败均**不是我的文件**：`resources/render/mesh_atlas_library.rs` 的 cfg(test) 仍写旧变体名 `GpuWorldSubsetKind::Main`（harness 里打补丁把 `Main→Canopy`，但断言文案仍是 "Main"/"Main:atlas"）。
- 新增 ECS 级测试（真实 `GpuWorld`+`MockDevice`）：maintain 创建/变更 mesh/删除全路径、容量用尽（不 panic、不插入、`total` 仍 +1）、空槽 / External panic 文案、plant 系统覆写 position/radius 且保持 `mesh_index`、flush 滚动计数；`nani_draw` raw/count/空槽三条端到端（真实 `RasterPass`+`MockHost`+`AssetSsboLibrary`）。
- `rustfmt --check --edition 2024`：我改的文件全部通过（`render/nani/mesh_atlas.rs`、`render/ssbo.rs` 的既有格式差异不是本次引入）。
- 整个 crate 仍无法 `cargo check`（`dolly` 离线缺失）。

## 6. 需要集成者处理 / 其它 owner（不在我的文件内）
1. `AssetSsboLibrary` 需要 `#[derive(Resource)]`（systems 的 flower 系统与 `systems/world_raster.rs` 都用 `Res<AssetSsboLibrary>`；harness 里用 `impl Resource` 绕过）。
2. `resources/render/mesh_atlas_library.rs` 测试里的旧变体名/文案（`Main`、"…shader Main"、"Main:atlas"）要改成 `Canopy`。
3. `utils/float_grid::bilinear_sample_at_world_xz` 仍是 `todo!` 且缺机器码的第 4 个参数 `130.0`（flower 系统调用点按 3 参数写）。
4. `terrain_rock_mask_words`（`AssetSsboLibrary` 的 CPU 影子访问器）仍 U → flower 系统在有行时走原版 "SSBO 未注册" 的 unwrap panic。
5. `GpuWorld`/subset 的 `gpu_flush` 原版还冲刷 `ShadowedBuffer<GpuInstance>`（positions），n_world 版只冲数据缓冲（U，未改）。
6. `components/drawable.rs`：`NaniActiveInstance` 现是 `InstanceKey` 别名，`NaniActiveInstance::{from_key,into_key,handle}` 已删（无外部使用者）；其余 gap 组件、`GlobalWindowLightsControl`/`AnimatedPlasterPeel` 等仍是 U。
7. 21 个 `nani_update_*` 的 kind 数字仍经 `kind_from_u8(K)`；与真名对应（SolidVertexColor=3 … StairsFloor=26）已被 n_world 枚举证实，没有逐个替换成变体名。
8. `DrawBuilder::bind_common` 的调用者 `common_draw_params.rs`、`gpu_generate_draw_lists` 若调用 `nani_draw`，需传入 `GpuWorld`/`AssetSsboLibrary`/`RenderGraph`。
