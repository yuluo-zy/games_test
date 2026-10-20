# 渲染尾部未闭合项取证报告（2026-10-20，只读）

方法：ReVa 端口 37229（`/tiny-glade.exe`）机器码/反编译 + 仓库已有证据（`recovery/.../decomp/0x14031af10_gather_screenshot_requests.c`、`evidence/round12/color-menu/*`）。未编译、未改源码。
地址前缀均为 `0x14…`。“O”=机器码直接观测，“I”=推断，“U”=未闭合。

关键前提：`gather_screenshot_requests`（0x14031af10，5999 B）是原版**唯一**写 `ACTIVE_SCREENSHOT_REQUESTS` 全局向量（DAT_14362b058/60/68，元素 0x98 B）的地方；xref 证实其余只有 compile（0x140162c57）/opportunity/finish/hurry 读取。它由两个 schedule 包装调用（0x1402cd31a、0x14047849d）。仓库 `screenshot_capture::install` 里自带的 `gather` 只覆盖了其中 purpose 1 与 purpose 2(Menu) 两个生产者；`gather_screenshot_requests(world, requests)` 在仓库中只被测试调用，**没有任何游戏系统调用**。

---

## 1. 截图请求生产者（purpose 0 / 3 / 4 / 6）

### 1.1 已取证

gather 的系统参数（栈参数，O）：`in_stack_28` = `Option<SteamClient/SteamScreenshotRequests>`（Arc→RwLock<Vec<Option<Arc<Slot>>>>，Vec 在 +0x20/+0x28）；`in_stack_30` = `AppMode`（首 u32）；`in_stack_38` = `GameConfig`（+0x120 = `screenshot_format`，0=Jpeg，1=Png）；`in_stack_40` = `Local<[u8;2]>`（[0]=照片模式保持状态，[1]=帧计数）；`in_stack_48` = `FrameRenderConfig`（+0x20 = `flag_20`，即 vermeer 的 skip_generate；`+0/+4` = extent_8）；`in_stack_50` = `FeedbackReportResources`（+0x3e 为 7/8 枚举）；`in_stack_58` = `LutGalleryState`；`in_stack_60` = `MetaCommander`；`in_stack_68` = 另一资源（+0x31 字节，U）；`in_stack_70` = EventReader 参数数组；`in_RDX` = `EscMenuState`；`in_R8` = 菜单背景 pending 槽。

**TypeId 校正**：AppMode 的 `0x13` = `AppMode::Photomode`（仓库 `native_tag()` 表 = 19），即 gather 与 final_post 中的 “theme==0x13” 实为 `AppMode==Photomode`。

逐生产者（地址为 gather 内）：

