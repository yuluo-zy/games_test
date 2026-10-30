## 当前 Mac 入口（2026-10-24）

2026-10-24 现场修复记录：[`tool-repair-2026-10-24.json`](../../evidence/toolchain-revalidation/ai-guide/tool-repair-2026-10-24.json)。共享 Ghidra 的旧进程可读元数据却无法访问符号索引，报 `Corrupted BufferMgr state`。独立 APFS 副本证明保存库可读后，获用户明确授权，仅重启该共享实例；没有重建数据库或覆盖其他 worker。触发进程损坏的具体原因尚未确定，不将磁盘余量或长时间运行直接认定为根因。

当前 launcher 的 `--check` 会读取已知 B 函数签名和 20 字节原指令，不再只以 metadata 存活判断健康。发现同类问题时先保留失败结果、在独立副本验证保存库，再协调共享实例重启；不要自动杀进程、删锁或回退原 seed。operator 默认结果目录已修正为当前 `reverse-engineering/tiny-glade/evidence/toolchain/`。工具首次启动需要运行时目录写权限和本机 socket 访问，沙箱拒绝时用受审查的执行权限，不修改全局权限或把权限失败当数据库损坏。

客户端也有独立生命周期：Ghidra bridge 不再永久锁死启动时 PID，而是在断链重连时核对同一 worker 状态文件的项目路径、程序名及原作 SHA，再跟随新 PID；同名 `TinyGlade` 不足以授权换库。ReVa stdio 长驻，但每次工具调用使用新的 HTTP MCP 会话，避免空闲后的失效会话；异常必须返回具体错误，写请求不自动重放。**修复前已加载的 Python 客户端需要重新建立 MCP 连接才能加载新代码**，不能只因新 SDK 进程通过就宣称应用内旧客户端通过。本机 Computer Use 禁止控制 Codex 本身，用户也未找到刷新入口；不要求寻找未核实的按钮或重启整个应用，当前用已复验的 SDK/operator 继续，旧客户端重建后加载修复。

本轮验证区分五个 MCP 后端和六种任务样例（ReVa 含反编译与脚本两种），另用 LLVM 读取真实原指令。两种 stdio 客户端都通过“同一客户端→只重启自身独立 worker→再次查询”的实测；跨项目、不同 SHA、不同程序名的重连被拒绝，写请求传输失败仅尝试一次。详见现场记录中的完整结果、脚本和保留的首轮验证脚本失败。

```bash
/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python /Users/yuluo/.codex/mcp-tools/reverse-operator.py ghidra /Users/yuluo/Desktop/game/reverse-engineering/tiny-glade/tool-usage/jobs/ghidra-read.json --worker shared
```

## Windows 备选入口（2026-10-19 续验）

Windows 主机（cwd 为 D:/game）优先使用 [Windows 持久 SDK 说明](../../scripts/reverse_tools/README.md)。解释器为 D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe，operator 为 D:/game/reverse-engineering/tiny-glade/scripts/reverse_tools/reverse_operator.py。下文的 `/Users/yuluo` 路径属于 Mac 检出，当前 Mac 工作区根为 `/Users/yuluo/Desktop/game`。

