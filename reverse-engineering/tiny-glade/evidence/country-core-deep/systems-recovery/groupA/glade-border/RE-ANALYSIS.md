# groupA RE-ANALYSIS — system_glade_border（谷地边界）

dbid: tg_xa1 · 2026-10-16 · decomp: `groupA/glade-border/decomp/`

## 1. 覆盖盘点

| VA | PDB 名 | 结论 | 说明 |
|---|---|---|---|
| 0x141be0ad0 | display_glade_border::display_glade_border | O | 边界调试可视化（DebugLine 族） |
| 0x141be1080 | display_glade_border::closure$1 | O | 边界点→线的闭包（shape 数据遍历） |
| 0x140650ab0 | impl$0::build | O | GladeBorderPlugin::build（注册 display_glade_border；资源在 terrain-water 域） |
| — | feedback_input_out_of_border (line 12) | U(内联) | PDB inlinee；无独立 VA；输入越界反馈逻辑在调用点（draw_water/树放置已见 is_pos_inside 用法） |

## 2. 契约

**build (0x140650ab0)**：1 个系统注册（Update；'crates/systems/glade-border/src/lib.rs' 内部标签）+ 资源操作（0x1400dd6f0 哈希查找 ×2 区，TypeId 常量 0x1429505c0 族 16B 模式 —— 条件性资源插入，U 类型名）。

**display_glade_border (0x141be0ad0)**（7 参）：
- 输入：param_2（Res，shape 参数资源 → `ShapeParameters::as_curve2` 0x1411ff2b0 + `perimeter` 0x1411ff640 —— 与墙域 valid_enclosure ShapeParameters 共享），param_3/4（ResMut<Vec> 输出：0x140613c20 推 0x18B 条目），param_5（DebugViz 资源）。
- 流程：shape → as_curve2/perimeter → 对每段：
  - `DebugLine::new`（0x140b04e70）→ `color`（0x140b04eb0）→ `line_width`（0x140b04f40）→ `set_ui_pass`（0x140b04ee0）→ `show`（0x140b04f70）；
  - 颜色参数从 Res（param_1？）读（0x142934694 常量区邻近）；push {u64, u64} 到输出 Vec；closure$1（0x141be1080）= 点列遍历闭包（0x14089d9b0/0x14089dad0/0x14089dc00/0x14089dc60/0x14089dc90/0x14089dd50 = SignifierEntityBuilder 族）→ 同域 signifier 流。
- **语义**：谷地边界以 debug 线段可视化（墙 ShapeParameters 同一曲线数据）；SignifierFactory/UiSignifierStream 参与（越界提示的图元生成，与 draw_water 的越界 signifier 呼应）。

## 3. 关键小函数

- **0x140ad2970 GladeBorder::is_pos_inside(pos, shape)**（O，已在 terrain-water 域证据）：判定 pos 是否在谷地形状内；water/forest/树放置共用。
- **0x140ad2950**：邻近判定辅助（ui_move_tree/ui_place_tree 用"是否在可放置区"），U 精确语义（is_pos_inside 的放宽版候选，I）。
- 0x1411ff2b0/0x1411ff640/0x1411ff2a0 = ShapeParameters::as_curve2/perimeter/center（墙域 valid_enclosure 权威，已章节化）。

## 4. panic / 串

- 0x142efa180 / 0x1429cd030 / 0x142934694 邻近串区：{file,line} fat 对（display_glade_border.rs）；具体 panic 文本见 decomp（均为 bevy/signifier 内部 unwrap）。

## 5. 消费者线索

- **GladeBorder 资源本体在 terrain-water 域**（chapters/resources/glade_border.rs.md）；本 crate 只做可视化 + 判定消费。
- DrawState::update 的 DrawState 资源（0x140ae4bd0）与 draw_water 共享（树/水域）。
- 0x14362c098 全局门禁（同 water）。

## 6. U 项

- build 中资源条件插入的 TypeId→类型名。
- feedback_input_out_of_border 内联体（越界输入的震动/提示具体实现）。
- 0x140ad2950 精确语义。
