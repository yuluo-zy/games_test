# systems/wall/shapes/rectangle/edit_rectangle.rs

> 文件作用：矩形编辑命令：EditRectangleCmd 到 update_wall_state_curve + update_wall_state_enclosure。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle` | 0x14097f620 | 0 | edit_rectangle（update curve + enclosure） | O | wall\decomp\decomp-0x14097f620.txt |

## 实现方式（算法级）

**edit_rectangle（O，0x14097f620）**：5 参系统（param_2/3/4 带 +0x1c 回写 = ResMut）。遍历 `EventReader<EditRectangleCmd>`（**0x40B/行**，矩形参数 @+0x10）。处理链：

1. **变更检测**：事件游标差（uVar3 与 reader +0x18/+0x38 位置）钳 0 后判 `uVar3−uVar13 != 0 || uVar4−uVar9 != 0`；无变化 → 早退。
2. `FUN_140c98cf0(&uStack_c0, lVar17+0x10)`（**Rectangle2d::as_curve2**，新参数）→ `FUN_140aaa090(wall, &curve)`（**update_wall_state_curve**）。
3. **enclosure**：从事件行组 16B 记录 `{tag u32@+0=1, center qword@+4, dims qword@+0xc}`（+ 事件其他字段）→ `FUN_140aa9610(wall, &record)`（**update_wall_state_enclosure**）。
4. **历史行**：push 0x20B `{u64, WallId, params qword, u32, u16=4}`（= EditRectangleCmd 快照）。
5. **屋顶高度事件**：`fVar18 = clamp(func_0x000140c904a0(0, rate_常量, *(f32*)(event+0x34)/*(f32*)(*param_5+0x58)), 0, 1)`；按 `*(u8*)(event+0x38)==0` 选率常量 **0x142a13848 或 0x142a7fa30**；push 0x28B 事件行 `{u64, WallId, params, u32, eased_f32, flag(0/1)}`。

**数据结构**：EditRectangleCmd 0x40B（矩形参数 @+0x10 = ShapeParameters payload 布局）。
**跨模块**：`FUN_140c98cf0` = utils/geometry 矩形 as_curve2；`FUN_140aaa090/140aa9610` = inner_wall_state _try_update_state 实例。
**常量**：率常量 _UNK_142a13848/_UNK_142a7fa30（U 位型）、1.0 @0x142925904。
