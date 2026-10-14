# systems/wall/shapes/rectangle/finish_edit_rectangle.rs

> 文件作用：矩形编辑收尾：UiState80 快照先发；维度 state1/朝向 state2；改变判据 abs(acos(prev·cur))*57.29578>0.01 或维度差>0.01；记 ConcurrentEdits + niche8000000000000004；无变化到 DecoratorArchivist 恢复（192 全原调用验证）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle` | 0x141464a60 | 22 | finish_edit_rectangle（0.01 阈值双判据 + ConcurrentEdits + 恢复） | O | wall\decomp\decomp-0x141464a60.txt |
