# 比统一流水线更快的执行路线：决策优化与试点

2026-10-11。**仅分析方案，未实施、未测得提速。** 本任务只读现有方案、源码和handoff；未改业务源码/依赖，未执行build、probe或oracle。当前用户暂缓验证，以下编译、差分缓存和试点执行均为后续设计，不自动启动。

推荐先采用 **消费者拉动的按需证据 + 双循环恢复 + 最小合同批次**。把常驻worker、SQLite和Native reader降为由实际阻塞触发的工具升级，不把它们列为业务开工的必经M1/M2。统一方案的大方向可保留，但最先改善的是下一问题的选择、原有证据的复用和共享写入的交接，而非索引覆盖量。

依据：统一方案、[自动化方案](automation-pipeline.md)、[并行计划](parallel-execution-plan.md)、[PDB能力审计](pdb-capability-audit.md)，以及实际[Gate/Fence](../../tracking/parallel-implementation-conflicts/gates.md)、[碰撞/DirtyTiles](../../tracking/parallel-implementation-conflicts/collision.md)、[编辑/附着](../../tracking/parallel-implementation-conflicts/rectangle-history.md)交付。示例中的历史失误只用于解释决策风险，不宣称重新验证了当前源码。

## 1. 先固定目标，不能通过缩小目标制造“更快”

| 目标 | 可复用什么 | 不能偷换成什么 |
| --- | --- | --- |
| 完整可执行建筑源码工程 | 原版本依赖、已恢复业务模块、原输入/资源owner、完整生命周期、必要fork adapter | 只通过少量样例的功能clone；永久resolved输入；把CPU记录或marker当实际渲染/释放完成 |
| 原作者逐字源码文本 | 实际发现的源文本/仓库/发行文件 | PDB路径/行号/校验、反编译C或行为等价Rust。目前没有证据支持从这些PDB取回完整作者文本 |
| 行为恢复/替代 | 同一基线的机器码约束下，恢复游戏规则与数据/事件语义 | 注入proposal/RNG/缓存结果来绕过游戏producer；靠epsilon、硬编码结果或toy状态表掩盖失败 |
| B参考到C的兼容迁移 | 已证明B算法、类型候选、相邻Rust组织与依赖边界 | B地址/layout/enum直接套C；把B普通分支差分当C原caller已接或完整C完成 |

本项目最终目标是完整建筑工程，以C为交付版本；B已验成果降低理解成本。GPU、存档、原输入/任务和必要版本边界仍属最终目标。局部正常domain可先恢复并保持typed gap，但这只是交付顺序，不是永久删除困难分支。性能指标必须以同一交付目标比较。

## 2. 为什么统一plan仍可能慢

1. **包过大也会形成串行。** “完整模块包”若要求所有helper、inline、字段和类型都齐备才发给agent，会把旧的逐函数等待换成整包等待。真正有用的是足以实现下一条源→消费者边的最小资料，完整包持续增量累积。
2. **基础设施回报有条件。** 查询次数少、已导出函数多时，SQLite/worker/parser搭建可能比直接读既有证据更贵。工具正确性、schema维护和analysis上下文又会消耗一席位；没有等待日志之前不能断言建设它们一定更快。
3. **父的瓶颈不只是工具启动。** alias、shared enum、register/tuple、StageGraph、CLI与Lock都经过父。即使反编译零等待，每包仍需父逐条裁决会继续堵住下游；应减少往返并将机械登记与语义裁决拆开。
4. **完整索引不替代关键判断。** Rust locals/位置记录已证全零；更完整地解析它们不会生成缺失参数。行号/inline很有用，但只有它们能改变本题归属/CFG/接口判断时才值得深入。
5. **上下文复用不等于全部历史继承。** 每个agent反复全文读同一大记忆、巨型ASM和其它包的草稿，容易混淆old/current/accepted。不断换agent而交付材料不变，也会重复schema误读；复用的是短证据状态和接口，不是所有对话。
6. **验收粒度尚需治理。** 局部proof、binary、World图与全工程freeze不同；42个全局绑定与24个显式列表并存，不能简单说“全部67都应缓存”。闭包不可靠时必须保守复验，但不能让一个无关新模块默认引发每个局部实验的全套重跑。

