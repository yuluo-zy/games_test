# systems/terrain/terrain_hover_idx.rs

> 文件作用：**地形笔画 hover：AppMode 位掩码 0x421 门 → MouseTerrainRaycastSsbo 读笔画 id（+0x10，-1 哨兵）→ 水位/光标/距离 0.5 门控 → 写 HoveredTerrainStrokeId。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::terrain::terrain_hover_idx::terrain_hover_idx` | 0x140aff720 | 17 | AppMode 0x421 位掩码门；SSBO（名 0x6c 字符）映射读 id@+0x10；水位采样≤0/光标无效/距离≤0.5 不变；否则写 HoveredTerrainStrokeId{tag=1,id} | O | terrain-water\decomp\decomp-0x140aff720.txt

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
