# ui-input 域 算法级深挖（2026-10-15）

> 覆盖 42 章：`systems/ui/**`（12）、`mode_manager`+`mod_key`（2）、`systems/backend/input/**`（7）、
> `ui_*` 顶层系统 + `ui_clutter/**`（13）、`resources/{toolbar,photo_mode,mouse_capture,onboarding,gui,ui_settings}`（6）、
> `ui_edit_freehand_walls`+`ui_move_freehand_walls`（2）。
> 证据 = `ui-input/decomp/*.txt` + round2 CLOSURE capstone + rne-read 串/表 + round3 复查修正。
> 标注：O=机器码证实 / I=推断 / U=未知。

---

## 1. 处理链图

### 1.1 模式切换链（mode_manager，最高优先）

```
[UI 请求] Toolbar 点击/快捷键/Steam 输入
    │  请求 mode id（10+5k，k=工具栏视觉序 0..9）
    ▼
Events<RequestAppModeChange> 双 VecDeque {cap,ptr,len,head}@+0x10/+0x30 + 计数@+0x40
    │  mode_manager 每帧排空：
    │  ① esc_menu_canvas 悬停早退（hash('esc_menu_canvas'@0x142ad8501)==widget_id
    │     且 hover@+0x18!=0 且交点载荷@+0x28!=2(None) 且 mode@+0x28!=2）
    │  ② 取最后一条请求 → check_if_can_switch(force=queue_drain_diff==0, pending, closure_env)
    │     ├─ force=true  → push {u32 tag=4, 计数@+0x18} 0x20B 记录（立即执行路径）
    │     └─ force=false → ① *模式状态首u32 = 0xd(Replay)【pending 置 Replay，历史回放联动】
    │                      ② push {id, 请求mode u32} 0x10B 记录
    │                      ③ env[2]+1(byte) < 2 → push {id, byte} 后返回
    │                         env[2]+1(byte) >= 2 → unreachable!() panic（toolbar.rs:40）
    ▼
状态@+0x80 的 u32 模式 id → jumptable 0x142ad890c（0..64 项，全部落在函数体
0x140a86cec..0x140a874e8 各块）：mode=10+5k → 直调 check_if_can_switch<closure_env_5..14>
    ▼
send_mode_just_changed_event：prev/current AppMode(16B×2) ne → push 0x28B {prev,current,id}
```

### 1.2 输入录制/回放链（backend/input）

```
[录制]  Idle(0) 且 (请求事件 或 UI 开关 或 工具/画布字节@+0xa3 bit0)
    → MainCamera::store_parameters(FUN_140911710) + clone ToolbarCategoryModes
    → 重建 InputStream（frames 容量 8 + 1 sync point）+ add_frame(当前游标 delta)
[回放]  Replaying(2)：control_input_replay 键控 seek；每帧 advance()：index@+0x48++，
    帧尽 → mode = MIN+4(Finished)
    帧写入：InputFrame 0xd0B 解码（见 §3.1）→ 恢复 InputTranslationState + 相机 + 工具栏
[Steam/拖车] record_and_replay_for_trailers：配置字节@+0xfc==1 + 键命中 →
    Idle/Finished→"Start recording"；Recording→"Finished recording"；Replaying→"Replay..."
```

### 1.3 ui_edit_focus 分发链

```
mode = *param_1（GeoSource 判别 u32）
├─ mode<=5 → (6>>mode)&1：mode∈{1,2}=跟随路径（过 FUN_140a1c930 谓词）；
│     否则遍历 wall 数组（0x2a8 反向步长 SIMD lane 匹配，entity 字段<2 或 f32<=0 →
│     panic 'assertion failed: w.simplified_curve_2d.is_valid()'(public_walls.rs:50)
│     + unreachable 'ui_edit_focus shouldn't include ...'）
│     → 推 0x18B 事件 {tag0, 计数}
│   mode==10 → 跳表 A（UNK_142ab73a0，idx=*state@+8）
│   6..9     → 跳表 B（UNK_142ab73b4）
│   长度==0  → 跳表 C（UNK_142ab738c，idx=*state@+0x90）
│   mode<6 分支（实际 = widget 注册后）→ 跳表 D（UNK_142ab73c8）
└─ 4 表 = 同一张 24 项表首部旋转 0/5/10/15（各表 4B int32 偏移，基址 0x142ab738c
     + 0xfdec0000 → 目标 0x14097745b..0x140977e34，全部在函数体内）
    —— GeoSource 24 变体 match；变体 serde 名见 §4.2
```

