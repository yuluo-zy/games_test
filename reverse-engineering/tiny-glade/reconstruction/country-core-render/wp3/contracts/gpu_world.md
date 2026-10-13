# GpuWorld 业务契约 — gpu_world.rs（158 procs，分 7 个子功能）

标注：**O**=反编译/PDB 直接支持；**I**=调用链/行号组合；**U**=未恢复。源 SHA256 `26c9dbcabf908aabb9393e4f00cc02e97cf43dba093a2adc3414083868809051`；所有地址为 B patched EXE（cf048e27...）。

核心角色：country-core 的 CPU↔GPU 实例桥。`GpuWorld` 持有 27 个 `GpuWorldSubsetKind` 的 subset 槽、两个 TypedStorageBuffer（`NaniMeshAtlasPartInfo`/`NaniDrawListInfo`）和一个总实例计数；`OwnedGpuWorldSubset` 是每个 subset 的 CPU 侧稀疏实例仓库（HashMap<InstanceKey,u32> + 紧凑 key 数组 + 类型化实例数据缓冲 + min/max 原子 + 计数）。

## 0. 关键类型（O，尺寸来自反编译字段访问）

### `GpuWorld`（≥0x14e8 字节）
| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00..+0x1440 | 27 × 0xc0 字节 subset 槽数组（每个槽在 +0x98/+0xa0 处为 trait 对象 {data_ptr, vtable}） | `import_to_rg` 0x1409921c0 循环 `for (0xa0; !=0x14e0; +=0xc0)`；`GpuWorld::new` memcpy 0x1440 |
| +0x1440 | `total_instance_count: u64`（创建/删除的净实例数累加） | `create_instances` 尾部 `*(+0x1440) += key_count`；`new` 置 0 |
| +0x1448 | 第二个 u64（计数或保留） | `new` 置 0（typed gap） |
| +0x1450 | `*mut [RwLock<Option<GpuWorldSubset>>; 7]`（7 个"主"subset 锁数组） | `new` 中 `array::default<RwLock<Option<GpuWorldSubset>>>` + 堆分配 7×16B |
| +0x1458 | `TypedStorageBuffer<NaniMeshAtlasPartInfo>` | `new` 中 `TypedStorageBuffer::new<NaniMeshAtlasPartInfo>`；`mesh_atlas_info_buf` |
| +0x1480 | `TypedStorageBuffer<NaniDrawListInfo>` | `new` 中 `TypedStorageBuffer::new<NaniDrawListInfo>`；`draw_list_info_buf` |
| +0x14a0 | `nani_calculate_draw_list_params` 结果数据指针 | `new` 中 `local_148 = local_a8`（params 的 Vec 数据指针） |

### `OwnedGpuWorldSubset`（u64 单位索引，O）
| 索引 | 偏移 | 字段 |
|---|---|---|
| [0]/[1]/[2] | +0x00/+0x08/+0x10 | `Vec<ChunkId>` {ptr, cap, len}（u32 元素；`IndexCondenser::remove`/`create_instances` 写 `*(+0x08 + idx*4)`） |
| [3]/[4]/[6] | +0x18/+0x20/+0x30 | `HashMap<InstanceKey, u32>` {buckets, mask, has_items}（`create_instances`/`update_instance_data`/`remove_instance` 探针） |
| [9]/[10] | +0x48/+0x50 | `AtomicU64 min / max`（实例索引下界/上界，CAS 循环） |
| [0xb]/[0xc] | +0x58/+0x60 | `Vec<InstanceKey>` 紧凑 key 数组（16B/元素：{u64, u32, f16 sort}，见 update_instance_data 写 +0x0c f16） |
| [0x12]/[0x13] | +0x90/+0x98 | 类型化实例数据缓冲 trait 对象 {ptr, vtable}（`T: NaniInstanceData`；vtable +0x18=type-id 断言、+0x28=数据 Vec 头、+0x40=import_to_rg、+0x50=remove） |
| [0x14] | +0xa0 | `instance_count: u64`（上限校验） |
| [0x16] | +0xa8 | `next_instance_index: u32`（单调分配，无 freelist 复用——swap_remove 维护紧凑性） |

