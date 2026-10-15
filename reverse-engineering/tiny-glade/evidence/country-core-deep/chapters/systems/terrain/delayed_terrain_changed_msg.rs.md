# systems/terrain/delayed_terrain_changed_msg.rs

> 文件作用：**延迟地形变更：ScheduledTerrainReadback {counter AtomicUsize@0, flag bool@8, delays Vec<u32>}；每帧递减 delays→0 触发 GPU 高度回拷 copy_from_gpu×2（权威+视觉），延迟 2 帧发 OnTerrainChange。**

函数总计 6：业务 2 / 内联无独立符号 1 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::on_terrain_modified` | 0x140aabdc0 | 39 | 修改计数原子自增（LOCK inc [rcx]; ret） | O | terrain-water/DIGEST.md
| `country_core::systems::terrain::delayed_terrain_changed_msg::delayed_terrain_changed_msg` | 0x140aabdd0 | 66 | delays 逐项递减→0：FrozenMap 取 SSBO（尺寸断言 0x3a/0x40）→ cpu_readback → copy_from_gpu×2；flag 置位 push(2)（延迟 2 帧）；压实完成项 | O | terrain-water\decomp\decomp-0x140aabdd0.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::on_gpu_copy_issued`（行 44）— GPU 拷贝已发出（read_terrain_data 内联）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 3 项

## 实现方式（算法级）

`ScheduledTerrainReadback {counter AtomicUsize@0, flag bool@8, delays Vec<u32>}`：
- `on_terrain_modified`（0xaabdc0）= `LOCK inc [rcx]; ret`（修改计数原子自增）。
- `delayed_terrain_changed_msg`（0xaabdd0）：delays 逐项递减 → 0：FrozenMap 取 SSBO（尺寸断言 0x3a/0x40）→ cpu_readback → `copy_from_gpu×2`（权威+视觉）→ 压实完成项；flag 置位 push(2)（延迟 2 帧）发 OnTerrainChange。
