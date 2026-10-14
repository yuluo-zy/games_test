# tabloid

游戏增量状态发布/订阅通道，订阅者批量消费从上次 take 之后积累的变化。

## 功能与目录

src/lib.rs；tests/dirty_tiles.rs；examples/review_publication_cost.rs。

## 来源

- 恢复源码：[shared-foundation/tabloid](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tabloid/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tabloid/`。
- 原 B 包/PDB 归属：`crates/tabloid/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`c05fad96`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[SUMMARY.md](../../../reverse-engineering/tiny-glade/evidence/foundation-plt-2026-10-13/SUMMARY.md)；[TABLOID_WIRING_GAP.md](../../../reverse-engineering/tiny-glade/evidence/foundation-plt-2026-10-13/TABLOID_WIRING_GAP.md)；[FOUNDATION_LIBRARY_REPAIRS.md](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/FOUNDATION_LIBRARY_REPAIRS.md)。

## 已完成

已恢复 publish/subscribe/take 及克隆订阅者共享消费边界、晚订阅只收未来变化、pending 合并规则。DirtyTiles 的空状态/OR 合并有原码证据。当前版本使用不可变 Arc 快照、弱订阅登记、每独立 pending batch 一次 clone/apply，并处理序列耗尽。

## 尚差的功能与验收

真实 country-core 的 DirtyTiles/ColliderDiff producer→consumer 联接以及其 State/StateDelta 实现是库外任务。State 的独立 Clone、不可重入/不 panic 合同仍需调用方遵守；高增长状态的拷贝成本需实际负载测试。

## 持续开发差异

保留 c05fad96 产品 API：publish 返回 Result，PublicationReader 不持 read lock，订阅者可即时回收。历史原版报告不验证这些所有权与耗尽策略改变。

在 workspace 根执行 `cargo test -p tabloid --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。
