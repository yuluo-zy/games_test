# render/sparse_instance_buffer.rs

> 文件作用：**GPU 驱动稀疏实例化**——`SparseInstanceBuffer<T>`（0x90 B，32 实例/块分块 + byte_map 活跃计数 +
> 双位图 active/dirty + 自由块栈 + 分配记录）+ `SparseInstanceBufferWriter<T>`（0x58 B，顺序 push 写实例缓冲）。
> 管理分配/隐藏/显示/回收（take_out_waste），绘制面出多实例间接范围（`allocation_to_draw_multi_instanced` /
> `to_draw_multi_instanced`），`gpu_flush` 上传脏块。31 业务 = 4 类型的模板族 + `AllocationHandle::is_null`。
> 证据：`reconstruction/country-core-render/wp2/contracts/sparse_instance_buffer.md`（O 全量）。

函数总计 53：业务 31 / 内联无独立符号 13 / 派生胶水 9

## 布局表（O，wp2 合同）

### `SparseInstanceBuffer<T>` — 0x90 B（18 qword）

| 偏移 | 字段 | 状态 |
|---|---|---|
| 0x00 | `pending_resource_handles: parking_lot::RwLock<Vec<rhapsody ResourceHandle>>`（写者标志 0/8 快路径） | O |
| 0x08 | `chunk_count: usize` | O |
| 0x10 | `free_chunks: Vec<u32>`（初始化降序 `[chunk_count-1 … 0]`） | O |
| 0x20 | `byte_map: Vec<u8>`（len = chunk_count+1；每块活跃计数 0..=32；byte[chunk_count]=0x20 哨兵） | O |
| 0x30 | `active_bitset: Vec<AtomicUsize>`（len = chunk_count/64） | O |
| 0x40 | `slot_meta: Vec<u32>`（len = chunk_count*32；per-slot u32） | U 角色（分配/清零 O，消费者未反编译） |
| 0x50 | `dirty_bitset: Vec<AtomicUsize>` | O |
| 0x60 | `allocations: Arc<parking_lot::RwLock<Vec<Allocation>>>` | O |
| 0x68 | `buffer: TypedStorageBuffer<T>`（40 B） | O |

`Allocation { field0 @0x00（非空即释放，U 角色）, chunk_ids_ptr @0x08, chunk_ids_len @0x10 }`；
`AllocationHandle`（16 B）= `{ word: u64 @0x00, allocation: Option<NonNull<Allocation>> @0x08 }`；`ChunkId` = u32。

### 类型常量（O，impl$10::new_storage_buffer 单态化）

| T | chunk_count | byte_map len | slots | 元素大小 |
|---|---|---|---|---|
| RoofTileSsbo | 0x2000 (8192) | 0x2001 | 0x40000 | — |
| BrickTransformSsbo | 0x10000 (65536) | 0x10001 | 0x200000 | 0x60 (96) |
| WoodenPlankSsbo | 0x10000 (65536) | 0x10001 | 0x200000 | 0x40 (64) |
| SparkleSsbo | 0x6000 (24576) | 0x6001 | 0x300000 | — |

### `SparseInstanceBufferWriter<T>` — 0x58 B（11 qword）

byte_map(ptr,len)@0x00/0x08、active_bitset(ptr,len)@0x10/0x18、`allocated_chunks: Vec<ChunkId>`@0x20（cap-first）、
`instance_data_ptr`@0x38（映射实例缓冲，stride size_of::<T>()）、`total_slots`@0x40、`free_chunks: RwLock<Vec<u32>>`@0x48、
dirty_bitset(ptr,len)@0x50/0x58、`current_chunk: u32`@0x60。

## 业务函数

