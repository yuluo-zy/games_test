# A/B/C 逆向任务领取索引

领取与角色任务模板入口：[tiny-glade-recovery 项目 skill](../workflow-design/tiny-glade-recovery/SKILL.md)；真实 CLI、MCP 接入与冻结审查命令见其 [project-entry](../workflow-design/tiny-glade-recovery/references/project-entry.md)。模板生成不自动领取或启动恢复。

这个索引将版本演进视为一套逻辑模块。一个任务持有同一路径的 A/B/C 文件组，以 C 为主目标，A/B 作历史和已有恢复参考；没有 C 的文件保留最新存在版本，不强行移植。索引不修改 `reconstruction/`，也不宣称已恢复任何游戏行为。

当前快照覆盖 3,667 个 Rust 文件/元数据占位变体与 39,169 个函数条目，形成 1,250 个跨版本文件组任务：工具文件组 4、常规业务 687、核心高影响 559。函数级工具候选有 636 个版本条目，包含混合文件中的 helper。2,395 个游戏候选逻辑函数组有同源文件或同码匹配，其中 485 组至少有一对版本的观察函数体字节集合一致。完整指标见 [summary.json](summary.json)，索引覆盖、交接引用及独占领取检查见 [verification.json](verification.json)。刷新后以这两个 JSON 为准。

## 三类

| 类别 | 文件索引 | 函数索引 | 领取用途 |
|---|---|---|---|
| 工具与低复杂度候选 | [01](01-工具与低复杂度候选.md) / [全部文件 CSV](01-工具与低复杂度候选.csv) | [工具函数 CSV](01-工具与低复杂度候选-函数.csv) | 小型数学、坐标、曲线、访问器；先确认 ABI/字段和匹配依赖，再直接写语义 Rust |
| 常规业务与待补证 | [02](02-常规业务与待补证.md) / [全部文件 CSV](02-常规业务与待补证.csv) | [业务函数 CSV](02-常规业务与待补证-函数.csv) | 有界业务逻辑、局部状态处理，或证据不足的函数；按消费者补输入/输出合同 |
| 核心枢纽与高影响 | [03](03-核心枢纽与高影响.md) / [全部文件 CSV](03-核心枢纽与高影响.csv) | [核心函数 CSV](03-核心枢纽与高影响-函数.csv) | 多调用者、多文件影响，或创建/更新/编辑/历史等协调者；优先确认权威状态和接口 |

同一文件通常有多种函数，文件按其中最高影响分类，函数保持各自类别。工具函数索引因此也包括常规/核心文件中的小 helper；可以领取单个函数范围，但仍锁定整个跨版本文件组，避免两个 worker 同时改一个文件。高调用量的小函数会标为核心高影响，同时保留“低复杂度”难度标记，避免把重要性和实现难度混淆。

每版全部文件导航：[A](A-文件分类.csv)、[B](B-文件分类.csv)、[C](C-文件分类.csv)。标准库、公开依赖、已确认泛型容器与纯引用/manifest 占位文件保留在导航中，`excluded_reason` 明确原因，不进入游戏实现队列。这是依赖边界，不是第四种逆向优先级。库内的游戏 callback 仍须在相应游戏 owner 中恢复。

## 判断依据和限制

- 调用者以版本内逻辑函数去重，不把重复内联和 monomorph 实例累加。直接内联、物理普通调用和完整程序的物理引用分别保存；传递内联、间接目标和共享地址别名不用于伪造大量确定调用者。
- 工具候选要求已有独立过程、最大观察体长不超过 256 字节、最多两个可归属游戏 callee、没有已观察的间接/歧义目标，并具有工具/访问器语义线索。这是静态低复杂度候选，不是完整签名或立即可运行代码已知。
- 核心候选包括至少 8 个去重调用者，或至少 5 个调用者来自 3 个外部文件；大体长、多个游戏消费者的协调函数也进入核心。阈值、原因、影响评分和完整 caller/callee ID 在函数索引中可查询。
- 多个源码行块重叠时，物理代码归属不等于定义 owner；公共别名和宏/内联归属仍要核实。源文件/函数分类是规划假设，不是行为证明。
- `existing_rust_name_candidates` 只是现有 Rust 的名字候选；`retained_B_native_evidence` 是已有 B 范围证据。开工前检查它们，避免重复实现。B 已有数值验证不自动证明 C 完成。
- GPU/renderer 等当前后置文件仍分类展示，但标为 `deferred`，不会出现在默认活动领取列表。

## A/B/C 相同项

三个版本都满足条件的完整三类清单：[ABC 三版共同函数索引](ABC-三版共同项/README.md)。当前三版同码共 211 组（工具 52、常规 121、核心 38）；仅三版同源文件另有 178 组，合并去重为工具 54、常规 284、核心 51。每个函数沿用原文件组领取锁，以该子索引的 summary.json 为刷新后的计数依据。

