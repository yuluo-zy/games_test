# 更新工具审计与 critters/pet 小试跑路由

日期：2026-10-11。已读 `~/.codex/mcp-tools/README.md`、`reverse-operator.py`、项目 `AI_REVERSE_TOOLS.md`、`tool-usage/manifest.json`、现场/快照schema与toolchain-revalidation。只进行一次bounded Ghidra只读连接和一个身份查询；未查询pet函数、修改共享数据库、改供应商源码、构建、运行游戏或原指令oracle。父agent统一实际pet查询。

证据：[tool-audit.json](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/toolchain-pet-pilot-2026-10-11/tool-audit.json)，现场完整结果：[tool-audit-connection.json](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/toolchain-pet-pilot-2026-10-11/tool-audit-connection.json)。本轮判断以新的实连结果为准，不沿用早前“入口未确认ready”的旧结论。

## 1. 实际连通与身份边界

以下argv通过Python MCP SDK实连，不依赖模型是否加载新namespace：

```text
/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python
/Users/yuluo/.codex/mcp-tools/reverse-operator.py
 ghidra <absolute-job.json> <absolute-result.json> --worker shared
```

此次进程退出0，`actual_mcp_connection=true`、`ok=true`，catalog1页/14精选工具，`get_metadata.isError=false`。stdout显示Strict program routing、UDS连接TinyGlade；没有运行数据库写操作。

实返回 `tiny-glade.exe`、B patched文件路径、`x86:LE:64:default`、`clangwindows`、base `140000000`、131517 functions、991681 symbols。该metadata不返回SHA，因此“路径/平台/计数绑定已确认”不能扩大为“原始RNE或全loaded-memory SHA已重新验证”。文档中默认patched SHA为 `cf048e27...7036323`，原RNE SHA为 `f507a6ce...7b7d03`，二者必须分开。

当前operator SHA为 `921bd98b9495220c118baab06487106d2e98912cde56383f6666f640bbf2fc05`，与`ai-guide/summary.json`一致。总`toolchain-revalidation/summary.json`内helper hash仍是早一修订的`d8128...`，不能用其hash宣称当前源码完全相同；它的关键路径验收与新的样例复验分别保留，本轮没有改写旧报告。

## 2. 本轮确认的现场工具与schema

现场14工具：`list_instances`、`connect_instance`、`list_tool_groups`、`load_tool_group`、`search_tools`、`disassemble_function`、`find_data_types`、`find_functions`、`get_functions`、`get_metadata`、`get_struct_layout`、`get_xrefs_from`、`get_xrefs_to`、`read_memory`。名称相似不保证不同backend参数相同；所选现场schema已存tool-audit.json。

| 用途 | 精确调用合同 | 控制输出/错误 |
|---|---|---|
| 身份 | `get_metadata(program='tiny-glade.exe')` | 该调用本轮实执行成功；没有SHA字段 |
| 有界候选 | `find_functions(program, name_pattern='critters', regex=false, offset=0, limit=20)` | 不省pattern，不用limit0；名称无命中不能推模块不存在 |
| 批量合同 | `get_functions(program, functions='<逗号串>', fields='signature,entry_point,body_start,body_end,callees,callers', include_call_context=false)` | 按guide每批≤20；functions不是JSON数组；空fields会取fullbundle |
| 限定反编译 | 上述fields只加所需`decompiled_code`；或ReVa分页 | `refs`也会触发decompile；不为初次名字发现请求全部locals/context |
| 有界xref | `get_xrefs_to(program,address,offset=0,limit=10)` | 多地址也为逗号串；读每项total/error，不只top-levelok |
| 原生内存 | `read_memory(program,address,length=20)` | length整数；返回是当前patched DB内存，不能当RNE原字节 |

`body_start/body_end`是辅助定界观察，完整原函数body可能非连续且patched CFG可不同。原PDB范围、原RNE定界ASM、同地址aliases应作为恢复包的另层证据。

ReVa快照的`get-decompilation`必须给 `programPath='/tiny-glade.exe'` 与`functionNameOrAddress`；`offset=1`是1-based源码页，`limit=20`。`includeIncomingReferences=false/includeReferenceContext=false`适合首读。`find-cross-references`给`location`、`direction`、`limit`；不能把Ghidra的address/program字段直接搬过去。本轮没有重连ReVa，能力依据新复验记录而非本轮实调用。

## 3. 最适合pet小试跑的路线

首选 **已经实连的Ghidra shared read +原PDB/ASM来源**。无需再次搭常驻worker、重新分析全程序或为一个小试跑启动REA独立provider。

