# Windows 持久逆向入口

2026-10-13 修复。该目录使用已安装的 ReVa Java 服务、Ghidra/PyGhidra 和官方 Python MCP SDK，不依赖旧 Mac 用户目录或当前 Codex 临时 MCP 会话。

> 当前 Mac 检出路径：`/Users/yuluo/Desktop/game/reverse-engineering/tiny-glade/scripts/reverse_tools/`。以下 `D:/` 命令仅适用于 Windows 主机。

```powershell
& D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe D:/game/reverse-engineering/tiny-glade/scripts/reverse_tools/reverse_operator.py ghidra D:/game/reverse-engineering/tiny-glade/tool-usage/jobs/windows-ghidra-read.json D:/game/reverse-engineering/tiny-glade/evidence/my-query.json --worker foundation-utils
```

`reva` 使用现场原生工具名/参数；`ghidra` 是仓库兼容适配器，实际也通过 ReVa MCP 在 Ghidra 引擎执行，不表示旧 ghidra MCP 已安装。可以用 `node scripts/multi-version/mcp-native.mjs ghidra ...` 或 `scripts/reverse_tools/reverse-native.mjs` 启动同一入口。

- 每个 agent 用唯一 worker。需要脚本、函数范围准备、原型或注释修改时追加 `--write`；`shared --write` 被拒绝。同 worker 调用串行，不同 worker 各自独立项目和后端。
- 所有 worker 固定 B patched EXE SHA `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。首次从已验证冻结种子复制；无种子时仅导入，不自动全程序分析。这里的库是 Windows 新建库，未迁移或合并历史 Mac overlays。
- 项目在 `D:/game/tools/reverse-runtime/<worker>/project/TinyGlade.gpr`。后端跨终端调用保持运行；PID 加 Windows 进程创建时间绑定；不终止其他 worker。控制命令 `reva --worker <name> --check` / `--stop`。停止前保存，正常重开可复用代码范围和注释。
- 每次 job 后端在脚本事务之外执行本地 checkpoint。不要在 `run-script` 中调用 `currentProgram.save()`，Ghidra 脚本的外层事务会使它失败。业务修改仍需显式事务并正确 commit/rollback。
- inline `run-script` 的 Python `print` 完整写入 `<result>.artifacts/<step>.stdout.txt`，核对字节数/SHA 后收入报告的 `results[i].stdout`。原生工具响应仍在 `results[i].result`；原生响应或脚本 stderr 被截断时报告失败。Java `println` 等不属于 Python stdout 转存范围。
- MCP `isError`、脚本 `success=false`、超时、截断及缺失 artifact 都失败；完整结果、异常和耗时落盘。`allow_error` 保留失败步骤，整体 `ok` 仍为 false。
- Ghidra 适配器支持 `get_metadata`、PDB `find_functions`、`get_functions`、`get_xrefs_to/from`、`read_memory`、`prepare_functions`、`save_program`。每批函数至多 20 个；内存最多 4096 字节。更多类型/原型/P-code能力使用原生 `reva` 的真实 schema。
- `find_functions` 查询原 LLVM PDB public-symbol 证据，不依赖全程序分析；名字不是 ABI。`get_functions` 标出签名来源及未验证 Rust ABI，遇到导入的一字节 body 明确返回缺口，不把 `void()` 空伪码当作真实实现。
- `prepare_functions` 要求每个地址的原码范围大小、原字节 SHA 和范围依据；检查当前 DB 原字节匹配后限定反汇编，保留相邻函数，更新 body，不猜原型。无 unwind 的下一 public symbol 上界不能当完整作者函数。

按需取一个 schema：

```powershell
& D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe scripts/reverse_tools/reverse_operator.py reva --worker foundation-utils --schema run-script
& D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe scripts/reverse_tools/reverse_operator.py ghidra --worker foundation-utils --schema get_functions
```

以上第二组命令的 cwd 为本 Tiny Glade 目录。原生 catalog 全量只落在 worker 的 `catalog.json`，不要整份放入模型上下文。初始化冻结 seed 的命令为 `snapshot_seed.py <已正常停止的 worker> <该 worker 身份核验通过的 report>`，已有 seed 不覆盖。

实测：[基础验证](../../evidence/toolchain-repair-2026-10-13/summary.json)、[并发与 seed 验证](../../evidence/toolchain-repair-2026-10-13/workers-summary.json)。工具验证不计作游戏源码恢复。
