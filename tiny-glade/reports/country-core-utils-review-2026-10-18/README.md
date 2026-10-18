# country-core utils 分析与审查证据

业务与功能：[utils/README.md](../../crates/country-core/src/utils/README.md)。完整问题与范围：[utils/REVIEW.md](../../crates/country-core/src/utils/REVIEW.md)。本目录针对 **country-core/src/utils**，与工作区共享 `crates/utils` 的既有审查报告分开。

本次仅添加说明和有界审查探针，没有修改业务源码、Cargo 依赖或逆向数据库。结果属于当前源码合同检查；未完成原版差分或全游戏/GPU/FMOD 验收。

结果：`utils::` 过滤测试 **85 passed**，其中本目录 **82** 项，另有 3 项同名工具测试；栅格直接源码检查 **10 passed**（8 既有 + 2 反例）；网格加载/资产库限定源码检查 **47 passed**（45 既有 + 2 探针）。两个正确性问题均可复现，尚未修复。

完整 crate 的集成探针构建失败：目录外的输入回放模块与 `InputStream` 接口不匹配，出现 `InputTranslationState` 缺失、`add_frame/on_history_sync` 等方法缺失及索引类型错误。先前单元测试成功与后来构建失败是各自运行时的事实，不能据前者宣称当前整个 country-core 可编译。本轮未改动这些目录外文件。

| 文件 | 内容 |
|---|---|
| `utils-tests.log` | 当前源码的 `cargo test --lib utils::` 原始输出，最终结果以 verification.json 为准 |
| [raster-probe.rs](raster-probe.rs) / [raster-probe.log](raster-probe.log) | 直接包含当前栅格源码，原有 8 测试 + 2 个有界反例 |
| [probes.rs](probes.rs) / [run_probes.py](run_probes.py) / [probes.log](probes.log) | 完整 crate 的探针入口；本次受目录外编译错误阻断 |
| [run_source_probes.py](run_source_probes.py) / [source-probes.log](source-probes.log) / [source-probes.json](source-probes.json) | 直接编译当前 Mesh、加载器、资产库、FrozenMap 和实际 Handle 声明，成功运行同一组探针；记录输入与依赖产物指纹 |
| [source-probes-missing-manifest.log](source-probes-missing-manifest.log) | 限定源码 runner 首次缺少 Bevy 派生宏所需的 Cargo 环境，补入实际 manifest 环境后通过 |
| `reviewed-sources.json` | 实际审阅源码/历史证据的 SHA256；不是整个工作区的冻结交付 |
| `verification.json` | 工具链、命令、退出码、范围与最终测试摘要 |

在 `games_test/tiny-glade` 根运行：

```sh
cargo test --offline --locked --target-dir /tmp/tg-country-core-utils-review-20261018-target -p country-core --lib utils::
rustc --edition=2024 --test reports/country-core-utils-review-2026-10-18/raster-probe.rs -o /tmp/tg-country-utils-raster-probe
/tmp/tg-country-utils-raster-probe
python3 reports/country-core-utils-review-2026-10-18/run_probes.py
# 完整 crate 被其他模块阻断时，复验限定源码（要求此前依赖已构建）：
python3 reports/country-core-utils-review-2026-10-18/run_source_probes.py
```

探针通过表示**问题可复现**，不表示问题已修复。极值遍历在第 4 次回调主动中止；没有实际等待数十亿次枚举。Cargo runner 独占创建临时 `tests/country_utils_review_20261018.rs`，结束后删除；保留在本目录的探针不会进入默认测试集。

限定源码 runner 直接引用现有实现文件，仅从 `asset_libraries/mod.rs` 选择第一个 `pub mod` 前的原始声明以避免引入其他资产子模块，没有用替身重写 Handle/Mesh/加载器/资产库。公共与共享依赖使用本轮 Cargo 已构建的产物，不使用旧 country-core rlib。此次范围不覆盖完整 crate 的调度、其他资产库或原版游戏进程。`source-probes.json` 确认所选源码在运行前后指纹一致。

首次使用默认 target 的测试在等待其他构建锁时被主动取消；随后使用本报告独立 `/tmp` target 重新构建。Rust 测试的 `utils::` 是子串过滤，会包含同名的其他 utils 测试，精确本目录数量在 verification.json 单列。源码原有警告照实保留，没有宣称严格 Clippy 通过。