---

## 2. 逐函数算法

### 2.1 mode_manager（O 全量）

- **mode_manager @0x140a86b60**：见 1.1。事件排空用 bevy Events 双 VecDeque 语义
  （counter - head - len；bevy_ecs-0.16.0 官方 registry 路径 0x142c3a148）。
- **check_if_can_switch<closure_env_0> @0x140af9e00**（timelapse）：非 force 时
  (a) 闭包环境[1]（Res<AppMode>）的 2×qword AppMode 写入环境[0]（ResMut）＝pending=当前；
  (b) 从相机资源读 f32@+0x60/u32@+0x68/u32@+0x6c 组 40B MetaCommand 记录
  `{i64::MIN, f32, u32, u32, 0, byte=1, ..., MIN+1}` → FUN_140b59be0（meta_commander 入队）；
  (c) `time_scale = f32 * 全局乘数`（_UNK_142925988）→ push {u32=1, f32} 事件；
  (d) 置环境[6] ResMut 字节=0。
- **15 个 closure_env_N 类别推进器**（env_3..14，每 ~2.3KB 两段式，O）：
  - force 段：push `{u32 4, 计数}` 0x20B → 返回。
  - 类别段（以 env_10=Water idx6 为例 @0x140afa190）：
    `Toolbar.categories.len(+0x10) < N → 返回`；`categories[cat].modes.len(+0xd0 区) == 0 → 返回`；
    `Toolbar.current(+0x18) == N` → `idx = (current+1) % len`（32/64 位双溢出路径，`(uVar7|uVar12)>>0x20==0` 判 32 位）；
    否则 `current >= len → FUN_1428d9518()`（bounds panic，文件 toolbar.rs）→ idx=current；
    写回 `categories[cat].current(+0xd8)`；`Toolbar.current = N(+0x18)`；
    取 `modes[idx]`（0x10B {cat u32@0, arg u64@4, mode u32@0xc}），`cat != 0x15`（空哨兵）
    → 0x10B 拷入 pending 槽 `{cat@0, arg@8, mode@0xc}`。
  - 闭包↔类别映射（O）：env_3=cat7(idx7 空类, switch_mode 直调)；env_6=idx1(Walls byte==1)；
    env_7=idx2(Decor)；env_8=idx3(Misc)；env_9=idx4(Paths)；env_12=idx5(Trees, entry@0xb0)；
    env_10=idx6(Water)；env_13=idx8(Meadow, entry@0x110)；env_11=idx9(Animals, entry@0x130)；
    env_14=idx10(Terrain, entry@0x150，`request.cat==0x13` 时清零 cat 直调 switch_mode)；
    env_5=idx0(Buildings, current!=0 变体, entry@0x10)。env_4=动态类别变体（类别取自参数字节）。
- **send_mode_just_changed_event @0x140a875e0**（O）：`func_0x000140a825e0(prev,current)`
  （=PartialEq::ne 2 qword）→ ne 则 Events push 0x28B `{prev qword×2, current qword×2, id}`，
  `current := prev`。**载荷是双 AppMode 对**（非 RequestAppModeChange）。
- **start/finish/cleanup_timelapse**（wp20 复核 O）：start=排空→check_if_can_switch<env_0>；
  finish=排空→MetaCommander::extend(1)+标志字节=1，窗口焦点@+0x18==0→push WindowFocusEvent{0}；
  cleanup=排空→log + `*(byte*)*state = 2`（回 Timelapse）。

