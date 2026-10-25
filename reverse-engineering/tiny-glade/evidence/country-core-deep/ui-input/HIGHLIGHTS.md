# ui-input 域 HIGHLIGHTS（≤10 条，供中央汇总）

1. **mode_manager 早退条件解出**：mode_manager @0x140a86b60 每帧哈希 `"esc_menu_canvas"`（0x142ad8501），
   当 UiWidgetContext `hover!=0 && widget_id == esc_menu_canvas && 交点载荷 tag != 2` 时整机早退——
   ESC 菜单画布悬停即阻塞模式切换（代码/合同均未记录此规则）。

2. **check_if_can_switch 写入 0xd = Replay 判别**（任务点「与 history 的 Replay 判别联动」证实）：
   分发器 0x140afa8a0 非 force 路径先把模式状态首 u32 置 **0xd**，再 push `{事件id, 请求mode}` 记录；
   15 个 closure_env_N 闭包 = 按工具栏类别 N 的「类别切换 + 模式索引推进」处理器（env_3=Walls、env_10=
   Buildings、env_11=Paths、env_13=Roofs、env_14=Water、env_0=Timelapse 元命令入队）。force 路径 push
   `{tag=4, 计数}` 0x20B 记录。重建代码的 AppMode 枚举缺 Replay 变体。

3. **Toolbar 布局全套纠正**：Toolbar = `{categories: Vec (cap,ptr,len), current: u8@0x18}`；
   ToolbarCategoryModes = 0x20B `{modes: Vec, current: u64@0x18}`；ModeEntry = 0x10B
   `{cat u32@0, mode u8@4, pad, arg u64@8}`。`Toolbar::new` 真实地址 **0x140af9000**（重建代码写
   0xaf8c50）；`switch_mode(&mut Toolbar, &record)` 是 16B 记录入参（非 (u32,u8)）；`contains` 的
   装饰类模式区间为 **0x38..=0x3f**（代码写 0x38..=0x3d）；panic 串非代码所写原文。

4. **bevy Events 布局权威判定**：二进制内嵌路径 = crates.io 官方 `bevy_ecs-0.16.0`，EventInstance =
   `{event_id: {id u64, caller(ZST)}, event@8}`（本地 registry 源码 base.rs 核对）。所有输入/模式事件
   push 点一致。两个例外（check_if_can_switch force-push、send_mode_just_changed）疑为自定义记录队列，
   待中央复核。

5. **send_mode_just_changed_event 事件载荷 = {prev: AppMode, current: AppMode} + id**（0x28B），
   不是重建代码的 EventWriter<RequestAppModeChange>；且 PDB 名实为 `mode_manager::closure$0`。
   `AppMode::is_tool_selected @0x140a8a5d0` 是伪声明——该地址 = `timelapse_replay::cleanup_timelapse`。

6. **ui_edit_focus 结构**：widget 名 `"edit_focus_canvas"`（volume tag=5、size f32::MAX）；模式 1/2 走
   跟随路径（FUN_140a1c930 谓词）；遍历 0x2a8 反向步长 wall 数组，坏曲线先 panic
   `"assertion failed: w.simplified_curve_2d.is_valid()"` 再 panic
   `"internal error: entered unreachable code: ui_edit_focus shouldn't include ..."`；结尾 push
   `{tag=0, 计数}` 0x18B 事件。EditSubject serde 变体名全套解出（WallId/Floor/Roof/flat/gable/
   DecoratorCollision/StairInteraction/Fence/Arches/Tree/Terrain/AutoClutter 等）。

7. **UiSettings 真布局 + ToolbarAlignment 纠正**：`{icon_size_mult f32@0, custom_cursor_size f32@4,
   toolbar_alignment u8@8, auto_hide_toolbar u8@9, input_lag_reduction u8@10, game_logo u8@11,
   custom_cursor u8@12}`；变体名 = **Bottom/Left/Right/Top**（重建代码写 Horizontal/Vertical/Bottom/Side，
   错误）；默认 auto_hide=true、alignment=变体 1。

8. **photo_mode / gui / onboarding / mouse_capture 面板状态**：PhotoModeCameraMode u8 {0..3}，next_demo
   对 3 panic（串已取）；PhotomodeState::on_enable_photomode f32@0=3.0（demo 1.0）+ u32@4=-1；
   EscMenuState 判别含 {3,4,7} 且 `allow_time_passage = !(state==7 && flag@8==1)`；WelcomeScreenState
   Closed=判别 2（与 UiFocus::is_mei==2 同址折叠）；mouse_capture 按 CursorCaptureCmd 末事件写
   CursorCapturedPosition 并推 WindowFocusEvent + 0x200000002/3 事件。

9. **input 回放状态机复核 + 新串**：input_record_replay case 0/2/3/4 与 wp20 合同一致；InputFrame
   @+0x30..0xc0 各字段映射解出（@+0x40 双 f32 delta、@+0x44 除数、十 qword → InputTranslationState）；
   panic "Out of bounds access"；trailers UI 面板串全套（"`O` - start/stop recording"、tmp/input_replay
   json 保存路径）。

10. **工具链 bug 上报**：`run-decomp-batch.py --chunks 2` 分片时 decompile 步骤沿用 `tg_a5` 而数据库开在
    `tg_a5_c0/c1` → 首轮 157 个函数全部 NotFound（202B 错误文本）。单进程重跑恢复。建议修 runner
    或禁用 --chunks。另：`Modifiers::disable_snapping` = `byte[rcx+1]`（位标志组，非 4 bool 字段）；
    `ToolbarCategoryModes::next_mode` PDB 地址是 serde unknown_variant thunk（名错位）。
