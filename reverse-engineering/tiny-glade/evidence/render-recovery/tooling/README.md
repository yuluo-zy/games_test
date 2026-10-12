# rhapsody 取证编排工具验证

`scripts/render-recovery/mcp-evidence-job.mjs` 使用已安装的官方 MCP Client SDK 和现有 stdio 配置；没有安装或替换引擎，没有修改 Codex 全局配置。只允许明确列出的静态查询与数据库 open/wait/close，禁止 execute/batch/call 通用入口、运行原目标、保存、类型/注释修改。open 必须绑定 job 指定的 PE 和 database，auto-analysis=false，close 始终 save:false。

```powershell
node scripts/render-recovery/mcp-evidence-job.mjs re_mcp_ghidra JOB.json REPORT.json --config D:/tools/reverse-engineering/verification/server-config.json
```

最小 job：

```json
{
  "target": {
    "path": "D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe",
    "sha256": "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
  },
  "provider": {"id": "ghidra", "version": "12.1.4"},
  "database": "tg_render_arch",
  "inputs": [{"path": "D:/game/reverse-engineering/tiny-glade/evidence/render-recovery/workgraph/functions.json"}],
  "steps": [
    {"tool": "open_database", "args": {"file_path": "D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe", "database_id": "tg_render_arch", "run_auto_analysis": false}},
    {"tool": "wait_for_analysis", "args": {"database": "tg_render_arch"}},
    {"tool": "decompile_function", "args": {"database": "tg_render_arch", "address": "0x140bf2560"}}
  ]
}
```

每个新 runner 默认共同锁 `ghidra:tg_render_arch`。原有脚本不读取此锁，主 agent 的数据库窗口协调仍必要。锁目录原子创建，owner 记录 PID/token/时间/身份/报告。释放前检查 resolved 绝对路径是 lockRoot 内直接 `.lock` 子目录且 token 匹配。数据库关闭失败会保留 quarantine；不自动抢占 stale lock。进程被强制终止可能留下锁，需要先确认旧进程和 provider 会话结束，再由协调者处理。报告保存完整连接 catalog/schema、实际 server self-report、完整 tool result、exception、cache provenance、close 结果和锁生命周期。

`provider.version` 是 job 声明；launch 配置 hash 不能证明底层引擎或持久数据库未变。因此**默认关闭数据库分析缓存**。只有 caller 同时提供 `cache_context.provider_manifest` 与 `cache_context.database_snapshot` 两个现有文件路径，才启用成功结果缓存。前者应绑定 provider 包/版本/内容，后者应绑定当前持久数据库 revision/快照；每次数据库类型、注释或状态改变后，caller 必须更新 revision/快照文件。runner 对两者及其它 inputs 读取实际 hash；不自动证明声明的快照代表当前活跃数据库。缺少任一文件时每次调用真实 MCP，报告明确 disabled。

缓存键包含 target 实际 SHA256、provider 声明、launch hash、operation、全部 args、input/context 实际路径与 hash。open/wait/close 不缓存。完整 session 和 cleanup 成功后才提交成功查询；error/exception 不缓存，cleanup 失败不提交本次成功查询。`step.reuse={"report":"旧新格式报告路径","index":1}` 可复用身份、完整成功和 cleanup 均匹配的新格式报告结果。缺少历史身份/输入证明的 legacy 报告拒绝当作可信缓存，旧证据仍可人工引用。

auto-analysis=false 的数据库 xref 完整度为 unknown。空返回或只有 PDB DATA 引用不等于全程序无 caller；报告和每项 xref 查询保留此限制。成功缓存不会扩大原证据范围。

运行本地控制 fixture：

```powershell
node evidence/render-recovery/tooling/test-runner.mjs
```

`runner-validation.json` 记录 27 项通过及源码 hash。`fake-mcp.mjs` 是本地 JSON-RPC stdio fixture，测试连接的是此 fixture，**没有连接原作或打开 Ghidra**。覆盖成功复用、输入/target/provider/DB revision 失效、完整结果、只读白名单、error envelope/协议异常/timeout、默认关闭、关闭失败隔离、锁冲突、删除路径边界及历史成功报告复用。`fixture/` 仅存这些可重建测试的二进制文本、context、缓存与空锁目录。

第四批门禁的 `batch04-*` 文件为实际编译、原 CPU 指令重放、Rust oracle 输出、原 SPIR-V 反射及跨 crate 测试日志。工具验证本身不计为游戏私有源码恢复。
