# Tiny Glade 项目记忆

## 2026-10-11 更新工具与 C 小宠物试跑

详见 [工作流结果](evidence/toolchain-pet-pilot-2026-10-11/RESULTS.md)。新增 native-evidence-pack、native-tool-batch、native-module-handoff，更新原重建技能与 Cargo 公共依赖对照脚本；技能安装于本机 ~/.codex/skills，项目快照/哈希已保存。实际 Ghidra SDK shared 连接与有界批量通过；B patched 分析上下文不作为 C 原指令 oracle。

C 羊抚摸选择行切片已恢复五状态、原变化 tick/being_petted 门控，独立 Bevy 0.17.3 消息/Query/派生输出/结束与删除链运行；120 原指令例与4独立检查通过。最终公共依赖锁113/113与原C匹配，首轮漂移和失败交接保留。仍为104B观察源投影；真实UI消息发送、Sheep创建/ID、GameTime、完整walk/renderer与原调度未闭合；完整critters/building未验收，旧firefly NaN失败不改。下一步原UI生产者与权威Sheep源优先，不能停留在门控投影。

## 2026-10-10 三版本计划入口

当前后续执行计划为 [THREE_VERSION_RUST_PLAN.md](THREE_VERSION_RUST_PLAN.md)：A 首发辅助追溯，B 维持当前实现/验证基线，C v1.16.0 为计划默认最终交付目标。源码恢复优先，场景/GPU后期必做。以下依赖和已验收结论仍限定于 B；各版本独立记录身份和依赖，新增计划未切换当前工程锁文件或扩大已有验证范围。

Bevy 分叉范围待按三版核查：A 的 git checkout 线索与 B/C registry 0.16.0/0.17.3 来源已有静态登记；来源类型不能单独排除补丁。保留用户提供的分叉说明，T02 用构建/调用证据核实后更新，不能把旧说明无条件套用三版，也不能仅凭 registry 宣称没有分叉。

## 用户已确定的目标与边界

- 原作研究路径：`D:/game/ljxsj_92385/Tiny Glade`。分析目录：`D:/game/reverse-engineering/tiny-glade`。Rust工程：`reconstruction/Cargo.toml`。
- 最终交付完整 Rust 游戏实现；当前先恢复源码、类型、状态与 CPU 算法。场景回放和 GPU 研究后置。
- 不分析用户自己的 demo。不得把已有通用 Rust 依赖也当作游戏算法重新逆向/手写。
- 引擎为 **Bevy 的分叉**：这是用户明确提供的项目事实。随包 Cargo.lock/PDB 独立确认 `bevy_app/bevy_ecs/bevy_tasks` 等为 0.16.0；分叉代码仓库、具体commit及补丁范围尚未核实。不能据此猜测完整渲染等同上游 Bevy。

## 原作依赖基线：直接复用

B 的依赖基线是原作 `build-info/Cargo.lock`，SHA256 `9e0cb4a0fa173122fa7cd552aa639f5ee977abc22c74aec92398808c707184bd`，901 个 package（多版本分别记录）。不能用最新版本覆盖这份基线。

| 依赖 | 原作使用版本/来源 | 当前处理 |
|---|---|---|
| Bevy family | 0.16.0；引擎分叉按用户说明 | 复用；只研究游戏调用及必要分叉改动 |
| glam | 游戏core/roof/utils用0.29.3；锁文件另有0.20.5/0.25.0/0.27.0 | 复用0.29.3数学API，不还原glam实现 |
| fastrand | 游戏core/roof/utils用1.8.0；另有2.3.0 | 复用1.8.0，不能误选2.3.0 |
| half | 2.4.1 | 调用f16转换，不手写转换器 |
| serde / serde_json | 1.0.200 / 1.0.85 | 复用；逆向的是游戏结构/格式和消费者 |
| ron | PounceLight/ron 0.8.1，commit `2787d8c05250c4dd706dfb358a2e9f16969d2968` | 复用原fork，不重写RON解析 |
| rkyv | PounceLight/rkyv 0.7.39，commit `031d58108ccf8c0e125b06b017f72b4afd67e9a3` | 复用原fork，恢复游戏存档schema |
| winit | PounceLight/winit 0.29.15，commit `d03de0a0a881be9422c360083151b0e7a8a2582e` | 复用原fork，窗口/平台研究后置 |
| slotmap / smallvec | 1.0.6 / 1.13.2 | 复用容器；恢复游戏ID和字段语义 |
| ash | 0.38.0+1.3.281 | 复用Vulkan接口；GPU后置 |

全部 package及来源应保存到 `tracking/dependency-policy.json` 和 `tracking/original-Cargo.lock`；原锁文件用于比对，不直接替换重建工程自己的锁文件。

