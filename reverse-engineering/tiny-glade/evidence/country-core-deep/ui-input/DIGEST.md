# ui-input 域 DIGEST（tg_a5，2026-10-14）

**Round2 更新（详见 evidence/country-core-deep/round2/ui-input-CLOSURE.md）**：
① check_if_can_switch 尾段 = `unreachable!()` panic（toolbar.rs line 40），非分发；真正的按模式分发在
mode_manager 跳表块内：mode id = 10+5k（k=工具栏视觉序 {Buildings:0,Walls:1,Decor:2,Misc:3,Paths:4,
Water:5,Animals:6,Trees:7,Meadow:8,Terrain:9}）→ env_5..env_14。② EscMenuState = 8B {disc u32, sub u32}：
is_open = qword∈{3,4} || (disc7&&sub1)；同文件另有 NextEscMenuState/PrevEscMenuState/PhotomodeScreen
资源（PDB 证实）。③ mouse_capture 的 0x200000002/3 = EscMenuState 值 {2,2}/{3,2}（非 SettingsMenuState）。
④ CameraMode 槽 6 = 6 姿态子跳表（body+fov 对），槽 7..12 fov 偏移 {8,0x20,0x14,0x2c,8,0x14,0xc}；
PhotoModeCameraMode 0..3 的写入点属 toolbar::photomode（skeleton 缺 toolbar crate，卡点）。

范围：`systems/ui/**`、`ui_*` 顶层系统、`mode_manager`、`mod_key`、`input/**`（含
`systems/backend/input/*`）、`mouse_capture`、`toolbar`、`photo_mode`、`onboarding`、`gui`。
证据：本批 157 过程反编译（`decomp/`，137 OK / 20 NotFound 折叠小函数已 --hex 手解）、
wp15/wp20 既有 analysis、rne-read 字符串/字节直读。标注 O=机器码证实 / I=推断 / U=未知。

## 0. 工具链备注（供中央汇总）

- `run-decomp-batch.py --chunks 2` 存在 **runner bug**：分片时 open_database 用 `tg_a5_c0/c1`，
  但 decompile 步骤仍写 `database: tg_a5` → 157 个函数全部 "Database not found" 失败
  （首轮产物是 202B 错误文本）。改为单进程重跑后成功。建议修复 runner 或禁止 --chunks。
- 本批 20 个 NotFound 全部是 ICF 折叠单行/小函数，已按 PLAYBOOK §1.3 用 `--hex` 手解，见 §3。
- `sysres-procedures.json` 不含 `input/action_set.rs`、`input/input_stream.rs`、`input/mod.rs`、
  `input/panic_repro_state.rs` 的 key（这些过程不在 1161 清单里），该子域复用 wp15 b1_input
  既有反编译（`reconstruction/country-core-render/wp15/analysis/b1_input/`）。

## 1. 模块清单（本域 157 过程 + 补充）

