# M02 地形：历史、存档、Reload 依赖的独立核验

本轮沿实际源码和原 B 元数据/指令调查，没有把已有统计或完成报告作为实现依据。已读取原路径 `reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/{systems/terrain,systems/history,systems/save,resources/terrain_editing.rs}`。保持现有 tg-terrain 手工重构边界；只修改其既有 `stroke.rs`、core 的地形 JSON 投影和必要调用，不修改渲染引擎、宿主或 focus/edit owner。

## 本轮发现并修复的数据丢失

`TerrainSectionJson` 原两字段带 `serde(default)`，地图键为String，然后 `into_editing_data` 以 `if let Ok(parse::<u32>())` 静默过滤。直接链接修复前country-core rlib的临时入口已复现：空对象、缺strokes、缺counter均接受；非法键和4294967296键都从输入1项变为输出0项。正式Rust回归在修复前实际0 PASS/2 FAIL，日志和临时入口均保留，没有把编译或静态搜索当成此反例。

原 `TerrainEditingData` JSON object visitor为 `1415c5ff0 / 2535B`（PDB module1630 record357004，inline visit_map record357264），原 `1415c64f4` 调用missing_field("strokes")，`1415c650a` 调用missing_field("next_stroke_id")。原 `1415c61c0..61e2` 比较17字节 `next_stroke_index` 并映射同一个counter字段，因此这是合法别名，不能删掉或推断成第二个计数器。现在两个字段都必填，canonical错误名保留next_stroke_id，别名仍接受；map使用既有StrokeId数值serde传播错误，不造自定义容错过滤器，也不把独立map key改成条目内index。

原 `1415c63bd` 调用deserialize_u64，完成后 `1415c6468/646f` 拷贝完整qword。现typed counter已从u32恢复u64，快照编码直接写u64，移除平台usize转换。strokes的ID保持u32，这与原new_stroke和get_last的32位ID读写不同于64位counter相符。大counter不是非法存档：原允许保存但停止发号，不可截断成小counter再发号。

## 原发号、碰撞与最后笔画

原 `new_stroke140924a10 / 302B`：`+38`读取qword，counter<=0x3ffffe才允许创建；先完整64位加一，随后ESI写32位ID。counter>=0x3fffff返回None且不变，包含u64MAX。HashMap insert返回旧项时，原先drop旧值再panic，计数和新条目都已经修改。现保留重构的正常Result/Option调用接口，碰撞在原失败位置panic，不再返回Err被UI的if let Ok静默吞掉。

原 `get_last_mut_stroke140924900 / 258B`：先检查完整64位counter非零，0直接panic；非零后DECL ESI，仅低32位减一，再以32位key查询。现以现有文件内具名 `last_stroke_id` 统一此合同，没有为高counter新增try_from拒绝。缺map仍由重构Result表达实际None，UI消费按原缺失分支跳过。replay更新counter时显式将合法ID加一提升为u64，不改原ID上限。

## 实际依赖与注册顺序

- 工具producer与 `history_record_terrain` 共享同一PendingTerrainHistory类型；旧别名TerrainEditHistoryPending没有建立第二份owner。recorder先take，再History.add，使用同一tg-terrain::schema::TerrainEdit。SavedEdit与ReplayCommand::Terrain也复用此类型，未发现此处重复schema。
- 原 `add_terrain_systems1409455f0 / 454B` 的工具/命令/changed/readback链归set15，history recorder归set34；宿主实际沿该trait安装。History start set14在其前，finish/process set35在其后；replay_terrain在set40，pop/finish在41，save在43。GPU回读、真实heightbuffer与图像验收由中央继续处理，本报告不据此宣布地形全部完成。
- 注册树有一处旧注释与真实原顺序相反：原parent `140943a04` 调用chain helper `141737ae0`，其PDB顶层三个槽依次是九参replay_pop_bucket、九参finish_replay、预先由run_if14098ee10建立的finish_timelapse配置。当前源码pop→finish→timelapse正确，不能按注释中timelapse→pop→finish去改生产顺序。三槽原完整类型已保存 `m02-history-set41-pdb-slots.json`。
- SaveSnapshotWorld真实采集TerrainEditingData与TerrainHeightsData，编码写同一Motomoto Snapshot/二进制Terrain chunk。load_snapshot_file先typed验证JSON；实际World投影先发每个输入stroke的EvTerrainStrokeChanged，再替换权威map/counter，随后恢复高度范围和原raw buffer。新回归通过真正文件编码、文件读取、地形域World投影检查，不注入手工JSON资源代替编码。

