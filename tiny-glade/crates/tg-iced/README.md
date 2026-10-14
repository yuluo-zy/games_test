# tg-iced

游戏手写的本地化对话框选项、结果和文本入口，native-gui 特性直接使用原 iced fork。

## 功能与目录

src/dialog.rs、gui_runner.rs、lib.rs；src/fixtures/dialog_cases.rs；examples/tg_iced_native_probe.rs。

## 来源

- 恢复源码：[shared-foundation/tg-iced](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-iced/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-iced/`。
- 原 B 包/PDB 归属：`crates/tg-iced/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`3e0387cd`、`c05fad96`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/REPORT.md)；[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/ui/REPORT.md)。

## 已完成

对话框标题/选项/结果、文本本地化、真实 GUI runner 和探针均已有实现。原库控件/窗口/font shaping 直接复用，不包含自写 iced。

## 尚差的功能与验收

原 GUI 默认样式、调度和完整像素验收、游戏启动真实 consumer、A/C 未完成。窗口/字体 glue 是重建适配，不能认作原函数 ABI。

## 持续开发差异

本工程默认仅库内策略，native-gui 显式启用；迁移时仅调整 feature 默认和 workspace 配置。

在 workspace 根执行 `cargo check -p tg-iced --features native-gui --all-targets` 可检查原库窗口适配。默认 feature 只编译库内业务，便于持续开发。