| purpose | 触发 | 请求内容 | 位置 |
|---|---|---|---|
| **0 GradingReference** | `DigitalActionState::just_pressed(id=*(bindings+0xf0), f32=*(+0xf4))`（0x14031b58c..b5a6，调用 0x14093fa90），每帧检查（照片模式 16 帧保持期间也检查） | `dims=None`，sink=`File{path="grading_reference.png"(.rdata 0x1429bc800，长 0x15，相对工作目录，`Path::to_path_buf`), quality=0x64}`，无 completion | b5c6..b640 |
| **3 SceneBeforeUi / 4 SceneWithUi（主截图键）** | “截图触发” `bVar14`，三个来源 OR：(a) Steam 回调放入的 **空占位 Arc**（见 1.3）被取走（0x14031b05x..b1b3 循环找 `*(slot)==0`）；(b) `EventReader<PhotoModeCmd>` 中存在 `byte@+8 == 4`（stride 0x10，扫描 0x14031b10a `cmp byte [r8+8],4`，**PhotoModeCmd tag 4 = 截图命令**）；(c) `just_pressed(id=*(bindings+0xe8), +0xec)`（0x14031b146..b15b） | **purpose = 4 − (AppMode==Photomode)**：Photomode → 3（拍照时无 UI，之后 T9 叠加 logo），否则 4（含 UI）（0x14031b73a..b744 `cmp [R13],0x13; setz; mov cl,4; sub cl,al`）；`dims=None`；sink=`File{path=paths::config()/"screenshots"/<date_time_string_for_filename>.<ext>}`，`ext = screenshot_format!=0 ? "png":"jpg"`（0x1429bc79b/798，目录串 0x1429bc79e 长 0xb，`CMP byte [GameConfig+0x120]` 0x14031b32f），quality = png?0x64:0x55（b760..b771）；completion = `Box<dyn …ScreenshotCapturedCallback>`（0x28 B 环境 {路径副本 + `Option<Arc<Slot>>`}，vtable 0x1429b1280） | b313..b80e |
| **4（MetaCommander）** | `MetaCommander::peek()`(0x14091f0c0) 返回变体 `0x8000000000000009` = **`MetaCommand::Screenshot{path: PathBuf}`**（仓库 `command_name` 索引 9）；随后 `consume()`(0x14091f100) | **0x14031c38d 就是 gather 内部直接构造 `ActiveScreenshotRequest` 的这块代码，不是独立函数**：purpose=4（`mov byte [rbp+0xb4],4`）、quality=0x55（固定，不看扩展名）、dims=None、`+0x50 state(hurry)=0`、`+0x54 remaining=1`（`mov dword [rbp+0xb8],1`）、payload tag `[rbp+0xfe]=6`；sink=File{path=MetaCommand 的 PathBuf（0x20 B：Vec+`is_known_utf8` 字节，字节来自命令 +0x20）}；**无 completion、无 Steam**；直接 `memmove` 进全局向量（绕过 fold 闭包，所以 remaining=1 而不是 3） | 0x14031c29b..c43c |
| **6 FinalFrame（反馈截图）** | **T13，用户已排除** | `just_pressed(id=*(bindings+0xf8))` 且 `EscMenuState!=ReportBug(8)`，或在 ReportBug 下另一输入 0x14093fbe0；两个请求：purpose 6 全尺寸 quality 0x3c + `Arc<Mutex<Option<…>>>`（0x30 B，i64::MIN niche），以及 purpose 6、dims=(w*0.1,h*0.1,各 `max(1)`) 的缩略图；Arc 存入 `FeedbackReportResources`（+0x3e 变为 8） | b865..bc4a |
| **1 LutPreview** | `EscMenuState==Photomode(false)`（`*(int*)state==0xa && !(byte+8&1)`，0x14031c0f8 前） | `NextEscMenuState::set(Photomode(true))`；边长 `max(1, (u32)(extent_8[1]*0.4))` 的正方形 dims；sink=Arc 槽→`LutGalleryState.preview`（`*in_stack_58 + 8`） | 与仓库 `gather` 一致 |
| **2 MenuBackground** | `EscapeMenuJustOpened` 两个 reader 之一非空（`bVar33&&bVar36 \|\| bVar34&&bVar35`） | dims=None，Arc 槽→菜单背景 pending 槽 | 与仓库一致 |
| **2（会话预览文件）** | **`SavingGameComplete{path}`**（事件 stride 0x28，取**最后一个**事件 `[..+0x28*(n-1)]`，0x14031bf30..bfd7） | 两个 purpose=2 请求：`dims=[0x200,0x200]` quality 0x55 → `path/"screenshot.jpg"`（0x1429bc890，长 0xe）；`dims=[max(1,(u32)(w*64/max(h,1))), 0x40]` quality 0x5f → `path/"loading_background.jpg"`（0x1429bc8b8） | 0x14031c0f8 附近 |

注意 purpose 3/4 不是由“可见性开关”选择，而是**纯由 AppMode==Photomode 选择**：Photomode 下照相用 purpose 3（对应 opportunity 0x1401632fa，R8B=3），其余用 purpose 4（0x140163494，R8B=4）；purpose 6 的 opportunity 在 0x140163531；opportunity 调用点共 5 个：0x140163091（purpose 来自 `[rbp+rax+0x1820]` 的逐帧字节，即 0/1/7 类）、0x1401630d3（2）、0x1401632fa（3）、0x140163494（4）、0x140163531（6）。这与仓库 world_postprocess T9（`purpose==3 || demo&&4`）/ 后 UI 的 4 / production_renderer 的 6 一致。

**照片模式 16 帧保持状态机（O，0x14031b2a0..b307）**：`Local<[u8;2]> = [active, counter]`。
- `active==0`：若 `trigger && AppMode==Photomode` → `*(in_stack_68+0x31)=1`、状态=`[1,1]`，**本帧不拍**（转去检查 purpose 0）；若触发但非 Photomode → 立即进入 b313 拍照；无触发 → 只检查 purpose 0。
- `active!=0`：`counter<0x10` → `FrameRenderConfig.flag_20=1`(并 set_changed)、`counter+=1`、仅检查 purpose 0；`counter==0x10` → `counter=0x11` 并**拍照**（进入 b313，purpose 3）；`counter>0x10` → `+0x31=0`、`flag_20=0`、`active=0`，然后若本帧又有触发则立即拍。
- 即 Photomode 拍照前 16 帧 vermeer `skip_generate=1` 与 in_stack_68 +0x31 开启（让画面累积/稳定），之后恢复。该资源 `in_stack_68` 的类型 U。