本轮 color-menu-full 专属 write worker 已复验启动；Windows兼容 ghidra 适配器底层使用 ReVa/Ghidra，并不表示旧 ghidra MCP服务已安装。按README每worker独立持久副本、明确程序身份；需要prepare/script/signature修改时使用唯一worker+--write。禁止重启其它worker。真实schema按单工具读取；工具复验不算业务恢复。旧Mac overlay不宣称已同步Windows库。

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
/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python /Users/yuluo/.codex/mcp-tools/reverse-operator.py ghidra /Users/yuluo/Desktop/game/reverse-engineering/tiny-glade/tool-usage/jobs/ghidra-read.json --worker shared
```

任务读取 B 的身份、两个函数、有限引用和 20 字节。stdout 只显示连接/调用摘要；完整结果自动保存到 `evidence/toolchain/session-<backend>-<worker>-<PID>.json`。需要稳定证据路径时，在 job 路径后追加独立的绝对 result 路径。

在提供 `functions.exec` 的 AI 环境中可这样调用：

```javascript
const r = await tools.exec_command({
  cmd: "/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python /Users/yuluo/.codex/mcp-tools/reverse-operator.py ghidra /Users/yuluo/Desktop/game/reverse-engineering/tiny-glade/tool-usage/jobs/ghidra-read.json --worker shared",
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

机器可读入口与实测参数快照：[`tool-usage/manifest.json`](../../tool-usage/manifest.json)。只按需读取对应 `schemas/*.json`；快照来自复验，不替代升级后的现场 schema。

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
| `tool-usage/jobs/reva-pcode.json` | `reva --worker <唯一名称> --write` | 原指令 ABI 签名修复，再执行只读脚本：SHA、签名、最多 12 个 High P-code op，断言有 FLOAT_DIV |
| `tool-usage/jobs/re-mcp-read.json` | `re-mcp --worker <唯一名称>` | staged B cache 打开、等待、定点反编译、关闭；database 固定 `ai_tools_b`，不同 worker 隔离 |
| `tool-usage/jobs/rea-identity.json` | `rea --worker <唯一名称>` | 打开发行 RNE、确认会话身份、关闭；仅为身份查询样例 |
| `tool-usage/jobs/radare-bytes.json` | `radare2 --worker <唯一名称>` | 发行 RNE 身份、20 字节、6 条指令、pdg 可用性、关闭 |

先复制样例为自己的 job，再修改目标/地址/选择字段。样例地址属于 B。自定义 PyGhidra 脚本的 `code` / `scriptName` / `scriptPath` 三选一。64 位地址用：

```python
addr = currentProgram.getAddressFactory().getAddress("140949c20")
```

直接 `toAddr(0x140949c20)` 曾触发 Java int 重载溢出；不可截断地址规避。写数据库时用 `startTransaction`，成功后 commit，异常 rollback；循环调用 monitor 检查取消。ReVa named script 自动加 worker 前缀；显式写 scriptPath 限定 `/Users/yuluo/ghidra_scripts/<worker>/`。

`reva-pcode.json` 现在先在专属 worker 修复已核实的 `float aspect_ratio(unsigned int *size)` 签名，再执行脚本并断言存在 `FLOAT_DIV`。依据为原 B RNE `140949c20..140949c33` 的 20 字节指令：从 RCX 指向的两项 u32 读取、零扩展、转 float、相除，经 XMM0 返回。原 seed 的 `void()` 占位签名只生成一个 `RETURN`，该历史结果仅证明脚本运行，不证明取得有效函数数据流。新样例会修改专属 worker 的签名，不能在 shared 上执行。正常关闭再重开只读核查签名与 P-code，才构成保存/重开证据。

## 6. 并行 agent 与持久性

模型出现 `Selected model is at capacity` 时，可恢复的调度调用保留任务与检查点，最多三次退避重试（例如1、2、4秒）；仍失败则使用用户已授权的可用模型续派，记录未完成责任，不把过载当业务失败或完成。当前会话若被服务端直接终止，agent 无法在终止期间主动重试；下次恢复须从已保存检查点接续。只读工具的瞬态传输失败可有限重试；有副作用或结果未知的写请求先核实际结果，不盲目重放。不要为应对容量错误新建并行生产实现、重启共享逆向后端或放宽验收。

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

### 6.1 本轮证据文件与共享 Cargo（2026-10-27）

**证据写入也需要 owner 与防覆盖检查。** 本轮已在自己创建的临时文件复验：`apply_patch` 的 `Add File` 可成功替换已有路径，不能把操作名当作 no-clobber 保证。写入前先检查目标是否存在、文件内实际 VA/版本/函数名和已有 SHA；已有 oracle 属于其他 owner 时保留原文件，协调复用或使用明确的本人前缀/VA 名，不以新结果覆盖旧失败。无人占用的新证据应采用真正的排他创建，例如 Python `Path.open("x")`；调用普通写工具前的 exists 检查只防常见误写，不是并发原子锁。写后核对路径、内容 VA、SHA 和夹具引用，报告区分原件 oracle 与候选输出。

共享构建出现 `ENOSPC` 时先检查磁盘及正在运行的 `cargo`/`rustc`，将环境失败与业务断言失败分开保留。**本轮由中央确认编译器均已退出后，仅删除共享 `target/debug/incremental`**，保留源码、deps、原证与夹具；这不是授权其他 worker 随时清 target。后续统一 `CARGO_INCREMENTAL=0`，继续使用现有 target/debug/profile，不另设 `CARGO_TARGET_DIR`、复制 target、清 deps 或改变 profile 来抢过锁。编译锁等待与其他 owner 的接口中间态先协调，不擅改他人源。

已实跑示例（在 `tiny-glade` 工作区内）：

```sh
CARGO_INCREMENTAL=0 cargo test --offline -p utils --lib original_expand_along_edge_complete_function_matches_native_bits_and_panics
```

磁盘故障、实际重跑及同 target 边界见 [共享几何复核](../../tiny-glade/reports/module-recovery-2026-10-23/m01-shared-geometry-independent-review.md)、[自由墙复原](../../tiny-glade/reports/module-recovery-2026-10-23/m01-freehand-recovery-2026-10-26.md)。不要把 `ENOSPC` 后未执行的测试记为通过，也不要把删缓存记为业务进度。

### 6.2 模块先盘点、证据批量补缺、实现后分层验收（2026-10-29）

每轮先按B原目录与PDB列出当前业务模块的函数，核对已有原证，再开始复写。清单至少包含：规范VA/RVA、物理范围、原源码归属、泛型实例／内联／别名关系、原证路径和完整性、Rust owner／源码指纹、调用与资源生产消费关系、生产注册、已执行验证及下一动作。**证据、算法、接线、验证分别记状态**；文件存在、同名函数、测试数量或旧进度统计均不能推导模块完成。未独立核验项明确记`not_reviewed`。

先查现有报告、原指令oracle、完整汇编和已修复的反编译。仅在缺证、截断、身份变化、源码与证据矛盾或调用合同不清时重新逆向，并记录原因。同一个规范入口由一个owner维护证据；完整证据直接复用。新增业务文件前先核B原归属，维持现有重构目录，不另造平行实现。

图工具先读[code-map入口](../../tools/code-map/README.md)，复用已发布snapshot，只读批量查询`search → neighbors/paths → state_users → evidence_pack/assembly`。先核调用者和资源写者，再核消费者、事件、历史、视觉和卸载；实例化包装与普通tuple／chain／IgnoreDeferred仍需原注册体证明。地址search带`0x`；函数key使用`function:B:RVAhex`。保留snapshot、原件指纹、查询参数、`truncated/next_offset/unknowns`，相关有界集合分页取完；数据库无边不代表原程序无调用。`module_map`只列模块，不能当业务函数完整目录。静态CALLS、curated关系与汇编范围覆盖不证明运行派发、CFG完整或业务验收。

按缺口集合批量取原证，检查每个嵌套结果的错误、截断、真实物理范围及跳表／EH后才实现。验收依次区分原指令差分、真实World资源／事件链、生产App调度、真实GPU和OS输入；限定样本不能升级为整模块结论。中央review负责共享接口及调度，避免多个owner重逆向同一函数或互改共享资源。Cargo由owner轮转，其他owner继续只读盘点，不重复排队构建。

编译按源码批次收口：owner先集中完成本轮断言与修改，再共享一次编译产物依次运行相关过滤组，避免每加一条断言触发全依赖重编；批次结束记录实际源码指纹，下一批新修改另验。测试命令退出0还要确认确实运行了目标测试；模块路径过滤错误导致`running 0 tests`不能记通过。真实App事件验收必须核整个启动调用顺序：本工程Bevy在`Events<T>`已被裸World初始化时，晚到的`App::add_event`会跳过维护注册；恢复通道要在最早App注册入口完成，并用实际多帧过期反例验证，不能只检查资源存在。

当前实践：[装饰UI函数基线](../../../tiny-glade/reports/module-recovery-2026-10-23/m01-ui-function-inventory-2026-10-29.json)。其中同名源码及文件名含VA只自动产生候选；尚未审核条目不能统计为逆向完成。后续轮次先消费这份清单和各模块owner清单，再查询缺口。`tools/code-map-runtime/`仅本机缓存与运行状态，不提交。

本轮独立review经验：①`ResMut`包装中的changed tick写入不是业务状态复位，先还原参数真实owner，保留无条件变更副作用；②结构采样`Vec::resize`只初始化新增元素，先追请求生产者是否每帧重建，不凭叶测试手工clear或改fill修正表象；③命令tuple的尾部字节不必然属于其中业务组件，须对照实际consumer读取面与对齐；④原rank是否就是mesh selector要核中间构造函数的实参，不能按命名臆断；⑤后台服务启动成功不等于后续可用，Neo4j daemon被宿主回收时按[code-map运行说明](../../tools/code-map/README.md)保持受管console并执行实际查询，再通知每个owner，不能静默改成无图流程。具体楼梯／楼板VA和样本结果留模块报告，不作跨版本通用常量。

M0／M1／M6接续经验：①原脏队列是否去重要看push入口，不能按旧注释给Vec加contains；原重复队列可能承担顺序和重复重建语义。②原曲线包含两个Vec和总长度时有效对象可占0x38，不能把0x30猜成下一字段起点；事件记录stride8也可能只有event ID，实际事件为零大小。③恢复ECS查询时使用现有Option／Query.get校验，不复制旧Entity私有generation位型判断。④生产App测试先批量盘点全部SystemParam及原资源构造，复用真实注册入口和资产初始化；独立World预先建树后搬进App会与OnEnter重复建树，且可能覆盖App的调度资源。⑤真实Vulkan失败先区分沙箱与loader搜索路径；本机已验证的M3配置为`VK_ICD_FILENAMES=/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json`、`VK_LAYER_PATH=/opt/homebrew/share/vulkan/explicit_layer.d`、`DYLD_FALLBACK_LIBRARY_PATH=/opt/homebrew/lib`，严格验收同时设`RHAPSODY_REQUIRE_VULKAN=1`并使用经审查的设备访问权限，不因ERROR_INCOMPATIBLE_DRIVER判机器无GPU。⑥有限几何样本通过后仍需非有限、退化、旋转、跨环和相邻ULP；已有M6反例表明圆与矩分支对NaN可能分别进入原panic和返回None，不能统一为“未相交”。

字段语义按原publisher来源确认，不按旧注释或变量名：本轮已证明PublicWall `+0x280`来自elevation，先后在楼板与抹灰音效发现被误称height；用不同height/elevation的权威输入验收实际调用层。颜色、时间和代数也应使用当前Rust语义API，禁止把旧ABI偏移直接套到手工重构后的布局。看似反直觉的原行为先保留：剥落值吸附目标的最后一帧原本不触发重建；改成“更合理”的规则前必须有用户明确要求，不能混入逆向还原。

## 7. 身份与已知失败

默认持久 Ghidra/ReVa 绑定 **B patched EXE**：

```text
path: /Users/yuluo/Desktop/game/B版本内容/tiny-glade.exe
sha256: cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323
seed: /Users/yuluo/Desktop/game/tools/ghidra-projects/TinyGlade.gpr
program: /tiny-glade.exe
```

原始指令 oracle 为发行 RNE：

```text
path: /Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne
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

完整复验证据：[evidence/toolchain-revalidation/summary.json](../../evidence/toolchain-revalidation/summary.json)。验证覆盖关键读取、脚本、保存/重开及并发隔离路径；不代表所有上游工具逐一验证，也不构成游戏业务还原验收。

本说明六份样例的实际执行记录：[ai-guide/summary.json](../../evidence/toolchain-revalidation/ai-guide/summary.json)。包括 catalog 分页修复前的失败报告和修复后的通过报告。

### 7.1 跳表 CFG 污染、真实范围与持久修复（2026-10-27）

本轮 shared `ui_move_shapes@14097a060` 报 body 3213 个离散字节，反编译却超过100KB并串入邻函数；freehand 两入口也达120000字符截断。遇到非法大 switch、跨邻函数、坏指令或正文截断，先拒绝将其当作完整业务，再按以下已验证步骤修复：

1. 锁定原发行 RNE/PDB 的 SHA、PDB 物理范围与入口原字节；分析 patched EXE 的范围另比 SHA，不能以库文件名代替身份。用 LLVM 有界读取原字节/跳表，不按反编译器猜出的 case 数决定合同。
2. 在**本人唯一 `--write` worker**内开事务，按 PDB 范围建立候选 `AddressSet`，剔除其他函数已拥有的 body；不得强吞邻函数。原相对跳表目标按 `table_base + sign_extend(i32)` 解码，核每个目标，再用 `JumpTable(jumpAddress, targets, True, 0).writeOverride(function)`。JPype 的 `Address.subtract` 返回 Java Long；传 `getBytes(Address, int)` 前显式 `int(...)`，不要截断64位地址。
3. 检查脚本 `success/timedOut/stdoutTruncated` 与每个函数 decompileCompleted/error；保存修后正文及原始失败。修复不等于 Rust 参数已核准，关键 branch/ABI 仍回原 ASM。
4. 按本指南第6节正常停止**自己的** backend、同 worker 重开；重读 body、四个 case 和 manual override，确认持久保存，最后正常关闭。不得重启/修改 shared 或其他 worker 来修一个函数。

可复用的实跑 job 为 [m01-move-shapes-repair-job.json](../../tiny-glade/reports/module-recovery-2026-10-23/m01-move-shapes-repair-job.json)，worker `m01-move-20261026`，跳转 `14097ac59`、表 `142ab7514`、四目标 `14097c31e/14097ac5b/14097ae9d/14097af78`。用第2节 operator 将该 job 与本人唯一 worker/result 路径配套，**不可原样借用其他 owner 的 worker 名**。持久复验在 [m01-move-worker-reopen.json](../../tiny-glade/reports/module-recovery-2026-10-23/m01-move-worker-reopen.json)，操作解释见 [移动复原报告](../../tiny-glade/reports/module-recovery-2026-10-23/m01-move-shapes-recovery.md)；freehand 双入口实例见 [自由墙报告](../../tiny-glade/reports/module-recovery-2026-10-23/m01-freehand-recovery-2026-10-26.md)。

**PDB span、Ghidra body、EH 不是同一个数。** 本例 PDB 物理跨度12822B，重建后Ghidra归属字节10677B；物理跨度含异常处理和已独立归属片段，不能要求填满跨度或据此计算恢复率。反方向也可能是坏CFG少读：`validate_blueprints@140827130` 原539B而Ghidra标签488B，`1408272b5` 间接跳表被误作调用；实际回调 `1412b84f0` PDB8161B而Ghidra标签2093B。以有界原ASM和完整表恢复被遗漏分支，保留Ghidra失真证据。另须区分正常 caller 与 EH 析构引用；EH `drop_in_place` xref 不能证明存在每帧清理系统。见 [独立业务复核](../../tiny-glade/reports/module-recovery-2026-10-23/m01-independent-final-business-review.md) 与 [signifier 生命周期审查](../../tiny-glade/reports/module-recovery-2026-10-23/m01-signifier-stream-lifecycle-2026-10-27.md)。

### 7.2 Rust ZST tuple 与真实调度拓扑（2026-10-27）

C 反编译会省略 Rust 零大小函数 item/tuple 实参；`chain_ignore_deferred()` 显示无参，**不能推出仅有一个系统**。先查完整 PDB 泛型的第一个类型参数，再查对应构造器中真实 `FunctionSystem.name` 字符串、配置数组长度与每0x70B记录的搬运；区分普通 tuple、chain 和 IgnoreDeferred，不能只凭物理 callee 的 ICF 名称、调用次数或临时栈槽推拓扑。

本轮 `add_ivy_systems@140944070` 的 `141450fa0` 原泛型是8节点tuple，尾部又是普通二元tuple，实际9个业务系统；`DL=0x16` 是 spawner 的 `after_ignore_deferred(GameSystemSet22)`，不是系统数或 in_set。全部 VA、静态name、PDB完整泛型与原字节见 [ivy 原注册审查](../../tiny-glade/reports/module-recovery-2026-10-23/m01-ivy-pruner-registration-2026-10-27.md)。同样，decorator parent19中先构造的visual配置最终搬入slot14，不能按构造器调用时间误放到slot1，见 [独立复核的父表](../../tiny-glade/reports/module-recovery-2026-10-23/m01-independent-final-business-review.md)。

### 7.3 SSE unordered 与独立原指令执行（2026-10-27）

`MAXSS/MINSS` 必须回ASM核**目的/源操作数顺序**：NaN/unordered和相等时取第二操作数，C中的 fmax/maxNum、Rust max/clamp 或普通比较不必然相同；还要核后续是否有显式NaN修复。保留 ±0、输入quiet/signaling NaN的符号/payload，以及运算新生invalid NaN，不能统一成 `is_nan`/ULP 宽容比较。实例：提示动画 [原指令复核](../../tiny-glade/reports/module-recovery-2026-10-23/m01-public-hints-repair-review.md)，门底高度 [anchor 复原](../../tiny-glade/reports/module-recovery-2026-10-23/m01-anchor-follow-recovery.md)。

**模拟器输出也须被独立执行交叉证伪。** 本轮 `expand_along_edge@140c9c620` 的双NaN样本中，Unicorn偏向更大payload；Rosetta x86_64直接调用同一原RNE函数，2592条非panic输入中96条否定该模拟器结果。保留两种输出和失败记录，不能迎合Unicorn把业务改成挑最大NaN，也不能用Rust候选生成oracle。ARM新生invalid为正NaN、原x86为 `0xffc00000` 的差异也要独立确认；局部恢复只处理原运算的新生invalid，保留已有NaN传播顺序。

2026-10-29 增量：圆射线 `140c97980` 的49组NaN传播中，Unicorn与Rosetta有17组payload差异；已撤销为迎合单一模拟器而引入的first-root NaN覆盖。Rosetta也是翻译执行，不能称Windows/x86实机；有限值逐位一致、NaN分类、panic路径和payload冲突分别记录，不把局部冲突扩大为所有既有有限值证据无效。复杂finder的27组高度门控原入口执行仅证明该门控，不能升级为完整finder原侧差分。

原执行先按PE section/RVA展开内存映像，**不能把raw RNE文件直接按RVA mmap**；校验入口原字节后用真实Win64 ABI调用。expand无需CRT，Rosetta探针没有数学/几何helper桥；其他函数桥接Darwin sinf/cosf等时须标明边界，不能冒称Windows UCRT全输入逐位等价。探针、两种原执行差异、最终严格夹具与命令均见 [共享几何复核](../../tiny-glade/reports/module-recovery-2026-10-23/m01-shared-geometry-independent-review.md)；`probe_m01_expand_x86.c` 是执行工具，不是重写算法。不要盲目重跑会重写共享fixture的探针；先按第6.1节核owner和已有SHA，再用独立结果名取证。

以上仅是本轮工具与取证流程的实证增量；不代表相关模块、完整游戏或GPU验收通过。沿用既有 native skills，不复制另一份技能或平行指南。

### 7.4 cleanup 反编译失败与同布局 owner 反证（2026-10-27）

`Unable to find unique hash for varnode` 不一定是服务损坏。对本轮31KB原碰撞更新体，先在本人唯一write worker核原字节/PDB范围，剔除其他函数拥有的EH体；不要吞邻函数。清推断局部变量和补候选间接ABI仍失败时，使用本地 `DecompInterface` 的 native `setOptions`，仅关闭 `decompile/cleanup` action subgroup；本轮完整返回324941B C。该设置不改变原指令，C仍需回ASM核业务，不能把关闭分析action当作算法验证。完整 job、原件身份及正常停止→同worker重开→原C SHA相同→正常关闭的结果见[主碰撞更新恢复](../../tiny-glade/reports/module-recovery-2026-10-23/m01-rapier-producer-recovery-2026-10-27.md)。不要向共享实例或其他owner推广未验证选项。

同样大小的两个资源不能靠 C 临时名推 nominal owner。本轮 Arch/Lintel 都有104B段；必须由原 typed FunctionSystem 参数顺序，追 entry RSP/RBP、参数槽及条件分支到真实表，再用 writer 方向反证。验收两张表都放不同非空数据，禁止以另一张表为空掩盖分派倒置。缺失错误也要区分 consumer：墙 required unwrap 与屋顶可缺失移除不是同一合同。

跨体系结构 external libm 差异与原算法差异分开记录。先保留完整 x86 输出、每次原外部调用实参与返回，再将真实目标架构 libm 返回重放到原调用点，严格检验剩余原指令/数据流；不得修改生产算法迎合模拟器或用ULP宽容冒充原CRT逐位一致。地板96组与3272次 trig调用的34处平台差别、原圆数量256位型与全部失败记录见同报告。

原控制器的验证不能只对末尾数学helper。全局射线本轮证实：循环内仅保存winner、循环后才转换；逐命中提前转换会改变已被覆盖坏数据的失败顺序。恢复UV变体后必须审调用者的hit判断（0/1命中、2miss），否则近UV命中会被远terrain覆盖。Option slice要区分None与Some(empty)：用is_empty代替指针存在位会吞掉原bounds panic；不要把旧假fixture的通过当作正确性。完整原控制流/无算术桥/真实World与259组数值回归见[全局射线恢复](../../tiny-glade/reports/module-recovery-2026-10-23/m01-global-ray-recovery-2026-10-27.md)。

### 7.5 重建后迁回 owner、复合历史顺序与真实析构（2026-10-29）

M06 独立 review 否定了仅看 Default/前段 memcpy 得出的 Reload 发布器替换判断：原`1408d8710`先保存旧世界、写入新物理世界，再逐槽析构新Publisher并迁回旧7qword，同时保留旧DirtyTiles/dirty，最后同样迁回ColliderDiff Publisher。`ResMut`入口`+0x10←+0x1c`是Bevy变更tick，不能写进业务注册计数器。完整父函数后半段必须检查；不要为错误恢复添加OnEnter重订阅来掩盖。

复合历史的组序是业务语义，不能按Rust字段声明或反编译临时名排序。原擦除finish末尾将五个连续24B Vec以5b8→5a0→588→570→558装组；错误顺序在局部全空桶测试中空真通过，却在真实拆墙undo中产生已删除子墙的高度命令。至少用每桶非空样本，并等待真实ReplayBuffer和PendingHistoryCommands清空、退出Replay模式且末帧事件被消费后再比较或发下一操作。

析构要按真实类型追：原抹灰generator的HashMap<WallId,Vec<Entity>>::insert只释放旧Vec存储，不能推断会销毁其Entity组件；重复Create确实可保留未登记旧实体。原最终OnExit另有Without<KeepOnLevelUnload>全局despawn与GpuWorld清空。因此区分创建时原行为、映射所有权、组件Drop和最终退出；实际诊断按wall/entity/registry/key/保留标记记录，不由GPU总数直接判泄漏，也不凭合理性改原行为。

显式记录Cargo features：`country-core`默认未启用native-physics的过滤组不覆盖真实Rapier；下游库启用native feature也不等于编译了core内部cfg测试。保留缺导入等首次失败并以显式feature复验。以上原证、源码和不同轮次验证边界见现存M06业务台账与中央review JSON，不构成全模块验收。

本轮补充：同类型实例及同WallId不代表同allocation，先沿原结构基址确认clear/append/drop字段，防止通用柱、高程支撑和半木互相覆盖。静态图无callee不代表无producer，过梁包含内联的三Vec柱缓存写入。真实App揭露的Raw工厂假schema须在原生产者修复，不给消费者写默认兼容。测试中的run_system_once会每次重建Local，不能用于验证跨帧保存稳定门；用同一Schedule保留真实状态，并确认snapshot引用及磁盘文件，而非只检查flush完成事件。原跳表函数若反编译大小远小于PDB范围，先在唯一worker核原表并修控制流，再恢复业务；成功反编译和工具修复本身不计游戏恢复。

本轮M6复核补充：词法清单的空体／todo必须检查原发行体，原ret和原明确panic不能作为待创造的业务。已有Events资源不等于App已注册事件轮换；兼容World先init会让后续add_event跳过维护，应把已证事件放到最早App注册并跨帧验证。完整typed history首次差异若是业务项缺失，不能降格为仅末项比较或数值容差；保存解析、迁移、快照和后续录制的所有owner都需沿实际消费者检查。图的服务存活与Publication完整是两种检查，机械裁剪后使用新发布快照，旧快照无效须保留错误而非静默换无图方案。

真实加载后历史被改写时，先逐阶段比较来源文件、复制目的文件、ApplyCounter、Loaded和后续运行帧，定位到首个实际生产者；本轮证实GPU异步等待的旧Local任务可跨同会话Reload存活。宿主延迟工作的生命周期应绑定其真实会话和Game状态代次，不能用固定帧数等待、删空历史或忽略初始编辑掩盖。保留真实存档、期望和子进程参数后只重跑失败恢复段，父完整链只在输入或相关行为改变时再跑。

预览交易的验证必须遵守原版相位：生产者接受并生成的预览可能在帧末恢复原桶，权威桶在整帧末为空不自动等于放置失败。先独立核原模式守卫及调度，分别检查生成阶段的真实预览和释放后的完整Info/ID/d2s/历史与后续稳定；不能只删除按下断言，也不能为保留预览修改原回滚逻辑。注册审查要区分兼容安装器与实际App逐项安装路径，单个安装器没挂载不能外推整个App没挂载。

## 8. 复用技能与一键小宠物试跑

任务索引接入与 agent 角色模板使用 [tiny-glade-recovery](../../skeleton/workflow-design/tiny-glade-recovery/SKILL.md)。其 workflow.py 只准备任务包/检查冻结内容，不代替本说明的 operator、schema、身份检查或工具结果检查。

本轮新增身份绑定原证据包、持久MCP批量、模块交接三个本地技能，原重建技能增加公共Cargo锁对照。可执行入口与真实成本/失败/源码边界见 [RESULTS.md](../../evidence/toolchain-pet-pilot-2026-10-11/RESULTS.md)，项目recipe由 `scripts/native-workflow/run_pet_pilot.py` 一次执行。所有步骤仍检查各自结果，脚本成功不等于完整游戏业务完成。默认Bpatched数据库与C原发行文件继续分离；不将可变DB反编译结果放入原字节缓存。

## 9. 本机 Windows Claude 辅助调用（2026-10-15）

已实测本机 Claude Code 2.1.299 可用，使用既有配置，调用时不传模型名称。先把最小证据片段写入 UTF-8 prompt 文件，再运行：

```powershell
py -3 C:/Users/liyu/.codex/skills/claude-reverse-assist/scripts/claude_print.py <prompt.txt> <output-prefix> --timeout 300
```

该技能入口直接调用 npm 安装的原生程序，通过 stdin 传递提示词，保留空 `--tools` 参数，并禁用本次分析的工具/MCP。输出为 `.stdout.json`、`.stderr.txt`、`.result.json`；检查进程退出码和报告 `ok`，不得只看 stderr。当前配置会输出 `[claude-code:unrecognized_model]` 日志，但真实请求退出 0、`subtype=success`、`is_error=false` 且返回预期文本，因此此日志本身不能判定调用失败。超时单独记录，不自动重试、不改模型配置。

复验证据：[probe.result.json](../../evidence/claude-call-repair-2026-10-15/probe.result.json)，5.316 秒返回 `CLAUDE_OK_20261015`。Claude 仅作辅助分析，业务结论仍按原指令/PDB/Ghidra/ReVa 证据验收。

## 10. 内网 LLM 分析 worker（2026-10-19）

用户要求把重复、独立且边界清楚的分析优先交给内网 LLM。入口与任务合同见 [llm-worker/README.md](../../scripts/llm-worker/README.md)，后续提示词为 `scripts/llm-worker/review-production.txt`；用 JSON manifest 批量调用，密钥仅从进程环境读取。实测模板参数低强度思考优于快速模式，默认 `template-low`；标准厂商关闭开关未生效。长函数基准、失败与原指令纠错保留在 `evidence/llm-worker-2026-10-19/`。

主 agent 提供冻结的原证据包；worker 返回分支/状态/副作用/未知项与行号。优先委派机械对照、独立函数/模块合同、已有 Rust 对原证据核对、字段读写整理。ABI/别名/跨模块权威状态与生命周期、数据库和源码写入、原指令核验与验收由主 agent 裁决。传输/格式通过不是语义通过；不得绕过依赖排除与完整调用链要求，T13 范围继续排除。具体失败、参数效果和性能以该轮真实报告为准。