人类可读导航：[同源与同码候选](跨版本相同项.md)；完整 [CSV](跨版本相同项.csv)。同一路径任务只领取一次。

`function-groups.jsonl.gz`、SQLite `functions` 的 `cross_version_pairs` 与每个任务记录中标明：

1. `same_file_source_checksum_set`：PDB 中整个源文件的 SHA-256 集合相同，证明该文件的输入文本相同；不能从中取回文本，也不能直接证明依赖/feature/布局相同。
2. `same_native_byte_sets`：该同路径、同符号候选的所有观察独立过程字节指纹集合相同。`some_native_instances_identical` 只表示部分实例匹配。
3. 仅名字/路径一致：保持版本候选，不能称为函数行为相同。缺独立范围的内联函数只报告已有源校验事实。

相同机器码可以具有不同调用目标或上下文；不同机器码也可能只是相对地址、编译器或优化变化。字段布局、ABI、功能行为和调用策略的跨版等价始终单独确认。没有按同址/字节指纹合并不同名字的函数。任务按跨版文件组去重，禁止按 A/B/C 创建三个重复 owner。

## 领取命令

解释器与脚本均为现有本机文件，无需新服务或重新扫描 PDB：

```sh
TG_QUEUE=/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/_tools/task_queue.py
TG_PY=/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python
"$TG_PY" "$TG_QUEUE" list --category tools --functions --version C --limit 10
"$TG_PY" "$TG_QUEUE" list --category core --version C --limit 10
"$TG_PY" "$TG_QUEUE" show <TASK_ID> --functions --version C
"$TG_PY" "$TG_QUEUE" show-function <FUNCTION_ID>
"$TG_PY" "$TG_QUEUE" claim <TASK_ID> --agent <UNIQUE_AGENT_NAME>
"$TG_PY" "$TG_QUEUE" claim <TASK_ID> --agent <UNIQUE_AGENT_NAME> --function <FUNCTION_ID>
"$TG_PY" "$TG_QUEUE" release <TASK_ID> --agent <SAME_AGENT_NAME> --reason "交付已保存或停止，说明断点"
```

领取使用独占文件锁，重复领取会失败，A/B/C 共用同一个 owner。不会自动超时抢占。源快照变化时拒绝基于旧索引领取，应先重新生成索引；重建不删除已有 claim。暂停/释放保存历史，不会丢弃 worker 工作。

工作写入 `work/<TASK_ID>/<AGENT>/`。领取结果给出唯一 worker、原始 artifact 身份和交接 ledger。提供独立候选 Rust/patch、版本适配、原字段/ABI 合同、实际输入→消费者→修改/删除/历史边界；父 owner 最后整合共享 Cargo/lib/注册/CLI。不要覆盖骨架、原发行文件、共享 DB 或其他 agent 输出。

`handoffs/<TASK_ID>.audit.json` 已按 `native-module-handoff` 检查引用、版本身份格式和 source fingerprint；这只证明交接引用存在，`semantic_acceptance` 始终 false。未知生命周期/消费者明确留在 ledger，不能填零来通过审计。遵从用户当前对业务验证的限制，索引不会自动启动业务恢复或验证。

当前默认 MCP 分析库仍绑定 B patched EXE。唯一 worker 只隔离写作业，不会自动切换到 C；处理 C 时先用已绑定的原始 C/LLVM 证据，确认 C 数据库身份后才查询其反编译。禁止把 C 地址、字段或函数范围发送到 B 库并当作 C 结果。

## 机器查询与刷新

- `index.sqlite3`：`files / functions / edges / tasks`，可直接用 Python 标准库只读查询。
- `functions.jsonl.gz`：全部函数分类、难度原因、去重 callers/callees、字节指纹、跨版匹配和已有 Rust 候选。
- `files.jsonl.gz`：所有源文件，包含明确排除项。
- `tasks.json`、`tasks/*.json`：跨版本领取包与相关上下游任务；相关 callee 不是未经分析的强制阻塞项。
- `summary.json`：覆盖计数、版本对比和输入指纹。

刷新脚本为 `skeleton/_tools/build_task_index.py`。保留 claims/work；不要把刷新分类计数写成游戏恢复完成率。


### 用户明确授权后置函数时

`claim` 增加 `--authorized-deferred-reason`：仅在用户明确把deferred函数列入当前范围时使用，并记录人类授权原文/范围；仍保留deferred索引分类、原source指纹和文件组独占锁。建议同时传 `--function` 限定授权函数；该参数不构成自动授权，不改变默认活动任务队列。2026-10-12仅用于用户指定52组中的AllocationHandle::is_null。
