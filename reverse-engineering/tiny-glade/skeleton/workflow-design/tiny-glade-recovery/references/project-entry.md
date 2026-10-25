# 项目索引与 MCP 接入

真实项目根为 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade`。下列 shell 变量只为了方便调用。项目迁移时修改入口变量；workflow.py 可用 `--project` 指定实际项目根。它只读取现有索引，绝不隐式 claim 或调用 MCP。

```sh
TG_PROJECT=/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade
TG_PY=/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python
TG_QUEUE="$TG_PROJECT/skeleton/_tools/task_queue.py"
TG_FLOW="$TG_PROJECT/skeleton/workflow-design/tiny-glade-recovery/scripts/workflow.py"
```

## 1. 找真实 ID，预览角色任务

```sh
"$TG_PY" "$TG_QUEUE" list --category tools --functions --version C --limit 10
"$TG_PY" "$TG_QUEUE" list --category core --version C --limit 10
"$TG_PY" "$TG_QUEUE" show TASK_ID --functions --version C
"$TG_PY" "$TG_QUEUE" show-function FUNCTION_ID
"$TG_PY" "$TG_FLOW" prepare --task TASK_ID --role restore-tools --function FUNCTION_ID
```

把 `TASK_ID / FUNCTION_ID / OWNER / ACTOR / WORK_DIR` 替换为实际值；不要执行字面占位命令。角色有 coordinator、evidence、restore-tools、restore-business、restore-core、reviewer、validator、integrator。函数角色依据函数类别，不只看文件类别；混合核心文件也可能有工具 helper。

预览默认 `implementation=forbidden`、`validation=paused`，未领取的包不能用于直接写候选。生成包列出 task 描述、最多 8 个目标版本函数摘要、索引指纹、原文件/sidecar/DB 查阅路径以及范围是否被截短。可通过 `--limit` 调整至最多 20；批次外范围需另取包。

导航摘要最多展示各 12 个 caller/callee 任务和每函数 6 个原生实例，保存真实总数和截短标记。该上限只压缩首读上下文，不改变领取范围、完整图或验收覆盖；按 task descriptor/SQLite payload 读取当前分析必需的其他实例和消费者。

若出现 stale source，先确认骨架/sidecar 的变化来源，使用既有 `skeleton/_tools/build_task_index.py` 刷新，再运行 `skeleton/_tools/verify_task_index.py`。不要把新 hash 写回旧 claim 来绕过限制；重建保留 claims/work，但旧 owner 的快照变更需保存断点、释放并重新领取。不要在其他 agent 正在刷新时同时重建。此过程不启动恢复，也不自动接受业务行为。

## 2. 在恢复已获授权的未来任务中领取

```sh
"$TG_PY" "$TG_QUEUE" claim TASK_ID --agent OWNER --function FUNCTION_ID
"$TG_PY" "$TG_FLOW" prepare --task TASK_ID --role restore-tools --owner OWNER --function FUNCTION_ID --implementation allowed --output WORK_DIR/prompts/restore-tools.md
```

不限定函数时省略两个命令的 `--function`。output 必须在当前 claim 的 work_dir 内且目标文件尚不存在。prepare 重核对索引 source_inputs、claim owner、函数范围和保存前的 claim 内容。--implementation allowed 只是任务包声明，不替代人类授权；当前建设工作流阶段不执行以上领取命令。

## 3. 原版取证与 MCP

读项目 `AI_REVERSE_TOOLS.md` 的实际 JSON 参数和 `tool-usage/manifest.json`，不要猜 schema。MCP namespace 未加载时，使用现有入口：

```sh
TG_OPERATOR=/Users/yuluo/.codex/mcp-tools/reverse-operator.py
"$TG_PY" "$TG_OPERATOR" ghidra WORK_DIR/jobs/ghidra.json WORK_DIR/evidence/ghidra-result.json --worker shared
```

job/result 必须为已准备的真实文件，不能执行该字面示例。普通共享读取只适用于已核验身份的 B 分析库。写 overlay 或 ReVa 脚本时，将 worker 换为 claim 返回的唯一 worker，追加 `--write`。reviewer 的脚本/overlay 使用任务包的 actor 专属 worker，不与作者共享可写视图。

C 任务先查保留 C 原始/LLVM 证据。C 数据库身份确认前不生成带 C 地址的默认 B MCP job。程序选择 Ghidra 用 `program`，ReVa 用 `programPath`，RE-MCP 显式 `database`；仅按项目说明选用独立 provider。共享库的后续 overlay 不会自动进入 clone。

## 4. 冻结与审查

```sh
"$TG_PY" "$TG_FLOW" freeze --root WORK_DIR --file candidate.patch --file contract.md --file obligations.md --file evidence/raw-report.json --output WORK_DIR/delivery.json
"$TG_PY" "$TG_FLOW" check --manifest WORK_DIR/delivery.json
"$TG_PY" "$TG_FLOW" prepare --task TASK_ID --role reviewer --owner OWNER --actor REVIEWER --delivery WORK_DIR/delivery.json --output WORK_DIR/prompts/reviewer.md
```

freeze 文件列表仅为示例，使用实际存在的候选/合同/证据/依赖快照。根下相对文件名，不可越界，不覆盖旧清单。check 无写入，拒绝已改文件或损坏清单。reviewer 的 actor 必须不同于还原 owner；这只标识任务与隔离 worker，不证明确实由独立模型审查。

现有代码/锁/evidence 在 work_dir 外时，将所需冻结副本放入 work_dir，并在合同记录原路径+hash；reviewer 核对实际原路径及完整依赖闭包。本 helper 不能自动发现所有传递依赖。

validator、integrator 输出包同样要求有效 delivery。validator 默认暂停；只有后续用户授权允许时声明 `--validation enabled`。integrator 仍须单独检查共享工程 base hash 和实际整合授权，不因有模板就应用 patch。

```sh
"$TG_PY" "$TG_QUEUE" release TASK_ID --agent OWNER --reason "说明已保存交付、审查结论或未关闭断点"
```

release 不等于完成/通过；领取 CLI 仍只有既有 list/show/show-function/claim/release，未增加自动完结或状态升级。索引重建、owner 接管、并行消息与启动 agent 均不由本 helper 执行。
