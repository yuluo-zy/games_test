# 写法与注释标准复核

日期：2026-10-18。范围：`tg-gpu-selector`、`tg-text`。按用户给定的命名、数据表示、结构、错误与所有权、注释及一致性标准继续整理现有 Rust 源码。

## 实现调整

| 标准 | 落地结果 |
| --- | --- |
| 命名 | 跨步骤变量采用 buffer、line、primitive、vendor_name、error_view 等明确名称；`tag`、`policy_word` 保留实际语义边界。原生视图去掉无助维护的单字母名。 |
| 数据表示 | shape packet 的 word offset/count、保留值与 opcode 集中到内部 encoding 模块；shader globals 使用具名字段及固定 24 字节编码，校验/编码共享布局定义。CPU 与 GPU 预检复用几何规则。 |
| 结构 | GPU 公开方法负责校验、完成门控、提交及发布结果；`gpu/input.rs` 负责输入验证/ABI 打包，`gpu/frame.rs` 负责具体资源和命令。`ValidatedFrame` 借用已验证输入，`PreparedFrame` 持有提交所需资源。布局 bounds 计算与已验证度量提交各集中到一个实现。 |
| 错误与所有权 | GPU 生产路径清除 unwrap；`TextRenderer::new` 唯一的 expect 保留，并明确进程耗尽 u64 图集 revision 的边界，fallible 构造器返回错误。测试中断言成功用的 unwrap 保留。字体验证/加载共享 fontdb `Source::Binary` 的 Arc 字节源。错误视图借用诊断文本；删除纯转发别名及重复 pipeline retain。 |
| 注释 | 公开入口说明约束、失败、缓存/编辑/帧生命周期及 unsafe 合同；内部说明验证后提交、映射内存完成门控和资源所有权。删除源码中的原地址/偏移来源说明，来源历史由原审查报告和源索引保留。 |
| 一致性 | 沿用现有 TextResult、rhapsody 资源接口和静态模块分工；globals 不再通过 Vec 构造，新增帧记录均为具名的具体数据结构。 |

## 额外修复：编辑器与 shader 的颜色边界

cosmic-text 的 `Color` 内部整数是 AARRGGBB，而本模块 primitive 的颜色是 RRGGBBAA。此前直接使用 `.0`，选区的 `0x80ffffff` 也把 alpha 放错了字节。

现在通过 `Color::as_rgba()` 和 `u32::from_be_bytes()` 在一个私有访问边界完成转换，选区使用 `0xffffff80`，全局 tint 保留局部颜色及透明度。示例背景也改为明确的 RRGGBBAA 值。

一个 CPU 回归测试验证跨类型颜色转换和 opacity；既有 GPU 测试增加两帧真实字体场景，验证绿色局部字色和半透明中性选区。测试仍为两项 GPU 集成测试，避免为每个内部实现细节新增测试。

## 注释准确性与不变量

- primitive 位置/尺寸为逻辑 UI 单位，GPU 适配器将 1000 单位逻辑高度映射到目标高度；字形 UV 为图集像素。修正原先“位置为设备像素”的错误说明。
- renderer 产生的图集 revision 保证进程唯一；手动构造的公开 descriptor 仍需调用方区分像素版本。
- pipeline ticket 等待配置五秒超时，但 compiler Drop 会等待正在进行的驱动编译；构造函数不承诺五秒墙钟上限。rhapsody 底层行为保持原样。
- mapped buffer 写入必须在 `reset_completed` 成功之后；超时不解除外部纹理保持不变的要求。
- 命令提交后由 rhapsody 保留所需对象；atlas/target 仅在 GPU 完成及读回成功后发布。独立只读复审确认拆分后的命令顺序和资源保留有效。

## 验证

| 检查 | 结果 |
| --- | --- |
| 两包完整 feature / all-targets | 25 项 CPU 测试通过：selector 7、text 18；普通环境的 2 项 GPU 入口跳过，严格结果见下一行 |
| 严格 MoltenVK | 2/2 通过，11 帧：9 帧既有基准逐字节一致，2 帧文字颜色/选区；无 SKIP、validation error/warning、VUID 或 SYNC-HAZARD |
| Clippy | 两包 all-features/all-targets，`-D warnings` 通过 |
| 文档 | README doctest 1 项通过，`RUSTDOCFLAGS='-D warnings' cargo doc --all-features --no-deps` 通过 |
| 格式 | 两包 rustfmt check 通过 |
| 示例 | 冷暖帧像素一致，上传区域 24→0，传输 buffer 2→2 |

命令及结果见 [validation.json](validation.json)、[tests.log](tests.log)、[gpu.log](gpu.log)、[clippy.log](clippy.log)、[doctests.log](doctests.log)、[rustdoc.log](rustdoc.log) 与 [example.log](example.log)。示例保留 [实际输出预览](example-output/text.png) 和 [像素哈希/统计](example-output/text.json)，不保留原始 RGBA 大文件或机器相关管线缓存。

原生对话框本轮只有编译与状态测试，未进行窗口交互验收。性能改动以数据复制、分配及复用规则为依据，未进行吞吐量基准。Cargo 仍提示锁定依赖 `block 0.1.6` 的 future-incompat 信息；本轮没有升级依赖。

## 文档和来源

正式模块文档：[tg-gpu-selector](../../../crates/tg-gpu-selector/README.md)、[tg-text](../../../crates/tg-text/README.md)。初轮问题、迁移及验证记录见 [前次审查报告](../2026-10-18-gpu-text-refactor/README.md)。

本轮之前的源索引条目保存在 [source-manifest-before.json](source-manifest-before.json)，当前文件哈希更新至 [sources.json](../../../provenance/sources.json)。历史报告与初始导入记录保持不变。
