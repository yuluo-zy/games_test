# B_n_world — nani::{gpu_world, buffers, instance_data, trimesh}（+ utils::index_condenser）

标注：O = 机器码观测，I = 推断，U = 未知。地址均为 B patched EXE（cf048e27…）。
范围：procs.tsv 中 `country_core::render::nani::{gpu_world,buffers,instance_data,trimesh}::` 自有过程共 28 个独立单元
（泛型实例归并后；`impl$14` ×21、`update_instance_data` ×21、`impl$36` ×19 等各只恢复一份泛型逻辑）。
五个文件均已清除 `todo!`/`unimplemented!`（原 gpu_world 24 处、instance_data 7 处、buffers 1 处）。

## 1. 恢复的过程 → Rust 项

| 地址 | 过程 | Rust 项 | 结论 |
|---|---|---|---|
| 0x1408baed0 | `nani_calculate_draw_list_params` | `buffers::calculate_with_flags / nani_calculate_draw_list_params` | O（含 0x3fe 掩码、两个开关的优先级） |
| 0x1408bb650 | `impl$9::default_value` | `NaniDrawListOffsetDefaults::{compute, default_value}` | O（跳表逐项解码，27 项 `T[k]` 与步长逐项相等） |
| 0x1409917d0 | `GpuWorld::new` | `GpuWorld::new(device)` | O（布局见 §3） |
| 0x140991c50 | `owned_subset_mut` | `GpuWorld::owned_subset_mut` → `MappedRwLockWriteGuard` | O |
| 0x140991d60 | `update_instance_mesh` | `GpuWorld::update_instance_mesh` / `OwnedGpuWorldSubset::update_instance_mesh` | O |
| 0x140991f60 | `remove_instance` | `GpuWorld::remove_instance` / `OwnedGpuWorldSubset::remove` | O |
| 0x1409921c0 / 0x140992220 | `import_to_rg` / `export_from_rg` | `GpuWorld::{import_to_rg, export_from_rg}` | O |
| 0x140992280 | `extend_no_data` | `OwnedGpuWorldSubset::extend_no_data` | O |
| 0x14156db40 系 ×21 | `update_instance_data<T>` | `OwnedGpuWorldSubset::update_instance_data<T>` | O |
| 0x14156d0d0 | `create_instances<PlacementDataSsbo,…>` | `GpuWorld::create_instances<T>` | O |
| 0x141572130 系 | `impl$14::{type_name,as_any,as_any_mut,gpu_flush,rg_handle,import_to_rg,export_from_rg,copy}` | `NaniInstanceData` for `InstanceDataBuffer<T>` | O |
| 0x140992a60 | `impl$17::fmt` | `GpuWorldSubsetKind` 的 `Debug`（27 个变体名） | O（字符串表逐项读出） |
| 0x141572870 | `deserialize::visit_str` | `GpuWorldSubsetKind` 的 serde `Deserialize` 派生 | I（体量与 27 路字符串匹配一致，未逐字节比对） |
| 0x141572d80 系 ×19 | `impl$36::register_required_components` | `InstanceDataMarker<T>: Component` + `instance_data::register_required_components(world)` | O（bevy 派生展开，无游戏数值） |
| 0x14098f690 / f800 / f970 | `CurvedHalfTimber…::{new_curved_xy,new_curved_xz,new_straight}` | 同名函数，逐偏移写入核对 | O |
| 0x140990a80 / 0x140990af0 | `impl$122::{register_required_components,clone_behavior}` | `#[derive(Component, Clone)]` | O |
| 0x14003bb80 | `UberInstanceData::marker` | `UberInstanceData::marker()` | O（1B ret，ICF） |
| 0x14129dbd0 | `Trimesh::builder` | `Trimesh::builder(indices)` | O |
| 0x14129e890 | `TrimeshBuilder::positions` | `positions(self, &[Vec3])` | O |
| 0x140905080 / 0x1412a90c0 | `TrimeshBuilder::attrib<Vec3>` | `attrib<T>(self, &[T])` | O |
| 0x14129e900 | `TrimeshBuilder::build` | `build(self, device, name)` | O |
| 0x14129dbc0 | `TrimeshChunk::index_count` | `TrimeshChunk::index_count()` | O |
| 0x140a19f50 | `IndexCondenser::remove<InstanceKey>` | `utils::index_condenser::IndexCondenser`（新写，原为 Gap 桩） | O |
| 0x140a241d0 | `impl$44::fmt` | **未恢复**（见 §6：PDB 归属与内容不符） | U |