## 排除范围与待归属范围

1. crates.io、公开git依赖与Rust标准库：读取公开源码/文档并使用匹配API，不反编译还原其实现。
2. 无source字段的52个path/local条目不能一律当作游戏独有代码。FidelityFX、XeGTAO、驱动/分配器等先确认已有上游；不要因为路径是local就重新实现。
3. `country-core`、`system-roof`、`system-wall-constructor`、`system-decorator` 等游戏模块为主逆向对象。游戏自有 `utils::CurveU/random_splits` 可以恢复；其中调用的glam、RNG、f16仍复用依赖。
4. 反编译器展开的Vec/Arc/ECS/Serde泛型胶水仅用于理解游戏数据与调用边界，不能计作游戏源码恢复成果。

## 已完成与证据界限

屋顶曲面/Curve2、脊线、圆形与矩形CPU铺瓦源码已存在。此前临时手写的随机数、f16转换、四元数/向量运算已改为原版本库调用，重建工程6个依赖包的版本和checksum与原锁文件一致。替换后重新验证：矩形33输入/19024条记录、seed和最终RNG一致；圆形37输入/11467条记录逐字节一致。具体见 `evidence/roof-tiles/dependency-boundary.json` 和 `dependency-verification.json`。

新增墙高约束数值后段与3个完整高度缓存叶函数，通过6624个机器码案例。约束前段的对象查询、特殊flag生产者仍待恢复，不能把局部数值规则记成完整墙系统。

新增 `roof/edit_rules.rs`：已解析Roof/Event的编辑后段、变化量、change tick、gable重建和数值反馈，通过4514个原指令案例；tip长度限制复用glam公开API。原ECS解析/调度/失败/队列扩容未恢复，不能记成完整屋顶编辑系统。记住height Absolute下限0、Delta下限0.1；eave/ridge/tip Absolute保留raw，Delta才限制；变化量不是实际最终差值。细节和边界见 `evidence/source-edits/findings.md`。

发行树/PDB未取回作者原Rust文本；当前是原机器码约束下的行为恢复源码。不能声称获得作者原始源代码、完整ABI或整个游戏完成。

## 工具与推进方式

- 使用 `native-rust-reconstruction` skill；复用已有 Ghidra/PyGhidra、LLVM PDB 和原指令验证证据。
- 2026-10-11 再复验后保留默认 `ghidra` 与 `reva` MCP；共享读14/6工具，专属worker写19/10工具。ReVa扩展及依赖已补齐，PyGhidra脚本/P-code、签名修正、并发和持久性通过。REA/radare2恢复为按需SDK入口；radare2补pdg后端但PDB TPI兼容仍不足。旧Windows RE-MCP已在Mac恢复3.0.3，并打GhidraProject.save兼容补丁，双数据库/双客户端及持久隔离通过。复验总报告在 evidence/toolchain-revalidation/summary.json。
- MCP 使用 `tools/ghidra-projects/mcp/TinyGlade.gpr`，为B现有数据库的独立持久克隆；不重导入/全程序分析、不抢占原数据库锁。副本上的注释/签名改进与原数据库分开，原RNE继续作oracle。显式指定 `program="tiny-glade.exe"`。
- 多agent可直接使用 `/Users/yuluo/.codex/mcp-tools/reverse-operator.py`，每修改agent唯一 `--worker` 并 `--write`；共享读默认，自动独立clone/backend、按PID绑定、独立结果文件。保留native-rust-reconstruction与单份pyghidra-scripting skill；CTF等重复技能不恢复。
- 复用持久Ghidra数据库；按原作函数/引用/类型检索，不全程序重复分析十万条符号。
- 从高收益游戏规则和权威状态链推进：Roof编辑消费者/字段映射 → Wall形状/高度/生成 → 历史提交/撤销消费者 → 门窗附着等。
- 里程碑与源码验收分开记；依赖复用排除在逆向覆盖统计之外。

## 三版本 Rust 工程实施记录（2026-10-10）

