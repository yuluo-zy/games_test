# crates/country-core/src/input

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_source_candidate; ownership_requires_dependency_filter。

仅有源码清单或候选交付；完整模块行为尚未评估

## 接线与验证

接线：not_established。

未建立模块/文件级验收；候选交付不等于原作等价

## 已恢复的函数和类型范围

原函数全集尚未核对。下面的Rust定义清单仅登记实现符号。

## 明确缺口

- 原函数全集、类型/ABI、生产者/消费者和版本适配尚未核对

下一闭包：先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向

## 证据

尚未登记模块级证明。

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/input/action_set.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/input/input_stream.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/input/mod.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/input/panic_repro_state.rs` | A,B,C | [skeleton/task-index/work/TG-867ec0f6d3ac488689/abc-tools-52/crates/country-core/src/input/panic_repro_state.rs](../../skeleton/task-index/work/TG-867ec0f6d3ac488689/abc-tools-52/crates/country-core/src/input/panic_repro_state.rs)：explicit_candidate_delivery_mapping | country_core::input::panic_repro_state::PanicReproState::is_inactive（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### skeleton/task-index/work/TG-867ec0f6d3ac488689/abc-tools-52/crates/country-core/src/input/panic_repro_state.rs

[skeleton/task-index/work/TG-867ec0f6d3ac488689/abc-tools-52/crates/country-core/src/input/panic_repro_state.rs](../../skeleton/task-index/work/TG-867ec0f6d3ac488689/abc-tools-52/crates/country-core/src/input/panic_repro_state.rs)

SHA256：`df197e6f3225c828399a003f036f0d4e3a0545d6fe3addfcc68cf5cb1b508a6e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_inactive` | 14 | 未由扫描证明；以证据限定范围为准 |
