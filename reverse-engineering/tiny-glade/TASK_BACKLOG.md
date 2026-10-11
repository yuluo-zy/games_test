# 游戏自有源码逆向任务与收益排序

> 2026-10-11 墙拱/台阶/统一Wall+Roof事务已接完整限定World业务链，阶段4 review已修队列和metadata生命周期，阶段4冻结54/67已结束，新并行实现批按用户要求暂不验证。下一恢复实际门mesh与实体消费者、矩形enclosure finish/cancel、扩大stairs边界；G00–G19/C/fork/GPU全验收仍进行中。原始noise/seed/height→砖提案显式输入边界已关闭，按[完整计划](BUILDING_SYSTEM_RECOVERY_PLAN.md)持续推进。

> 后续顺序与跨版本验收采用 [THREE_VERSION_RUST_PLAN.md](THREE_VERSION_RUST_PLAN.md)。本表 S01–S26 保留作为细粒度源码任务；先执行新计划 T01–T05，再接续 Roof/Wall/History 核心。当前已实现项仍按 B 基线验证，C 适配要重新取证和验收。

当前阶段是源码恢复；场景回放、GPU与最终画面验收后置。原作依赖全部保留在[dependency-policy.json](tracking/dependency-policy.json)，不要把901个package当作901个逆向任务。依赖实现不计入游戏源码覆盖。

2026-10-12 用户授权的有界渲染例外：C `extract_half_res_depth_normals` 的包装层/Shader候选已直接填入已有 C 文件，implemented_unverified，验证仍暂停。下一缺口为 RenderTemporalState+0x108 递增/重置、GameViews 首块矩阵更新、锁定 rhapsody API 适配、四直接消费者 Shader 与资源生命周期；未改变整建筑/C/GPU验收结论。具体合同见 skeleton/C/crates/country-core/src/systems/render_loop/compute_passes/half_res.rs.recovery.json。

## 当前选择

当前先执行 [三版依赖筛除](DEPENDENCY_FILTERING.md)，再做 **建筑权威状态发布与关联 → Roof创建后段/删除/方向/Flat消费者 → 墙几何与附着 → 历史事务**。这能把已有屋顶CPU源码接到真实权威状态，并为墙、门窗、楼梯和历史操作提供共同基础。无需先重建渲染器或场景。

优先级为基于现有证据的工程判断，不是准确工时预测：P0立即进行；P1核心后续；P2在状态/几何基础后；P3当前后置。每项必须交付游戏自有Rust源码、输入输出/类型证据和必要的原码校验，不能以调用了反编译工具或写了提取脚本代替交付。