- 基线/符号候选/模块源路径索引已落 `tracking/baselines.json`、`version-map.json`、`module-coverage.json`；候选同名不代表跨版本行为等价。A/C 分析数据库仍待绑定，B 复用现有数据库只读。
- REA 过滤后29工具已实际通过 MCP SDK连接；使用已有 Node24/Ghidra12.1.4/JDK21，没有改 Codex配置。原 instruction oracle 直接用 B RNE。
- `tg-history` 解析 A/B/C 98份历史、733条屋顶命令；Roof参数/VisualEdit/离散回放已恢复。B真实VisualEdit到已解析编辑核心708例通过。创建/删除消费者、Flat几何、对象查询和历史事务未实现完毕。
- B Width=0/Length=1已由原序列化证实；参数块 rim 字节+25及padding保留，缺失默认值未知。
- ARM编辑核心初次43例NaN符号差异已修复，当前4514例全部通过；该修复只处理确认的游戏算术边界，glam仍直接复用。旧全铺瓦报告属于历史Windows构建；扩展workspace待重新全链验证，不冒称当前主机全游戏通过。
- 当前registry依赖从6包扩展至15包，version/source/checksum均对照B原锁；不能用这个结论代替features/fork核查。
- 98历史/101快照跨版本去重50/52；B22缺失快照仅2个同ID候选，尚未确认可替换。rhapsody2/shabriri匿名访问失败，只登记访问未解决，不据此判断仓库不存在。
- 验证报告和边界集中于 `evidence/multi-version/B-parameter-findings.md`。保持源码恢复优先，GPU与场景回放后置；最终目标仍为完整工程。

## 2026-10-11 依赖筛除与建筑业务联接

用户明确要求三版本对比筛除库/依赖函数，聚焦完整建筑系统。指导已落DEPENDENCY_FILTERING.md及两级AGENTS.md，机器索引tracking/dependency-exclusions.json。B/C分别849/907外部来源包登记复用；A用PDB registry/git路径；local包先核实。共享private source或泛型名称含游戏类型均不自动决定归属。

新增building/wall_state.rs（游戏读取guard和状态投影）、roof_creation.rs（原create初始化）、roof_spatial.rs（wall→roof同步），分别对应2659/848/512例原指令更新/创建/联接通过。PublicWalls内的哈希算法、Bevy实体表/query/事件容器未重写；返回观察值/typed业务通知，由调用者提供已解析对象。CLI明确resolved_only，create不spawn。

原B/C Roof::new构造器字节相同；A后部字段偏移不同。禁止据此宣称整个B/C建筑系统等价或将B布局套A。style与shape变化同时发生会发两条有序kind2通知，不能去重；Roof::ty为false时related组件写pivot y偏移，为true时写零。

下一步恢复PublicWall发布/实体关联、create后段游戏组件与注册、删除/方向/Flat消费者，再进入墙砖/支撑/开口/门窗和完整历史事务。具体范围与报告见evidence/building-state/findings.md。仍无完整建筑/游戏验收。

## 2026-10-11 建筑生命周期第二批

新增wall_publication（发布数值/确认字段及缓存失效）、roof_associations（真实wall_roof_hashmap业务策略，std HashMap复用）、roof_components（TileSim/动画）、roof_deletion（匹配/移除意图）、roof_lifecycle（显式标记提交边界）。2316原指令案例、128生命周期联接、A/B/C共384 TileSim leaf差分通过，当前cargo4项测试及release构建通过。

重要规则：MissingWall走清理分支，不是一律错误；阶段13/14限制。有效shape的已标记roof仍保持关联直到移除；未标记重复关联失败。发布清除max-y缓存flags，不能填造缓存默认值。删除由已提交标记及动画第一通道ordered零共同决定。库HashMap/API边界按报告声明，Bevy/Arc/Vec底层没有重写。

当前是发布/关联/创建后段/删除的业务核心增量，未完成dirty/terrain gathering、缓存生产、资源ownership、真实注册调度、drawable/spire/support与TriggerVolume元素schema。证据见evidence/building-lifecycle/findings.md；不能把此批局部验收称为完整建筑系统。

### 真实 Bevy 建筑运行时增量（2026-10-11）

新增crates/tg-building-runtime，registry bevy_app/bevy_ecs0.16.0，113个依赖身份匹配B。真实App/World组件注册、EventReader/Writer及Commands/ApplyDeferred接入发布→缓存→创建→关联/空间更新→CPU瓦片→标记/动画/despawn→清理；5项多帧World测试、整workspace9测试通过，可运行building-run合成JSON场景。原作调度/分叉完整graph未验收，不能写成已经恢复原Bevy fork。

wall_cache.rs恢复circle/rectangle缓存producer（init_wall_height_and_arch_segment_height，A/B/C各1024原码例）及B gable第二最大值最终store（256例）。state尺寸/extraoffset三版不同；B基础最大值来自轮廓max(-2.5) + constrainedextra -0.46并floor0.960000038。逐arch gable profile明确pending；不能把typed base arch heights当成完整山墙拱段。RoofSpire/Support为确认的ZSTmarker；先前以transform零写猜allocation不可采用。Trigger stride36/36/24按Vec语义复用；Nani只接确认初始rank/tag/transform/创建discriminant。

