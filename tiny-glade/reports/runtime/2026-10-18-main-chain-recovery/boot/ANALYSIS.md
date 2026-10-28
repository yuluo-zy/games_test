# B 启动与相机、时间生产者：原证据及当前实现

本轮只按 B 原发行 RNE（SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`）和 B 保留反编译调查。没有从旧进度统计推断完成度。macOS 宿主适配与原游戏业务分别记录。

## 1. 原 main 顺序

入口 `0x1403db340` 的原始反汇编：`skeleton/task-index/work/TG-135f736ac5c9d1ccab/main-chain-boot-20261018/evidence/main-rne.asm`。

| 原指令位置 | 已观察行为 | 当前消费者 |
|---|---|---|
| `1403db36d..379` | Instant::now 并写两个启动计时全局 | main::set_startup_instant / 首帧日志 |
| `1403db37f..38b` | 本地时间初始化，再解析 CLI | utils::local_time / app_config |
| main 非 crash-server 分支 | AppSteamIntegration::try_new；失败返回错误，不能默认当作无 Steam 成功 | 普通 main 保留失败；真实 SDK 可经 main_with_steam 进入 |
| `1403dc8c4` | 路径初始化成功后 get_app_config | 已连接真实 paths 和 RON 文件读取 |
| `1403dd06d..074` | ECX=3，DL=1，始终 Info 日志和文件输出 | logging::set_up_logging(Info,true) |
| `1403dcf7a..ff9` | capture_crashes 门控 CrashReporter::spawn_server；失败日志并继续 | minidumper 后端仍缺，不构造假 ClientHandle |
| `1403dd54a` | 更新 Vulkan allowlist 默认值 | 原有实现及真实设置文件 |
| `1403dd564..57d` | 环境变量 MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS=1 | Rust 2024 两个 binary 在任何应用线程之前设置；时机为平台适配 |
| `1403dd584..5c9` | CLI repro_panic 为 None 才安装 panic hook | 已按原门连接；不是 capture_crashes 门 |
| `1403dd611..624` | 加载语言后写 config.dev.internal_hotkeys 全局 | 已连接原语言映射及 core setter |
| `1403dd63b..777` | rhapsody 日志；verify-integrity 优先于 configure-vulkan、mock、游戏 | 验证调用真实 manifest；缺失 GUI 显式错误 |

普通 `tiny-glade` binary 未携带 Steam SDK factory，不能声称已实现成功的原 Steam 初始化。`tiny-glade-host` 是明确单列的本机宿主命令，使用用户显式提供的 `TG_INSTALL_ROOT`，不会复制或伪造原资产。原 Windows 双线程拆分没有强套到 macOS main-thread-only 的窗口循环。

## 2. MainCamera::new 是必须恢复的真实启动消费者

原 run_game_thread 在 `14059dba3` 调 WindowSize::aspect_ratio，`14059dbe3` 将结果放 XMM1 调 `MainCamera::new(0x14090ec70)`，随后插入 MainCamera。不能使用旧实现的全零 Default 代替构造。

已恢复六个 body 的初始控制值、lens、Normal/TiltShift/TopDown 初始姿态和三副真实 dolly 0.6.0 rig：

- Free：Position(0,1,0) → YawPitch(0,0) → Smooth(position=1,rotation=1)。
- Walk：Position(0,1,0) → YawPitch(0,0)；原 WalkCamera::new `140993e20` 的状态字节 0x11。
- Butterfly：Position(0,0,0) → Rotation(identity) → SmoothPosition(2).predictive(true) → Arm(0,1.5,3.5) → SmoothPosition(4) → LookAt(0,0,0).tracking_smoothness(4)。
- Normal/TiltShift：yaw=-31°、pitch=-30°，距离分别 50/125，FOV 分别45/15。
- TopDown：yaw=0°、pitch=-90°，距离500，FOV=8。

这里复用原锁定 dolly 的驱动、平滑和 build/update(0) 依赖实现。原始字节视图中的分配指针不复制成 Rust 可解引用指针；真实 owner 保留在 startup_rigs。原构造最后把四个单位矩阵拷贝到首0x100字节，因此 new **没有擅自调用矩阵重建**。原 main_camera_update 仍不是本轮构造证明的一部分。

一次重要纠错：仅看对象 `+0x10=4` 容易将 Butterfly 的 LookAt 误读成 target=(4,0,0)。原 fixture 的最终四元数及位置否定了该解释；结合原依赖字段与原存储，确认它是 smoothness=4、target=(0,0,0)。源码已纠正，未用近似图像掩盖。

保留的原构造执行证据：`evidence/parallel-camera/B-constructor-native.json`（同一 B RNE SHA）。本轮试图重新运行其 Unicorn oracle：首次因磁盘满失败；清理后进程退出132（SIGILL），未生成新成功记录。因此本轮候选单测消费的是**保留的原指令 fixture**，不声称 fresh oracle 成功。浮点姿态容差只覆盖原 x86 依赖和当前 ARM 依赖的数值差；lens/控制字段直接比较字节。指针与未定义 padding 不比较。

## 3. time_update 的门不是 AppLoadingState

PDB 直接调用方 `0x140aad185` 的完整参数类型与具名资源注册确认：

`Time<()> → ShaderRefactoringState → EscMenuState → InputRecordReplayState → GameTime`。

ShaderRefactoringState 的资源 TypeId 由 `App::insert_resource<...>@14152fac0` 和 `register_resource_with@1402157b0` 交叉核对（立即数 `e67d7e3df983f8ce / c95ed87f413070ab`）。`disabled@140b1f210` 实际是 `disc == 0`。原 `insert_resources@140940d80` 尾部 `1409410ea xor edx,edx` → `14152fac0`，证明启动 disc=0。

新增 core resource 是已有 render RefactoringState 的 alias，**没有创建第二个 ECS 身份**。time_update 现在调用真实 GameClock.update，重放态从真实 stream.frame(index) 取 delta。新增 GameClock.paused 对应原+0x31，保持只读门。原资源初始化 `140940eb4` 写+0x30的word1，故本宿主初始化 ran=true、paused=false。

## 4. 启动资源与调度的新增核对

原 `insert_resources` 的 `140940dce..dd7` 用两个零 XMM 参数调用具名 CursorPositionSS 插入，证明初值(0,0)。TimeOfDayScreen 插入前的原清零覆盖其 visible@+0xa，当前显式 false，不给未知类型盲加 Default。

原 `add_stages@140945ed0`：enum_iterator 的跳表 `142c390a4` 枚举0..44，45为终止；以 Chain::Chained(empty TypeIdMap) 交 `SubApp::configure_sets<Update>@140b68190`。当前 GameSystemSet(u8) 只保留已观察的标记身份，以 Bevy `.before` 对相邻标记表达相同依赖图，不恢复依赖内部容器。

`add_backend_systems@140942c80` 明确将 time_update tuple 加入 set2、reset_input_state 加入set3、输入链加入set5，最终调用 `SubApp::add_systems<Update>@140b67280`。当前 time_update 已修回 Update 的set2，reset接set3。time→现有GUI消费者另有清楚标注的宿主顺序；不声称已恢复完整原系统清单。

## 5. 修正 reset_input_state 错误接口

原 `in_set@14168c0e0` 的类型明确为 ActionSetValues、NewSessionStartedCmd、SessionLoadedCmd、ButtonInput<KeyCode>。旧实现将它误认成 InputTranslationStateResource、InputCursorMoved、InputReplayCommand，虽然尺寸接近，却是不同业务身份。

原 `140942b00` 每帧都将 current 移入 previous 并清 current；只在两条会话事件中至少一条出现时重置键盘。第二条 reader 被第一条的存在短路。原 Bevy reset_all 清 held pressed 和边沿，旧实现只清边沿也不正确。这些都已在真实原指令与原签名上修正。

## 6. 没有被本轮掩盖的缺口

完整原50余插件/域注册仍需逐 consumer 恢复。当前只装配已核对的资源和安全调用，并没有挂空音频、空地形或假GPU句柄。GPU texture/shader/SSBO 的构造必须属于 renderer 的同一真实设备。

`input_record_replay` 当前恢复签名读 ToolbarCategoryModes，而原注册模板读 Toolbar；未解决其真调用映射前，没有把整个 add_input_cpu_consumers 无差别接入。MainCamera 的构造证明不替代原相机每帧控制器、游戏场景加载或完整世界图验证。`tiny-glade-host` 能进入平台入口也不等于完整原游戏恢复。

## 7. 原程序的双 App 生命周期

原 `run_game_thread` 不是用同一 World 删除 MainCamera 模拟加载。它先建立完整游戏 App，再在 `GameConfig.dev.skip_splash == false` 时建立独立加载 App。

- 完整游戏插件 tuple：`14059dedf xor ecx,ecx`、`14059dee1 xor edx,edx` → `1405b0140` → `14037c160`，Vermeer 参数为 enabled=false、hold=false。
- 加载插件 tuple：`1405a4518 mov cl,1`、`1405a451a mov dl,1` → `1400cd450` → `14037c160`，参数为 enabled=true、hold=true。tuple 的原类型清单是 MinimalStartup、SettingsMenu、AnimatedLogo、LoadingScreen、Vermeer、Mei、SplashScreen。
- 加载 App 独立 new Shader/Texture/Mesh/SSBO/VAO/Placement/DynamicConstants；复制或 clone GameConfig、WindowSize、FrameRenderConfig、AdaptiveRenderState、RuntimePlatform、AudioCore、ActiveInputModeTracker，GraphicsDevice 是同一实际设备的 Arc clone。`1405a501e` 给 MeshWatch::new 明确传 false。
- 异步 `closure_env8@1400d7440` 在 `1400d772a` 不断 SubApp.update，观察 Events<LoadingGameComplete> 两个 buffer。该身份由原立即数扫描和具名 `register_event@14151ffb0` 交叉确认。收到后 `1400d7a65` 做 mode1、无 swapchain 的世界预渲染，再 `1400d7b6f` 发送 SubApp。
- 主循环等待 LogoAnimation.is_waiting 才领取结果并 finish_wait；等待 is_finished 后，把设置、输入模式、窗口、显示器、帧配置、自适应状态、Vermeer.into_disable_hold、egui Context 移入游戏 SubApp，替换当前 App。GPU 库没有在交接中被混成同一库。

当前 `GameLoop::new/frame` 已接入上述双 World、独立 shader owner、同 device 的 renderer.activate_application，以及真实 async SubApp 通道和交接。宿主会提前接收失败，避免缺生产者成为永久加载；成功结果仍受 logo 两阶段门控制。本轮后续已补原 AppMode、Manager/source 与会话请求生产；当前止于真实 snapshot/history 世界写回缺口，世界预渲染未恢复也明确报错，**不发假的 LoadingGameComplete，不按帧数设 ready**。

新恢复的 FrameRenderConfig 保留原三组尺寸、world/tgsr 分辨率倍率和尾部标志。AdaptiveRenderState 只实现已证明的初始 tag0 与 `!ignore_vram_budget` enabled；未解码原 inactive payload，未冒充后续 GPU 预算控制器。RuntimePlatform 的原分支是默认2、GFN成功且true为1、真实SteamDeck报告覆盖0；macOS单列宿主没有GFN/Steam SDK检测能力。

原 asm/查询保存在 `TG-c459c7f73345e24709/main-chain-boot-20261018/{async-loader-rne.asm,loading-plugin-tuple-rne.asm,gameplay-plugin-tuple-rne.asm,map_loading_ready.json,runtime-platform-rne.asm}`，主循环取证在 `TG-aff540cfd24b989ccb/main-chain-boot-20261018/loading-handoff-rne.asm`。

## 8. AnimatedLogo 的真实场景生产者

`AnimatedLogo::build@140b19d40` 的 Startup 和 Update 都已接加载 App。Startup `140b19d90` 的原路径/长度从发行 RNE 实读：

| 网格 | 库名 | shader路径 / 库名 |
|---|---|---|
| assets/meshes/tiny_glade_logo.json | tiny_glade_logo | /logo_animation.raster.hlsl / logo_animation |
| assets/meshes/logo_branches.json | logo_branches | /logo_branches_animation.raster.hlsl / logo_branches_animation |

每个网格经真实 JSON loader、MeshWatch.watch、AssetMeshLibrary.add、ShaderBackend 编译，再 spawn 原 DrawableMeshBundle、KeepOnLevelUnload、DesignatedRenderPass(18)、DontCastShadow。平移(0,6,0)、缩放(5,5,5)直接来自 `142ae8fb0/1429cd050` 常量；旋转复用原 Transform::default。

两个 draw callbacks 现在读同一 canonical LogoAnimation：env37 读 progress[0]/[4]，env38 读 [2]/[4]，无需插第二份同名 View 资源。KeepOnLevelUnload 的原组件注册 `140852670` 明确 size0/align1，使用统一ZST。

真实 FMOD 类型可调用新增 `animate_logo_system<S: Studio>`，声音实例化失败先 panic，再禁止进度写入，保持原失败顺序。当前 macOS 明确宿主仅使用原 AudioCore None 分支推进动画，没有假 Studio 实现。shader live-link watcher与原完整音频启动仍单列缺口。

加载 App 另接经原证据确认的 set7 字体缩放、set44 TextFragment/Taffy/Mei/真实GPU flush 链。它消费真实资源和实体；LoadingScreen/SplashScreen 的其余内容模板与完整会话生产者仍不能以空节点或0 primitive声称完成。


## 9. 独立重审并修复启动存档 producer（后续轮）

新一轮没有复用旧完成度报告作为依据。fresh B 反编译、发行 RNE 与PDB参数交叉确认后，发现旧源中“13参完整恢复”的表述过强：真实函数没有 Local<bool>，它消费并移除NextSession；None ActiveSession需要Commands插入；NextSession.kind和LoadGameManager.source是不同枚举；两种会话事件不能无条件同时发送。

目前真实装配顺序为：原45个Update sets → StatesPlugin/init_state(Splash) → PreStartup读取profile.ron → 原PostStartup NextState(Game) → OnEnter(Game)消费NextSession → 真实会话/模板路径选择 → 更新GladeSettings/ActiveSession、AppMode=14 → Manager.source→Update set12 load_game解析选中history.json。

AppLoadingState的三态0=Splash、1=Game、2=Reload来自fmt@140308a00的原串表1429b8460/长度表1429b8448。OnEnter(Game)的state1由140941158证明；resource_exists<NextSession> helper141740a20使用vtable142ccf9d0，run_unsafe14173e110真正调用startup_session。load_game注册set12由1409443de=0xc、helper14174d040使用vtable142cd2d78（run_unsafe141749a10），in_state(Game)由1409443ec=1证明。此前临时Update顺序桥已被原调度取代。

异步存档线程在资源装配时启动，名为原Async save，真实命令与错误通道连接AutoSaveState。UnsavedFileList是与该线程共享的同一Arc，启动会话有效性检查读取它，而不是新建一份空列表。当前channel实现仍复用既有crossbeam适配，不声称ABI同Windows std mpsc。

Manager的真实new为state2、None source、一项SwitchAppMode(Editing(None))的owned post-command。状态偏移是+0x48，旧文档+9把qword索引当成字节。startup真正重建source/options后才进入state0；原后续state1用于发送有符号计数差，state2消费postcommands。安全Rust用Option表示原idle时未初始化的flags，不把它们猜成有效零值。

新建场景首次选择assets/starting-builds/{full|demo}/intro；其后从该根中只收目录、排除intro、排序后取(count−1)%len。GameProfile.new_glade_count原serialize_field<usize>及+1d8 qword证实应为usize，旧u32已修。读取选中build/history.json，仅复制其snapshots子目录。GladeSettings从assets/glade/{theme}加载，旧源从存档目录读取也已修正。

当前new/continue/latest的普通空on_load请求已可达。拥有复杂on_load payload、预解析请求及重载转换仍有界显式错误；没有忽略它们后假装成功。原demo全局在本机由配置值于工作线程启动前设置，属于明确宿主适配。

最关键的修正是load_game不再在首次状态推进或仅JSON解析成功后发送LoadingGameComplete，也不再丢弃Apply然后发ready。现在真实读取、校验选中history.json后，在原snapshot/history写回@1417a9c50缺口给出明确错误；当前snapshot helper漏Roofs/Trees/颜色等，未被拼空资源绕过。此缺口和完整world graph的剩余consumer共同阻止进入可玩的世界，尚不是完整游戏启动成功。

加载App同时接入原MeiPlugin的set6 mei_pre_update（1406873a4/3d2/42a）与SplashScreenPlugin set36 ui@141ec1390。随后set7/44与真实texture/font/GPU资源链不变。原ConfigWatch loading构造@1405a4d93/95明确false；gameplay@14059eb24读取DevSettings.live_link。两App的watch owner分开。

本轮9文件SHA冻结清单：`skeleton/task-index/work/TG-c459c7f73345e24709/main-chain-boot-20261018/startup-source-freeze-20261018.json`。取证合同及原码位于TG-82359d6f5a5ad5c05d/main-chain-boot-20261018；统一编译、原数值fixture与真实GPU执行由parent记录，本文不把候选接线当运行验收。


## 10. Snapshot/intro history 独立下钻（生产冻结期间）

fresh原init_game_state_from_snapshot@1417a9c50与SystemParam::init_state@1417ab290证明：旧SnapshotPrefix/0xc0头+14个descriptor把R8的真实资源借用包误当文件格式。R8实际为20个ResMut/Events，+0x10是changed Tick、+0x1c是this_run Tick；RCX才是owned LoadedSnapshotData。当前合成0x280B测试不能作为真实快照业务通过证据。

原snapshot应用先加载InnerWalls，再发Roof创建、三类颜色事件、写六栅格、替换Terrain/Decorator/Stairs等owner、发Tree插入事件。当前Rust的Walls parser丢曲线，Roofs/颜色/Trees只trace，Tree命令字段不全，CPUHeight的u16表示与原memcpy(len*4)还未闭合；不能把这些缺口绕过后发ready。完整独立合同和真实原指令位于task-index/work/TG-3adf041a3108edb59a/main-chain-boot-20261018/SNAPSHOT-APPLY-CONTRACT.md。

直接读取发行full/demo intro，只有history.json、无snapshot文件。因此首次新建最小路径应走原缺snapshot fallback，从cursor=-1回放13个墙历史条目。它们含空组与双操作组；当前expand_wall_compound只接受单操作组，真实资产会被拒绝。已有生产replay_wall/ReplayBuffer/InnerWalls写回可复用，但还需要原旧JSON输入→真实History控制请求→回放→高度/颜色/墙发布消费者注册→完成计数与postcommands。不能把“有函数名”当已打通。

旧WallState兼容也有实缺口：原+65是StructureType，+69是ClutterVersion；现source字段名称把二者混淆。原has_skirts和style是同字段别名，legacy字段custom反序列化允许bool0/1，现仅字符串enum入口不足。已按原字段/default/curve3→(x,z)准备wall-save-serde.rs候选，及直接提取的23个真实WallState fixture；它保留owned曲线点列和缓存，不构造native堆指针。候选只做rustfmt语法检查，生产冻结下未集成、未Cargo、不发ready，IsEnclosed额外旧格式仍需继续恢复。INTRO-HISTORY-CONTRACT.md记录了完整边界。

同轮另已按父任务明确授权，实际应用Camera yaw_cache/pitch_cache/position_cache三个原alias，并加入真实intro Camera解析与canonical序列化回归。当前source-manifest.json记录这些后续源指纹，旧冻结清单保留，不声称统一构建全过程源未变。运行验收仍以父任务实际Cargo/GPU输出为准。

后续独立复核确认另一条实断线：原LoadingGameComplete携带16B AppMode，mode_manager读取的正是save::load_game同一事件。当前源码把save owner写成unit、mode_manager另写同名struct，形成不同ECS身份。原loadGame state2的SwitchAppMode postcommand实际发送此事件，不是RequestAppModeChange。已有两文件身份修复候选restore-loading-complete-identity.patch，尚未因GPU冻结写入生产。原async只看事件非空，不能从这个检查推导payload为unit，也不能额外要求“全部墙回放耗尽后再ready”；后续Replay阶段另有原消费者。

### 后续原RNE反证：HistoryCmd第四字节的真实消费边界

旧独立审查以Ghidra表达式推断第四字节必须传播到ReplayBucket，结论过强，现由完整原RNE寄存器消费纠正。history_process@140af5ff2..6015取payload[1..2] word并把byte3放bit16；@140af60ba/6114仅and bl,1，再@140af6119 mov r8d,ebx，尾跳replay_edit@140af6134。replay_edit@140af61f6原封传R8d到generator；完整10371B generate_replay_cmds_from_edit@14143fb50只使用DIL低字节，无其它EDI/DI读取，随后RDI被重新赋为容器/临时指针。故高位不能按常识新增到桶或领域命令。

后续完整writer参数映射证明byte3在入口@140af5e5d读出，并@140af5e68比较3、@140af5e6c相等跳过PlayReplayAudio。早先把此通知误命名NewReplay，现已在生产纠正，详细原证见下节三身份修复。需要修复的是原事件流的清队/即时化/音效策略；“必须增加桶字段存第四字节”从阻断清单删除。原decomp报告保留为审查历史，不覆盖本节新原始指令证据。原RNE完整bounded asm位于TG-c4cdc0aa5e62db9266/main-chain-boot-20261018/{history-process,replay-edit,generate}-rne.asm。

## 11. 原 intro History 主链已集成并通过限定生产ECS验证

本节更新第9/10节当时冻结的候选状态；旧节保留审查经过，不再代表最新源码。恢复以原B RNE和PDB为依据，现有重构Rhapsody接口保持不变。

已写入原WallState serde兼容：保留Curve3的(x,z)、points_u和length，支持has_skirts/style旧布尔别名、FollowTerrain与No/Yes等实际intro格式，恢复+65 StructureType和+69 ClutterVersion的区别。后续已按下节新证据补齐Premade载荷。SavedMultiEdit只接受真正恢复的Wall/WallNewSystem输入；保留空组和并发组，不把未知领域或aux payload替换为空操作。

原History.history_start实际上是i32，缺省-1（1415bd861、1415bd976），不是JSON数组。原计数字段+58/+60由serde清0。LoadingGameComplete已统一为save owner的16B AppMode载荷，mode_manager重导出同一身份，state2逐条遇SwitchAppMode时发送事件；没有加“等所有墙回放耗尽”这一原版没有的ready门。

原缺snapshot回退路径已实装：从选中history文件parent取snapshots目录，逐个尝试最近{id}.snapshot，打不开则移除该引用并重选，最终无快照时以cursor=-1装入真实typed History。相机、TOD、颜色、ID上限、自动保存OpenFile与版本开关来自实际解析数据。状态1按有符号counter发送原0x03010101/0x03010001请求，状态2继续原postcommands。若文件真实打开，现入口明确停在未恢复的容器/领域应用处，不把它当普通JSON或编造空世界。

本轮还修复了真实相机故障：CameraRigParametersCache漏了+0x18的FOV槽，导致原+0x194从pitch_offset(常为0)取值。原serializer六字段跳过+18，原object visitor1415c3f20无条件写45度；现用serde skip/default45保留9个f32槽的36B owned cache。load_parameters读取正确FOV，并按140911b50的已证明Normal分支完成100000秒插值及四矩阵更新。非Normal/active camera arm显式拒绝，未把窄分支包装成完整相机模式恢复。

已恢复并实际注册的调用关系：

| 阶段 | 真实所有者 / 消费者 | 原锚点 |
|---|---|---|
| 启动资源 | History空账本、ReplayBuffer速度1、Pending/Hold/Restore | insert_history_resources 140943e10，速度写140943e87 |
| 地形CPU | 两个512KiB实际readback owner，初始零字节、范围-2.5/24 | insert_terrain_resources 1409457c0 |
| 栅格 | Path的Shared publisher、三Cobblestone的Secondary，共用同一owner | PathsPlugin 140b68e40、140b69097..69199 |
| UI输入视图 | UiState真实inactive构造→EscMenuProbe派生→mode_manager | add_ui 140945d25..d70，名字esc_menu_canvas@142ad8501 |
| set10 | mode_manager→mode changed；LoadingComplete真实模式消费 | runthread 14059fb02，in_set1400d6520 |
| set19 | 地形派生缓存→adjust_wall_height→PublicWalls发布 | add_wall_systems 140493197、140493246，chain335b30 |
| set35 | History命令入口→cursor→clear_previous_state→ReplayMode→finish | 1409438dc/e1、helper141737fb0 |
| set40 | 原并发组wall replay，直接写同一InnerWalls | 140943981，helper140ae3740 |
| set41 | 原bucket pop→真实归档extract_diff→真实Rapier flush→finish | 140943a09，原pop140b38250 |

额外反证：history_clear_previous_state的inline run_unsafe141439d70完整664B仅在有事件时clear DecoratorArchivist+0x30的允许ID集合（141439e7b..ecc），不是extract_diff。生产已按此修正。原cursor还受AnimateWallBulldozeBuffer非空门控制；当前注册消费同一实际动画owner。

生产依赖现在启用已有native-physics/Rapier，回放结束不再走无feature的空flush。DecoratorArchivist使用原Default(false+空map)，两ColliderMaps、IdGen、Deferred事件资源同App所有；不会创建native地址/假的碰撞句柄。非空stairs aux仍有结构连接map清理缺口，实际consumer在弹桶前返回错误。

已新增shipped_intro_reaches_the_same_live_wall_authority回归，直接运行上述生产ECS阶段，原13entry/26内组/22操作最终必须留下27/28/29三墙，分别验证原height位值4.158195495605469、1.5993374586105347、3.0318603515625及11/6/4曲线点数、PublicWalls发布、账本cursor12、空pending/replay。根任务v22已实际执行该测试：1 passed / 0 failed，64轮Update达到上述断言。真实日志为`/private/tmp/tg-original-intro-live-authority-v22.log`，binary为`tiny_glade-9c6c44ffe289177b`。这验证了真实intro的typed History→并发组→同一InnerWalls→PublicWalls生产ECS闭环；测试没有窗口、GPU或完整启动，不据此宣称游戏可玩。Cargo与真实GPU执行始终由根任务串行完成。

仍需闭合：完整motomoto Snapshot容器与typed域apply（旧SnapshotPrefix错误模型不能用）；所有非Wall历史serde与aux领域；墙构造器完整后续消费者；真实UiState交互producer与全部输入阶段；真实音频宿主与其完整消费链；完整世界渲染图和真实几何/GPU producer。加载Logo与WSI已有根任务独立GPU验收，但这不等于世界恢复成功。

原快照路径/容器只读取证另见 TG-06f4abbe570c5f0ee3/render-prelude-lut-20261018/snapshot-path-audit/FINDING.md。原RNE数据流证明文件是u64 tag/u64长度/块payload，首Snapshot块payload才是JSON，后续包含Terrain及截8字节栅格tag。

本轮后续已接原同owner的RT启动：run_game_thread@14059dbef分配唯一Arc/RwLock，@1405a0100插入Glue，@1405a403c以同Arc启动原名bvh_builder_worker。游戏App现只建一份RtWorld、Glue、worker，update_rt_world和upload_rt_world保持原set44未chain，与prepare并列；加载App没有额外worker。其真实SSBO/几何producer和最终世界图仍由主链验收，不把注册本身算作渲染通过。完整锚点见TG-c459c7f73345e24709/main-chain-boot-20261018/RT-OWNER-STARTUP-CONTRACT.md。


### v22之后的已证增量（尚待根任务再验）

set19现在继续消费OnWallChanged/OnWallDeleted，把同一PublicWalls同步到canonical WallSpatialHash；原chain helper140335b30的四资源函数签名与现spatial_hash_update一致。intro测试新增三墙各有空间段断言，此新增断言尚未包含在v22通过结果中。

原gate真实快照的5个Walls记录揭示PremadeShape是`{"PremadeShape":{"params":{"Circle":...或"Rectangle":...}}}`。原1416ec2e0/14157c4c0及字段访存证明Rectangle四字段orientation/width/length/center，现替换ShapeParameters旧伪tag/payload格式，仍复用已有owned Shape/geometry。Circle的rotation缺省0且零值不输出由原1416ec478..4cc证实。WallState现直接消费同一canonical ShapeParameters，保存曲线与缓存；新增真实gate5墙fixture回归，待统一Cargo。

另复核load_game的History计数字段：原14146f54f读History+0x10的entry长度，14146f564写回+0x58。故serde初始清0与加载后写len不冲突，当前loader显式写len有实际原证，不能按初始默认值删去它。

CustomCursor的原16B布局、f32大小、真实managed buffer lease和双App本地EventCursor切换已形成候选；生产GameLoop后续已按下文真实managed lease合同合入，运行验收另记。原TimePlugin会将EventRegistry置Waiting，事件维护还受FixedPostUpdate信号控制，不能泛以“两个App.update后必丢Created”作定论；必须结合实际Startup调用阶段和异步加载循环完成取证。当前未通过伪造重发事件绕过该边界。


两路wall lazy dirty现已闭合原owner：once_startup_walls原wrapper1408c61d0的630a先订阅PathRaster，6322再订阅同TerrainHeightsData.publisher，6339插canonical WallLazySubscriber。publisher原bd2b/daa按此两字段take；生产现已接PreStartup producer与set19 drain。`WallPublicationTerrain`只从同一实际Terrain owner派生，不是第二个发布源。新增Path和Terrain实际发布的限定回归待根任务执行，合同见TG-3594e8acede6203727/main-chain-boot-20261018/WALL-LAZY-SUBSCRIBER-CONTRACT.md。

后续原墙构造链已独立定位：Update set28，chain_ignore_deferred依次update_gates_hashmap、construct_gates、construct_fences、construct_platform_stairs、place_pillars、construct_walls、construct_floors、debug_viz_threshold_curve、structure_outline。原caller14049338e→14033def0，set字节140493393=0x1c，Update.add_systems140493414。现intro回归的authority/publication/spatial不等于这条几何/碰撞/RT构造链已验收；缺失的构造器不能用空函数注册填数。


### 完整writer复核后的必要业务纠正

原HistoryCmd第四字节控制PlayReplayAudio.kind，3跳过音效；NewReplayCommandEvent是真ZST，在实际出队通过discard门后、Undo/Redo bounds检查前发送。原arg6/7/8分别是Audio/NewReplay/Interrupt，现生产已纠正先前的身份误映射。Interrupt在相反方向清队/explicit clear/即时化入口分支发送，未恢复的timelapse consumer仍不伪造。原replay_audio_system@140ad5800独立证明Audio确有byte+8载荷和16B stride，撤销旧ZST疑点。新增quiet3与中断/音效身份两项回归待parent；旧v22没有覆盖这些身份断言，保留其限定通过范围。合同：TG-c4cdc0aa5e62db9266/main-chain-boot-20261018/HISTORY-WRITER-IDENTITY-CORRECTION.md。

CustomCursor的生产GameLoop已合原f32 size、独立mode/last-size、相同App BufferHost的实际ManagedBuffer强lease与事件ptr核对，所有写通过现Rhapsody write_bytes同步API；没有按原ABI重建底层库。换App先撤旧writer并重置五个World本地EventCursor。普通三状态新会话在TimePlugin.Waiting/首次delta0机制下，第三次update至多第一次buffer rotate，handoff先flush再update可读Created；复杂on_load的寿命另核，不补发假事件。原OS absolute写发生在确认ID过滤之前，OS owner负责同一入口调用。完整72producer的真实原PreStartup位置已核，注册/实际GPU写验收仍以主任务结果为准。


### 后续原录制门与事件迭代纠错

原is_recording完整10B@140a86970与history_process入口@140af5cf3读取的是同一History+30 pending owner。原EventReader先在5ce9消费当前事件，再检查录制；若正在录制，Warn后从整个函数返回，未读的后续事件保留，也不进入pending队列消费。没有新HistoryCmd时则直接走5f81的队列路径，原未额外检查录制。此前拆分enqueue/cursor并在cursor_step_expanding手加录制门会改变这两个行为。

已将生产set35及通用CPU注册统一为history_process_system，entry迭代和队列选择在同一次系统调用内；cursor公开API签名保留，删除错误的录制门，所有消费者复用同一cursor业务体。PDB的DebugHistoryMsg参数在完整原body未读，旧16B captured transport不再被描述为第二条原事件流。新增两条事件寿命/录制回归，quiet/Audio/Interrupt测试也转为实际统一入口；当前仅rustfmt，尚未扩大v22运行验收。完整证据与diff位于TG-c4cdc0aa5e62db9266/main-chain-boot-20261018/recording-entry-audit。


### 原PreStartup的四项外链（收紧旧“十链”表述）

add_game_startup_systems@140941110→chain140a97110→PreStartup.add140b67620。完整外链是create_preset_textures→十项startup tuple→load_mesh_atlases→startup_nani；只有外层已证chain，内层into_configs140a2bf40未chain。原MeshAtlasLibrary::register_meshes在1412a00ae被load_mesh_atlases调用。mesh_atlas注册、startup_nani与完整72初始化必须共享此游戏App资源和调度，当前宿主未闭合该整个阶段。

普通Steam入口的固定None、真实backend/pump/语言及本机SDK/client缺失，已另写入根ORIGINAL-FIRST-REVIEW的直接阻碍；本机GPU已验证不意味着普通binary启动通过。


### v28 后续实际CPU验证

根任务已执行新源码：History整组14 passed/0 failed（含quiet/三writer身份和录制中单条消费/无新事件队列两边界）、Path/Terrain同owner订阅1/0、SavedWallState含真实gate五墙Premade3/0。日志为logs/original-core-v28-09/10/11.log，source-manifest记录各log SHA，完整根验证入口core-algorithm-validation-v28.json。该轮证明新增core业务回归实际通过；新的tiny-glade统一入口+spatial整体验证仍待执行，没有把v22旧测试或core单测扩大到完整游戏。


### v30 实际窗口与窄候选交接（后续结果另记）

根任务 v30 新统一 History + spatial 的 intro 生产 ECS 测试实际 1 passed/0 failed（`/private/tmp/tg-original-intro-v30-tests.log`），补足了上文“新统一入口仍待验证”的范围。它依然不是完整构造器、GPU 场景或整游戏验收。

同轮实际 Apple M3 macOS 窗口/OS loop 已进入，但 frame0 后台 gameplay channel Disconnected，进程 exit1；日志是 `/private/tmp/tg-original-window-startup-v30.log`，binary SHA 为 `dc80d71a311c135561ffad4cdb0e1324bc47cdd94a7367d4837bdd082d0c0f3d`。缺失 intro snapshot 只产生 warn 并继续，不能把它推断为当前失败根因。根授权合入的 run_game 窄诊断将 worker 可 unwind 的真实 panic String/&str、phase、已完成 update 次数返回为 Err，不更改资源校验、LoadingGameComplete、warmup 或 ready 门。普通原 panic hook 会 abort 的分支仍可能不可捕获，不能声称此补丁保证暴露所有失败。

Prefab/atlas 独立原证审查发现当前 loader 仍创建空 Mesh/零 AABB，音频 String 长度被误作 kind_tag；原实际 Part 持有 render Arc、subset、独立 audio/RT bounds。外部 owner 正持有相关 systems/resources，未被本任务覆盖。可安全拆出的 atlas RON/属性/19程序网格候选与完整 reload 合同已保存 `TG-1a0d194affaf7f0d17/main-chain-boot-20261018/safe-atlas/HANDOFF.md`；该窄模块没有 PrefabPart 字段依赖，也没有启动注册，不能把候选/模块导出算作完整 producer 已接入。
