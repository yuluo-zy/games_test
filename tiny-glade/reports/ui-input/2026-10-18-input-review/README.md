# input 源码审查证据（2026-10-18）

业务与功能说明：[input/README.md](../../../crates/country-core/src/input/README.md)。完整问题、触发条件、位置与修复建议：[input/REVIEW.md](../../../crates/country-core/src/input/REVIEW.md)。本轮只做分析、文档和审查验证，没有修改业务源码或原逆向数据库。

| 证据 | 用途 |
|---|---|
| [input-tests.log](input-tests.log) | 输入相关现有单元测试：46 passed，0 failed |
| [probes.rs](probes.rs) / [run_probes.py](run_probes.py) | 四个定向缺口探针和可重复运行入口 |
| [probes.log](probes.log) | 最新源码 Cargo 重建受 render/sparse_instance_buffer.rs:425 的 E0502 阻断 |
| [cached-probes.log](cached-probes.log) / [cached-artifacts.json](cached-artifacts.json) | 缓存编译快照的四个探针全部通过，绑定 core 与直接依赖产物 SHA256 |
| [run_cached_probes.py](run_cached_probes.py) | 重放原缓存快照；产物指纹改变则拒绝运行 |
| [probes-build-blocked.log](probes-build-blocked.log) | 首次探针构建被目录外 descriptor_table.rs 的 E0502 阻断；保留失败记录 |
| [probes-harness-failed.log](probes-harness-failed.log) | 首次可执行探针 3 通过；调度探针误读构建后已转移的 graph.systems，改用 Schedule::systems 后重跑 |
| [reviewed-sources.json](reviewed-sources.json) | 实际审阅的 22 个源码/依赖文件 SHA256 |
| [verification.json](verification.json) | 命令、结果、工具链及证据指纹 |

从 `games_test/tiny-glade` 根运行：

```sh
cargo test -p country-core --lib input:: --offline --locked
python3 reports/ui-input/2026-10-18-input-review/run_probes.py
# 仅重放原缓存编译快照（不证明最新源码构建通过）：
python3 reports/ui-input/2026-10-18-input-review/run_cached_probes.py
```

探针断言的是**当前缺口能够被观察到**，因此通过不等于问题已修复，也不是修复后的回归验收条件。runner 独占创建临时 `tests/input_review_probe_20261018.rs`，运行后移除；保留的 probes.rs 不参与默认 Cargo 测试。探针中的 panic 由 catch_unwind 预期捕获。

原证据复用范围为 B 的现有反编译；本轮没有原二进制行为差分。审阅快照也不是整个并行工作区的冻结交付，不据这些检查提升原作等价性状态。