所有当前参数/回放/创建/更新/生命周期报告已重跑，详见evidence/building-runtime/findings.md。扩大ARM矩形CPU流水线33组/19024 records发现case14单个filler Y相差1ULP，当前host报告ok=false，原因未证实；Windows历史报告仍须区别。功能场景通过不等于全部CPU位精确或完整建筑验收。下一批优先raw发布采集/逐arch gable/完整required-components、屋顶方向和Flat消费者，再向墙砖/支撑/开口与门窗历史联接推进。


## 2026-10-11 原始采集、逐拱段山墙与 Nani 注册增量

原 raw Curve2→resample→地形映射→相对高度→发布→基础缓存→逐arch gable→屋顶CPU/Nani→删除清理已接真实Bevy建筑链。64×64原空间格覆盖已恢复，TerrainDirtyTiles可自动从raw memberships找候选墙，移动/删除清理旧格。完整B创建10组件及空required图已确认，Named/AtlasPart策略和Roof StoneFloor CPU64 record已接World请求。当前17个workspace测试与六帧raw CLI场景，证据见 evidence/building-gather/findings.md、building-collection/findings.md、building-nani/findings.md。

更正旧解释：墙+10为raw点数，+30/+28c为raw Curve2.length；Nani AtlasPart tag1/index0不是DrawableRank，Desc+16 kind9不是生命周期tag；Bundle24字节Desc+48字节Transform+ZST，80为align尾padding。required Self注册回调不能推断自依赖。旧段落相反描述由本条替代。三版比较继续排除registry/std/ECS实现，共享private几何仍恢复业务。A terrain固定bounds，B/C动态且带fix；C Nani kind7、新增wall/color/margin输入，A upload48，不能直接套B。

下一批高收益：Roof方向/FlatStyle实际消费者和历史事务进入现有World链；freeform缓存/墙砖列→支撑与开口→门窗附着。C行为迁移单列，不与B验收混合。GPU/fork排程仍后置；ARM瓦片原case14一处1ULP失败不冒称全通过。


## 2026-10-11 专用skill与屋顶编辑链

native-rust-reconstruction用户skill已创建并校验；按需参考覆盖三版依赖/原模块映射、原指令验证、Bevy链与token控制。具体地址/字段仍留本项目。新增roof_direction/roof_customization，真实Query无删除过滤已由PDB参数确认。方向同值仍通知geometry/gable/sound；Missing解析失败通过fault停止该消费者。Full Flat style与rim分开，后者是replay tag4；等值仍写/tick，变化才通知wall holes。Rim from来自cmd，record1即使同值仍产生history mutation。

B方向1024、B/C customization各1536及各4skip guards通过；八帧CLI及新增5个多帧World测试通过，整workspace22项。Create/Delete/Direction/Rim离散replay实际World分发及Delete undo重建已接；History requests为adapter待处理，不宣称原History commit/cursor/snapshot事务恢复；wall holes仍仅通知未生成几何。全部证据见evidence/building-edits/findings.md。

下一步：RecalculateWallHoles消费者接墙/门窗；History add/commit/cursor和VisualEdit实际World消费者；freeform缓存、墙砖列/支撑/开口。C Flat/Rim两个resolved消费者已执行验证，C Nani及完整迁移仍pending。旧ARM瓦片1ULP失败保留。


## 2026-10-11 开口消费者与Rim事务闭环

RecalculateWallHoles实际reader在墙publisher，以InnerWalls存在检查转OnWallChanged，非直接写hole；当前已接到四类门窗actual raw贡献与WallHoleStorage跨帧/周期/排序/缓存及删除清理。原finalizer caller使用源Curve2长度且shape tag!=2闭合；不要猜3D长度或仅0/1闭合。位置和资产collision为明确resolved输入，原asset/UV/roof-aware morph、其他hole生产者及视觉Bundle/GPU仍未恢复。

原History start/add/finish/cursor和Rim命令展开恢复，真实World完成显式账本→Rim录制→commit→instant undo/redo→分叉截redo→门窗更新。启动账本不默认猜，未配置保持adapter队列；原token Drop不commit。其他MultiEdit owners、快照装载、保存、VisualEdit实际World和timed调度仍未关闭。C Replay判别值与B不同，A历史布局不同。

当前24项workspace测试、十二帧新CLI链；B invalidation512/storage768/opening3072/history462分别通过限定原码范围。范围与三版出处见evidence/building-holes/findings.md、building-history/findings.md。下一步优先asset collision/Decorator宿主投影、其他开口贡献与墙砖裁切消费者，以及其他历史owners/快照联接。C迁移、GPU/fork排程及ARM瓦片1ULP问题仍单列。

## 2026-10-11 完整建筑规划审计（未实施）

