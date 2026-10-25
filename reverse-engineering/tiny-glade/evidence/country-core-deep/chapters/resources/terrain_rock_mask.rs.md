# resources/terrain_rock_mask.rs

> 文件作用：**岩体掩码：512×512 位栅格（8192 u32 字，按位打包），世界范围 ±130。GPU 回拷 TerrainRockMaskSsbo 后 CPU 查询（nani 摆放/碰撞过滤）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz` | 0x1409ea800 | 43 | (x/130+0.5) clamp[0,1]→×512 索引；(y*512|x)>>5 字索引边界检查后取位 | O | terrain-water\decomp\decomp-0x1409ea800.txt

## 实现方式（算法级）

`sample_at_world_xz`（0x1409ea800）：`u=(x/130+0.5)` clamp[0,1] → ×512 索引；`(z*512|x)>>5` 字索引边界检查后取位（512×512 位栅格 = 8192×u32，位打包）。prior 已 O，本域未推翻。