## Reload 与卸载

实际同App路径：OnExit(Game) begin_glade_unload发ClearCanvas(false)并关闭旧保存文件；Reload帧Update set33的clear_canvas只受on_event门控制，没有增加会令其不消费的in_state(Game)门；该consumer清strokes、counter与rasterized，再由PostUpdate将Reload转Game，下一帧OnEnter消费真正NextSession。`run_game`全部0..44 stages只是顺序边，没有整个set33的Game门。因而“OnExit参数里没有TerrainEditingData所以未卸载”是错误判断。

clear_canvas原本保留cursor height override与未提交UI状态，不在没有原证时主动增加额外清零。snapshot载入地形后会把cursor override置None并发changed，是否与具体交互跨Reload的Local状态完全相容还需联合运行验证；本轮没有擅自修改清场或宿主。

## 证据与验收边界

RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。8份原完整LLVM范围及每条指令原字节校验、manifest/report均保存现目录。初6项首次采集7 analysis+2version进程约3.037秒；后两次分别仅新增1项，各2analysis+2version进程，不称零成本缓存。

最终收据：

| 范围 | 结果 | 证据与边界 |
| --- | --- | --- |
| tg-terrain完整lib | 9 PASS / 0 fail / 0 ignored | `m02-terrain-counter-final-tests.log`；原边界、宽counter低32投影、碰撞失败前副作用 |
| core snapshot全部tests（native-physics） | 25 PASS / 0 fail / 0 ignored | `m02-terrain-snapshot-final-tests.log`；含严格wire、真实Motomoto文件→地形World写回、显式B资产目录36原快照兼容 |
| 中央提供W3真实GPU收据 | 1 PASS / 3真实帧 | `m02-root-terrain-gpu-validation.json`；M3 Vulkan，4→0→4 passes，checksum 91001741→91001741→91002950，使用原shader与重构渲染宿主；不算本agent原指令差分case |

初次整合被其它owner Modifiers测试中间态编译阻挡，未修改该owner。保留v2的24 PASS/1 FAIL：真正文件另带独立empty Stairs图，测试World未初始化宿主实际已有StairsState，所以loader严格报告缺owner。v3仅给真实文件夹具加入canonical StairsState消费该原payload，未删payload、未忽略gaps、未修改生产loader。源码指纹、原地址/长度与最终日志见 `m02-terrain-wire-final-validation.json`。

本轮没有执行上述原8个函数的完整行为差分，原指令取证与Rust链验收分别计数。中央全App仍被M11具体mesh/source owner缺口明确阻挡；不因地形叶子GPU通过宣称整游戏恢复。

## 下一轮复用经验

1. ID宽度、发号counter宽度、存档字段宽度必须分别核；不可因为当前可发ID有限就把持久counter也截短。
2. alias、default、invalid-key过滤都是业务数据合同；不应为方便读档吞错误或丢条目。
3. PDB tuple的语法槽位比函数调用出现次序更能说明chain顺序。预先构造run_if不证明它排在链首。
4. 保留修复前反例及真实文件/World验收，源码存在、统计表或叶子绿测不等于图像、输入、保存重启全部完成。

## 2026-10-29 原卸载与拖动中的重入验证