用户要求先规划并创建多个agent并行挖掘，父agent整理。三线审计见evidence/building-planning/{walls,roofs-decorators,history-schedule}-audit.md；后续攻关计划BUILDING_SYSTEM_RECOVERY_PLAN.md及tracking/building-system-plan.json包含G00–G19和逐包退出条件。只新增规划/候选索引与文档，本轮未改生产Rust、未重跑原码、未扩大系统验收。

关键纠正：construct_walls是已证finalizer caller，下一步沿AL后真实输出恢复主体；construct_brick_columns定义在elevation support/pillar，不未经CALL认定为主墙brick loop。regular cache的tag>=2/freeform仍UnsupportedShape；空间索引只验收ID membership而非段metadata/query。Roof TriggerVolume一直空创建，calculate_roof_trigger_volumes与UV→position是屋顶装饰真实定位前置；Spire/Support marker不等于几何。当前19stage是adapter图，非原fork排程。History只Rim闭环，compound undo须反转展开后的完整command数组，不能推广为反转raw edit。现有101/去重52份snapshot可直接研究，62缺引用(A18/B22/C22)只阻碍对应缺失样本，不等待全补齐。

第一批并行：G04实际墙输出、G09→G10资源/原墙面门窗输入、G14 Roof/Wall owner（同时确认VisualEdit和原注册接口）；父agent管G00共享契约/Plugin/CLI和连续验收。G02自由墙、G03相交metadata、G08 trigger按consumer需要前置。C新增木墙/基础/木拱/木楼梯和风格材质差异单列G17，不以B代验。

## 2026-10-11 建筑计划实施：资产、墙砖与屋顶历史

三线结果已接真实Bevy World，详见evidence/building-integrate/findings.md。原loader18keys；raw Curve2世界定位+真实Roof/mappedCurve3/arch heights决定普通/半窗/门，实际collision进入hole/storage。半窗随方向/平斜切换实际换资产与开口。World intent持久/重投影是adapter，原anchor/UI及corner/roof UV仍缺。EditRoof真实Query/tick/geometry，Keep不误标Changed；CPU type切换恢复gable/sound/瓦清理，sourceflag1 Flat通过真实源owner请求1.6（adapter下一帧）。Roof五类owner扩账本：Visual仅最后未读、Direction全部原序；前桶计时produce全部cmd后统一移除finished，不造逐cmd flush，也不在Create/Delete/Edit核心自动录制。

新字段证据：source+54对应Raw.extra，是可编辑高度输入，publisher同字段读/约束，并非PublicMaxY。底层赋值无条件写/dirty；实际AdjustWallHeightCmd只接受Keep/Absolute，clamp后同值/unordered跳过，无anchor用Curve U=.5加terrain。cmd flag为audio而非record-history，原SystemParam无History/AppMode；PublicWall+296特殊flag生产者仍显式输入。独立WallHeight历史请求已驱动实际consumer，但完整Wall compound cursor/aux/snapshot未关闭。

墙分行/裁行/归一化/row洞候选/横向partition及resolved proposal裁切→fresh flags→96B有原码证明；WallCpuBricks真实实体随洞更新/移除恢复/宿主删除清理。宿主发布或arch heights变化撤除过期context并标pending_proposal_owner。**raw rows→noise/seed/authority walker→proposal/context仍缺**，下一沿building-constructor/README末段真实seed/noise/truncf/Curve证据关闭，不把显式输入当完整from_curve。研究harness memset/truncf IAT误识已修正并全部重采；旧ancillary报告superseded，失败证据保留。

当前33workspace/29runtime（9新增World）、14帧CLI、36组受影响校验冻结后重跑通过；113 registry依赖身份仍匹配B。3516墙阶段、1668placement、1326subtype、1264owner、1024heightauthority、1280heightcommand按各限定scope分开解释。台账40局部组/system_acceptances=0，C迁移/原调度/GPU及ARM矩形瓦case14单filler1ULP继续pending，未宣告完整建筑。

## 2026-10-11 原始墙砖生产阶段与独立 review

新 wall_production 恢复 B plain kind2/top-tag2/fence-fit=false 的完整分行→noise/hash phases→原 fastrand 消费→authority forward walker→proposal/context→裁切/96B 有序输出。正式报告85例/6476记录；独立review90例/7075记录逐字节一致，全部原码实际return，未注入 private 业务结果。只验收此分支，不扩大到 runtime 来源或其他 kinds/fence。wall_construction_sources 独立恢复真实seed=WallId与初mapped count、BrickSourceId、perturb/circle Option和construction kind政策，共6144限定原码case；BrickSourceId不是material。独立报告见 evidence/building-integrate/stage-reviews/2026-10-11-stage0-stage1.md。

Review 复现并修复：高度命令error导致RemoveWall raw清理被跳过、删除对象后来重新发布；单独加载door真实asset时不应要求window asset。新增两项回归，asset_parameter_history_chain 11项通过。当前有新源码和schema修改，旧全workspace provenance报告已过期，待阶段冻结后重执行，不仅改hash。