### 1.2 Steam 部分（O）
- `add_steam_integration`（0x140945320）：新建 `Arc<RwLock<Vec<Option<Arc<Slot>>>>>`（0x30 B），`SteamScreenshots::HookScreenshots(true)`（IAT 0x142923630），`register_callback::<ScreenshotRequested>`（0x14091aa30；回调闭包 = **0x14091ad50**）。
- **0x14091ad50 = Steam “ScreenshotRequested” 回调体**：加写锁，`Vec.push(None /*0*/)`，解锁（0x14091ae36..ae77）。这就是 gather 里“空占位”的来源；gather 取走一个空占位 → 拍一张。
- gather 拍照时（Steam 存在）新建 `Arc<Mutex<Option<Result<(PathBuf,[u32;2]),String>>>>`（初始 niche 0x8000000000000001），一份 push 进该 Vec，一份进回调环境；回调在写盘完成后填槽。gather 每帧开头的 `Vec::retain_mut(add_captured_screenshots_to_steam_library::closure = 0x1402c13d0)` 在槽被填充后调用 `steamworks::screenshots::AddScreenshotToLibrary`（`path_to_absolute_cstring` 0x140b84ff0）并移除。
- `hurry_active_screenshots_on_shutdown`（0x14031eed0，由 0x1402cc875/0x14044ee13 包装调用）：把全局向量每个元素 `+0x50 = 1`（hurry），然后 join 全局写盘线程表（DAT_14362b090）。仓库 `hurry` 初值 false 与此一致。

### 1.3 未闭合
- `PhotoModeCmd(4)` 的**生产者**：仓库 `photomode_buttons`（0x14234c230）把按钮 tag 4 映射为 `OpenScreenshotsFolder`；机器码里跳表索引 = `tag-4`，tag 4 的目标 0x14234c9c8 只做“检查某 u8∈{0,1} 后把一个资源字节置 1（`mov byte [rax],1`，资源在 rbp+0x168）”，**没有 open::that 调用**；tag 5 才有 `0x14362c098` 与 `0x140b596b0`（next camera），tag 0xf 是 hide_ui 翻转（`xor byte [rax+0xd],1` 与仓库一致）。该函数的跳表目标在 DB 中**未反汇编**（故反编译截断在 201 行），需写 worker 强制反汇编后复核 tag 4/7/8/0xa/0x10。gather 读 `PhotoModeCmd.byte==4` 触发截图，所以真实“快门”按钮 → `PhotoModeCmd(4)` 的写入者未定位。
- gather 里 `in_stack_68`（+0x31）资源类型、`bVar43/bVar36` 等两个 reader 的精确 TypeId（仅凭 stride/布局推断为 `EscapeMenuJustOpened`、`SavingGameComplete`）。
- completion 回调 vtable 0x1429b1280 的函数体（写 Steam 槽的逐语句）未逐条核对。
- purpose 6/T13：按用户指示不推进。

### 1.4 建议实现要点（仓库）
1. `screenshot_capture.rs::gather` 之外新增一个 **`photo_screenshot_gather` 系统**（同在 PostUpdate、同一函数内顺序执行，保持原版先后：Steam 取空槽 → reader 判定 → 状态机 → 请求构造 → Meta 命令），参数：`Option<ResMut<SteamScreenshotRequests>>`（需把 `pending: bool` 改为 `Vec<Option<Arc<Slot>>>`，**需新增字段/类型**）、`Res<AppMode>`、`Res<GameConfig>`（`screenshot_format`）、`ResMut<FrameRenderConfig>`（`flag_20`）、`EventReader<PhotoModeCmd>`、`EventReader<SavingGameComplete>`、`ResMut<MetaCommander>`、`Local<[u8;2]>`。
2. `ScreenshotRequest` 已具备 `purpose/dimensions/sink/completion`；需**新增** `ScreenshotSink::File` 的 `quality`（已有）与一个“remaining 预算”入口：当前 `enqueue_meta_capture` 已设 remaining=1，可复用给 Meta 命令，但 Meta 路径还应保证 **无 completion、quality 固定 0x55（不依据扩展名）**；`enqueue_meta_capture` 现在要求 purpose==4，符合。
3. 路径：`paths::config().join("screenshots").join(format!("{}.{}", date_time_string_for_filename(), ext))`；purpose 0 的路径是**相对 `grading_reference.png`**，quality 0x64。
4. Photomode 16 帧状态机必须逐分支实现（含 `flag_20` 的置位/复位与 change tick）；purpose 3 与 4 由 `AppMode::Photomode` 选择。
5. 会话预览：`SavingGameComplete` 事件需要在仓库中存在（`country-core/src/systems/save/save_game.rs:534` 已有 `SavingGameComplete{path}`）；gather 里取最后一个事件写两个 purpose-2 文件请求。
6. Meta 命令的 `Screenshot` 目前 `commander_update_system` 把它当不支持并保留（见 meta_commander 测试）；接入 gather 后应由 gather `consume()`，与原版一致（原版由 gather 消费而非 commander_update）。

