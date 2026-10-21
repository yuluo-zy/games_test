# 水声 Startup / Update 注册原证合同（B，2026-10-21）

本合同解决 root 的真实 App 装配；所有函数范围来自匹配 PDB procedure index，新增包均校验原 RNE 指令 bytes。只登记两个现有 init 和两个现有 consumer，保留当前唯一 WaterAudioGrid owner 与 Native/Disabled 共用 Studio 泛型。工具包成功只证明证据完整性，不等于业务或 GPU 验证。

## 身份与范围

- 原文件：/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- PDB：/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb，SHA-256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
- 以下地址均为原 B VA（image base 0x140000000），没有把 Ghidra patched bytes 当原作。
- 生产修改与 Cargo/GPU 验证归 root；本 worker 没有写生产或执行编译。

## Startup 两个 init 的真实注册

1. 保留原包 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/round12/independent-startup-reload/140941110.asm` 的 `country_core::startup::impl$0::add_game_startup_systems` 入口 **140941110**，PDB procedure **511:24800**。内联 `add_audio_startup_systems` 的原 record 是 **511:25056**，inlinee29220，ChangeCodeOffset116/Length18，host范围 **140941184..140941196**；子 AddSystems inline **511:25080**。因此宿主不是 run_game_thread。
2. host 的 **140941191 JMP140b65bc0**。完整316 B注册函数 PDB **596:24644**明确 `SubApp::add_systems<..., Startup, tuple<tuple20,tuple3>>`；**140b65c4a CALL140b63200** 生成 Startup configs。注册函数和 builder 全路径分别是 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/native-pack/startup-audio-registration.asm.txt` 与 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/startup-configs-native-pack/startup-audio-configs.asm.txt`。
3. builder 的 **140b6341e CALL140985570**。这个421B config leaf 的 **140985623 LEA142ab82fe** 指向原字符串 `country_core::systems::audio::water_audio::water_audio_init_system`；**1409856bb**装入 vtable **142ab7d38**，`+0x50` 原 qword **140982bc0**。381B invoker 的 **140982c7f CALL140873140**，闭合真正 water init。leaf/invoker路径 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/startup-leaves-native-pack/water-startup-config-a.asm.txt`、`/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/startup-invokers-native-pack/water-startup-run-unsafe.asm.txt`。
4. 同一 builder **140b634cc CALL1412ebb30**。这个421B config leaf 的 **1412ebbe3 LEA142c2ffc7** 指向原字符串 `country_core::systems::audio::water_ambience_audio::water_ambience_audio_init_system`；**1412ebc7b**装入 vtable **142c2fa68**，`+0x50` 原 qword **1412eb090**。381B invoker **1412eb14f CALL1409ebf20**，闭合真正 ambience init。leaf/invoker路径 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/startup-leaves-native-pack/ambience-startup-config.asm.txt`、`/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/startup-invokers-native-pack/ambience-startup-run-unsafe.asm.txt`。
5. 两个 init 都属于上述 Startup tuple。此处没有发现它们之间显式链式顺序依赖，不新增排序；grid 与 ambience 由同一个现有 ambience init 一起建立。water init 建立独立 WaterAudioState。

## 演示模式初始化门

- water init原 **14087315d CMPbyte[143634d90],0;140873164 JNE1408733e8**。保留证据 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/round12/audio-loop-init/native-caller-windows.asm` 的首个140873140窗口；当前 water_audio_init_system 已有同一DemoModeEnabled门。
- ambience init原 **1409ebf74 CMPbyte[143634d90],0;1409ebf7b JNE1409ec3ae**，位于任何 loop/grid 构造或 commands 插入前。完整3955B原包 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-ae03b2df699599085c/core-water-activity-20261021/native-pack/water-grid-init.asm.txt`。
- 原 public `_DEV_OVERRIDE_IS_DEMO_MODE::h6f2a49c135369f5a` 实际存储 **143634d90**，导入指针 **14362c098** 的原 qword是 **143634d90**。匹配 PDB/public与字节证据 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-ae03b2df699599085c/core-water-activity-20261021/demo-pointer-identity.json`。没有把这个导入槽误认 profiler；没有仅凭零填充推断运行时固定false。main原1403dcf78会写入该实际字节。
- 当前 ambience init 漏此门；已获 root 窄委派的候选只添加 Option<Res<DemoModeEnabled>> 与开头返回。root已在 run_game 安装现同名资源，不另建全局开关。

## Update 两个 consumer 的真实注册与条件

- 原 `add_audio_systems` **1409411a0**完整680 B，PDB **511:25116**。实际 run_game_thread 的 **14059f828 CALL1409411a0** 可复用 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-b85d349cabeedc081f/oschain-20261018/input-replay-v3/14059d450-full-B-rne.asm`。
- **1409411d5 CALL140985d30**为 water_audio_system.run_if wrapper；**140985ecc CALL14144ed80**构造条件。后者 **14144ee23 LEA142c75a9f** 加载原名称 `bevy_ecs::schedule::condition::common_conditions::resource_exists<country_core::systems::audio::water_audio::WaterAudioState>`。对应原条件类型 `bool(Option<Res<WaterAudioState>>)`。
- **140941290 CALL1412ec640**为 ambience.run_if wrapper；**1412ec7d3 CALL14144e4a0**构造条件。后者 **14144e543 LEA142c75a16** 加载原名称 `bevy_ecs::schedule::condition::common_conditions::resource_exists<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>`。对应原条件类型 `bool(Option<Res<WaterAmbienceAudio>>)`。这两个 resource_exists 名字已由静态原 bytes 直接确认，不只从 Option 参数猜语义。
- **14094131a CALL141451630** 是两 configs 的 `chain_ignore_deferred`；**14094131f MOVbyte[rbp+1d8],0x26**、**14094133b** intern SystemSet、**14094134d CALL141453000** in_set；**1409413a6 CALL140b67280** 加入 Update。0x26=十进制38。不要将独立 music_debug_gui 注册混入这个水声 set。
- 可读原证 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/native-pack/update-audio-registration.asm.txt`、`/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/native-pack/water-audio-run-if.asm.txt`、`/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/native-pack/ambience-audio-run-if.asm.txt`、`/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/condition-builder-native-pack/water-condition-builder.asm.txt`、`/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-5ac8da716b7e89155d/boot-audio-lifecycle-20261019/registration-evidence-20261021/condition-builder-native-pack/ambience-condition-builder.asm.txt`。
- water_audio_system 的 Res<ActiveSession> 是函数参数；此注册完整函数没有额外 ActiveSession run_if。本合同不添加该 outer gate。其它调用边/全局set是否间接限制session属于本合同外的未知，不据参数猜测。

## root 装配验收边界

- `install_gameplay_device_runtime` 共同注册 Startup 两 init、Update 两 consumer，对 Native/Disabled 使用同一 GameplayAudioStudio 泛型，避免只在禁用音频的小loading测试里错挂完整域系统。
- ambience init 一次建立100cell同一WaterAudioGrid与100loopowner；producer实际借用该owner，duck也读该owner；不插默认空grid绕开真实Startup。
- 演示Startup两init均不插对应owner，Update各自resource_exists自然跳过；非演示DisabledStudio也保留原Disabled实例分支并建立合法owners，不伪造成功后端。
- root需要执行候选三ECS场景、现完整CPU构建/真实GPU与长稳；本合同不把证据采集算作这些验证。

## 原始诊断

`string-read-first-failure.json`保留一次只读字符串导航脚本误读虚拟零填充地址的失败。修正为只读实际.rdata静态字符串后，所有collector report均ok。没有重启shared worker，没有数据库写入。