## 3. 首选路线：两个循环同时走

**交付循环**从现有CPU链的实际消费者倒查，选择一个可闭合切片：输入owner/ID、状态写入、派生更新、撤回、删除/取消/历史。已有API足够时直接复用实现，不等待全面schema。未知分支保Result/Option/pending并留下实际owner；已证普通分支继续推进。每个切片交付完整生命周期，避免累计一串永远没有输入的leaf。

**判别循环**只处理会改变交付决策的未知点：单位/坐标系、原source、default、enum、RNG消费、真正writer、事务排序、资源释放。每题写竞争解释、影响范围、最小可区分证据和结束条件；有限查询后更新合同，交付循环立即消费。不能把“已报pending”当判别循环完成，也不能默默默认某解释。

两循环共享同一接口事实表。未知字段若仅由已证clone传递而当前消费者不读，先保opaque bytes；未知字段若被当前consumer读取并决定几何/ID/记录/释放，优先判别，不填零。有证上游允许resolved输入时可先恢复下游，但记录剩余sender，禁止把fixture配置长期冒充startup生产者。

四席位仍为父/三个业务agent，避免新设“永久工具agent”挤掉业务。父可以把一个批次的机械integrator角色交给已有单写owner，但必须明确shared文件权限移交；单写不等于永远只能父亲自写。父保留未知语义、跨包事件顺序和最后scope的裁决，不逐个代查原函数。

## 4. VOI：只取能改变下一决定的信息

此处Value of Information是决策框架，不是假定已有精确概率或机器学习模型。对一个未知点q，比较：**得到答案后能减少多少关键路径等待/返工/错误验收风险，是否能用更便宜的证据得到同样答案**。收益、成本未测量时标高/中/低和理由，不计算虚构分数。

建议每题保存：

```text
q / package / target baseline
decision：答案将改变哪个source/API/consumer/删除合同？
hypotheses：至少两个真正不同的解释；已排除哪些？
impact：具体依赖它的下游owner，是否在本批关键路径？
evidence：已有位置与最低成本下一查询；是否能直接判别？
cost：查索引/范围ASM/完整decompile/跨版定位/原码执行哪一级？
stop：什么结果足够采用合同，什么结果必须保持gap？
status：unasked / discriminating / resolved-contract / typed-gap
```

| 优先级 | 实际判断标准 | 例子与动作 |
| --- | --- | --- |
| 必须先解 | 未知会影响ID、owner、写入顺序、源/派生坐标或删除；错误会传播至多个consumer | Info“door”是platform有效性还是bottom bool；raw/prepared与两种长度；Roof/Wall删除录制顺序。直接查真实accessor/callsite/producer |
| 高信息收益 | 有竞争假设，一个短范围/serializer/descriptor可区分，答案立即解锁至少一个consumer | bundle word是Named判别还是Solid flag；用实际写入与Query符号裁定，不继续完整dump整个Nani |
| 可暂后置 | 当前只保留不解释的payload，换解释不改变本批状态操作 | 尚无renderer读的Info metadata；保opaque并记录下一consumer，不为了完整struct查询全部字段 |
| 停止重复 | 已有能力证据证明这一路无法提供所需内容，且没有新增判别条件 | Rust locals全零之后再次请求同namespace TPI取局部变量；切换producer/consumer/serializer，而非换正则重复问 |

选择一个题，先比较候选查询：已存证据→公开依赖API→单callsite/常量→bounded ASM→caller+callee/serializer→仅必要的完整owner导出。并非固定“永远先反编译”或“永远先PDB”；查询顺序取决于本题的判别力。新别名、不同artifact、不同范围属于新问题，不受旧负结果自动排除。

## 5. Active learning用于判别，不用于盲目增加case

当前只做静态证据选择：选最能区分解释的原分支或读写点。后续启用验证后，才用差分输入选择竞争解释的分歧点，并保留独立覆盖矩阵。