| ID | 游戏自有任务 | 优先级 / 收益 | 明确逆向内容 | 直接复用/排除 | 交付与现状 |
|---|---|---|---|---|---|
| D00 | 原依赖适配纠正 | P0；消除重复工作和行为偏差 | 库版本/特性/调用顺序与游戏接口 | fastrand1.8.0/glam0.29.3/half2.4.1 | 已改用原版API；圆形/矩形全记录、seed及RNG回归通过 |
| S01 | 权威对象与ID模型 | P0；解锁全部系统 | Wall/Roof/Shape/Decorator字段、枚举、ID和所有权 | Bevy/slotmap/Vec/Arc实现 | 保留已证实布局，未知字段不补造 |
| S02 | 墙体高度与屋顶标高规则 | P0；连接已恢复屋顶并服务墙/楼梯 | clamp_wall_height、max_y、flat_roof_y、形状高度分支 | glam数值API、terrain容器、ECS查询 | 数值约束后段和3个缓存叶函数已实现；6624案例通过；查询前段/flag生产者待恢复 |
| S03 | 屋顶编辑/生命周期 | P0；把算法接到权威状态 | create/edit/update/delete/switch参数、触发条件与输出事件 | Commands/Query/Event库实现 | edit已解析后段、变化量/反馈源码已实现并通过4514案例；完整ECS/lifecycle待恢复 |
| S04 | Roof参数/枚举消费者 | P0；消除语义猜测 | height/profile/eave/ridge/tip和Width/Length映射、apply/replay字段更新 | serde/serde_json/ron解析器 | 328参数快照已提取，edit 0/1/2及对应字段消费者已核实；B序列化字段与Width=0/Length=1已确认；A/C及缺失默认值待核实 |
| S05 | 历史事务/撤销数据逻辑 | P1；解锁可靠编辑 | mutation token、链索引、前后diff、提交/撤销/重做消费者 | Arc/drop/线程局部库、通用容器 | 已定位收尾，尚未确认全部容器语义；不运行场景回放 |
| S06 | 墙轮廓与形状编辑 | P1；核心建造输入 | circle/rectangle/freehand、闭合、变形、伸缩与形状转化 | glam、Bezier/geometry已有库及原fork | 源码规则和曲线输入输出闭环 |
| S07 | 墙几何/砖块/支撑 | P1；实现建筑主体 | 砖列、支撑、墙裙、拱口、边缘和实例布局 | RNG/half/glam、已有mesh依赖 | construct_brick_columns已精确定位；完整算法待恢复 |
| S08 | 建筑交叠与连接策略 | P1；复现自动适应 | 墙-墙/墙-屋顶关系、相交后修剪、脏变化传播 | Rapier/空间索引库通用算法 | 恢复游戏关系策略，不重写碰撞引擎 |
| S09 | 门窗/装饰附着 | P1；核心可见编辑 | 墙/屋顶UV宿主、位置/尺度、变形/宿主变化 | ECS/数学/容器实现 | Decorator diff数据已梳理，规则源码待恢复 |
| S10 | 门阶与楼梯 | P1；连接地形和建筑 | 阶梯分段、门口/地面适配、前后边关系 | 曲线与数学依赖 | construct_door_stairs已有4668B边界；CPU源码待恢复 |
| S11 | 开口/连窗/装饰成组 | P2；提升建造完整性 | 自动合并/分离、挤压、重叠限制与样式切换 | 几何/容器库 | 按真实消费者识别分支 |
| S12 | 地形编辑 | P1；世界基础 | 网格schema、笔刷核、插值/平滑、标高和建筑基础传播 | 已有noise/math/栅格容器库 | 恢复游戏调用/参数/组合，不恢复公开noise库 |
| S13 | 道路/石板/石子 | P2；打通地形和建筑 | 曲线笔刷、宽度/端点、交叉、台阶/墙开口关系 | 原CurveU/几何公共依赖 | 恢复自有布局和关系规则 |
| S14 | 水域CPU状态 | P2；补完整世界 | 水面标高/范围、岸线、输入约束、相邻对象变化 | 依赖库；水shader当前后置 | 状态与CPU源码先做 |
| S15 | 树/植被/园景生成 | P2；重要内容系统 | 自有种类、形态参数、散布、种子、地形/建筑关系 | fastrand/noise/glam；局部path包先核实归属 | 依据private source路径区分自有与已有库 |
| S16 | 动物/小生物行为 | P2；补游戏活性 | 自有移动/目标/状态机/避让与参数 | 通用寻路/碰撞/数学依赖 | history已有Sheep状态样本，不据此猜整个状态机 |
| S17 | 季节/天气/昼夜CPU控制 | P2；补环境状态 | 自有参数、过渡曲线、季节物件和事件 | suncalc等已有库；光照GPU后置 | 恢复控制代码和数据 |
| S18 | 音频游戏事件 | P2；补交互反馈 | 事件触发、材质/状态参数、随机选取、淡入淡出 | FMOD/libfmod/音频DSP实现 | 恢复游戏胶水，不逆向FMOD |
| S19 | 输入/拾取/相机/拍照逻辑 | P1/P2；可交互工程 | 指令映射、拖动阶段、gizmo策略、选择/相机约束 | Bevy输入/dolly/winit/平台接口 | 恢复游戏层输入与约束，库实现排除 |
| S20 | UI/工具栏/引导/本地化 | P2；可用产品 | 工具状态机、自有布局/选项/引导/资源key | egui/iced/字体平台库 | private包装与fork变化先核实，不重做整套GUI库 |
| S21 | 存档/快照/迁移 | P1；持久世界 | 游戏schema、压缩/打包调用、版本迁移、缺失快照处理 | 原rkyv/ron fork、serde、flate2等 | 22缺失引用登记；恢复源码不等于已回放全部样本 |
| S22 | 资产/prefab/mesh metadata | P1/P2；供全部模块取数据 | 游戏格式schema、资源ID、atlas/prefab构建与绑定 | rkyv/ron/image/dds等公开依赖 | 解自有schema和资源关系，不重写通用解码器 |
| S23 | 更新调度/缓存/任务 | P1；模块一致性基础 | 自有脏标记、依赖顺序、重建/取消/缓存策略 | Bevy scheduler/tasks与线程库 | 只恢复游戏系统的注册与依赖 |
| S24 | 引擎分叉必要适配 | 按实际阻塞选择 | 可证实且影响游戏调用的fork差异 | 原Bevy/引擎整体 | fork仓库/commit待核实；不从0恢复Bevy |
| S25 | GPU/游戏专属render passes | P3；用户要求后置 | 未来仅研究自有pass/参数/独特效果 | ash/kajiya/驱动/通用渲染依赖 | 当前不做shader逻辑/动态/GPU复现 |
| S26 | 场景回放/视觉/性能/完整游戏集成 | P3；最终验收 | 未来跨模块功能/画面/存档/性能对照 | 已复用依赖及完成模块 | 当前后置，不作为源码阶段的先决条件 |

