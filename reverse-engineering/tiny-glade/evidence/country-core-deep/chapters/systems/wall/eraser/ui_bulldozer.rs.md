# systems/wall/eraser/ui_bulldozer.rs

> 文件作用：推土机 UI 入口：wall_eraser_bulldoze_condition = AppMode{tag==9, +4==3}；handle_decorators 排空墙属装饰 + terrain/stair cull；can_reconnect_stairs_if_midpoint_deleted；StairsState::disconnect_edge；命中到 AnimateWallBulldozeCmd 入 buffer（RoofDelete FIRST + WallCompound StateChange+Delete 双组）。

函数总计 14：业务 3 / 内联无独立符号 2 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze` | 0x1409de190 | 0 | wall_eraser_bulldoze（装饰排空 + 楼梯断开 + RoofDelete FIRST 双组） | O | ui-input\decomp\decomp-0x1409de190.txt, wall\decomp\decomp-0x1409de190.txt |
| `country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze_condition` | 0x1409de170 | 49 | wall_eraser_bulldoze_condition（AppMode{tag==9,+4==3}） | O | ui-input\decomp\decomp-0x1409de170.txt, wall\decomp\decomp-0x1409de170.txt |
| `country_core::systems::wall::eraser::ui_bulldozer::hammer_cursor_icon` | 0x1409e35a0 | 933 | UI 交互 | I | ui-input\decomp\decomp-0x1409e35a0.txt, wall\decomp\decomp-0x1409e35a0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::eraser::ui_bulldozer::can_reconnect_stairs_if_midpoint_deleted`（行 817）— 谓词/校验
- `country_core::systems::wall::eraser::ui_bulldozer::handle_decorators`（行 836）— 系统:事件响应

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 8 项

## 实现方式（算法级）

### wall_eraser_bulldoze_condition 0x1409de170（O，机器码）
- `48 8b 01 83 38 09 0f 94 c1 80 78 04 03 0f 94 c0 20 c8 c3` = `(*app).tag==9 && (*app).byte@+4==3`（AppMode 判别 u32@+0 + 子判别@+4）。✓ DIGEST。

### wall_eraser_bulldoze 0x1409de190（O）
- **光线拾取**：`FUN_1408ce180(&hit_type, 光标资源, &ray, 0)` → 按 `hit_type` 分派：
  - `0x11`（树/装饰）→ `FUN_140d79f90`（spawn `cursor_hammer_` 前缀 signifier，UNK_142ac4080）+ `FUN_14146c880`（写事件，装饰删除，`delete_tree_` 语义）。
  - 墙型（0,1,2,6,7,8,9）→ 扫描 **AnimateWallBulldozeBuffer**（0x98 步长匹配 `+0x70` WallId）防重复。
  - roof/stair（4,0xd,0xe,3,0xa..0xc）→ 解析 roof 实体（walls 数组 0x14 步长 → roof map 0x48 步长，同 roof_entity 链），3D 距离 `fVar20=SQRT((p-hit)²)`（初值 f32 max 0x7f7fffff）。
- 命中判定：`hit 存在 && distance<=best && 非已动画中` → `uStack_100=2`（推土目标）；`FUN_1409236d0(资源, &"bulldoze_canvas", 0xf, ...)` 注册画布 signifier。
- **尾跳表**：`(*(&UNK_142ac4544 + *(int*)(&UNK_142ac4544 + uStack_100*4)))()` —— 按 uStack_100（0..5）分派动作（RoofDelete FIRST / WallCompound StateChange+Delete / 装饰排空 / 楼梯断开，I：具体 handler 在跳表目标）。
- **panic**：实体索引越界 → fmt panic（UNK_142ac2ed0/142ac2eb0，line 0x2b）+ 多处 `w.simplified_curve_2d.is_valid()` 断言（UNK_142ac4100/142ac3c60/142ac3ca8 三个文件指针=ui_bulldozer.rs）。

### hammer_cursor_icon 0x1409e35a0（I）
- 锤子光标图标系统：UI 交互层，4.4KB，未逐行。
