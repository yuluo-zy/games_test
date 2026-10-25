# audio-misc 域 round-2 U 项闭合报告

方法：rne-read/capstone 直读机器码 + 定点 MCP（tg_a8，`tools/round2-job.json`）+ history 域交叉证据。
状态：**CLOSED**=已闭合；**PARTIAL**=结构闭合、细节卡点注明。

## 1. MusicPlayer::update 双跳表 case 语义 —— CLOSED

表基址：A=0x142a9fa24、B=0x142a9fa34（各 **4 个有效条目** = 状态 1..4；表后 4 个 dword 为邻接
.data 非 case —— A[4..7]=B[0..3]-0x10 的巧合布局，B[4..7] 落入 wall_audio_ui 代码区，证实无效）。
机器码（capstone 逐 case 反汇编）O：

- 每 case 先 `time_acc@+0x11c += dt`（xmm1 累加）。
- 分派条件：本帧曲目位置 < last_position@+0x128（曲目回卷）→ 表 B，否则表 A。
- **A1**（0x1408b56ef）：acc > 阈值1 且主事件使能[+0x40]==1 → 主事件 stop(立即) → 公共尾。
- **A2**（0x1408b597d）：读 Duration@+0x120（i64；负值走 round-to-even 路径 0x1408b5a9f：
  `(v>>1)|(v&1)` 后 ×2）→ acc 超过 → 0x1408b5b52（cvttss2si 夹取→推进/切换路径）。
- **A3**（0x1408b59b0）：**节拍同步**：全局 0x143634c68==1 且 0x143634c90&3==0 → state=4 +
  主事件 start。
- **A4**（0x1408b5826）：acc > 阈值2 → 主事件 stop(ALLOWFADEOUT)；acc > 阈值3 → **state=0**
  （下一帧 switch_track）。
- **B1**（0x1408b55d2）/B2（0x1408b5a75）：回卷路径对应处理（acc 累计后并入 A4/A1 续块）。
- **B3**（0x1408b5975）：无条件跳 0x1408b59be（0x143634c68==1 → state=4+start；随后 stop 事件@+0x48）。
- **B4 = A4**（同块）。
- 公共尾 0x1408b5bee：取当前曲目（stride 0x30；[+0]==1 使能）→ get_paused(track+8) → 已暂停则
  恢复/推进；track[0]&1==0 → 0x1408b5c72 停/切。
- state==0 → switch_track() + 日志（verbosity>4，"snapshot:/Track Switching" 串区）。
- 表初始化代码：无需找——两表为镜像静态 .data（编译时常量），无运行时写入者（xref 无写侧）。

残留 U：A1/A2/B1/B2 的阈值常量语义（0x142ad01cc 区浮点）、0x1408b5b52 之后的精确动作
（switch_track vs 位置写入）——不影响状态机结构闭合。

## 2. update_steam_inputs 逐动作映射 —— PARTIAL（结构 CLOSED，配对卡点）

归纳（decomp 7947 行，14 个同构块，每块 ~223 行）O：
- 每块 = 一个 Steam 动作的帧处理：读动作状态（FUN_1409e86b0 + func_0x0001409e8aa0 族，
  数字位/模拟值）→ 与前帧比对 → 有沿变化时更新游戏输入资源计数（**+0x170 减、+0x178 增**，
  即按住/释放簿记）→ 把 **u16 游戏输入码**压入游戏输入环形队列（元素 2 字节，容量检查
  FUN_140b5fff0）。
- 14 个输入码（代码顺序）：`0x400, 0x500, 0x600, 0x700, 0x900, 0x800, 0xa00, 0xb00, 0xc00,
  0xe00, 0xd00, 0x200, 0x300, 0xf00`（每块尾 `*(u16*)(q+l*2) = 0xN00` O）。
- 帧计数全局（FUN_140d4d740/0x140d64e00）+ PrevSteamInputState 帧差同步 + 块间 31 处
  FUN_140b5fff0（部分为 prev 状态同步路径）。
- 代码顺序非单调 → 输入码 = 游戏 InputEvent 枚举值（0x200..0xf00 为 Steam 来源段），
  **码→动作名配对需 ui-input 域的游戏输入枚举定义**（跨域依赖），本域不再臆造。
  动作候选表（0x1417c3720 表序 O）：CameraPan/cursor_move/camera_height/left_click/right_click/
  undo/redo/tool_building/tool_path/tool_decorator/tool_customize/tool_eraser/tool_terrain/
  tool_garden/tool_tree/tool_water/time_of_day/photomode/toggle_snapping/zoom_in/zoom_out/
  main_menu/ui_cancel/ui_confirm/ui_up/ui_left/ui_right/ui_down。

