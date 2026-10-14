# paths

管理安装目录、账户配置和 saves/beta-saves/backup-saves/tmp/deleted-saves 的创建与查询。

## 功能与目录

src/lib.rs；tests/global_initialization.rs。

## 来源

- 恢复源码：[shared-foundation/paths](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/paths/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/paths/`。
- 原 B 包/PDB 归属：`crates/paths/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`c05fad96`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[SUMMARY.md](../../../reverse-engineering/tiny-glade/evidence/foundation-plt-2026-10-13/SUMMARY.md)；[FOUNDATION_LIBRARY_REPAIRS.md](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/FOUNDATION_LIBRARY_REPAIRS.md)。

## 已完成

B 版目录创建顺序、Steam 账户子目录、Saved Games 获取与 getter 均已有恢复。当前持续开发版本增加初始化错误返回、重试/并发边界、绝对路径化、非 Unicode 路径、Windows/macOS/Linux 默认路径，以及 preference_root_path。

## 尚差的功能与验收

跨平台默认策略与安装目录来自产品修复，并非 B 原版证明；Windows/Linux/Rust 1.86 多平台运行尚须复验。启动与真实存档生命周期仍需主工程接线。

## 持续开发差异

保留 c05fad96 的持续开发修复：重复初始化返回 Err，不再 panic；installed 使用可执行文件父目录；不会默默回滚为旧等价快照。

在 workspace 根执行 `cargo test -p paths --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。