### 2.2 mod_key（O 手解）

- `Modifiers::disable_snapping @0x140af9bf0`：`0f b6 41 01 c3` = 返回 **byte@+1**。
  **Modifiers 是位标志字节组**（disable_snapping ∈ 第 1 字节某位），非 4 bool 字段。
- `is_disable_snapping_just_pressed @0x140af8f70`：ICF 折叠单行（判断刚从松开→按下沿）。

### 2.3 输入录制/回放状态机（backend/input，7 章全 O）

- **状态编码**：mode u64@+0x40，`^0x8000000000000000` 后 0=Idle/2=Replaying/3=Recording/4=Finished
  （存储 = MIN+变体，niche 编码）。index@+0x48。
- **input_record_replay @0x1409423d0**：
  - Idle(default 分支)：条件 = `Events 有未消费 || *param_11!=0 || 字节@+0xa3 bit0`
    → 克隆 ToolbarCategoryModes（FUN_1412f7fa0）→ 重建流 {frames cap=8, 1 sync point}
    → `add_frame`（FUN_140949190）帧 = 当前游标 delta：`delta = (cursor - 原点0x1429e95b0)`
    经 2×2 矩阵 _UNK_142a171f0 + 偏移 *param_7 变换 → `divps 归一`。
  - Replaying(case 2)：`index <= head? → panic "Out of bounds access"(0x142ab3224:20)`;
    元素基址 `frames_ptr + 0x68 + idx*0xd0`（VecDeque 回绕数学：(idx+len) clamp cap 后×0xd0）；
    帧字段 → 状态（见 §3.1）；游标 f32→u32 clamp（NaN 安全 0..u32::MAX，0x1429a07d0=最大 f32 比较）；
    `norm = (cursor - 原点) / 缩放除数@+0x44` 写入游标资源；恢复 InputTranslationState。
  - Recording(case 3)：清零 0xa0B（20 qword @+0）＝重置输入翻译状态。
  - Finished(case 4)：no-op。
- **control_input_replay @0x140941a30**（O，48B 事件元素 stride 0x30）：
  排空事件队列（bevy Events 双缓冲）；`mode>0` 时重建流（clone toolbar + store camera
  + `on_history_sync` FUN_1409494c0）；verbosity>2 log "Starting input replay"(0x142ab2650)；
  `try_load`（FUN_1409499c0）；`sync_points.len==0 → panic "No sync points found in replay state"`
  (0x142ab2740)；`mode != MIN+2(Replaying) → panic "Can only seek to sync point if replaying but
  mode is "(0x142ab3658)`；seek：`index@+0x48 = syncpoint.frame_index(-1)`（sync point stride 0x60，
  frame_index @+0x58），恢复 游标 + Toolbar 快照（clone 覆盖 categories Vec + current byte@+0x18）
  + `MainCamera::load_parameters`（FUN_140911520，数据 @sync+0x30）；不足 → panic
  "Not enough player inputs to rewind "(0x142ab27b0)。
- **advance_input_replay_frame @0x140942280**（O）：`mode==MIN+2(Replaying)` →
  `index = index+1`；`index > frames.len(+0x38)` → verbosity>2 log + `mode = MIN+4(Finished)`。
- **record_and_replay_input_for_trailers @0x14083c620**（O）：配置字节@+0xfc==1 且
  命中 HashMap<KeyCode>：Idle/Finished→log "Start recording"(0x142a93740)+store+重建流；
  Recording→"Finished recording"(0x142a93678)+mode=Finished；
  Replaying→"Replay..."(0x142a93778)+index=-1+恢复状态+camera load。
- **do_input_replay_ui @0x14083cdd0**（O 串）：egui 面板 "Enable input recording"（写@+0xfc）、
  "Stream frame count: "、`` `O` - start/stop recording ``、保存路径 "tmp/input_replay"+"json"。
