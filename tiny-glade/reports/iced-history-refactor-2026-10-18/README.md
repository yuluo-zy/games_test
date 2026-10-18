# tg-iced 与 tg-history 源码审查及重构

2026-10-18，基于当前工作区源码完成格式访问与 GUI 生命周期重构。仅审查 Rust 源码，未使用逆向工具、修改锁定依赖或启动真实窗口。以现有未提交改动为基线；历史重建副本不随本轮更新。

## 确认问题与修复

| 问题 | 影响与处理 |
| --- | --- |
| `summarize_json` 隐式解码屋顶字段 | 结构解析和摘要能够接受的文档，汇总入口却可能拒绝。现在汇总只统计结构，已知屋顶字段通过显式、惰性的类型解码报告错误。 |
| 解析与统计耦合，查询复制全部 JSON 载荷 | 拆分一次性结构校验与按需统计；命令、条目和快照均使用借用迭代器，单个快照查询直接扫描，不构造全量副本。屋顶类型直接从 `&Value` 解码。 |
| 未知标签路径未转义 | 标签包含 `/` 或 `~` 时，摘要路径不能正确定位。现在输出 RFC 6901 转义路径，并保留结构化错误路径和 JSON 底层错误。 |
| 无业务结果的关闭请求未形成 runner 终态 | iced 同一批消息可能在关闭命令之后继续执行。新增 `NativeAction::Close` 和关闭状态，立即停止后续业务更新及 tick。 |
| 截图超时没有独立终态 | 迟到的截图可能把超时改为成功。现在从首次 tick 计时，200 毫秒请求一次，达到 5 秒返回 `CaptureTimedOut`，拒绝迟到或重复结果。 |

## 接口与兼容

- `tg-history` 按文档访问、屋顶类型、统计和错误拆分模块。遍历 API 返回借用迭代器，`summary()` 直接返回统计，快照选择直接返回 `Option`；类型解码保留逐项 `Result`。迁移示例见 [tg-history README](../../crates/tg-history/README.md)。
- 屋顶 owned Serde 类型、字段名、枚举标签和存档格式保持不变；未知 JSON、大整数、命令顺序及游标后的条目保留。最近快照仍由调用方筛选，距离相同时选择较大索引。
- `NativeProgram::update_native` 与 `platform_command` 返回 `NativeAction`，同步迁移最新 GPU 原生适配；`ProgramWithOutcome` 保持原接口。用法和错误语义见 [tg-iced README](../../crates/tg-iced/README.md)。
- 保留既有截图文件输出的 `create_new` 和 RAII 回滚；没有增加通用持久化抽象、游戏撤销重做或世界回放。

## 验证范围

新增回归围绕结构边界、延迟解码一致性、借用引用身份、未知标签路径、快照筛选与整数极值，以及关闭消息序列和截图时间边界。截图状态测试使用显式时间，不进行真实等待。

| 检查 | 结果 |
| --- | --- |
| `tg-history` 默认 all-targets | 9 项单元测试通过 |
| `tg-iced` 默认 all-targets | 5 项单元测试通过 |
| `tg-iced` 与 GPU 调用方 native-gui all-targets | 13 + 7 项单元测试通过，两份原生示例编译通过 |
| README rustdoc | `tg-history` 1 项、`tg-iced` 原生配置 2 项通过 |
| 三包原生配置 all-targets 严格 Clippy | 通过；保留上游 `block 0.1.6` 的 future-incompat 提示 |
| 两包格式及本轮补丁空白检查 | 通过 |
| 独立源码终审 | 无阻断问题 |

在工作区运行主要集成检查：

```sh
cargo clippy --offline --locked -p tg-history -p tg-iced -p tg-gpu-selector --all-targets --features tg-iced/native-gui,tg-gpu-selector/native-gui --no-deps -- -D warnings
cargo check --offline --locked -p country-core -p system-roof -p tiny-glade
cargo fmt -p tg-history -p tg-iced --check
```

已验证受影响的 `country-core`、`system-roof`、`tiny-glade` 依赖编译成功；这些范围外包仍有既有编译警告。本轮未将局部验证扩展为全库零警告、真实 GUI 视觉或游戏运行时验收。

## 二次源码复审

2026-10-18，重新读取两个包的全部实现、README、测试和直接调用方，并交叉更换审查分工。未发现新增的确定性 Bug，本轮未修改业务源码。

