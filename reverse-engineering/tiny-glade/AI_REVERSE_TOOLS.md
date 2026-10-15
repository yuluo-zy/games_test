# AI 逆向工具使用说明

适用：本机 Tiny Glade 原生 Rust 游戏逆向。验证日期：2026-10-11。先读本目录 `AGENTS.md`；业务恢复应用 `native-rust-reconstruction`，自定义 Ghidra Python 分析应用 `pyghidra-scripting`。以下入口执行真实 MCP SDK 调用；当前 agent 未加载对应 MCP namespace 时，也可直接通过终端调用。

## 1. 先选工具

| 需要的证据 | 首选入口 | 约束 |
|---|---|---|
| 已有程序的函数、批量反编译、引用、内存、类型 overlay | `ghidra` | 复用持久 B 数据库；每批最多 20 个函数，指定 fields |
| 定点反编译、High P-code、Ghidra API 自定义分析 | `reva` | 使用持久 B 程序；脚本要求专属 `--write` worker，即使脚本只读 |
| 独立数据库、多目标/版本的分析会话 | `re-mcp` | 每个 worker 独立输入副本；每次查询显式 database |
| 独立 provider 的身份与原生代码交叉核对 | `rea` | 临时项目，不能继承持久库注释；冷启动约 93 秒 |
| 原始字节、局部反汇编、独立反编译核对 | `radare2` | 已安装 pdg；不能依赖其 PDB TPI 类型导入 |
| Rust PDB 模块/符号/行号/内联信息 | 已有 LLVM 提取脚本 | 先复用项目 evidence；MCP 无法补造缺失的局部变量记录 |

默认 Codex 只注册 `ghidra` / `reva` 精选只读工具。其余按需启动。完整工具目录落盘，按工具名读取参数；不要把全部 schema 放进模型上下文。

## 2. 最短调用路径

解释器和入口固定为：

```text
/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python
/Users/yuluo/.codex/mcp-tools/reverse-operator.py
```

以下命令可直接运行：

```sh
/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python /Users/yuluo/.codex/mcp-tools/reverse-operator.py ghidra /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/tool-usage/jobs/ghidra-read.json --worker shared
```

任务读取 B 的身份、两个函数、有限引用和 20 字节。stdout 只显示连接/调用摘要；完整结果自动保存到 `evidence/toolchain/session-<backend>-<worker>-<PID>.json`。需要稳定证据路径时，在 job 路径后追加独立的绝对 result 路径。

在提供 `functions.exec` 的 AI 环境中可这样调用：

```javascript
const r = await tools.exec_command({
  cmd: "/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python /Users/yuluo/.codex/mcp-tools/reverse-operator.py ghidra /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/tool-usage/jobs/ghidra-read.json --worker shared",
  yield_time_ms: 1000,
  max_output_tokens: 1000
});
text(r);
```

若返回 `session_id`，用 `tools.write_stdin` 等待完成；不要在完成前读半写入的报告。其他 AI 环境使用其终端执行接口即可。MCP namespace 已可见时，可直接调用所列工具，参数与 job 的 `args` 相同；写工具仍通过专属 worker 入口连接。

## 3. Job 与结果协议

JSON 数组中的步骤顺序执行：

```json
[
  {
    "tool": "get_functions",
    "args": {
      "program": "tiny-glade.exe",
      "functions": "0x14090e510,0x14090e6f0",
      "fields": "signature,decompiled_code",
      "include_call_context": false
    },
    "timeout_seconds": 120
  }
]
```

- `timeout_seconds` 为 operator 每次调用上限，默认 300。ReVa 的 `timeoutSeconds` 是脚本自身上限，放在 args 中。
- 默认遇到 MCP `isError` 或 ReVa 脚本失败就中止并保存失败报告。仅用于预期失败验证时设置步骤的 `allow_error: true`；这时报告 `ok: true` 不代表该步骤成功。
- 检查进程退出码、报告 `ok`、每步 `result.isError` 和返回内容。ReVa `run-script` 还必须满足 `success: true` 且 `timedOut: false`。连接成功不等于目标分析成功。
- 返回正文通常在 `results[i].result.content` 的 text block 中；其中可能再包含 JSON。按实际结构解析，不打印整份 catalog。
- 各 agent 使用独立 job/result 文件；禁止把用户输入直接拼进 shell。用文件工具或 Python JSON 序列化生成 job。

