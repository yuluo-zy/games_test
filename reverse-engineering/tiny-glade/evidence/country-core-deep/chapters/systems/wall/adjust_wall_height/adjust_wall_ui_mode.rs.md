# systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs

> 文件作用：墙高 widget/曲线 UI：UiWallHeight{widget,signifier,new} + adjust_wall_height_curve 插值曲线（widget 层）。

函数总计 7：业务 2 / 内联无独立符号 2 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier` | 0x140956cd0 | 0 | UiWallHeight::signifier（构建 wall_height_edit_ signifier：读预览曲线点数组 +0x90/+0xe0、长度 +0x98/+0xe8） | O | ui-input\decomp\decomp-0x140956cd0.txt, wall\decomp\decomp-0x140956cd0.txt |
| `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget` | 0x1409566d0 | 0 | UiWallHeight::new/widget 族 | O | ui-input\decomp\decomp-0x1409566d0.txt, wall\decomp\decomp-0x1409566d0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::new`（行 32）— 构造
- `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve`（行 165）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项

## 实现方式（算法级）

### UiWallHeight::widget 0x1409566d0（O）
- 参数：`(state, entity?, PublicWallState value, is_dragging u8, flat_roof_w_no_rim char)`。读预览墙 `points Vec3@+0x90（12B 步长）、点计数@+0x98、heights f32@+0xe0、长度@+0xe8`。
- **构建预览曲线**：按 `points` 逐点拷贝，y 分量 = `heights[i] + offset`，offset 三选：`state+9==0` → **0.6**（_UNK_142925908）；有屋顶且 flat_w_no_rim → **-0.29**（_UNK_142a94920）；有屋顶非 flat → **0.0**。len=0 → 悬空哨兵 {cap=4,ptr=0}。
- **颜色/参数选择**：`uVar10 = shape_tag(value+0x258)<2 ? 0.7(0x142ab3c08) : 0.5(0x142ab3c0c)`。
- **输出 widget 状态**：`FUN_140d79f90`（生成 UI 实体，signifier 前缀 `edit_height_`）+ `FUN_1409231f0` 更新；状态含 `+0x18 byte（ty）、+0x19 byte、+0x38 ptr（指向生成实体）`——ui_adjust_wall_height 据此分派事件（见该章）。

### UiWallHeight::signifier 0x140956cd0（O）
- 构建 `edit_height_` 前缀 signifier 实体：`FUN_140d79f90`×2（前后两条曲线线框，UNK_142ab4820/142ab47d0 等子名前缀）+ `FUN_1409231f0`；分配 0x20B/0x28B 记录，从预览曲线（12B Vec3）拷贝，y 加 0.6 偏移（_UNK_142925908）；`_UNK_1429cd068`(0.2) 参与。

### adjust_wall_height_curve 0x140957390（O；adjust_wall_ui_mode.rs:165，章节缺文件，归此章）
- 构造 signifier 用曲线对象：入参 `(out, 曲线源{points@+8, len@+0x10}, 参数3, 参数4, 参数5)`；分配 `len×0xc`（Vec3）缓冲拷贝；构建 0x13B 魔数区（UNK_142a9948c/94/9b）；调 `FUN_14089dd50(&uStack_88, &颜色16B, param3, param4, param5)`（几何回调，常量 0x3f800000=1.0、0x101）；push 0x28B vtable 记录 `{FUN_1412dc8f0, 数据, 0x4040000000000001, 0, FUN_141564d30}`；返回 `{1, ptr, 1}`。分配失败 → `FUN_1428d8fe3` 分配错误 panic（UNK_142a98f20）。
- **纠错**：章节文件 adjust_wall_height_curve.rs.md 不存在，实际源码位于 adjust_wall_ui_mode.rs（行 165）；该函数是 widget 预览曲线的构造器，非独立系统。
