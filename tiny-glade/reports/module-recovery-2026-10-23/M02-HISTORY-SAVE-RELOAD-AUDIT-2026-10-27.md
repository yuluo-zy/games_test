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

## 2026-10-30 新进程图像分段诊断（Sol，只读复核）

复用中央真实完整 App/GPU 结果 `D:/game/tmp-merge/m1-positive-time-save-restore-v4.log` 及 `m1-positive-time-image-diagnostics-v4/`。本增量没有运行原作、原机器指令或 Cargo/GPU，没有改中央冻结的 `original_gameplay_chain.rs`，也没有将静态诊断计作新恢复或验收。

父 `m2-saved-state.json` 与子 `m2-reloaded-matched-state.json` 的所有顶层字段逐值一致，唯一例外为 `app_mode: Terrain → Editing(None)`。最终图像 30,738/129,600 像素不同，RGB MAE 0.2578935185，最大通道差81。父/子实际 ShadowMaskMeanSsbo 分别为 bits1060574941 /1060575643（0.7151926159858704 /0.7152344584465027）；该值经原 `lerp(old,mean,clamp(1-exp(-5*dt)))` 更新，零dt冻结差异。现有 canonical `RequestAppModeChange(Editing(None))` 修正与自然计数对齐尚未实跑，不能称该修正通过。

**W24 已有独立分歧，不能只归因于最终曝光/TAA。** 两份 `*-terrain-lighting.rgba` 在512²图上有1,340像素不同，包围盒 x62..372/y74..427；733像素最大通道差>5，最大140。该输出原绑定为两份地形高度、shadow atlas预设0x11、ColorLibrary与FrameConstants，当前地形lighting pass无时间历史与ShadowMaskMean输入。现有两份完整高度及FrameConstants已严格一致，但没有原样捕获这帧shadow atlas与实际GPU ColorLibrary；所以首个差异的精确来源仍未决。AppMode对齐可消除不同工具状态的部分输入，但未运行之前不能证明足够。

下一次受中央串行窗口控制的GPU执行应一次捕获同一公共绝对帧，保留最终严格断言，并按以下边界排查：

1. W23后三层shadow atlas深度原字节＋actual ColorLibrary owner/upload字节；W24 lighting原格式字节。W24存在差异时优先回追此分支，不能以增加TAA观察帧数掩盖。
2. W34后depth/gbuffer，W42 screen-shadows与SSGI temporal输出，W43 deferred color。
3. W51 taa_input（DoF后），W52 TAA输出及W54最终taa，最后T4 MenuBackground与实际ShadowMaskMean。第一个持续分歧结合其输入判断来源，后段差异不重复算缺失。

精确诊断优先复用现有 `surface_comparison.rs::raw_capture` 的当前安全图协议：原source handle→声明0x25 texture range→GpuToCpu TRANSFER_DST buffer0x34→export0x26→实际receipt映射同Arc→等待完成→read_bytes。该函数目前仅首层/首mip，shadow atlas须显式覆盖三层，不能误称首层等于完整atlas。也不能在提交后按token直接复制临时/可别名图像，届时内容可能已被后续pass复用。通用截图shader会sRGB编码、clamp与RGBA8量化，只适合可视辅助，不能用于证明HDR/depth/G-buffer原字节一致。

建议修改范围需中央解除/协调所有权后再动：测试文件仅 `run_m2_production_frame` 及M2专属捕获helper/父子循环；图声明挂点为现有 `world_graph.rs`、`world_lighting.rs`、`world_postprocess.rs` 的只读诊断分支，复用现有诊断承载。不开新renderer，不更改存档schema、counter、temporal有效性、原算法、rhapsody/utils或正常pass。当前仅提出精确改动合同，尚未实施。

### canonical-mode v5与有界SSBO诊断准备