`original_session_control.rs::terrain_clear_owner_runs_during_real_reload_transition` 使用受审查权限、本机 MoltenVK 真实设备和实际纹理/SSBO，执行真实 ui_draw 的 phase 3/4 创建未释放笔画；Game→Reload 时执行 begin_glade_unload、clear_toolbar_session、uninit_clearing、despawn_glade_entities，Reload Update 的真实 clear_canvas 消费事件。重入 Game 的第一帧仍运行同一 ui_draw 系统。`/private/tmp/m1-reload-run-v3.log` 1/1：清除 strokes、next_index，原有 cursor override、TerrainEditUiState 缓存和 ScheduledTerrainReadback pending 均保留，且首帧未发生 counter 0 panic。设备环境 `VK_ICD_FILENAMES=/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json VK_LAYER_PATH=/opt/homebrew/share/vulkan/explicit_layer.d DYLD_FALLBACK_LIBRARY_PATH=/opt/homebrew/lib RHAPSODY_REQUIRE_VULKAN=1`。本夹具尚未挂实际 load_game_session、新会话 history replay、未决 GPU 回读完成或直接测量陡度 Local，不能宣称完整新会话隔离；其后新加状态及 pending 断言尚待重测。

追加断言的 v4 `/private/tmp/m1-reload-run-v4.log` 1/1：Game 首帧 `TerrainDrawState.state == 0` 且 pending history 为空。此结果仍不能覆盖真实 OnEnter 会话及旧回读完成后的新会话影响。

v7 `/private/tmp/m1-reload-run-v7.log` 1/1：采用同文件真实 App 构造、OnEnter `load_game_session`/`observe_source`、两个磁盘 session 及 `LoadGameManager` 达 Loaded；真实 ui_edit phase3/4 未释放时，明确捕获旧 hover/height 并由真实消费者修改 falloff。此时 `PendingTerrainHistory` 本就为空，不能称已有 pending 载荷。Reload 后新会话创建笔画并重新捕获、释放，载荷的前值来自新会话。关键事件先 `App::add_event` 后兼容 init；旧拖动累计 0.2、新拖动 0.1 不触发 0.3 提示，覆盖同一系统 Local 在新 phase3 重置。此测试未验证旧 GPU ticket 晚到后的数据。v5 对 source 在同帧已消费仍期待 Path 的断言失败，保留 `/private/tmp/m1-reload-run-v5.log`，后改用 OnEnter 真实 source receipt；没有伪造 source。

W5 异步反馈子链 `/private/tmp/m1-w5-late-run.log` 1/1：三帧真实纹理、原 shader 和六张 ticket 的基础上，独立 publisher 为同一第一帧取得两张真实 ticket，保留其反馈闭包，第二帧两个高度场发布到 CPU 后再执行第一帧闭包；submitted 与两个 readback frame 仍为 2，两份完整 512² CPU 高度未回退。它与会话 Reload fixture 是不同 World，不证明跨会话票据生命周期。随后新增回读字节本身的双场对照，尚待重验。

W5 强化复验 `/private/tmp/m1-w5-late-run-v2.log` 1/1，旧帧闭包晚到后还对两份 `GpuReadbacks` 的完整 u16 内容逐值与第二帧比较；保留同 World 子链边界。Timer 首次和重复插入的显式 0/1000/0 位型断言 `/private/tmp/m1-timer-register-v2.log` 1/1。


## 2026-10-29 M1侧任务：真实跨Reload旧GPU反馈修复

本轮用户要求补四项：跨Reload联合验证、完整GPU上游、计时资源原业务、可见场景保存恢复。未调用子agent，未修改M6算法或自动杂物，不重启共享逆向服务。

**发现并修复真实数据污染。** 原异步反馈仅以WorldId和帧号隔离；同一个World完成新会话加载后，旧GPU票据可以覆盖CPU权威高度。修复前原始失败保存在 `/private/tmp/m1-side-reload-old-ticket-direct-20261029.log`，明确为旧权威高度被覆盖，不能把既有两个World的测试当联合通过。