- **translate_input_to_cursor_moved @0x140b17a80**（O）：遍历录制事件（stride 0x18，
  tag@+8：0=mouse_moved 相对/1=moved 绝对/2=跳过）；tag0：`acc = clamp(acc + 事件delta,
  [0, cursor-原点])`（maxps(0,x) 下界 + minps 上界，NaN 掩码安全），`sum += delta`；
  tag1：`sum += (事件值 - acc)`；事件尽 → push InputCursorMoved（0x10B {id, Vec2=sum}）
  + `状态@+0x58 += sum`；`状态@+0x60 += (最后事件游标 - 当前游标)`。
- **translate_input @0x1417cd0a0**（O，wp20 复核）：逐 Steam/键盘动作（~50+ 处 FUN_14089aef0）
  analog→ActionSetValues、digital→ActionSetStates、MouseMoved 消费；谓词族
  FUN_14089a890/FUN_14089ab50/FUN_14089a9f0。
- **calculate_input_delta @0x1416feeb0**（O，wp20 复核）：kind2 → `value+0.1`、`≤0.05 夹取 0.05`、
  `delta = value - prev`；kind1 → `value + prev`；kind0 → 输出 prev（delta 0/状态 3）；
  三通道 +0x30/0x31/0x32；拷贝区 +0x58..+0x80（6 qword）。
- **reset_input_state @0x140942b00**：清 11 qword + 按最后 sync point 修剪 +
  `ButtonInput::reset_all<KeyCode>`（wp20 复核）。

### 2.4 systems/ui/**（12 章）

- **UiSignifierStream = 0x170B 无锁环形流**（round2 REVIEW，O 结构）：read cursor@+0x160、
  write counter@+0x168、0x28B 槽（UiAppendSignifier 紧凑记录，非 SignifierEntityBuilder 0x90B
  的包装）；write 自旋追加，signifier_create 排空生成实体。
- **SignifierEntityBuilder 0x90B**：shader String@+0x60..0x70、position Vec2@+0x7c、
  默认 {0, 1.0f64, 0x100 u16@+0x40, 0.2f32@+0x78}；`rotation @0x14089dc30` 写 16B f32 +
  memcpy 0x90；SignifierFactory 40B；build 三分支（0=命名网格/1=transient/2=mesh_from_pts）。

- **ui_state @0x140922fb0（add）/ @0x14092e3e0（add_with_id）**（O）：
  `LOCK; [p+0x60]==0 → 写 8` 取写锁（超时 1e9 ns）；拷贝 0xb0B WidgetParams；
  alloc 0xc0B Arc；`HashMap<WidgetId, Arc<WidgetParams>>` insert（hashbrown）。
- **UiWidgetContext::dragged / slider3d::into_drag_state @0x14092df10**（O，round3 w6 修正：
  非折叠，真实 26B）：`mov rdx,[rcx]; mov al,2; cmp byte[rdx+0x18],2; jb ret;
  mov r8,[rdx+0x38]; cmp r8,[rcx+8]; jne ret; movzx eax,byte[rdx+0x19]; ret` ——
  默认返回 2；`[rdx+0x18]>=2 且 [rdx+0x38]==[rcx+8]` 时返回 byte@+0x19。
  PDB size 26 与机器码一致（同址含 dragged 与 IntoHandleDragState 两符号）。
- **UiSounds::on_drag_release @0x1409a90c0**（O，round3 w6 修正：非折叠，真实 70B）：
  UiSounds 结构复制/重排——槽 0..0x50 复制，`[dst+0x30]=[src+0]`（重复）、
  `[dst+0x40]=[r8+0x10]`（替换）、0x48/0x50 直拷。
- **clearing_canvas_w_layer_and_sound @0x14092df50**（O）：体积 130×130
  （`0x43020000_43020000` 双 f32），哈希 widget 名后插 Arc(0xc0)。