机器可读入口与实测参数快照：[`tool-usage/manifest.json`](tool-usage/manifest.json)。只按需读取对应 `schemas/*.json`；快照来自复验，不替代升级后的现场 schema。

## 4. 参数不可跨工具混用

| 操作 | Ghidra | ReVa | RE-MCP |
|---|---|---|---|
| 程序选择 | `program: "tiny-glade.exe"` | `programPath: "/tiny-glade.exe"` | `database: "ai_tools_b"` |
| 反编译 | `get_functions`：`function` 或逗号串 `functions`，`fields` | `get-decompilation`：`functionNameOrAddress`，`offset: 1`，`limit: 20` | `decompile_function`：`address` |
| 修复签名 | `set_function_prototype`：`function_address`, `prototype` | `set-function-prototype`：`location`, `signature`, `createIfNotExists: false` | 本地精选面无此写工具 |
| 保存 | `save_program`，显式 program | 专属后端正常关闭保存；脚本修改使用事务 | `save_database`，显式 database，再关闭/重开确认 |

Ghidra `read_memory.length` 是整数；radare2 `hexdump.size` 是字符串（如 `"20"`）。Ghidra 多函数参数是逗号分隔的字符串，不是 JSON 数组。未知参数先查 schema，禁止靠另一工具的 API 猜测。

如快照缺少所需工具，现场发现：

```sh
# 不提供 job：只初始化并落盘 catalog；stdout 不输出全部 schema。
/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python /Users/yuluo/.codex/mcp-tools/reverse-operator.py ghidra --worker shared
```

从生成报告选取一个工具：

```python
import json
from pathlib import Path
report = json.loads(Path(result_path).read_text())  # result_path 为本次真实报告路径
tool = next(t for t in report["catalog"]["tools"] if t["name"] == "get_functions")
print(json.dumps(tool["inputSchema"], ensure_ascii=False, indent=2))
```

写 profile 的 schema 用 `--worker <自己的唯一名称> --write` 发现。operator 自动读完 `tools/list` 的 `nextCursor` 分页；单读第一页可能漏掉工具。catalog 中出现的工具也可能在 operator 精选面外；不要绕过 allowlist，以为所有历史工具都验证过。

## 5. 已准备的任务样例

以下路径相对于本说明所在目录；调用时用绝对路径。

| Job | backend / worker | 作用 |
|---|---|---|
| `tool-usage/jobs/ghidra-read.json` | `ghidra --worker shared` | 身份、两函数、引用、20 字节 |
| `tool-usage/jobs/reva-read.json` | `reva --worker shared` | 函数数目和 20 行定点反编译 |
| `tool-usage/jobs/reva-pcode.json` | `reva --worker <唯一名称> --write` | 自定义只读脚本：SHA、签名、最多 12 个 High P-code op |
| `tool-usage/jobs/re-mcp-read.json` | `re-mcp --worker <唯一名称>` | staged B cache 打开、等待、定点反编译、关闭；database 固定 `ai_tools_b`，不同 worker 隔离 |
| `tool-usage/jobs/rea-identity.json` | `rea --worker <唯一名称>` | 打开发行 RNE、确认会话身份、关闭；仅为身份查询样例 |
| `tool-usage/jobs/radare-bytes.json` | `radare2 --worker <唯一名称>` | 发行 RNE 身份、20 字节、6 条指令、pdg 可用性、关闭 |

先复制样例为自己的 job，再修改目标/地址/选择字段。样例地址属于 B。自定义 PyGhidra 脚本的 `code` / `scriptName` / `scriptPath` 三选一。64 位地址用：

```python
addr = currentProgram.getAddressFactory().getAddress("140949c20")
```

直接 `toAddr(0x140949c20)` 曾触发 Java int 重载溢出；不可截断地址规避。写数据库时用 `startTransaction`，成功后 commit，异常 rollback；循环调用 monitor 检查取消。ReVa named script 自动加 worker 前缀；显式写 scriptPath 限定 `/Users/yuluo/ghidra_scripts/<worker>/`。

