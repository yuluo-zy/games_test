# nani draw 扩展契约 — draw_ext.rs（1 proc）

标注：**O**=反编译直接支持；**I**=推断；**U**=未恢复。

## 1. `NaniDrawExt::nani_draw`（`impl$0::nani_draw`，0x140904ae0，行 19；1185B）

DrawBuilder 扩展 trait 方法，把 nani GPU-driven subset 的间接绘制接到 pass 声明层。

**签名（I）**：`fn nani_draw(self, kind: GpuWorldSubsetKind) -> DrawBuilder`（builder 模式，按值消费 0xf8B 并回写）。

**业务逻辑（O）**：
1. 经 `Components::get_resource_id()`（hash `0x662978458699a5ef` → GpuWorld；缺 → `unwrap_failed`）取 GpuWorld 资源。
2. 锁 kind 对应 subset 槽（`+0xc0*kind` 偏移，`RwLock`；CAS `+0x10` 计数）。
3. 读 subset 内 `Vec<InstanceKey>`（+0x58/+0x60，16B/元素）。
4. `AssetSsboLibrary::get_rg_buffer<NaniDrawListLengthSsbo>()` + `get_rg_buffer<NaniDrawListSsbo>()` + `read_storage_buffer<NaniInstanceIndirectionSsbo>()`（WP1 `DrawBuilder::read_storage_buffer`）→ 绑定三个 nani 间接 SSBO。
5. `DrawBuilder::draw_indexed_indirect_raw()` 或 `draw_indexed_indirect_count()`（二选一，取决于 kind 与开关）：
   - `NANI_FORCE_FINE_GRAINED_SORTING == 0` 且 `(kind >= 2 || NANI_FORCE_NO_FINE_GRAINED_SORTING != 0)` → `draw_indexed_indirect_raw`；
   - 否则 → `draw_indexed_indirect_count`（带 per-list draw-count 间接）。
6. 释放锁（deadlock_impl release + RawRwLock::unlock_shared_slow 条件路径）。

`bVar3 = DrawBuilder+0xf0`（"fine grained" 标志字节）参与分派。

## 2. 消费者（O）

- `gpu_generate_draw_lists`（0x140987dd0）与帧 pass（main_pass 等，见 render-passes INDEX.md §2）用 `nani_draw(kind)` 把各 subset 的绘制声明进 render graph。
- `ComputePass`/`DrawBuilder` 侧入口见 pass-declaration.md §1.2/§1.3（`multi_draw_indexed_indirect` 0x140907990 等）。

## 3. Unknown

- `DrawBuilder+0xf0` 标志的写入方 U（推断为 `NANI_FORCE_FINE_GRAINED_SORTING` 状态注入）。
- 间接缓冲的 GPU 布局/计数语义（NaniDrawListSsbo 元素 = `VkDrawIndexedIndirectCommand` 20B，I，来自 pass-declaration §1.2 步长 0x14）。
- `nani_draw` 的 trait 全名与其它方法（若有）U。
