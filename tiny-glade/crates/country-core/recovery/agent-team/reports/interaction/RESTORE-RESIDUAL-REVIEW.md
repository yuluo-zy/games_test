# REVIEW — interaction 库内残余闭合(restorer-interaction2 5 项核对点)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer-interaction2 RESTORE-RESIDUAL.md;`cargo check --lib` exit 0(独立复验,
8 警告含 K/O 常量 dead-code 与 time_of_day HOUR_TO_MS——O 表完整保留的良性冗余)、
`cargo test` 1725/1(唯一败 geometry/sphere 他组既有)。

## 核对

### ① translate_input 动作↔槽映射全解 — PASS
- `ANALOG_TARGETS` 10 槽:0..3 → analog[2] camera_pan(±dt×30)、4..5 → analog[3]
  camera_height、6..7 → analog[5] camera_rotate(±1800×原始 delta,_UNK_142ce2d34/2d30)、
  8..9 → analog[4] camera_zoom(±1.5×dt×30,_UNK_1429258d0/1429df180)——与反编译逐条对应 ✓。
- `DIGITAL_TARGETS` 29 槽:10..36 → 26 动作、37/38 鼠标{1,2} → camera_rotate_mode/
  camera_pan_mode、39 鼠标{1} → focus_edit ✓。
- 滚轮 `MouseWheel` → analog[4] camera_zoom(权重 tag0=8.0/其余 1.0),**已纠旧实现误累加
  analog[0]** → 现 analog[0]/[1] 由 translate_input_to_cursor_moved 写 ✓。
- 右键手势 `Local<MouseMoved>` 累加游标增量长度,`right_click=(MouseMoved<3.0)&右键释放|槽23`
  (_UNK_1429cd078=3.0)✓。
- 硬编码键(ui_cancel 0x73/screenshot 槽25&!0x3d/demo 门 8 键)登记 ✓。

### ② suncalc_pos_to_vec 常量 — PASS
RNE 直读 0x142ad58e0..5990(f64 π/π/2 + f32 矩阵 A..P:−0.762443/0.647056/0.581319/
0.418681/−0.493343/0.551646/0.834078/0.806015/0.591895);实现按 decomp 行 348..424
两级旋转(colatitude=π/2−(azimuth+π)、f32 输入、矩阵 N/M/P 方程)逐项对应机器码 ✓;
分量→语义(x=东/y=高/z=北)与 uVar63 次级输出(−0.15)登记 I/待接 ✓。

### ③ ui_draw_terrain 三道门 — PASS
高度采样≤0 → 不注册画布;`tag&1==0`(未悬停)→ 不注册;`distance>0.5`(_UNK_142925870)→
不注册。通过后推 `TerrainCanvasRegister`(clearing_canvas_w_layer_and_sound tag 0x11)+
`TerrainOutOfBorder` 界外事件;CursorTerrainRaycast 补 distance()(+8 槽)✓。

### ④ seek 子字段类型化 — PASS
`InputTranslationState` 访问器:toolbar_current()@+0x18 字节、toolbar_vec_cap_ptr()@+0x20/
+0x28(cap-first Vec 头)、camera_params()@+0x30..+0x58(40 B,load_parameters 源)——偏移 O ✓;
seek_to_sync_point 设置 index+回写 buffer+子字段恢复(toolbar clone 与 camera 依赖宿主分配,
登记待接)。`InputReplayInitialState` 10 字段(window_size/cursor_depth/cursor_pos_ws/
center_pos_ws/ui_settings/sheep_positions/sheep_rotations/app_mode/toolbar/camera)字段名/
序 O,ui_settings/sheep 载荷 U(透传,诚实)✓。

### ⑤ signifier_animation_t + despawn_marked — PASS
`signifier_animation_t`(0x140a69340):Query 遍历 SignifierAnimation,guard duration!=0&&!nan,
`t'=clamp(t+tick/duration,0,1)` ✓;`signifier_mark_for_delete`(0x140929a90)+`despawn_marked`
(Query despawn)O ✓;scale/alpha switch 0..0x10 与 config_hash→widget 宿主匹配待接。
focus_mode 167KB switch 表(U,Ghidra UNRECOVERED_JUMPTABLE)、camera body 更新 20+、Steam
全链——诚实登记待续 ✓。

## 结论与 DISPATCH 建议

- 5 项核对点全部通过:**interaction 库内残余闭合**。
- 建议:残余 → 闭合;诚实登记项(focus_mode switch 表/camera body 更新/Steam 全链/
  spawn_brush_previews/ui 顶层系统/signifier config_hash 匹配)归后续卡/re-analyst。
- 编译 exit 0 + cargo test 1725/1(与审计基线 1709/1 一致,仅 geometry/sphere 他组既有败)。
