# systems/ui_edit_focus.rs

> 文件作用：编辑焦点系统：按模式 id 分发（mode<6 注册 'edit_focus_canvas' widget（tag5/f32::MAX）；mode 1/2 跟随路径过 FUN_140a1c930 谓词；0x2a8 反向步长遍历 wall 数组并断言 simplified_curve_2d 有效性；mode==10 特判；4 张旋转跳表 = 24 变体 GeoSource 匹配；结尾推 {tag0, 计数} 0x18B 事件）。geo_src_to_edit_subject(_w_deco_clutter) 做 GeoSource→EditSubject 映射。

函数总计 5：业务 3 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_edit_focus::ui_edit_focus` | 0x1409771f0 | 0 | 系统:周期性处理（ui_edit_focus） | O：完整结构（DIGEST §2.5）；两条 panic 串：'assertion failed: w.simplified_curve_2d.is_valid()'（0x142ada193）+'internal error: entered unreachable code: ui_edit_focus shouldn't include '（0x142ab6a70） | ui-input\decomp\decomp-0x1409771f0.txt |
| `country_core::systems::ui_edit_focus::geo_src_to_edit_subject` | 0x140977f50 | 312 | GeoSource→EditSubject 映射 | O：172KB（含内联）；EditSubject 变体 serde 名全套（WallId/wall_id/Floor/Roof/flat/gable/StairGraphEdge/DecoratorCollision/DecoratorInteraction/StairInteraction/ColumnStairs/Fence/Arches/Tree/Terrain/AutoClutter/entity 等，串 @0x142ab7268 起） | ui-input\decomp\decomp-0x140977f50.txt |
| `country_core::systems::ui_edit_focus::geo_src_to_edit_subject_w_deco_clutter` | 0x140978000 | 317 | GeoSource→EditSubject（含 deco/clutter） | O：_w_deco_clutter 变体额外考虑 deco-clutter 选择（同 0x140977f50 族，172KB 反编译） | ui-input\decomp\decomp-0x140977f50.txt |

## 实现方式（算法级）（O，decomp-0x1409771f0 + 跳表字节解码）

**ui_edit_focus @0x1409771f0（系统主流程）**：
1. `mode = *param_1`（GeoSource 判别 u32）。**mode > 5** → 写 `bool = (mode == 10)`；
   - `*param_3 + 0x10 == 0`（某 GeoSource 列表长度 0）→ 跳表 C（UNK_142ab738c），
     idx = `*(*param_4 + 0x90)`；
   - `mode == 10` → 跳表 A（UNK_142ab73a0），idx = `*(*param_4 + 8)`；
   - `6..=9` → 跳表 B（UNK_142ab73b4）；
   - `mode < 6`（widget 注册后）→ 跳表 D（UNK_142ab73c8），idx = `*(*param_4 + 8)`。
2. **mode < 6**：注册 widget **"edit_focus_canvas"**（0x142ab69e0，0x11=17 字符）：
   WidgetParams 栈记录 `{tag=5, f32 0x7f7fffff(MAX), u16 0x100...}` → FUN_1409236d0（UiState 添加）。
3. **mode ∈ {1,2}**（`(6>>mode)&1` 命中）→ 「跟随」路径：`FUN_140a1c930` 谓词（I：History/相机
   状态检查，输入 = Res 状态 + puVar3[1]）；通过才继续。
4. **未命中（mode 0 或 ≥3 且 0x18 位测失败）** → 遍历 wall 实体：
   SIMD 16B lane 匹配（每 16B 槽 4 字节×4 比较 → 16 位位图 → `while(bit!=0)` 取最低位 →
   `(idx + base) * -0x2a8` 反向步长定位实体——元素基址在指针前方，反向存储的 wall 数组）；
   命中后检查 `字段 < 2 || f32 <= 0` → 先 panic `"assertion failed: w.simplified_curve_2d.is_valid()"`
   （0x142ada193，public_walls.rs line 0x32），再 panic
   `"internal error: entered unreachable code: ui_edit_focus shouldn't include ..."`
   （0x142ab6a70，格式化 panic 0x142ab6a90）；实体字节==-1 终止扫描。
5. **结尾**：push 0x18B 记录 `{u32@0 = 0(标签), u64@0x10 = 计数}`（I：UiHistoryPlaySelectAnimation/
   ToolSelectionCancelled 类事件，事件队列 @+0x20..0x30 + 计数@+0x40）。

**4 张分发表（O 字节解码，基址 0x142ab738c）**：每项 4B int32 偏移，高 2B 恒 0xfdec →
目标 = `0x14097738c + low16`，全部落在函数体 0x14097745b..0x140977e34：
`idx→target`：0→776, 1→45b, 2→766, 3→45b, 4→45b, 5→6c4, 6→98e, 7→6b7, 8→98e, 9→98e,
10→6ca, 11→91a, 12→87c, 13→91a, 14→91a, 15→6d1, 16→4fa, 17→6c4, 18→4fa, 19→4fa, 20→7aa,
21→7aa, 22→d9c, 23→e34。**4 表 = 同一张 24 项表首部旋转 0/5/10/15**（A=0x142ab73a0 自 entry5 起；
B=0x142ab73b4 自 entry10 起；D=0x142ab73c8 自 entry15 起）——GeoSource 24 变体 match 的 4 个
编译入口共用。

**geo_src_to_edit_subject @0x140977f50 / _w_deco_clutter @0x140978000（O，172KB 含 serde 内联）**：
GeoSource→EditSubject 映射；嵌套跳表族 `0x142ab747c（首字节分派）/0x142ab7504/0x142ab7514/
0x142ab7524/0x142ab7568` = serde deserialize 的 variant match；变体/字段名 24 token 串
（0x142ab7268 区）：
`DecoratorCollision / decorator_id / addr / DecoratorInteraction / StairInteraction / ColumnStairs /
Fence / Arches / Tree / Terrain / AutoClutter / entity / WallId / wall_id / Floor / Roof / flat /
gable / StairGraphEdge / graph_edge / StairSupport / SupportedStairStructural / point_idx /
UnsupportedStairStructural` → 17 变体（DecoratorInteraction{decorator_id,addr}、WallId{wall_id}、
Floor{flat}、Roof{gable}、StairGraphEdge{graph_edge}、SupportedStairStructural{point_idx} 带字段）。
邻区 serde crate 变体表：StairGraphEdge@0x140201698、DecoratorId@0x140385340、WallId@0x140b30c70、
Roof@0x140671510、DecoratorCollision@0x140541098。
panic 锚：`0x140ab6841` 区为邻近函数的 unreachable 串（compute_passes/mouse_read_terrain_stroke_index
与 ui_clutter/rbm_autoclutter）；`"internal error: entered unreachable code"` 表
（0x142ab69e8 区，文件 ui_move_freehand_walls.rs 引用）。

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
