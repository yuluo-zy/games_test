# 更快的证据获取与运行路线：替代方案比较

日期：2026-10-11。仅分析、只读元数据检查；未改业务 Rust、构建、执行游戏原指令 oracle、运行游戏、启动或重新连接分析服务。本轮只写此文。目标仍是完整、独立的 Rust 项目，原 binary 只能作为研究证据和后续验收参考，不能作为永久业务 backend。

## 1. 核心改进：按问题分流，不把完整反编译作为统一入口

比“先全索引，再全函数 decompile，再手写”更快的首选是四条分流：

1. 已知地址的源码/内联归属 → **LLVM symbolizer**，直接得到 source/line/inline owner。
2. 原参数、字段读写、分支或直接 call → **原 RNE 的定界 ASM/Capstone**，只查必要块与前后调用点。
3. CFG、多个分支组合或长计算规则确实不清楚 → **已有 Ghidra 上下文中的有界 decompile**，常驻 worker/MCP 是控制面，不是每个问题的前置。
4. 真实 ECS acquisition、authoring、事件顺序和资源生产者仍不清楚 → 优先追 source/caller；在真正能运行原 Windows 目标的环境才考虑 **限定 live trace**。现有 Unicorn owner wrapper 用于独立算法与接口观察，不能代替真实游戏调度。

MVP 可以没有完整 SQLite、没有 Native PDB reader、没有新 MCP 连接：从当前地址、已有原 ASM/报告以及 symbolizer 输出构成恢复包，直接交给三个业务 agent。数据库是可复用缓存和关系索引；按需求增长，不承担开工前的全库建设任务。

## 2. 本轮实际检查与明确边界

### 已立即实用的 fast path

本机 `/opt/homebrew/opt/llvm/bin/llvm-symbolizer` 是 LLVM22.1.2，支持 `--output-style=JSON --inlines --relative-address --no-debuginfod`。本轮只读取 B 原 RNE/PDB，成功执行：

```text
llvm-symbolizer --no-debuginfod --output-style=JSON --inlines --relative-address \
  --obj='/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne' \
  0x21aaea0 0x21aaed9
```

真实返回：

| RVA | 返回证据 |
|---|---|
| `0x21aaea0` | `system_roof::create_roof::create_roof`，原 `crates/systems/roof/src/create_roof.rs:17` |
| `0x21aaed9` | `bevy_ecs::system::system_param::impl$19::deref_mut`，registry Bevy0.16.0 `system_param.rs:1235` → `EventReader::read`，`reader.rs:28` → RoofCreate原源 `create_roof.rs:29` |

这一次工具调用的进程等待约0.08秒，仅是本次小样本观察，不能承诺冷启动、全部目标或全库的通用性能。JSON 中 Column/StartLine等为0/空，不能将其当原变量或范围数据。点查询返回该 PC 的内联栈，不提供完整函数 inline 区间、全部别名或 ABI。

`llvm-objdump --disassemble --no-show-raw-insn --start-address=0x1421aaea0 --stop-address=0x1421aaec0 B/tiny-glade.rne` 也成功读出32字节范围的原x86-64指令。此操作不打开 Ghidra、不执行原机器码、不改数据库。已知 VA用 objdump；RVA用 symbolizer `--relative-address`，不要混淆。

建议下一步给同一函数的入口、callsite、分支块起点成批符号化；完整资料落盘，以 source路径/inline链区分游戏与依赖。一个artifact可复用一个stdin symbolizer进程，避免每 PC 重启。未命中 `??`、源line0、地址映射冲突必须显式返回 unknown；不做顺延猜测或将一次点结果扩展到整函数。

### 服务连接与工具名不能替代可用性证据

现有 [mcp-native.mjs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/scripts/multi-version/mcp-native.mjs) 使用已配置 REA toolgate+Node MCP SDK。路径存在；脚本显式 JAVA_HOME为现有 Corretto21路径，本机默认 `java -version` 实际为OpenJDK26.0.1。不能把两条启动路线的Java环境当作同一个已确认配置，也无须本轮更改它们。

本轮只读了历史catalog/reports，没有连接服务：

- [REA catalog](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/multi-version/catalog.json) 保存29工具。历史 `mcp-session.json` 显示连接成功、catalog可列，但 `results=[]`；不能据此证明当前目标已能反编译。
- REA `batch_decompile` 是显式地址的批量 decompile，文档称并发且逐项错误；`procedure_info` 可给observed完整body ranges/blocks。可用性需要当前provider/target/实际结果证明。
- REA `trace_feature` 为字符串/procedure/xrefs；`trace_call_path` 为direct call图遍历。**两者不是运行时 trace**，未知indirect调用仍未知。
- [RE-MCP Ghidra catalog](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/mcp/re_mcp_ghidra-catalog.json) 保存33工具；有retain worker、batch、execute、type APIs；不等于当前mac会话存在ready worker。
- [source-probe.json](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/mcp/source-probe.json) 顶层 `ok=true`，但 `decompile_function` 内部 `isError=true`、`No function at 0x1408E3250`。目标为旧D盘`tg_source`，不能当当前original函数导出成功。`wall-accessors.json` 的batch和 `wall-accessors-prepared.json` 的execute也有CreateFailed。
- ReVa PyGhidra scripting是另一入口，要求实际附着的PyGhidra Ghidra会话；本轮没有可调用的直接ReVa scripting工具，也未证明live session。不能因安装bundle或skill描述就将它列为ready。

