# lang-lib

加载 YAML 语言包，发布本地化快照，并保存/恢复最近语言偏好。

## 功能与目录

src/lib.rs（Localization 与全局服务）；src/language_library.rs（语言包与缺失 key 缓存）；src/tests.rs、tests/global_configuration.rs、examples/。

## 来源

- 恢复源码：[shared-foundation/lang-lib](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/lang-lib/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/lang-lib/`。
- 原 B 包/PDB 归属：`crates/lang-lib/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`c05fad96`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[SUMMARY.md](../../../reverse-engineering/tiny-glade/evidence/foundation-plt-2026-10-13/SUMMARY.md)；[FOUNDATION_LIBRARY_REPAIRS.md](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/FOUNDATION_LIBRARY_REPAIRS.md)。

## 已完成

已恢复 pack 加载、en-US fallback、TG_DISABLE_LANG_FALLBACK、缺失 key 格式、偏好文件策略。当前版本采用 Arc<str> 和 ArcSwap 的可回收快照、显式配置、完整包事务加载、1024-key 缓存上限、验证语言标识和临时文件替换偏好。

## 尚差的功能与验收

真实游戏 UI key/语言 producer 的全链接线、发行语言包全量消费，以及各 OS 偏好替换行为需持续验证。安全所有权、缓存上限和事务偏好写入的产品策略不受历史原版逐函数报告覆盖。

## 持续开发差异

保留 c05fad96 产品修复的 owned API。写偏好失败会记录日志，原 B 静默忽略；独立 Localization::load_locale 与带保存入口分离。

在 workspace 根执行 `cargo test -p lang-lib --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。
