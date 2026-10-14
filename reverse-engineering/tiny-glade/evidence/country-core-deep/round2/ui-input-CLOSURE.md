# ui-input 域 U 项闭合报告（round2，tg_a5，2026-10-14）

6 个 U 项逐项闭合。方法：capstone 线性反汇编（本机已装）、rne-read 字符串/字节、PDB 注解
（skeleton/*.rs 依赖注解）、与 camera-time DIGEST 交叉。

## 1. check_if_can_switch 尾段分发（byte@+1 >= 2 之后）——✅ 闭合

- 分发器 FUN_140afa8a0 完整反汇编（0x140afa8a0..0x140afa9e8，capstone）：
  - `force(cl!=0)`：push 0x20B 记录 `{u32 4@0, 计数@0x18}` → 返回。
  - `非 force`：① 闭包环境[0]（mode_manager param_1 = ResMut<模式状态>）tick 回写后
    **`mov dword [*env0], 0xd`**（pending = Replay）；② push 0x10B `{计数, u32 *env[2]（请求 mode）}`
    入环境[1] 的队列；③ 读 `byte [env[2]+1]`：
    - `< 2`：环境[3] 队列 push `{计数, byte}` 后返回；
    - **`>= 2`：`lea rcx, 0x142ae5650; mov edx, 0x28(行 40); lea r8, 0x142ae5658; call panic_str`**
      —— 即 `unreachable!()`：**"internal error: entered unreachable code"**，文件 =
      `crates/country-core/src/resources/toolbar.rs`（0x142ae5678 胖指针，line 40）。
- **尾段不是分发**，是不可达 panic。真正的按模式分发在 mode_manager 函数体内：
  - jumptable（0x142ad890c，模式 id 0..64 → 函数体 0x140a86cec..0x140a874e8 内各块）；
  - capstone 精确追踪（消除中指令假跳转后）得到干净映射：**模式 id = 10 + 5×k**（k = 工具栏
    类别视觉序 {Buildings:0, Walls:1, Decor:2, Misc:3, Paths:4, Water:5, Animals:6, Trees:7,
    Meadow:8, Terrain:9}）：
    | mode | 闭包 | 工具栏类（Vec 序 idx） |
    |---|---|---|
    | 10 | env_5 | Buildings (idx0) |
    | 15 | env_6 | Walls (idx1) |
    | 20 | env_7 | Decor (idx2) |
    | 25 | env_8 | Misc (idx3) |
    | 30 | env_9 | Paths (idx4) |
    | 35 | env_10 | Water (idx6) |
    | 40 | env_11 | Animals (idx9) |
    | 45 | env_12 | Trees (idx5) |
    | 50 | env_13 | Meadow (idx8) |
    | 55 | env_14 | Terrain (idx10) |
  - 注：mode↔类别按 **视觉序** 而非 Toolbar::new 的 Vec 序（Trees/Water、Meadow/Animals 两对互换），
    说明 mode id 由另一张"标签页顺序"表生成。idx7（cat 0，无模式类别）无对应 mode id。
  - mode 0/5 = 同形块（`cmp [state+0x30], 0 → jno` 分流）；mode 1/2/6/7/9 的跳表项落在相邻
    指令中部（跳入共享块中段——switch 优化）；mode 5 与 8 额外调 env_3/env_4（回到 cat0 类）。
- **AppMode 判别空间结论**：2=Timelapse；6..9=Wall 形状族；10+5k=Tool(类别 k)；0xd=Replay；
  0/1/3/4/5 = None/其它（0、5 同形，语义名仍 U）。check_if_can_switch 的 env_0 = timelapse
  元命令入队（meta_commander.rs），env_3/env_4 = cat0 复位路径。

## 2. EscMenuState 变体名与结构——✅ 结构闭合，变体名 ⛔ 卡点

- 结构（O，机器码）：EscMenuState = **8 B `{disc u32@0, sub u32@4}`**（enum2$，PDB 注解）。
  - `is_open` 0x14093fc00：`qword ∈ {3,4}`（单位变体）|| `(disc==7 && sub==1)`。
  - `allow_time_passage` 0x14093fc60：`!(disc==7 && sub==1)`。
- 消费者链（O，PDB 注解）：`tiny_glade::gui::gui_escape_menu::do_gui_escape_menu`（调用
  `EscMenuState::is_closed`）、`toolbar::menu::{new_session, report_bug, credits}` 族、
  `country_core::systems::audio::menu_audio::{MenuAudioState, PlayMenuUiSound}`、
  `systems::render_loop::menu_background::MenuBackgroundResources`、`systems::steam`
  （SteamOverlayOpenResources）、save_game（ResMut<EscMenuState>）。
- 同文件资源（O，PDB 注解，重建代码缺失）：**NextEscMenuState**（写入侧，ResMut）、
  **PrevEscMenuState**（Res）、**PhotomodeScreen**（Res）。
- 变体名 ⛔：EscMenuState 无 serde 过程（1161 清单无 serialize/deserialize）；lang-lib 本地化
  不覆盖枚举名；skeleton 无 tiny_glade 源码（仅 PDB 注解）；二进制中 "Closed" 单例均为
  {fn_ptr, name} 查找表（非变体表）。变体名需 tiny_glade/toolbar crate 源码或枚举 Debug 输出
  才能定——**卡点：toolbar 与 tiny_glade 两个 crate 不在 skeleton**。判别语义图已足：
  {2,2}/{3,2}（mouse_capture 推）、{3,0}/{4,0} = 开、{7,1} = 时间冻结。

## 3. 0x200000002/3 事件类型名——✅ 闭合

- mouse_capture 0x140b17830 的两次 push 确认（补读尾部）：
  - 0x18B 记录 `{id, u32 0/1, u64 窗口}` = **WindowFocusEvent `{focused u32, window u64}`**
    （捕获→focused=0，释放→focused=1）。
  - 0x10B 记录 `{u64 0x200000002/3, 计数}` = **EscMenuState 值**：0x200000002 = {disc 2, sub 2}
    （未捕获时推），0x200000003 = {disc 3, sub 2}（捕获时推）。配合 is_open 的 {3,4} 判定：
    捕获（光标锁回）→ disc 3 状态，未捕获（OS 光标可见）→ disc 2 状态。
- PDB 注解交叉证实：mouse_capture 系统参数含 ResMut<NextEscMenuState>、Res<EscMenuState>、
  `country_core::utils::get_last_event<CursorCaptureCmd>`（"取最后一条命令"实现名）。
- 布局注记（fork 问题）：该 0x10B push 是 `{载荷@0, id@8}`，而 translate_input_to_cursor_moved
  的 0x10B push 是 `{id@0, 载荷@8}`——两个形状并存，与官方 bevy_ecs 0.16.0
  `EventInstance{event_id@0, event@8}` 不完全一致；mouse_capture 的队列疑为自定义
  `{event, id}` 记录队列（grow Location 仍是 bevy event/collections.rs）。此 fork 细节
  供中央裁决，不影响语义结论。

## 4. photo_mode 0..3 ↔ CameraMode 槽 6..12 槽位分配——✅ 结构闭合，判别映射 ⛔ 部分卡点

- camera-time DIGEST 交叉 + 本域补证：
  - CameraMode 字节 @+0x4d0；13 槽 fov 表 @0x142aabb98（f64 编码的偏移）：
    {0x8c, 0x144, 0x1c4, 0x264, 0x31c, 0x3a4, **8, 0x20, 0x14, 0x2c, 8, 0x14, 0xc**}——
    槽 6..12 的 fov 偏移 = {8, 0x20, 0x14, 0x2c, 8, 0x14, 0xc}。
  - paused_main_camera_update 0x1409145a0 的 switch（O，capstone 补全 case 6 的 6 个 handler）：
    - case 0..5 = 6 个 body 槽（0x8c/0x144/0x1c4/0x264/0x31c/0x3a4，body @+0x100/0x1b8/0x270/
      0x2f0/0x390/0x448）。
    - **case 6 = 子跳表 @0x142aabdd8（6 项 handler 0x1409149f3/4a51/4a19/4a35/4a04/4a66）**：
      每项 = `{body ptr = rdx+{0,0xb8,0x170,0x1f0,0x290,0x348}, 常量(3 项有), fov 对
      {0x70/0x8c, 0x128/0x144, 0x1a8/0x1c4, 0x248/0x264, 0x300/0x31c, 0x388/0x3a4}}`——
      **6 个拍照姿态，每个复用 6 body 之一 + 自己的 zoom/fov 对**。
    - case 7 = body3 + 常量 0x42ffddbd（f32 ≈127.99）；case 8 = 二分（body1 | body4，常量
      0x42aac050 ≈85.4）；case 9 = body2 + 0x42ffde99；case 10 = body0；case 11 = halt（截断）。
  - 写入侧：camera-time 的 0x14090ec70/0x140911520 只写 `+0x4d0 = 0`（复位 Normal）；
    写 6..12 的系统在 **toolbar::photomode**（PDB 注解提及 toolbar::photomode::lut_gallery 等，
    toolbar crate 不在 skeleton）——⛔ 最终判别映射（PhotoModeCameraMode 0..3 具体对应
    case 6 子索引 0..5 的哪 4 个、还是对应槽 7..10）需 toolbar crate 或逆向该 UI 系统才能定。
  - 本域可交付的硬事实：PhotoModeCameraMode = u8 {0..3}（next 0→1→2→3→0；next_demo 3→panic
    "this camera mode is not part of the demo!"）；PhotomodeState::on_enable_photomode 写
    f32@0=3.0（demo 1.0）+ u32@4=-1；PhotomodeScreen 资源存在（gui.rs）。

## 5. EscMenuState 消费侧补充（顺带闭合）

- `do_gui_escape_menu` 只用了 `is_closed`（PDB 注解）——esc 菜单 UI 按 Closed/非 Closed 分支。
- mouse_capture 语义终版：Cmd==false（未捕获）→ CursorCapturedPosition={1,pos}、
  WindowFocusEvent{focused=0}、EscMenuState {2,2}；Cmd==true（捕获）→ {0}、{focused=1}、
  EscMenuState {3,2}。

## 6. 剩余卡点清单

1. EscMenuState 各判别（0..7+）的变体名 —— 需要 toolbar/tiny_glade crate（不在 skeleton）。
2. PhotoModeCameraMode 0..3 → CameraMode 6 子索引/槽 7..12 的精确写入点 —— 同上（toolbar::
   photomode 系统）。
3. Events<T> fork 布局中 {event@0,id@8} 与 {id@0,event@8} 并存 —— 需中央在 engine 层裁决
   （或其中一个为自定义队列，如 UiSignifierStream 家族）。
