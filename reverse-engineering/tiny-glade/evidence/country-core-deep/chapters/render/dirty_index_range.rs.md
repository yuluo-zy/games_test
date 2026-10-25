# render/dirty_index_range.rs

> 文件作用：**脏区间原语**——`DirtyIndexRange`（两个 AtomicUsize 端点对，`usize::MAX` 空哨兵），
> 被 `ShadowedBuffer`（72B 首字段）使用。见 shadowed_buffer.rs 章布局表。

## 内联/无独立符号函数

- `country_core::render::dirty_index_range::DirtyIndexRange::add`（行 13）— **O**：`fetch_min(start)` + `fetch_max(end)`（两端点各自原子收缩/扩张）
- `country_core::render::dirty_index_range::DirtyIndexRange::is_empty`（行 18）— **O**：`start > end`（空哨兵判定；两端含）
- `country_core::render::dirty_index_range::DirtyIndexRange::clear`（行 22）— **O**：写回 `(usize::MAX, 0)`（清空哨兵）
- `country_core::render::dirty_index_range::DirtyIndexRange::to_range`（行 26）— **I**：转 Range（半开转换 `start..end+1` 由调用方/`ShadowedBuffer::gpu_flush` 的 `exclusive_end = end+1` 承担）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- **算法**：`add(range)` = 两 AtomicUsize 无锁更新（`fetch_min`/`fetch_max`，并发安全）；`is_empty` = `start > end`（`usize::MAX` 哨兵下 `add` 首调 = `fetch_min(MAX, s)=s` + `fetch_max(0, e)=e`，天然初始化）；`clear` = 写回哨兵对。
- **消费链**：`ShadowedBuffer::get_mut/write/copy` 标脏 → `gpu_flush` 读区间（**两端含**，`exclusive_end = end+1` 是唯一正确转换）→ 守卫 `byte_size >= exclusive_end*es` → `update_buffer_bytes` → 清脏。nani 实例数据每帧改写→flush（shadowed_buffer 章 §3）。
- **并发语义**：两端点独立原子 → 并发 add 可能产生超集区间（宽覆盖），但绝不小（不影响正确性，只多传）。

## 派生胶水（1 项：fmt 等）