### `GpuWorldSubsetKind`（u8 判别；**27 个变体**，O）
- 枚举长度证据：`nani_calculate_draw_list_params`/`default_value` 的 switch 上限 0x1b=27；`import_to_rg` 27 个槽。
- 已命名变体（public-symbols/cull_shaders）：Main, Bush, CurvedHalftimber, Flowers, Plaster, RoofTile, Smoke, StairsFloor, WoodenPlank（对应 `_nani_cull_and_bucket{,_bush,_curved_halftimber,_flowers,_plaster,_roof_tile,_smoke,_stairs_floor,_wooden_plank}`）。其余 18 个变体名 **U**（typed gap）。
- 方法（PDB 证据，多被内联）：`shader`、`try_from_shader`（0x141572870 ron deserialize `visit_str<ron::Error>`）、`max_instances`（`DAT_142aba790/142c77010[kind]` 常量表）、`max_draws_per_list`（`DAT_142aa14c0[kind]`）、`use_fine_grained_sorting`（`NANI_FORCE_FINE_GRAINED_SORTING` + kind<2 规则）、`draw_list_index`、`gpu_world_subset_desc::{owned,external,owned_custom}`、`impl$2::from`、`impl$44::fmt`（0x140a241d0 Debug）。

### `InstanceKey`（16 字节，O）
`{ u64 chunk_id / key_hi; u32 instance_index; f32 sort_key }`。证据：`create_instances` 按 16B 复制；`update_instance_data` 写 {8B, 4B, f16(来自 f32)}；fast-path `index = key[1]` 直查 `Vec<ChunkId>`。

### `InstanceHandle`（O）
`{ kind: GpuWorldSubsetKind, index: u32 }`（create_instances 的 pending `Vec<(u32,u32,u8)>`；`remove_instance` 读 `*(key+8)` 为 kind 字节）。

### `NaniInstanceData` trait（impl$14，O）
每实例类型的方法：`type_name`（0x141572130 系 13B）、`as_any`/`as_any_mut`（11B）、`gpu_flush`（9B）、`rg_handle`（0x141572410 43B）、`import_to_rg`（0x141572440 82B）、`export_from_rg`（0x1415724a0 73B）、`copy`（0x1415724f0 起，T 尺寸×复制 + 原子 min/max 更新）、`register_required_components`（0x141572d80 系 1776B）。类型化的 `Vec<T>` 由各 `update_instance_data<T>`/`copy<T>` 的元素尺寸体现（0x30/0x40/0x44）。

## 1. 子功能 A：构造与布局（`GpuWorld::new`，0x1409917d0；`owned_subset_mut` 0x140991c50）

`GpuWorld::new() -> GpuWorld`（O）：
1. `buffers::nani_calculate_draw_list_params()`（见 buffers.md）→ 得到 27 种 subset 的绘制参数（4 个 `Vec<u32>` + 4 个 u32 汇总）。
2. 把 params 的数据克隆进一个 `Vec<u32>`（容量 4×len，元素 4B）→ 存到 `+0x14a0`。
3. `array::default<RwLock<Option<GpuWorldSubset>>>` 建 7 元素锁数组，堆分配，存 `+0x1450`。
4. 复制 0x1440 字节（27 槽数据）到 `+0x0`。
5. `TypedStorageBuffer::new<NaniMeshAtlasPartInfo>`（+0x1458）与 `TypedStorageBuffer::new<NaniDrawListInfo>`（+0x1480）。
6. `+0x1440/+0x1448 = 0`。

`GpuWorld::owned_subset_mut(kind) -> subset`（O）：按 kind 从槽数组取 `RwLock<Option<GpuWorldSubset>>` 写锁，`Option` 空则 `unwrap_failed`（0x140991c50）。`get_subset`/`try_get_subset_mut`/`owned_subset_exclusive_mut` 为同族访问器（I，内联）。

## 2. 子功能 B：实例创建（`create_instances` 0x14156d0d0；`extend_no_data` 0x140992280；`extend`；`new`；`clear`）