| 族 | 实例数 | 代表地址 | 逆向结论 | 证据 |
|---|---|---|---|---|
| `impl$10::new_storage_buffer<T>` | 4 | 0x1412e4a90（RoofTile）/ 0x1412e5800（Brick）/ 0x1412e5170（WoodenPlank）/ 0x1412e5f50（Sparkle） | **O**：分配并初始化 byte_map（零 + 哨兵 0x20）、slot_meta（0x40000 u32 零）、free_chunks（降序）、双位图（from_iter AtomicUsize::new(0)）、Arc<RwLock<Vec<Allocation>>>、TSB、pending-handles RwLock；设备参数守卫（`if (in_R8 != 0) panic`，语义 U） | render-top/decomp/decomp-0x1412e4a90.txt（原 wp2 合同） |
| `SparseInstanceBuffer::hide` | 4（每类型 1） | 0x1412e3c00 | **O**：unwrap handle.allocation；逐块 bounds `chunk_id/64 < bitset_len`，清 active 位（AtomicUsize fetch_and） | render-top/decomp/decomp-0x1412e3c00.txt（原 wp2 合同） |
| `SparseInstanceBuffer::show` | 4 | 0x1412e3c80 | **O**：hide 镜像，置 active 位 | render-top/decomp/decomp-0x1412e3c80.txt（原 wp2 合同） |
| `SparseInstanceBuffer::take_out_waste` | 4 | 0x1412e3d00（BrickTransform） | **O**：扫描 allocations：每块 byte_map=0、清 active 位、压回 free_chunks（reserve + reverse copy）、释放 Allocation box 与 field0 指针；用 pending-handles RwLock 快路径 | render-top/decomp/decomp-0x1412e3d00.txt（原 wp2 合同） |
| `SparseInstanceBuffer::gpu_flush<T>` | 4 | 0x1412e2f10（BrickTransform） | **O**：先 take_out_waste；扫 dirty_bitset（trailing zeros 找最低置位块）累积非重叠块区间 `[start,end)`，逐词清零；每区间转元素范围 `[chunk*32, chunk*32+32)`（`<<5`）→ `TypedStorageBuffer::gpu_write_ranges_nonoverlapping<T>`；puffin scope | render-top/decomp/decomp-0x1412e2f10.txt（原 wp2 合同） |
| `SparseInstanceBuffer::allocation_to_draw_multi_instanced<T>` | 4 | 0x1412e3a60（RoofTile） | **O**：unwrap handle；逐块 `start_slot=chunk*32`、`count=byte_map[chunk]`；累积连续槽段 `(start_slot, count)` 对（grow_one 输出缓冲）；返回 `(0, 4, range_count)`（多绘制命令三元组，I：first=0/stride=4/count） | render-top/decomp/decomp-0x1412e3a60.txt（原 wp2 合同） |
| `impl$3::to_draw_multi_instanced<T>` | 4 | 0x1412e4250（RoofTile） | **O**：同形但遍历整个 active_bitset（全部活跃块），返回 `(0, 4, count)` | render-top/decomp/decomp-0x1412e4250.txt（原 wp2 合同） |
| `SparseInstanceBuffer::buffer` | 1 | 0x140005c50（5 B 桩） | **O**：`mov rax,rcx; ret`——访问器（`&TypedStorageBuffer<T>`）；与 `impl$12::handle` ICF | hex 手解 0x140005c50（rne-read --hex 本批；原 wp2 合同 + 本批字节复核） |
| `impl$12::{as_any, as_any_mut, handle}` | 3×4 | 0x1412e6640..0x1412e6670（11 B 桩） | **O**：trait-object 接口（`as_any`/`as_any_mut` = 自身指针 + Any 虚表；`handle` = `lea rax,[rcx+0x68]`）——供 `AssetSsboLibrary::get_rg_buffer<T>` 使用 | hex 手解 0x1412e6640（rne-read --hex 本批；原 wp2 合同） |
| `AllocationHandle::is_null` | 1 | 0x140985f30 | **O**：`self.word == 0` | hex 手解 0x140985f30（rne-read --hex 本批；原 wp2 合同 + skeleton task-index 证据） |
| `SparseInstanceBufferWriter::push<T>` | 4 | 0x1412e4450（BrickTransform 元素 0x60，块步长 0xc00） | **O**：`current_chunk` 未满（byte_map<32）→ 槽写 + byte_map+1 + 置脏位；满 → 锁 free_chunks（空则 no-op）pop 新块、append allocated_chunks、写 `chunk*32` 槽、置 active+dirty 位、current_chunk=新块 | wp2 合同 |
| `SparseInstanceBufferWriter::clear<T>` | 3 | 0x1412e4760（BrickTransform） | **O**：current_chunk = byte_map_len-1（满哨兵）；逐 allocated_chunks 块 byte_map=0 + 清 active 位；drain 回 free_chunks（reverse append）；**脏位不清**（留给 gpu_flush 上传） | render-top/decomp/decomp-0x1412e4760.txt（原 wp2 合同） |

## 族明细（每类型 VA 对）

```
BrickTransformSsbo:  new_storage_buffer@0x1412e5800  hide@0x1412e3c00  show@0x1412e3c80  take_out_waste@0x1412e3d00
  gpu_flush@0x1412e2f10  allocation_to_draw_multi_instanced@0x1412e3a60  to_draw_multi_instanced@0x1412e4250
  push@0x1412e4450  clear@0x1412e4760  buffer@0x140005c50  (+2 个未标注 VA 的 push/clear 变体)
RoofTileSsbo:     new_storage_buffer@0x1412e4a90  + 其他 8 个 @0x1412e3a60..0x141a3af80（wp2 已反编译 allocation_to_draw/to_draw）
WoodenPlankSsbo:  new_storage_buffer@0x1412e5170  + 其他 7 个 @0x14120b030..0x1412e6640
SparkleSsbo:      new_storage_buffer@0x1412e5f50  + 其他 5 个 @0x141209f40..0x1412e6640
```

> 注：每类型族内 hide/show/take_out_waste/gpu_flush/push/clear/to_draw 的精确 VA 配对未逐条反编译（除标注者），
> 按「同类型簇地址连续」推断（I）；总覆盖 31 业务 = 每类型实例数之和（BrickTransform 10 + RoofTile 8 + WoodenPlank 7
> + Sparkle 5）+ `AllocationHandle::is_null` 1。