关键纠正：Public+38初mapped Curve3和+88 prepared Curve3不同；regular/freeform/gable/construct实际positions均读prepared +90/+98，当前旧runtime曾用initial outline，须随下一阶段统一修正。prepared实际owner是sample_wall_path_intersections，消耗四路真实u8 PathRasters资源、法线层采样、exp衰减/阈值切分后生成曲线与segment Vec；不能将空邻墙查询或初mapped copy当作该producer。源path标量原名/default未定，新增Raw observed_path_scalar与ClutterVersion必填，没有新增seed/context/proposal字段。G02/G03已并行启动。整建筑/C/GPU仍未验收，system_acceptances继续0。

## 2026-10-11 原始墙生产第二阶段及下一里程碑

已移除 SetResolvedWallProposals API，实际 raw owner、四路 encoded path资源、prepared 曲线与 cache、开口和 source policy 直接生成96字节 CPU bricks。不能再把该路径描述成显式 proposal 边界。真实 seed 使用初始 mapped 点数，brick source ID 与材质分开；subtype/cache/constructor 使用 prepared，不把 path segment scalar 当第三高度数组。

第二阶段独立 review 记录 source policy/全部构造分支/非空路径以及 invalid input 原码边界；实际 World 又发现 Roof 删除后旧砖 P1，最终 cache 后消费 Placement/Openings/WallGeometry 修复并回归。原 mask→MaterialStats 已接入，楼梯后处理必须独立两个 mask map 和32字节平台列表。G05原拱和过梁会写 third array，初始化零不代表最终墙高度输入闭合。

当前推进 G01真实作者创建、G03stairs后处理、G05实际结构输出/third写回。全部报告需冻结源码后重执行；原创建 UI、更广结构/装饰/历史/存档、C及GPU均仍属待完成，system_acceptances仍0。

## 2026-10-11 第三阶段冻结验收

46项workspace测试和55个限定验证器全通过、整轮Rust/Cargo冻结，详见 stage-reviews/2026-10-11-stage3.md 及其冻结快照；后续源码变更不自动继承本轮当前性。实际创建/删除→source/public/roof/caches/masks/bricks、raw/processed mask双owner和special lintel CPU+third均已有连续原码证明。过梁降低source不得在旧third上无限max，必须由真实cache init owner重置后跨全部mask累积。新Source64字段必填并copy→Pub294；真实new_wall已证0，direct raw fixture的0是明确输入。

后续最大收益：普通arches及其third/closed首末复制、实际stairsCPU后段和真实Decorator blockers、G14Wall/Roof原bucket barrier与typed source恢复。真实mixed创建replay始终两个bucket，forward Wall→Roof/undo Roof→Wall，不能flatten旧Roof-only展开。C构造器从真实CALL选中14144f690，无C行为等价推断；完整system_acceptances仍0。

## 2026-10-11 阶段4运行闭环与下一消费者

普通/自由拱原码130/270、实际World12帧22墙帧318拱记录/4个120B gate命令/3654砖闭合。128B发送元素=120B game ConstructGateCmd+8B Bevy EventId，不能做游戏分配counter。真实gate finalCPU mesh/TransientMesh/Nani仍由G06继续。原empty-mask oracle统一CPU翻译block使Unicorn until越过已cache终点，修为原CODE edge stop/RIP assert，未host空返回或绕过原CMP/JZ。

台阶coregeometry/holes、actual80B blocker collector、96 renderer6484记录及实际World9帧16cells/8holes/928stairs96/2073wall96通过各限定proof。stats真实+a4+=stairslen、+90-=stairslen，必须从原init快照应用本轮长度，不idle重复扣减；removed source清80B rows/revision、CPU/hole/stats恢复。扩大16消费者研究10 InvalidInput仍保留；初查probe矩形axes缺失导致与native shape不同，需复现修正输入而不是改真实算法或删case。

G14 unified Wall/Roof outer buckets与inner sameWall group barrier已接。真实拆除录制是RoofDelete→WallCompound(StateChange op1 shrink→Delete)，undo反序；actualbulldoze动效.2先phase推进，history timed.4先eval后elapsed，GameTime+58独立于+28需要真实输入。完整authoring回归包含创建/直接删除/bulldoze动画/undo/redo/颜色/ID/meta/源缓存砖恢复。

