# crates/tg-integrity-report/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B报告三列表保持顺序，all-empty归一，单文件删除与状态/建议/UI联接。

## 接线与验证

接线：shared_foundation_workspace_and_library_adapters_original_game_producer_consumer_pending。

共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。

## 已恢复的函数和类型范围

- 三列表均空才归一成功，保留输入顺序
- missing/corrupt/extra映射到状态与本地化section，真实窗口适配
- 固定Steam完整性建议链接与对应文案key
- 单次unlink；成功只删同名extra，失败保留且不重新校验
- update/outcome与crash内嵌报告联接，真实原iced独立窗口

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| tg_integrity_report::controls::Controls::new | [reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs)：`new` | B：原定位 0x140dbaea0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_integrity_report::controls::remove_unrecognized_package_file | [reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs)：`remove_unrecognized_package_file` | B：原定位 0x140dbaed0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_integrity_report::controls::normalize_verification_result | [reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs)：`normalize_verification_result` | B：原定位 0x140dbaed0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_integrity_report::controls::verification_status | [reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs)：`verification_status` | B：原定位 0x140dbb0d0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_integrity_report::controls::advice | [reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs)：`advice` | B：原定位 0x140dbb0d0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_integrity_report::controls::update | [reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs)：`update` | B：原定位 0x140dbe760；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |

## 明确缺口

- 原verify入口至实际启动报告消费者
- 完整原GUI样式/平台失败边界与A/C适配

下一闭包：原verify入口至实际启动报告消费者；完整原GUI样式/平台失败边界与A/C适配

## 证据

- [evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json)
- [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)
- [evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json](../../evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/tg-integrity-report/src/controls.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | tg_integrity_report::controls::Controls::new（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_integrity_report::controls::remove_unrecognized_package_file（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_integrity_report::controls::normalize_verification_result（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_integrity_report::controls::verification_status（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_integrity_report::controls::advice（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_integrity_report::controls::update（candidate_business_rule_mapping；业务等价以限定证据为准） | candidate_present；B报告三列表保持顺序，all-empty归一，单文件删除与状态/建议/UI联接。：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_integrity_report::controls::Controls::new：原定位 0x140dbaea0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_integrity_report::controls::remove_unrecognized_package_file：原定位 0x140dbaed0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_integrity_report::controls::normalize_verification_result：原定位 0x140dbaed0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_integrity_report::controls::verification_status：原定位 0x140dbb0d0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_integrity_report::controls::advice：原定位 0x140dbb0d0；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_integrity_report::controls::update：原定位 0x140dbe760；single unlink, failures ignored；successful unlink retains nonmatching extra names；normalize only all three lists empty; no reverify; preserve input order；fixed Steam integrity advice link and localized section keys；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| `crates/tg-integrity-report/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-integrity-report/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/lib.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | 全集/逐函数映射待核对 | candidate_present；B报告三列表保持顺序，all-empty归一，单文件删除与状态/建议/UI联接。：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/tg-integrity-report/examples/tg_integrity_report_native_probe.rs

[reconstruction/shared-foundation/crates/tg-integrity-report/examples/tg_integrity_report_native_probe.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/examples/tg_integrity_report_native_probe.rs)

SHA256：`d151e249c4ca8bbe1f127e0bb1df80c7dc56a82ed89d1d945bea53c9d4829667`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `main` | 6 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs

[reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/controls.rs)

SHA256：`ed0b9eaf7318f9d1cf6fb0bec41d0aa46edff4c97fe3839cce1617cc8329edd6`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `remove_file` | 32 | 未由扫描证明；以证据限定范围为准 |
| `new` | 35 | 未由扫描证明；以证据限定范围为准 |
| `normalize_verification_result` | 38 | 未由扫描证明；以证据限定范围为准 |
| `remove_unrecognized_package_file` | 46 | 未由扫描证明；以证据限定范围为准 |
| `verification_status` | 58 | 未由扫描证明；以证据限定范围为准 |
| `advice` | 91 | 未由扫描证明；以证据限定范围为准 |
| `view` | 98 | 未由扫描证明；以证据限定范围为准 |
| `update_with_remover` | 101 | 未由扫描证明；以证据限定范围为准 |
| `update` | 117 | 未由扫描证明；以证据限定范围为准 |
| `program_outcome` | 124 | 未由扫描证明；以证据限定范围为准 |
| `new` | 134 | 未由扫描证明；以证据限定范围为准 |
| `remove_file` | 141 | 未由扫描证明；以证据限定范围为准 |
| `native_element` | 165 | 未由扫描证明；以证据限定范围为准 |
| `update` | 229 | 未由扫描证明；以证据限定范围为准 |
| `program_outcome` | 233 | 未由扫描证明；以证据限定范围为准 |
| `view_native` | 239 | 未由扫描证明；以证据限定范围为准 |
| `update_native` | 242 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-integrity-report/src/lib.rs

[reconstruction/shared-foundation/crates/tg-integrity-report/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/lib.rs)

SHA256：`3c70cbc88400ad25131962cb66e472b7100581b252921e547bf79e500b53ace1`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `run_integrity_report_gui` | 5 | 未由扫描证明；以证据限定范围为准 |
| `run_integrity_report_gui_native` | 14 | 未由扫描证明；以证据限定范围为准 |
