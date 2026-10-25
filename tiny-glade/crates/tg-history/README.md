# tg-history

`tg-history` 是 Tiny Glade 历史 JSON 的格式访问层，负责解析文档、分类命令、读取条目附属数据、投影屋顶命令，以及查询游标和快照引用。它不执行游戏世界中的撤销、重做或回放；这些业务属于 `country-core`。

## 当前业务与数据流

输入 JSON → 一次结构校验 → 保留私有、不可变的原始 JSON → 借用命令、快照和统计查询。结构校验检查 `Version`、有符号 32 位 `History.chain_index`、`edit_chain`、单标签命令和快照引用；屋顶字段的类型解码是独立的按需操作。

外层命令分为 `WallNewSystem`、`Wall`、`Roof`、`DecoratorV5`、`Terrain`、`Tree`、`Color`、`Stairs`、`Path`、`Water`、`Garden`、`Cobblestone1` 和 `Cobblestone2`。未知标签和各命令载荷均保留为 JSON。条目的 `decorators`、`stairs` 附属记录单独暴露，不当作 `edits` 中的命令。墙的分组编辑和各版本差异不在本包中推断或迁移。

屋顶只投影四种已支持命令：创建、删除、外观前后值编辑、屋脊方向切换。`Roof*` 类型保留现有 Serde 字段名和 `serde(flatten)` 扩展字段。`wall_index` 是交由调用方解释的序列化墙引用；本包不进行运行时实体转换。屋顶投影拥有自己的字段，可以在原文档释放后继续使用。

上述分类和投影并未覆盖 B 的完整历史类型：`Cobblestone3`、`Decorator`、`AutoClutter` 当前归为未知外层标签，屋顶 tag 4 尚无本包类型投影。其 JSON 值仍保留。原历史运行时位于 `country-core`，详细去向及未完成部分见 [B 版覆盖对照](../../reports/ui-input/2026-10-18-iced-history-b-comparison/README.md)。

`raw()` 和 `to_json()` 保留 JSON 值、未知数据和 `u64` ID；不保证空白、键顺序或数字文本写法原样不变。屋顶的 `f32` 投影不会覆盖原始 JSON 数值，也不添加范围钳制或默认迁移规则。

## 游标和快照

所有命令均按存储顺序访问，包括游标之后的条目。统计会给出游标之后的条目数量，不删掉这些条目，也不把它们解释成当前世界状态。`history_start` 作为可选原始 JSON 返回。

快照引用包含 `i32 chain_index` 和 `u64 snapshot_id`。最近快照查询先应用调用方提供的“已解析”谓词，再按索引距离选择；距离相同时选择较大索引。当前快照查询仅返回索引恰好等于文档游标的引用。本包不打开快照文件，也不通过文件是否存在推断运行时解析状态。

## API 与使用方式

| 接口 | 行为 |
| --- | --- |
| `HistoryDocument::parse(bytes)` | 返回 `Result`；校验必需元数据、单标签命令（含 `Roof` 的内层标签）和快照字段，不检查已知屋顶载荷的类型字段。 |
| `commands()` | 迭代全部外层命令，借用标签、分类和 JSON 载荷，并保留条目/命令索引。 |
| `history_entries()` / `entry.commands()` | 借用条目、附属记录和该条目的命令，按需遍历。 |
| `roof_commands()` | 惰性返回逐项 `Result<LocatedRoofCommand, DecodeError>`；仅解码四种已知屋顶命令，未知变体跳过并仍可从原文及统计访问。 |
| `snapshot_references()` | 借用快照原始值，迭代已校验的索引和 ID。 |
| `closest_resolved_snapshot(...)` / `current_snapshot_reference()` | 返回 `Option` 借用视图，无需复制全部快照或分配候选列表。 |
| `summary()` | 直接返回 `HistorySummary`；统计结构标签，不触发屋顶类型解码。 |
| `summarize_json(bytes)` | 返回序列化统计的 `Result`，与 `parse(...).summary()` 的结构接纳范围一致。 |

