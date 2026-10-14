# Windows 主力逆向工具修复结果

2026-10-13，用户要求先修复工具。本轮验收为工具可用性，不计游戏模块恢复。

## 修复

1. 旧批处理入口仅指向Mac用户目录。新增Windows官方Python MCP SDK入口，修复 `scripts/multi-version/mcp-native.mjs` 的Windows分支。
2. 默认ReVa项目为会话临时目录，中断/会话重建后程序消失。改用各worker独立的持久项目和后端；重连不重复导入、不要求重启Codex。
3. Windows虚拟环境启动器与实际Python后端PID可能不同。用本次启动token确认后端，再以真实PID+进程创建时间绑定；只停止本worker。
4. Ghidra脚本外层事务使脚本内保存失败。保存改到后端事务外checkpoint，修改后退出/重开验证存在。
5. ReVa脚本stdout有64KiB上限，注入的print也不能直接用redirect_stdout截获。改为完整转存Python print、固定LF、核对磁盘字节数和SHA。原始MCP响应与全文均保留。
6. 检查MCP失败、脚本失败、超时、截断、artifact缺失、selector和只读profile；不能只凭连接成功判定。目录/schema按需读取，不灌全部工具定义。
7. 从核验通过且正常关闭的独立B库冻结seed，复制时核对所有文件，减少agent重复导入。

## 实测

[基础验证](summary.json)11项、[worker验证](workers-summary.json)4项，以及[Node兼容入口](node-route.json)通过。覆盖2个限定函数的真实反汇编/反编译和原RNE字节SHA、20万字符完整输出、错误/截断/selector/profile失败、保存重开、独立worker隔离、seed启动、同worker串行和不同worker并发。

本轮首个新导入样本约63秒；复用seed项目启动约10–11秒；保留后端的一次metadata job约1.6秒，包含Python启动、SDK会话、identity和checkpoint。耗时是本机样本，不保证固定时延或并行倍数提速。

[LLVM PDB实读](llvm-pdb-summary.txt)确认LLVM20.1能打开B PDB：GUID `{09F1A02A-49F2-F8C8-4C4C-44205044422E}`、Age1、4892 streams，存在debug/types/IDs/globals/publics。缺失Rust局部字段和void占位仍是证据边界，未据此猜ABI。

统一用法：[Windows持久SDK说明](../../scripts/reverse_tools/README.md)。主入口为ReVa MCP/Ghidra引擎；ghidra为限定范围兼容适配器。REA/radare2/RE-MCP旧Mac报告未算作当前Windows已验收能力。

## 范围

B-only，未启动游戏，未全程序分析，未猜原型，未同步历史Mac数据库/overlays。失败重试的部分历史报告保留在history；旧artifact路径可能被同名后续验证更新，完整正文仍在历史报告内，不能当作当前artifact校验。库、state和认证key位于git忽略的tools/reverse-runtime；交付源码、用法和验证记录。
