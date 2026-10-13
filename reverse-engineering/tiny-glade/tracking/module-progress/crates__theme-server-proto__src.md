# crates/theme-server-proto/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_project_protocol_contract_not_algorithm。

B协议ActiveEvent(start/end i64,cid String)、EventsResponse(events)字段契约；serde派生不算手写算法。

## 接线与验证

接线：shared_foundation_workspace_and_library_adapters_original_game_producer_consumer_pending。

共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。

## 已恢复的函数和类型范围

- B ActiveEvent start/end=i64,cid=String，EventsResponse events；派生serde复用
- 时间选择/cache等规则在theme-fetcher消费者，非协议包派生解码器

## 明确缺口

- 原服务兼容/版本契约；未证明独立手写迁移/校验
- 完整主题plugin消费与A/C适配

下一闭包：原服务兼容/版本契约；未证明独立手写迁移/校验；完整主题plugin消费与A/C适配

## 证据

- [evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json)
- [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)
- [evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json](../../evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/theme-server-proto/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/theme-server-proto/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-server-proto/src/lib.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | 全集/逐函数映射待核对 | candidate_present；B协议ActiveEvent(start/end i64,cid String)、EventsResponse(events)字段契约；serde派生不算手写算法。：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/theme-server-proto/src/lib.rs

[reconstruction/shared-foundation/crates/theme-server-proto/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-server-proto/src/lib.rs)

SHA256：`200fffb101dff95867b06f1b9b0fa8f80ec09e2cb9f6c46f17af57cb9944da1f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
