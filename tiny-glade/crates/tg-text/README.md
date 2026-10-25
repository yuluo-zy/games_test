# tg-text

界面文本的字体配置、编辑、排版、字形缓存和绘制批次模块。默认只运行 CPU 逻辑；`gpu` feature 提供基于 `rhapsody` 的 Vulkan 离屏渲染适配器。

## 职责与业务流程

1. `FontTable` 将 `Label`、`MenuButton`、`TextEditor`、`Hand` 四种业务角色映射为字体 family 与缩放比例。表允许只配置需要的角色；创建片段或切换角色时必须能找到该角色。
2. `TextRenderer` 显式加载字体字节，复用 fontdb 的共享字体源，不复制整份字体数据。默认字体数据库为空，不扫描系统字体；文本准备和设置操作在无字体时返回错误。配置中的 family 是字体匹配名称，找不到时沿用 cosmic-text 的已加载字体回退规则。
3. `TextFragment` 保存 cosmic-text 编辑器、原始字号、角色、对齐、最大宽度、操作队列及布局缓存。`prepare` 应用操作并排版；`render` 自动准备变化的文本、生成字形/选区/光标 primitive，并按中心和可用范围追加到帧。实际换行宽度取最大宽度与可用宽度中的较小值。
4. `TextRenderer` 栅格化未命中的字形，将 RGBA 上传区域和 primitive 放入当前帧。图集使用 etagere 分配空间；已消费旧帧的分配可被淘汰，当前帧引用的分配受到保护。
5. `consume` 转移 `RenderBatch` 的所有权并推进帧号。批次同时携带 primitive、shape words 和所需字形上传；GPU 消费者先上传再绘制。适配器返回 RGBA8 像素，文件编码及窗口合成由调用方负责。

`TextRenderer` 可作为 Bevy Resource，`TextFragment` 可作为 Component。当前 toolbar 使用字体配置和 renderer 资源；完整游戏窗口的文本提交调度由调用方集成。

## 模块

| 模块 | 功能 |
| --- | --- |
| `font_table` | RON 配置、字体角色、字号及缩放验证 |
| `fragment` | 编辑状态、动作、排版、选区/光标、renderer 专属缓存 |
| `renderer` | 字形栅格化、帧生命周期、批次构造 |
| `renderer::atlas`（内部） | 预算、空间分配、淘汰、句柄和图集代际 |
| `shape_stream::encoding`（内部） | shape packet 字段、保留值及 opcode 的共同定义 |
| `shape_stream` | 填色、图片、圆角的 shader 指令编码 |
| `validation` | CPU 批次预检与渲染资源预算 |
| `gpu`（可选） | 公开 API、初始化、串行提交和结果发布 |
| `gpu::{input,frame}`（内部） | 输入验证/ABI 编码、具名帧资源、命令记录和回读 |
| `error`、`color` | 可恢复错误及颜色工具 |

## 字体配置与使用

`FontTable::from_ron` 校验所有已配置条目。字体名称不能空白；字号、行高、scale 和 pixels-per-point 必须有限且大于零，缩放结果也必须有效。直接 Deserialize 后需调用 `validate`；片段入口还会再次验证所用条目。

```rust,no_run
use std::sync::Arc;
use tg_text::{FontFamily, FontMetrics, FontTable, TextFragment, TextRenderer};

# fn main() -> tg_text::TextResult<()> {
let fonts = Arc::new(FontTable::from_ron(
    r#"(fonts:{Label:(family:"Ubuntu",scale:1.0)})"#,
)?);
let mut renderer = TextRenderer::new();
renderer.load_font_from_file("assets/fonts/ubuntu/Ubuntu-Regular.ttf")?;
let mut label = TextFragment::new(FontMetrics::new(24.0, 30.0), FontFamily::Label, fonts)?;
label.set_text(&mut renderer, "Tiny Glade")?;
renderer.begin_frame();
label.render(&mut renderer, [150.0, 50.0], [300.0, 100.0], u32::MAX)?;
let batch = renderer.consume()?;
assert!(!batch.primitives.is_empty());
# Ok(())
# }
```

公开颜色输入使用 `0xRRGGBBAA`，最低字节是 alpha；编辑器的 `cosmic_text::Color` 通过其通道接口转换，不能直接使用 `.0`。primitive 位置和尺寸使用逻辑 UI 单位，GPU 适配器把 1000 单位的逻辑高度映射到目标高度；字形 UV 使用图集像素。

片段的度量/角色变化保留文本、光标、选区和编辑设置。`editor_mut`、替换编辑器、对齐/宽度/光标变化都会使绘制缓存失效。`render` 的颜色参数是全局 RGBA tint，逐通道乘上局部颜色；白色保持原色及选区透明度。`bounds` 在准备或绘制后更新。

## 帧、缓存与错误恢复

`begin_frame` 丢弃未消费的绘制指令，并保留仍需上传的字形。通常每帧从 `begin_frame` 开始，追加所有片段，然后 `consume`。`consume_with` 借用帧的 slices 执行不可失败的回调并复用向量容量；`try_consume_with` 的回调返回 `TextResult`，失败时保留指令、上传和帧号，允许重试。所有权已转入批次后，GPU 失败应保留该批次以便重新提交；重试期间不要推进其他文本帧或更改图集，以免其坐标所指的分配被重用。

缓存同时验证 renderer 身份、图集代际、分配标识和几何，不能把另一个 renderer 的 glyph 当作命中。加载字体、修改 DPI、显式 reset 或图集增长都会发布全局唯一的代际；片段会据此重建缓存。重建 GPU 适配器后，应 `reset_glyph_cache` 并重新生成完整帧，使新图集收到像素。