中央解除M2专属段冻结后，先运行已存在的canonical-mode修正＋原mean/lighting诊断，没有为仪表推迟基线：`TG_INSTALL_ROOT=D:/game/tmp-merge/shared-chain-assets/Tiny Glade TG_M2_DUMP_DIR=D:/game/tmp-merge/m1-canonical-mode-save-restore-diagnostics-v5 CARGO_INCREMENTAL=0 cargo test --offline -p tiny-glade --test original_gameplay_chain m02_actual_terrain_history_save_new_process_reload_and_image -- --exact --test-threads=1 --nocapture`；移除FMOD_LIB_DIR。日志 `D:/game/tmp-merge/m1-canonical-mode-save-restore-v5.log`。北京时间17:45发起，共享Cargo锁等待约三分钟，9分12秒后编译完成。二进制SHA256 `04529b3879a831360ed1df9411a126809bbd9d398a24ab69bb543efaa2ae1958`。只读进程核对确认父PID13636→Cargo22856，子PID24488属于本轮；中央所见同名进程并非据此推断外部并发GPU。

父端通过实际编辑、Undo/Redo、GPU双高度恢复和保存worker，并捕获到`Editing(None)`、total_secs24.266667932271957/frame_count2433、实际mean bits1060575134。v5最终失败，370.75秒（child内部50.88秒）：child `M2_HISTORY_INSERT_START cursor_before=12 entries_before=13` 后真实Dormer Holes上传540,000B报`owned upload admission capacity exhausted`，`systems/decorator/src/dormer_holes.rs:121` unwrap Device。没有到子端匹配图像，故本轮不能判断canonical-mode修正后的图像一致性，也不能把本轮失败计为canonical模式失败。

新增但尚未编译/实跑的诊断完全留在同一测试文件M2专属段：`M2RenderInputOwner/m2_render_input_owners/m2_dump_render_input_bytes`从真实已注册lease选择ColorLibrary/LeafParticle/ParticleEmitter/SheepAnimation/BrickTransform；提交receipt先核实际物理Arc与pass访问，再等完成读取。总raw上限32MiB，超大allocation只保留明确未捕获的元数据，不截前缀冒称完整输入；墙砖约192MiB，已有独立有效记录范围probe继续保留。`TG_M2_RAW_INPUTS`仅在父子M2调用处选择既有诊断设备（额外TRANSFER_SRC），不修改M6 helper。额外在原有自然对齐到frame128的那一帧捕获父/子SSBO，无新增帧、无counter赋值；最终严格图像断言不变。CPU dump增加当前ColorLibrary完整bytes、真实粒子/发射器count、羊runtime（不含跨进程无意义实体ID）。rustfmt解析通过；本增量未执行Cargo/GPU，未修改任何生产渲染文件。

v4原始RGBA转换为无损PNG的只读视觉核对显示父子飘叶位置不同；这是定位线索，不是完整因果证明。256个1/60正dt对初始mean的理想残留约5.4e-10，远小于v4实测4.18e-5差，提示持续输入差异，不能仅盲增曝光等待。生产W18粒子shader确实读取旧位置、frame_index与delta；父子最终时钟相同不证明积分历史相同。原leaf drawable没有DontCastShadow，故粒子也在W23阴影列表中，能影响W24，但仍需上述实际SSBO收据确认这两帧的差异。父图像还早于SaveGameCmd；保存循环继续推进App，而Sheep保存三个权威字段，需区分capture状态/落盘状态/加载后继续运行。合法动态状态不得混列地形缺失。

中央请求v5后交还Cargo/GPU用于M6统一补丁；当前源码已冻结，不自动连续占窗口重跑600秒。后续扩诊断由中央重新串行安排。

v5后修正M2测试宿主：新增`m2_tick_production_app`，真实App::update后调用既有`bridge.prepare_test_execution`推进pipeline/buffer/texture上传；父TOD等待/save循环、子TOD/Reload循环四处均改用该helper。M2自己的Undo/Redo等待也使用同helper，保持原游标、ReplayBuffer、PendingHistoryCommands、非Replay模式及完成后两帧稳定检查；未修改共享M6 helper。原m2_settle两处已泵的循环统一复用它。没有增大队列、清待处理上传、过滤Dormer或改变生产系统。最新文件窄范围修改保留中央同文件M6 typed增量；rustfmt解析通过，未进行第二轮Cargo/GPU。2026-10-30北京时间18:05前源码已冻结交中央统一编译；本修正仍待实际全链复验。