## 具体入口与本轮状态

| 入口 | 作用/为何优先 | 当前证据 |
|---|---|---|
| clamp_wall_height `0x140ac7f30` | 权威墙高度约束，解锁高度编辑/屋顶/门阶共用规则 | 匹配PDB422B；MCP读取完成，数值后段已实现并原码验证 |
| history_record_adjust_wall_height `0x140ac79e0` | 高度操作到历史事务联接 | 匹配PDB1349B；先查消费者 |
| edit_roof `0x1421ab7c0` | 现有屋顶核心的主要编辑入口 | 匹配PDB3175B；实际MCP读取；已解析编辑后段4514案例通过，ECS前段排除 |
| PublicWallState::max_y `0x140b2a620` | create/update Roof均使用的标高来源 | 匹配PDB82B；缓存值/缺失panic条件已恢复并原码验证 |
| PublicWallState::flat_roof_y `0x140b2a6e0` | 平屋顶与墙状态联接，补已恢复算法的缺口 | 匹配PDB90B；max_y偏移和缺失panic条件已恢复并原码验证 |

具体证据增量保存在 `evidence/scope-priorities/` 与 `evidence/mcp/`。实际MCP工具catalog、创建/反编译/引用调用必须保存结果；失败或缺失范围保留，不把MCP注册正确当成工具已使用。

所有新实现都按[项目记忆](PROJECT_MEMORY.md)排除依赖代码，并更新[项目进度](PROJECT_PROGRESS.md)和机器可读任务台账。

## 三版本实施首批交付

已实现三版本历史解码、B参数字节映射与eave、VisualEdit插值和已解析状态联接、创建/删除/方向/样式的观察事件生产者。当前编辑核心4514例、VisualEdit回放1600例、离散回放7168例、实际历史联接708例均通过；格式核查98份/733条通过。所有数字均为各自报告范围，不相加计算完整游戏覆盖率。

下一批：优先恢复 create_roof/update_roof_spatial 与 PublicWalls 关联；明确presence字段与缺失默认值；恢复delete消费者及方向/样式消费者；再接历史事务游标、snapshot装载与墙体形状。A/C跨版本函数候选需要类型/控制流及行为证据后才能迁移。三版本详细任务状态见 [three-version-plan.json](tracking/three-version-plan.json)。

## 2026-10-11 当前建筑链