---

## 2. color_menu 状态机与 show_color_options 逐 case 表

### 2.1 已取证（均 O，来自 `wave2/reviews/color/evidence/decomp/*_full.c`，写 worker 全量反编译）
**重要澄清**：`color_menu`（0x142291230）末尾主 switch 变量 `local_100[0x30]`（`0x1422960f5` 跳表 `0x14303ba18`）是**菜单类别字节 = `ShowColorMenuCmd.kind`**（`show_color_options` 0x14229a7f0 的第一/第二个 switch 变量即 `*(u8*)event`，同域 0..0x26），不是逐帧的 UI 状态。图标 `kind@IconInfo+0x18`（3=色板、4=halftimber/skirt、5=分隔、7、0xc=plaster peel…）是另一层。所以“状态机”=“菜单类别 + 交互阶段位”。

`ShowColorMenuCmd`（仓库 `systems/color/src/lib.rs`，24 B）：`kind u8 @0`、`decorator_id u32 @4`、`WallId payload u64 @8`、`position Vec2 @0x10`；`autoclose_color_menu` 读 `kind==0xc` 关闭。生产者：`ui_color_wall_and_roof`（0x141c9ab70，AppMode==Color(12) 守卫，raycast mask 0xcb，命中类型 0/2/6/7/8→kind 分派）、`ui_color_decorator`（命中字符码 `'8','9',':'→3`、`';'→10`、`'<'→9`、`'='→6`、`'?'→bVar21+0xb`）、`ui_color_gates`。

`show_color_options` 第一 switch（0x14229a7f0 行 447-1433，kind → 目标块，O）：
| kind | 目标 | 说明 |
|---|---|---|
| 0 | 0x14229b725 | 墙：13 色板（bricks×5+plaster×8）+ plaster 条件追加 peel(kind 0xc)+halftimber/skirt 族(kind 4)+flat-roof rim/非平顶 5 图标块（`PublicWalls::get`，`Roof::is_flat`，`ClutterVersion::is_initial`）；已在仓库 show_color_menu.rs 重建 case 0 |
| 1 | 0x14229bde9 | 屋顶：11 色板（ochre/red/purple/blue/light_blue/turquoise/green_light/green/dark_wood/dark/grey，v1c=8,0,10,2,9,3,6,4,1,5,7） |
| 2 | 0x14229af43（块 1835-2381） | 大块，U（未逐行解） |
| 3 | 0x14229b5fd（2381-2439） | `closure_0` + `DecoratorSubtype::from/subtype()`，非 `(subtype-9)<2` 时追加 `"…windows…purplestone_floor_icon"` 串图标，`IconInfo{kind=3,v=999}`，`DecoratorId 0x1b` |
| 4、7 | 0x1422a0f96 | `core::panicking::panic`（不可达） |
| 5、8、0xc | 0x14229aa6e | 仅 `closure_0` |
| 6 | 0x14229b5b9（2453-2756） | `DecoratorStorage::lookup_decorator_info` |
| 9 / 10 / 0xb | 0x14229aa86 / 0x14229ae6e / 0x14229c1b3（2756-3332） | 装饰器子类（门/窗/…，`DecoratorExtraInfo::try_as_*`） |
| 0xd | 0x14229eb2b | 实体/列表索引查询 + `StairsState` 类（3332-3362，`unwrap_failed` 路径） |
| 0xe / 0xf | 0x14229eb5b / 0x14229ea98（3362-4759，最大块） | `get_instruction` 边界检查、`StairsState::get_all_connected_nodes` 家族 |
| 0x11、0x14 | 0x1422a0f7a | 不可达 |
| 0x12 / 0x15 | 0x14229eff5 / 0x14229ea68 | 小块 |
| 0x19..0x26 | 0x14229efc4, ae84a, aea87, ae724, ae7d5, ae4d3, aeaed, aeb71, ae391(0x21/0x22 共用), ae630, aec61, ae46f, ae53c | 上半段枚举（多为 `Events<(DecoratorId,Vec<PreferredNodeAlignment>)>` 家族及 `0x4303..` 串表图标）；逐 case 精确图标表 U |
- 尾部第三 switch（行 5094 `pauVar43[3][0]`）按 kind 写事件写者（case 0..）。