独立review修P2 unknown owner Drain丢失败和后续队列，预验保queue再mutate；旧无recording Invalid/creation observations不回填后来新事务；Source cleanup必须本帧consume/edit/bulldoze后执行，避免旧Remove reader次帧误清freshReplay metadata；active bulldoze缺metadata不得当source missing静默删cmd；Bulldoze producer已录RoofDelete时adapter Mark前Invalid只保cleanup不重复History owner。stage4 full Rust/Cargo freeze revalidation运行中，未完成前不更新为当前workspace验收。全部system_acceptances保持0。

阶段4冻结完整复验已结束：54workspace tests/release build、67限定验证器全过，Rust/Cargo不变，113依赖身份匹配。15帧Openings历史fixture补真实bucket分帧后在同Rust输入重执行唯一失败项，保retry_history和首次失败输出。快照stage4-frozen/summary.json；下一Gate/Rect/stairsconsumer修正后不自动沿用当前性。目标active，完整system_acceptances仍0。

## 2026-10-11 用户调整：并行实现优先，验证暂缓

用户明确要求多个 agent 并行推进，先尽量实现、不跑验证，冲突先写文件最后整理。三条线复用已有上下文：Gate实体/墙面、Rapier/DirtyTiles/楼梯、矩形finish/cancel与pending/history；父管理共享lib/Nani/CLI。新增代码状态为 implemented_unverified，不把阶段4冻结54/67报告更新为当前验收。冲突和未恢复接口记录于 tracking/parallel-implementation-conflicts；最终仍需统一构建、原码及World验收。

## 2026-10-11T100110Z 当前实现保存

用户要求先保存，三个agent已中断。当前候选实现/未验证边界/依赖Lock待解析/冲突汇总保存在 checkpoints/building-parallel-2026-10-11T100110Z/CURRENT_IMPLEMENTATION.md；完整源码与证据快照及SHA256清单同目录。本次不构建/不测试/不运行oracle，不升级阶段4验收。业务推进停在保存断点，后续需用户恢复后统一梳理。

## 2026-10-11 PDB驱动效率方案审计

用户要求多agent深入重新设计恢复方案。三个agent仅审计PDB能力/自动化/并行组织，父汇总至 NATIVE_RECONSTRUCTION_ACCELERATION_PLAN.md，机器计划 tracking/reconstruction-strategy-plan.json 状态proposed_not_implemented。三版Rust LOCAL/REGREL/BPREL/DEFRANGE全部0、样本void()占位；C/C++阳性对照可读取真实变量，故不能承诺从这些PDB恢复不存在的Rust locals/types。可用的procedure/IPI/源checksum/line/数百万inline应建立共享模块恢复包和缓存队列，并结合原指令字段/API/已有源码。新功能C-first、已有B作为参考；本轮没有业务源码修改、构建或oracle，原保存候选/失败/最终G00–G19验收保持。

## 2026-10-11 更轻的效率路线比较

三agent继续分析减码/自动提升、证据/运行替代、执行决策；父汇总 NATIVE_RECONSTRUCTION_FAST_ROUTE.md。推荐消费者拉动+需求证据+真实closure/API复用+机械胶水+双循环，基础设施条件升级。本机LLVM两PC内联/源查询与原RNE32B汇编读成功（约0.08s小样本，不推总体倍数），可避开这类Ghidra排队。自动lifting、永久binary backend和全库索引不作当前主线；原scope目标独立完整C Rust不变。本轮不改业务源码/不构建/oracle。

## 2026-10-11 项目恢复 skill 与角色模板

已固化 skeleton/workflow-design/tiny-glade-recovery/SKILL.md，并通过 ~/.codex/skills/tiny-glade-recovery 符号链接发现。八类角色为协调、取证、工具还原、常规业务还原、核心还原、独立审查、验证、整合；scripts/workflow.py 只准备索引任务包及冻结/check显式文件指纹，复用既有 task_queue、MCP operator 和 handoff审计，不自动领取或执行MCP。默认禁止游戏实现、业务验证暂停，未来按最新用户授权调整。

试跑发现旧源指纹失效，已用既有build_task_index刷新，verify_task_index通过3667文件/39169函数/1250任务一致性检查；没有实际claim或业务验收。工作流临时fixture检查和八角色真实索引只读预览见 skeleton/workflow-design/SKILL_VALIDATION.json。本轮未写游戏还原代码、未改分析数据库、未构建/测试游戏或运行oracle，building-parallel保存断点和未验证候选状态保持。


## 2026-10-12 三版同码52组工具函数候选

用户明确授权先恢复 ABC 共同项中的52组同码工具。已基于原始 A EXE/B RNE/C EXE 和匹配PDB，使用 native-evidence-pack 读取156个版本映射，检查指令原字节与索引body SHA一致；这仅为取证，不是原码行为差分。候选写入29个合法文件组的 `skeleton/task-index/work/<TASK_ID>/abc-tools-52/crates/...`，保持原路径，每个函数含中文注释、三版VA、逻辑组ID。对应与导航见 [交付总览](evidence/abc-tools-52-2026-10-11/README.md)，机器台账 [abc-tools-52-candidates.json](tracking/abc-tools-52-candidates.json)。

