# systems/ui_2d_follow_mouse.rs

> 文件作用：2D UI 标记跟随鼠标：双分支 Query（标志@+0x108），源索引 @+0x94（stride 0xa0）/@+0x10（stride 0x48）选每实体槽，写入游标 Vec2 + u32 layer（wp20 合同复核）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_2d_follow_mouse::ui_2d_follow_mouse` | 0x14092a100 | 8 | 系统:周期性处理（ui_2d_follow_mouse） | O：puffin scope（函数全名）+ 双分支写槽（wp20 + 本域复核） | ui-input\decomp\decomp-0x14092a100.txt |
