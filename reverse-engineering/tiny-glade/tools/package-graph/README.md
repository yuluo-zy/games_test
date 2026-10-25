# B 原作包引用图

交互页面：[package-dependencies.html](../../package-dependencies.html)。页面自包含，无CDN或网络API，直接在浏览器打开即可使用。

默认紧凑展示29个主要游戏包与9个选定外部版本，包含近期开始恢复的motomoto与tg-package-integrity。全部local视图保留52个无source条目，包括未进入主包构建闭包的包；它们不自动计为游戏私有源码。搜索和聚焦使用原锁的全部901个包及2868条已解析直接边。同名多版本/source使用独立ID。

- 箭头方向：使用者→依赖包。职责分层是阅读布局，不表示严格依赖级别；依赖深度按显示子图的拓扑关系计算。
- 主干模式只删除显示子图中可经其他直接边到达的重复边，不新增虚构边。点击包后补画其显示范围内的全部直接边；聚焦显示全部直接上下游。
- Cargo.lock不能区分build/dev/target类别或证明运行调用时序；两条无法唯一解析的zeno边保留在页脚，不猜测连线。
- 模块状态只来自源码进度表，包级状态是“已登记子模块中存在部分接线/候选”的摘要，不是完整包验收。`system-tree`包对应原PDB目录`systems/tree-tool`。

节点以五段显示分析→实现→接线→原作对照→完整验收。橙色背景表示已开始恢复，橙色斜纹表示该阶段部分完成，灰色表示未登记推进；绿色实心填满已完成阶段，整包完整验收后整个节点变绿。外部依赖不计逆向进度。顶部统计随范围、聚焦和搜索选择刷新，并标出台账时间。

进度按已登记B模块汇总，不按文件、函数、测试案例数量推算百分比。实现阶段要求本机恢复文件存在，`reported_missing`仅计已有研究记录。接线来自`integrated_partial`；原作对照来自显式原入口计数、限定原码绑定或verification报告，编译通过不自动计原作对照。仅完整模块验收或全部相关模块显式阶段门禁才将整段标为完成。

阶段完整验收可单独登记在`tracking/package-graph-stage-acceptance.json`的`modules`中，以原模块路径为键，阶段键为`analysis`、`implementation`、`integration`、`verification`。记录形如`{"status":"complete","baseline":"B","scope":"明确验收范围","evidence":["evidence/.../report.json"]}`。每个阶段需要全部相关B模块的明确完成记录，证据文件存在才生效，缺一仍不升格。完整验收阶段和整包仍以模块台账的`full_module_accepted`和`full_module_acceptance`为准。当前没有新增完成声明；三个小库以`in_progress`登记已证实的局部原码对照，以补足台账没有结构化对照计数的情况。

`shader-pipeline`对应原嵌套目录`crates/rhapsody/crates/shader-pipeline/src`；`shader-compiler`使用已登记B工具链合同的独立模块键，不补造其未知原PDB路径。

刷新：在调查根目录运行 `py -3 scripts/build-package-graph.py`。数据源为`tracking/original-Cargo.lock`、`tracking/original-packages.json`、`tracking/dependency-exclusions.json`和`tracking/module-progress.json`；输出身份在`tracking/package-graph.json`。

视觉/交互验证脚本：`node tools/package-graph/verify.cjs <Playwright模块目录>`，使用已安装的Edge。验证结果在`tracking/package-graph-validation.json`；不运行游戏或原码业务测试。
