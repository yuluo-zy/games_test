# systems/wall/adjust_wall_height/ui_adjust_wall_height.rs

> 文件作用：墙高 UI 入口：未拖动时选中预置墙到 UiWallHeight::new 开高度控件；roof flat/no-rim/动画分支；写 AdjustWallHeightCmd。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height` | 0x140b2fc00 | 15 | ui_adjust_wall_height（未拖动选中预置墙→UiWallHeight；roof flat/no-rim/动画分支） | O | ui-input\decomp\decomp-0x140b2fc00.txt, wall\decomp\decomp-0x140b2fc00.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

### ui_adjust_wall_height 0x140b2fc00（O，40KB 大函数）
- **前置**：`func_0x000140c16a00()`（状态字节，cStack_91：是否拖动/是否允许开控件）；EventReader（param_10，16B 行墙选事件）。
- **墙解析链**（三查）：1) WallEntities map（param_2，0x10 步长 {index u32, generation u32}）→ walls 数组（0x14 步长）→ roofs map（param_8：+0x28 ctrl、+0x38 mask、+0x110 计数）→ roof 记录（0x48 步长，lVar7+0x1a8 表）→ 取 roof f32（`fVar3`）；2) PublicWalls（plVar33，0x2a8 步长）命中墙：`raw 计数<2 或 raw 长度<=0` → 断言 `w.simplified_curve_2d.is_valid()`（UNK_142ada193/142aeb940）；3) `lVar31 = roof_entity(plVar33, wall_id, roofs)` → `flat_w_no_rim = lVar31!=0 && roof+0x3d!=0 && roof+0x30<=0.1`。
- **条件门**：`fVar3 != 1.0`（屋顶高度系数非满值）才继续；`value+0x296 字节==0` 且形状 tag(value+0x258)≥2 才进主分支（预置形状+默认样式）。
- **UI 打开**：`uStack_b8 = UiWallHeight::widget(&lStack_138, entity, value, 是否拖动, flat_w_no_rim)`（0x1409566d0）→ 返回 widget 状态；`widget+0x18 byte（ty）`：
  - ty≤1 且 `widget+0x38==生成实体` → 不动；
  - 否则 `UiWallHeight::signifier(&lStack_138, value, &widget, param_11+0x2c, param_9)`（0x140956cd0），按 `cVar54 = (widget+0x18>1 && widget+0x38==entity) ? widget+0x19 : 2`、`bVar25=clamp(cVar54-2, 0..3)` 分派 **4 类事件**（switch 跳表 UNK_142aeba90，每类写 16B 行 `{X, wall_id}` 到不同 cap-first Vec，扩容 FUN_140562630/140562870）。
- **另一分支**（cStack_91!=0）：扫描 .rdata 字符串区（UNK_142aeb2d4/142aeb8d8，UTF-8 续位解码找 `/`、`\` 分隔）→ 走事件流（func_0x000140c18130/c16d40 = Bevy Events next/refresh）。
- **纠错**：`w.simplified_curve_2d.is_valid()` 断言在 ui_adjust_wall_height_mode.rs:50（本文件）与 public_walls.rs 为不同实例；widget 返回值含 ty 枚举（+0x18/+0x19），是事件分派依据。