`color_menu`（0x142291230）主 switch case 表（O，反编译行 3062-3454）：
| 菜单类别 | 值槽写（`local_180+off`，即 `*in_R8` 图标值记录） | 事件（ICF 合并型 `Events<(&str,…)>` stride 0x18：`{event_id, str_ptr, u32 值, u8 flag}`） | `History::add` 记录（`EditType::Color`=tag 9，载荷首字节为 tag） |
|---|---|---|---|
| 0 墙 | +0x18/+0x1c；`PublicWalls::get` 为空则不写；`WallStyle::is_halftimbered` 且 `*(wall+600)<2` 时 +0x24 取 `get_instruction` 表项、+0x42=`wall+0x299`；否则 +0x40=`wall+0x295` | 6 个同形事件写者之一 | tag 3（`{1,1,3, str, old, new}`） |
| 1 屋顶 | +0x20 | 同上 | tag 5 |
| 2 | — | 无写，直达 `code_r0x142295d8c`（状态保持） | — |
| 3/6/9/10/11（`default:`） | 由 `+0x34` 载荷 | `(uVar34,uVar33)` 事件 | 经 case 0xd 内层 switch（见下） |
| 4 | +0x30 | 同上 + `WindowFocusEvent` | tag 8 |
| 5 | +0x38 | 同上 | tag 4 |
| 7/8 | +0x24（7）/+0x2c（8） | 同上 | tag 9（8 字节记录 `{1,0,…,1,…,0,9}`） |
| 0xc | +0xc | 同上 | tag 0xd |
| 0xd | 内层 switch(icon 子类 `+0x30`)：3 → +0x0/+4（`uVar33==999 || (uVar33==0&&flag==0)` 清零否则置 1 写 `uVar33`）；6 → +0x34；9 → +0x28；0xb → +8 | 同上 | tag 7 |
| 0xe/0xf/0x11/0x12/0x14 | `WriteSettingsFileCmd` 面（反编译 `halt_baddata`，U） | — | — |
| 0x10 / 0x13 | +0x2c | History 直接 add（`pcVar30` 载荷） | — |
| 0x15 | +0x28（+字节自增） | — | — |
| default 尾 | — | `WindowFocusEvent{event_id,0}`（`code_r0x142295d9b`） | — |
- 前半（O，见 `color_menu.rs` 头注）：遍历 `CustomizationIcon`（0xa0 B）实体；双 `just_pressed()` 命中置 used；选中步进跳过 kind 5；每图标 `format_inner`→`WidgetParams::new`→`UiState::add<String>`→`is_active/clicked`；收尾 state 6 → `WindowFocusEvent`+History tag 0xa；state 9 → SettingsMenuState 事件+tag 0xb（`ColorMenuBrightnessValue::to_f32`）；state 10 → tag 0xc；`History::try_start_input_recording` 门否则 `WriteSettingsFileCmd`。布局 `cartesian_to_polar`+`AngleRange::clamp`+`polar_to_cartesian`（中心 +0x50、半径 +0x90）。
- 与 color_button_animations（0x1422979b0）/ delete_color_icon_entities（0x142297ff0）配合，仓库已重建 `color_button_animations`。

### 2.2 未闭合
- show_color_options kind 2、3、6、9、10、0xb、0xd、0xe、0xf、0x12、0x15、0x19..0x26 的精确图标表与门（装饰器分类、楼梯连通节点）；反编译含跳表错位（`halt_baddata`、`*(char*)piVar10 = …`），需对 `0x14303ca70` 两段跳表做强制反汇编后逐目标重建。
- color_menu 的 6 个 `Events<(&str,SettingsMenuState)>` 同形事件的真实类型（应对应仓库 `SetWallColorIdCmd/SetRoofColorIdCmd/SetHalftimberColorIdCmd/SetDecoratorColorIdCmd/SetGateColorIdCmd/SetWindowLight`，ICF 合并导致名字丢失）与逐 case 的对应关系。
- CustomizationIcon 0xa0 B 字段名、图标值记录布局（core typed-gap）。

### 2.3 建议实现要点
- 先在 `systems/color/src/show_color_menu.rs` 补 kind 2/3/6/9/10/0xb/0xd 的图标表（需 `IconInfo` 保持现有字段，不新增）；其余 kind 依赖 `DecoratorStorage::lookup_decorator_info` 等 core 面。
- `color_menu.rs` 需要**新增** `ColorMenuKind(u8)`（菜单类别，来自 `ShowColorMenuCmd.kind`）字段到 `ColorRadialMenu` 并以其做 switch；History 载荷用 `EditType::Color`；`(&str,…)` 同形事件改用已有 `Set*ColorIdCmd`。属于**新增字段 + 逐 case 事件映射**，改动大、风险中高（依赖 core typed-gap），建议排在第四优先级。

