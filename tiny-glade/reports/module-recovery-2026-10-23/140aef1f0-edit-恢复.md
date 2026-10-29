# M02 普通地形编辑的Widget生产链审查与修复

本轮限定既有`country-core/src/systems/terrain/ui_edit_terrain.rs`原140aef1f0，以及该文件内测试。未修改Mode5聚焦编辑、渲染/GPU读回、保存加载、全局Widget默认参数或其它owner文件。本轮恢复可观察的Widget注册→悬停高亮→真实UI更新→笔刷预览显示/移除链；不声明Mode10全部算法、M02或整游戏已验收。

## 原归属与完整函数

实际先核B骨架同路径，源SHA `53a935ad88c3861ff7e10040e1ea039253fb7f21f9c91bec0e35faf4b84115cf`。原发行RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，匹配PDB SHA `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。

PDB物理函数3764B，140aef1f0..140af00a4。共享Ghidra函数只归属1973B，正文却越入140af0380相邻grading visitor；旧资料由此误认subtype6等控制，不能当完整函数。独占worker `m02-height-edit-20261027`限定原3764B、核无其它函数归属交叠，依据原表142ae404c恢复140aef9c3四分支：140aef71a/140aef9c5/140aefa80/140aefbc0。原范围字节SHA `0cc159e97b1f808142a9a67ddb2d072171dc7e93ebabd404597dcc5be41b12c5`，与分析库同范围一致。

修后C627行，SHA `7c4a047757098df0c1e853094e293e518b10dc1821d947fae7a409783164aedf`。正常关闭本人worker、重开复读后3764B/一项manual switch和C字节完全一致，decompile completed、无timeout/truncation；最后已正常关闭。分析库整体是patched EXE SHA `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`，仅本函数原字节已与RNE绑定，不能把库整体说成原RNE。保留VA命名repair/reopen job/result及`140aef1f0-edit完整原函数.c`。

PDB FunctionSystem实际15组：Mode、Modifiers、Terrain、UiState、TerrainStrokeParamsCache、PendingHistory、Archivist、EditWriter、CaptureWriter、ArrowWriter、HighlightWriter、(TerrainHint/SteepnessHint/LocalSteepness)、CameraReader、(CursorReader/CursorTerrainRaycast)、HoveredStroke。当前重构将效果合并为SystemParam、将ParamsCache与陡度累计承接于既有TerrainEditUiState；未复制原ABI。原Local累计量与当前Resource adapter的所有权仍是后续完整Mode10生命周期审查项，本轮不擅改其它初始化/消费者。

## 确认的业务缺口与修复

- 原140aef8bc..8eb只在hover tag1且phase1、WidgetId相同、HoveredID有效、没有未读CameraMoved时发高亮。旧实现使用`clicked()`（tag>1/phase1），把真正悬停遗漏并错误接到释放。现在使用真实WidgetContext.hovered()==1，拖拽状态仍独立读取。
- 原140aef812..8a1注册四个声音首tag全0；旧实现采用全局默认[1,2,3,3]。现只在地形本入口复用UiSounds::empty，不改变其它Widget的全局默认。
- 原preview+78首字节为`!camera_did_move`，次字节kind11；旧实现两word全0，导致真实UI从不发布该预览。现复用既有16B安全载荷与ui_update真实消费者；相机移动隐藏，下帧无新相机事件恢复。
- 原尾字0x0100：occlude_by_world=false，record_history_when_used位保持1；旧实现遮挡为true。现仅本入口关闭遮挡，保留真实历史位。
- 本系统只读terrain，用原HashMap键迭代替代每帧收集ID Vec；迭代顺序不变，不删原分支/失败检查。此仅消除一项确定的额外分配，不据此宣称帧耗时或GPU性能提升。

TerrainWidgetResponseTable当前已deprecated，ui_edit不读它；没有producer不是本次真实阻断。生产链是UiState::add注册→WidgetContext→原UI状态机。不能因名字和空表再造响应生产器或替代输入事件。

## 原侧与真实资源验收

`140aef812-terrain-widget-native.py/.txt/.json`直接执行原发行RNE的两个有界段，没有算法、事件或条件结果桥接：四组Widget活跃参数写面、192组原hover门。参数覆盖Canvas4/Empty7和相机两值，hover覆盖四tag/六phase/ID匹配与否/Hovered有效与否/相机两值。仅比较原活跃声音tag、preview两byte、volume/layer/size、occlusion/history；不比较未活动padding。原pack可重复运行，已存在证据必须内容一致，不覆盖不同原证。

最终23/23通过，含既有18组有限数值拖动/256值dispatch/真实EditTerrainStrokeCmd消费者，以及新真实Widget注册/hover与release区分/相机/禁用吸附、ui_edit→ui_update→brush_preview三帧链、196组原参数与门条件逐项真实World差分。命令`CARGO_INCREMENTAL=0 cargo test --offline -p country-core --lib original_edit_producer_tests`，保留`140aef1f0-edit-final-tests.log`；源码/原证SHA、完整修表和重开边界见`140aef1f0-edit-validation.json`。

三帧联合用真实UiState注册、真实hover交点输入边界和实际ui_update；没有预先填捕获Arc或伪造OverwriteBrushPreview。真实BrushPreview(11)实体在未移动相机时插Transform并取CursorTerrainRaycast位置，相机移动时移除，下帧恢复；其它kind的Transform确实删除。ui_update按原清本帧注册表，活动Arc留在原owner，声音kind0。此验证从picker交点边界开始，不冒称几何picker、OS鼠标输入、shader加载或最终GPU画面验收。

## 本轮经验

1. “未有producer”的兼容表须先查真实消费者。deprecated表未被读取，不等于真实Widget路径没输入；修错误旁路会继续扩散假资源。
2. 同一UI相位byte必须连同tag核对。hover phase1与drag释放phase1语义不同；不能仅凭clicked字样替代机器分支。
3. Widget默认构造可用，不代表原调用点没覆盖：声音、preview、occlusion及history位要按caller实际写面核定，再沿真实consumer验收。
4. C body小于PDB且越入邻函数时先修CFG。分析库整体patched身份与有界原字节身份分别记录，重开检查持久性，不能靠旧报告统计判断功能完成。
5. 夹具应覆盖真实owner生命周期：本帧注册、UI取得Arc、清注册、下一帧刷新参数，以及事件reader不重放。只断言私有参数不能证明预览实际显示或卸载。

## 2026-10-29 所有权续核

原 PDB 函数系统参数明确将 `TerrainSteepnessEditAmount` 声明为 `Local`，而 `TerrainStrokeParamsCache` 是 `ResMut`。原完整函数和现有代码的 case 1 清零、case 2 增加与阈值分支已复核：本轮将累计值改为 `ui_edit_terrain` 系统独立的 `Local<f32>`，避免其他系统实例或共享资源修改累计值；旧 `TerrainEditUiState.accum` 保留为宿主兼容字段，不再参与生产计算。原 cache 的 hover、height、falloff 仍由现有 Resource 承载，尚未证明与其他消费者完整等价；`MouseActivityTimer` 生产消费者尚未定位。此改动尚待定向 Cargo 验证，不代表 Mode10 或 M02 全链验收。

W5 已有日志 `/private/tmp/m02-height-gpu-run-20261029.log` 显示真 GPU 测试 1/1 通过：三帧六 ticket、双高度场完整比较、旧帧等待和 dirty tile 检查；不覆盖 floor/rocky-depth producer。

定向验证：`CARGO_INCREMENTAL=0 cargo test --offline -p country-core --lib original_edit_producer_tests`，23/23 通过。原完整 C 的 `pfVar22` 是 float 指针，case 1 写 0、case 2 加值；此测试未覆盖 Local 初始化、不同系统实例隔离或退出重入，原构造初始化仍待证据，不能将通过扩展为生命周期验收。

新增真实 World 生命周期反例 `steepness_local_is_per_system_and_survives_frames`：两个独立系统实例各累计 0.2 不发提示，同一实例跨帧再累计 0.2 发提示；重建系统后 0.2 不发提示。命令 `CARGO_INCREMENTAL=0 cargo test --offline -p country-core --lib steepness_local_is_per_system_and_survives_frames`，1/1 通过，日志 `/private/tmp/m02-local-20261029.log`。此测证明当前 Bevy adapter 的所有权行为，并未证明原构造初值；重建不等同模式退出重入。现有代码全仓 terrain 范围内 `accum` 无读者，公开字段暂保留避免破坏宿主 API，不再作为镜像同步。原 B 的系统注册无 mode run condition，原函数自身在 mode != 10 时返回，但原 schedule 替换与 Local 初始化的完整证据仍待追踪。

### Local 初值及参数投影原指令核验

B RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`：`System::initialize` 的 `1408c26bf` 对 `rsi+128` 写入 32 位零。`run_unsafe` 的 `1408c2fd1` 取同一槽地址，`1408c2fe6` 写入调用者栈 `rsp+592`；`1408c30e8` 将 `rsp+528` 的参数结构地址放入调用栈 `rsp+88`，`1408c316d` 直接调用 `140aef1f0`。被调函数入栈八个寄存器并减 728 字节，故其 `rsp+888` 对应调用者 `rsp+88`；`140aef27e` 取结构指针，`140aef30f` 从结构偏移 64 取出指针并保存于 `rsp+200`，即调用者的 `rsp+592`。`140aef70c–714` 通过该指针清零，`140aefdaa–dbb` 通过同一指针读取、浮点加法并写回。结合原 PDB 的系统参数元组 `Local<TerrainSteepnessEditAmount>` 和完整函数 case 1/2，证明此槽就是该 Local，初始位型为 `00000000`。原指令片段、PDB/RNE 身份及图分片来源见 `140aef1f0-local-flow-evidence.json`；图 snapshot `053060f0c5d388b353567d8b`。此证明不覆盖模式退出时 schedule 是否重建。

