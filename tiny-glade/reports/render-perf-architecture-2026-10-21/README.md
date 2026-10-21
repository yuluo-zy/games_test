# 渲染性能重构文档集(2026-10-21)

阅读顺序:

1. [DESIGN.md](DESIGN.md) — 目标、边界、现状与目标架构、设计决策 D1–D10、测量协议、验收门、阶段计划。
2. [IMPLEMENTATION.md](IMPLEMENTATION.md) — 实施顺序、任务表、命令、完成定义。
3. [IMPL-A-host.md](IMPL-A-host.md) — A 层宿主止血(A1–A7),逐项现状/改法/验证,文末 20 条实施前 grep 清单。
4. [IMPL-B-rhapsody.md](IMPL-B-rhapsody.md) — B 层 rhapsody 内部(B0–B7),数据结构与算法,文末 25 条【未核实】。
5. [IMPL-C-architecture.md](IMPL-C-architecture.md) — C 层保留图 / GPU 驱动绘制 / pass 合并,迁移里程碑与像素等价验证。
6. [ANALYSIS.md](ANALYSIS.md) — 根因分析与原版对照(证据来源)。

所有内容为只读代码分析,基线提交 `f98b40229`,未构建、未运行;真机数据为空,第 0 步测量先行。`measurements/` 目录存放每次真机记录。