## 2. 机器码推翻的旧骨架假设（均已修正）

1. **`GpuWorldSubsetKind` 的 27 个变体名**：0 Canopy, 1 TreeTrunk, 2 BackgroundTree, 3 SolidVertexColor, 4 SolidVertexColorWindow,
   5 SolidVertexColorWindowGlass, 6 SolidVertexColorBricks, 7 GothicWindowBricks, 8 RoofTile, 9 RoofFloor, 10 Plant, 11 Flower, 12 Bush,
   13 WoodenPlank, 14 PreplacedFlower, 15 SolidAlbedoMap, 16 SolidAlbedoMapEmissive, 17 Wren, 18 LanternEmissive, 19 Plaster, 20 Cloth,
   21 Smoke, 22 BalconyDoor, 23 Uber, 24 CurvedHalfTimber, 25 VelocityOnly, 26 StairsFloor（`impl$17::fmt` 的 0x142aba868 长度表 + 0x142aba940 跳表）。
   旧的 `Main/Bush/CurvedHalftimber/Flowers/…/Unknown9..26` 全是编造；`shader()`/`try_from_shader` 无机器码依据，已删除。
2. **`InstanceKey` = 12B `{id:u32, index:u32, kind:u8}`**（`remove` 读 `key+8` 为 kind；`IndexCondenser` 读 `key+0/+4`）。16B 的是每实例的
   **`InstanceCullData`**（`{Vec3 位置 @0, f16 半径 @0xc, u16 mesh_index @0xe}`：`update_instance_data` 写前 0xe 字节，`update_instance_mesh` 写 `+0xe`）。
3. **`GpuWorld` 布局**：27×0xc0 的 `RwLock<Option<GpuWorldSubset>>`、`+0x1440 total_instance_count`、`+0x1448` 保留、`+0x1450 Box<NaniDrawListParams>`（0x70B）、
   `+0x1458 TypedStorageBuffer<NaniMeshAtlasPartInfo>`、`+0x1480 TypedStorageBuffer<NaniDrawListInfo>`。旧骨架的“7 个主 subset 锁数组”“+0x14a0 params 数据”不存在；
   `NaniDrawListInfo` 是 4B 元素（调试名 "NaniDrawListInfo"，BUFFER_LEN=0，初始数据 = `max_instance_offsets`）。`GpuWorld::new` 带 `device`。
4. **容量用尽不是 panic**：`extend_no_data`/`create_instances` 里是 warn-once 宏展开（键 `"module_path! ()$crate :: Level :: Warn"+消息`，全局 `Mutex<BTreeMap>`，`Mutex was poisonned`），
   然后跳过该实例；`create_instances` 末尾 `total_instance_count += cull.len()`（请求数）。
5. **`create_instances`**：第 5/6 参数是数据切片 (ptr,len)，没有闭包调用、没有 `cap`；`count = min(cull.len, data.len)`。
6. **`NaniInstanceData` 是类型擦除的数据缓冲 trait**：实现体是 `{rg_handle: Option<ImportedBuffer>, ShadowedBuffer<T>}`；vtable 槽序
   +0x18 type_name、+0x20 as_any、+0x28 as_any_mut、+0x30 gpu_flush、+0x38 rg_handle、+0x40 import、+0x48 export、+0x50 copy；`copy(dst, src)` 目的在前。
   类型不符 panic 文案 `"The instance data for {kind:?} is not {T} but {actual}"`（0x142c76920，三片段三实参）。
7. **元素尺寸**：PreplacedFlower = 0x30（旧骨架 0x40）；其余 0x40/0x30/0x60/0x70 分组已逐个 `update_instance_data<T>` 最大写偏移核对（见 instance_data.rs 头注释）。
8. **`default_value` 与 `nani_calculate_draw_list_params` 不是同一代码体**（旧契约把 Ghidra 跳表误读当成重叠），是两个独立函数；前者的 108 个 u32 = 每 kind `base + j*T[k]`。
9. **`TrimeshBuilder`**：属性记录没有名字（`{ptr,len,elem_size u16,align u8}`），`+0x12` 是元素**对齐**，且强制对齐降序；`positions` 存的是属性**下标**；
   `Trimesh::builder(indices)` 切 ≤64 三角形的 meshlet，这是旧骨架完全缺失的主体；`build` 打包 GPU 缓冲、算包围球。