例如有“先row draw再height skip”和“先skip再draw”两解释，选择zero-height后接非zero row并核最终RNG/输出顺序，比扩大均匀正height随机cases更有信息。若有“使用raw还是prepared”，选择两曲线确实不同的合法输入，而不是两者恰巧相同的圆墙。若有“旧观察是否回填下一事务”，连续跨Start/Cancel/Delete而非只看单帧结果。例子是方法，不表示这些原问题仍未知或已新运行。

这类选择仍须满足实际caller允许domain；不能用非法ABI状态推断正常World bug。分歧输入可确认一种规则，但不能证明全部输入域。后续验收同时保留普通分支、阈值、无变化、缺资源、释放/undo和版本差异；不把active selection变成只测容易过的样例。

## 6. C-first实现，每题跨三版选择最清楚证据

每包写 `target=C / references=A,B / prototype=<实际版本>`。新功能从C的真实entry/caller/resources开始；复用B成熟算法与RustAPI，不先独立做完B再迁移C。

具体字段可先看最清楚的一版：A没内联的短accessor、B已有完整serializer或oracle、C当前最明确callsite，都可降低理解成本。**跨版最清楚证据只产生候选规则**，必须回C确认字段/控制流/单位/调用与依赖版本。给每个迁移事实标 `reference-only / C-static-confirmed / C-native-proven / C-World-bound`；不以相同source checksum升级状态。

C的reader/DB尚未覆盖时，先对C做最小section/符号/callsite/ASM包，同时用B参考解释它；不是停工建全C数据库，也不是拿patched B CFG充C行为。若三版都无所需layout，停止寻找所谓“丰富版Rustlocals”，沿原producer stores和consumer读取推ABI。engine/fork差异使用必要版本adapter，仍按原实际边界恢复；不重新实现Bevy/HashMap来获得一个可编译替代品。

## 7. 共享合同与冲突：先减少人工往返

每包用一份短manifest产生**候选**alias/register/事件schema/CLI机械patch和proof-input清单。生成器只连接已声明名字与文件，不决定default、stage语义、query过滤、required edge或历史顺序；这些由原证据和单写owner裁决。

最小manifest字段：包与baseline、独占文件、复用API、public新增签名、共享类型唯一导出身份、register是只资源还是含系统、前/后stage及flush、实际writer/reader、ID与资产输入、error/pending队列、撤回/删除/取消/undo、未知点、当前状态。父按合同一次反馈跨包冲突，agent直接继续独占实现。

处理优先顺序：共享API不兼容/双writer/删除丢失优先；不影响业务的CLI观察字段和文档命名批末处理。一个consumer只有一条schedule路径，不能既在register安装又在父tuple调用。same-ID owner切换需转移/撤回合同；不能以多reader恰好同帧为依据。

冲突记录至少带：冲突ID、state/event、双方writer、触发次序、原证据、候选裁决、兼容影响、owner和验收场景。每项状态区分 `contract-agreed`、`implemented_unverified`、`reviewed`；不要把“写了防护代码”直接标resolved。共享文件单写owner可按批轮换，权限交接后其它agent只提交patch建议，不能并发落同一个lib/Lock。

上下文交付只带：当前revision/证据身份、已证事实/更正、最小API、已改文件、尚未闭合边、下一问题及关键报告引用。保留原模块长期owner，整包完成或明显独立consumer才换人；审查时换另一agent读取有界包。不让新agent重新阅读全部数十万行ASM，也不让实现者反复自证自己的完整验收。

## 8. 后续最小编译与差分缓存，不能混淆四层

当前不编译。后续恢复验证后，候选包可先做最小切片编译，但**当前`cargo build --bin ...`不自动等于小切片**：probe仍可能依赖整个root lib，类型也可能被path和alias重复定义。只有实际依赖被隔离的薄crate/一致module组才有此收益，避免新建第二份业务实现。需要拆分时用同一源文件、锁定依赖和唯一类型身份；生成包装成本也计入试点。

