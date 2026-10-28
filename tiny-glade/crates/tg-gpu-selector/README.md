# tg-gpu-selector

游戏层 GPU 选择结果、driver 建议、graphics 错误文案及 Vulkan workaround 策略。

## 职责与业务流程

本包接收调用者提供的 GPU 候选、驱动支持分类、初始化错误和 Vulkan layer 列表，负责选择/配置对话框的业务策略。设备枚举、驱动分类、Vulkan 设备创建、配置落盘都由包外负责。

- GPU 流程：`Device` 列表和日志路径 → `Controls` → 驱动建议与错误展示 → 用户选择可用设备，返回对应列表索引；关闭返回 `None`。不支持的驱动仍可选择“忽略并开始”，已有初始化错误的设备不能选择。
- Vulkan 流程：layer 列表与初始 `Configuration` → 按首次出现的 vendor 分组（包含无 vendor 的组）→ 编辑 layer/vendor allowlist 和六项 workaround → Apply 返回配置快照，Cancel 返回 `None`。`policy_word` 原样传递，不解释其含义。
- 默认 feature 只提供纯 Rust 状态机和可注入 `GuiBackend` runner。`native-gui` 提供 iced 原生布局，打开浏览器/日志目录的动作由原生 consumer 执行，失败通过 `GuiError` 返回。locale 和资源目录由调用方通过 `tg-iced` 配置。

当前 Rust 调用方是 `examples/tg_gpu_selector_native_probe.rs`。游戏 `tiny-glade/src/vulkan_config_main.rs` 有独立 `VulkanConfigGui` 接口，并未直接接入本包 runner；不能把探针编译视为生产启动链已接通。

## 模块与 API

两个公开入口模块 `gpu_selection`、`vulkan_config` 保留 facade，子模块按数据类型、纯策略、状态机和原生布局拆分，策略测试不依赖窗口。

- `gpu_selection/{model,advice,errors,controls,native}.rs`：GPU/驱动类型、静态更新链接、借用型建议、图形错误投影、选择原因、选择状态机和原生布局。
- `vulkan_config/{model,controls,native}.rs`：layer/vendor allowlist、workaround 与配置类型、预期线性时间的保序分组、配置状态机和原生布局。
- crate 根的四个 `run_*` 函数：默认可注入后端，以及 `native-gui` 原生运行入口。

## 业务不变式与验证

选择消息统一校验候选索引及 `Device::is_selectable()`，避免绕过 UI 禁用；有效选择、Apply、Close、Cancel 都冻结状态，后续消息只请求关闭，不修改配置/结果也不执行 IO。所有 runner 对已关闭的 Controls 直接返回结果。状态字段为私有，提供只读视图。原先空设备列表接受任意索引的 fixture 已移除，不能作为安全 API 契约。

验证聚焦非法/不可用选择、选择和取消后的状态保护、allowlist 去重切换、分组保序、workaround 与字段映射、驱动及错误展示策略。本轮不进行原作逆向，也不把逻辑测试等同于真实 GUI 像素验收。

## 开发期 API 迁移

- `gpu_selection::Message::Select(DeviceId::new(index))` 代替裸 `usize`；合法结果仍是输入列表的 `usize` 索引。`DeviceId` 不承诺索引合法，状态机负责验证。
- `vulkan_config::Message::ToggleWorkaround(Workaround::SerialRecording)` 代替魔数索引；`Workaround::ALL`、`label()` 和 `Workarounds::get/toggle` 集中定义顺序/字段。`workarounds_view()` 返回 `(Workaround, bool)`。
- `Controls` 的数据字段改为私有：GPU 使用 `view()`、`more_info()`、`log_path()`、`heading()`、`is_closed()`；Vulkan 使用 `view()`、`allowlist_view()`、`workarounds_view()`、`is_closed()`。`allowlist_view()` 的元素改为具名 `LayerGroup { vendor, layers }`。
- `GraphicsErrorView<'a>::details` 改为借用错误文本的 `Option<&'a str>`，避免每帧复制诊断字符串；无调用方的纯转发别名 `render_error_to_ui()` 已移除，统一使用 `localize_graphics_init_error()`。
- `get_vendor_driver_update_links()` 和 `DriverAdvice.links` 返回静态 slice；`DriverAdvice<'a>` 借用版本文本，`requirement` 改为 `MinimumVersion`、`KnownGoodVersion`、`VendorId` 三种具名详情，避免把 vendor ID 或 known-good 版本误标成最低版本。

原生布局使用 Advanced shaping、纵向按钮与整页滚动，使长文本和较小窗口仍可滚动到操作区。现有最低版本文案沿用本地化 key；known-good 版本及 vendor ID 补充标签与 Vulkan 说明当前使用英文。原生 runner 的主线程/单事件循环约束和截图文件格式、仅新建输出约束见 `tg-iced` README；探针输出是 RGBA 原始字节及 `<输出路径>.json`，不是 PNG。

## 本地验证（2026-10-18）

在 workspace 根运行：

```sh
cargo test -p tg-gpu-selector --all-targets --offline
cargo test -p tg-gpu-selector --features native-gui --all-targets --offline
cargo clippy -p tg-gpu-selector --all-targets --offline --no-deps -- -D warnings
cargo clippy -p tg-gpu-selector --features native-gui --all-targets --offline --no-deps -- -D warnings
```

默认和 `native-gui` 的 all-targets 测试各 7 项通过，两个配置的 Clippy `-D warnings` 和本包格式检查通过。检查在 macOS 离线完成，未启动真实窗口；锁定依赖 `block 0.1.6` 有 Rust future-incompat 提示。

写法、接口注释及借用数据整理记录见 [整理报告](../../reports/ui-input/2026-10-18-gpu-text-style/README.md)。

## 来源

- 恢复源码：[shared-foundation/tg-gpu-selector](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-gpu-selector)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-gpu-selector/`。
- 原 B 包/PDB 归属：`crates/tg-gpu-selector/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`3e0387cd`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/REPORT.md)；[B-SURFACE.md](../../../reverse-engineering/tiny-glade/evidence/closures-2026-10-13/gpu-selector/B-SURFACE.md)。
- 本轮规则测试与原 RNE 叶案例复验：[收口报告](../../../reverse-engineering/tiny-glade/evidence/tg-gpu-selector-completion-2026-10-14/REPORT.md)。

## 已完成

已观察到的 B 版库内策略已实现：设备选择 outcome、vendor/driver 建议、graphics 错误映射、allowlist 分组与切换、六项 workaround 的顺序及 Apply/Cancel。保留原生 GUI 探针；原码生成式消息 fixture 已由业务不变式测试替换。规则、函数身份与证据边界见 [BUSINESS_RULES.md](BUSINESS_RULES.md)。

## 尚未验收的边界

真实 rhapsody 设备枚举/driver_support 生产者及 Apply 后 device 创建调用链属于库外接线；`GraphicsInitError` 等跨依赖原始类型 ABI、完整 iced 元素树/像素和 A/C 行为尚未恢复或验收。通用驱动分类与 Vulkan 枚举由原依赖提供，本 crate 不重写这些库。

在 workspace 根执行 `cargo check -p tg-gpu-selector --features native-gui --all-targets` 可检查原库窗口适配。默认 feature 只编译库内业务，便于持续开发。