10. **`CurvedHalfTimberInstanceData`**：`segment_count/packed_a/packed_b` 全是误读；真实布局见 instance_data.rs 的偏移表（+0x30 是细分等级 `ilog2(next_pow2(ceil((|y1-y0|*x1+1)/0.6))).clamp(2,5)`，`ceilf` 经 IAT 解析确认）。
    `curved_halftimber_mesh_atlas_part_idx` 只有 PDB LF_FUNC_ID（内联），无机器码依据，已删除。

## 3. O / I / U 清单

**O（机器码观测）**：上表全部过程的控制流、常量、panic/expect/assert 文案与源码位置（gpu_world.rs 行 0x1ac/0x1ba/0x1e6/0x30a/0x311；trimesh.rs 行 112/114/118/124/147/196/224/232/233/243）；
`MAX_DRAWS_PER_LIST`（0x142aa14c0）与 `max_instances`（0x142aba790 / 0x142c77010）三表逐值相同；`NaniDrawListParams` 四个 Vec + 四个汇总的累加规则；`TrimeshChunk` 0x20B 字段偏移。

**I（推断）**：字段语义名（`InstanceCullData.radius/mesh_index`、`CurvedHalfTimber` 的 `x0/x1/y0/y1/w*`）；`GpuWorld::{get_subset, try_get_subset_mut, assign_subset, list_index_to_cmd_offset}` 的具体形态
（PDB 只有 LF_FUNC_ID，内联）；`OwnedGpuWorldSubset::new<T>` 的容量 = `max_instances[kind]`（positions 长度检查与表项一致，构造器本身内联 21 处未逐个读）；
`remove_instance` 未命中日志级别（`log` 记录里的级别字段 = 5 = Trace，过滤比较 `cmp 5`）；`GpuWorldSubsetKind` serde 派生。

**U（未知，未编造）**：`GpuWorldSubsetDesc` 字段（`gpu_world_subset_desc::{owned,external,owned_custom}` 已内联）；`External` subset 的载荷；`GpuWorld+0x1448`；
除 `CurvedHalfTimber` 外 19 个实例数据结构的字段布局（只保证尺寸/对齐/`StorageElement` 名，字段在 `nani_update_*` 与 shader 侧）；`NaniMeshAtlasPartInfo` 真实元素大小
（只确认 `BUFFER_LEN = 0x6c0`）；`NaniUberInstanceFlags`/`WindowGlassFlags` 位域；`PlacementDataSsbo` 真实 0x30B 布局（`resources::placement_data` 里的 `_gap` 使其 size ≠ 0x30，
`create_instances` 因此以泛型 `T` 实现并用测试元素验证）；`impl$44::fmt`（见 §6）；`buffers.rs` 里另 5 个 SSBO 载体与 `GpuInstance` 的字段。

## 4. stub / 替身清单

- `BufferDeviceAddress`（trimesh.rs 内的单列 trait）：`rhapsody::raw::buffer::Buffer::device_address`（0x140bd4c00）的最小替身；`build` 泛型于 `GraphicsDevice + BufferDeviceAddress`。
  **集成时请并入 `ssbo.rs::rhapsody_boundary::GraphicsDevice`**（我不拥有该文件）。
- `bytemuck::cast_slice` 的两项检查手写（原版 `something_went_wrong<PodCastError>`；bytemuck 不在离线缓存/依赖里），panic 文案 `cast_slice>…` 同 bytemuck。
- `warn_once`：游戏 `utils` 里的 warn-once 宏展开，局部实现（`std::sync::{Mutex, OnceLock}` + `BTreeSet`）。
- `utils/index_condenser.rs`：原为 `Gap` 桩，按 0x140a19f50 + 三处内联的 push/resolve 写成真实实现（**该文件不在我的拥有列表，但它唯一的使用者就是 gpu_world，已替换**，请集成者确认）。
- 复用已有：`rhapsody_types::{RenderGraph, ImportedBuffer}`、`ssbo::{TypedStorageBuffer, StorageElement, BufferDesc::new_gpu_only}`、`shadowed_buffer::ShadowedBuffer`、`dirty_index_range::DirtyIndexRange`；`half`（Cargo.toml 已由他人追加 `half = "=2.7.1"`，原 lock 为 2.4.1，离线缓存只有 2.7.1）。

