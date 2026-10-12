# PDB 与现有源码驱动的建筑系统恢复方案

日期：2026-10-11。父 agent 汇总三个 agent 的独立审计。状态：方案已提出，流水线尚未实现；本轮不修改业务 Rust，不运行构建或业务差分。先前实现快照继续保留，新增候选仍未验证。

## 1. 改变恢复单位和信息组织

此前主要按“找到符号 → 获取反编译 → 补 helper → 手写 Rust → 局部差分”串行前进。局部算法已积累，但父 agent 反复转发地址、启动 headless、追加共享接口，agent 缺少可以直接实现整个模块的上下文。验证与源码的关联又有不少使用全工程粒度，放大了修改后的重执行范围。

新的工作单位是**原模块的一条完整业务链**。一次交给 agent 的资料包括原模块/函数范围、源码行与内联来源、调用关系、已知类型和字段合同、匹配依赖 API、相邻恢复源码、真实输入与最终消费者、删除和历史行为、未解决问题。agent 主要完成游戏规则和生命周期；数据采集、目录映射、机械骨架、依赖排除与登记准备尽量批处理。

不能把“90%尚未利用”作为测量结果；先区分本 PDB 确实提供的数据和没有提供的数据。也不预先承诺提速倍数。

## 2. 三版本实测：有丰富行号和内联，没有所需 Rust 变量调试记录

完整统计和阳性对照见 [PDB 能力审计](evidence/reconstruction-strategy-2026-10-11/pdb-capability-audit.md)。本机 LLVM 22.1.2 可正常读取三版，第三方 C/C++ 的参数、变量和位置范围亦可读。

| 数据 | A | B | C | 使用方式 |
| --- | ---: | ---: | ---: | --- |
| TPI 类型记录 | 6,133 | 6,125 | 6,126 | 所查游戏 namespace/类型无命中；不能将库类型当游戏布局 |
| IPI 身份记录 | 119,594 | 132,599 | 144,737 | 建立函数、作用域、内联身份关联 |
| Rust 内联点记录 | 2,333,877 | 3,204,897 | 3,966,178 | 分析物理函数中的逻辑函数与源码来源 |
| Rust 行号子节 | 75,298 | 83,699 | 92,381 | 关联指令范围和原源文件/行号；不是源码行数 |
| Rust LOCAL/REGREL/BPREL/DEFRANGE | 0 | 0 | 0 | 不规划从不存在的记录直接恢复变量或参数 |

Rust统计按 `.rcgu.o` 编译单元集合，包含依赖和泛型代码，不是建筑业务覆盖数。建筑函数样本的类型索引是 `0x1001 void()` 空参数占位。S_FRAMEPROC 提供帧属性，不能解释为真实参数布局。没有一版能为另外两版补齐完整 Rust locals。上述形态不能单独反推出确切 rustc 编译参数。

因此：保留 TPI/变量解析能力，但按实际覆盖启用。当前第一收益来自 **procedure + source/checksum + lines + inline + 泛型接口约束**，结合指令、serializer/accessor、依赖源码和已有 Rust 恢复缺失的游戏字段与调用合同。泛型名提供类型候选，不提供原参数变量名；行号不包含原作者源码文本。

