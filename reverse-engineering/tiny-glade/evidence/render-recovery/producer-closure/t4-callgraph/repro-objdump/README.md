# rhapsody 工作图

由 `scripts/render-recovery/build_rhapsody_workgraph.py` 从 PDB 范围和原二进制确定性生成。
它是工作分包与缺口导航，不是源码恢复完成证明。

- PDB 记录：432
- Public symbol 记录：328
- 仅 public symbol 的已定界入口：26
- 唯一地址：450
- 语义族：431
- 直接调用位置：4408
- 间接调用位置：138
- 共享地址语义冲突：44

下一步只对 D 包的 `ResourceInfo::calculate → registry/pool → compiled/state` 缺口做定向取证。