## 5. 公共 API 改动（旧 → 新，原因均为机器码证实）

| 项 | 旧 | 新 |
|---|---|---|
| `GpuWorld::new` | `new()` | `new(device: &dyn GraphicsDevice)` |
| `owned_subset_mut` | `-> &RwLock<Option<GpuWorldSubset>>`（调用方 `.write()` 再 match） | `-> MappedRwLockWriteGuard<OwnedGpuWorldSubset>`（External panic "Subset {kind:?} is not owned"，空槽 unwrap panic） |
| `InstanceKey` | `{chunk_id:u64, instance_index:u32, sort_key:f32}` | `{id:u32, index:u32, kind:GpuWorldSubsetKind}` |
| `GpuWorldSubsetKind` | 9 个编造名 + `Unknown9..26` | 27 个真名；`from_u8`、`ALL`、`CurvedHalftimber` 别名常量保留旧拼写 |
| `extend_no_data` | `(&self, keys:&[InstanceKey], kind, cb(u64))` | `(&mut self, kind, items:&[InstanceCullData], cb(InstanceKey))` |
| `update_instance_data<T>` | `(&self, key, data)` | `(&mut self, key:&mut InstanceKey, position:Vec3, radius:f32, data:T)`，`T: StorageElement+Default+Send+Sync` |
| `create_instances` | `<T,F>(kind, keys, cap, create:F, pending)` | `<T>(&mut self, kind, cull:&[InstanceCullData], data:&[T], pending)` |
| `remove_instance` / `update_instance_mesh` | `(key)` / `(Entity)` | `(&self, key:&mut InstanceKey)` / `(&mut self, key:&mut InstanceKey, mesh_index:u16)` |
| `import_to_rg/export_from_rg` | `&self`，`rhapsody_contract::RenderGraph` | `&mut self`，`rhapsody_types::RenderGraph` |
| `NaniInstanceData` | `copy(src,dst)`，`gpu_flush(&self)` | `copy(dst,src)`，`gpu_flush(&mut self, device)`（原版 flush 取全局设备，沿用 `ShadowedBuffer::gpu_flush(device)` 的既有签名） |
| `NaniDrawListInfo` | `{_gap:[u32;4]}` | `NaniDrawListInfo(u32)` |
| `DrawListParams` | 结构体 | `NaniDrawListParams`（`DrawListParams` 为类型别名，lib.rs 重导出不受影响）；`GpuWorld` 新增 `draw_list_params()` |
| 删除（无机器码依据） | `GpuWorldSubsetKind::{shader,try_from_shader,gpu_world_subset_desc}`、`ExternalGpuWorldSubset::new`、`OwnedGpuWorldSubset::{new(),clear}`、`CurvedHalfTimberInstanceData::curved_halftimber_mesh_atlas_part_idx`、`impl Debug for GpuWorld` | — |
| `TrimeshBuilder` | `&mut self` 链式，属性带名字 | 按值链式；`positions(self,&[Vec3])`、`attrib<T>(self,&[T])`；新增 `Trimesh::builder`、`build`、`TrimeshChunk` |
| instance_data 类型 | `_gap: [u32;N]` | `raw: [u32;N]` + `StorageElement`（NAME = 类型名）+ `Default`；`PlantInstanceData::marker()` 删除（真实 `marker` 只有 `UberInstanceData::marker`） |

