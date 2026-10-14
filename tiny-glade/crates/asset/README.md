# asset

游戏 prefab 与 texture 配置、继承合并、扫描和归档数据合同，供资产导入与运行时选择使用。

## 功能与目录

src/inheritable.rs；src/prefab/{config,scan}.rs；src/texture/{config,desc,blob,scan}.rs。

## 来源

- 恢复源码：[shared-foundation/asset](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/asset/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/asset/`。
- 原 B 包/PDB 归属：`crates/asset/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`326cb236`、`3e0387cd`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 导入时源工作树还有修改：`src/prefab/config.rs`；是否选用以审计记录的 selected_revision/source_sha256 为准。未选的改动单列 unselected_worktree_sha256，旧提交验证不自动覆盖新的源内容。
- 逆向/验收报告：[FOUNDATION_BUSINESS_COMPARISON.md](../../../reverse-engineering/tiny-glade/FOUNDATION_BUSINESS_COMPARISON.md)；[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-review-fixes-2026-10-14/REPORT.md)。

## 已完成

prefab 命名/继承/default 配置、texture 配置与描述、目录扫描、blob archive schema 均有手写实现；RT/audio 可选对象的深层默认和 required markers 后续恢复也已迁入，含原 native-default fixture。发行 RON/texture/prefab 回归测试已迁入。原 rkyv/RON fork、bytecheck、相对路径等直接复用。

## 尚差的功能与验收

原类型/消费者深层合同及 native 布局仍需复核；archive wire 与 Rust native ABI 是不同验收层。完整 country-core 资产 producer→pass、A/C 和真实游戏加载流程仍未接线。

## 持续开发差异

已保留源工作树 prefab/config.rs 当前修复，准确修改状态和哈希见 foundation-audit；不以旧 Git 快照的验证结论覆盖它。

在 workspace 根执行 `cargo test -p asset --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。
