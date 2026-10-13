# crates/geforcenow/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B固定DLL/三导出全部加载后initialize(0)，成功query/shutdown/unload；失败不吞为false。

## 接线与验证

接线：shared_foundation_workspace_and_library_adapters_original_game_producer_consumer_pending。

共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。

## 已恢复的函数和类型范围

- 固定DLL与三导出、init非零原码传播；成功query/shutdown/卸载
- 仅库入口已恢复；哪些实际启动/路径/配置消费云判断仍待联接

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| geforcenow::is_running_in_cloud | [reconstruction/shared-foundation/crates/geforcenow/src/lib.rs](../../reconstruction/shared-foundation/crates/geforcenow/src/lib.rs)：`is_running_in_cloud` | B：原0x14061e660入口18例注入库/SDK边界；本地DLL14组，真实SDK未执行 | scoped_business_comparison：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json) |

## 明确缺口

- 真实GFN SDK未加载/云服务未运行
- 实际游戏启动/路径/配置消费者
- 原错误枚举标签/Display与A/C适配

下一闭包：真实GFN SDK未加载/云服务未运行；实际游戏启动/路径/配置消费者；原错误枚举标签/Display与A/C适配

## 证据

- [evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json)
- [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json)
- [evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json](../../evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/geforcenow/src/lib.rs` | B,C | [reconstruction/shared-foundation/crates/geforcenow/src/lib.rs](../../reconstruction/shared-foundation/crates/geforcenow/src/lib.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | geforcenow::is_running_in_cloud（scoped_business_comparison；业务等价以限定证据为准） | candidate_present；B固定DLL/三导出全部加载后initialize(0)，成功query/shutdown/unload；失败不吞为false。：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json)<br>geforcenow::is_running_in_cloud：原0x14061e660入口18例注入库/SDK边界；本地DLL14组，真实SDK未执行：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/geforcenow/src/lib.rs

[reconstruction/shared-foundation/crates/geforcenow/src/lib.rs](../../reconstruction/shared-foundation/crates/geforcenow/src/lib.rs)

SHA256：`46d37685ce21b8e74d1eeb9364864ff41a1cc9148d8a4a79b391c611e0a44889`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `fmt` | 17 | 未由扫描证明；以证据限定范围为准 |
| `initialize` | 36 | 未由扫描证明；以证据限定范围为准 |
| `is_running_in_cloud` | 37 | 未由扫描证明；以证据限定范围为准 |
| `shutdown` | 38 | 未由扫描证明；以证据限定范围为准 |
| `probe_runtime` | 44 | 未由扫描证明；以证据限定范围为准 |
| `is_running_in_cloud` | 56 | 未由扫描证明；以证据限定范围为准 |
| `probe_runtime_library` | 66 | 未由扫描证明；以证据限定范围为准 |
| `initialize` | 83 | 未由扫描证明；以证据限定范围为准 |
| `is_running_in_cloud` | 87 | 未由扫描证明；以证据限定范围为准 |
| `shutdown` | 91 | 未由扫描证明；以证据限定范围为准 |
