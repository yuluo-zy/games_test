# 上层模块派发次序 v3（2026-10-17 优化版）

> 依据：SOP 2026-10-17-recovery-sop.md + reverse_speedup_levers.md + 本会话 core 类型落地核实。
> 并发按账户额度（2026-10-17 用户取消 3 并发错峰，满并行）；波内按依赖排序，解锁线先行。

## 依赖核实（2026-10-17）
- ✅ InsertTreeCmd（tree_edit.rs）→ tree-tool Insert/replay 可落
- ✅ RequestAppModeChange/RequestToolbarCategoryChange/WindowFocusEvent → toolbar 可落
- ✅ RoofTileSsbo SparseSsbo（roof_tile_ssbo.rs:78）→ roof gpu_flush / decorator dormer GPU 可落
- ❌ NaniBundle<T> / DrawableMeshBundle → 仍缺（解锁 decorator visual build / roof switch_roof_type / toolbar spawn）

## 派发次序
- **W1（满并行）**
  1. unlock-nani：落 NaniBundle<T>/DrawableMeshBundle（合同在 reports/gap/rhapsody-resources）
  2. consume-types：tree-tool Insert/replay/ui_move + roof gpu_flush + decorator maintain_dormer GPU（用已落类型）
  3. verify-gap：独立核验 gap 轮 core-events/utils-toolbar 落库（事件载荷面/rotation_from_normal/toolbar 容器）
- **W2（NaniBundle 落地后）**：decorator visual build / roof switch_roof_type / toolbar spawn；各模块剩余域内未实现
- **W3**：核验 + 数值断言测试 + 全量 cargo check（rhapsody 稳定后）

## 每路五件套（固定）
AGENTS + AI_REVERSE_TOOLS + 判等协议(分歧检查表 8 条) + 域速查 + 前轮 LOGIC/REBUILD；
红线：只读 shared、唯一 --write worker、不改他模块、缺类型登记不编造、宏胶水不重建。
收口三态：完成 / 登记(缺类型·跨域·hardpoint，写明解锁条件) / 阻塞。