替代路线应先用**一次读-only provider/target/exact-entry/1函数导出试点**，失败立即使用原ASM+已保存C或Java batch fallback。不能投入多天“修服务”才让业务agent开工；也不能用force_new删除已有分析。

### 运行环境和原端观察的实际限制

本机是arm64 mac；`/usr/bin/lldb`存在。Wine、`qemu-x86_64`、Frida未在本轮PATH查询中找到；这不是全磁盘未安装的证明。没有本轮可确认的原Windows游戏live session，也没有启动它。

已保存 [native_fixture.py](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-constructor/native_fixture.py) 使用 Unicorn x86-64模拟原RNE，按PE sections加载并核对hash，显式构造memory/register/stack。依赖/CRT等通过hooks bridge；它不是Windows loader或完整原Bevy World。该文件顶端明确是research harness，不因“全body调用”自动成为已接受验证。

已有 [native_authoring.py](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-authoring/native_authoring.py) 可观察原业务create/update/delete范围的源记录、ID与notification；存在explicit branch输入和std storage bridge，UI成功写事件分支不包含之前真实点击/renderer判定。读脚本即可界定边界，本轮未import/run这些模块，避免import时实例化原模拟环境。

## 3. 路线比较：成本、瓶颈与覆盖

成本为相对工程判断，没有承诺工时或倍数。

| 路线 | 当前可用证据 | 收益 / 成本 | 排队或失真风险 | 使用建议 |
|---|---|---|---|---|
| 现有C/ASM/report/合同检索 | 本地文件现成 | 最低启动成本；避免重复取证 | 报告可能过期、局部scope、patched原端混淆 | 每个查询先查cache/证据范围；旧scope不升格 |
| LLVM symbolizer+定界raw ASM | 本轮实际成功 | 无Ghidra队列；直接给point inline/source、原指令 | 没有完整interval/alias/类型；地址base误用 | **首选fast path**；成批入口/callsite/块点，按artifact缓存 |
| 需求驱动CLI PDB module包 | 全三版审计已可读 | 不等全库；针对procedure/line/annotation补缺 | pretty parser须保raw/unparsed；每module过大 | 只索引owner和必要helper，复用现有audit样本 |
| 长驻Ghidra Java worker | 现脚本可复用program+DecompInterface，worker未实现 | 分摊启动/DB成本，类型/CFG一致 | 单writer串行队列，长函数阻塞，clone有启动成本 | 仅未解语义用；短请求优先，长请求独立，checkpoint |
| REA/Re-MCP已有入口 | 历史catalog/连接存在；具体函数有失败 | 可能复用现server而省自建worker，批量meta降低往返 | provider绑定/target/DB不ready；batch concurrency增加CPU/内存；静态trace名误导 | 1次ready/exact-entry成功试点才选为控制面；失败快速fallback |
| ReVa现live-session scripting | 当前未确认可用 | 若已ready，直接查询Ghidra对象/批量抽取 | 启动PyGhidra、服务绑定/锁及工具未暴露 | 条件路线；不做基础设施前置 |
| LLVM Native reader→SQLite | API头文件已检查，未实现reader | 需要完整关系/范围时更稳定；保record offsets | 基础设施成本、950万inline存储/解析 | 只做一个实际owner inline/C13试点；CLI足够就暂停扩展 |
| 全PDB/全程序索引+whole decompile | 当前无首批必要性 | 全局搜索有长期价值 | 大初始等待、库代码噪声、过量C/CFG错误 | 不作第一阶段；需求密集后后台扩充 |
| Unicorn原owner/native wrapper | 已有脚本与限定scope报告 | 可执行完整CPU owner、observe IO/branch，跨ARM可研究 | fixtures/bridges影响可达性，body返回不证明原调度 | 后续获准时按缺口用；不再为整ECS搭模拟世界 |
| Windows原游戏live trace | 本轮无ready运行环境证据 | 最能取真实UI/资源/事件/ID链，降低输入猜测 | host/loader/资产/调试setup；观测扰动，样本不全 | 条件优先用于**runtime-only未知**；不是眼下可执行承诺 |
| LLDB调候选Rust/record replay | LLDB已存在；候选既有runtime | 快查候选注册/资源/删除/历史问题 | 只能证明候选行为；可能循环以自己验证自己 | 未来调试集成用；原规则仍需原证据 |
| 永久调用binary提供未恢复功能 | 不属于目标交付 | 表面省代码 | 独立Rust未完成、版本环境依赖 | 禁止当完成；研究oracle与产品runtime分开 |