新实现已解析Wall读取guard/shape/cache/style→Roof空间更新（2659例），创建事件→Roof初始化（848例），回放Create/撤销Delete→初始化→墙更新（512例）。当前剩余的是状态发布/ID实体关联、创建后段组件及注册、删除/方向/Flat消费者、墙轮廓/砖/支撑/开口及门窗历史联接。不要重复将已确认库函数加入源码任务。每个入口先查询DEPENDENCY_FILTERING.md与dependency-exclusions.json，再按业务链验证推进。

## 生命周期第二批后续入口

wall_roof_hashmap实际生产者已确认0x140a94cd0，恢复其已解析业务策略。publish_public_wall_state的确认字段/数值与缓存失效已恢复；dirty输入/geometry/terrain gathering及后续缓存填充待恢复。create后段TileSimulation/Animation已接，删除请求匹配/淡出/移除意图已接。下一批优先找max-y缓存填充及注册顺序、恢复drawable/spire/support和TriggerVolume元素schema，再进入墙砖/支撑/开口及门窗附着。当前系统状态仍in_progress。

### 运行时联接后剩余入口（2026-10-11）

S02/S23已有真实Bevy headless链，circle/rectangle base缓存producer与gable scalar consumer已联接；三版3072+ B256原码案例及5个真实World连续测试有报告。原raw采集、freeform cache、逐arch gable和整注册/fork图仍in_progress。next：恢复14078bd9b..14078c404的gable轮廓采样与每arch修正，定位发布dirty/Curve/terrain源；补Nani/marker完整required schema，再接方向/FlatStyle及墙砖/支撑/开口与门窗历史。不要把当前创建tag/marker等同完整visual消费者。

ARM瓦片整链case14单filler Y 1ULP差异保留在失败报告中；排查glam/host CRT边界而不重写依赖或修oracle。其余状态链报告重跑通过。


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

## 当前保存断点：building-parallel-2026-10-11T100110Z

源码与证据已归档；本批未验证。先参阅 [building-parallel-2026-10-11T100110Z](checkpoints/building-parallel-2026-10-11T100110Z/CURRENT_IMPLEMENTATION.md) 与 tracking/parallel-implementation-conflicts，再继续原计划。


## 2026-10-12 本轮工具候选交付

用户选定的三版同码52组工具已保存29个原模块路径的共享Rust候选，中文注释和156个版本地址映射齐备，见 [总览](evidence/abc-tools-52-2026-10-11/README.md)。状态implemented_unverified，未接生产工程，按既有暂停要求未运行构建/测试/原码；原签名/投影类型/实际消费者义务见各CONTRACT.md。机器台账 [abc-tools-52-candidates.json](tracking/abc-tools-52-candidates.json)。


## 2026-10-12 按优先级恢复35组候选

用户授权按优先级开始逆向还原。首批35组（几何14、建筑接口17、历史接口4）基于A原EXE/B原RNE/C原EXE及对应PDB取证，105个版本映射、每版33个不同原范围。已写21个合法文件组原路径的共享Rust候选，中文注释与ABC对应齐全；复用上批类型/函数，不改生产Cargo/lib。见 [交付总览](evidence/abc-building-35-2026-10-12/README.md) 与 [机器台账](tracking/abc-building-35-candidates.json)。

原指令与限定真实消费者补证：动态signed32网格宽高，乘加回绕后符号扩展；holes closed路径record+17==0过滤，未过滤holes别名保留；DecoratorLibKey保留EAX/EDX32位返回槽；Vec/std/glam复用；Rectangle3d32B字段互证、Triangle使用Vec3而非Vec3A。完整作者ABI、容器/枚举全集和真实producer/consumer仍未关闭，不把叶接口完成称为完整建筑链恢复。

独立合同审查和冻结候选源码审查已记录；仅rustfmt与冻结引用完整性检查，状态implemented_unverified，无构建/测试/oracle或生产整合，旧建筑保存断点及验收不改变。21份delivery与依赖闭包保留，21个claim释放。下一优先点为装饰物source/asset/subtype→位置/碰撞/开口→move/delete/历史传播，先补本批key/holes/Archivist真实owner，材料统计和其他history owner随后，GPU/场景后置。
