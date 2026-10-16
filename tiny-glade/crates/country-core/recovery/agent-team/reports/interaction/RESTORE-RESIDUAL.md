# RESTORE-RESIDUAL — interaction 组库内残余缺口专项（restorer-interaction2）

实现者: restorer-interaction2 | 日期: 2026-10-16
范围: 完整性审计 `reports/interaction/completeness-audit.md` 分类的**库内可闭合残余缺口**，
按「业务逻辑分析→整理→按 PDB 骨架实现 Rust」工作流。只改 interaction 域文件。

## 已闭合缺口

### 波 1 — 输入（systems/backend/input）

**① translate_input 动作↔槽映射（O 全解）— `translate_input.rs` 重写**
- 证据: 反编译 `ui-input/decomp/decomp-0x1417cd0a0.txt`（逐槽读序）+ ron serialize 字段名表
  @0x140831740（`utils-input-config/DIGEST.md` §2.3）+ RNE 常量直读。
- **模拟槽**（`ANALOG_TARGETS` 表）: 绑定槽 0..3 move_forward/back/left/right →
  `analog[2] camera_pan`（±delta×30）；槽 4..5 move_up/down → `analog[3] camera_height`；
  槽 6..7 rotate_left/right → `analog[5] camera_rotate`（±1800×**原始** delta，
  `_UNK_142ce2d34`=−1800、`_UNK_142ce2d30`=1800）；槽 8..9 zoom_in/out →
  `analog[4] camera_zoom`（±1.5×delta×30，`_UNK_1429258d0`=1.5、`_UNK_1429df180`=−1.5）。
- **数字槽**（`DIGITAL_TARGETS` 表）: 槽 10..36 → 26 动作（building→tool_building …
  ui_tab_next→ui_tab_next、36→cinematic_camera_ctrls）；槽 37/38 鼠标 {1,2} →
  camera_rotate_mode/camera_pan_mode；槽 39 鼠标 {1} → focus_edit（右键手势）。
- **滚轮**（O 行 282..336）: `MouseWheel` 事件 → `analog[4] camera_zoom`（权重 tag0=8.0/其余 1.0）。
- **右键手势**（O 行 438..471）: `Local<MouseMoved>` 累加游标增量长度，`right_click =
  (MouseMoved < 3.0) & 右键释放 | 槽 23 focus_edit 绑定`。
- **硬编码键**（O）: ui_cancel=0x73（恒）、screenshot=槽 25 & !0x3d、demo 门（0x143634cd8）
  内 8 键（0x35/0x40/0xab/0x25/0x23/0x29/0xa6 + 0xa7 恒启）。
- 默认键序列按 O 原字节修正（36 u32 键码 + `[0x1b0]=0xc3` + 鼠标 `{1,2,1}`）。
- 纠错: 旧实现把 InputCursorDelta 累加进 analog[0]（错误）——O 中滚轮进 analog[4]、
  InputCursorDelta 仅喂右键阈值；analog[0]/[1] 由 translate_input_to_cursor_moved 写。

**② input_record_replay 帧 serde 形态 — `InputReplayInitialState` 10 字段类型化**
- 证据: `input/ALGORITHMS.md` §2.9（键表 0x142c5b422..46d，长 0xb/0xc/0xd/0xd/0xb/0xf/0xf）。
- `RecordingFile.replay_initial_state` 从 `serde_json::Value` 透传升级为
  [`InputReplayInitialState`]（window_size/cursor_depth/cursor_pos_ws/center_pos_ws/
  ui_settings/sheep_positions/sheep_rotations/app_mode/toolbar/camera）。字段名/序 O；
  ui_settings/sheep 载荷 U（透传，诚实登记）。`InputFrameSerde` 4 键序既有 ✓。

**③ 同步点 seek 子字段类型化 — `InputTranslationState` 访问器 + seek 完善**
- 证据: `control_input_replay` @0x140941a30 seek 体（sync+0x18 current、+0x20/+0x28 工具栏
  Vec、+0x30..+0x58 camera、+0x58 frame_index）。
- `InputTranslationState` 新增 `toolbar_current()`/`toolbar_vec_cap_ptr()`/`camera_params()`
  （O 偏移已定）；`seek_to_sync_point` 设置 index + 回写 buffer + 子字段恢复（toolbar clone
  与 camera load_parameters 依赖宿主持有分配，登记待接）。

### 波 2 — 地形（systems/terrain）

**④ ui_draw_terrain 消费面（O 全解）— `ui_draw_terrain.rs` 重写**
- 证据: `terrain-water/decomp/decomp-0x140a9fdc0.txt`（168 行）+ RE-ANALYSIS2 §3。
- 三道门: **高度 ≤ 0 → 不注册画布**（`bilinear_sample_at_world_xz`，本实现用
  `TerrainHeightsData::sample_at_world_xz`）；`tag&1 == 0`（未悬停）→ 不注册；
  `distance > 0.5` → 不注册。