- **widget_params @0x1409a8e10（new）**（O）：WidgetParams 0xb0B，u32 `{0,8,0,1,0,2,3,3}`@
  `{0,8,0x10,0x18,0x20,0x30,0x48,0x60}`；sounds@0x18..0x78（4×24B UiSounds）；volume@0x78
  （32B WidgetVolumeShape）；u16@0xa8=0x101；`sounds(0x1409a8e80)`/`occlude_by_world
  (0x1409a8ee0, 字节@+0xa8 bit0)`/`no_history_recording(0x1409a8f10, 字节@+0xa9 bit0)`/
  `allow_negative_distance(0x1409a8f40, 字节@+0xa9 bit2)`；`UiFocus::as_mei/is_mei/is_toolbar`
  @0x1409a9130/40/50（u32 判别 None=0/Toolbar=1/Mei=2/World=3）。
- **ui_update @0x140b2de60 + get_ui_volume @0x140b2dc60 + hover_intersection @0x1408383a0**（O，wp15 b5 复核）。
- **signifiers/**（8，wp15 b7 复核）：`SignifierEntityBuilder::rotation @0x14089dc30` 写 16B f32
  + memcpy 0x90（round3：xrefs 仅 1 DATA/0 call，**"13 callers"未证实**）；`signifier_create`/
  `signifier_mark_for_delete`/`animation`。
- **slider3d**：`impl$0::apply/new_archetype/queue @0x14003bb80` = ICF 折叠区（round3 修正：
  该址=bevy 系统参数胶水 1B ret，非 slider3d 专属业务）。

### 2.5 ui_* 顶层系统 + ui_clutter（13 章）

- **ui_edit_focus**：见 1.3 + §4.1。widget "edit_focus_canvas"(0x142ab69e0, 0x11=17 字符)，
  WidgetParams 栈记录 {tag=5, f32 0x7f7fffff, u16 0x100...}；结尾 push 0x18B `{u32=0, 计数}`。
- **geo_src_to_edit_subject @0x140977f50 / _w_deco_clutter @0x140978000**（O，172KB 含 serde 内联）：
  GeoSource→EditSubject 映射；4+ 层嵌套跳表（0x142ab747c 首字节分派 / 0x142ab7504 /
  0x142ab7514 / 0x142ab7524 / 0x142ab7568）＝serde deserialize 的 variant match；
  变体名/字段名 24 token 见 §4.2。
- **ui_move_shapes @0x14097a060**：widget "move_shape"(0x142ab6c90)；`move_gizmo_and_trigger_vol`
  @0x14097d280 = 施加 delta + 触发 volume 重算（wp20 复核）。
- **ui_raise_shape @0x140952e70 / impl_raise_shape @0x140955650**：widget
  "outline_wall_bottom_"/"arrow_raise_shape_"/"raise_wall_elevation_"（0x142ab4548/45f8/46c0）；
  `ProportionalScreenSpaceCoord::project_onto_ray` + `History::add`（wp20 复核）。
- **ui_2d_follow_mouse @0x14092a100**（O）：puffin scope；双分支 Query（标志@+0x108），
  源索引 @+0x94（stride 0xa0）/@+0x10（stride 0x48）→ 每实体槽写游标 Vec2 + u32 layer。
- **ui_camera_mode_gizmos @0x1409527a0**（O）：模式门 `mode!=2 && !(mode==10 && byte==0)` → 返回；
  相机模式字节@+0x4d0 `(byte&6)==2`(Free)→返回；窗口 w/h ≤ 2 才画；`*(int*)*param_9 != 0x14` →
  `UiState::add`("camera_mode_active", 0x12)；工具字节@+0x11c `==6→0; >4→返回; 0x13>>v&1==0 → 返回`
  （限 {0,1,4}）；通过 → AppendOnlyVec 推 gizmo signifier "rotation_"(0x142ab4438)。
- **ui_clutter/（7 章）**：debug_spawn_clutter / decoclutter_show_pins / defered_autoclutter_history /
  rmb_autoclutter / ui_autoclutter / ui_focus_decoclutter / mod —— 结构级（DIGEST 串
  "systems/ui_clutter/rbm_autoclutter.rs" 与 ui_edit_focus 的 deco/clutter 路径联动）。

### 2.6 resources（6 章）

- **Toolbar::new**：PDB/S_PUB32 符号 @0x140af8c50（size 3058）；**函数体 0x140af9000 = 符号 +0x1b0**
  （同一函数内，非独立函数——round3 w6 修正，"不同单态/内联"说法已否）；11 类别 Vec
  （cap-first {cap,ptr,len}@0/+0x10 + current u8@0x18）；类别 id 集合 {6,7,8,0xb,0xc,9,0x10,10,0,0xf,0x11,0x13}
  （含 id=0 类别；Buildings 列表含 (0xb,0x3f)/(0x12,0) 两跨类条目，置信度中）；ModeEntry 0x10B
  {cat u32@0, mode u8@4, pad, arg u64@8}。
- **ToolbarCategoryModes::contains @0x140af89d0**（O）：SIMD 16B 块遍历 0x10 步长元素；
  `cat<=5` → `0x38<=mode<0x40`（lea eax,[mode-0x38]; cmp al,8 → **0x38..=0x3f**）；否则精确匹配。
- **switch_mode @0x140af9850**（O）：入参 `(&mut Toolbar, &{cat u32, mode u8, arg u64})` 16B 记录；
  逐类 contains → 写 Toolbar.current=类别下标；panic 串含 "mode but no toolbar"（完整原文 U）。
- **next_mode @0x140afb590**：PDB 错位 = serde unknown_variant 错误 thunk（真 next_mode U，ICF 折叠）。
- **serde visit_str @0x140afb5b0**：比较 `"Path"`（`cmp dword [rdx],'Path'`）——类别按名序列化。
- **photo_mode**：PhotoModeCameraMode u8 枚举 {0..3}（repr(u8)）；`next` 0→1→2→3→0；
  `next_demo @0x140b596b0` 0→1→2→0，**3 → panic "this camera mode is not part of the demo!"**
  (0x142af1188)；Plugin build @0x140b59640 = register_event+组件注册胶水；
  `PhotomodeState::on_enable_photomode @0x140870570`（wp15 b10）写 f32@0=3.0(非 demo)/1.0(demo)、
  u32@4=0xffffffff、byte@10=0、byte@11=0。
- **mouse_capture @0x140b17830**（O）：消费 `get_last_event<CursorCaptureCmd>`：
  `Cmd==false(未捕获)` → CursorCapturedPosition={1, 窗口坐标} + WindowFocusEvent{focused=0}(0x18B)
  + EscMenuState {disc2, sub2}(=0x200000002)；`Cmd==true(捕获)` → {0} + {focused=1} + {3,2}
  (=0x200000003)。
- **onboarding**：`WelcomeScreenState::is_closed @0x1409a9140` = `*self==2`（Closed=2；与
  UiFocus::is_mei 同址折叠——语义巧合）。
- **gui**：`EscMenuState::is_open @0x14093fc00` = 8B `{disc u32, sub u32}`：
  `qword∈{3,4} || (disc==7 && sub==1)`；`allow_time_passage @0x14093fc60` = `!(disc==7 && sub==1)`；
  同文件 NextEscMenuState(ResMut)/PrevEscMenuState(Res)/PhotomodeScreen(Res)（PDB 证实）；
  `PrevEscMenuState::get @0x14093fbb0` = `state==0xb → None 否则 Some`；
  `DebugTriggerProxyVis::as_string @0x14093ffd0` = 按 u8 索引查串表（FUN_1428d8fe3(1, 0x12/0xe/0x10)
  长度 18/14/16 + 不可恢复跳表 = match 分发）。
- **ui_settings**：serialize @0x140a46ad0 pretty serde_json 字段序
  `{icon_size_mult f32, custom_cursor_size f32, toolbar_alignment u8@8, auto_hide_toolbar u8@9,
  input_lag_reduction u8@0xa, game_logo u8@0xb, custom_cursor u8@0xc}`；ToolbarAlignment 变体
  = **Bottom/Left/Right/Top**（0x142acec66）；默认 {1.0, 24.0, 0x101, 0}。

### 2.7 ui_edit_freehand_walls / ui_move_freehand_walls（2 章）

- 属 wall 形状编辑 UI：panic 位置串 `crates/.../ui_move_freehand_walls.rs`（0x142ab6a10 区）；
  widget 名 "move_freehand_wall"（0x142ab6a28 区，串表 0x142ab6a0f.. 区间）；自由墙
  SIMD 匹配/`add_support_`/`move_shape_line`/`move_shape_outline` 等 widget 名同区
  （DIGEST 串表；算法级在 wall 域）。

---

## 3. 状态机与数据结构

### 3.1 InputFrame 0xd0B（回放帧，O 机器码）

| 偏移 | 字段 | 回放写入目标 |
|---|---|---|
| +0x30 | u64 | 输出状态@+0xe8 |
| +0x38 | u32 | @+0xf0 |
| +0x3c | u8 | @+0xf4 |
| +0x40 | 双 f32 游标 delta | 游标资源（f32→u32 clamp，NaN 安全 0..u32::MAX） |
| +0x44 | f32 缩放除数 | `norm=(cursor-原点)/除数` |
| +0x58..0xa8 | 11 qword（88B InputTranslationState 快照） | 状态@+0x58.. |
| +0xb8/+0xc0 | u64×2 尾部 | 变换 `pos' = pos + frame*scale(0x1429258e0/e4)` |

### 3.2 InputStream 64B + InputReplayState

```
{0x00..0x18 sync_points: VecDeque {cap,ptr,len,head},
 0x20..0x38 frames:      VecDeque {cap,ptr,len,head},
 0x40 mode u64（MIN+变体 0/2/3/4）, 0x48 index u64, +0x50/+0x58 保留}
```
- `InputStreamSyncPoint` stride 0x60：`+0x18` Toolbar current 字节、`+0x20/+0x28` ToolbarCategoryModes
  Vec 快照、`+0x30` 相机参数（store/load_parameters）、`+0x58` frame_index u64。

### 3.3 AppMode 判别空间（u32）

| 值 | 语义 |
|---|---|
| 0/1/3/4/5 | None/其它（0、5 同形块，语义名 U） |
| 2 | Timelapse（早退检查 `mode@+0x28 != 2`） |
| 6..9 | Wall 形状族（ui_edit_focus 跳表 B） |
| 0xd (13) | Replay（check_if_can_switch 置 pending 值） |
| 10+5k (10..55) | Tool(类别 k，视觉序 Buildings/Walls/Decor/Misc/Paths/Water/Animals/Trees/Meadow/Terrain) |

### 3.4 EscMenuState 8B `{disc u32, sub u32}`

| disc | sub | 语义 |
|---|---|---|
| 2 | 2 | 未捕获（OS 光标可见）→ mouse_capture 推 0x200000002 |
| 3 | 2 | 捕获（光标锁回）→ mouse_capture 推 0x200000003 |
| 3 / 4 | 任意 | is_open==true（单位变体） |
| 7 | 1 | 时间冻结（allow_time_passage==false） |
| 0xb | — | PrevEscMenuState::get → None 哨兵 |

---

## 4. 常量表

### 4.1 ui_edit_focus 24 项分发表（O，基址 0x142ab738c，4B int32 偏移）

目标 = `0x14097738c + low16`（表内高 2B 恒 0xfdec）：
`idx→0x140977xxx`：0→776, 1→45b, 2→766, 3→45b, 4→45b, 5→6c4, 6→98e, 7→6b7, 8→98e,
9→98e, 10→6ca, 11→91a, 12→87c, 13→91a, 14→91a, 15→6d1, 16→4fa, 17→6c4, 18→4fa, 19→4fa,
20→7aa, 21→7aa, 22→d9c, 23→e34。
旋转：A=0x142ab73a0 从 entry5 起；B=0x142ab73b4 从 entry10 起；D=0x142ab73c8 从 entry15 起。

### 4.2 GeoSource/EditSubject serde 名 24 token（0x142ab7268 区连续串，O）

`DecoratorCollision / decorator_id / addr / DecoratorInteraction / StairInteraction / ColumnStairs /
Fence / Arches / Tree / Terrain / AutoClutter / entity / WallId / wall_id / Floor / Roof / flat /
gable / StairGraphEdge / graph_edge / StairSupport / SupportedStairStructural / point_idx /
UnsupportedStairStructural`
→ 17 变体（DecoratorCollision、DecoratorInteraction{decorator_id,addr}、StairInteraction、
ColumnStairs、Fence、Arches、Tree、Terrain、AutoClutter、entity、WallId{wall_id}、Floor{flat}、
Roof{gable}、StairGraphEdge{graph_edge}、StairSupport、SupportedStairStructural{point_idx}、
UnsupportedStairStructural）+ 7 字段。
邻区变体表 vtable：StairGraphEdge@0x140201698、DecoratorId@0x140385340、WallId@0x140b30c70、
Roof@0x140671510、DecoratorCollision@0x140541098 等（serde crate 表）。

### 4.3 mode_manager 跳表 0x142ad890c（O）

0..64 项全指向函数体 0x140a86cec..0x140a874e8；干净映射（capstone 消中指令假跳转）：
mode 10/15/20/25/30/35/40/45/50/55 → env_5/env_6/env_7/env_8/env_9/env_10/env_11/env_12/
env_13/env_14（视觉序）；mode 5/8 额外调 env_3/env_4（cat0 复位）。

### 4.4 输入码表（translate_input_to_cursor_moved 事件 tag）

| tag | 语义 |
|---|---|
| 0 | mouse_moved 相对事件（delta 累加 + clamp [0, cursor-原点]） |
| 1 | moved 绝对事件（sum += (值 - acc)） |
| 2 | 跳过（录制中的噪声帧） |

### 4.5 字符串/panic 地址（O）

`esc_menu_canvas`0x142ad8501（副本 0x142ad6b01）、`edit_focus_canvas`0x142ab69e0、
`move_shape`0x142ab6c90、`outline_wall_bottom_`0x142ab4548、`arrow_raise_shape_`0x142ab45f8、
`raise_wall_elevation_`0x142ab46c0、`camera_mode_active`0x142ab4418、`rotation_`0x142ab4438、
`assertion failed: w.simplified_curve_2d.is_valid()`0x142ada193、`ui_edit_focus shouldn't include`
0x142ab6a70、`this camera mode is not part of the demo!`0x142af1188、
`Starting input replay`0x142ab2650、`Failed to load an input recording from `0x142ab26e0、
`No sync points found in replay state`0x142ab2740、`Not enough player inputs to rewind `0x142ab27b0、
`Out of bounds access`0x142ab3224、`Can only seek to sync point if replaying but mode is `0x142ab3658、
`Start recording`0x142a93740、`Finished recording`0x142a93678、`Replay...`0x142a93778、
`icon_size_mult toolbar_alignment ... Bottom Left Right Top`0x142acec66、`variant identifier`0x142ae59e4、
`Path`（inline cmp）、`... mode but no toolbar ...`0x142ae3cbc 区（原文 U）。

---

## 5. 未决（U，不臆造）

- EscMenuState 0..7 判别变体名（需 toolbar/tiny_glade crate）。
- PhotoModeCameraMode 0..3 → CameraMode 槽 6..12 的写入点（toolbar::photomode 系统，跨域 camera-time）。
- 事件队列 {event@0,id@8} 与 {id@0,event@8} 两种形状并存 —— fork 布局待中央裁决。
- Toolbar::new 的 Buildings 跨类条目 (0xb,0x3f)/(0x12,0) 真伪（Ghidra 类型混淆风险）。
- switch_mode panic 串完整原文。
- Modifiers 位标志完整位定义（仅知 disable_snapping ∈ byte1）。
- AppMode 0/1/3/4/5 的语义名。
