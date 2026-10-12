# 小宠物逆向工作流试跑

范围：Tiny Glade C v1.16.0 羊的抚摸状态/动画业务切片。A/B 作证据参考；默认持久 Ghidra 是 B patched EXE，原始指令读取使用每版原发行文件。整个小宠物模块、完整游戏和完整建筑系统的验收不在本次工具效率试跑中自动成立。

执行顺序：保留证据与三版归属 → 已知地址/消费者 → 一次有界 MCP 函数批量 → 原发行文件 LLVM 内联/源码点 + 指令范围 → 原跳转表补漏 → Rust 小切片及运行边界 → 证据交接审计。库函数复用原 Cargo API；源码路径与校验和只表明来源，不能等同完整原作者源码。

可复用技能：`native-evidence-pack` 负责原始元数据/指令包及不可变证据缓存；`native-tool-batch` 负责持久 MCP 查询及错误摘要；`native-module-handoff` 负责完整链的证据引用、版本边界和缺口交接。原 `native-rust-reconstruction` 保留业务恢复与验收约束，并按需调用三个技能。

测量定义：operator processes 为 CLI operator 启动次数；MCP steps 为 operator 内实际发出的分析调用（不包含 initialize/list_tools）；LLVM process counts 单独列出。agent/orchestrator 交互、缓存命中、结果字节数与耗时分别记录，不将一条脚本调用伪装成一个底层查询。只比较同一原始请求包的 cold/warm 时间，未建立旧手工全流程基准则不宣称总体提速倍数或精确 token 节约。

所有完整结果在本目录。工具成功与语义恢复成功分别报告，失败 fixture 不计游戏业务证据；仅脚本修复导致的重跑单列。旧 firefly 的 NaN 比较失败继续保留，不因羊试跑通过而改写。