---

## 3. custom_lut 生产者（ICC 显示器配置文件 LUT）

### 3.1 已取证
`final_post` 读取资源 TypeId `0x5f06b298510fb768/0x054e14bacecb858d`（0x14090d317）= **`ActiveIccProfile`**：`[+0]==i64::MIN` 表示 None（无名），否则 `+0x18` 为纹理句柄 → `FinalPostEnv.lut_texture`（+0x28），在 `final_blit_body` 绑定到 `B_LUT_CUSTOM`；None 时回退默认 LUT。资源插入点：`BackendSystems::insert_backend_resources`（0x140942f80，调用 0x14094348d）。

- **更新系统**：`country_core::systems::backend::icc::inner::update_window_icc_profile`（**0x14093d830**），参数 `(GraphicsDeviceResource, ResMut<ActiveIccProfile>, EventReader<IccProfileChanged>)`，事件 stride 0x28（id + PathBuf）。系统 wrapper 0x14143a94c / 0x1414ea5a7（`ResMut<ActiveIccProfile>::init_state` 0x141394900，`Res<…>` 0x1401845d0）。
- 流程（O）：取**最后一个**事件的路径；先把旧 `ActiveIccProfile` 置 None（`*name = i64::MIN`，`RawManagedHandle::drop` 释放旧纹理，0x14093d92c..d962）；`OpenOptions::open`+`read_to_end` 读 ICC 文件；`cmsOpenProfileFromMemTHR(0, buf)`（0x14093da1b）；`cmsCreate_sRGBProfileTHR`；`cmsCreateTransformTHR(in=sRGB, fmt 0x4009a, out=该 ICC, fmt 0x4009a, intent 0, flags 0)`（0x14093da8e；0x4009a = lcms `TYPE_RGBA_16`：3 通道+1 extra，16 位）；用 itertools 风格三重 range 迭代器（`Vec::from_iter` 0x1413036c0，范围 0..32，元素 `[u16;4]`，32³=32768 项）生成输入网格；`cmsDoTransform(xform, buf, buf, 32768)`（0x14093db17）原地变换；`cmsDeleteTransform`/`cmsCloseProfile`×2。
- **纹理**：`TextureDesc::new_3d(32,32,32, format=0x5b)`（0x14093db8e；`0x5b`=91=`VK_FORMAT_R16G16B16A16_UNORM`）`.sampled()`，数据 `32768*8` 字节（`R14<<3`），经 `GraphicsDevice` 的 create-with-data（0x14093dbe8，R8=0x142ab20e6 起的串 `ICC`/`ICC LUT texture`）创建；失败走 `unwrap_failed("ICC LUT texture")`（0x142ab20e9，长 0xf）。
- 命名：写入 `ActiveIccProfile.name = Path::file_name(path).to_string_lossy()`（0x14093dc2c..dc47；无文件名则整路径），句柄写 `+0x18`。
- 失败路径：错误经 `anyhow` 并 `log`（0x142ab2158 `country_core::systems::backend::icc::inner`，消息 `"Failed creating an ICC LUT: "` 位于 0x142ab20f8 附近，U 消息拼接细节），**并且旧 profile 已被清空**（先清后建）。
- **事件生产者**：`run_event_loop`（0x14026f5xx）在窗口/显示器变化标志（`[rbp+0x2e3]`，置位点 U）且 `monitor.is_some()`（`[rbp+0x78]==1`）时调用 `tiny_glade::icc::get_monitor_icc_profile`（0x1403ac400，仓库 `icc.rs` 已复原，调用点 0x14026f613），`Ok(path)` 则写 `Events<IccProfileChanged{path}>`（0x14026f62f..f722），`Err` 则 log（0x140d4a330）。
- 更新时机：事件驱动（启动后首次显示器检测、窗口跨显示器移动等），**不是逐帧**；LUT 在 `Update` 类调度中由系统写入，`final_post` 在渲染线程读取。

### 3.2 未闭合
- 输入网格迭代器的轴序/归一化（`0x1413036c0` 内 `(i*0xffff/31)` 与 R/G/B 嵌套顺序）未逐指令还原；`0x142ab1530` 只是 panic Location，不是迭代体。
- `[rbp+0x2e3]` 的置位事件（Moved/ScaleFactorChanged/显示器枚举？）。
- `ActiveIccProfile` 完整布局（仅 `+0` Option<String> niche、`+0x18` 句柄已证）。
- `GraphicsDevice::create_texture_with_data` 精确签名（label 取 `"ICC LUT texture"` 或 `"ICC"` 未判定）。