| 层 | 复用依据 | 必须重新确认的变化 |
| --- | --- | --- |
| 算法历史proof | 已审查的传递Rust/依赖/harness/input闭包 | 游戏算术、RNG、helper、features/宏/build脚本/target边界改变；不明依赖保守扩大 |
| 编译产物 | 实际binary hash与flags/target/toolchain | 新binary必须有自身身份，不能换旧hash冒称已运行；切片成功不代表最终工程通过 |
| World链 | stage/event/query/resource/ID/flush/fixture合同 | sharedtypes、writer-reader或删除/record时序变化，须运行相关连续链 |
| 全workspace | 版本化suite与完整freeze | 多包里程碑/依赖或调度大改/最终C；局部通过不能代替 |

原端完整结果可缓存，但键必须包含原binary、入口/实际return边界、source inputs、全harness/桥、环境、active输出schema与比较策略。原harness alloc/内存范围等变更失效全部依赖它的缓存，不能只重跑一个Rustcase。原端不变而候选改动时可只执行候选重比；旧报告不改hash，状态仍是其历史输入的证明。

编译和验收要按完整closure选task，不能只按“这个文件名没变”跳过。slice通用库/宏/cfg依赖未可靠追踪时先保守；没有可靠closure的老verifier保留全绑定。全工程一次/批，局部失败修复只追加受影响报告，重大共同边界变化再全套。ARM1ULP失败继续原精度；跨架构行为不能用“本机看上去一样”消除。

## 9. 备选路线、成本与升级/退出触发

以下阈值是建议的**执行规则**，不是已测得性能数据；试点后调整。成本包含搭建、解析正确性、维护、查询/阅读、返工与共享等待，不只算命令运行秒数。

| 路线 | 推荐位置/成本 | 升级或退出触发 |
| --- | --- | --- |
| 按需证据+双循环 | 默认首选；成本是短合同/问题清单与现有索引读取 | 已有证据足以决定next consumer就停止采集并交付；真实schema/owner竞争才继续判别 |
| 常驻headless worker | 条件升级；需异常隔离、版本/上下文与请求可靠性 | 同一analysis上下文至少有多个待处理独立请求，或连续两个handoff显示重启/转发是关键阻塞；先单worker试点，不先服务化全部工具 |
| Compact JSON缓存/短manifest | 立即可用的低成本组织；不保证自动语义正确 | 同query重复、baseline/范围已明确就缓存；事实冲突/overlay变化必须开新key |
| SQLite全关系索引 | 延后；schema/导入/查询正确性要维护 | 至少两个独立业务owner出现现有文件检索无法可靠回答的同类关联题；只建支持该题的表，不先入库百万inline |
| LLVM Native reader | 最后按需升级；工具代码与record/annotation语义成本高 | 当前CLI明确无法输出一项阻塞业务的原record/annotation，或实测重复parse成本已超过一个有界reader改造；先一函数/一module验证，失败退CLI+raw |
| 最小切片crate | 后续编译试点；包装/类型身份/闭包维护成本 | 实际build日志证明无关root依赖占关键路径，且共享业务types可唯一导入；没有收益日志不先拆workspace |
| 父逐函数转发 | 仅共享DB权限临时fallback；成本随往返增长 | 业务agent能读既有index/结果就立即绕过父；真正跨包语义裁决仍交父 |
| 全套revalidation | 后续里程碑/不明影响域保守路径 | harness/依赖/图大改或闭包不明必须用；无关局部新增不应每次默认使用 |

淘汰或不采用：以“自动恢复90%”驱动的大型PDB平台；全Rustlocals迁移；按函数数平均分工；每题只看单版本或一律先重复三版全量恢复；fixture注入冒充生产者；无证默认值/假GPU handle；到处复制dependency内部实现；为通过差分修改oracle、放宽精度或删除失败case；同源码多个crate里随意重复类型定义；把几十个通知模块当完整建筑完成。它们不是成本更低的同目标方案，而是缺数据、形成重复劳动或偷换交付目标。

## 10. 三个可比较试点：先设计，后授权执行

