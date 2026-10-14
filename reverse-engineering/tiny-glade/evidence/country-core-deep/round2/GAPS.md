# 第二轮缺口清单（2026-10-14 审查后）

第一轮 8 域 DIGEST 收齐。本文件登记第二轮目标：**逐文件实现逻辑审查** +
**U 项闭合**。机器数据：`round2/file-audit.json`（459 文件标记矩阵）、
`round2/gaps-v1.json`（每域文件统计 + U 项）。

## 全局审计数（459 文件）

- 59,001 行 / 1,144 Gap 标记 / 261 typed_gap / 265 空函数体 / stub 4 文件
- v2 权威索引覆盖 1,501 个过程（部分文件 v2=0 但 v1 有过程 = 全内联/泛型，
  需调用点驱动恢复，如 resources/session.rs）

## 第二轮任务（每域一个 closure agent，复用第一轮 agent 上下文）

1. **逐文件实现逻辑审查**：对域内每个 reconstruction .rs 文件，对照本域
   DIGEST 的 O 条目审查现有实现，产出每文件表格：
   `{file, 状态(ok/需修/未知), 具体错误（"代码写 X，证据为 Y"）, 缺失项}`。
   交付 `evidence/country-core-deep/round2/<domain>-REVIEW.md`。
   不改 reconstruction 源码。
2. **U 项闭合**：按 gaps-v1.json 中本域的 digest_gaps + DIGEST §U，用 v2 索引
   地址 + 定点反编译/xrefs/rne-read 尽量闭合；不能闭合的注明卡点与已排除的
   候选。交付 `evidence/country-core-deep/round2/<domain>-CLOSURE.md`（或并入
   REVIEW）。
3. 汇报：SendMessage 给 main，10 行以内：审查统计（ok/需修/未知 文件数）、
   闭合 U 项数、仍卡住的 U 项。

## 跨域遗留（各 agent 审查时若碰到请一并核）

- photo_mode 0..3 ↔ CameraMode 槽 6..12 的槽位分配（camera-time + ui-input）
- RaycastMeshData 写入面 vs 读取面 ±8 偏移差（collision-eco）
- SubworldIndex 17 槽完整名序（collision-eco；Terrain=5 已锚定）
- resources/session.rs 无 S_PUB32 符号 → Session 方法全内联？（session-save）
- 0x14362c420/0x14362c3a8/0x14362c011 全局族（已三域合证，实现时统一）
- eraser archivist 倒序串、u_space 全局常量重定位疑点（wall）

## 纪律

- 一切以 v2 索引 + 机器码为准；PDB 骨架名只作线索。
- 字符串/字节用 rne-read（--unks）；MCP 反编译批量跑（run-decomp-batch.py 已切 v2）。
- 不写 reconstruction 源码、不重写 DIGEST 已证实内容。
