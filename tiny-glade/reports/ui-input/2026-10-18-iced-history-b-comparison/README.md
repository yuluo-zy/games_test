# tg-iced / tg-history 与 B 版目录及功能覆盖对照

目标仅为 B。复用已有 PDB 源目录、原指令记录和当前源码进行静态覆盖审计，未新增反编译、原码差分或运行测试，也未补写业务实现。本次使用 `tiny-glade-recovery` 的审查分类规则；结论不是原 B 行为等价验收。

## 结论与比较基线

存在数据类型投影、原运行规则和消费链缺口。此前源码重构与单元测试解决的是当前实现的质量，不能据此宣布 B 版功能完整。

- 原 B 的 `tg-iced/src` 有 `lib.rs`、`dialog.rs`、`gui_runner.rs` 三个源目录项。
- 原 B 没有独立 `tg-history` crate。对应原域是 `country-core/src/resources/history` 的 12 个目录项，以及 `country-core/src/systems/history` 的 15 个目录项。当前 `tg-history` 是额外拆出的只读 JSON 格式访问层，运行时历史主要仍在 `country-core`。
- `skeleton/B` 保存 PDB 元数据和已有取证注解，不是取回的作者 Rust 源文。具名实体、源码路径、函数实例、内联、derive、依赖代码分别判断；不能按文件数或符号数计算业务完成比例。
- 同名函数存在不证明等价；同名文件不存在也不证明功能丢失。本次对已合并和迁移的实现检查了实际位置。

本次读取文件的快照见 [source-snapshot.json](source-snapshot.json)。
原历史域的逐文件去向见 [directory-map.md](directory-map.md)，包括全部 12 个资源文件和 15 个系统文件；该表记录实现去向和剩余边界，不给出未核实的完成百分比。

## tg-iced 对照

| B 原内容 | 当前对应 | 判定 |
| --- | --- | --- |
| `dialog.rs` 的 `MessageDialog`、`Controls`、`Message` | `dialog.rs` 与 `dialog/native.rs` | 主要逻辑结构存在，配置、确认/取消及终态有实现。原完整字段布局和 Rust ABI 未由元数据恢复，不能宣称完整结构等价。 |
| `new`、标题/说明/等级 setter、Controls 构造/update/outcome | 同名方法或状态实现 | 有对应逻辑。已有历史限定叶函数证据不覆盖当前重构后的完整执行链。 |
| `view`、`show` | 借用视图、原生布局、注入后端和 `show_native` | 功能拆分，不能按原文件行数判缺失；原字体、样式、布局、像素结果仍未完整映射。 |
| `ProgramWithOutcome` | 当前同名 trait | 协议存在，但目前的关联类型和消息/tick API 是候选工程接口，不是已证实的原 Rust 签名。 |
| `run_gui` 与真实的 `run_gui_with_options`、相关闭包 | `run_gui` + `run_native_gui` + `NativeWindow` | 有可运行的后端链；原完整运行选项、默认配置、回调、事件批处理和错误策略仍缺完整对应。当前 `Options` 只有标题/宽/高，16 ms tick 是重建选择。 |
| `gui_runner::impl$0::default` | 当前要求显式构造 Options | 原默认入口存在，但所属类型、字段和值的合同未完整确定。属于需补证的默认配置，不得猜测 `Default`。 |
| `tg_iced::text` | 当前 `text` 查询翻译，`native_text` 构造 widget | 原入口有 `Text::new` 与 `Text::shaping` 的直接内联证据，原本具有 widget 包装行为。当前是语义拆分，不能把同名 lookup 当作原函数完整等价。具体字体/样式/shaping 参数未完整核实。 |
| iced/winit 的绘制、布局、事件循环等依赖实现 | 锁定的 iced fork 及其依赖 | 直接复用，不重复恢复，不计自有业务缺失。C 的 SDL/winit 分文件不属于 B 缺口。 |
| 原完整性/崩溃报告窗口及文件删除消费者 | 当前项目按用户要求排除相关包 | 属明确缩减范围，不列为待恢复缺口。不能从当前 GUI 命令枚举中没有删除命令推断原 B 没有文件删除业务。 |
| 截图包装、文件输出、NativeAction/GuiBackend/GuiError | 当前新增工程接口 | 有实现，但不是据此证明恢复了 B 的同名原结构或业务。 |