共享函数只写一份，PrefabName同址as_ref委托as_name_ref，timer apply/queue复用空ret。圆内严格比较、闭区间端点、signed LoopRange推进、32位回绕后符号扩展、RNG种子三步、NaN/零符号选择均按原指令写中文说明。未知原类型/签名的word/tag/记录/切片保留观察投影；不补造完整作者结构或producer。CirclePoints2仅构造器，未补未取证next；DebugRectangle默认颜色payload未补零；完整caller接入尚待验收。

52组全部为implemented_unverified：仅rustfmt和冻结引用指纹检查，没有构建/测试/oracle、独立审查或生产整合。旧建筑保存断点及验收状态不改变，system_acceptances不增加。29份delivery已冻结、29个claim已释放。AllocationHandle::is_null原为deferred，用户本轮52组清单明确包含它，因此queue新增显式authorized-deferred-reason参数记录授权，保留索引deferred分类，仅恢复该访问器，没有扩展GPU。

## 2026-10-12 C half-res Pass 实施

用户要求开始实施单 Pass 逆向，并延续直接填入已有 A/B/C 项目的交付要求。已合法领取 TG-b1237e511d78c48e8a，直接在 C 原 half_res.rs 追加 recovered_c；A/B 未套用 C。状态 implemented_unverified，此前验证暂停继续：本轮不构建/测试/oracle/GPU，不增加 system_acceptances，不沿用旧建筑验收。

C 原完整745B包装函数与实际 country_core Shader 已取证：输出尺寸 max(ceil(depth/2),1)，R32f depth 与 RGBA8_SNORM normals，条件debug初始化、同depth资源stencil view0x800000、五纹理绑定及DispatchThreads。Shader按帧选2×2子像素、材质低3位1/2写depth最低位、GBuffer.x低三字节解码法线、行向量变换并normalize、alpha编码子位置。GLSL恢复稿同目录保存，未编译验收；CPU投影复用C锁glam0.30.10，不伪造GPU量化/OOB行为。

完整128-bit TypeId与已命名get_res<T>实例比对，确认FrameCounter读RenderTemporalState+0x108，CurrentView读GameViews首1024B；候选有真实resource payload→所用uniform字段的读取投影。字段原名/帧号递增重置/矩阵更新规则未知。GBuffer由render_world创建，depth来自上层temporal main depth；四直接实参消费者为GTAO、ray_trace_gi、eval_gi_screen_radiance、ReSTIR direct lights，后端同步/consumer shader和生命周期未闭合。

合同、源与缺口见 skeleton/C/crates/country-core/src/systems/render_loop/compute_passes/half_res.recovery.md 及 half_res.rs.recovery.json；19事实/7绑定/4消费者/8缺口。私有rhapsody匹配commit适配器尚缺，当前trait无真实引擎实现，骨架尚未成为可运行渲染工程。原始证据保留现有task work缓存；交付源码没有另建目录。tracking/render-pass-pilot-plan.json已记录实施阶段，旧planning audit只保留为历史静态检查。


## 2026-10-12 按优先级恢复35组候选

用户授权按优先级开始逆向还原。首批35组（几何14、建筑接口17、历史接口4）基于A原EXE/B原RNE/C原EXE及对应PDB取证，105个版本映射、每版33个不同原范围。已写21个合法文件组原路径的共享Rust候选，中文注释与ABC对应齐全；复用上批类型/函数，不改生产Cargo/lib。见 [交付总览](evidence/abc-building-35-2026-10-12/README.md) 与 [机器台账](tracking/abc-building-35-candidates.json)。

原指令与限定真实消费者补证：动态signed32网格宽高，乘加回绕后符号扩展；holes closed路径record+17==0过滤，未过滤holes别名保留；DecoratorLibKey保留EAX/EDX32位返回槽；Vec/std/glam复用；Rectangle3d32B字段互证、Triangle使用Vec3而非Vec3A。完整作者ABI、容器/枚举全集和真实producer/consumer仍未关闭，不把叶接口完成称为完整建筑链恢复。

独立合同审查和冻结候选源码审查已记录；仅rustfmt与冻结引用完整性检查，状态implemented_unverified，无构建/测试/oracle或生产整合，旧建筑保存断点及验收不改变。21份delivery与依赖闭包保留，21个claim释放。下一优先点为装饰物source/asset/subtype→位置/碰撞/开口→move/delete/历史传播，先补本批key/holes/Archivist真实owner，材料统计和其他history owner随后，GPU/场景后置。