`GpuWorld::create_instances<T, F>(kind, keys: &[InstanceKey], cap, create: F, pending: &mut Vec<(u32,u32,u8)>)`（O）：
1. `owned_subset_mut(kind)` 拿写锁。
2. `count = min(keys.len(), cap)`；cap 语义 **U**（推断为剩余容量）。
3. 对每个 key（16B）：
   a. `i = subset.next_index++`。
   b. 若 subset 有 hash 表（`[6]!=0`），`HashMap<InstanceKey,u32>::insert(key→i)`；`Vec<ChunkId>` push i。
   c. `pending` push `(i, key_low, kind)`（12B 记录；消费方 **U**，推断为 CPU 侧 indirection 表更新）。
   d. `Vec<InstanceKey>`(+0x58) 按索引写 key（16B），`panic_bounds_check` 若越界。
   e. CAS 更新 min/max（+0x48/+0x50）。
   f. 经 trait 对象（+0x90/+0x98）取类型化缓冲；**type-id 断言**（常量 `-0x990b32ccf5b0500` / `0x2095664170371110`，monomorphization 专属）失败 → `panic_fmt("... type mismatch ...")`。
   g. 把 `create(key)` 的 T（0x30 字节/元素）写入数据 Vec（+0x28 头）索引 i；CAS 更新缓冲侧 min/max。
   h. `instance_count++`。
4. 容量用尽：`panic "Max instance count reached for ..."`（含 kind 名格式串）。

`OwnedGpuWorldSubset::extend_no_data(keys, callback, kind)`（O）：nani_maintain_instances 的内层——同 create_instances 的 a–h 路径，但 T 数据由调用方另行写入；`callback`（经 stack+0x20 传入的函数指针）每实例调用一次（接收 `(index, kind)` 打包值）。无 hash 表的 External 变体跳过 b 的查找（`[6]==0`）。

`OwnedGpuWorldSubset::new/extend/instance_count/clear/atlas_mesh_count/subset_mesh_count`（O 存在，多内联；`new` inline 21 处、`extend` inline 1 处）。`clear` 重置计数与索引（I）。

## 3. 子功能 C：实例更新（`update_instance_data<T>` 0x14156db40 系 ×28；`update_instance_mesh` 0x140991d60）

`OwnedGpuWorldSubset::update_instance_data<T>(key, data: T)`（O，28 个 monomorphization 共址）：
1. fast-path：`index = key[1]`（u32）；若 `index < len(Vec<ChunkId>)` 且 `chunk_id_vec[index] == key[0]` 低 32 位 → 直接命中。
2. 否则 `HashMap<InstanceKey,u32>::get(key)`（hashbrown 探针）→ `unwrap_failed` 若缺（外部 subset 无 hash → 直接 unwrap_failed）。
3. 写紧凑 key 记录到 +0x58（16B = {key[0], key[1], f16(sort_key)}，`half::f16::from_f32` 转换）。
4. 锁类型化缓冲（+0x90/+0x98），**type-id 断言**（如 Uber 常量 `-0x62d1b05a65660029`/`0xe47589a4beda355c`）失败 → panic。
5. 把 T 复制到数据 Vec 索引（Uber=0x40、GothicWindowBricks/VelocityOnly/Wren=0x44、PlacementDataSsbo=0x30 元素尺寸）。
6. CAS 更新 min/max。

`GpuWorld::update_instance_mesh(entity, desc, transform)`（0x140991d60，O）：nani_maintain_instances 的 Changed<NaniInstanceDesc> 分支——重算实例 mesh/atlas 引用并更新（具体字段流 U，主体在维护系统）。

## 4. 子功能 D：实例删除（`remove_instance` 0x140991f60；`clear`）

`GpuWorld::remove_instance(key: InstanceKey)`（O）：
1. `kind = key[8..]`（InstanceHandle 编码）。
2. `owned_subset_mut(kind)`。
3. `IndexCondenser<InstanceKey>::remove(key)`：从索引压缩器移除 key→index；返回是否命中（0/1）。
4. 命中：`count = instance_count-1`；`swap_remove` 紧凑 key 数组（+0x58，16B/元素）：末元素复制到被删槽；CAS 收缩 min/max；经 vtable+0x50 调缓冲的 `remove`（数据 Vec 同样 swap_remove）。
5. 未命中：仅记 log（`log::MAX_LOG_LEVEL_FILTER` 检查）。