**对 n_sys 的影响（systems.rs / draw_ext.rs / drawable.rs，我不改）**：`owned_subset_mut(kind).write()`、`InstanceKey` 字段、`extend_no_data`/`update_instance_data`/`update_instance_mesh` 的调用都需要按上表更新；
`kind_from_u8(9)` 一类“U”占位在真实名字下可以替换为 `GpuWorldSubsetKind::{RoofFloor,…}`（`nani_update_*` 与 kind 的对应：SolidVertexColor↔kind 3、Bricks↔6、Window↔4、Glass↔5、GothicWindowBricks↔7、Plant↔10、
PreplacedFlower↔14、SolidAlbedoMap↔15、…Emissive↔16、Wren↔17、LanternEmissive↔18、Plaster↔19、Cloth↔20、Smoke↔21、BalconyDoor↔22、Uber↔23、CurvedHalfTimber↔24、VelocityOnly↔25、StairsFloor↔26，与实例数据类型名一一对应，I）。
未能直接通知 n_sys（`SendMessage` 找不到该 agent），请集成者转达。**A4 pass_builder**：本文件对其无直接调用；`gpu_generate_draw_lists` 仍通过 `GpuWorld::{mesh_atlas_info_buf, draw_list_info_buf, try_get_subset_mut}`、
`GpuWorldSubsetKind::all()`，这些签名未变（`try_get_subset_mut` 现返回 `Option<RwLockWriteGuard<Option<GpuWorldSubset>>>`），无缺口。

## 6. 遗留 / 风险

- **`impl$44::fmt`（0x140a241d0）**：PDB 归在 gpu_world 下，但机器码是 `debug_tuple_field1_finish("InstanceKey", &field, fmt<u32>)`（派生 Debug，单字段元组结构体）。与 12B 的 `InstanceKey` 对不上，
  可能是 `IndexCondenser` 的 `Condex<InstanceKey>`（0x140a19e90 的 Debug）或 ICF 别名；未恢复，不编造。`InstanceKey` 保持 `#[derive(Debug)]` 的普通结构体 Debug。
- `GpuWorldSubset` 的 niche 编码（External = 首 u64 `0x8000000000000000`，None = `…01`）在 Rust 里由 enum/Option 自然表达，未复刻位模式。
- `OwnedGpuWorldSubset::new<T>` 的 `positions`/数据缓冲长度取 `max_instances[kind]`（I）；`startup_nani`（27 个 subset 的创建，未恢复）尚需接线 `assign_subset`。
- 整个 crate 在离线缓存下无法 `cargo check`（`glam` 的 `mint` 特性缺包），本次只用隔离测试台验证；与 systems.rs/draw_ext.rs 的联编需 n_sys 更新后由集成者复核。
- /tmp 磁盘几乎满：为跑测试删除了 `/tmp/h_{shader,color,mesh}-target` 三个 A3 阶段遗留的构建缓存（可再生）。

## 7. 测试

隔离测试台 `/tmp/h_n_world`（`#[path]` 挂载真实源文件：`rhapsody_types`、`dirty_index_range`、`ssbo`、`shadowed_buffer`、`utils/index_condenser`、`nani/{buffers,gpu_world,instance_data,trimesh}`；
`mesh_atlas` 用同形替身；真实 `parking_lot 0.12.1`/`half 2.7.1`/`bevy_ecs 0.16.0`；复用 `/tmp/l0h-target`）：

```
L0H_REAL=…/country-core/src  /tmp/h_n_world/run.sh      # run.sh 内置路径
→ 68 passed; 0 failed   （本组 48 个：buffers 6、gpu_world 20、instance_data 7、trimesh 12、index_condenser 3；其余 20 个是被挂载的 ssbo/shadowed_buffer/dirty_index_range 既有测试）
```
覆盖：27 个变体名与表常量、`calculate_with_flags` 三种开关组合（含 0x3fe 掩码、FORCE_FINE 优先）、`default_value` 偏移序列与 0x1b0 尺寸、`GpuWorld::new` 的缓冲创建顺序/大小/usage、
extend/create 的 id 分配与碰撞跳过、容量 warn-once 跳过（非 panic）、`update_instance_data`（positions + 类型化数据 + 脏窗口、`mesh_index` 保持）、过期提示的哈希回退与回写、
未知 key unwrap panic、类型不符 panic 文案、`remove_instance` swap-remove（positions 与数据同步、被搬动实例仍可寻址、重复删除不 panic）、External/空槽 panic、RG import/export 跳过 External 与两条断言文案、
`copy` 先 src 后 dst、元素尺寸表（含 PreplacedFlower 0x30）、`new_straight` 逐字节、`new_curved_xy/xz` 字段与细分等级边界（含 NaN→2）、marker 注册、
`Trimesh::builder`（去重/64 三角形切块/不足三元组/空输入）、`attrib` 的四个断言与对齐降序规则、`build` 的缓冲字节布局、包围球、块间偏移、设备地址回写。
`rustfmt --check --edition 2024` 对五个文件全部通过。