### 原动态输入的静态边界复核（不作为像素放宽依据）

本段只读取原RNE/保留原ASM及shader SPIR-V，没有运行原作或原机器指令。重新核验`tmp-merge/original-tiny-glade.rne`全文SHA仍为f507a6…2b7d03；原restore1416f24f0/773B的SHA `cf958db610d50283b07bf633d97b621e19ac1e2b6a6bfc6978086aa7e5a350fd`、move142318fa0/5774B的SHA `2931cee02c39da92f677887dd8eb816b3b2f955d7d06f82423c89693661d202a`均与既有原证manifest一致，不依赖patched导航库字节。

- **羊快照只恢复三个权威字段。** 原capture1412f064f..066f读取Transform+10/18、CritterGoal16B、Sheep+30/38；原serializer1416f9eb0只写position、goal、move_state三项。原restore1416f2784..27d0实际写Transform+10/18、Goal16B、Sheep+30/38，未写Transform+0四元数、scale、Sheep动画+10/14/28/48/4c/50/54/5c。原件复用`evidence/core-recovery-20261015/critters/0x1412f03f0.asm`、`0x1416f9eb0.asm`与`evidence/round12/history/sheep-abi-independent-review/0x1416f24f0.original.asm`。这不是“羊不保存”，也不支持扩schema补动画字段。
- **朝向更新无条件收敛保证不存在。** 原move142319df4..f20从水平位移取目标方向；方向无效/为零跳过rotation更新。有效时142319eb5读取旧Quat，142319ee3..f1d执行带符号选择的旧Quat加权、归一化和写回，权重来自1423193c8..d4的`powf(0.01,dt)`。因此静止羊的旧rotation可以原样保留；移动羊的平滑亦不能仅凭固定等待帧数宣称逐bit相同。该结论针对原控制流，不声称已证明当前失败像素全部由羊产生。
- **步行动画包含独立相位。** 新从原RNE静态解码14231aa29..abc1：状态1/2读取并累加Sheep+5c的dt、fmod1.25，再以`floor(accum*24)+id*20+3`模30写+28帧；状态2另累加progress并最终设state3。原表14304b0f8的state0/3直接跳回主循环，未重置accum或帧索引。既有`parallel-critters/B-decompile/14231a630.asm.txt`缺这两跳表handler，因此本次直接从原RNE补看，不能把旧截短ASM当完整状态机。实际draw closure1409afcc0读取该帧索引、pet幅度、control flag和PrevTransform；`sheep.raster`原提升VS再用animation_frame选择静态动画库顶点。静态SheepAnimationSsbo相等不代表实际pose相等，下一raw须联查CPU sheep/walk/transform与draw输入。
- **落叶是旧GPU记录递推，零dt也不等于冻结。** 实际安装`compiled-shaders/_leaf_particles.cs.hlsl_cs_main_0.bin`与已有oracle逐字节一致，容器SHA `fca92500d7ad1dbde8b011093586af65d0b2bbfd5f0fe8bf5b85b605e6b73221`；按容器字段解析出的13,134word SPIR-V SHA `cd4c94d61e010fd6d3224f492d5539914b27d81fa13b35ed19f58aa7af42cecf`。原`%184=OpAccessChain`指向当前LeafParticle槽，`%185=OpLoad`后最终`OpStore %184 %1299`，并从真实emitter槽读取重生点。原提升分支`leaf_particles.cs.hlsl:653`的ground XZ drift无dt乘数，`:731`还改drift_pos.x；v4 capture的theme_id=2，不是theme4关闭分支。故父子相同末帧clock/counter，不足以证明其此前不同数量zero/positive GPU帧产生同一粒子状态。当前图像可见落叶位置不同仍只是定位线索，待same-owner原字节确认。
- **树木风动与上述递推不同。** `clearing/tree_canopy.raster.hlsl`、`tree_trunk.raster.hlsl`原提升VS使用当前game_time_sec/frac和实例记录直接计算正弦弯曲，没有类似LeafParticle的读旧位再写回累积器。Weather.wind_strength也是实际tree draw输入；现有v4已证FrameConstants一致，但Weather及实际实例记录还没capture，所以既不能把风动笼统记为“未存档缺失”，也不能先排除所有树木输入差异。Weather非冬季更新不修改wind_strength，仍应读真实owner确认。

