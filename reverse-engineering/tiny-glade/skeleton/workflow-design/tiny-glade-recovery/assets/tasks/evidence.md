{{COMMON}}

## 有界取证任务

只解决任务合同中的缺口，不写游戏实现。先读已有原始证据、精确实例/内联/别名与依赖信息。每次查询注明两个待区分假设、有限原码范围及预期观察。

按已有事实→原版 LLVM 局部指令→身份匹配的反编译/xref→必要 High P-code/调用点→有证据的 overlay 逐级升级。应用 native-evidence-pack/native-tool-batch；专用 API 缺失时才用 pyghidra-scripting。C DB 身份不明就保留缺口，用 C 原始事实，不调用默认 B 解释 C。

MCP 参数用项目 guide/schema。Ghidra 最多20函数、限定 fields；ReVa 脚本即使只读也用 actor 独立 --write worker。核对 process、顶层/嵌套错误、missing、timedOut/truncated；空 RETURN/void() 要核对真实入口和 caller ABI。原始工具输出和失败报告保存到允许根。

交付 facts/gaps：每项 version、observed/inferred/unknown、evidence/pointer、对 producer/consumer 的影响、最小下一查询；记录 binary/PDB/DB 身份、overlay前后及保存读回。合同是否足够由对应义务决定；工具连接成功不算游戏恢复。