范围内上层接线仍有差异：`tiny-glade/src/app_config.rs` 的默认 `NoopCliErrorDialogBackend` 丢弃已构造对话框；`vulkan_config_main.rs` 仍使用 `VulkanConfigGui` 注入面，未直接接到现有 GPU 原生 wrapper，接受结果后的配置持久化也未在该入口完成。这些属于调用方接线，不能记成 tg-iced 缺少基础对话框类型。

### 对上一轮删除项的准确解释

删除当前重建版的 `run_gui_with_options` 是合并一个没有额外语义的转发层；**原 B 确实存在这个函数及闭包**，删除转发层不证明其原选项与回调业务已完整合并。`Command::RemovePackageFile` 未见原 B tg-iced 类型合同证据，它是旧重建适配器；其真实文件删除消费者属于已批准排除的报告功能。两者都必须按功能和归属判断，不能仅凭名字保留或删除。

## tg-history 与原 country-core 历史域对照

| 内容 | 当前对应 | 判定 |
| --- | --- | --- |
| B `EditType` 的完整外层分类 | tg-history 的 `HistoryCommandKind` 识别样本中的 13 类 | B 运行时已有证据为 15 类；当前分类缺 `Cobblestone3`、`Decorator`、`AutoClutter`，会返回 Unknown。13 类中另含旧文档的 `WallNewSystem`，所以不能简单做 15−13。原始 JSON 仍保留，不是载荷丢失。 |
| 屋顶命令 | tg-history 的四个 owned RoofCommand 变体 | B 原生 tag 还有第 5 种（tag 4，flat rim style change）。本包没有它的 typed 投影；现有证据未确定其 JSON 标签名，不能猜一个 wire 名称。当前 core 的 `RoofHistoryEdit::rim` 和回放事件已有实现，不是运行时屋顶风格功能整体缺失。 |
| 非屋顶载荷、decorators/stairs 附属记录 | 本包借用 JSON；core 有 Color/Tree/Decorator/Stairs/Raster/AutoClutter 等类型与实现 | 本包缺完整 typed 投影，但许多数据结构在 core 已存在。不能把本包返回 Value 误报为全项目没有对应类型。 |
| 墙历史子编辑 | core `wall_history_edit.rs` / `wall_history_replay.rs` | 17 个子编辑中，索引 6..16 仍以 `UnresolvedWallEditPayload` 承载；索引 5 MorphPremadeShape 已有类型，但相关回放仍返回未恢复操作。不能将具备枚举名等同于具备完整载荷和行为。 |
| 装饰差分地址键、颜色目标的楼梯类型 | core `decorator_edit.rs` / `color_edit.rs` | 装饰地址 wire 键仍用明确标注的占位编码；StairRailingType/StairSupportType 缺原类型合同，以不透明 u8 暂存。属于尚未完整恢复的数据表示。 |
| History、HistoryEntry、EditType、HistoryIdentity、录制/分支截断/undo/redo | core `resources/history/mod.rs`、`recovered_building/history_transactions.rs` 等 | 已有实际类型和函数；原 `history/mod.rs` 的多项业务由这些模块承接，不应重复放进 tg-history。全链等价仍未验收。 |
| 当前/最近快照、插入快照和保存 | 本包提供引用查询；core `history_snapshots.rs`、save/snapshot/save_game 等提供运行时逻辑 | 不是全部缺失。原运行时 resolved 谓词仍有证据解释冲突，见下节；本包 caller predicate 不能代替真实运行时解析与所有权状态。 |
| 原多种光栅回放源文件 | core `systems/history/replay_raster.rs` 与 `systems/terrain/replay_terrain.rs` | Path/Water/Garden/Cobblestone 等有合并实现。原同名文件没有逐个保留不等于功能丢失。 |
| 统一 Terrain 编辑→回放桶入口 | `ReplayBucket::expand_with_aux` | 当前遇 `EditType::Terrain` 返回 `UnrestoredReplayOwner`。另有 TerrainReplayCmd 直入桶路径，因此是统一入口未闭合，不是完全没有地形回放。 |
| 序列化 History → 运行时完整编辑链 | save/load_game 的 HistorySectionJson、ParsedGameData 与独立运行时 owner | 已有 JSON 读档、快照解析和保存；但 edits/history_start/snapshots 仍有 Value 层，完整转换与运行时 owner 组装未见统一闭环，不能把读 JSON 当作完整历史恢复。 |
| 原 `signifier_replay_active` 与相关指示器组件 | 当前活动源码未找到对应或替代映射 | 当前缺实现映射。B PDB 有实际系统符号，现有 core inventory 也标记 `no_formal_mapping`；不能把它误归为依赖或仅 Bevy 注册胶水。具体动画数值仍须补证。 |
| 完整 `replay_pop_bucket` 的 aux 恢复/碰撞刷新 | 函数体存在；当前 CPU schedule 注册 `pop_empty_replay_bucket` | 属函数已有、生产接线未闭合：完整函数当前只找到测试调用。不能把仅弹空桶当作完整 aux 恢复链。 |
| `history_process` 第二控制事件流 | 当前 `process_history_control_stream` | 已恢复 case 0..3；其余原分支明确未解码并走当前默认路径，不属于完整原控制流实现。 |