为下一观察补齐原draw读取项，已准备**未apply**的小补丁`D:/game/tmp-merge/m1-runtime-input-observation-addendum.patch`，只增加CPU dump的PrevTransform与Weather。普通`git apply --check`因当前CRLF上下文失败；`git apply --check --ignore-space-change`通过。源文件持续冻结，本段无新增Cargo/GPU执行。没有降低严格像素要求，没有把合法动态合同或未证因果混列业务缺失；B全部函数复核仍保留后序。

落叶零dt结论另用已安装spirv-cross对同一原SPIR-V重新转译确认（stdin以`-`输入，不写renderer/shader或执行GPU）：原输出`_440 = _205.xz + (...).xz * 0.1`与后续`_441.x = _440.x - (...)*0.0056*...`均无delta因子，与现有lifted HLSL一致。首次误用该版本不支持的`--stdin`参数返回usage/exit1，改用文档指定`-`后exit0；这只是静态读码，不计游戏恢复或新测试通过。原动画跳表16B也直接由RNE解码为`14231a729,14231aa29,14231aade,14231a729`，进一步确认state0/3保持帧索引而非重置。

## 2026-10-31 跨进程最终画面：生产修复与原版非确定性实测

本轮使用正式 App、发行资产和本机 MoltenVK，保留 `m02_actual_terrain_history_save_new_process_reload_and_image` 的整图零差异断言。**修复了实际所有权／内容依赖和测试错帧问题，但严格整图仍失败，不能声明 M2 或任意运行态截图恢复已验收。** 独立只读审核 agent `m2_image_independent_review` 逐项复核原 B 类型签名、机器码、原 SPIR-V、修复 diff 及采集同步；审核没有运行 Cargo/GPU，以下运行由主 agent 完成。

### 已落地的修复

1. **保存和截图绑定同一 App 帧。** 原测试在截图后才发送 SaveGameCmd，等待过程继续推进羊的位置和移动计时。现在在捕获帧前发送真实命令，要求 set43 同帧同步 flush 并收到该存档的 SavingGameComplete；落盘 Sheep 的 position／goal／move_state 按查询顺序逐浮点 bits 核对，日照三个 f32 的逐位断言保留。没有改存档 schema 或补存原版未保存的动画状态。
2. **动物 RNG 恢复原资源写访问及两条独立流。** 羊、蝴蝶两系统、萤火虫、鸭目标更新由 Res＋Mutex 恢复 ResMut，移除调度器不可见的锁；鸭出生、离场、轨迹规划改回同一个 DuckTrajectoryPlanner 内的独立 PerchanceContext。B 插件 `14067d5b4` 的原始状态 `0x66e610699` 对应 perchance 0.4.0 seed `0x33730834c`；共享动物目标流仍为原始状态 `0xc0df`／seed `0x606f`。原 tuple、chain_ignore_deferred 边界、分支、抽样先后及 Vec 容量复用均未改变。原证为 `evidence/parallel-critters/B-firefly-public-related.json` 的类型化 FunctionSystem 符号、`evidence/round12/startup-critters/plugin-config-name-evidence.json` 与 `14067d440.original.asm`，不能把八个系统统改为同一 RNG。
3. **Filter GI 补齐真实内容依赖。** 原 blur shader 读取 `temporal_stats_out_tex` 的计数来选择滤波步长；原 SPIR-V 对它有一次 ImageRead、没有 ImageWrite。当前图原先把 native 0x37 当无需旧内容的写，缺少保留声明。现仅增加 `preserve_storage_texture_contents(TEMPORAL_STATS_OUT_TEX)`，保留原 0x37、描述符、绑定、uniform、dispatch 和 shader 数学。真实捕获的四帧中，Filter 前后的统计纹理原字节均未改变。