默认 `AtlasConfig` 初始为 1024×1024，上限 4096×4096、64 MiB RGBA。构造器验证维度和 allocator 整数范围；增长在可达的两轴尺寸组合中选择能容纳请求的最小面积，受最大尺寸及字节预算限制。分配与负缓存元数据也有固定数量上限。

- `AtlasChanged`：增长已经发生，当前帧被废弃；调用 `begin_frame` 后重新绘制**整个帧**，不能仅重试失败片段。增长次数受配置上限约束，调用方仍应设置有限重试次数。
- `AtlasFull`：当前工作集无法在预算内分配；减小字号/文本工作集或调整配置。不要无限重试同一帧。
- 无字体、缺少角色、非法几何/配置、过长 shape 和无效图片索引：返回 `TextError`，由界面展示或调用方处理。

帧内生成失败不会追加半个文本片段。图集增长会使此前追加的所有 primitive 失效，禁止消费，直到新帧开始。

## Shape 协议与 ABI

`Primitive` 固定为 48 字节，`stream == u32::MAX` 表示实心矩形，`stream == 0` 表示字形，其余为 24 位 word 起点和 8 位 word 数量。空 shape 绘制实心矩形；单个 shape 最多 255 words。填色占两个 words，图片和圆角各占一个。图片索引限制在 16 位；圆角限制为有限的 0…255 并截去小数。图片支持 Stretch、FitHeight、BlurExtend 和 Nine。

命名图片通过 resolver 转为索引；`encode_no_images` 允许数字索引但拒绝命名图片。`RawShapeStream::try_append` 在成功编码整个程序后才追加，错误保留已有 words；resolver 自身的副作用由调用方负责。公开 raw 数据便于手动批次构造，CPU rectangle 入口及 GPU 提交前都会检查指令语法。

## Vulkan 适配器

启用 `gpu` 后，`GpuTextRenderer` 接收设备、已编译 text shader 和 blue-noise 纹理；可创建独立 bindless heap 或通过 `new_with_heap` 复用宿主的 heap。构造时预热管线，ticket 等待设置五秒超时；compiler 释放时会等待正在进行的驱动编译，因此该超时不等于构造函数的墙钟上限。帧内轮询已有管线。`render_batch_offscreen` 自动使用批次上传，`render_offscreen` 允许显式传入上传 slices。

提交前验证几何、shape 引用、图片、图集代际、上传边界、设备限额与 `RenderLimits`。默认限制：目标 256 MiB、图集 64 MiB、100 万 primitive、100 万 shape words、64 MiB 上传、16,384 个上传区域和 400 万次非精确字形覆盖比较。重复 shape packet 只解析一次，累计解析 words 也受 shape words 预算约束；完整 glyph 上传使用快速精确查找，子矩形覆盖检查受工作量预算约束。超过预算返回错误。

渲染器复用 primitive/shape/uniform、上传和回读 buffer，以及同尺寸目标与同代际图集。只有确认前次提交完成后才覆写映射内存；descriptor 绑定逻辑长度，避免扩容容量超过设备 range 限额。等待默认 30 秒，可配置为大于零且不超过 300 秒。超时后资源由 timeline 保留，下次提交会在前次未完成时被拒绝。调用者必须遵守 unsafe 合同：外部采样纹理属于该设备且可采样，并保持不变直到实际提交完成，超时也适用。

可运行示例（输出原始 `text.rgba`、尺寸 JSON 和管线缓存）：

```sh
cargo run -p tg-text --features gpu --example text_gpu -- <游戏资源目录> <输出目录>
```

示例显式使用资源目录中的 Ubuntu 字体，不依赖 Windows 或系统字体路径。

## 验证

在 workspace 根运行：

```sh
cargo test -p tg-text --all-targets --offline
cargo test -p tg-text --doc --offline
cargo clippy -p tg-text --all-targets --all-features --offline --no-deps -- -D warnings
```

单元与集成测试聚焦配置/编码失败的原子性、编辑状态、缓存所有权、增长/预算和消费恢复。真实字体测试使用仓库素材目录；GPU 测试需显式资源与设备：

```sh
TG_TEXT_REQUIRE_GPU=1 TG_TEXT_ORIGINAL_ROOT="<游戏资源目录>" \
  cargo test -p tg-text --features gpu --test gpu_pipeline --offline -- --test-threads=1
```

`TG_TEXT_REFERENCE_PATH` 可指定已有 RGBA 基准，同时读取其 `.image.rgba` 对照；路径不存在时测试会导出首次结果。普通 CPU 环境缺少 GPU 参数时测试会输出 SKIP，不能据此声称 GPU 验证通过。该测试覆盖 shader 消费与资源复用，未覆盖真实游戏窗口调度。

2026-10-18 验证：18 项 CPU 测试、README 示例编译、完整 feature 的严格 Clippy 与格式检查通过。严格 MoltenVK 测试 2/2 通过，9 帧与既有像素基准一致，另有 2 帧验证非白色文字与半透明选区，验证层及同步验证零错误。新示例的两帧像素一致，上传区域从 24 降为 0，传输 buffer 稳定为 2。

本轮写法与注释标准复核见 [整理报告](../../reports/ui-input/2026-10-18-gpu-text-style/README.md)。初轮源码审查、修复和实际验证结果见 [审查报告](../../reports/ui-input/2026-10-18-gpu-text-refactor/README.md)。历史 shader 消费基准见 [bindless 报告](../rhapsody/reports/refactor/2026-10-17-v9-text-bindless/README.md)。本模块源自行为恢复工程，本轮仅审查现有 Rust 源码并进行工程重构。
