---
name: tiny-glade-recovery
description: "Coordinate Tiny Glade native-to-Rust recovery and independent review using its A/B/C task index, original evidence and existing MCP workers. Use for task preparation, bounded reconstruction, review and integration in this project; respect planning-only and paused-validation requests."
---

# Tiny Glade 项目还原与审查

## 入口与当前授权

本 skill 的真实目录在 Tiny Glade 项目的 `skeleton/workflow-design/tiny-glade-recovery/`，本机 `~/.codex/skills/tiny-glade-recovery` 仅为发现入口。沿真实路径向上定位含 `AI_REVERSE_TOOLS.md` 的项目根；不要从当前 cwd 猜到 tiny-garden 或其他工程。

先读项目 `AGENTS.md`、`PROJECT_MEMORY.md` 的最新断点、`TASK_BACKLOG.md` 当前选择，以及 `skeleton/task-index/AGENTS.md`。执行 MCP 前读 `AI_REVERSE_TOOLS.md`。遵守用户最新授权：本 skill 和模板不授权恢复代码、验证、整合、启动 agent 或给其他线程发消息。2026-10-11 安装时仅授权工作流建设，游戏恢复未启动、业务验证仍暂停；以后以用户明确的新指令为准。

## 选择模式，只读必要资源

- 规划/领取：读 [项目入口与命令](references/project-entry.md)，使用 `coordinator` 模板。
- 有界取证：使用 `evidence` 模板和项目工具说明；复用原始证据，不重新扫描全 PDB。
- 还原：先应用 [native-rust-reconstruction](/Users/yuluo/.codex/skills/native-rust-reconstruction/SKILL.md) 与项目依赖排除，按函数范围选择 `restore-tools / restore-business / restore-core` 模板。
- 审查、验证、整合：读 [交付与门槛](references/delivery-and-gates.md)，分别使用 `reviewer / validator / integrator` 模板。reviewer 独立核对原始事实，不以作者解释作为唯一依据。

模板位于 `assets/tasks/`，只读当前角色。`scripts/workflow.py prepare` 从真实 SQLite 索引生成角色任务包；默认 stdout 预览、禁止实现、暂停验证，不领取、不启动 MCP。命令详见 project-entry。

## 不可省略的项目边界

1. 同一路径 A/B/C 只领取一个文件组；函数范围缩小也保持文件组独占。复用现有 `task_queue.py`，不要另建锁、超时抢占或修改他人的 claim。多个文件的链需各自合法锁。
2. 有 C 时以 C 为目标；没有 C 保留最新存在版，不强行移植。同源 checksum、同码、名字候选和已验证 B 只支持各自范围的复用候选。
3. 默认 Ghidra/ReVa 库是 B patched EXE。唯一 worker 不切换版本；C 地址只能查询身份确认的 C 库。原发行 oracle 与 patched 分析库分开。
4. 公共库、std、容器、Serde/ECS 泛型内部直接复用锁定依赖；私有/path 路径不自动等于游戏源码。库内游戏 callback 单独确认。
5. 合同区分观察事实/推断/未知，包含真实 producer、consumer、字段与 ABI、依赖、副作用、失败分支和适用的编辑/历史/删除。PDB `void()`、缺 locals 或空反编译不能补造签名/默认值。
6. 游戏候选输出仅在 claim 的 work_dir。共享生产 Cargo/lib/注册/CLI 由获授权的整合 owner 应用；不覆盖骨架、原发行文件或共享数据库。
7. 一批 Ghidra 查询最多 20 个函数，指定 fields；嵌套错误、缺失、超时和截断要记录。overlay/ReVa 脚本用自己的唯一可写 worker，保存读回与版本身份。原始结果留盘，LLM 读取相关片段。

按需路由 [native-evidence-pack](/Users/yuluo/.codex/skills/native-evidence-pack/SKILL.md)、[native-tool-batch](/Users/yuluo/.codex/skills/native-tool-batch/SKILL.md)、[pyghidra-scripting](/Users/yuluo/.codex/skills/pyghidra-scripting/SKILL.md) 和 [native-module-handoff](/Users/yuluo/.codex/skills/native-module-handoff/SKILL.md)。这些能力已有，不在本 skill 中复制其实现。

## 交付与接受

合同先就绪，再在授权范围内写候选；核心任务先审合同。冻结候选及证据/依赖清单后交独立 reviewer。审查通过仍为 `implemented_unverified`；仅目标版本的有效 runtime + verification 事实支持对应 handoff link 的 `verified`。按当前授权执行原码差分和真实业务链验证，暂停时只记录待验收。

`workflow.py freeze/check` 只检查明确列举文件的内容指纹，不检查 ABI 真伪、验证覆盖或游戏等价；完整依赖闭包由交付者声明、reviewer 复核。handoff 使用原有四种 link 状态和原审计器，不用新阶段替代其 schema。代码、合同、证据或依赖变化使相关旧报告失效。

完整设计见 [项目方案](../LLM_RECONSTRUCTION_WORKFLOW.md)。按消费者缺口推进完整建筑链，GPU/场景后置。完成时分别报告新增事实、候选实现、审查范围、目标版本验证范围及剩余消费者；索引/工具/audit 成功不计作游戏恢复完成。
