# A/B/C 三版共同函数索引

只收录三个版本都存在、同路径且同符号，并满足三版完整源校验集合相同或三版观察过程字节集合相同的游戏候选函数。不会把仅 A=B 或 B=C 的记录混入此表。

| 类别 | 三版同码 | 三版同源文件 | 两者均满足 | 仅同源文件 | 合并去重 | 完整清单 |
|---|---:|---:|---:|---:|---:|---|
| 工具与低复杂度候选 | 52 | 5 | 3 | 2 | 54 | [全部函数](01-工具与低复杂度候选.md) / [CSV](01-工具与低复杂度候选.csv) |
| 常规业务与待补证 | 121 | 170 | 7 | 163 | 284 | [全部函数](02-常规业务与待补证.md) / [CSV](02-常规业务与待补证.csv) |
| 核心枢纽与高影响 | 38 | 14 | 1 | 13 | 51 | [全部函数](03-核心枢纽与高影响.md) / [CSV](03-核心枢纽与高影响.csv) |
| 总计 | 211 | 189 | 11 | 178 | 389 | |

“三版同码”指每版所有已观察独立函数体的 SHA-256 集合相同；“三版同源文件”指 PDB 中整个原源文件的校验集合相同。两列有重叠。相同字节不保证跨版 ABI、地址指向、依赖、布局或行为等价；同源文件也不证明生成机器码相同。仅名字相同、部分实例相同或缺失字节范围不计入同码。

标准库、已确认公共依赖和通用容器继续排除。GPU/renderer 后置项保留 `deferred` 标记。类别取 A/B/C 函数中的最高影响级别，与包含它的整文件类别可能不同。每类 Markdown 均列出全部函数，不截断。

沿用原文件组 task ID 和 C 函数 ID，A/B/C 共用一个领取锁；这是原任务队列的筛选视图，没有新建重复任务或领取任何任务。先查看函数依赖、签名边界、已有 Rust 候选与保留 B 证据，再领取：

```sh
TG_PY=/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python
TG_QUEUE=/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/_tools/task_queue.py
"$TG_PY" "$TG_QUEUE" show-function <C_FUNCTION_ID>
"$TG_PY" "$TG_QUEUE" claim <FILE_TASK_ID> --agent <UNIQUE_AGENT_NAME> --function <C_FUNCTION_ID>
```

[functions.jsonl.gz](functions.jsonl.gz) 按逻辑组保存完整 A/B/C 函数记录，包括原生范围/字节哈希、caller/callee ID、公库依赖、已有 Rust 候选、分类原因及原文件元数据路径。现有 `.rs.dependencies.json` / `.rs.pdb.jsonl.gz` 保留在各版骨架文件旁。

[summary.json](summary.json) 保存类别计数和输入 SQLite SHA-256；[verification.json](verification.json) 保存三版成员、完整匹配集合、任务复用和落盘覆盖核验。

主索引刷新时自动重建此视图；单独刷新可执行 `skeleton/_tools/build_abc_common_index.py`，仅读取已有数据库，不重新分析二进制。
