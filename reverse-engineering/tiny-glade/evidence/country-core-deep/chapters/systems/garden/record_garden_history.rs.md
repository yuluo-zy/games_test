# systems/garden/record_garden_history.rs

> 文件作用：**花园编辑历史记录：把本帧 12B 编辑单元封装 Arc 进 History::add。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::garden::record_garden_history::record_garden_history` | 0x141695e10 | 13 | 把当前花园编辑记录（12B 编辑单元 Arc）封装成历史项 History::add | O | terrain-water\decomp\decomp-0x141695e10.txt

## 实现方式（算法级）

`record_garden_history`（0x141695e10）：把当前花园编辑记录（12B 编辑单元 Arc）封装历史项 History::add（record_raster_history 同族）。
