# tg-iced

本地化对话框与独立 GUI 运行模块，提供消息配置、确认结果、可注入后端，以及 iced 原生窗口和静态截图能力。

本模块拥有对话框交互策略与窗口适配。窗口、事件循环、控件绘制和字体排版由锁定的 iced/winit 提供；业务链接、文件操作和具体设备配置由调用方负责。`tg-gpu-selector` 使用本模块运行 GPU 选择与 Vulkan 配置界面。

业务流程分为三条：对话框将确认/取消消息变成可选结果；通用 runner 将业务消息交给应用并适配窗口命令；静态截图包装器只借用应用视图，自行计时、请求截图和结束运行。截图持久化是单独步骤，调用方选择输出路径，并处理 I/O 错误。

## 功能与构建特性

| 特性 | 能力 |
| --- | --- |
| 默认，无 feature | 对话框配置、纯视图、结果状态机、本地化查询和 `GuiBackend` 抽象 |
| `native-gui` | iced 重导出、原生窗口、命令适配、locale 加载、字体 shaping、截图与文件输出 |

默认构建不创建窗口，可以接入调用方提供的 `GuiBackend`。原生接口只在启用 `native-gui` 时导出。

## 模块结构

| 模块 | 职责 | 主要接口 |
| --- | --- | --- |
| `dialog` | 配置、静态按钮描述、消息与终态 | `MessageDialog`、`Controls`、`DialogView`、`Message` |
| `dialog/native` | 对话框原生布局 | `show_native`、`Controls` 的原生视图 |
| `gui_runner` | 窗口参数、应用协议、命令与后端抽象 | `Options`、`ProgramWithOutcome`、`Command`、`GuiBackend`、`GuiError` |
| `gui_runner/native` | 原生事件循环、命令适配与结果回收 | `NativeProgram`、`NativeAction`、`run_native_gui`、`platform_command` |
| `gui_runner/capture` | 静态捕获与业务输入隔离 | `capture_native_gui` |
| `gui_runner/capture_file` | 原始像素和元数据输出 | `write_native_capture` |
| 包根 | 翻译与文本入口 | `text`、`load_gui_locale`、`native_text` |

## 对话框业务

`MessageDialog::new(width, height)` 创建空标题、空正文、Info 等级及 Ok 按钮的配置。`set_title`、`set_description`、`set_level`、`set_buttons` 消费配置并返回更新值，可以链式调用。标题和正文接收最终显示文本，按钮语言键在原生视图构造时查询。

| 等级 | 图标 |
| --- | --- |
| `Info` | 🐑 |
| `Warning` | ⚠️ |
| `Error` | ❌ |

| 按钮组 | 语言键与消息 |
| --- | --- |
| `Ok` | `Ok` → Confirm |
| `OkCancel` | `Ok` → Confirm；`Cancel` → Cancel |
| `Continue` | `Continue.Yep` → Confirm；`Continue.Nope` → Cancel |

`Controls::view()` 返回借用标题、正文的 `DialogView`，按钮为静态切片，不在每次查询时分配集合。首次 Confirm / Cancel 形成不可覆盖的终态，并请求关闭窗口。

| 返回结果 | 业务含义 |
| --- | --- |
| `Ok(Some(true))` | 用户确认 |
| `Ok(Some(false))` | 用户取消 |
| `Ok(None)` | 窗口关闭，未形成业务选择 |
| `Err(GuiError)` | 后端或命令失败 |

`show(backend)` 使用注入后端，`show_native()` 使用原生后端。取消与直接关闭具有不同语义，调用方应分别处理。

## 使用示例

### 不创建窗口的状态处理

```rust
use tg_iced::dialog::{Controls, Message, MessageButtons, MessageDialog};
use tg_iced::gui_runner::ProgramWithOutcome;

let dialog = MessageDialog::new(640, 480)
    .set_title("Continue?")
    .set_description("Apply the selected configuration.")
    .set_buttons(MessageButtons::OkCancel);
let mut controls = Controls::new(dialog);
controls.update(Message::Confirm);
assert_eq!(controls.program_outcome(), Some(true));
```

### 原生对话框

开启 `native-gui` 后，在主线程设置资源工作目录、加载 locale，再运行窗口。

```rust,no_run
# #[cfg(feature = "native-gui")]
# fn main() -> Result<(), tg_iced::gui_runner::GuiError> {
use tg_iced::dialog::{MessageButtons, MessageDialog};

tg_iced::load_gui_locale()?;
let result = MessageDialog::new(640, 480)
    .set_title("Tiny Glade")
    .set_description("Apply the selected configuration?")
    .set_buttons(MessageButtons::OkCancel)
    .show_native()?;
match result {
    Some(true) => println!("Confirmed"),
    Some(false) => println!("Cancelled"),
    None => println!("Closed without a selection"),
}
# Ok(())
# }
# #[cfg(not(feature = "native-gui"))]
# fn main() {}
```

## 应用与后端合同

`ProgramWithOutcome` 定义关联消息、结果、`update`、`program_outcome` 和可选 `tick_message`。`GuiBackend<P>::run` 负责界面、消息、命令及结束运行。`run_gui` 在后端成功返回后读取结果，后端错误优先传播。

`program_outcome()` 是无副作用的观察方法，结果一旦产生便须保持稳定。当前接口返回拥有所有权的值，具体实现可能需要 clone；原生 runner 在运行中检查状态，进入关闭状态后停止轮询，只在成功回收时再读结果，因此已完成的结果只在终态附近被取出少数几次。接口按值返回，复制成本由具体结果类型承担；截图的 clone 共享 `Arc` 像素存储，不复制整幅图像。