`parse` 返回拥有 JSON 的文档，不借用输入字节。视图和迭代器的生命周期受 `HistoryDocument` 约束，条目的命令迭代器可以比临时条目视图活得更久，但仍不能超过文档。命令、条目和快照访问不深度复制 JSON；屋顶投影从 `&Value` 直接反序列化为原有 owned 类型，只有返回对象自身需要的扩展数据才会拥有副本。`summary()` 返回独立拥有的统计，仅为不同标签计数和未知标签路径分配空间。可选附属记录和 `history_start` 区分缺失字段与显式 JSON `null`。

```rust
use tg_history::{DecodeError, HistoryCommandKind, HistoryDocument};

let document = HistoryDocument::parse(br#"{
    "Version": 27,
    "History": {
        "chain_index": -1,
        "edit_chain": [{"edits": [
            {"Roof": {"SwitchRoofRidgeDir": {
                "wall_index": 18446744073709551615,
                "from": "Width", "to": "Length"
            }}},
            {"Future": {"opaque": true}}
        ]}],
        "snapshots": []
    }
}"#)?;

let commands: Vec<_> = document.commands().collect();
assert_eq!(commands[0].kind, HistoryCommandKind::Roof);
assert_eq!(commands[1].payload["opaque"], true);
for entry in document.history_entries() {
    assert_eq!(entry.commands().len(), 2);
}

// 收集到 Result 时，任一已知屋顶载荷的类型错误会使收集失败。
let roofs = document.roof_commands().collect::<Result<Vec<_>, _>>()?;
assert_eq!(roofs.len(), 1);
let summary = document.summary();
assert_eq!(summary.command_counts["Roof"], 1);
assert_eq!(summary.entries_after_chain_index, 1);
assert_eq!(summary.entries, 1);
# Ok::<(), DecodeError>(())
```

`DecodeError::path()` 返回 RFC 6901 JSON Pointer；语法错误的路径为空，表示整个文档。`kind()` 区分结构预期错误和 JSON 编解码错误，后者通过标准 `Error::source()` 保留 `serde_json::Error`。屋顶类型错误定位到对应 `/Roof` 载荷，原始错误说明缺失字段或错误类型。未知命令路径转义 `/` 和 `~`，可交给 `raw().pointer(...)` 定位。

旧调用方迁移：`commands()?`、`history_entries()?`、`snapshot_references()?` 改为直接遍历，需要列表时显式 `.collect::<Vec<_>>()`；条目的 `commands` 字段改为 `commands()`。`roof_commands()?` 改为 `.roof_commands().collect::<Result<Vec<_>, _>>()?`，也可逐项处理错误；`summary()?` 改为 `summary()`，快照选择器直接处理 `Option`。`HistoryCommandKind`、命令和快照引用类型增加借用生命周期，`Roof*` 的字段、Serde 格式及已有消费者无需迁移。

统计类型和 `summarize_json` 输出已删除旧的恒假字段 `replay_verified`；消费统计的代码应删除对该字段的读取，回放验证由实际执行回放的调用方负责。这项变更只影响统计输出，历史存档 JSON 的数据合同保持不变。

## 职责边界

本包只访问历史文档的序列化数据，不执行游戏世界编辑或回放。快照引用不意味着对应资产存在或能够加载；调用方负责解析外部资源和验证运行时状态。版本字段和原始载荷按存储值保留，不推断不同命令家族之间的对应关系，也不执行版本迁移。

[源码审查、接口迁移与验证报告](../../reports/ui-input/2026-10-18-iced-history-refactor/README.md)

## 验证

从 `games_test/tiny-glade` 工作区运行：

```sh
cargo test -p tg-history --all-targets --offline --locked
cargo test -p tg-history --doc --offline --locked
cargo clippy -p tg-history --all-targets --offline --locked -- -D warnings
cargo fmt -p tg-history -- --check
```

测试覆盖未知字段和大整数往返、13 类命令识别、屋顶四种投影、条目附属记录、结构/整数边界、惰性类型错误及错误来源、借用身份和路径转义、快照筛选/等距/重复索引/有符号极值；README 示例作为 rustdoc 测试执行。
