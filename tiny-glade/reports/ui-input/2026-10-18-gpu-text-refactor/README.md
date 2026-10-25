# GPU selection and text module source review

日期：2026-10-18。范围：`tg-gpu-selector`、`tg-text` 现有 Rust 源码审查与工程重构，及 toolbar 字体表调用方的必要错误传播适配。未调用逆向工具或分析原作二进制；GPU 验证直接使用已有着色器资源与已有像素基准。

## 正式模块文档

- [tg-gpu-selector](../../../crates/tg-gpu-selector/README.md)：职责、设备选择与 Vulkan 配置流程、模块/API、状态不变式、迁移及验证边界。
- [tg-text](../../../crates/tg-text/README.md)：字体角色、编辑排版、图集/帧生命周期、shape ABI、错误恢复、CPU/GPU 预算及示例。

两份 README 均作为 crate 的 rustdoc 首页。历史恢复证据与当前工程合同分开，历史 fixture 不再覆盖当前正确性要求。

## 已修复的问题

| 范围 | 问题与修复 |
| --- | --- |
| 设备选择 | 裸 Select 消息可绕过 UI 禁用；类型化索引并统一校验边界及初始化错误 |
| 对话框生命周期 | 选择、关闭、Apply/Cancel 后仍能更新结果或执行动作；状态封装并冻结终态 |
| 驱动提示 | vendor ID、known-good 版本误标为最低版本；具名详情类型及静态链接 |
| Vulkan 配置 | 按 vendor 分组反复扫描；HashMap 索引配合 Vec 保留首次出现顺序；workaround 使用枚举 |
| 字体配置 | 空 family、非法缩放/度量和溢出进入排版；输入校验、缺失角色错误；空字体库返回错误，避免 panic |
| 文本状态 | 角色切换没有完整写回，编辑/对齐/宽度/光标改变未可靠失效；集中失效并保留编辑状态 |
| shaping 缓存 | 片段首次绘制前换 renderer 仍可能携带旧字体 ID；独立 shaping token 同步字体上下文 |
| 布局范围 | 可用范围变化后 bounds 宽度仍旧；成功生成后同步计算宽、高，prepare 也遵守最大宽度 |
| 图集身份 | renderer 的代际冲突、同帧 reset 失效遗漏及 allocator ID 重用；全局唯一代际、owner/nonce/几何验证 |
| 图集增长 | 贪心扩容在字节预算足够时误报满；枚举两轴可达尺寸，选择符合请求的最小面积；8×8→32×8 回归 |
| 图集资源 | 无界增长、元数据累积与计数器回绕；尺寸/字节/分配数/负缓存上限及 checked counter |
| 帧事务 | 增长后旧 primitive 混入新图集；清空并禁止消费，要求重建完整帧；回调失败新增 try_consume_with 原子消费 |
| Shape | 空 shape 被误编码为 glyph；保留实心 sentinel；拒绝索引截断、非法圆角、未知 opcode、越界 packet；编码失败保留已有数据 |
| 颜色 | 全局颜色覆盖局部色与选择透明度；RGBA tint 逐通道相乘，白色保持原色 |
| GPU 预检 | 上传越界、巨大目标/图集、无界 region 数和重复校验工作；统一纯 CPU 预检及明确预算 |
| GPU 复用 | 每帧新建上传/回读 buffer；完成提交后复用；descriptor range 使用逻辑长度，避免扩容容量错误触发设备上限 |
| GPU 等待 | 无限等待；配置有限超时，未完成资源继续由 timeline 保留，阻止后续覆写 |
| 调用方 | toolbar 绕过字体验证并 unwrap；两层 loader 传播 TextResult，成功后才替换共享表 |
| 示例 | Windows 字体硬编码与忽略 Result；改为显式资源目录、可恢复图集重试和冷暖帧一致性验证 |

优化依据是算法复杂度、资源上限和实际复用计数；本轮未进行吞吐量基准测试。默认与原生对话框的 UI 依赖保持现有锁定版本。

## 开发期 API 迁移

`tg-gpu-selector` 的选择消息改用 `DeviceId`；workaround 使用 `Workaround`，Controls 数据通过只读 getter 访问。runner 的最终结果类型保持不变。

`tg-text` 的构造/修改/准备/绘制、shape 编码及消费等失败入口返回 `TextResult`。`FontMetrics::new` 仍是普通值构造，使用入口会验证。`TextRenderer::new` 保持无需参数，字体必须显式加载。图集 generation/frame 计数改为 u64，Glyph 的身份字段私有，font system 只读访问。具体签名和恢复顺序以正式 README 为准。

消费得到的批次包含 glyph_uploads；推荐 `render_batch_offscreen` 避免遗漏上传。保留错误批次重试应在推进其他文本帧或更改图集前进行；重新创建 GPU 适配器需要 reset 并重建完整帧。

## 验证结果

| 检查 | 结果 |
| --- | --- |
| tg-gpu-selector 默认 / native-gui | 各 7 项通过 |
| tg-text CPU 回归 | 17 项通过 |
| 两包完整 feature all-targets | 24 项 CPU 通过；其中 GPU 的 2 项普通环境入口跳过，单独严格执行见下一行 |
| 严格 MoltenVK GPU | 2/2 通过、9 帧，无 SKIP、VUID 或 SYNC-HAZARD；与两份既有 RGBA 基准逐字节相同 |
| README doctest | 1 项编译通过 |
| Clippy / rustfmt | 两包完整 feature、all-targets、`-D warnings` 及格式检查通过 |
| toolbar 调用方 | cargo check 通过；依赖 country-core 有 14 个既有警告 |
| 真实文本示例 | 冷暖像素一致：31 primitive；24→0 上传区域；2→2 传输 buffer |

锁定依赖 `block 0.1.6` 的 Rust future-incompat 提示仍由 Cargo 输出，本轮未更换依赖版本。

[实际渲染预览](example-output/text.png) 与 [尺寸、像素哈希和复用统计](example-output/text.json) 已保留；生成的 4 MB 原始 RGBA 和机器相关管线缓存未纳入报告。

最终验证记录见 [validation.json](validation.json)、[tests.log](tests.log)、[clippy.log](clippy.log)、[doctests.log](doctests.log)、[gpu.log](gpu.log) 与 [example.log](example.log)。

CPU 回归覆盖业务状态、角色/布局/编辑失效、编码原子性、跨 renderer shaping、图集扩容与预算、分配句柄和消费恢复。GPU 测试明确区分普通环境的 SKIP 与严格要求真实硬件的运行。真实 GPU 使用 MoltenVK，验证层及同步验证开启；9 帧检查 solid/glyph、运行时图片数组、已有基准逐字节一致性与传输 buffer 复用。示例另外验证真实字体与圆角背景的冷暖帧。

## 验证边界

原生 GUI 编译与状态机测试不代表真实窗口交互验收，本轮没有启动对话框。Vulkan 适配器的离屏渲染不代表完整游戏窗口文本调度。外部纹理所有者须遵守提交完成前保持资源内容不变的 unsafe 合同，超时也不解除该要求。小型默认测试不替代长时间压力、跨厂商 GPU 或完整游戏集成验收。

源文件索引的旧记录保存在 [source-manifest-before.json](source-manifest-before.json)；当前文件 SHA-256 更新到 [sources.json](../../../provenance/sources.json)。初始导入与历史恢复证据保持不变。toolbar 仅适配字体 loader，其他并行任务的输入系统变更不属于本报告。