### 当前需要补证的快照解释冲突

旧 `tg-history-continuation-2026-10-14/REPORT.md` 把 `0x140a84260` 解释为 Arc strong count ≥ 2 时成功；当前 core 的 `HistoryChainSnapshotPayload` 把同一位置解释成解析状态且 `< 2` 时 resolved。两套解释不一致。

现有原 RNE 汇编显示：比较读值与 2，低于 2 跳过载荷写入；大于等于 2 时写 `out+8`、`out+16` 并返回 tag 0，否则返回 tag 1。成功/失败对应的完整返回布局及被读对象类型须重新锚定；仅凭分支或名称不能确认“引用计数”或“状态枚举”。本次保留冲突，不改代码，也不宣称该原规则已经完整恢复。

## 依据与下一步边界

- [B tg-iced 原目录](../../../../reverse-engineering/tiny-glade/skeleton/B/crates/tg-iced/src/gui_runner.rs)：`run_gui_with_options` 元数据在第 2067 行，原 text 的 widget 内联在同目录 `lib.rs` 第 679–708 行。
- [已有 B UI manifest](../../../../reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/ui/manifest.json)：`observed/0` 的对话框规则，`unknown/0` 的原选项/默认值/调度缺口及原像素边界；旧验证不能直接验收当前改版源码。
- [B 屋顶命令元数据](../../../../reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/resources/history/roof_edit.rs)：第 2458 行列出 0..4 tag，第 4 项 serialized_name 仍 unknown。
- [当前 core EditType](../../../crates/country-core/src/resources/history/mod.rs)、[统一回放桶](../../../crates/country-core/src/resources/history/replay_cmd_buffer.rs)、[当前历史调度](../../../crates/country-core/src/systems/history.rs)、[读档](../../../crates/country-core/src/systems/save/load_game.rs)。
- [B 指示器元数据](../../../../reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/history/signifier_replay_active.rs) 与 core `recovery/inventory.json` 中同一 `original_path` 的空 formal_candidates；宏注册与真实业务分开判断。
- [快照旧解释](../../../../reverse-engineering/tiny-glade/evidence/tg-history-continuation-2026-10-14/REPORT.md)、[原 RNE 汇编](../../../../reverse-engineering/tiny-glade/evidence/tg-history-continuation-2026-10-14/tryfrom-original-rne.asm)、[当前 core 快照规则](../../crates/country-core/src/recovered_building/history_snapshots.rs)。
- [已批准排除范围](../../../README.md)：报告/云平台四包不重新计入缺口。

可直接确认的类型覆盖缺口是三个外层分类及屋顶第五命令投影；未知 wire 名和原 runner 默认/样式合同需要补证。统一历史导入、Terrain 标准扩展、完整 pop 消费及回放指示器属于 core 实现/接线问题，不能靠扩大 tg-history 的解析 API 假装补齐。此次仅交付对照审计，不新增原版恢复进度。