`reva-pcode.json` 在原 seed 的 `void()` 占位签名下可能仅返回一个 `RETURN`；这是实测的数据边界。结合原机器码确认 ABI 并修复专属 worker 的签名后，再分析完整 High P-code，不能把该输出解释为函数没有业务。

## 6. 并行 agent 与持久性

只读 Ghidra/ReVa 使用 `--worker shared`，共享已分析数据库，多个客户端可以同时请求。需数据库修改或 ReVa 脚本时，每个 agent 使用自己的名称，例如 `roof-owner`、`wall-owner`，追加 `--write`。名称首字符为字母或数字，其余允许字母、数字、下划线、短横线，总长不超过 64。

Ghidra/ReVa 的非 shared worker 自动从原 seed 建立独立 APFS clone；同名 worker 跨任务会复用，因此新任务需要唯一名称。新 clone **不继承共享副本后续 overlay**。同函数的不同签名/注释不自动合并；交付时记录 worker、地址、修改前后、来源和保存/重开证据，由集成 owner 选择。不要复制整个数据库覆盖另一 worker。

REA/radare2 每次 operator 独立进程会话。RE-MCP 每个 worker 独立 stage 输入、数据库 ID 用 `[a-z][a-z0-9_]{0,31}`，所有程序操作带 `database`。不同 agent 不共享可写项目路径，job 内依赖步骤顺序执行。并发连接不等于固定倍数提速。

后端位置：

| 后端 | 共享项目 | 专属项目 / runtime |
|---|---|---|
| Ghidra | `/Users/yuluo/Desktop/game/tools/ghidra-projects/mcp/TinyGlade.gpr` | `tools/ghidra-projects/workers/<worker>/`；`~/.codex/mcp-tools/ghidra/runtimes/<worker>/` |
| ReVa | `/Users/yuluo/Desktop/game/tools/ghidra-projects/reva-test/TinyGlade.gpr` | `tools/ghidra-projects/reva-workers/<worker>/`；`~/.codex/mcp-tools/reva/runtimes/<worker>/` |
| RE-MCP | 无此共享项目 | `/Users/yuluo/Desktop/game/tools/re-mcp-inputs/<worker>/<SHA前16位>/` |

Ghidra mutation 后调用 `save_program`。ReVa 修改后在使用结束时正常停止自己的 backend，再以同 worker 重连核对。管理命令：

```sh
TG_MCP_WORKER=roof-owner /Users/yuluo/Desktop/game/tools/venv/bin/python /Users/yuluo/.codex/mcp-tools/ghidra/launch.py --check
TG_MCP_WORKER=roof-owner /Users/yuluo/Desktop/game/tools/venv/bin/python /Users/yuluo/.codex/mcp-tools/ghidra/launch.py --stop
TG_MCP_WORKER=roof-owner /Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python /Users/yuluo/.codex/mcp-tools/reva/manage.py --stop
```

把 `roof-owner` 替换为自己的实际 worker。`--stop` 只向已验证属于该 worker 的 PID 发 SIGTERM；等进程退出再重连。不要停止其他 agent 仍使用的 shared 后端。客户端结束不等于持久 backend 已停止。不要硬编码端口/PID，manager 自动绑定正确实例。

## 7. 身份与已知失败

默认持久 Ghidra/ReVa 绑定 **B patched EXE**：

```text
path: /Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.exe
sha256: cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323
seed: /Users/yuluo/Desktop/game/tools/ghidra-projects/TinyGlade.gpr
program: /tiny-glade.exe
```

原始指令 oracle 为发行 RNE：

```text
path: /Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne
sha256: f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03
```

查询时核对返回身份。默认库不自动切换到 A/C，不可将 B 地址/类型套到别版。RE-MCP 只有输入 SHA 完全匹配 patched B 才预置 seed cache；其他目标需要先明确分析计划。不要直接调用 raw RE-MCP 对发行路径建库，operator 会 stage 输入；`force_new` 被拒绝。

