# Tiny Glade 原作逆向任务的持久上下文

本机逆向工具调用先读 [AI_REVERSE_TOOLS.md](reverse-engineering/tiny-glade/AI_REVERSE_TOOLS.md)：包含真实 MCP SDK 入口、可执行 JSON job、按需 schema 和并行 worker 规则。工具用法以此说明为集中入口。

仅当任务涉及 `D:/game/ljxsj_92385/Tiny Glade` 的逆向与 Rust 还原时，先读 `D:/game/reverse-engineering/tiny-glade/PROJECT_MEMORY.md`、`tracking/project.json` 和 `TASK_BACKLOG.md`。

- 研究原作发行文件/PDB/游戏自有逻辑；不得转去分析用户 `tiny-garden` demo。
- 用户 2026-10-19 明确排除 T13 反馈捕获：直接跳过，不推进其捕获/状态生产者，不再将此项作为本轮三主流程的待完成阻碍；其它未知、未恢复、未验证路径继续按完整调用链推进。
- 正确性优先（用户 2026-10-19 明确要求）：保证游戏逻辑、算法和设计正确，以及整个游戏的交互完整性。不得简化原作复杂业务或算法，不得省略分支、状态、边界、副作用、生命周期及跨模块交互来换取编译/测试通过或进度。允许多态、模块划分和资源所有权等设计更新，但必须保持完整业务语义；证据不足的路径继续取证并明确未闭合，不能以默认值、空操作或跳过功能冒充完成。
- 优先恢复可编译 Rust 源码与类型/控制流；场景回放和 GPU 暂时后置。
- 依赖版本/来源以原作 `build-info/Cargo.lock` 为准。直接复用原依赖；不得逆向或重新实现 std、Bevy、glam、fastrand、half、serde 等通用库。
- 用户说明引擎是 Bevy 分叉；按此作为项目约束记录。已证实 Bevy family 0.16.0，具体分叉仓库/commit 尚未核实，不编造。
- 私有游戏模块与自有工具函数仍需恢复；仅对必要的引擎分叉改动检查差异，不重新还原整个引擎。
- 逆向主力为 Ghidra/PyGhidra 持久数据库、LLVM PDB 提取和原指令差分。默认 MCP 为 `ghidra`（批量函数/引用/类型）与 `reva`（定点反编译/PyGhidra/P-code）。REA、radare2、RE-MCP已复验恢复，按需SDK使用；不暴露全部schema。脚本编排本身不计作逆向交付。

这些规则只适用于上述逆向任务，不约束本目录中的其他工程。

三版 Tiny Glade 的逆向统一遵守 `reverse-engineering/tiny-glade/DEPENDENCY_FILTERING.md`：先查 Cargo/PDB 来源与定义模块，排除已确认库实现，恢复游戏调用策略。不要把含 Wall/Roof 的通用库泛型实例当作游戏业务，也不要把全部 Cargo/local 包一律当作游戏私有。优先完成建筑系统的权威状态、生命周期、几何、附着及历史联接。

## Token 效率约定

本仓库的逆向任务单次请求固定开销很大，遵守以下约定：

- `ghidra` 经 `~/.codex/mcp-tools/ghidra/launch.py` 连接严格绑定PID的共享后端。共享只读14工具，专属write worker19工具；配置在同目录 `tools.json`。`reva`共享只读6工具、write worker10工具。
- 当前 MCP 使用 `tools/ghidra-projects/mcp/TinyGlade.gpr`：从现有 B 数据库克隆的独立持久项目，避免与已有只读/批处理工作争抢项目锁。原 `TinyGlade.gpr` 和原 RNE 验证基线保留。MCP 注释/类型修改持久保存到副本；不得宣称已同步原数据库。
- 用 `find_functions` 定位；`get_functions(functions=..., fields=...)` 每批最多 20 个，按需取 signature/parameters/locals/decompiled_code/callees 等字段，并显式指定 `program="tiny-glade.exe"`。只读字节用 `read_memory` 限定范围。
- PDB 的 `void()` 占位不能当真实 ABI；空反编译先核对机器码/调用点与签名，证实后修复并保存。MCP 不会补造缺失的 Rust 类型或局部变量。
- 已加载的 skill 和证据复用，不在每轮重新全文读取。批处理入口仍为 `scripts/multi-version/mcp-native.mjs`，现已改接官方 Python MCP SDK，不依赖 REA 包。

## 多 agent 工具使用（2026-10-11 复验）

- Agent自行调用统一SDK入口 `/Users/yuluo/.codex/mcp-tools/reverse-operator.py`；解释器 `/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python`。用法 `reverse-operator.py <ghidra|reva|re-mcp|rea|radare2> <job.json> <result.json> --worker <唯一模块名>`。需要确认过的数据库/脚本修改时追加 `--write`，由agent按已授权任务自行选择，无需父agent逐调用转发。
- 共享profile只读。每个修改agent使用唯一worker；Ghidra/ReVa自动为其建立独立APFS持久clone和后端。ReVa named脚本按worker前缀隔离。同函数的不同签名/注释不会自动合并，交付时登记overlay/revision证据，由集成owner裁决。
- 所有程序级查询显式指定程序/数据库。Ghidra `program="tiny-glade.exe"`；ReVa `programPath="/tiny-glade.exe"`；RE-MCP每个agent独立输入副本与唯一database selector，禁止省略selector导致读错worker。
- RE-MCP operator自动把输入stage到工具目录，匹配B patched SHA时预置既有数据库clone；未匹配版本不套B数据库，默认不全程序分析。3.0.3已打有记录的GhidraProject.save兼容补丁，依赖升级后须重测保存/重开。
- REA每agent独立stdio会话，临时Ghidra项目不能替代持久库。radare2已补pdg后端，用于原字节/反汇编/独立核对；当前PDB TPI兼容仍不足，类型/内联/行号主力为LLVM。
- 原始B数据库和发行文件保留；当前Ghidra/ReVa持久worker只绑定B patched EXE，原RNE继续作机器码oracle。A/C需单独身份与分析状态，不靠同地址推断等价。
- 实测报告入口 `reverse-engineering/tiny-glade/evidence/toolchain-revalidation/summary.json`；并行请求重叠不等于倍数提速，脚本/连接/工具测试不计为游戏业务验收。
