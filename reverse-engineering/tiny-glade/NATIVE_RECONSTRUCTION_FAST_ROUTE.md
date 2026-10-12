# 更快的恢复路线：先减少工作量，再降低证据获取成本

日期：2026-10-11。三个 agent 分别比较减码/自动提升、证据/运行路线、执行决策。此文修订上一版加速方案的实施顺序；仅方案与元数据可用性样本，不是新服务或业务实现已完成。

## 推荐组合

最适合当前工程的是：**消费者拉动的最小切片 + 现有证据和 LLVM 按需查找 + 游戏闭包/依赖 API 分离 + 机械胶水生成 + 三业务 agent 并行 + 按真实瓶颈升级工具**。

先把当前消费者所需的来源、规则、事件和释放合同交给 agent；不知道的分支进入独立判别循环。完整 SQLite、全 PDB inline 解析、Native reader 和新 worker 都不是业务开工前置。它们只有在成本可以被重复查询/排队节省覆盖时升级。

## 1. 证据获取存在更轻的入口

本轮本机 LLVM22.1.2 直接读取 B 原 RNE/PDB 的两个地址，返回 RoofCreate 源行以及 Bevy deref_mut→EventReader::read→RoofCreate 的内联链；一次调用等待约0.08秒。原 RNE 的32字节定界汇编也成功读取。它们没有启动 Ghidra或执行原业务机器码。样本不能推算全工程速度、冷启动或完整函数范围。

优先路径：已有证据 → 原地址的symbolizer JSON → 原始PE定界ASM → 仅未解的块/caller申请反编译。RVA与VA分别换算，点内联信息不自动扩成整个区间。没有命中的信息明确unknown。

