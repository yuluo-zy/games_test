# systems / resources 原业务恢复与复核（2026-10-18）

状态：进行中。下面是有界业务修复与验证记录；两个目录及完整游戏尚未全部验收。

按实际模块查看：[模块进度图](module-progress.png) · [可筛选详图](module-progress.html) · [结构化快照](module-progress.json)。快照不按文件数、测试数推算完成率。

## 用户永久约束

见 [country-core/AGENTS.md](../../crates/country-core/AGENTS.md)。保留当前手工重构的模块、类型和架构，补功能、修 bug；原版算法更复杂时直接升级现有算法，不重复恢复整个模块，不建立双份状态。

## 范围与版本

初始源码盘点覆盖 systems/resources 的 493 个 Rust 文件、126680 行，见 inventory-before.json。113 个文件含启发式待办或未知标记，数量只用于定位，不代表语义缺口数；没有标记的文件也未自动验收。

本轮执行目标 B：原发行 tiny-glade.rne SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，匹配 PDB `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。任务索引默认 C 不改变本轮 B 身份。C 羊快照仅有明确范围的静态交叉核对，没有全版本行为验收。

## 已整合的业务修复

| 业务链 | 修复及现有架构复用 | 明确边界 |
| --- | --- | --- |
| 羊快照、冻结、实时保存 | 纠正原字段归属；实际 Transform/CritterGoal/Sheep 查询捕获；每帧先应用再解冻；保存读取实时实体，保留 Some(empty)。 | 原查询表已解析后执行捕获/应用；不覆盖出生、完整物种生命周期。 |
| 羊真实运动消费者 | system-critters 重导出现有 core Sheep/Goal，统一 ECS TypeId 和完整手工字段；修 force、超时、XZ 距离、随机方向投影、短路水查询、unstuck 仅清 Idle timeout。 | 有界阶段原执行；完整查询遍历、RNG 和出生仍分别登记。 |
| 碰撞查询 | 最近命中与地形 TOI、miss 位模式、形状姿态、回调停止规则；复用锁定 Rapier 的全部 sphere options，恢复地形 sphere 投影及边界。 | 原指令 oracle 桥接已解析依赖查询；真实 Rapier 单槽结果另验。全局句柄/完整装饰谓词仍待核。 |
| 自由曲线 | 升级现 InProgressCurve：加权平滑、原尾重采样、增量 RDP；Rdp 新增显式原算法入口，保留当前改进版默认接口。 | 原全链及失败 append 已核；实际 draw_wall 入口仍单独盘点。 |
| 历史 Replay 与录制 | 修输入 phase、长按计时、真实模式/命令事件及结束判定；恢复 start/finish/mutable、真实 UI permission、instant reader 短路、实际 archivist/stairs diff 与下一帧 Sync 消费。 | 原327个生命周期案例与真实首次操作链已验；aux pop 的 CPU 入口与并行真实宿主入口需区分。 |
| 地形绘制、编辑、回放 | typed pending 复用现 TerrainEdit；恢复 spacing/XYZ/起笔高度、mapped index、drag payload、New/Edit/Move 动画与回放倒计时；原 Terrain 序列化三变体已接现 EditType schema。 | GPU mapped 资源创建/提交/同步、所有非 Terrain 保存变体未验收。 |
| 地形真实端口 | 纠正 ICF 折叠引发的事件误认；接实际 WaterRaster、PlayTerrainSound、边界反馈、光标捕获、Highlight、Arrow、教学事件与同一 DecoratorArchivist.culling owner。 | 不将事件写入等同 GPU/宿主完整消费者接通。 |
| Arrow 实际动画 | 在既有 ArrowGizmoView/Transform 上恢复 query 内双 reader、跨实体轴门、逐调用衰减及 change tick；不引入第二套组件。 | 原实体出生、render、全局 GPU 顺序仍待核。 |
| 音频生命周期 | init 受现状态资源保护并与 consumer chain；复用 AudioLoop 原 stop(Duration) 策略，19 个实时 stop 调用改传实际单调时间，修常量 ZERO 造成无法停止。 | 全局声音生命周期和原系统总调度未据此接受。 |
| 教学三个消费者 | 复用现 hint 条件/profile/display；恢复 Terrain tag0/tag1/tag2 与 Steepness 规则，使用实际 hover 标签；配置入口执行真实 completion consumer。 | 宿主尚未发现调用该新配置入口；不称完整游戏教学可用。 |
| 预制体热重载 | 原调用按48字节选取并转发同一PlacementData；现手工helper只收Transform并造默认数据，丢失id/custom/animation/bend且cull归零。现API直接收完整既有PlacementData，局部投影现Transform供RT/audio，GPU/cull保留实际输入。 | 原resolved-row64+callee64及独审逐bit相等；完整query/filter和真实宿主caller未验收。 |
| 岩石mapped遮罩 | 原32768字节被当32768个u32，修为8192个；修设备name为TerrainRockMask。 | 原5构造fixture和实际library边界回归，GPU全局生命周期仍未验收。 |
| 输入移动量与地形右键聚焦 | 修 CursorMovedScreenSpace 为 analog0 移动量，保留现 anchor/clamp/capture；聚焦在现 UI/raycast 上发送真实 RequestAppModeChange+hint2。CPU startup 复用现 mode_manager 与真实 UI projection。 | 其他 ui_edit_focus subject/退出分支未恢复；并行完整宿主已有 mode set10，未重复注册其模式系统。 |
| 墙旁/区域树隐藏与路径清树 | 在既有 WallSpatialHash、TreeAnimation、PrefabInstanceData 上恢复专用网格查询和严格距离；纠正 PrefabInstance 为现48字节placement。路径清树跨模式消费全部事件，最后有效事件生效；使用实际placement而非缓存位置，保留原失败策略。 | 原84次callback/61距离fixture与实际World回归通过；原query nominal类型名仍有绑定歧义，实际数据面已核；完整树出生/渲染另列。 |
| 树历史、保存与回放 | 将运行时u64 storage index与placement i32 variant分离；恢复原外层index/tree或index/from/to保存格式、实际回放、完整已知Insert字段，Move/Delete读写现placement。恢复Move→Insert→Delete顺序。 | 原serializer/replay静态逐字节独审与真实typed消费通过；未新增整原entry执行。预置树转换其余过滤、树弯曲/地形更新仍待恢复；不伪造上层prefab出生。 |
| 普通墙/栅栏绘制与结束 | 复用现InProgressCurve/InnerWalls/WallState，接UI、nursery延迟建墙、原smoother、静止点权重、颜色/高度/音频端口与两组墙历史。静止权重原roundf曾误认ceil，独审后纠正。接入现core CPU入口并补真实首次UI→录制→结束提交→下帧Sync。 | UI原320案与smoother原163次执行已验；原draw动态entry曾fetch-unmapped，不能宣称整entry原执行通过。现有墙续画、反向和合墙正在挖掘；普通路径CPU World闭环已通过。 |
| 墙色完整ID与初始化 | 沿用现拆分字段，三个reader恢复完整u64 WallID、原查找/边界顺序；初始化仅补缺失颜色表，避免每帧覆盖已有值。 | 颜色专项8项与core/graph通过；不代表完整颜色UI和GPU上传链。 |
| 三类mapped mask | Water/Exclude/Garden沿用现canonical类型，修为u32×8192、原短名称；真实typed library/map/readback接口与mock GPU回归通过。 | 独审原构造字节与字段；真实GPU提交/完成/CPU接受时序仍待闭合。 |

## 复核中撤回和纠正的旧结论

- ICF 折叠的 grow_one 容器符号不证明事件类型。完整 PDB SystemParam、frame 写入端口及真实消费者证明原地形音效是 PlayTerrainSound，不是 StairsPlacementBucket；旧类型归属结论已撤回。
- 同一 +1e0 写入属于实际 DecoratorArchivist.culling.allow_any，不属于 UI.active；未建立镜像 bool。
- sphere 的解析 TOI 不是 GJK 容差契约。回归改为严格比较锁定依赖同一 QueryPipeline 的结果，并保留 maxTOI/target_distance 的业务断言。
- 新建 RecordingRg 的首帧 temporal history 无效；旧断言已据原标志位纠正。Nani buffer fixture 的并行修复不归功于本轮。
- Bevy 同一 publisher 在 Input 和晚期各注册一次时，函数类型 `.after(...)` 并不能区分两次注册，会触发 SystemTypeSetAmbiguity。已用零状态 ModeChangePublication 标记晚期现有 publisher，三个消费者依赖该 set；实际编辑图初始化与执行已通过。
- hide_trees 的旧“缺失可见性数组”解释已撤回：原写入是 Bevy change tick 与现有 TreeAnimation.hidden_this_frame。继续沿用现组件；树 placement、runtime storage index 与 variant id 的旧混淆正在修复，未接受完整树链。

## 验证记录

最终验证在进行中，所有 Cargo 结果均需结合 *-sources-before/after.json 和 source_drift 阅读。

- critters-native-tests：4 个库测试 + 5 个集成测试通过，包括实际 critters Sheep/Goal 进入 core 实时保存查询。该次漂移仅为独立音频文件。
- utils RDP：4 项通过；羊快照早期独立回归 3 项通过，最终组件统一后还将复跑。
- core-lib-tests-r2：早期快照 2164 通过、4 失败；失败已定位为 sphere 依赖契约和过期 Nani/temporal fixture，待最终复跑确认。
- core-native-final / core-native-r2：整合期编译失败已保留日志，分别修事件 derive/Res 访问/可变 library，以及并行 WallShapeSnapshot 测试改用现 bytes() API 比较；不计通过。
- onboarding-native-tests：首次构建因 ENOSPC 失败；onboarding-native-r2 已通过 33 项，source_drift=0，覆盖真实 hint/profile/completion CPU 消费。完整宿主入口仍未验收。
- core-native-r3：编译失败，14处Arrow空Vec测试类型已修，其余是并行PlacementData/RT/GPU接口旧调用。详见core-native-r3-assessment.json；运行期未执行。
- core-native-r4：source_drift=0，2351 项通过、3 项过期 fixture 失败。热重载完整48字节、实际聚焦→mode、core startup真实UI保留、Arrow、rock buffer和实时音频均通过。失败诊断与只改fixture的纠正见core-native-r4-assessment.json；全crate复跑尚未通过，两个external target因lib失败未执行。
- editing-and-sheep-r2：发现上述 Bevy 类型 set 歧义，已修；原失败报告保留。
- core-native-r5：2357通过、1个旧 shape fixture 失败，运行期仅独立宿主文件漂移；fixture 的零方向矩形违背原不变量，按实际轴/宽高改正。
- core-native-r6：新生命周期测试的8处 Bevy 测试API用法错误，未执行运行期；已在测试中改用当前依赖 send/run 及实际借用规则。
- core-native-r7：2365个库测试、1个真实编辑调度测试、3个羊快照测试全部通过，source_drift=0。包含首次实际UI操作录制/释放提交、instant短路reader、327个原生命周期案例，以及完整placement热更新。此报告接受限定CPU范围，未升级为宿主/GPU/全模块验收。
- tree-history-r1：当前核心2370个库测试与system-tree 10项全部通过，source_drift=0，CARGO_INCREMENTAL=0。含61原距离bits、真实墙旁隐藏/释放删除、跨模式多路径消费/下一帧无重复、64位index原保存格式、六方向回放、Move/Delete实际placement及原同帧reserved-query失败。核心新增计数亦含其他owner渲染测试，不归为本轮独占成果。
- ordinary-wall-r1：2382个核心库测试和真实编辑调度图1项通过，source_drift=0。包含320原UI fixture、原smoother bits、roundf反例，以及空registry墙/栅栏首帧到提交账本/下帧同步。没有手工预先开始录制来替代该新增生命周期验收。主链独立重跑intro→墙authority/spatial也通过，日志 /private/tmp/tg-original-intro-v37-cpu.log；该日志不接受本轮复杂续画或全部游戏。

- color-wall-r1：2384核心库、真实编辑图1项、system-color8项通过；仅宿主game_loop/world_lighting漂移，核心/颜色源hash一致，限定接受见color-wall-r1-assessment.json。
- masks-r1：2386核心库测试通过，source_drift=0，58.168秒；新增实际三mask typed storage factory / readback接口测试均执行通过，三mask均已独审。数目包括已有及其他owner测试，不计新增完成率。

有界原指令验证分别保存于 task-index/work 的合同中：羊 capture80/apply128、movement16/direction8、freeze15、sphere40+terrain12、curve16+1、terrain curve24/release3、初始高度36+独审6、dispatch256/drag18、真实 event ports7/desired bits36、Arrow56+2多帧、audio6+6边界、hint42+hover9、focus20、input两个inline共20+独审4、prefab resolved-row64/callee64、rockmask5构造。数字描述各自 fixture 范围，不汇总为恢复完成率。

## 本轮后续整合

- 已合入 Delete 独立 restored_height 与 lossless 存档，Some-source undo 仍保留捕获高度；续画 false 完成族、Pending 一次 drain、真实 suppression 及先 remove 后 color 的失败顺序已经双独审。core本地 finish 改在 adjust 之前，遵循新取到的原 set19 内顺序；本地首次 UI/start 适配仍不等同原全局调度。
- Bush canonical 改为 [u32;2]×9601、原名称、None 无初始上传、真实 mapped storage；上层直接读取同 AssetSsboLibrary，移除无人填充的假 Resource。恢复 header/count、raw unsigned排序、low32 hash XOR emptyhash，空格初始hash为0。上层实际注册与8专项测试通过，source_drift=0。下游 populate 中另一个 BushGrid/unused consumer 仍待闭合，GPU completion/coherency 未验收。
- TerrainEditingData 增加同owner runtime rasterized，提供 map+bool 借用API，快照恢复初始false。没有ActiveStrokes镜像。该最小接口已独审，实际W3渲染consumer由主链owner接入。
- continuation-r1：2404通过，1个新增serde测试fixture非法style3失败；仅改fixture为既有saved支持的HalfTimbered2，保留生产拒绝。graph因lib失败未执行。漂移仅为独立上层clutter整合。continuation-r2复跑核心2399项及真实编辑图1项全部通过；仅独立宿主world_terrain.rs漂移，核心/terrain/clutter当前输入均与run after指纹一致，见continuation-r2-assessment.json。测试总数也受并行W3旧录制测试替换影响，不作为完成率。
- 端点UI/reversal20输入候选、共享Sphere/publication flag36输入候选、实际merge与Rocky canonical资源仍分别审查/挖掘，不计已整合通过。

## 尚需推进的闭包

普通新墙/栅栏CPU链已通过；复杂续画/反向/合墙、独立Delete恢复高度候选待审/整合；Bush实际mapped消费者与构造正在恢复；完整宿主地形/教学接线；GPU mapped buffer 的创建与同步；Arrow 出生；ui_edit_focus 其他分支；CPU replay aux owner；其余 EditType 保存 schema；实时保存 History 的宿主数据来源；树的上层prefab出生、弯曲和地形变动消费者。其他模块仍按初始盘点逐链审查，不能因本轮修复称其完成。

## 沉淀与协同

本轮可复用规则见 [RECOVERY_LESSONS.md](RECOVERY_LESSONS.md)，用户永久架构约束已写入 crate AGENTS.md 与项目 PROJECT_MEMORY.md。已将事件/保存/调度/首帧验证方法增量写入现有 tiny-glade-recovery 的 references/original-driven-integration.md；继续复用 native-rust-reconstruction / native-evidence-pack，没有另装重复技能包。与宿主恢复 owner 的接口交接见 [COORDINATION.md](../main-chain-recovery-2026-10-18/COORDINATION.md)。主链资源构造新增待办见 [PENDING_RESOURCE_PRODUCERS.md](PENDING_RESOURCE_PRODUCERS.md)。

## 并行、工具与缓存

保留其他任务的并行修改；只按冻结 hash 和必要增量 hunk 整合，记录 before 快照和测试漂移。完整宿主 tiny-glade/src/run_game.rs 的并行 mode/history/wall 接线不归功于本轮。

沙箱原指令 JIT 因执行内存限制 SIGILL132，相同只读脚本经批准宿主执行通过。共享只读 Ghidra 后返回 Corrupted BufferMgr state，保留失败并复用原 RNE/匹配 PDB/已有证据，未改共享 DB。工具连接不计业务恢复。

磁盘清理均有报告：临时独立构建缓存已删除；首次旧测试清理实际删除0文件；后续只在无 Cargo/rustc 且持 Cargo 独占锁时清过期增量目录、24小时前 country_core 临时 rcgu.o。未删除源码、原发行二进制/PDB、逆向证据或活跃构建缓存。逻辑大小与实际释放空间分别记录，不能混用。

## canonical producer 与共享 API 后续验收

- Dormer 双真实类型 / draw aliases、StairsCounter canonical mapped/pass alias、Lantern / Grounded 本地 None 与原NAME已独审并整合。新4个Dormer与3个counter/instance回归均实际执行。
- producers-r1：2401核心库测试+1实际编辑图全部通过，所有workspace源 before/after无漂移。数字含并行owner测试增删，不当新增成果或完成率。
- wall-ui-api-r2：country-core/system-tree/toolbar/system-roof/system-decorator五包全部目标check通过，完整workspace跟踪、源码无漂移。r1漏掉四个上层调用点的指纹边界已单独保留，r2完整覆盖；不将工具改进计游戏恢复。
- Sphere中心与公开墙bulldoze派生字段API已就绪；实际publisher hook、端点交互真实首次录制闭环未验收。root未覆盖main-chain-boot的claimed publisher。
- 通用render/ssbo.rs被其他合法owner占有；没有抢改generic None策略。Stairs使用局部原4字节零初始化+returned-buffer长度检查。音频独立StairsRasterValue和真实GPU同步仍是后续consumer缺口。

验证后独立owner继续修改resources/rt.rs；两个run的before/after仍均0漂移，但完整最新core/RT不据旧二进制接受。producer/UI相关源保持匹配，当前差异已列在assessment。

## Rocky canonical 真实资产与构造闭包

已在现`resources/rocky_terrain.rs`修复原JSON字段、3个signed i32字段、48B/32B完整GPU打包和固定容量148794/49598。保留当前load→Self及CPUVec API，增加唯一RockyTerrainMeshVertCount(usize)供原Startup和W4/W5借用；不复制SSBO库。原29完整范围30002B/7082指令与实际31MB资产已双独审，147输入冻结独立保存。

rocky-r1在两处测试assert_eq的cast_slice目标类型遇E0283（0漂移），仅显式补u8后rocky-r2实际5项通过/0失败/0忽略/0漂移，包括完整实际资产经过同AssetSsboLibrary的两缓冲上传记录。source SHA22dc994d38a50af2d7a241f636042b27e12f02c85bfd6b6062be5f04ff9c051d，详见rocky-r2-result/assessment/log。不是原指令动态执行或真实GPU验收。

原stage为Startup八元组slot4，PreStartup相似签名是color init；完整startup_terrain其余逻辑、actual loader/count插入与W4/W5 canonical改接仍由主链owner推进。72producer现由主链落盘挂PreStartup，不与Rocky资产loader合并为同一阶段。新保存原证由主链os worker接管canonical，root保持不并发编辑input/save。

## 后续端点、保存和快照真实闭包

endpoint20+emptyarchive9已双独审合入现三源：发布时bulldoze门、两个endpoint sphere、真实first/last选点与reversal、continued draw、false finish/Pending一次drain。两处terrain None仍保存Some(empty) first-before，不能后补真实桶覆盖。local helper新增同WallContinueState/PublicWalls/MainCameraMoved事件初始化及UI→真实history_start→draw→finish→deferred→Record→history_finish。新增实际first-input四组合test未预启动History。原endpoint动态执行/full merge/signifier/GPU仍未接受。

endpoints-r1：2416通过/2失败/0漂移。新增endpoint/emptyarchive/首次续画全部通过，旧fresh首次操作因新增dependency注册缺PublicWalls失败；已补同owner资源。DoF独立owner test也失败，已交接。r2根路径拼错相机event失败已修，r3因主链新Prefab测试HashSet collect推断失败，失败均保留；r3生产lib编译完成。endpoint-graph-r1真实编辑全图initialize1项通过/0漂移。r4在Prefab owner修复后重跑，暂排除已知DoF独立失败，未预填结果。

History pending-ID与ReplayCounter正式候选已完成原证和独审，未由root写main-chain-boot claimed生产：见history-snapshot-owner/frozen-inputs.json（12inputs）及两个patch。重要纠错：TryFrom原37B将id0/1判Err，>=2才resolved；ledger应持共享Arc<AtomicU64> ID，与tag9 writer同owner，而非SnapshotCaptureState域数据镜像。原真实保存需要Motomoto Snapshot/Raster/Terrain压缩chunks，现snapshot.rs schema/encoder和DecoratorStorage非空snapshot正在按原证修复，完整保存/加载尚未接受。

## 保存三批实际整合及统一回归

Roof139+Circle rotation、DecoratorStorage69+转换/事件最小v2、Snapshot61+testv2+WallIdv3及现motomoto依赖边已同批增量合入。未覆盖其他owner新增drain/Eventcounter源码；上层DecoratorOpEvents wrapper已alias同唯一Deferred。Butterfly定义仅下沉core并上层reexport，字段/算法未变；DebugGui必要field仅owner patch未生产，等待相机enum同批。

snapshot-batch-r1实际2454 core lib +1编辑graph全部通过，0失败/0忽略/0过滤。仅独立gpu_generate_draw_lists.rs在run中漂移，其他全部workspace源匹配；不能据本次CPU测试接受该GPU新源码。新非空14域/真实chunk编码、Roof schema、canonical Event/实际counter和bucket/ID生成器回归实际通过。随后snapshot两个helper传真实DecoratorIdGen最小delta已合，正在有界复测；完整host文件load目前仍明确Err，不会假恢复DefaultInfo。

结果见snapshot-batch-r1-result/assessment/log。现实际域收集→Result writer→后台文件/blob hash→同History pendingID及完整反序列化/World写回仍需后续整合，不能把此编码库回归称完整保存读回。