## 3. 0x14362c420 语义 —— CLOSED（与 history 域交叉确认）

- history 域证据（batch-report 0x140a86980 finish_potential_mutation 同款模式）：
  `if ((*0x14362c420 & 0x7fffffffffffffff) != 0) goto 早退路径`。
- 我域引用点：set_parameter（0x140a53db0）与 set_position（0x140a57da0）的
  `(0x14362c420 & 0x7fffffffffffffff) != 0 && FUN_1428d86b0()==0` → 写报告缓冲第 2 字节=1。
- 结论：**0x14362c420 = panic 进行中标志（panic_count 镜像）**，非 0 = 已处于 panic/报告期。
  我域语义：panic 期间跳过“已消费”标记（防双重写入/重入）。与 history 域用法一致，无需另立语义。

## 4. FeedbackReportResources 0x18..0x36 字段 —— CLOSED（结构级）

texture_receive（0x140909ab0）写序 + is_available（0x140909860）+ clear_screenshot（0x140909930）O：
- +0x0：`Arc<Mutex<Option<Vec<u8>>>>` 截图字节（clear_screenshot 锁清）。
- +0x8：`Arc<Mutex<Option<…0x38B 纹理结构>>>`（内部：+0x10 锁标志字节、+0x18..+0x44 元数据/
  句柄、+0x48 ptr、+0x4e 状态字节=7）。
- **+0x10..+0x38：截图元数据区**（6 qword）：事件分支写 param_1[2..7]（+0x10/+0x18/+0x20/
  +0x28/+0x30 qword、+0x34 u32）；**+0x36 字节 = 背景纹理可用性枚举（<7 即可用，7=不可用）**
  —— is_available 即读此字节；FeedbackReportBgTexture 为该字段的访问视图（I）。
- +0x3e：状态字节（7=空、8=收到截图；clear_screenshot 按 min(state-7,2) 分派清理）。
- +0x40..+0x46：选择字节 {0,1,1,0,0,0}（reset_choices O）。
- 第二分支（state==8）：锁 +8 Mutex → 更新纹理内部结构（FUN_140c81e10）→ 拷
  {+0x18/+0x20/+0x28/+0x30/+0x40/+0x44 区} 到资源 +0x10..+0x38 → [纹理内部+0x4e]=7、
  资源 +0x3e=7 → 换 Arc。
残留 U：元数据 6 qword 各字段含义（宽/高/格式/句柄，语义未钉）。

## 5. 0x140949c40（PDB aspect_ratio 实为析构例程）宿主类型 —— CLOSED（结构级）

- get_xrefs_to（tg_a8）O：**唯一引用 = 0x14368E2FC，ref_type=DATA，非 call** —— 无任何代码调用者。
- 结论：0x140949c40 是**进程级静态变量的析构注册**（0x14368e2fc 位于 .data 静态析构注册表，
  Rust 静态 Drop 的 dtor 表项）；函数体（cap-first Vec<stride 0x40 元素> 逐元素双指针释放 +
  i64::MIN 哨兵）与析构语义吻合。宿主 = window.rs 域内某 lazy 静态（疑窗口/显示器缓存，
  具体 Rust 符号名 U）；PDB 名 `WindowSize::aspect_ratio` 确认错位，真实 aspect_ratio 体 U
  （0x140949c00 无函数、0x140949cc0 为另一含 f32 累加 + Vec<27B 元素> 的函数，疑 monitors 逻辑）。

## 附：本 round 顺带新证

- wall_audio_ui 事件串补全（数据区 0x142a9f730 邻域）：`event:/SFX/ChangeWallStyle`、
  `event:/SFX/Lantern/TurnOn`、`event:/SFX/Chimney/ToggleSmokeOn|Off`（按 [+9] 字节二选一）、
  `event:/SFX/BuildingSupportsRemoveWooden`、`event:/SFX/WallSplit` 等（kind 分派 case 块 O）。
- GradingLuts::load 的 per-file 闭包（0x140ae5bb0）：DirEntry 空 → panic `"read_dir entry"`
  （0x142ae3218）；"assets/luts/" + 文件名拼接 → 按扩展名分派 load_cube/load_unwrapped（结构 O）。
- load_unwrapped（0x140b00bc0）：路径前缀 `"assets/luts/"`（0x142ae6c08）O 名。
- roof_audio::instantiate_audio_w_position 真实签名 4 参（core, name&str, pos）（调用点
  0x1408b7788：rcx=rsi(核心), r8=串长, r9=&{name ptr,len}）——现有 2 参模型错。