### 3.3 建议实现要点
- 新增 `country_core::systems::backend::icc` 的 `ActiveIccProfile{name: Option<String>, texture: Option<ManagedTexture>}` 资源、`IccProfileChanged{path: PathBuf}` 事件、`update_window_icc_profile` 系统；仓库 world_postprocess.rs:1145 改为读该资源并构造 `custom_lut: Some((ManagedHandle, AttachmentTexture{extent 32³ 视为 3D, format 91}))`（需确认 `BindTexture::Raw` 对 3D 纹理描述的支持，`AttachmentTexture` 当前是 2D 描述，**可能需要扩展 depth/维度字段**）。
- **新增依赖 lcms2**（`cmsOpenProfileFromMemTHR/cmsCreate_sRGBProfileTHR/cmsCreateTransformTHR/cmsDoTransform`），来源以原作 Cargo.lock 为准（当前仓库 `Cargo.lock` 无 lcms）。
- `event_loop.rs` 需在显示器变化时调用已有 `icc::get_monitor_icc_profile` 并写事件（Windows 专属；Linux/mac 构建下无事件则 `custom_lut=None` 正确，保持与原版“无 profile”一致，不要伪造）。
- Linux/lavapipe 测试环境无 ICC 事件，**对现有整帧像素基线无影响**。

---

## 4. T12 `lut_gallery_thumbnail`

### 4.1 已取证（与仓库逐项核对，结论：一致）
- 着色器串：`/lut_gallery/thumbnail.raster.hlsl`（0x142a9cdf0，长 0x22）紧接 `lut_gallery_thumbnail`（0x142a9cdf0+0x22，长 0x15）；与 `lut_gallery.rs` 的 `SHADER_URI/SHADER_NAME` 一致。gameplay_startup.rs:305 在 PreStartup 用 `load_shader_into_library(URI, NAME)` 注册，对应原版 `LutGalleryPlugin::build`（0x1408b04c0）：`insert_resource(LutGalleryState{Arc<Mutex 0x48>, preview:None})` 后尾调 `SubApp::add_systems`（0x140b67880）注册 startup。**未闭合**：startup 系统函数体与调度标签没有单独反汇编（tail-jump 目标需再追）；串相邻性 + 仓库行为一致，风险低。
- `render_photomode_lut_gallery`（0x1408b0560，6721 B）：入口 `cmp byte [0x143634d90],0; jnz return`（demo 静态，见 §5）；资源/预览槽缺失返回；`get_raster_handle_by_name`（0x1408b09a5）；`RasterPass::new_impl`（0x1408b0b7a，pass 名 “LUT thumbnail”）；3 次纹理绑定（0x1408b0fad/0ff6/104f，预览 `0x11ba5697cc40b3dd`、图标磁盘、`NameHash(lut名)`）；每格 `optional_uniform` + 6 个 `write_uniform`（0x1408b10c8..126f，对应 alpha/pos/size/highlight/is_current/is_first，共 7 个 uniform，与仓库一致）；`RasterPipelineOpts::default`（0x1408b0e9a）；`draw_no_vao(6)`；`RasterPass::render`（0x1408b142d）。
- purpose 1 请求（gather，O）：`EscMenuState==Photomode(false)` → `NextEscMenuState=Photomode(true)`、边长 `max(1,(u32)(extent_8[1]*0.4))`（0x14031c0f8 前 `mulss 0.4`，`(int)(i64)` 饱和转换，`iVar26+(iVar26==0)`），Arc 槽 → `LutGalleryState.preview`，旧槽 Arc 引用减一后替换。仓库 `screenshot_capture.rs::gather` 与此一致（含 `.max(1)`）。

### 4.2 未闭合
- `render_photomode_lut_gallery` 内 `RasterPipelineOpts` 的 state 字节改写（byte0&=0x7f，byte3=(b3&0x9f)|0x40）仓库已按此实现；未发现差异。
- startup 系统体（见上）。

### 4.3 建议
无需改动；仅建议把 `(demo 静态)` 与 `GameConfig.dev.demo` 的关系统一（见 §5）。

---

## 5. 其余 U / 未闭合点