格式参考：[LLVM CodeView Symbols](https://llvm.org/docs/PDB/CodeViewSymbols.html)、[LLVM CodeView Types](https://llvm.org/docs/PDB/CodeViewTypes.html)。Rust 调试级别的差异见 [rustc debuginfo](https://doc.rust-lang.org/rustc/codegen-options/index.html#debuginfo)。项目实际覆盖以本次审计为准。

## 3. 统一证据库，替代反复临时查找

首批使用现有 LLVM CLI 对目标 module 提取 compact JSON，逐包写 SQLite；不等待全部百万内联记录入库，不先开发大型 PDB 解析器。源数据保留 raw record 与偏移，解析失败显式记录。后续只试点一个函数的 LLVM Native reader，再决定是否扩大。

MVP只要求身份、source/procedure、小范围inline、合同与当前Rust映射。以下完整表是按需求扩展的目标，不是首批开工前置。

建议证据表及关系：

- `baseline`：原始 PE/PDB SHA、GUID/Age、PE section、编译器与 Cargo 来源。
- `source_file` / `compiland`：原路径、校验算法和值、参与/引用编译单元；定义owner需程序记录及line/inline另判，路径相同和校验相同分别记录。
- `procedure` / `symbol_alias`：GPROC/LPROC、record offset、section/offset/size、全部同址别名；地址按实际 section 换算。
- `type_record` / `variable_location`：TPI/IPI 原始记录、作用域与存活区间；缺失与占位签名单独标志。
- `line_range` / `inline_site`：物理范围、父子内联、inlinee 原文件/行号及未解析 annotation。不能将 inlinee 记录数当独立函数数。
- `call_edge` / `data_reference`：调用点、目标候选、来源指令、常量/全局引用。
- `dependency_api`：包版本/commit/features（未获证据时为unknown）、公开签名、游戏调用策略、fork 缺口。
- `field_fact` / `interface_contract`：字段名称/offset/单位/来源、producer/consumer、证据等级、版本和冲突。
- `rust_mapping` / `work_package`：原模块到当前 Rust 的映射、可复用实现、生命周期要求和完成边界。

每条事实保留 `observed / inferred / conflicting / unknown`。键包含 baseline 和 artifact 身份；不能将 B 的类型索引、字段偏移、参数或事件直接套 C。符号去 hash、同源码校验和、相似机器码只生成匹配候选。

## 4. 用内联和行号筛除依赖，再进行有类型约束的反编译

首先把指令范围关联到定义来源和内联链。对 std、Bevy、glam、RNG、Serde、hashbrown、Rapier 等确认依赖生成公开 API 调用候选，实际参数/语义绑定完成后才能替换；不继续恢复其容器、查询和数学实现。对优化器混合了库和游戏指令的范围保留多来源，不能粗暴删除整段。std/alloc wrapper中折叠的游戏闭包必须保留。

对缺游戏类型的对象，批量组合四类证据：serializer 的字段/variant 名，constructor/setter 的 stores，accessor/consumer 的 reads，caller/Bundle/SystemParam 的接口约束。已有恢复类型和相邻 Rust 是候选契约来源；原始指令裁决字段层级、单位、默认值和数值顺序。少量低置信字段进入 unknown，不为了生成完整 struct 填零。

在独立 analysis clone 中导入已确认结构/enum/参数约束及命名；只有总大小与offset已证时构造完整native布局，局部字段仅用partial view/sidecar。语义类型与native布局分离，未确认位置不强制回填。PDB 的 `void()` 占位不导成真实 ABI。Rust `repr(Rust)`、enum niche、hidden return buffer 和泛型 lowering 不按 C ABI 默认处理。业务 Rust 用语义类型；ABI/probe 投影单独保存，不统一套 `repr(C)`。

现有 B Ghidra 数据库属于 patched EXE，只用其映射和历史分析；实际 CFG/指令证据需回到原始 RNE。C 新工作优先建立原始 C 的分析上下文，不把 patched B 的跳表或反编译标号作为 C 行为。若C分析尚未就绪，先交metadata/已有ASM的恢复包继续工作，不等待完整分析。

## 5. 一个常驻反编译 worker，三个 agent 直接领取模块恢复包

复用当前 ExportFocusedFunctions 的 program/DecompInterface，改成单写 worker 消费原子 JSON 请求队列。每项请求独立处理 missing/timeout/error，保存 checkpoint；一个入口失败不丢失整批。业务 agent 可以自行查证据库、提交请求和读取结果，父不再逐地址传话。

缓存键包含artifact、范围和原目标bytes hash、analysis DB/CFG revision、language/compiler spec、decompiler options、类型overlay及工具版本。缓存正结果及明确的负结果；假设、范围或身份变化才重取。timeout/cancelled是可重试运行状态，不能永久当不存在；按budget/options/DB变化重试并保firstfailure。常驻会话只绑定一个基线，切换基线采用独立上下文。

每个恢复包提供：

1. 原始文件/函数/ABI 身份与三版差异摘要。
2. owner 及必要 helper 的反编译、定界 ASM、inline/行号、caller/callee。
3. 依赖 API 和已确认字段/资源/事件合同。
4. 当前 Rust 对应文件、真实相邻实现、重复类型/缺实现提示。
5. source→消费者→修改→删除→取消/历史的链清单。
6. unknown、冲突及最小下一查询。长原文放文件，给 agent 的入口摘要保持有界。

自动生成的是目录映射、typed skeleton、接口文档、登记请求和证据引用；游戏行为由 agent 基于完整包恢复。不会宣称自动反编译成作者 Rust 源码。

## 6. 四个席位的实际分工

| 席位 | 持有完整业务链 | 交付责任 |
| --- | --- | --- |
| 编辑/附着/历史 agent | Source/ID→storage→anchor→实际 asset/opening→holes→历史 | 创建、修改、取消、删除、undo/redo与输入 owner；不能留给父收尾 |
| 建筑输出 agent | raw/prepared/cache→结构提案→砖/门/栅栏/楼板/屋顶输出→实体/CPU所有权 | 每次领取一个完整 family，包含 stale 撤回、无变化和宿主删除 |
| 碰撞/空间 agent | 实际几何→依赖 collider/query→游戏 filter→DirtyTiles/订阅→双方重建 | 查询、替换、删除、传播及实际资源 owner，不复写物理引擎 |
| 父 agent | 队列/共享DB写权限、共享接口、注册/调度、依赖与集成 | 合同及冲突裁决；每批一次共享集成，不再承担第四条长业务链 |

父负责队列与共享权限，常规查询由worker自动处理；M2服务改造由一个轮换agent承担短批，其余两条业务链继续推进，不集中让父开发全部基础设施。

当前最大并发为父加三个 agent。一个业务包稳定后领取下一个包，不增加无上下文等待的 agent。私有 helper 在包内批量解决，不将每个函数变成单独任务。

文件独占优先。本项目当前无 Git，不假设可以使用 Git worktree；必要时将小源码树复制到独立目录并提交 patch，保留基底 hash。共享 lib、Cargo/Lock、Nani、StageGraph、CLI 由父单写。M3/M4按包交错，最小API合同先登记使下游开工，成熟后再批交完整handoff；不等三个包全部结束才注册。模块提交 `register`、事件、排序/flush、CLI schema、依赖增量和删除合同的一个 handoff，父统一应用。非阻塞观察字段可延后。

冲突记录应包含 owner、状态/事件、触发条件、读取时点、单写权、顺序、撤回/失败队列、候选方案、原证据和后续验收场景。文字合并成功不能把双 writer 或错误时序标成 resolved。

## 7. 完整建筑的实施顺序：新功能 C-first，保留 B 已恢复参考

1. 先梳理当前保存的未验证候选与未注册草稿；保留已落地代码，批次对齐接口、注册、源/派生所有权及待解析 Lock。此时仍按用户指令暂缓验证。
2. 用现有 Gate/Fence、Info72/Storage/Anchor、Collider/DirtyTiles 三条相邻链试点新资料包和 worker，直接服务真实消费者，不独立造 demo。
3. 补齐权威输入：UI 开始/拖动/结束/取消、startup/ID/颜色、装饰附着源及各历史 owner；持续消除永久 resolved 输入。
4. 按完整族推进 C 的 G06 楼板/栅栏/墙面，G08 roof trigger/尖顶/梁/模拟，G11 其他装饰与 roof holes，G12 玩家楼梯，G13 木构/材质/灯光。已恢复 B 作为接口与算法参考；每个 C 合同独立取证。每包显式记录target_baseline=C、reference=B和prototype实际版本，B试点不能计为C落地。
5. G14 所有历史 owner、G15 保存/恢复/迁移、G16 原注册/任务/取消穿插每包推进；不等所有 B 完成才开始 C。G17 C 木构/样式差异随所属 family 处理。
6. 完成 G18 实际 GPU 分配/上传/更新/移除，再关闭 G19 全建筑交互、状态/画面和长期资源清理验收。GPU 后置但不会从完整目标中删去。

## 8. 分开实现和验收，降低重复工作而保留证据

本阶段继续实现先行，候选标 `implemented_unverified`。以下是后续验证设计，本轮不执行：

- 算法证据：记录实际传递源码依赖、oracle/harness、fixtures、native ranges/RET 与原库边界。
- 编译证据：记录 compiler/target/features、Cargo/依赖及实际 probe 产物；源码报告不自动接受新产物。
- World 证据：记录注册、阶段顺序、事件/ID/资源 owner 与场景；数值叶函数通过不等于业务链通过。
- 全工程证据：里程碑冻结和最终系统验收保留全工程范围。

静态筛查的66个验证脚本中42个使用全局 candidate_provenance，24个声明显式 candidate_sources；显式列表还需审查依赖闭包。先在3个包试点变更影响选择，不能立即缩小所有报告。共享 harness、不可确定的 feature/codegen 变化或调度大改，保守扩大复验。

原端执行缓存仅在 binary、实际输入、范围、桥接环境、RET 与输出 schema 均不变时复用；候选有变化仍须重新执行候选并比较，不改 hash 伪造新验证。全套集中在大里程碑或最终验收；首次失败、RNG/数值/顺序变化仍定向深挖。保留 ARM 屋顶1ULP失败，不放宽比较。

## 9. 落地里程碑与退出条件

| 里程碑 | 具体交付 | 退出条件 |
| --- | --- | --- |
| M0 能力审计 | 三版 PDB 统计、Rust/C++对照与函数小样本 | 已完成；不存在的数据不列入自动恢复承诺 |
| M1 最小索引/恢复包 | 3个有界owner恢复包、身份/源/小inline/合同/Rust映射及unknown | 三个 agent 能独立开始实现，不等待全库索引 |
| M2 元数据服务 | 常驻 worker、查询/缓存/失败隔离、一个 inline→SQLite 试点 | 重复请求不重启/重导出；raw范围/别名/annotations保全，parsed/unparsed明确 |
| M3 并行恢复试点 | 三条完整业务链、机械登记请求与冲突表 | 源、消费者、增改删、取消/历史均有明确owner；未验证状态保持 |
| M4 父批次整合 | 共享接口、Lock、StageGraph、CLI和版本边界 | 无重复类型/重复注册/双writer的未裁决合同；后续验证待启用 |
| M5 后续限定验收 | 依赖闭包选择、实际候选与原端比较、连续World | 每项执行/跳过可解释；旧失败及当前产物身份可追溯 |
| M6 C 全建筑 | G00–G19各族与原任务/存档/GPU闭环 | 达到完整退出条件，而非以函数数或case数计算完成率 |

M1和M2可重叠：业务包拿到第一批资料即可实现。基础设施若阻塞实现，使用限定 module 导出 fallback，不停工等待索引完美。

## 10. 如何判断效率确实提高

试点记录每包的元数据等待时间、同 key 重取次数/缓存命中、父共享接口往返次数、agent 阻塞原因、完整链交付时间、文本/所有权冲突、后续验证执行集合与实际 wall time。比较相近复杂度与相同交付边界，不以代码行数、文件数或工具调用数冒充恢复进度。

计划减少的活动：每函数重新启动分析、整 PDB反复搜索、依赖体反编译、父逐地址转发、零散共享登记、每个小改动跑全部证明。计划增加的投入：可复用证据包、原逻辑函数与内联归属、调用/字段合同、相邻源码检索、完整模块生命周期实现。

三份专项方案：[PDB 审计](evidence/reconstruction-strategy-2026-10-11/pdb-capability-audit.md)、[自动化流水线](evidence/reconstruction-strategy-2026-10-11/automation-pipeline.md)、[并行落地与验收](evidence/reconstruction-strategy-2026-10-11/parallel-execution-plan.md)。它们和本汇总是新方案文档，不是新流水线已经运行或建筑系统已完成的证明。

## 后续路线比较修订

多角度比较后的推荐顺序见 [NATIVE_RECONSTRUCTION_FAST_ROUTE.md](NATIVE_RECONSTRUCTION_FAST_ROUTE.md)：先消费者拉动、LLVM点符号化/原ASM、库API/真实闭包分离与机械胶水；worker、SQLite和Native reader按实际阻塞条件升级，不作为业务开工前置。原审计与数据缺失结论保持。