## 4. 需求驱动索引可进一步缩小

证据单位优先是“要决定的一个合同”，例如某publisher source来自哪、某query是否排除自身、某history cancel是否删异步任务、某cache是否用prepared。每包先列这些未决问题，再按问题选择工具：

| 问题 | 首查证据 | 升级条件 |
|---|---|---|
| 游戏与依赖归属 | symbolizer inline/source+Cargo/exclusions+原module | point来源不足→module records/IPI；不直接whole decompile |
| 原field offset/ABI | raw ASM caller load/store+已证accessor/serializer | 多branch/别名冲突→局部decompile+owner范围 |
| 分支输入/默认值 | 原constructor/setter、source caller | 只有consumer→追producer；禁止凭fixture默认 |
| 修改/删除/dirty传播 | direct calls+原consumer+现Rustmapping | 间接系统/任务顺序未知→原registration/必要runtime观察 |
| 整体复杂算法 | 已存wholeC+raw范围+依赖API | 缺流程→有限完整owner decompile；native观察未来定向使用 |
| 真UI/ECS运行时来源 | 原source caller与注册/PDB泛型 | 数据只能从运行时确证→限定Windowslive trace，而非无限扩Unicorn ECSbridge |

可先交JSON/Markdown函数包，无服务也可查。SQLite从已用包入库，symbolizer结果做point evidence，不提前许诺全inline范围。PDB候选同址 aliases须单独保留；point仅一个name返回不用于自动rename。源码checksum相同不合并native行为。

如果很多owner重复查询相同module，或跨包scope/alias无法靠点查询支持，再扩大module索引；只有已有任务真实需要全关系才做Native reader。触发条件用观测的重复查询、unparsed范围和agent等待原因，不用预估“90%没有利用”推进基础设施。

## 5. 静态、native wrapper、live trace与record replay各自承担什么

### 静态减少解释对象，不降低业务范围

无需每个库helper都生成C。保留原owner中的阈值/选择/dirty/ID/资源关系和最后consumer；stdlib/Bevy glue按source/API标注。优化代码同时来自多个inline owner时保持多来源，不能按一条库line删掉整个块。

完整owner decompile仍有价值，但范围由未决合同决定；可先请求caller+constructor+writer，再按缺口追加。只拿局部函数不会自动得到完整模块，恢复包仍包含修改、删除、取消、历史及消费者要求。

### 原owner wrapper是观察接口，桥接边界必须显式

现有原body模拟可以在真正返回后观察完整records、IDs、cache/dirty、RNG和顺序；只截到指定中点则是suffix/slice证据，不能称wholebody。std桥接可复用，但不得hook游戏结果返回预设输出、装填未证zero资源再称真实source。

扩native wrapper的停止条件：越来越多ECS/container/TLS/query需要人为模拟，关键真实producer仍无法采集。此时应停止用“补更多bridge”追求完整原运行，把未知转成required contract；取真正source caller或有条件的原游戏trace。桥接数量不是恢复完成率。

已保存 [B-stair-wall-runtime-verification.json](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-integrate/B-stair-wall-runtime-verification.json) 的限定报告包含8 wall frames、16 cells、8 holes、928 stair96与2073 wall96有序输出；其excluded明确为原ECS acquisition/fork schedule、manualUI、GPU、C migration和wholebuilding。它说明CPU owner链研究方法已有价值，不能证明这些运行时缺口也闭合。本轮不重跑旧报告，也不把它升级到当前未验收源码。

### live trace只解决真实运行时未知

需要原目标可运行的Windows上下文、正确artifact/资产、调试接口以及可控gesture；目前不具备已确认入口，因此这条是条件方案。若后续ready，首个试点应记录一条真实UI gesture的start/drag/finish/cancel及相邻consumer，而非抓全游戏所有函数。

优先捕获原入口参数/返回、source ID→entity关联、资源revision、command/event追加与消费、异步任务token/cancel、dirty集合及每frame边界。数据来源保原PC/PDBinline链；无locals记录就观察寄存器/内存合同，不伪造原变量名。保实际采集触发/gate/顺序与未观察分支。

动态trace是一个具体运行样本：缺事件可能是gate未触发，不能推出系统永不发生；call顺序不一定是注册全部先后关系；地址/FPU/线程扰动须保留观察环境。成功观察也仍要静态恢复规则，转换为独立Rust owner。

### record replay区分原端输入记录和候选自证