一个artifact可复用symbolizer stdin，批量提交入口/callsite/分支点。缓存直接引用原binary/PDB与地址；无需先建立全局数据库。格式能力参考 [LLVM symbolizer](https://llvm.org/docs/CommandGuide/llvm-symbolizer.html)，本次成功范围见 [证据路线报告](evidence/reconstruction-strategy-2026-10-11/alternatives-evidence-runtime.md)。

## 2. 减少需要恢复的内容，比增加反编译吞吐更有价值

对每个有界业务root标出四类区域：纯依赖、纯游戏、库包装器内游戏callback、混合优化区域。

- 纯依赖直接复用锁定公开API，停止抄写allocator/query/hash/Vec实现。
- 库包装器里的游戏闭包恢复捕获输入、筛选、参数、排序和副作用；其外层iterator/container用真实库。
- 混合区域保留来源及待判别点，绑定参数与语义后才替换，不能按名字或一条inline位置粗暴删除。
- 已有Curve/cache/geometry/history/asset实现调用原API，不再复制“差不多”的算法。

先为Gate/Fence这样的包形成减码清单：真正游戏规则、Perchance/glam/Bevy API边界、必须恢复的callback、现成Rust接口。交付者面对的是规则与生命周期，不是全文汇编中的全部库胶水。

## 3. 自动化首先处理机械工作

机械生成优先：有证据的schema/字段投影、Bundle/Query候选、模块映射、root exports、register请求、CLI事件转换、状态/证据清单。stage顺序、默认值、required edge、历史owner及删除策略仍由实际证据裁决。

P-code/SSA第一阶段只自动提取load/store/guard/常量/call参数的候选事实，减少人工逐offset追踪。随后只试一个已知schema、无动态内存和间接调用的小策略生成；未知op/ABI/溢出/NaN语义拒绝生成。Ghidra P-code提供指令语义表示，不能因此宣称能自动恢复作者Rust或完整Bevy应用。[Ghidra P-code](https://ghidra.re/ghidra_docs/languages/html/pcoderef.html)

不优先建设通用Ghidra C→Rust或整PE→Rust翻译器。当前反编译C缺真实签名/类型，夹杂macro、库内联和编译器临时值；直接翻译会保留低层unsafe与库实现，再花成本恢复业务。Remill等机器码提升工具输出LLVM层语义，完整系统仍需CFG、ABI、外部调用及所有权恢复。[Remill](https://github.com/lifting-bits/remill)

## 4. 两个循环并行，避免每个未知点都阻塞模块

**实现循环**：每次完成能进入现有链的一个切片，含真实source、状态/关系、消费者、更新撤回、删除/取消/历史。可用合同足够即实现，不等待整模块每个字段全部命名。

**判别循环**：只追答案会改变下一实现决定的问题。优先影响多个消费者的source、ID、坐标/长度、RNG、事件顺序和资源释放。当前仅透传且尚无消费者读取的字段保opaque；真正参与规则的未知不填默认值。

每题记录：竞争解释、受影响的实际consumer、最便宜可区分证据、结束条件。例：需要区分raw/prepared，就找二者不同的合法输入或真实读取点；需要区分bundle判别/上传flag，就读对应stores及Query，而不是dump整个renderer。后续获准验证时才执行诊断输入，继续保留独立覆盖矩阵。

## 5. C目标不等于所有问题只能研究C

新包仍以C交付。具体问题选择最清楚的证据：A较短的accessor、B已完整恢复的算法、C明确的当前caller。跨版只形成候选，回C确认offset/enum/分支/单位/API及生产者。记录 `reference-only / C-static-confirmed / C-native-proven / C-World-bound`，避免重复全量恢复，也避免B结论直接冒充C。

## 6. 并行效率取决于少等待和少返工

三个长期业务owner继续负责附着/历史、建筑输出、碰撞/传播；共享文件只有一个批次owner。父保留跨包语义裁决，机械integrator可按批移交，不必每行alias/CLI都亲自写。

每包提交短manifest和候选patch：唯一type身份、API、register含义、前后stage/flush、writer/reader、撤回/删除、未知点。最小接口先登记，完整交付批次合并；非阻塞观察字段放后面。上下文只传当前证据/接口/修正/下一问题，原文按需查，避免继承后又反复读全部历史。

## 7. 各路线的采用次序

| 路线 | 当前建议 | 收益和门槛 |
| --- | --- | --- |
| 现有Rust/原证据检索 | 立即 | 最低启动成本；先检查scope和版本 |
| LLVM点符号化+原ASM | 立即首选 | 已有小样本成功；主要消除已知地址查询的Ghidra排队 |
| 闭包提取+依赖API替换 | 立即 | 直接减少库展开恢复量；真实参数/副作用必须绑定 |
| 已证schema/登记胶水生成 | 优先短试点 | 省重复代码和共享往返；不自动发明业务规则 |
| 常驻反编译控制面 | 条件升级 | 启动/排队在多个包中反复阻塞时；已有provider需一次实际导出确认 |
| SQLite/LLVM Native reader | 条件升级 | compact包检索不能满足反复关系/完整inline范围查询时；一次小试点后决定 |
| SSA事实提取/小策略生成 | 条件短试点 | schema与纯策略闭合；输出可审查，未知op停止 |
| 原端接口观察/record-replay | 后续按未知点启用 | 研究规则与状态；不把Unicorn桥接当原完整ECS调度 |
| Windows原游戏live trace | 条件路线 | 最适合UI/资源/事件/异步时序未知；当前没有确认ready环境 |
| 永久binary bridge | 不作为最终方案 | 仅研究过渡；必须标binary-required，不算独立Rust完成 |
| 通用自动lifting/整程序反编译 | 不作为当前主线 | 首期投入、低层输出和ABI/依赖恢复成本高，尚未证明端到端优势 |

现有REA/RE-MCP历史catalog或顶层连接ok不是当前函数导出成功；trace_feature/call_path是静态关系，不是动态trace。控制面只选一个，失败立即用现成LLVM/ASM/Java batch fallback，不并行修多个服务。

## 8. 三个可证伪试点

1. **同一恢复包的资料获取比较**：现有批导出与symbolizer/定界ASM路线获取同样合同，分别记录cold/warm、总整理成本、遗漏与补查。点查询不足时计入补查，不只比较工具进程秒数。
2. **同一未知题的决策比较**：固定展开helper树与竞争解释驱动的最小判别查询，比较到达同等可信合同的请求量、等待及后续返工。错误source/owner解释直接判失败，不能以少查为胜。
3. **后续启用验证时的切片/缓存比较**：同一change set试最小crate和可信闭包选择，与全路径对比遗漏及成本。`cargo --bin`仍会编译root library，不能当真正隔离。缓存原端需覆盖binary、输入、范围、桥接/环境和真实RET；候选变化仍实际执行。

本轮仅提出这些比较，没有运行Rust编译、业务试验或新服务。没有测量前不承诺几倍提速。选择标准是**到同一完整业务结果的总耗时下降，并保持证据与生命周期完整**。

专项报告：[减码与自动提升](evidence/reconstruction-strategy-2026-10-11/alternatives-code-reduction.md)、[证据与运行路线](evidence/reconstruction-strategy-2026-10-11/alternatives-evidence-runtime.md)、[执行决策](evidence/reconstruction-strategy-2026-10-11/alternatives-execution-decision.md)。前次保存的实现与失败证据不变。
