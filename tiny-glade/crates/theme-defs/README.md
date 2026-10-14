# theme-defs

主题获取器与游戏之间的活动事件传输合同，承载数据目录、开始/结束时间和错误状态。

## 功能与目录

src/lib.rs（ActiveEvent、EventDataError、is_active_interval）。

## 来源

- 历史恢复起点：[shared-foundation/theme-defs](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/theme-defs/)。当前源码在本目录，后续修正没有同步到历史副本。
- 原 B 包/PDB 归属：`crates/theme-defs/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 最初导入提交：`3e0387cd`；历史导入 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[FOUNDATION_BUSINESS_COMPARISON.md](../../../reverse-engineering/tiny-glade/FOUNDATION_BUSINESS_COMPARISON.md)；[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/REPORT.md)。

## 已完成

活动数据字段、错误家族和 start<=now<end 半开区间规则已恢复；字段/错误名称为语义重建名。Clone/Eq/thiserror 由依赖派生。

2026-10-14 静态复核了 B 的全部相关 PDB IPI/TPI 与 public 库边界：已发射的本库合同为 `ActiveEvent`（目录及两个 `SystemTime`）和 `EventDataError`（两个单位状态及一个 owned String 状态）。所有 16 个含本库类型的 public 符号都是 std/ArcSwap 容器胶水；没有定位到独立手写业务函数。完整字段和错误结果参与发布去重，两个时间保留 `SystemTime` 的完整精度。新增测试覆盖目录、开始时间、结束时间和错误文本分别变化。

## 尚差的功能与验收

作者的精确字段/变体名和两个单位错误的 Display 文案未由 line-tables-only PDB 或已发射代码提供；当前可编译名称/文案仍是标注的重建接口。未定位到本库 serde 序列化实例，因此不增造错误 wire 格式。下载后的 `langs.json`、`meta.json`、`history.json`、`credits.json` 由游戏 consumer 消费，完整内容接线和 A/C 验收不属于这两个传输结构的恢复范围。

静态轮的原字节、PDB 和测试范围见 [schema/REPORT.md](../../../reverse-engineering/tiny-glade/evidence/theme-static-2026-10-14/schema/REPORT.md)。该轮没有发送真实主题服务请求。

2026-10-14 后续获授权向活动列表接口请求一次，HTTP 200，123 字节响应解析出 1 个活动；同一响应的协议与 fetcher 离线复验通过。此验证覆盖列表获取和解析，没有下载主题包，也没有验收本库到游戏 consumer 的接线。见 [真实请求报告](../../../reverse-engineering/tiny-glade/evidence/theme-live-once-2026-10-14/REPORT.md)。

在 workspace 根执行 `cargo test -p theme-defs --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。