现fixtures与CLI World traces可重放候选输入链，适合发现缺registration/stale/delete/undo问题。原历史/快照可帮助对齐serialized业务schema；缺快照引用、参数默认、版本差异不能被候选fixture填补。

原live采集的source/event/ID资源记录未来可生成最小回放fixture；用户动作顺序与原系统消费顺序分层保存。捕获的80B/typedobserved rows可作接口证据或研究中间输入，不是永久绕过实际authoring producer的最终实现。最终须让Rust从真实输入自动产出这些记录，并闭合修改/删除/取消/保存。

## 6. 最小试点、退出与停止条件

| 试点 | 最小范围 | 成功条件 | 立即停止/退路 |
|---|---|---|---|
| F0 快路径 | 一个已有owner的entry+callsite+2分支点 | 正确artifact/RVA，返回source/inline或明确unknown，原ASM可定界 | line/alias不充分→限定module；本轮仅2点已实际读成功 |
| F1 直接给agent函数包 | 一个owner+必要producer/writer/helpers | 有真实输入/字段/消费者/删除/历史清单与unknown；agent能开工 | 不等types完整；已有C/ASM先发，metadata晚补 |
| F2 控制面选择 | 当前target的1函数只读查询/decompile | DB/provider身份正确、exact entry/body和输出明确、重复请求无需重启 | 失败一次保存reason，退现Java batch/CLI；不扩大服务修复 |
| F3 Native reader小试点 | 单owner的procedure+C13+inline父子→SQLite | offsets/范围与已有CLI原records一致，unsupported保raw；明显消除实际重复查询 | 没收益、复杂支持分支过多→保CLI包，暂停全库方案 |
| F4 原wrapper观察 | 后续获准后一个已证CPU owner的真正RET与必要输入 | 游戏body未被fake结果替代，输入/桥接/返回可追踪 | ECS模拟蔓延或关键producer未知→转source研究/live条件方案 |
| F5 Windowslive条件试点 | 实际authoring一种gesture+cancel+删除/undo | 真运行环境和采集artifact已绑定；source→consumer链可观察 | host/loader/资产未ready→不阻塞CPU静态任务，不虚报可用 |
| F6 回放接合 | 已采集的source/event链→独立Rust实现 | 输入自动产派生记录，ID/取消/删除/历史owner明确 | 仅captured records驱动renderer→仍缺producer，不标完成 |

本轮实际只有F0的两个symbolizer PC与32B objdump检查；F1–F6未新执行。不存在“新服务已连接”“trace已抓”“原wrapper新案例已过”的承诺。

## 7. 排队、并行与成本控制

- 三个业务agent先读恢复包；已存证据+LLVM fast path可独立请求，不等Ghidra。若要daemon，复用每artifact一个symbolizer stdin上下文是轻控制面，不先开发大关系服务。
- Ghidra只排尚未解释的语义请求；一个program一个writer，overlay变更串行。短元数据/ASM请求用CLI旁路，不被长函数timeout堵住。
- REA batch并发数不等于吞吐提升；共享provider/DB锁、JVM内存和decompilerCPU要实测。不能三agent各open/reanalyze同program或启动三个新重分析数据库。
- original analysis clone未ready时先发metadata/原ASM，patchedDB只提供historical mapping；不得为赶速度复制其CFG当original。C新owner独立绑定C原artifact。
- cache绑定artifact hash、RVA/VA规范、原range bytes、查询kind/options、分析DB/CFG/overlay版本和tool版本。timeout/cancelled可重试，旧missing在DB补entry后失效。完整firstfailure保留。
- 控制面选型只需一个ready入口；REA、RE-MCP、ReVa、自建headless不是都建一遍。能复用实际ready服务优先；没有ready服务先批脚本，后升级worker。
- 成本记录真实“等待元数据、服务启动、decompile排队、父转发、bridge新增、一次业务包交付”时间；不能按工具数量/文件数衡量更快。

## 8. 对现计划的建议修订

将M1首包入口从“限定module CLI→SQLite”进一步缩为 **已有地址/原ASM/合同→symbolizer point来源→函数包；按需追加module**。M1和原模块业务并行，不等M2。

将M2从“必须自建常驻Ghidra”改为 **选一个当前能用的持久控制面**：已有service通过F2就复用；未通过则现Java batch+有限队列，随后常驻。当前报告不足以证明REA/Re-MCP新会话已ready。

类型化反编译仍必要，但只有确证native布局/ABI才导。三版Rust locals零不能靠另一个UI或新的PDB解析器修复。Native reader解决检索效率和结构关系，不会产生文件里不存在的类型。

为真实运行时source缺口预留条件live trace路线，防止无限搭Unicorn ECS仿真；当前mac环境不把这条当开工前提。阶段正式交付仍是每family独立Rust输入→派生→消费者→取消/删除/历史闭环，采集接口不能永久留为binary调用或captured fixture。