- 画布注册事件 `TerrainCanvasRegister`（clearing_canvas_w_layer_and_sound，tag 0x11）；
  界外反馈 `TerrainOutOfBorder`；`TerrainDrawState` Local 记录积累。
- `CursorTerrainRaycast` 补 `distance()` 访问器（O 布局 +8 槽）。

**⑤ ui_edit_terrain ABI 对齐（RE-EDITCMD E7 吸收）**
- Adjustment 从 `Absolute` 改 `Delta`（O 生产者判别 2=Delta）；原字节序 `{falloff,height,
  position,id}`（id@+0x1c）+ 紧凑 value 槽差异在注释登记（crate 内走 tg_terrain 共享接口）。

**⑥ ui_terrain_focus_mode 常量表 + 行为（O）**
- 加入 O 常量（0.01 阈值/`−10` 速度/`1.2`/`7.0`/`2.5` 缩放/`f32::MAX` 未命中/`0.5` 命中门）。
- 167 KB 内联 switch 转移表 U（卡点如实标注，Ghidra UNRECOVERED_JUMPTABLE）。

### 波 3 — 相机/时间

**⑦ suncalc_pos_to_vec 正交基常量直读 — `time_of_day.rs`**
- **RNE 直读 0x142ad58e0..5990**（本专项落盘证据）: f64 `π`@0x58f8、`π/2`@0x5900；
  f32 矩阵元 A..P @0x590c..5984（−0.762443/0.647056/0.581319/0.418681/−0.493343/0.551646×3/
  0.834078×3/0.806015×2/0.591895）；`_UNK_142a81460`={0.0,−0.0}。
- 实现按反编译 `decomp-0x140a73460.txt` 行 348..424 两级旋转（正交基矩阵 → N/M/P →
  高度双角四元数）逐项对应机器码（O）；分量→语义（x=东/y=高/z=北）与 get_position 输出序
  登记 I；uVar63 次级输出（含 −0.15）登记待接。

### 波 4 — UI signifiers

**⑧ signifier_animation + mark_for_delete/despawn（库内组件/系统落地）**
- `animation.rs`: `SignifierAnimation` 组件（O 布局 duration@+0x10/t@+0x14/eased@+0x18/
  scale/alpha@+0x1c/+0x20/+0x24/tick@+0x28/kind@+0x98）+ `signifier_animation_t`
  （`t' = clamp(t + tick/duration, 0, 1.0)`，O）+ `signifier_animation`（kind 分发 U，
  eased 线性缓动 I）。
- `signifier_mark_for_delete.rs`: `SignifierMarkedForDeletion` 组件 + `despawn_marked`
  （Query 遍历 despawn，O）+ `signfier_mark_for_delete`（config_hash→widget 匹配登记待接）。

## 登记待取证/待接（诚实分层）

- **focus_mode switch 转移表**（167 KB 内联，U）：进入/退出状态转移精确表 Ghidra 未恢复，
  ReVa 定点可后续补证。
- **camera body 更新算法**（butterfly/free/normal/walk/camera_body，20+ 函数）：审计已标
  「大件后置」missing_in_library；`camera-time/ALGORITHMS.md` §2.3..2.6 有 O 级算法但依赖
  dolly CameraRig/driver 机制与 body 内 quat 来源（normal_body.rs 已闭 zoom/rotate/constrain，
  update/translate_screen_space 待续）。
- **spawn_brush_previews / path_brush_preview_size / ui 顶层系统**（ui_update/get_ui_volume/
  hover_intersection/debug_visualize_volumes）：实体机制/宿主 UI 画布外部（ui_mode RESTORE
  登记）。
- **signifier scale/alpha switch 0..0x10** 与 **config_hash→widget** 宿主匹配待接。
- **Steam 全链**：`systems::steam` 库外登记（`SteamInputActive` 门已留）。
- 编译注: 他组在途 `PlayWallUiSound` 曾暂态 error（audio/eco），现消；本专项文件全 clean。

## 编译与测试

- `cargo check -p country-core --lib`: **exit 0**（8 警告，含 time_of_day HOUR_TO_MS 良性、
  K/O 常量 dead-code——O 表完整保留）。
- `cargo test -p country-core --lib`: **1725 过 / 1 败**（geometry/sphere 他组既有，审计
  基线 1709/1 一致；本专项新增测试全过）。

## 留给 reviewer 核对点

1. translate_input 动作↔槽映射: `ANALOG_TARGETS`/`DIGITAL_TARGETS` 逐条对 0x1417cd0a0 反编译；
   滚轮权重（tag0→8.0）与右键 `MouseMoved < 3.0` 阈值语义。
2. InputReplayInitialState 10 字段名/序对键表 0x142c5b422；seek 子字段访问器偏移。
3. ui_draw_terrain 三道门 + 画布注册/界外事件。
4. suncalc_pos_to_vec 常量直读对 RNE 字节；两级旋转方程对 decomp 行 383..422。
5. ui_edit_terrain Adjustment→Delta 与 RE-EDITCMD 生产者判别。