精确分段采集复用现有 screenshot_capture::Requests 和渲染图：源 image 0x25 → GpuToCpu buffer 0x34 → export 0x26；成功提交时核 GraphId／export／目标物理 Arc，统一等待该帧一次后读取全部结果。16 个 stage、64 MiB 固定上限。瞬态源在生产边界立刻声明 copy，不能提交后按退役 token 猜源；persistent shadow atlas／terrain lighting 则持有同 receipt 的真实 preset owner，覆盖三层深度／全部五层 mip。诊断 feature 与额外 TRANSFER_SRC capability 不改变普通构造的描述符，不替换 shader、实体或业务输入。

### 两个全新进程的严格分段对照

冻结同一份真实存档、使用同一个可执行文件，分别经过独立临时账户的 Intro → 生产 Reload → 原输入／dt／提交序列。末帧均为 frame384，实际 GI `history_valid=true`、`allow_non_determinism=true`、`ShaderRefactoringState.disc=0`。CPU dump 的全部记录字段一致，真实 Lantern instance_count=0；不能把其它未采集字段也泛称全部输入已证相等。

| 真实捕获内容 | 原始字节大小 | 两新进程不同 u32 word 数 |
|---|---:|---:|
| GPU ColorLibrary | 592 | 0 |
| LeafParticle／ParticleEmitter／SheepAnimation | 26,624／17,600／853,120 | 均 0 |
| 三层 shadow atlas | 17,280,000 | 0 |
| terrain lighting 全部五层 mip | 1,396,736 | 0 |
| GBuffer／depth／velocity／geometry color | 1,105,920／552,960／1,105,920／552,960 | 均 0 |
| Eval screen radiance 后 | 829,440 | 0 |
| Reproject GI 后 | 829,440 | **36,467** |
| Reproject GI 统计 | 138,240 | **485** |
| Filter GI 后 | 829,440 | 111,574 |
| deferred color | 552,960 | 24,927 |
| DoF 后 TAA 输入／TAA 输出 | 552,960／552,960 | 20,811／21,262 |

最终 480×270 RGBA 有 **16,329／129,600 像素不同**，最大通道差 4，RGB MAE 0.0526054527；实际 ShadowMaskMean 两边同为 bits1060712785。**首个观测分歧位于 Reproject GI，已排除这次末帧的地形高度／光照纹理、阴影、GBuffer、深度、速度、GPU 调色板、曝光均值和叶片差异作为首发点。** 之前帧的全部时序历史并未逐张采集，不能将本表升级为所有历史输入的穷尽证明。

原版 `gi/reproject_screen_radiance.cs.hlsl` 在 `updated && allow_non_determinism` 分支中，对同一 dispatch 内的 output／stats storage image 做 5×5 邻域读取并原地写回。独立审核从原 SPIR-V 确认读写指令存在，ControlBarrier／MemoryBarrier 均为 0；这是原版明确允许的非确定性，不能通过清零纹理、双缓冲改写、强加同步或关闭分支来宣称忠实恢复。实际安装容器与原 oracle SHA 完全一致：Reproject `ba4f117f62426f66b00a2be1ca79f5d44259ad7f4895328f11710f033391e113`，其 SPIR-V `8bedd322f9df10fbd11bc43cdbfa096e91d73451b93c8b7219833b551bf14a19`；Filter `1ab4c5120fa042b8eeeaf7f3452d3504eeb88299fe46b88cc47ee74592256b4e`。本轮没有重新运行 Windows 原作，原侧结论是原容器／机器码静态证据，实际 GPU 为 Rust 生产消费者执行同一发行 shader。

### 命令、结果及证据边界