| 位置 | 原标注 | 取证结论 |
|---|---|---|
| world_postprocess.rs:640 “Static 0x143634d90 selects the alternate background name (U)” | U | 0x143634d90 = **`_DEV_OVERRIDE_IS_DEMO_MODE`**（show_color_menu.rs 头注已命名）。xref 共约 23 处读取（vermeer_render_from_target 0x1408a8de2、lut_gallery 0x1408b05b0、session_selection 0x1408f8d6c/0x1408f92b1、GpuLutPlugin 0x14090e116、startup_water/terrain、audio、camera_rig…），**零写入**；映像初值字节为 0。vermeer 中：非零 → 背景名哈希 `0x52c2c594db0dcfb0`，否则 `0xf736d24c0ec5bd77`（仓库 `vermeer.rs` 的 `BACKGROUND_NAME_ALT/DEFAULT` 常量一致）。结论：传 `false` 正确。**风险**：同文件 run_lut_gallery 用的是 `GameConfig.dev.demo`（不同变量），与该静态不是同一来源；建议新增 `dev_override_is_demo_mode()`（常量 false）供原版读取该静态的所有位置使用，保留 `config.dev.demo` 仅用于原版确实读 GameConfig 的位置（如 photomode next_demo 用的是 `*0x14362c098` 指针指向的字节，U）。 |
| world_postprocess.rs:1145 `custom_lut: None (U)` | U | 见 §3：生产者 = `update_window_icc_profile`(0x14093d830)，事件源 `run_event_loop`→`get_monitor_icc_profile`。 |
| world_postprocess.rs 头注 “steps whose host or producer is not recovered / explicit U controls” | — | 控制量里 `bvar43` 来自 `active_capture_purpose`（gather 请求）；purpose 3/4/6 在仓库目前**无生产者**（见 §1），所以 T9 的 purpose-3 分支、logo 叠加、`opportunity(3/4/6)` 现在只在测试里触发。 |
| world_postprocess.rs:1203 注 “T18 custom cursor (visibility byte is U)” | U | `cursor_visible = GameConfig.ui.custom_cursor (+0xf4)` 已在 `resolve_world_tail_controls` 取值；注释过期，未发现新差异。 |
| screenshot_capture.rs 头注 “See CONTRACT.md for remaining producers.” | — | 剩余生产者即 §1.1 表：purpose 0、3/4（截图键/Steam/PhotoModeCmd(4)）、4（Meta）、2 文件（SavingGameComplete），T13(6) 排除。 |
| menu_background_host.rs / session_preview_host.rs | 文件内无 U/未闭合标记 | `session_preview_host.rs` 的预览**写盘**生产者原本在 gather（purpose 2 文件请求，512×512 q85 `screenshot.jpg` + 64 高 q95 `loading_background.jpg`），仓库尚未接入；host 只负责读取/渲染缩略图。 |
| gather 里 `Local<[u8;2]>`、in_stack_68 +0x31、FrameRenderConfig.flag_20 | — | 见 §1，新增发现。 |

---

## 实现优先级建议（收益/风险）

1. **P1：接入 gather 的主截图键生产者（purpose 3/4 + Photomode 16 帧状态机 + Steam 占位 + Meta 命令 purpose 4 + purpose 0）**。收益高（打通整条截图/拍照链，purpose 3 才会触发 logo 与 T9 opportunity；Meta 脚本测试需要）；风险低-中（逻辑集中，参照 0x14031af10 反编译即可；需把 `SteamScreenshotRequests` 改为 Vec<Option<Arc<Slot>>> 并确认 `flag_20` 的 change-tick 语义）。先复核 `PhotoModeCmd(4)` 生产者与 photomode_buttons tag 4 映射（需强制反汇编 0x14234c9c8 跳表目标）。
2. **P2：会话预览文件请求（SavingGameComplete → purpose 2 两个文件）**。收益中（主菜单读档缩略图在新存档后才会更新）；风险低（参数全已取证）。
3. **P3：统一 `_DEV_OVERRIDE_IS_DEMO_MODE` 常量**（vermeer/lut gallery/GpuLut 等位置）。收益小但防止 `GameConfig.dev.demo` 误开导致分叉；风险极低。
4. **P4：ICC 自定义 LUT（lcms2 + 32³ RGBA16 3D 纹理 + 事件）**。收益取决于平台（仅 Windows 显示器有 ICC；当前 Linux/macOS 验证环境无事件，无像素差异）；风险中（新增 lcms2 依赖、3D 纹理绑定的 `AttachmentTexture` 扩展、输入网格轴序尚未还原）。建议在 Windows 实机验证阶段再做，且先还原 0x1413036c0 网格序。
5. **P5：color_menu / show_color_options 余下 case**。收益对渲染尾部无直接影响（交互功能），风险高（依赖 core typed-gap 与跳表强制反汇编）；放最后，先做强制反汇编取证。
6. **不做**：T13 purpose 6 反馈截图（用户已排除）。

附：本次取证新增辅助脚本均在 `/tmp`（`gfc.py`、`rq.py`、`jt.py`、`rb.py`、`tgt.py`、`fs.py`），未修改任何仓库源文件、未执行构建。