## 内联/无独立符号函数

- `allocate` — **I**：pop free_chunks、置 byte_map/active、建 Allocation、返回 handle
- `write_allocation` — **I**：写分配槽 + 置脏（closure 用 checked_sub，O 依赖）
- `free_impl` — **I**：单 handle 版 take_out_waste
- `bitmap_set` / `bitmap_unset` — **I**：`AtomicUsize::fetch_or` / `fetch_and` 位图操作（O 依赖）
- `drain_chunk_bitmap` / `iter_chunk_bitmap` / `iter_all_chunks` / `iter_allocation_chunks` — **I**：位图迭代器族
- `to_draw_multi_instanced_impl` — **I**：两绘制构建器的共享实现
- `take_out_waste` 的 RwLock 快路径辅助（I）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. 分配/回收算法
- **分配（allocate，内联）**：pop free_chunks → 置 byte_map[chunk]=32、active 位 → 建 `Allocation{field0, chunk_ids(ptr,len)}` → 返回 16B `AllocationHandle{word:u64, allocation:Option<NonNull>}`；`is_null = word==0`（0x140985f30）。
- **hide/show**（0x1412e3c00/0x1412e3c80）：unwrap handle.allocation；逐块 bounds `chunk_id/64 < bitset_len`；`AtomicUsize::fetch_and/fetch_or` 清/置 active 位。
- **take_out_waste**（0x1412e3d00）：扫描 allocations：每块 `byte_map=0`（0 活跃）→ 清 active 位 → 压回 free_chunks（reserve+reverse copy）→ 释放 Allocation box 与 field0 指针；pending-handles RwLock 快路径（写者标志 0/8）。
- **Writer::push**（0x1412e4450）：`current_chunk` 未满（byte_map<32）→ 槽写 `instance_data_ptr + chunk*32*es + slot*es` + byte_map+1 + 置脏位；**满 → 锁 free_chunks（空则 no-op）pop 新块**、append allocated_chunks、写 `chunk*32` 槽、置 active+dirty、current_chunk=新块。
- **Writer::clear**（0x1412e4760）：current_chunk = byte_map_len-1（满哨兵）；逐 allocated_chunks 块 byte_map=0 + 清 active；drain 回 free_chunks（reverse append）；**脏位不清**（留给 gpu_flush 上传）。

### 2. `gpu_flush<T>`（0x1412e2f10）—— 脏块上传
先 take_out_waste → 扫 dirty_bitset（**trailing zeros 找最低置位块**）累积非重叠块区间 `[start,end)`，逐词清零 → 每区间转元素范围 `[chunk*32, chunk*32+32)`（`<<5`）→ `TypedStorageBuffer::gpu_write_ranges_nonoverlapping<T>`；puffin scope。

### 3. 绘制范围构建
- `allocation_to_draw_multi_instanced`（0x1412e3a60）：unwrap handle；逐块 `start_slot=chunk*32`、`count=byte_map[chunk]`；累积连续槽段 `(start_slot,count)` 对（grow_one 输出缓冲）；返回 `(0, 4, range_count)`。
- `to_draw_multi_instanced`（0x1412e4250）：同形但遍历**整个 active_bitset**（全部活跃块），返回 `(0,4,count)`。
- 消费方：`DrawBuilder::draw_multi_instanced`/`multi_draw_indirect`（0xb8B 命令）；stride 4 与 WP1 对编码跨包一致。

### 4. 常量（O，impl$10::new_storage_buffer 单态化）
RoofTile chunk_count 0x2000/slots 0x40000；BrickTransform 0x10000/0x200000/es 0x60；WoodenPlank 0x10000/0x200000/es 0x40；Sparkle 0x6000/0x300000。
- 纠错：`slot_meta` u32 表与 `Allocation.field0` 无已反编译消费者——不编造语义；旧骨架把 push 写直接当 flush 内容（实际 push 写映射缓冲，gpu_flush 再从 CPU 数组重传脏块）。

## 派生胶水（9 项：closure ×7、drop ×2）

- closure$0 ×6 + closure$1（write_allocation/push 的闭包）、drop ×2（Allocation 族）——标准胶水，不补。

## 纠错/差异（wp2 合同集成注）

- `slot_meta` u32 表（0x40）与 `Allocation.field0` 无已反编译消费者——**不要编造语义**。
- `(0, 4, count)` 返回三元组由 WP1 `DrawBuilder::draw_multi_instanced`/`multi_draw_indirect`（0xb8 B 命令）消费——stride 4 与对编码跨包一致。
- 旧版骨架把 push 写直接当 flush 内容：实际 push 写映射缓冲，gpu_flush 再从 CPU 数组重传脏块（冗余重写，为回收路径保留，I）。
- `buffer` 桩 0x140005c50 与 `impl$12::handle` 均 5 B `mov rax,rcx`（本批字节复核；contract 记为 0x140005c50）。
