# render/shadowed_buffer.rs

> 文件作用：**CPU 影子 + 脏区间 GPU 刷新的双缓冲原语**——`ShadowedBuffer<T>`（72 B）= `DirtyIndexRange`（AtomicUsize
> 含端点对，usize::MAX 空哨兵）+ CPU 影子 `(ptr, len)` + 内嵌 `TypedStorageBuffer<T>`。`get_mut`/`write`/`copy`
> 改影子并标脏，`gpu_flush` 在 nani 绘制路径把脏区间上传并清脏。39 业务 = `new`×22 + `gpu_flush`×17 单态化。
> 证据：`reconstruction/country-core-render/wp2/contracts/shadowed_buffer.md`（O 全量）+ `dirty_index_range.md`。

函数总计 43：业务 39 / 内联无独立符号 4 / 派生胶水 0

## 布局表（O，wp2 合同）

| 偏移 | 字段 | 证据 |
|---|---|---|
| 0x00 | `dirty.start` — `AtomicUsize`（含端点，`usize::MAX` = 空哨兵） | gpu_flush / get_mut |
| 0x08 | `dirty.end` — `AtomicUsize`（含端点） | gpu_flush / get_mut |
| 0x10 | `data.ptr` — CPU 影子元素指针 | new / gpu_flush |
| 0x18 | `data.len` — 元素数 | new / gpu_flush |
| 0x20 | `buffer: TypedStorageBuffer<T>`（40 B，布局见 render/ssbo 章） | new |

- 无 capacity 字段（`data` 建后不再增长；脏对 = `DirtyIndexRange`：`add` = `fetch_min(start)` + `fetch_max(end)`，空 = `start > end`）。

## 业务函数

| 族 | 实例数 | 代表地址 | 逆向结论 | 证据 |
|---|---|---|---|---|
| `ShadowedBuffer::new<T>` | 22 | 0x14168f870（SolidAlbedoMapInstanceData，元素 0x20） | **O**：`fn new(initial: …, device)`；CPU 缓冲按 initial 长度 realloc/shrink（`__rust_realloc`/`__rust_dealloc`，OOM → handle_error）；`TypedStorageBuffer::new<T>` 同元素数；存 `data(ptr,len)` + buffer 5 qword + `dirty=(usize::MAX, 0)` | wp2 合同 |
| `ShadowedBuffer::gpu_flush<T>` | 17 | 0x141691b30（元素 0x30；Uber 0x40 等按元素大小分体） | **O**：`fn gpu_flush(&mut self, device: &GraphicsDevice)`；`dirty.start > dirty.end` → no-op；`exclusive_end = dirty.end + 1`（**含端点→半开**，load-bearing）；`slice_index_order_fail`/`slice_end_index_len_fail` 断言；守卫 `buffer.byte_size >= exclusive_end * size_of::<T>()`（否则 unwrap_failed）；`device.update_buffer_bytes(&buffer, &[(start*es .. end*es+es)], slice_as_bytes(&data[start..exclusive_end]))`；清脏为 `(usize::MAX, 0)`；puffin scope | wp2 合同 |

## 族明细（22 类型；有 2 个实例的 = new + gpu_flush 各一，单实例的仅 new）

```
SolidAlbedoMapInstanceData:    new@0x14168f870  gpu_flush@0x1416921a0
SmokeInstanceData:             new@0x141690040  gpu_flush@0x1416921a0
SolidVertexColorBricksInstanceData: new@0x14168fb90  gpu_flush@0x1416921a0
LanternEmissiveInstanceData:   new@0x14168feb0  gpu_flush@0x1416921a0
GothicWindowBricksInstanceData:new@0x1416901d0  gpu_flush@0x141692e70
PlacementDataSsbo:             new@0x141690360  gpu_flush@0x141691b30
StairsFloorInstanceData:       new@0x141690500  gpu_flush@0x141691b30
SolidVertexColorInstanceData:  new@0x1416906a0（仅 new）
SolidAlbedoMapEmissiveInstanceData: new@0x141690b50（仅 new）
PlasterInstanceData:           new@0x141690ce0  gpu_flush@0x141691b30
StoneFloorInstanceData:        new@0x141690e80  gpu_flush@0x1416921a0
SolidVertexColorWindowGlassInstanceData: new@0x141691010  gpu_flush@0x1416921a0
PlantInstanceData:             new@0x1416911a0（仅 new）
VelocityOnlyInstanceData:      new@0x141691340  gpu_flush@0x141692800
PreplacedFlowerInstanceData:   new@0x1416914e0  gpu_flush@0x141691b30
SolidVertexColorWindowInstanceData: new@0x141691680（仅 new）
GpuInstance:                   new@0x141691810  gpu_flush@0x1416934d0
CurvedHalfTimberInstanceData:  new@0x1416919a0  gpu_flush@0x1416921a0
BalconyDoorInstanceData:       new@0x1416909c0（仅 new）
ClothInstanceData:             new@0x141690830  gpu_flush@0x1416921a0
UberInstanceData:              new@0x14168fa00  gpu_flush@0x1416921a0
WrenInstanceData:              new@0x14168fd20  gpu_flush@0x141692e70
```

> 注：上述 new/gpu_flush 归属中，0x14168f870（new）/0x141691b30（gpu_flush）为 wp2 反编译证实；其余对按「new 地址簇 0x14168f870..0x1416919a0 + gpu_flush 地址簇 0x141691b30..0x1416934d0」推断（I），未逐对反编译。

## 内联/无独立符号函数

- `buffer` — **I**：`&self -> &TypedStorageBuffer<T>` 访问器（依赖表 O，无调用）
- `get_mut` — **I**：返回 `&mut [T]` 并 `dirty.add(range)`（依赖 O：`DirtyIndexRange::add` + fetch_min/fetch_max）；参数形态（全量 vs 子区间）I
- `copy` — **I**：影子内复制（src→dst）并标脏（依赖 O）
- `write` — **I**：`&[T]` 写入影子某下标并标脏（依赖 O）

## 派生胶水（0 项）

- 无；模块无 glue 过程。

## 纠错/差异（wp2 合同集成注）

- 脏区间**两端含**；`exclusive_end = end+1` 是唯一正确转换（勿建模为半开区间）。
- 消费者必须经 `get_mut`/`write`/`copy` 改影子（直改 `data` 会跳过 flush）。
- 消费面：nani instance-data 拥有者（Uber/SolidAlbedoMap/PlacementData/GpuInstance 等）每帧改写，nani 绘制路径读 SSBO 前 gpu_flush。