当前修复在既有异步宿主增加地形来源 `TerrainReadbackEpoch`（SessionId＋State<AppLoadingState>变化版本）：票据、缓存和copy确认共享这一来源。新会话拒收旧票据；CPU消费也验证已存在缓存的来源；旧copy_issued确认位不能伪装为新会话请求。原ScheduledTerrainReadback pending请求仍保留，原两帧软件延迟、每帧最多一对拷贝和两拷贝间发通知的次序不变。该来源校验属于当前并行渲染架构的安全适配，不宣称原B持久映射有这些ABI字段，不修改原地形数学。

**真实同World联合测试通过。** 复用 `original_session_control.rs` 原真实加载/退出/清场/编辑流程，包含两类：加载另一个会话、再次加载同一存档ID（子进程隔离进程级路径）。先保留已发布的旧高度缓存和旧确认位，再保留一组真实GPU票据至Reload之后；断言两份完整512² CPU高度不被覆盖、旧缓存revision不重复发布、无新增脏区或OnTerrainChange。随后实际上传并回读新高度，双完整高度逐值一致，4096脏块均发布，通知恰一次。

统一命令为 `CARGO_INCREMENTAL=0 cargo test --offline -p tiny-glade --lib --test original_session_control --test original_scene_compute_gpu terrain_ -- --test-threads=1 --nocapture`，配 `TG_INSTALL_ROOT=B版本内容`、严格Vulkan及本机MoltenVK环境。`/private/tmp/m1-side-lifecycle-final-20261029.log`：lib过滤6项（其中4项地形反馈）、原W3/W5真GPU子链2项、两类Reload 2项均通过；同ID子进程内部输出1项是同一用例，不重复统计。早期夹具误用cpu-probe专属API的编译失败、沙箱设备失败及同ID遗漏真实加载完成consumer的失败均保留。后者补入原 `check_load_completion_system`，由真实系统解除loading.tmp门控，没有手工删标记或改变生产加载算法。

**另外两项仍被完整App阻断。** 已运行 `original_gameplay_chain::shipped_session_through_actual_app_and_production_world_graph`，`/private/tmp/m1-side-production-world-20261029.log` 在真实系统 `systems/clutter/src/autoclutter/manage_autoclutter_entities.rs:25` 失败：`ORIGINAL_IMPLEMENTATION_PENDING: autoclutter selected mesh/source owner erased (kind=3, key=0x8e53fbdb)`。未删系统、过滤实例或制造资源以通过。故floor/rocky-depth完整场景上游和最终画面、可见场景编辑→保存→重启恢复仍未验收；原W3/W5局部通过不升级为这两项完成。

**MouseActivityTimer取证边界。** 已流式限定检索B procedure-index的85,312条procedure记录，只匹配 `App::insert_resource<MouseActivityTimer>@14153dcb0`；同文件PDB仅构造new inline。原RNE SHA已校验，其TypeId `d540d6b4fbc978a8/cd0fedd0298f5319` 的64位字面量四处均位于该插入函数（14153dcfd/dd07/e008/e012）。types表仅一个IPI StringId，未给字段语义。没有找到业务更新者/typed消费者，**负面检索不能证明全部间接路径不存在**；保留已证0/1000/0构造及注册，不臆造算法、不删资源，该项保持待原证。侧任务沙箱拒绝Neo4j本地socket（PermissionError），未把它当数据库损坏或修改服务；本段原证来自保留PDB和共享Ghidra只读查询。

本轮经验：①同World不等于同会话，同存档ID不等于同一次Game生命周期。②只过滤晚到反馈还不够，旧缓存与旧确认位都要保留来源。③不能通过清零原pending资源掩盖跨会话问题，必须同时证明新反馈仍正常消费。④同ID重载夹具要包含真实加载完成consumer，不能删loading.tmp迎合断言。⑤完整场景遇外域缺口时保留失败，不以关闭系统换取“地形已完成”。源码指纹、命令、失败和剩余范围追加在现有 `m02-terrain-wire-final-validation.json` 的 `m1_side_lifecycle_review_2026_10_29`。
