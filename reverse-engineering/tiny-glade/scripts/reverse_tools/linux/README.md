# Linux 主机 ReVa 环境(2026-10-20)

此前 round11 记录 “ReVa no programs / RE-MCP 无 database”:本机没有任何 Ghidra/ReVa,文档中的 Windows/Mac 入口在 Linux 不存在。已在用户目录搭建(无需 sudo):

| 组件 | 版本/位置 |
|---|---|
| JDK | Temurin 21.0.5,`~/tools/jdk-21.0.5+11`(GitHub releases 下载;adoptium/oracle/corretto 直链 403) |
| Ghidra | 12.1.3,`~/tools/ghidra_12.1.3_PUBLIC`(GitHub releases) |
| ReVa | 7.3.1 扩展(解压到 Ghidra/Extensions)+ 同版本 whl |
| Python | `uv venv ~/tools/venv`(3.12),PyPI 走 USTC 镜像 `https://mirrors.ustc.edu.cn/pypi/simple`;`mcp` 必须 `<1.23`(2.x 无 `streamablehttp_client`) |
| 程序 | `~/Downloads/tiny-glade.exe`,SHA256 `cf048e27…6323` = B patched,与基线一致 |

## 使用
```sh
export JAVA_HOME=~/tools/jdk-21.0.5+11 GHIDRA_INSTALL_DIR=~/tools/ghidra_12.1.3_PUBLIC
~/tools/venv/bin/python import.py                 # 仅首次:导入 exe 到 ~/tools/reverse-runtime/main/project,不做全程序分析
nohup ~/tools/venv/bin/python serve.py > serve.log &   # 日志输出 PORT <n>
~/tools/venv/bin/python probe.py <n>              # 列工具/验证程序
```
MCP 端点 `http://localhost:<port>/mcp/message`,程序路径 `/tiny-glade.exe`。

## 实测
- `list-project-files` 返回 `/tiny-glade.exe`;86 个工具。
- `get-decompilation 0x14090e6f0` 返回 162 行反编译。

## 批量建函数(2026-10-20)
库导入后只有 4 个函数。按以下顺序补建(均不做全程序分析,先 `serve.py` 停止再运行,项目同一时刻只能被一个进程打开):
1. `build_functions.py`:解析 PE `.pdata`(116269 条 RUNTIME_FUNCTION,begin/end 为权威范围),逐条反汇编并建函数;以 PDB public symbol 的 demangled 名命名(名里的空格改 `_`,同名符号取第一个)。约 2.5 分钟,命名 40002。
2. `build_leaf.py`:骨架 B 的 5318 个 `"rva"` 中 725 个不在 `.pdata`(无栈帧的叶函数),按控制流建函数并命名 647。
结果:函数总数 116997;骨架 RVA 全部都有函数起点。`get-function-count` 只统计命名函数(≈40652),不等于函数总数。

注意:`.pdata` 范围只是主体块,PDB `code_size` 可能更大(例:`construct_door_stairs` 0x14200c7f0,pdata 3724B,PDB 4668B,差额是冷路径/funclet 块),取证函数尾部时以反编译中的跳转目标为准,不要直接信任单一范围。

## 已知限制(未解决)
- 未应用 PDB 类型/参数:签名均为 `undefined f(void)` 之类,参数需用 `set-function-prototype` 或结合骨架补;PDB 名不等于 ABI。
- 约 76k 个无 public symbol 的函数保留 `FUN_` 名。
- 历史 Mac/Windows overlay 未迁移。
- 参数名以现场 schema 为准(如 `folderPath`、`functionNameOrAddress`)。
- RE-MCP 未安装;当前只恢复 ReVa。
