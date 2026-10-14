# tg-gpu-selector

游戏层 GPU 选择结果、driver 建议、graphics 错误文案及 Vulkan workaround 策略。

## 功能与目录

src/gpu_selection.rs、vulkan_config.rs、lib.rs；src/fixtures/message_cases.rs。

## 来源

- 恢复源码：[shared-foundation/tg-gpu-selector](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-gpu-selector/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-gpu-selector/`。
- 原 B 包/PDB 归属：`crates/tg-gpu-selector/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`3e0387cd`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/REPORT.md)；[B-SURFACE.md](../../../reverse-engineering/tiny-glade/evidence/closures-2026-10-13/gpu-selector/B-SURFACE.md)。
- 本轮规则测试与原 RNE 叶案例复验：[收口报告](../../../reverse-engineering/tiny-glade/evidence/tg-gpu-selector-completion-2026-10-14/REPORT.md)。

## 已完成

已观察到的 B 版库内策略已实现：设备选择 outcome、vendor/driver 建议、graphics 错误映射、allowlist 分组与切换、六项 workaround 的顺序及 Apply/Cancel。原码消息 fixture 和 native GUI 探针已迁入。规则、函数身份与证据边界见 [BUSINESS_RULES.md](BUSINESS_RULES.md)。

## 尚未验收的边界

真实 rhapsody 设备枚举/driver_support 生产者及 Apply 后 device 创建调用链属于库外接线；`GraphicsInitError` 等跨依赖原始类型 ABI、完整 iced 元素树/像素和 A/C 行为尚未恢复或验收。通用驱动分类与 Vulkan 枚举由原依赖提供，本 crate 不重写这些库。

在 workspace 根执行 `cargo check -p tg-gpu-selector --features native-gui --all-targets` 可检查原库窗口适配。默认 feature 只编译库内业务，便于持续开发。