- 重新执行三包原生配置的 all-targets 测试，29 项单元测试全部通过；三包严格 Clippy 再次通过。
- 复核私有 JSON 原文与一次校验的不变量、借用生命周期、错误后的继续迭代、未知标签路径、快照筛选和平局规则，以及有符号索引极值。
- 历史包独立复测 9 项单元测试及 1 项文档测试通过；4 项临时探针验证从 `&Value` 解码的嵌套扩展、大整数与原文保留，以及 country-core 屋顶字段、方向与未解释尾字节的兼容。
- 对照锁定 iced 的源码确认：同一批消息在 close 命令后仍会继续派发，当前关闭状态能够阻止后续业务 update 和 tick。
- 通过临时独立验证程序限制单个文件大小，分别制造像素部分写失败、像素写完后元数据部分写失败；操作返回 `FileTooLarge`，两种情况下本次新建的像素与元数据文件均被回滚。

截图超时依赖事件循环获得执行机会，无法抢占阻塞的视图回调或底层同步截图操作。真实窗口、字体/DPI 和 Windows 行为仍未进行运行验证。

范围外消费者还有一项既有输入边界：`country-core/src/resources/roofs/geometry/replay.rs` 的 `VisualReplayState::from_observed` 接受原始字节，若将第 20 字节设为非法布尔编码（例如 2），`advance_direction` 会在 `expect` 处 panic。该状态不由 `tg-history` 的 JSON 接口构造，未发现当前消费链输入此类状态；本轮仅记录，未扩大修改范围。

## 写法与注释标准核对

按用户明确的六项工程标准继续清理。此前的行为测试通过不能替代此项核对，公开接口说明和历史接口残留确有补充工作。

| 标准 | 当前落实与保留项 |
| --- | --- |
| 命名 | 使用条目、命令、快照和截图生命周期等业务名称；保留存档字段名，不对未知风格数据或墙引用编码作额外推断。 |
| 数据表示 | 两个目标包的业务流程无裸内存偏移；复用 Serde、屋顶类型和 JSON 借用视图。未知字段保留为 JSON。 |
| 结构 | 分离文档校验、类型投影、统计、原生执行、截图和文件提交；移除没有额外语义的 `run_gui_with_options` 转发入口。 |
| 错误与所有权 | 保留统一错误传播和写入回滚；生产 `expect` 由私有不可变文档的一次校验、本地单标签检查或整数范围保证支撑。保留拥有值的结果观察协议，见下述成本说明。 |
| 注释 | 补齐公开用途、输入约束、错误和借用/拥有生命周期；内部解释锁与静态视图、校验不变量及关闭顺序。历史样本与来源集中到本文，包 README 只说明当前使用合同。 |
| 一致性 | 使用 derive、借用迭代器、Result、RAII 和 feature 边界；未新增依赖、动态分派或复杂泛型抽象。 |

删除了两个不属于当前业务接口的重建遗留：`HistorySummary.replay_verified` 恒为 false，现不再混入统计类型或 `summarize_json` 的输出；`Command::RemovePackageFile` 没有生产消费者或实现，现从通用 GUI 命令中移除。前者改变统计输出字段，不改变存档 JSON；后者与删除转发入口均已检查活动源码调用方。原生失败测试改用已有的未适配 `OpenPath` 命令。

仍保留一项有成本的旧接口：`ProgramWithOutcome::program_outcome()` 返回拥有值，原生 runner 在形成终态附近及最终返回时可能重复复制配置结果。为遵守已确认的接口保留方案，本轮没有改成借用查询和一次性移动结果，不能宣称 GUI 全面零复制。截图自身的 clone 只共享 Arc 像素存储。`Arc<Mutex<_>>` 用于 iced 消耗应用后回收业务状态；视图必须拥有文本，因为渲染时不能继续持有该互斥锁的借用。这些保留属于明确的所有权与兼容选择，不是模拟原生内存布局。

此次清理后重新通过 29 项联合单元测试、3 项文档测试、三包原生配置严格 Clippy、两包格式检查及 rustdoc 链接检查（断开的内部链接视为错误）。没有修改业务依赖或锁文件。

后续 [B 版覆盖对照](../iced-history-b-comparison-2026-10-18/README.md) 确认原 B 确有 `run_gui_with_options` 及相关闭包。本轮删除的只是当前重建代码中的纯转发层，不表示原选项、默认配置和回调业务已经完整合并。本文的源码质量与测试结论不应解读为 B 功能完整性验收。

### 历史来源归档

以下材料从包 README 移入此处，作为既有格式来源，不代表本轮新增逆向或运行时验收：B 的 33 个历史文档包含 2,120 个条目、2,774 个外层命令；三版本静态格式检查覆盖 98 个文档和 733 个屋顶命令。

- [历史格式观察](../../../reverse-engineering/tiny-glade/reconstruction/history/findings.md)
- [三版本格式检查](../../../reverse-engineering/tiny-glade/evidence/multi-version/history-rust-verification.json)
- [既有实现记录](../../../reverse-engineering/tiny-glade/evidence/tg-history-completion-2026-10-14/REPORT.md)