`NativeProgram` 额外提供拥有数据的 `view_native()`，以及返回 `Result<NativeAction<_>, GuiError>` 的 `update_native()`。视图返回 `'static` 元素，因为程序互斥锁在构造视图后释放，不能把锁内数据的借用交给后续渲染；需要的显示文本由视图拥有。`NativeAction::Continue(command)` 执行 iced 命令，`NativeAction::Close` 请求结束运行。原生 runner 收到关闭请求、结果或错误时，立即进入关闭状态，再关闭窗口；运行前已完成的程序直接返回，不创建窗口。

`tick_message()` 为 Some 时订阅约16毫秒周期消息，处理时再次获取消息；该方法应轻量且无业务副作用。关闭状态停止查询 tick，并丢弃同批及后续排队的业务消息，已有结果和首个错误不会被覆盖。原生覆写必须通过 `NativeAction::Close` 关闭窗口，不能将 iced 窗口关闭命令藏在 `Continue` 中。

### 命令分派

| `Command` | 默认原生处理 |
| --- | --- |
| `None` | 无操作 |
| `CloseWindow` | 返回 `NativeAction::Close`，由 runner 冻结更新后关闭主窗口 |
| `CopyToClipboard` | iced 剪贴板命令 |
| `OpenUrl`、`OpenPath` | 返回 `UnsupportedCommand`，由业务应用覆写适配 |

业务应用在 `update_native` 中处理自己的 I/O，成功后返回 `NativeAction::Continue(iced::Command::none())`；其他命令交给 `platform_command`。失败应返回 `GuiError`，由 runner 保存错误、关闭窗口并返回调用方。GPU 选择器的浏览器和日志目录打开操作采用此路径。

### 错误与运行约束

| 错误 | 含义 |
| --- | --- |
| `BackendUnavailable` | 后端不可用，由适配方报告 |
| `Backend(String)` | 后端、业务 I/O 或原生执行异常 |
| `UnsupportedCommand(Command)` | 缺少命令适配 |
| `ClosedWithoutOutcome` | 要求结果的操作未获得结果，例如用户在截图返回前关闭窗口 |
| `CaptureTimedOut` | 截图在五秒期限内未返回 |

锁定 iced/winit 要求在主线程创建事件循环，同进程只允许创建一次。后端执行阶段将可捕获的 Rust unwind 转为含详情的 Backend 错误，保留现有 panic hook；运行前后读取 `program_outcome()` 的调用位于该捕获边界之外。此转换不提供事件循环复用能力。结果通过共享互斥状态回收，业务更新与视图回调由事件循环调用。

## 本地化与布局

`text(key)` 返回 `lang-lib` 的共享翻译文本。`load_gui_locale()` 读取上次选择的语言；调用方负责资源目录和工作目录，显示窗口不隐式加载语言。`native_text()` 启用 Advanced shaping，不注册额外字体。

原生尺寸至少为1×1。对话框使用整页滚动、16像素内边距、纵向伸缩按钮和受限内容宽度。目标平台的字体、DPI、长翻译及窗口尺寸需在集成环境进行视觉验证。

## 静态截图

`capture_native_gui(program, options)` 打开可见窗口，只渲染业务视图。包装器丢弃输入，不调用底层 update、业务 tick 或 outcome；调用方仍须保证 `view_native()` 没有副作用。

首次 tick 开始计时，达到200毫秒时请求一次截图；达到5秒仍未返回则关闭并报告 `CaptureTimedOut`。迟到的截图不能将超时变成成功，重复响应不能覆盖首次截图；已完成的捕获不会再超时。用户提前关闭窗口返回 `ClosedWithoutOutcome`。计时与状态转换独立于系统时钟读取，测试不需要等待真实时间。捕获不等待业务异步加载，也会占用本进程的原生事件循环。

`write_native_capture(image, path)` 输出原始 RGBA8 sRGB 像素至 `path`，输出 width、height、bytes、format 元数据至 `<path>.json`。它不进行 PNG 编码：`capture.rgba` 对应 `capture.rgba.json`，`capture.json` 对应 `capture.json.json`。

宽高必须非零，字节数必须等于 `width × height × 4`。两个文件只允许新建，目标存在返回 AlreadyExists；普通错误或 Rust unwind 回滚本次创建的文件，保留已有输出。该保证不涵盖强制终止、文件系统故障或其他进程并发修改输出。

## 构建与验证

在 workspace 根执行：

```sh
cargo test --offline --locked -p tg-iced --all-targets
cargo test --offline --locked -p tg-iced --features native-gui --all-targets
cargo test --offline --locked -p tg-iced --features native-gui --doc
cargo clippy --offline --locked -p tg-iced -p tg-gpu-selector --features tg-iced/native-gui,tg-gpu-selector/native-gui --all-targets --no-deps -- -D warnings
cargo fmt -p tg-iced --check
cargo run --offline --locked -p tg-iced --features native-gui --example tg_iced_native_probe -- <new-output-path>
```

测试覆盖配置、按钮键、确认/取消/直接关闭、关闭后消息与 tick 冻结、后端错误、异常转换、截图隔离、单次请求与超时终态，以及文件回滚。普通测试不启动窗口；截图示例会创建窗口并写入指定的新路径。

## 相关文档

- [B 版目录、类型和功能覆盖对照](../../reports/iced-history-b-comparison-2026-10-18/README.md)：当前接口和测试不代表原运行选项、默认配置、回调与文本样式均已等价恢复。
- [本轮源码审查、接口迁移与验证](../../reports/iced-history-refactor-2026-10-18/README.md)
- [GPU 选择与 Vulkan 配置消费模块](../tg-gpu-selector/README.md)
- [重构与接口迁移记录](../../reports/theme-ui-refactor-2026-10-17/README.md)
- [源码复审与修复记录](../../reports/theme-ui-rereview-2026-10-17/README.md)