## 5. 子功能 E：生命周期与 RG 接线（`import_to_rg`/`export_from_rg` 0x1409921c0/0x140992220；impl$14 族；`gpu_flush`；`rg_handle`；`copy`）

- `GpuWorld::import_to_rg(rg)`（O）：遍历 27 槽（+0xa0..+0x14e0 step 0xc0），对非空 subset 调 vtable+0x40（子集 import_to_rg）。`export_from_rg` 对称（0x140992220，85B）。
- impl$14 族（O，monomorphized ×28）：
  - `copy<T>(src_idx, dst_idx)`（0x1415724f0 系）：数据 Vec 内元素尺寸复制（0x40 主流 / 0x44 Wren/GothicWindowBricks / 0x30 PlacementData）+ CAS min/max。
  - `rg_handle<T>`（0x141572410 43B）：返回 RG 资源句柄。
  - `gpu_flush<T>`（9B 薄封装）；`type_name`（13B）、`as_any`/`as_any_mut`（11B）为 Any 降级。
- `OwnedGpuWorldSubset::gpu_flush`（inline 1；`clear` inline 1）——缓冲写后标记 GPU 侧脏（I）。

## 6. 子功能 F：枚举与序列化（`GpuWorldSubsetKind`）

- `try_from_shader`（0x141572870 `_::impl$0::deserialize::visit_str<ron::Error>`，O）：RON 字符串 → kind 变体；非法 → `Error::Message`（1290B 反编译含错误分支）。
- `gpu_world_subset_desc::{owned, external, owned_custom}` + 各自 `closure$0`（O 存在）：返回 subset 描述（尺寸/布局），`owned_custom` 带自定义参数。`impl$2::from`、`impl$41::eq`、`impl$19::eq`、`impl$21::hash` 为派生比较/hash。
- `impl$17::fmt`（Debug 输出，0x140992a60 等 5 处 monomorphization）。

## 7. 子功能 G：组件注册（`impl$36::register_required_components<T>` ×20，0x141572d80 系）

每个 1776B 的 monomorphization 调用 `ComponentsRegistrator::register_component<InstanceDataMarker<T>>`（为 T 的实例数据组件注册，以及 4 个相关 marker 组件；inlinee 证据 `register_component<InstanceDataMarker<VelocityOnlyInstanceData>>` 等）。`impl$122::register_required_components`（instance_data.rs 0x140990a80）同族。纯 bevy ECS 胶水，无游戏数值。

## 8. 消费者接线（O/I）

- CPU 侧：`nani_maintain_instances`（systems.rs）调 `owned_subset_mut`+`extend_no_data` 建实例、`remove_instance` 删、`update_instance_mesh` 改；各 `nani_update_*` 调 `update_instance_data<T>`。
- GPU 侧：`gpu_generate_draw_lists`（0x140987dd0，见 evidence/render-architecture/0x140987dd0.c）经 `ComputePass::dispatch_with<cull_and_bucket::closure_env$1>`（0x14160a2e0）读 subset 槽、`NaniMeshAtlasPartInfo`/`NaniDrawListInfo` 缓冲；`draw_ext::nani_draw`（0x140904ae0）把 subset 的间接绘制绑定到 DrawBuilder。
- RG 生命周期：`import_to_rg`/`export_from_rg` 由 `render_world`（0x1404ec110）的 RG import/export 消费。

## 9. Unknown / typed gaps

- 27 个 kind 变体除 9 个外未命名；每 kind 的 `max_instances`/`max_draws_per_list` 数值表（`DAT_142aba790`/`DAT_142aa14c0`）未逐项读内存。
- `InstanceKey` 的 f32 sort_key 精确字段名与 GPU 侧解释 U；+0x1448 字段语义 U。
- `OwnedGpuWorldSubset` 空闲/复用策略：observed 单调 `next_index` + swap_remove，无显式 freelist。
- `create_instances` 的 `cap` 参数来源与 `pending` 消费方 U。
- 各 `nani_update_*` 的实例数据计算（transform→struct 字段）见 systems/instance_data 契约，GPU 布局按元素尺寸对齐。
