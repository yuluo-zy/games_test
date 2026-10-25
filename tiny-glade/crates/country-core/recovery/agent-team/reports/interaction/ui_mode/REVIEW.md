# REVIEW — systems/ui_mode(卡E-slice1 终验 → 闭合·widget_params+slider3d+AppMode 对齐)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md(reports/interaction/ui_mode/);`cargo check --lib` exit 0
(独立复验;协调器所述"全 lib 6 error"系 startup 组在途暂态,现状态已消)。

## 核对

### ① WidgetParams / UiFocus / clearing_canvas — PASS
- `WidgetParams` 0xb0 B:默认 u32 组 `{0,8,0,1,0,2,3,3}` @{0,8,0x10,0x18,0x20,0x30,0x48,
  0x60}、UiSounds 4×24B@0x18..0x78、WidgetVolumeShape 32B{tag@0,layer@1,size@0x10}、
  u16@0xa8=0x101 + 整字节 occlude_by_world、flags@0xa9 bit0(no_history_recording)/
  bit2(allow_negative_distance)——与 RESTORE/章节一致 ✓(引用地址 0x140985f30/0x1409a9130
  等)。
- `UiFocus` u32 {None=0, Toolbar=1, Mei=2, World=3} + is_none/is_toolbar/is_mei/as_mei ✓。
- `clearing_canvas` = {tag 4, layer, 130×130}(双 130.0 = 0x43020000)✓。

### ② Slider3dState + project_cursor_onto_line — PASS(结构)
`project_cursor_onto_line`(0x140a9bd40):`axis=end−start`、`t=clamp(dot(cursor−start,axis)/len²,0,1)`、
最近点 `start+axis·t`;退化 len²≤EPSILON → (0, start) ✓。O 结构(字段语义 I)。

### ③ AppMode 接线 — **PASS(机器码验证)**
- native_tag:Editing(None)=0 ✓、Terrain=10 ✓、Replay=13(0xd)✓(与地形 0x421 门
  bits{0,5,10}、ui_draw mode==10 一致)。
- **is_tool_selected 与 0x3cfe 掩码一致(fresh 0x140a86b40 机器码验证)**:
  ```
  mov ecx,[rcx]; lea eax,[rcx-6]; cmp eax,0xf; setb dl   ; (tag-6)<0xf → tag∈[6,20]
  add cl,0xfb; mov eax,0x3cfe; shr eax,cl; and al,dl     ; result=bit(tag-5) of 0x3cfe
  ```
  0x3cfe=0b0011_1100_1111_1110,bit 布局 [0]=0,[1..7]=1,[8..9]=0,[10..13]=1,[14..15]=0 →
  对 tag∈[6,20] 为真集 = {6..12, 15..18}(Wall..Color + Water/Tree/Garden/Fences)。
  **crate `is_tool_selected` {6..12,15..18} 与机器码完全一致** ✓。
- 早期裁决(4=Water/5=Garden/6=Tree)与 crate 15/16/17 差异:机器码证实工具判别用
  15/16/17,早期列表为显示层序非工具判别——**判定成立** ✓。

### ④ 待取证登记诚实 — PASS
ui_update/get_ui_volume/hover_intersection、signifiers 族、brush_preview、mode_manager
模式转换/工具栏(check_if_can_switch 闭包族)——全列下子卡,无冒充。

## 结论与 DISPATCH 建议

- 卡E-slice1 复验通过:**widget_params + slider3d + AppMode 对齐闭合**。
- 建议:slice1 → 闭合;slice2(ui 顶层系统/signifiers/brush_preview/mode_manager 转换)
  续派。is_tool_selected 机器码验证纳入证据。