- `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo test --offline -p system-critters --lib`：**36/36 通过**，包含实际调度的 RNG 写冲突、鸭 planner 独立流、音频完整构造和跨帧容量保留。新调度断言修复前已真实失败于 update_butterfly_states。日志 `/private/tmp/m2-critter-rng-before-exact-20261030.log`、`m2-critter-rng-after-20261030.log`。
- `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo test --offline -p country-core --features surface-compare-test --lib eval_gi_screen_radiance::tests -- --nocapture`：**7/7 通过**，日志 `/private/tmp/m2-gi-content-contract-tests-20261031.log`。
- `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo test --offline -p rhapsody --lib rg::test_image_copy::tests -- --nocapture`：**3/3 通过**，日志 `/private/tmp/m2-raw-image-copy-tests-v2-20261031.log`。首轮新 shape 夹具漏 sampled capability 导致 1 项失败，已纠正；没有放宽原描述符校验。
- `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo check --offline -p tiny-glade --lib`：普通非诊断路径构建通过，日志 `/private/tmp/m2-image-default-check-20261031.log`。实际 GPU 测试前另以相同变量执行 `cargo test --offline -p tiny-glade --features surface-compare-test --test original_gameplay_chain --no-run`。
- v4 实际 GPU 主命令：发行安装根、严格 Vulkan 与本机 MoltenVK 环境下，`/private/tmp/m2-image-root-20261031-v4/original_gameplay_chain m02_actual_terrain_history_save_new_process_reload_and_image --exact --nocapture --test-threads=1`，设置 `TG_M2_DUMP_DIR=/private/tmp/m2-image-root-20261031-v4 TG_M2_RAW_INPUTS=1`。**父目标 213.41 秒失败，内含子目标 106.80 秒失败**：保存、完整历史／游标、笔画／counter、同来源双 512² 高度和日照 bits 通过，保存前运行帧到重启帧仍有 34,839 像素不同。该比较还包含原版未保存的叶片／羊动画运行态，不能算严格图像通过。
- 冻结存档再次启动同一子入口，指定 `TG_M2_SAVE_CHILD=1` 及 `frozen-recovery.json` 里的 session／save／recovery 路径；独立进程耗时 **86.20 秒**，仍保留其与保存前帧的失败。随后用原 bytes 对比两个子进程，得到上表，不将两次失败包装为整项通过。
- 实际二进制 SHA256 **`ca758178909d5417f21e8daa7238bea5612f5d977cef75bf29be5bcc30320087`**；源码 SHA、构建记录、与基线的 patch 在 `/private/tmp/m2-image-root-20261031-v4/build-manifest.json`。共享工作区在运行中由其它聊天提交为 `5566d2cc9`／`39d013a7d`；本报告以可执行文件和逐文件 SHA 绑定实测，不把其它工作域算作本轮验证。
- 完整原始输出、三层／五 mip manifest、两个子进程比较 JSON、冻结存档在 `/private/tmp/m2-image-root-20261031-v4/` 和 `...-v4-repeat/`；日志分别为同前缀 `-gpu.log`。临时目录是本机可复核工件，并非已入库的大二进制。v1／v2 失败及 v3 诊断自身重复等待错误也保留，未记作通过。
- `cargo fmt --all -- --check` **未通过**，首先命中未改动的 `country-core/build.rs` 等既有格式；strict clippy `-p system-critters --lib --no-deps -- -D warnings` **未通过**，报告现有 61 项 lint。未改原浮点分支来迎合 lint，也未将二者标绿。日志 `/private/tmp/m2-image-fmt-check-20261031.log`、`m2-critter-strict-clippy-20261031.log`。本轮没有 Release FPS、4× scaling 或长稳性能结论；正常热路径未新增实体状态复制，RNG 改为 ECS 可变借用并移除锁。

**当前裁定：** 真实缺陷修复已完成并取得上述有界验证；普通原版 GI 不具备普遍的跨进程逐 bit 画面确定性，原存档也不承诺保存任意动态运行态。原整图零差异用例继续保留为未通过，M2 不据本次结果升级整模块验收。若后续要求无条件整图一致，需要单独约定确定性渲染／完整运行态持久化合同，不能混作原版缺失算法或偷偷改变默认行为。