| 现象 | 判断 / 处理 |
|---|---|
| 默认 catalog 看不到 run-script / 写工具 | 只读 profile 正常；换专属 `--write` worker，无需重新安装 |
| 空反编译、`void()`、缺少局部变量 | 先查机器码和真实调用 ABI；PDB 有占位且缺 Rust S_LOCAL/DEFRANGE，不能归咎连接或编造类型 |
| 新 clone 的签名与共享库不同 | clone 从原 seed 来，共享 overlay 未传播；登记并按证据修复 |
| ReVa 全局 get-functions 很慢 | 131k 函数的 caller/cache 构建实测 107.7 秒；该工具不在精选面，用定点/批量脚本 |
| 程序不对、锁冲突、连接失败 | 查所选 worker `state.json` / `backend.log`、SHA、PID 和独立项目路径；不用 connect_instance 猜同名实例 |
| REA 结果无既有注释/启动慢 | 临时项目能力边界；持久查询换 Ghidra/ReVa |
| radare2 PDB 类型失败 | 仍有 TPI 兼容限制，使用 LLVM；pdg 已实测可用 |
| radare2 缺少 hexdump / disassemble | 先检查 catalog 分页；旧 operator 只读第一页 32 项的问题已修复，当前精选入口无需 run_* 模式 |
| RE-MCP 保存异常 | 3.0.3 本地 session.py 有 GhidraProject.save 兼容补丁；升级后必须重测保存/重开 |
| RE-MCP 退出 BrokenResourceError | 曾有 SDK 晚到通知竞争；operator 已加 list_databases 完成屏障，检查最终报告和保存状态，不能把失败报告当通过 |

完整复验证据：[evidence/toolchain-revalidation/summary.json](evidence/toolchain-revalidation/summary.json)。验证覆盖关键读取、脚本、保存/重开及并发隔离路径；不代表所有上游工具逐一验证，也不构成游戏业务还原验收。

本说明六份样例的实际执行记录：[ai-guide/summary.json](evidence/toolchain-revalidation/ai-guide/summary.json)。包括 catalog 分页修复前的失败报告和修复后的通过报告。

## 8. 复用技能与一键小宠物试跑

任务索引接入与 agent 角色模板使用 [tiny-glade-recovery](skeleton/workflow-design/tiny-glade-recovery/SKILL.md)。其 workflow.py 只准备任务包/检查冻结内容，不代替本说明的 operator、schema、身份检查或工具结果检查。

本轮新增身份绑定原证据包、持久MCP批量、模块交接三个本地技能，原重建技能增加公共Cargo锁对照。可执行入口与真实成本/失败/源码边界见 [RESULTS.md](evidence/toolchain-pet-pilot-2026-10-11/RESULTS.md)，项目recipe由 `scripts/native-workflow/run_pet_pilot.py` 一次执行。所有步骤仍检查各自结果，脚本成功不等于完整游戏业务完成。默认Bpatched数据库与C原发行文件继续分离；不将可变DB反编译结果放入原字节缓存。

## 9. 本机 Windows Claude 辅助调用（2026-10-15）

已实测本机 Claude Code 2.1.299 可用，使用既有配置，调用时不传模型名称。先把最小证据片段写入 UTF-8 prompt 文件，再运行：

```powershell
py -3 C:/Users/liyu/.codex/skills/claude-reverse-assist/scripts/claude_print.py <prompt.txt> <output-prefix> --timeout 300
```

该技能入口直接调用 npm 安装的原生程序，通过 stdin 传递提示词，保留空 `--tools` 参数，并禁用本次分析的工具/MCP。输出为 `.stdout.json`、`.stderr.txt`、`.result.json`；检查进程退出码和报告 `ok`，不得只看 stderr。当前配置会输出 `[claude-code:unrecognized_model]` 日志，但真实请求退出 0、`subtype=success`、`is_error=false` 且返回预期文本，因此此日志本身不能判定调用失败。超时单独记录，不自动重试、不改模型配置。

复验证据：[probe.result.json](evidence/claude-call-repair-2026-10-15/probe.result.json)，5.316 秒返回 `CLAUDE_OK_20261015`。Claude 仅作辅助分析，业务结论仍按原指令/PDB/Ghidra/ReVa 证据验收。