试点共同要求：固定原artifact/问题与终点，包含cold-start与后续复用成本；两路线用相同可访问原证据和同样质量审查；答案正确/链完整优先于快。不要让路线A做完整生命周期、路线B只返回一段函数后比较用时。实验不在本轮运行，不以现在的未验证代码宣称有效。

### 试点一：按需资料 vs 最小统一索引/worker

挑选一个实际新C owner的两到三项相邻合同题（例如输入字段来源、modified选择、删除owner），两路线都交付相同source/consumer/删除合同与证据。A使用现有文件/限定CLI/批ASM按需查询；B先建限定module的compact索引/worker再答同类题。可用历史已解问题做脱敏盲测，或者对复杂度相近的未来题交叉分配；不能让知晓旧结论的一方与陌生agent比较。

控制：同一baseline与类型缺失条件，独立证据目录，预先锁定答案审查标准；分别记录首次搭建与第二/第三题warm-query，避免仅报告缓存命中快而隐藏首建成本。指标：合同正确性/unknown保留、elapsed关键路径、工具setup/read时间、父往返、遗漏aliases/原范围、cache命中。B仅在总交付成本和质量不劣时升为默认；若A更快就保按需并保留小缓存。

### 试点二：主动判别题选择 vs 固定逐函数展开

对同一有竞争解释的任务给两路线相同初始摘要与原证据入口。A先列假设与最小分歧callsite/分支，逐题选择高VOI查询；B按入口→helper→所有相关decompile固定顺序。最终都必须给同一业务API、有效domain、source与生命周期合同，而不是一个给猜想、另一个给完整恢复。

建议三题类型覆盖：坐标/长度来源；标记字段真实owner；事务/删除顺序。使用冻结历史证据作盲测可避免重复运行原码；未来原码分歧输入实验只在恢复验证后执行。不同agent做交叉任务，记录各自已有背景，防止熟练度替代策略效果。

指标：改变实际合同的查询数、无决策作用查询数、time-to-resolved-contract、错误解释/返工、独立review遗漏、实际闭合consumer。仅更少函数输出不算胜；A若跳过关键输入域/删除分支，即使更快也失败。该试点不要求先建设SQLite/worker。

### 试点三：最小切片+闭包选择 vs 当前全工程路径

此试点需用户恢复验证后执行。对同一后续真实change set、同一候选与目标target，比较A全workspace/全绑定路径和B实际最小slice构建、影响closure及原端缓存。不能简单用`--bin`假称隔离。B包括一次包装/dep manifest审查成本，最终产物与World整合仍按共同验收确认。

选三类变化：独立新module/导出；共享业务helper或类型变化；共享native harness/桥变化。变更只在隔离候选进行，报告和源码基底不可变；不为指标故意修oracle或丢case。用人工闭包审查/真实compiler依赖证据与共同全套结果检查B是否漏验，不以“跳过后没看到失败”判closure正确。

指标：build wall time和无关依赖耗时、setup成本、执行/跳过报告集合及理由、cache失效范围、candidate binary身份、原端/候选执行次数、missed dependency、相关World结果与总整合时间。共享harness变化若未失效其全部消费者，B直接失败；slice成功但final工程或生命周期失败也不接受。选择性路线只有质量边界保持后才谈收益。

## 11. 下一批最小行动

本轮只提交分析，不启动工具或恢复业务。下一次获准实施时：先给三个现有owner各一个“下一消费边+最多几个高价值未知题”的短handoff；父把共享alias/register最小合同一次裁决；复用现有raw证据及RustAPI，保持未验证状态；记录metadata/父等待而非预先搭平台。真正出现服务阈值时才轮换一席位做小worker/索引升级，其余链继续。

选一条新C功能作为目标包，已有B参考注明范围；最先做的不是全C数据库，而是C实际entry/caller/依赖与跨版候选确认。之后按试点数据决定工具投入，最终按原码、World、存档/版本/backend完整目标关闭。若不能证明哪条路线更快，就诚实保留未知，不用代码量或案例数量制造效率结论。
