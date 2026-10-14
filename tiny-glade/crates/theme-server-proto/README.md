# theme-server-proto

主题服务器活动列表的序列化数据合同。

## 功能与目录

src/lib.rs（ActiveEvent、EventsResponse）。

## 来源

- 历史恢复起点：[shared-foundation/theme-server-proto](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/theme-server-proto/)。当前源码在本目录，后续修正没有同步到历史副本。
- 原 B 包/PDB 归属：`crates/theme-server-proto/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 最初导入提交：`3e0387cd`；历史导入 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/REPORT.md)；[B-SURFACE.md](../../../reverse-engineering/tiny-glade/evidence/closures-2026-10-13/theme-fetcher/B-SURFACE.md)。

## 已完成

ActiveEvent 的 start/end:i64、cid:String 和 EventsResponse.events 字段已按 B producer 恢复；serde 派生用于编码/解码，不算手写协议算法。

2026-10-14 从 B 原字节重新核对 `start`、`end`、`cid`、`events` 字面量，以及 `ActiveEvent` 3 元素、`EventsResponse` 1 元素序列提示。原 visitor 接受对象与 serde 结构序列；所有已知字段必须存在，重复已知字段报错，未知字段递归跳过。时间接受完整 i64 范围，不在协议层检查开始/结束顺序；cid 是任意 String，不在协议层限制长度、格式或空值。events 保留输入顺序，空数组合法，缺失/null 均非法。5 条测试覆盖这些边界，继续复用 B 的 serde/serde_json。

2026-10-14 后续获授权向真实 events 接口请求一次，收到 HTTP 200、123 字节响应；保存的响应成功解码为 `EventsResponse`，包含 1 个 ActiveEvent。同一响应经 fetcher 离线复验通过，刷新间隔为 3600 秒。见 [验证报告](../../../reverse-engineering/tiny-glade/evidence/theme-live-once-2026-10-14/REPORT.md) 和 [原响应](../../../reverse-engineering/tiny-glade/evidence/theme-live-once-2026-10-14/events-response.json)。

## 尚差的功能与验收

B 发射客户端中的协议合同已完整盘点：PDB 专属模块只有两项自动生成的 expecting；public 7 个匹配中两项为 visitor，五项为 Drop 胶水，未发现独立手写迁移或验证方法。一次真实列表响应已验证兼容，但未覆盖服务端其他 API/错误 response、所有响应组合、作者完整源文本及 A/C，不能据此补造接口或宣称服务全量验收。

静态轮的原字节、PDB 与已恢复 visitor 分支见 [schema/REPORT.md](../../../reverse-engineering/tiny-glade/evidence/theme-static-2026-10-14/schema/REPORT.md)。该轮没有发送真实主题服务请求；上述单次请求是后续单独授权的验证。

在 workspace 根执行 `cargo test -p theme-server-proto --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。
