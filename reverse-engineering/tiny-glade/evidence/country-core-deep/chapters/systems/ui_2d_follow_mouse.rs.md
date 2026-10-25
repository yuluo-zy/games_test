# systems/ui_2d_follow_mouse.rs

> 文件作用：2D UI 标记跟随鼠标：双分支 Query（标志@+0x108），源索引 @+0x94（stride 0xa0）/@+0x10（stride 0x48）选每实体槽，写入游标 Vec2 + u32 layer（wp20 合同复核）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_2d_follow_mouse::ui_2d_follow_mouse` | 0x14092a100 | 8 | 系统:周期性处理（ui_2d_follow_mouse） | O：puffin scope（函数全名）+ 双分支写槽（wp20 + 本域复核） | ui-input\decomp\decomp-0x14092a100.txt |

## 实现方式（算法级）

- `ui_2d_follow_mouse` @0x14092a100（O，本域 decomp 复核）：puffin scope 名为函数全名。
  - 双分支 Query：按标志@+0x108 选源索引路径——@+0x94（实体槽 stride 0xa0）或 @+0x10（stride 0x48）；从组件表 +0x1a8/+0x48 读出目标槽基址。
  - 每实体槽写 `cursor Vec2`（8B，`*(undefined8*)(base + i*0x10)`）+ `u32 layer`（4B，`*(undefined4*)(base2 + i*4)`）。
  - 结果：2D UI 标记/图钉跟随鼠标游标并按层写入（wp20 合同一致）。