| 模块 | 过程数 | 主要地址 |
|---|---|---|
| systems/mode_manager.rs | 17 | mode_manager 0x140a86b60; send_mode_just_changed_event(closure$0) 0x140a875e0; check_if_can_switch 主体 0x140afa8a0; 闭包 env_0/env_3..14 @0x140af9e00..0x140afadb0 |
| systems/timelapse_replay.rs（mode_manager 联调） | 3 | start_timelapse 0x140a8a310; finish_timelapse 0x140a8a460; cleanup_timelapse 0x140a8a5d0 |
| systems/mod_key.rs | 2 | Modifiers::disable_snapping 0x140af9bf0; is_disable_snapping_just_pressed 0x140af8f70（PDB，折叠） |
| input/** + backend/input/** | 14 | input_record_replay 0x1409423d0; control_input_replay 0x140941a30; advance 0x140942280; record_and_replay_for_trailers 0x14083c620; do_input_replay_ui 0x14083cdd0; translate_input 0x1417cd0a0; translate_input_to_cursor_moved 0x140b17a80; calculate_input_delta 0x1416feeb0; 另有 wp15 b1_input 的 action_set/input_stream 全组 |
| systems/ui/resources/ui_state.rs | 12 | add 0x140922fb0; add_with_id 0x14092e3e0; clearing_canvas_w_layer_and_sound 0x14092df50; 等 |
| systems/ui/resources/widget_params.rs | 12 | new 0x1409a8e10; sounds 0x1409a8e80; occlude_by_world 0x1409a8ee0; no_history_recording 0x1409a8f10; allow_negative_distance 0x1409a8f40; UiFocus::{as_mei,is_mei,is_toolbar} 0x1409a9130/0x1409a9140/0x1409a9150 |
| systems/ui/ui_update.rs + hover_intersection.rs | 3 | ui_update 0x140b2de60; get_ui_volume 0x140b2dc60; hover_intersection 0x1408383a0（wp15 b5 既有） |
| systems/ui/signifiers/** | 8 | 见 wp15 合同（b7 既有），本批复核 |
| systems/ui_edit_focus.rs | 3 | ui_edit_focus 0x1409771f0; geo_src_to_edit_subject 0x140977f50; _w_deco_clutter 0x140978000 |
| systems/ui_move_shapes.rs / ui_raise_shape.rs / ui_2d_follow_mouse.rs / ui_camera_mode_gizmos.rs | 7 | 0x14097a060/0x14097d280; 0x140952e70/0x140955650; 0x14092a100; 0x1409527a0/0x140952b60 |
| resources/toolbar.rs | 5+ | Toolbar::new 0x140af9000; switch_mode 0x140af9850; contains 0x140af89d0; next_mode 0x140afb590（PDB 错位，见 §7）; serde 0x140afbba0/0x140afc140/0x140afb5b0 |
| resources/photo_mode.rs | 3 | Plugin build 0x140b59640; next 0x140b59720(既有); next_demo 0x140b596b0; PhotomodeState::on_enable_photomode 0x140870570（wp15 b10） |
| resources/gui.rs | 3 | EscMenuState::{is_open,allow_time_passage} 0x14093fc00/0x14093fc60; DebugTriggerProxyVis::as_string 0x14093ffd0 |
| resources/mouse_capture.rs | 1 | mouse_capture 0x140b17830 |
| resources/onboarding.rs | 1 | WelcomeScreenState::is_closed 0x1409a9140（与 UiFocus::is_mei 折叠） |
| resources/ui_settings.rs | 1 | serialize 0x140a46ad0（pretty serde_json） |
| systems/ui_2d_pass/ui_pass（render_loop 侧） | 3 | ui_2d_pass 0x1409c4470; ui_pass 0x140837c40; loading_screen_pass 0x140837ff0 |

## 2. 布局表（资源/结构）

### 2.1 Toolbar / ToolbarCategoryModes（mode_manager 核心联动，本批重点纠正）

- **Toolbar** = `{categories: Vec<ToolbarCategoryModes> {cap@0x00, ptr@0x08, len@0x10}, current: u8@0x18}`（O：
  Toolbar::new 0x140af9000 尾部写 `{len=0xb, ptr, cap=0xb, byte@0x18=0}`；各 check 闭包读
  `len@+0x10 < N` 与 `byte@+0x18 == 类别`）。**类别数量 11**（非 demo）。
- **ToolbarCategoryModes** = 0x20 B = `{modes: Vec<ModeEntry> (cap@0x00, ptr@0x08, len@0x10), current: u64@0x18}`
  （O：闭包按 `data + cat*0x20 + 0x10/0x18` 读 modes.len / current）。
- **ModeEntry** = 0x10 B = `{cat: u32@0x00, mode: u8@0x04, pad 3B, arg: u64@0x08}`（O：
  Toolbar::new 的 `dword@0=cat + byte@+4=mode` 写入序列；contains 0x140af89d0 读 `[rdx+4]` 为 mode；
  闭包按 0x10 步长拷贝整条 `{u32@0, u64@4, u32@0xc}` 到请求记录）。
- **Toolbar::new @0x140af9000** 建的 11 类（O，非 demo / demo 对比）：
  | cat | 模式列表（非 demo） | demo |
  |---|---|---|
  | 6 (Buildings) | [1, 2, 0x3f, 0, 0] 共 5 条（含 (0xb,0x3f) 与 (0x12,0) 两条跨类条目！见 §7） | [1, 2, 0] |
  | 7 (Walls) | [8, 1, 2, 3] | [0]（1 条） |
  | 0xb (Decor) | [0x38, 0x39, 0x3a, 0x3c, 0x3d, 0x3b] | [0x38, 0x3c] |
  | 0xc (Misc) | [0]（1 条） | [0] |
  | 9 (Paths) | [1, 2, 3, 5, 6, 4] | [1, 2, 5, 3] |
  | 0x10 (Trees) | [1] | 空（0 条，ptr=8 哨兵） |
  | 10 (Water) | [0] | [0] |
  | 0 (未命名/清除类) | [0] | [0] |
  | 0xf (Meadow) | [0] | 空 |
  | 0x11 (Animals) | [0] | [0] |
  | 0x13 (Terrain) | [0] | [0] |
  注意：类别 id 集合 = {6,7,8,0xb,0xc,9,0x10,10,0,0xf,0x11,0x13}，含一个 **id=0 的类别**（重建代码
  `ToolbarCategory::from_id` 未收录 0 与 0x12——0x12 作为 ModeEntry.cat 出现在 Buildings 列表）。
- **ToolbarCategoryModes::contains @0x140af89d0**（O 手解）：入参 `&{cat u32@0, mode u8@4}`；
  SIMD 16B 块遍历 0x10 步长元素；`cat <= 5` 时对 mode 做区间匹配 `0x38 <= mode < 0x40`
  （`lea eax,[mode-0x38]; cmp al,8`），否则精确匹配。重建代码写 `0x38..=0x3d` 区间——应为 **0x38..=0x3f**（纠正）。
- **switch_mode @0x140af9850**（O）：入参 `(&mut Toolbar, &{cat u32, mode u8, arg u64})`（**不是**重建代码的
  `(u32, u8)` 两参）；逐类 contains 匹配后写 `Toolbar.current = 类别下标`。panic 串存在但原文与重建代码的
  "Switched to mode but no toolbar category contains it" 不一致（实际串含片段 "mode but no toolbar"，
  位于 0x142ae3cbc 附近的字符串表中，完整原文 U）。
- **next_mode @0x140afb590**：PDB 名与行为错位（见 §7）——实为 4 指令 thunk，尾跳到
  `(&"variant identifier", 0x12)` 的 serde unknown_variant 错误路径；真 next_mode 地址 U（被折叠）。
- 反序列化 **visit_str @0x140afb5b0**：比较 `"Path"`（`cmp dword [rdx],'Path'`）——工具栏类别按名
  序列化（"Path"…），供 input replay 流持久化。

### 2.2 AppMode / 模式切换状态（mode_manager）

- 模式判别 id 空间：u32，至少 0..0x40+（jumptable @0x142ad890c，0..64 全部指向 mode_manager 函数体
  0x140a86cec..0x140a874e8 内的按模式处理块）。已知 id：**2 = Timelapse**（early-return 检查
  `mode@+0x28 != 2`；ui_camera_mode_gizmos 同判）、**10 = 带标志的特殊模式**（gizmos 检查
  `mode==10 && 第二 qword byte != 0`；ui_edit_focus `mode == 10` 特判）、**0xd = Replay**（见下）。
- **check_if_can_switch 主体 @0x140afa8a0**（O 手解，非 157 清单内）：
  - `force=true`：向 param_2 的队列 push 0x20 B 记录 `{u32@0 = 4, ..., u64@0x18 = 事件计数}`。
  - `force=false`：**先把闭包环境[0]（mode_manager 的 param_1 = ResMut<模式状态>）的首 u32 置为 0xd（Replay）**，
    再向请求队列 push 0x10 B 记录 `{事件计数 u64@0, 请求 mode u32@8}`（请求 mode 取自闭包环境[2] 的解引用 u32）；
    随后按 pending 事件的 `byte@+1` 与 2 比较分流：<2 → 再 push 一条 `{id, byte}` 记录；>=2 → 进入按
    类别/模式的闭包分发（尾段未完全手解，U）。
  - 语义（I）：mode_manager 收到模式切换请求时先把 pending 置为 **Replay(0xd)**（历史回放联动），
    通过后由闭包/跳表执行真正的切换。
- **15 个 check_if_can_switch<closure_env_N>**（O，每个 ~2.3KB，两段式）：
  - 段一 `param_1 != 0`（force）：push `{u32 tag=4, id=计数}` 0x20 B 记录（同上）。
  - 段二（类别 N 的推进逻辑）：`Toolbar.categories.len <= N` 返回；`ToolbarCategoryModes[cat N].modes.len == 0` 返回；
    若 `Toolbar.current == N`：`idx = (current+1) % len`（带 32/64 位溢出双路径）；否则读 current，`current >= len`
    时 panic（FUN_1428d9518，文件 toolbar.rs @UNK_142ae5638——越界 panic）；回写 `current`；`Toolbar.current = N`；
    取 `modes[idx]`，若其 `cat != 0x15`（0x15 = 空哨兵）把 0x10 B 记录拷入 pending 请求槽
    `{cat u32, arg u64, mode u32}`。
  - **闭包↔类别映射**（O）：env_3=cat7(Walls，比较 `=='\x07'` 且调用 switch_mode)；env_6=cat1；env_7=cat2；
    env_8=cat3；env_9=cat4；env_12=cat5；env_10=cat6(Buildings)；env_13=cat8(Roofs，entry@0x110)；
    env_11=cat9(Paths，entry@0x130)；env_14=cat10(Water，entry@0x150，且 `request.cat==0x13` 时清零 cat
    后直接 switch_mode)。env_4/env_5 为动态类别变体（类别从参数来，U 细节）。
  - **env_0 = timelapse_replay::start_timelapse::closure_env_0 @0x140af9e00**（O）：非 force 时
    (a) 把闭包环境[1]（Res<AppMode>）的 2 qword AppMode 对写入环境[0] 的 ResMut 状态（pending 模式 = 当前 AppMode）；
    (b) 从相机资源读 f32@+0x60、u32@+0x68、u32@+0x6c，组 40B 栈记录
    `{0x8000000000000000(i64::MIN), f32, u32, u32, 0, byte=1, ..., 0x8000000000000001}` 调 FUN_140b59be0
    （MetaCommand 入队，文件 meta_commander.rs——timelapse 起始元命令）；
    (c) `time_scale = f32 * _UNK_142925988`（全局乘数）后 push 事件 `{u32=1, f32=time_scale}`；
    (d) 置环境[6] 的 ResMut 字节 = 0。
- **send_mode_just_changed_event（mode_manager::closure$0）@0x140a875e0**（O）：比较 prev/current AppMode
  （各 2 qword，`PartialEq::ne`），不等则 push 0x28 B 事件 `{prev@0x00, current@0x10, 计数@0x20}` 并把
  `current := prev`。**事件载荷是 {prev, current} 双 AppMode，不是 RequestAppModeChange**（纠正重建代码）。
- **mode_manager @0x140a86b60**（O）：
  1. `hash("esc_menu_canvas")`（0x142ad8501，15 字符）；若 UiWidgetContext 型状态（param_4）满足
     `hover 字节@+0x18 != 0 && widget id@+0x38 == 该 hash && 交点载荷 u32@+0x28 != 2(None)` 且
     `mode@+0x28 != 2` → **早退**（ESC 菜单画布正被悬停/相交时阻塞模式机）。
  2. 排空模式请求双队列（VecDeque 布局 `{cap@0,ptr@8,len@0x10,head@0x18}` ×2 @0x00/@0x20 + 计数@0x40；
     元素 0x10 B = `{事件id u64@0, 载荷 u64@8}`——bevy 0.16 EventInstance 布局，见 §2.7）：
     有未消费请求 → 取最后一条，调 `check_if_can_switch(force=双队列读数差为0, pending, &closure_env)`。
  3. 按 `状态@+0x80 的 u32 模式 id` 走 jumptable @0x142ad890c 分发（0..64 → 函数体内各模式处理块）。
- **start_timelapse @0x140a8a310 / finish_timelapse @0x140a8a460 / cleanup_timelapse @0x140a8a5d0**（O，wp20 既有复核）：
  - start：排空请求；有未消费 → `check_if_can_switch<start_timelapse::closure_env_0>`。
  - finish：排空；有未消费 → `MetaCommander::extend`(1 条 MetaCommand) + 标志字节=1；随后若
    `窗口焦点态@+0x18 == 0` → push `EventInstance<WindowFocusEvent>` 0x10 B `{id@0, 事件 u8@8 = 0}`（失焦）。
  - cleanup：排空；有未消费 → log（文件路径经 TweakerView 内联，日志原文 U）并把 `*(byte*)*state = 2`
    （回到 Timelapse 模式）。

### 2.3 input 域（action_set / input_stream / 回放状态机）

- **EventInstance 布局（权威）**：bevy_ecs 0.16.0（二进制内嵌路径 = crates.io 官方 registry
  `/home/h3/.cargo/registry/.../bevy_ecs-0.16.0/src/event/collections.rs`）=
  `{event_id: {id: usize@0, caller: MaybeLocation@8(无 track_location 时 ZST)}, event: T@8}`（本地
  cargo registry 源码 base.rs L118/182 已核对）。实例大小 = 8 + size_of::<T>()。
  `Events<T>` = 双 VecDeque `{cap,ptr,len,head}` @0x00/@0x20 + 计数@0x40（send 写 buffer b：cap@0x20/ptr@0x28/len@0x30）。
  机器码各 push 点与此一致（mode_manager 请求队列 0x10B/{id@0,event@8}；WindowFocusEvent 0x10B；
  translate_input_to_cursor_moved 的 InputCursorMoved `{id@0, Vec2@8}`）。
  例外：check_if_can_switch 的 force-push（0x20 B，`{4@0, 计数@0x18}`）与 send_mode_just_changed
  （0x28 B，`{prev@0, current@0x10, 计数@0x20}`）不符合该布局，疑为**自定义记录队列**（I，见 §7 待定项）。
- **InputStream** = 64 B `{sync_points: VecDeque<InputStreamSyncPoint>, frames: VecDeque<InputFrame>}`
  （wp15 合同已证实；本批以 wp15 b1_input 为据复核：add_frame 0x140949190 / on_history_sync
  0x1409494c0 / drop_oldest_sync_point 0x140949660 / try_load 0x1409499c0）。
- **InputFrame** = 0xd0 B（O，input_record_replay case 2 读出）：@+0x30 u64、@+0x38 u32、@+0x3c u8 →
  输出状态 @+0xe8/+0xf0/+0xf4；@+0x40 双 f32 游标 delta；@+0x44 f32 缩放除数；@+0x58..+0xc0 十 qword →
  `InputTranslationState` 0x58..0xa8 区（11 qword = 88 B 状态快照）+ 尾部。回放时
  `index@+0x48` 帧序、`mode@+0x40`（`^0x8000000000000000` 后 case 0=Idle/2=Replaying/3=Recording/4=Finished），
  越界 panic "Out of bounds access"（UNK_142ab3224，input_record_replay.rs）。
- **input_record_replay @0x1409423d0**（O）：Idle 且条件满足（工具/画布字节 @+0xa3 bit0 等）→
  `MainCamera::store_parameters()`、克隆 ToolbarCategoryModes（UNK_142ab2ba0=toolbar.rs）、以 1 个 sync point
  重建流（frames 容量 8）、`add_frame` 当前游标 delta 帧（delta 经 `_UNK_142a171f0` 2×2 矩阵 + 偏移
  `*param_7` 变换，再除以 fVar31 归一——I：屏幕空间→世界变换）。
- **control_input_replay @0x140941a30 / advance @0x140942280**（O，wp20 合同复核通过）：
  键控录制/回放；Replaying 时 index++，帧尽 → mode = `0x8000000000000004`（Finished）。
- **record_and_replay_input_for_trailers @0x14083c620**（O）：`配置字节@+0xfc == 1` 且命中
  HashMap<KeyCode>：Idle/Finished → log "Start recording" + store_parameters + 重建流 + mode=Idle；
  Recording → "Finished recording" + mode=Finished；Replaying → "Replay..." + index=-1 + 恢复
  InputTranslationState + `MainCamera::load_parameters()`。
- **do_input_replay_ui @0x14083cdd0**（O 串）：egui 面板 "Enable input recording"（写 @+0xfc）、
  "Stream frame count: "、`` `O` - start/stop recording ``、保存路径含 "tmp/input_replay"+"json"。
- **translate_input_to_cursor_moved @0x140140b17a80**（O）：遍历录制事件双队列（stride 0x18，
  tag@+8：0=mouse_moved/1=moved/2=跳过），NaN 安全 clamp，累加 delta，push `InputCursorMoved`
  （0x10 B `{id, Vec2}`）。
- **translate_input @0x1417cd0a0 / calculate_input_delta @0x1416feeb0 / reset_input_state
  @0x140942b00**：wp20 合同复核通过（analog→ActionSetValues、digital→ActionSetStates、MouseMoved 消费；
  死区 kind2 走 0.1/0.05、kind1 累加 delta@+0x58；reset 清 11 qword + 按最后 sync point 修剪 +
  `ButtonInput::reset_all<KeyCode>`）。
- **ActiveInputModeTracker**（wp15 b1 复核）：32 B；`new_default_controller/mouse_keyboard`
  0x140958eb0/ef0 用 `Instant::now()` + duration 数学建 tracker，mode u32@0。
- **PanicReproState::is_inactive @0x140833230**（O 手解）：`cmp byte [rcx],0; sete al` → 0 = Inactive。
  17 调用点，gates 调试 panic 复现输入路径。

### 2.4 systems/ui/resources（UiState / WidgetParams）

- **UiState**（wp15 合同 + 本批复核）：`{..., RwLock 锁字@+0x60, HashMap<WidgetId, Arc<WidgetParams>>@+0x68}`；
  `add_with_id` 0x14092e3e0：`LOCK; [p+0x60]==0 → 写 8` 取写锁（超时 1e9 ns，FUN_1428cb5f0），拷贝
  0xb0 B WidgetParams，alloc 0xc0 B Arc（ArcInner），HashMap insert。
- **WidgetParams** = 0xb0 B（wp15 合同默认值序列复核通过）：u32 `{0,8,0,1,0,2,3,3}` @
  `{0,8,0x10,0x18,0x20,0x30,0x48,0x60}`；sounds 区 @0x18..0x78（UiSounds 4×24B）；volume @0x78..0xa8
  （WidgetVolumeShape 32 B {tag u8@0, layer u8@1, size Vec2@0x10}）；`u16 @0xa8 = 0x101`；
  occlude_by_world = 字节@0xa8 bit0；no_history_recording = 字节@0xa9 bit0；allow_negative_distance = 字节@0xa9 bit2。
- **clearing_canvas_w_layer_and_sound @0x14092df50**：体积 **130×130**（`0x43020000_43020000` 双 f32），
  哈希 widget 名后插 Arc（0xc0）入表。
- **UiFocus**（O 手解）：u32 枚举。`is_toolbar` 0x1409a9150 = `== 1`；`is_mei` 0x1409a9140 = `== 2`
  （与 WelcomeScreenState::is_closed 折叠，两侧判别恰好相同——巧合折叠，语义不同）。变体序：
  None=0, Toolbar=1, Mei=2, World=3（I）。重建代码枚举序正确。
- **WelcomeScreenState::is_closed**（onboarding，折叠处同址）：`*self == 2` → Closed 判别 = 2。

### 2.5 ui_edit_focus / ui_move_shapes / ui_raise_shape / ui_2d_follow_mouse / ui_camera_mode_gizmos

- **ui_edit_focus @0x1409771f0**（O）：
  - `mode u32 > 5` → 写 `bool = (mode == 10)`；`== 10` → WallShape 跳表 A（UNK_142ab73a0）；
    `6..=9` → 跳表 B（UNK_142ab73b4）；`len@+0x10 == 0` → 跳表 C（UNK_142ab738c）。
  - `mode < 6`：注册 widget **"edit_focus_canvas"**（UNK_142ab69e0，0x11=17 字符）：
    WidgetParams 栈记录 `{tag=5, f32 0x7f7fffff(MAX), u16 0x100...}`；然后按 mode 跳表 D（UNK_142ab73c8）。
  - 模式 1/2 位测（`6 >> mode & 1`）→ 命中时走「跟随」路径：FUN_140a1c930 谓词（I：History/相机
    状态检查）通过才继续；未命中（0 或 ≥3 且 0x18 位测失败）→ 遍历 wall 实体（SIMD 16B lane 匹配，
    元素反向步长 0x2a8——元素基址在指针前方，即反向存储的 wall 数组），命中实体后检查字段
    `< 2` 或 `f32 <= 0` → 先 panic `"assertion failed: w.simplified_curve_2d.is_valid()"`
    （UNK_142ada193，public_walls.rs，line 0x32），再 panics
    `"internal error: entered unreachable code: ui_edit_focus shouldn't include ..."`
    （UNK_142ab6a70，ui_edit_focus.rs）。
  - 结尾 push 0x18 B 记录 `{u32@0 = 0(标签), u64@0x10 = 计数}`（I：UiHistoryPlaySelectAnimation/
    ToolSelectionCancelled 类事件；事件队列 Vec@+0x20..0x30 + 计数@+0x40）。
  - 4 个分发跳表（UNK_142ab738c/142ab73a0/142ab73b4/142ab73c8）互为首部旋转的同一张 24 项表
    （idx 0..23，目标全在 ui_edit_focus 函数体内 0x14097745f..0x140977e90）——GeoSource/WallShape
    枚举约 24 变体，4 处 match 入口共用（O，跳表字节解码）。
- **geo_src_to_edit_subject / _w_deco_clutter** @0x140977f50/0x140978000（O，172KB/含内联）：GeoSource→
  EditSubject 映射。EditSubject/GeoSource 变体 serde 名（O 串，UNK_142ab7268 起）：
  `DecoratorCollision`、`decorator_id`、`addr`、`DecoratorInteraction`、`StairInteraction`、
  `ColumnStairs`、`Fence`、`Arches`、`Tree`、`Terrain`、`AutoClutter`、`entity`、`WallId`、`wall_id`、
  `Floor`、`Roof`、`flat`、`gable`、`StairGraphEdge`、`graph_edge`、`StairSupport`、
  `SupportedStairStructural`、`point_idx`、`UnsupportedStairStructural`。
- **ui_move_shapes @0x14097a060**（O 要点）：widget 名 **"move_shape"**（UNK_142ab6c90）；
  `move_gizmo_and_trigger_vol` 0x14097d280 施加 delta + 触发 volume 重算（wp20 合同复核）。
- **ui_raise_shape @0x140952e70 / impl_raise_shape 0x140955650**（O 要点）：widget 名
  **"outline_wall_bottom_" / "arrow_raise_shape_" / "raise_wall_elevation_"**（UNK_142ab4548/45f8/46c0）；
  `ProportionalScreenSpaceCoord::project_onto_ray` + `History::add`（wp20 合同复核）。
- **ui_2d_follow_mouse @0x14092a100**（O）：puffin scope 名 = 函数全名；双分支 Query（标志@+0x108），
  源索引 @+0x94（stride 0xa0）/@+0x10（stride 0x48）→ 每实体槽写游标 Vec2 + u32 layer（wp20 合同复核）。
- **ui_camera_mode_gizmos @0x1409527a0**（O）：
  - 模式门：`mode != 2 && !(mode == 10 && 第二qword byte == 0)` → 返回。
  - 相机模式字节 @param_8+0x4d0：`(byte & 6) == 2`（Free 相机）→ 返回。
  - 窗口 w@+0x10/h@+0x18 均 > 2 → 返回（窗口过小才画 gizmo）。
  - `*(int*)*param_9 != 0x14`（20）→ `UiState::add` 注册 widget **"camera_mode_active"**（0x12=18 字符）。
  - 工具模式字节 @param_6+0x11c：`==6 → 0`；`>4 → 返回`；`0x13 >> v & 1 == 0`（限 {0,1,4}）→ 返回；
    通过后经 AppendOnlyVec 推 gizmo signifier **"rotation_"**（UNK_142ab4438，closure FUN_141281be0）。

### 2.6 toolbar / photo_mode / gui / mouse_capture / ui_settings

- **PhotoModeCameraMode**（O）：u8 枚举 4 变体 {0,1,2,3}。`next`：0→1→2→3→0；`next_demo` 0x140b596b0：
  0→1→2→0，**3 → panic "internal error: entered unreachable code: this camera mode is not part of the
  demo!"**（UNK_142af1188，photo_mode.rs）。PhotoModePlugin build 0x140b59640 = register_event + 组件注册
  胶水（bevy event/registry.rs、component.rs）。
- **PhotomodeState::on_enable_photomode @0x140870570**（wp15 b10，O）：`f32@0 = 3.0`（非 demo）/
  `1.0`（demo）；`u32@4 = 0xffffffff`；`byte@10 = 0`；`byte@11 = 0`。
- **EscMenuState**（O 手解）：`is_open` 0x14093fc00 = qword 判别 `∈ {3,4}` 或 `==7 且附加条件`；
  `allow_time_passage` 0x14093fc60 = `!(state==7 && byte@+8==1)`。判别空间含 {3,4,7}（变体名 U）。
- **DebugTriggerProxyVis::as_string** 0x14093ffd0（O 结构）：按 u8 索引查串表
  （`FUN_1428d8fe3(1, 0x12/0xe/0x10, ...)` 长度 18/14/16 的字符串索引 + 不可恢复跳表 = match 分发），
  生成调试代理可见性描述串（egui 调试面板）。
- **mouse_capture @0x140b17830**（O）：消费 CursorCaptureCmd（0x10 B 事件队列，取最后一条）：
  - 捕获（flag==0）：写 `CursorCapturedPosition = {1, u64 窗口坐标}`；push 0x18 B 事件
    `{id, u32=0, u64=窗口}`（WindowFocusEvent{focused=0} 家族）；push 0x10 B 事件 `0x200000002`
    （= `{2, 2}` 双 u32 事件，I：SettingsMenuState 类）。
  - 释放（flag!=0）：`CursorCapturedPosition = {0, ...}`；push `{id, u32=1, 窗口}`（focused=1）；push
    `0x200000003`。
- **UiSettings**（O，serialize 0x140a46ad0 pretty serde_json 字段序）：`{icon_size_mult f32@0x00,
  custom_cursor_size f32@0x04, toolbar_alignment u8@0x08, auto_hide_toolbar u8@0x09,
  input_lag_reduction u8@0x0a, game_logo u8@0x0b, custom_cursor u8@0x0c}`。ToolbarAlignment 变体名 =
  **Bottom / Left / Right / Top**（UNK_142acec66 串表，**不是**重建代码的 Horizontal/Vertical/Bottom/Side）。
  默认值（wp21 O）：`{1.0, 24.0, 0x00000101, 0}` → alignment=1（第二变体，U 是 Left 还是别的序）、
  auto_hide_toolbar=true。

## 3. NotFound 小函数手解（--hex）

| 地址 | 字节 | 语义 |
|---|---|---|
| 0x140af9bf0 Modifiers::disable_snapping | `0f b6 41 01 c3` | 返回 `byte [rcx+1]`——**Modifiers 是位标志字节组，disable_snapping 在字节1**（不是 4 个 bool 字段） |
| 0x1409a9150 UiFocus::is_toolbar | `83 39 01 0f 94 c0 c3` | `*self == 1` |
| 0x1409a9140 UiFocus::is_mei / WelcomeScreenState::is_closed | `83 39 02 0f 94 c0 c3` | `*self == 2` |
| 0x140833230 PanicReproState::is_inactive | `80 39 00 0f 94 c0 c3` | `byte *self == 0` |
| 0x14093fc00 EscMenuState::is_open | 见 §2.6 | 判别 ∈{3,4} 或 7+条件 |
| 0x14093fc60 EscMenuState::allow_time_passage | 见 §2.6 | `!(state==7 && flag@8==1)` |
| 0x140afb590 ToolbarCategoryModes::next_mode | thunk → serde unknown_variant("variant identifier", 0x12) | **PDB 名错位**（§7） |
| 0x140b12f30 ui_settings previous | (81B NotFound) | U，折叠 |
| 0x1409de170 wall_eraser_bulldoze_condition | (81B NotFound) | 跨域（wall/eraser），U |
| 0x1416feeb0 calculate_input_delta | NotFound（Ghidra 未建函数） | wp20 既有反编译可用，合同复核通过 |

## 4. panic/日志串清单（原文 + 地址）

| 串 | 地址 | 出处 |
|---|---|---|
| `esc_menu_canvas` | 0x142ad8501 | mode_manager 哈希的 widget 名（另一副本 @0x142ad6b01） |
| `edit_focus_canvas` | 0x142ab69e0 | ui_edit_focus widget 名 |
| `move_shape` | 0x142ab6c90 | ui_move_shapes widget 名 |
| `outline_wall_bottom_` / `arrow_raise_shape_` / `raise_wall_elevation_` | 0x142ab4548/0x142ab45f8/0x142ab46c0 | ui_raise_shape widget 名 |
| `camera_mode_active` / `rotation_` | 0x142ab4418/0x142ab4438 | ui_camera_mode_gizmos widget/signifier 名 |
| `assertion failed: w.simplified_curve_2d.is_valid()` | 0x142ada193 | ui_edit_focus → public_walls.rs（line 0x32） |
| `internal error: entered unreachable code: ui_edit_focus shouldn't include ` | 0x142ab6a70 | ui_edit_focus.rs（格式化 panic） |
| `internal error: entered unreachable code: this camera mode is not part of the demo!` | 0x142af1188 | photo_mode.rs next_demo |
| `Starting input replay` | 0x142ab2650 | input_record_replay.rs |
| `Failed to load an input recording from ` | 0x142ab26e0 | input_record_replay.rs |
| `No sync points found in replay state` | 0x142ab2740 | input_record_replay.rs |
| `Not enough player inputs to rewind ` | 0x142ab27b0 | input_record_replay.rs |
| `Out of bounds access` | 0x142ab3224 | input_record_replay.rs（case 2 越界 panic） |
| `Can only seek to sync point if replaying but mode is ` | 0x142ab3658 | input_stream.rs |
| `Start recording` / `Finished recording` / `Replay...` | 0x142a93740/0x142a93678/0x142a93778 | input_replay_for_trailers.rs |
| `Enable input recording` / `Stream frame count: ` / `` `O` - start/stop recording `` / `Save` / `tmp/input_replay` `json` | 0x142a937a0/0x142a937e0/0x142a937f0/… | do_input_replay_ui（egui） |
| `icon_size_mult toolbar_alignment auto_hide_toolbar input_lag_reduction game_logo custom_cursor custom_cursor_size Bottom Left Right Top` | 0x142acec66 | ui_settings serde 字段 + ToolbarAlignment 变体 |
| `variant identifier` | 0x142ae59e4 | toolbar serde unknown_variant |
| `Path`（ToolbarCategory 名） | visit_str 0x140afb5b0 内联比较 | toolbar serde |
| `... mode but no toolbar ...`（完整原文 U） | 0x142ae3cbc 附近 | switch_mode panic |
| `/home/h3/.cargo/registry/.../bevy_ecs-0.16.0/src/event/collections.rs` | 0x142c3a148 | Events::send grow Location——引擎来自 crates.io 官方 registry（路径证据） |

## 5. 与现有合同/代码的差异（逐条）

对照 wp15 `contracts/{ui,input,app-shell,startup}.md`、wp20 `contracts/{ui-misc,input-backend}.md`
与 `reconstruction/country-core/src/**`：

1. **mode_manager.rs（代码）AppMode 缺 Replay 变体**：证据：check_if_can_switch 0x140afa8a0 把模式状态
   首 u32 写 **0xd**，任务要求的「Replay 判别 0xd」成立。代码枚举只有 None/Timelapse/Wall/Focus/Tool。
2. **mode_manager.rs `AppMode::is_tool_selected @0x140a8a5d0` 是错的**：PDB 清单中无此名；0x140a8a5d0 =
   `timelapse_replay::cleanup_timelapse`（wp20 既有反编译，机器码=带 ResMut/EventReader 的系统函数，
   不是单行 bool）。is_tool_selected 真实地址 U。
3. **mode_manager.rs `send_mode_just_changed_event` 事件类型错**：代码写 `EventWriter<RequestAppModeChange>`；
   机器码 push 0x28 B 载荷 `{prev: AppMode 16B, current: AppMode 16B, id}`——是双 AppMode 事件。
   （另：该函数在 PDB 清单中名为 `mode_manager::closure$0`。）
4. **toolbar.rs `Toolbar::new` 地址错**：代码写 0xaf8c50；机器码/PDB = **0x140af9000**。
5. **toolbar.rs 模式项布局近似但类别表有出入**：真实 ModeEntry = `{cat u32@0, mode u8@4, pad, arg u64@8}`
   （0x10 B）；代码写 `(u32, u8, u64)` 元组——布局等价但字段序书写易误读。类别表：
   代码缺 **类别 id 0**（存在）、缺 cat 0x12 条目（Buildings 非 demo 列表含 (0xb,0x3f) 与 (0x12,0)——
   注意这是 Toolbar::new 的原始字节，Ghidra 类型混淆可能把相邻向量读串，此两条目置信度中等，需再验）。
6. **toolbar.rs `switch_mode` 签名错**：代码 `(category: u32, mode: u8)`；真实入参 `(&mut Toolbar,
   &{cat u32, mode u8, arg u64})` 16 B 记录（check 闭包调用点证实）。
7. **toolbar.rs `contains` 模式区间错**：代码 `0x38..=0x3d`；机器码 `mode-0x38 < 8` → **0x38..=0x3f**。
8. **toolbar.rs panic 串**："Switched to mode but no toolbar category contains it" 未在二进制中找到
   （0 hits）；实际含 "mode but no toolbar" 片段，完整原文待取。
9. **mod_key.rs `Modifiers` 布局错**：代码 `{ctrl, shift, alt, disable_snapping}` 四个 bool；机器码
   `disable_snapping` 读 **byte@+1** → 位标志字节组（第 1 字节某位），至少 2 字节。
10. **ui_settings.rs `ToolbarAlignment` 变体名错**：代码 Horizontal/Vertical/Bottom/Side；二进制串表 =
    **Bottom/Left/Right/Top**。默认值数值部分正确（1.0/24.0/0x101/0）。
11. **photo_mode.rs**：next/next_demo 判别与 panic 串正确（复核通过）；PhotoModeCameraMode 是 u8 枚举
    （代码用 Rust enum 默认 i32 布局——需 repr(u8)）。
12. **onboarding.rs / gui.rs / mouse_capture.rs**：facade 表面；本批补齐：
    WelcomeScreenState.is_closed = 判别 2；EscMenuState 判别含 {3,4,7} + flag@8；mouse_capture 的
    0x200000002/3 事件与 CursorCapturedPosition 写入模式（§2.6）。
13. **wp15 ui.md / wp20 ui-misc.md 合同**：本批复核大体一致；补充：mode_manager 早退条件精确化为
    `hover(esc_menu_canvas) && 交点载荷 tag != 2`；UiState 锁超时 = 1e9 ns 位于 add_with_id
    （wp15 说 UiNavState 1e9-ns——同一数值两处出现）。
14. **wp20 ui-misc.md `update_mod_key_state` PDB rva @0x140AF9000 落在 Toolbar::new**：确认
    **0x140af9000 = Toolbar::new 本体**（本批反编译头 `country_core::resources::toolbar::Toolbar::new`），
    update_mod_key_state 实际地址仍 U（ICF）。

## 6. 跨领域线索

- **→ camera-time**：PhotoModeCameraMode {0..3}（next/next_demo 判别 0..3）与 camera-time 发现的
  CameraMode 槽 6..12（拍照/工具模式，6 个 body+fov 槽）的对应关系：本域证据只有
  ui_camera_mode_gizmos 的相机模式字节 @+0x4d0（free 判定 `&6==2`）、PhotomodeState::on_enable_photomode
  的 f32@0=3.0/1.0（疑似 zoom/fov）、env_0 读相机资源 +0x60 f32 组 PhotoMode 元命令。**判别值 0..3 与
  槽 6..12 的映射在本域内无法闭合**——需要 camera-time 的 main_camera/camera_rig 代码（0x140911e40
  camera_snapshot、0x140912c20 main_camera_update 等）核实 PhotoModeCameraMode → CameraMode 槽的写入点。
  建议中央把该问号挂到 camera-time。
- **→ history**：check_if_can_switch 置 pending = 0xd（Replay）即与 history 回放联动；ui_edit_focus 模式
  1/2 路径调用 FUN_140a1c930（I：History/相机谓词）——需 history 域确认真实语义（地址
  0x140a1c930，输入 = in_stack_00000058 的 Res 状态 + puVar3[1]）。
- **→ wall**：ui_edit_focus 遍历 0x2a8 反向步长 wall 数组并断言 `w.simplified_curve_2d.is_valid()`
  （public_walls.rs line 50）；EditSubject 变体串含 WallId/wall_id/Floor/Roof/flat/gable/Stair*（serde 名）
  ——供 wall 域恢复 EditSubject 时使用。ui_raise_shape/ui_move_shapes 触发 volume 重算与
  `History::add`。
- **→ audio-misc / 全局**：ui_settings 字段串与 ToolbarAlignment 四变体串（0x142acec66）供
  ui_settings 恢复；EscMenuState 判别 {3,4,7} 供 gui/menu 域。
- **→ meta_commander（app-shell）**：env_0 的 FUN_140b59be0（Location=meta_commander.rs）推 timelapse
  元命令；finish_timelapse 直接 `MetaCommander::extend` 1 条 + 标志字节。
- **→ render_loop**：ui_2d_pass 0x1409c4470（"ui_2d"/"ui_2d_color_button"/"UI 2D" 串）、ui_pass
  0x140837c40/loading_screen_pass 0x140837ff0 属渲染侧，本域仅登记。

## 7. 错位/纠错（PDB 名 vs 行为）

1. `ToolbarCategoryModes::next_mode` PDB @0x140afb590 → 实为 serde unknown_variant 错误 thunk
   （尾跳 `&"variant identifier"`）。真 next_mode U（可能被 ICF 吸收）。
2. `mode_manager::closure$0`（PDB）@0x140a875e0 → 行为 = send_mode_just_changed_event（wp20 名）。
3. `AppMode::is_tool_selected @0x140a8a5d0`（重建代码头注释）→ 该地址实为 cleanup_timelapse；
   PDB 清单无 is_tool_selected 条目。
4. `UiFocus::is_mei` 与 `WelcomeScreenState::is_closed` 同址折叠（0x1409a9140，均为 `==2` 判别）——
   语义巧合（UiFocus.Mei=2 与 WelcomeScreenState.Closed=2）。
5. `update_mod_key_state` PDB rva 0x140AF9000 落于 Toolbar::new 体内（wp20 已知）——本批确认
   0x140af9000 就是 Toolbar::new 本体起点。
6. EventInstance 布局两个观察与官方 bevy_ecs 0.16.0（{id u64@0, event@8}）不符的点（check_if_can_switch
   force-push 0x20B `{4@0, 计数@0x18}`、send_mode_just_changed 0x28B `{prev@0, current@0x10, 计数@0x20}`）：
   **疑为自定义记录队列而非 Events<T>**（待定项，建议中央复核）；其余所有 push 点（请求队列、
   WindowFocusEvent、InputCursorMoved、finish_timelapse、mouse_capture）均符合官方布局。

## 8. 待定项（U，不臆造）

- check_if_can_switch 尾段（byte@+1 >= 2 之后的闭包分发码）未完全手解。
- mode_manager 跳表中 0..64 各模式 id 的语义名（仅 2=Timelapse、0xd=Replay、10=特殊模式已知）。
- EscMenuState 判别 {3,4,7} 的变体名；0x200000002/3 事件的确切类型名。
- Toolbar::new 中 Buildings 列表混入 (0xb,0x3f)/(0x12,0) 条目的真伪（Ghidra 类型混淆风险）。
- switch_mode panic 串完整原文。
- Modifiers 位标志的完整位定义（仅知 disable_snapping ∈ byte1）。
