# Tiny Glade 逆向工具链

全量汇编与 Neo4j 代码地图入口：[`code-map`](../reverse-engineering/tiny-glade/tools/code-map/README.md)。
源码与查询工具在逆向域维护；本目录 `code-map-runtime/` 只保存本机运行时、图数据库、分层汇编和检查点，不提交。

2026-10-24：修复 Ghidra 共享进程查询故障、默认结果路径和语义健康检查，补全 ReVa 工具分页与有实际除法数据流的 P-code 样例。现场结果见 [修复复验记录](../reverse-engineering/tiny-glade/evidence/toolchain-revalidation/ai-guide/tool-repair-2026-10-24.json)；下面 2026-10-11 的数量是历史范围，不能替代本轮复验或游戏业务验收。

2026-10-11重新实测后，默认MCP为Ghidra和ReVa；REA、radare2、RE-MCP保留为按需SDK工具。
完整报告：`reverse-engineering/tiny-glade/evidence/toolchain-revalidation/summary.json`。

| 工具 | 当前作用和实测 |
|---|---|
| Ghidra MCP 7.0.0 | 3客户端45查询通过；共享只读14工具，专属write worker19工具；2worker同函数不同签名隔离 |
| ReVa 已构建版本 | 既有B clone，4客户端反编译、3并发PyGhidra脚本输出隔离、P-code和签名持久修复通过；共享6、专属10工具 |
| RE-MCP Ghidra 3.0.3 | 两cached B数据库、两proxy客户端并行及持久隔离通过；本地GhidraProject.save兼容补丁有diff/原文件，stock包保存bug不可忽略 |
| REA 6.3.0 | 原B身份+真实20字节机器码封装fixture反编译/P-code/双会话隔离通过；临时数据库，冷启动约93秒，不复用既有Ghidra库 |
| radare2 6.2.4 / r2mcp 1.8.8 | B原字节/局部函数、引用和独立会话有效；已补r2ghidra6.2.4/SLEIGH6.1.4 pdg；PDB TPI尚不兼容，不能算完整类型导入 |
| LLVM + PyGhidra + 原指令验证 | PDB/内联/行号、持久原库、机器码差分主力；保持已有依赖与基线 |

## Agent直接使用

先读 [AI_REVERSE_TOOLS.md](/Users/yuluo/Desktop/game/reverse-engineering/tiny-glade/docs/current/AI_REVERSE_TOOLS.md)。其中有完整绝对路径命令、六份可执行 JSON job、各工具参数快照、并行 worker 与失败处理。

机器可读入口：[manifest.json](/Users/yuluo/Desktop/game/reverse-engineering/tiny-glade/tool-usage/manifest.json)。按需读取对应 backend 的 schema，不要加载整个上游工具目录。

```sh
/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python /Users/yuluo/.codex/mcp-tools/reverse-operator.py ghidra /Users/yuluo/Desktop/game/reverse-engineering/tiny-glade/tool-usage/jobs/ghidra-read.json --worker shared
```

## 目标和持久性

原Ghidra seed：`tools/ghidra-projects/TinyGlade.gpr`，B patched EXE SHA256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323。
原指令oracle仍是发行B RNE，SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。
共享Ghidra项目为`tools/ghidra-projects/mcp/TinyGlade.gpr`；ReVa共享项目为`tools/ghidra-projects/reva-test/TinyGlade.gpr`。
专属Ghidra/ReVa worker在`workers/`、`reva-workers/`下；输入与overlay分开，原数据库不自动同步。
这些默认持久worker只绑定B；A/C不会自动套用B布局，未有缓存时不默认全程序分析。

RE-MCP创建项目在输入旁，operator先stage到`tools/re-mcp-inputs/<worker>/<digest>/`；只在patched B hash完全匹配时预置B cache。
独立数据库worker能并行，单个worker的MainThreadExecutor内部串行；需要同函数写入隔离时使用不同worker副本和database ID。
已验证手动/断开自动保存；保留SDK晚到通知引起一次关闭竞争的证据。operator末尾list_databases作为消息完成屏障，不宣称SDK竞争彻底消除。

## 默认注册与按需能力

`~/.codex/config.toml`仅默认注册Ghidra/ReVa精选面；其余通过operator按需连接，完整schema只存结果文件。
Ghidra launcher：`~/.codex/mcp-tools/ghidra/launch.py`；ReVa proxy/manager：`~/.codex/mcp-tools/reva/`。
通过各manager的`--stop`只停止所选worker；共享后端供多个读agent复用。
ReVa全局`get-functions`在131k函数上实测107.7秒，不放默认面；反编译默认不展开incoming-reference context。

逆向skill保留`native-rust-reconstruction`与单份`pyghidra-scripting`。不恢复CTF/重复副本。
其他资产/Blender工具与通用运行时保留；此轮不修改游戏业务Rust，也不计为游戏系统验收。