父可先运行tool-audit.json里的 `suggested_parent_discovery_job`：身份→`critters`限20名字。之后按返回的真实名称选择少量original owner。若pet是独立命名或该词无命中，使用已有PDB/module源路径缩小，再另做有界`pet`/`poodle`查询；不以宽泛空pattern拉131k函数。

第二job用实际地址逗号串取signature/entry/body/callees/callers，先排除std/Bevy泛型adapter；第三步只给选中的producer、consumer、cancel/delete/历史直接邻接owner补decompiled_code和limitedxref。原输入和最后consumer都列入恢复包，不能拿两个数值helper当pet系统恢复。

大函数首读ReVa20行分页；没有真实ABI而只得到`void()`或HighPcode `RETURN`时，先查原ASM/caller，不判业务为空。需要HighPcode/自定义Python时，root使用新的唯一名字如`pet-owner-20261011-root`、`reva --write`，即使脚本只读也必须专属worker；不能在shared上开run-script。

原字节交叉核对继续用原RNE LLVM symbolizer/objdump，或按需要radare2；共享Ghidra/ReVa只提供patched上下文。A/C地址/类型不能直接套B；小试跑先声明B范围，C迁移另取证。

## 4. 各后端已重验的能力与使用成本

| 路线 | 新记录确认能力 | pet选择 |
|---|---|---|
| Ghidra shared14/read、dedicated19/write | 45 game reads、PID固定路由、多client；两个独立clone写签名/save/reopen保持差异 | 默认只读首选；本轮另实连metadata成功 |
| ReVa shared6、dedicated10 | 定点decompile、脚本stdout隔离、signature/Pcode、保存重开、脚本失败检测 | 分页与必要Pcode；不调用慢global get-functions（旧实测107.7s） |
| RE-MCP3.0.3+本地save兼容补丁 | 独立stageB clone、显式database、26calls与持久comment隔离 | 需要独立可写数据库才启；无需为初次pet名字检索冷开 |
| REA6.3.0 | 原B身份、PE fixture20字节、decompile/Pcode与target隔离 | 独立复核用；临时项目、不继承overlay，fixture冷启动约93秒不是当前pet测量 |
| radare2/pdg | 原B字节、局部instructions、decompiler、两client隔离 | 原字节/独立decompiler按需；PDB TPI仍用LLVM |

“已重验”来自新toolchain-revalidation记录，不代表所有上游工具逐一验证或pet业务已经恢复。新REA精选工具包括`inspect_native_instruction/trace_native_values`，此审计只看schema，未调用；schema的深度/图数量限制不能证明它是原游戏live execution trace。

## 5. Operator真实防护与剩余错误边界

读源码确认：worker名字合法字符与长度校验；`--write --worker shared`拒绝；只读profile拒绝WRITES中的run-script/签名/保存等；逐页读取catalog且重复cursor报错；每step默认300秒可单独缩为45/90；遇isError通常中止并保存report。

ReVa run-script另解析payload，要求`success=true`且不timedOut。`allow_error=true`可继续但top-levelok不能证明该step通过。还有每项嵌入error/空结果/错误selector的语义检查责任，不能仅依赖transport退出0。operator保存不是原子rename，job未完成时不可把半写报告当最终。

RE-MCP会stage源副本，patched B hash匹配才APFS复制seed cache；`force_new`拒绝，database明确选。结束有list_databases屏障处理SDK晚通知竞争；屏障通过也不能代替单项结果/保存重开证据。

shared后端结束client不会停止persistentbackend；不能调用stop影响其他agent。专属clone从原seed起，不自动继承shared最新overlay，同名worker复用旧状态；新任务要唯一名。写overlay由集成owner按地址/改前改后/原证据逐项review，不整DB覆盖。

## 6. 首批退出条件

1. 父的pet名字发现/函数bundle每项有实际selector、program、成功/失败及有界输出，原module归属清楚。
2. 最小包能指出真实source/producer/consumer/修改/删除/取消或历史owner，以及未解资源输入；依赖body不计业务。
3. 若seed占位阻碍Pcode，只在专属worker按原ABI恢复；原RNE事实与patched overlay保独立证据。
4. 连接/读取成功只接受工具入口；游戏业务未实现/未验证项保持其状态。不得由工具小试跑声称整个pet或游戏已完成。

本轮仅确认工具入口和一个身份查询。实际pet检索/反编译、ReVa脚本、任何数据库写、构建和oracle均交给父按任务范围统一安排。