### 生产资源注册续核

真实 App 的 `TerrainSystems::insert_terrain_resources` 原先未初始化当前承载 `TerrainStrokeParamsCache` 语义的 `TerrainEditUiState`，而依赖宿主另一处初始化。现由该注册入口插入默认资源，保留原插入重置语义；实际 App 测试预置非零缓存后调用入口，验证 hover、height、falloff 初始化，`CARGO_INCREMENTAL=0 cargo test --offline -p country-core --lib terrain_resource_registration_initializes_edit_capture_owner` 1/1 通过，日志 `/private/tmp/m1-cache-registration.log`。该测试不证明原 cache 的所有潜在消费者或持久化链闭环。原 typed ui_edit 系统参数包含 ParamsCache 而不包含 MouseActivityTimer；图搜索仅发现其构造与插入，不能据此证明无其他消费者。

### 磁盘历史链

`toolbar_module_chain/history.rs` 新增独立 M1 fixture：以合成 UiInteraction 驱动实际 ui_draw 创建并延伸笔画，再经 history_record_terrain 记录；第二事务用实际 ui_edit 产生参数命令，由 edit_terrain_stroke 消费并记录。真实 HistoryCommand 经过 replay_terrain 和编辑消费者执行 undo/redo；生产 save worker 写磁盘，另一进程经完整 LoadGameManager 状态机加载，比较序列化历史、笔画点、参数与 next_index，并再次 undo/redo。`CARGO_INCREMENTAL=0 cargo test --offline -p tiny-glade --test toolbar_module_chain terrain_history_disk_fresh_processes -- --nocapture` 通过，日志 `/private/tmp/m1-terrain-disk-test-v3.log`。夹具在测试边界直接提供 UI interaction、射线、SSBO 回读与 History 录制生命周期，不代表 OS 输入、GPU 或退出卸载验收。后续新增了更严格的写入进程逐步数值断言，须再次运行验证。

最终 v5 `/private/tmp/m1-terrain-disk-test-v5.log` 1/1 通过：逐步断言写入进程 undo 的 falloff 0.4、redo 的 0.6；加载后比较完整 canonical `TerrainStroke` 序列化值（points、falloff、index、height_ws）、next_index 和完整历史，再验证 undo/redo。History 的 start/finish 由夹具直接调用，因此验证 producer 到历史载荷及磁盘恢复，不覆盖宿主输入启动录制。v4 前的 shell 修改路径错误，v4 只验证旧源码；v5 才验证完整 stroke 对照。

### MouseActivityTimer 注册

原 B `1409457dd..7f2` 从 `142ab23c0` 载入 `0000000000007a44`，写入两个 f32 位型 0、1000，再写第 9 字节 0 并插入原类型。已在原归属 `ui_edit_terrain.rs` 恢复资源，字段名只表明偏移，不推断语义；`TerrainSystems::insert_terrain_resources` 首项插入，宿主兼容注册仅 init。首次及重复注册重置测试 `/private/tmp/m1-timer-register.log` 1/1。原 ui_edit typed 参数不含 Timer，当前未找到实际消费者；不能由静态搜索为空证明不存在。
