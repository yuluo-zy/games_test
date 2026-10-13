# crates/tg-gpu-selector/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B游戏私有设备选择/driver建议/graphics错误策略及Vulkan allowlist六项workaround状态，未重建枚举/驱动库。

## 接线与验证

接线：shared_foundation_workspace_and_library_adapters_original_game_producer_consumer_pending。

共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。

## 已恢复的函数和类型范围

- 选择Option索引、更多信息/取消/outcome及真实原iced窗口
- 原vendor表及条件建议/链接，classification由rhapsody边界输入
- raw图形错误族与建议override、本地化key恢复
- layer/vendor tag去留切换，重复项一起移除，Apply快照
- 原六bool措施映射、标签和Apply/Cancel结果

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| tg_gpu_selector::gpu_selection::Controls::update | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs)：`update` | B：原定位 0x1421bb740；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_gpu_selector::gpu_selection::Controls::view | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs)：`view` | B：原定位 0x1421bba80；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_gpu_selector::gpu_selection::device_driver_advice | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs)：`device_driver_advice` | B：原定位 0x1421be370；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_gpu_selector::gpu_selection::get_vendor_driver_update_links | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs)：`get_vendor_driver_update_links` | B：原定位 0x1421c36d0；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_gpu_selector::gpu_selection::localize_graphics_init_error | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs)：`localize_graphics_init_error` | B：原定位 0x1421c4410；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_gpu_selector::vulkan_config::Controls::new | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs)：`new` | B：原定位 0x140df35e0；stable first appearance vendor grouping with None group；tagged layer/vendor retain-toggle; matching duplicate entries all removed；Apply snapshots allowlist word and six bools; Cancel close command；exact six English workaround labels retained；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_gpu_selector::vulkan_config::toggle_allowlist_item | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs)：`toggle_allowlist_item` | B：原定位 0x140df3d10；stable first appearance vendor grouping with None group；tagged layer/vendor retain-toggle; matching duplicate entries all removed；Apply snapshots allowlist word and six bools; Cancel close command；exact six English workaround labels retained；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_gpu_selector::vulkan_config::update | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs)：`update` | B：原定位 0x140df3e70；stable first appearance vendor grouping with None group；tagged layer/vendor retain-toggle; matching duplicate entries all removed；Apply snapshots allowlist word and six bools; Cancel close command；exact six English workaround labels retained；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_gpu_selector::vulkan_config::view | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs)：`view` | B：原定位 0x140df4020；stable first appearance vendor grouping with None group；tagged layer/vendor retain-toggle; matching duplicate entries all removed；Apply snapshots allowlist word and six bools; Cancel close command；exact six English workaround labels retained；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |

## 明确缺口

- rhapsody真实AvailableGraphicsDevice/driver_support生产者
- opaque allowlist u32字段语义和实际device创建消费
- 原GUI像素与A/C适配

下一闭包：rhapsody真实AvailableGraphicsDevice/driver_support生产者；opaque allowlist u32字段语义和实际device创建消费；原GUI像素与A/C适配

## 证据

- [DEPENDENCY_FILTERING.md](../../DEPENDENCY_FILTERING.md)
- [evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json)
- [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)
- [evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json](../../evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/tg-gpu-selector/src/gpu_selection.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | tg_gpu_selector::gpu_selection::Controls::update（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_gpu_selector::gpu_selection::Controls::view（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_gpu_selector::gpu_selection::device_driver_advice（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_gpu_selector::gpu_selection::get_vendor_driver_update_links（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_gpu_selector::gpu_selection::localize_graphics_init_error（candidate_business_rule_mapping；业务等价以限定证据为准） | candidate_present；B游戏私有设备选择/driver建议/graphics错误策略及Vulkan allowlist六项workaround状态，未重建枚举/驱动库。：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_gpu_selector::gpu_selection::Controls::update：原定位 0x1421bb740；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_gpu_selector::gpu_selection::Controls::view：原定位 0x1421bba80；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_gpu_selector::gpu_selection::device_driver_advice：原定位 0x1421be370；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_gpu_selector::gpu_selection::get_vendor_driver_update_links：原定位 0x1421c36d0；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_gpu_selector::gpu_selection::localize_graphics_init_error：原定位 0x1421c4410；selection writes Option index; ShowMoreInfo sets true; URL/log external；B vendor table 0 Nvidia,1 Intel,2 Other,3 AMD,4 Qualcomm,5 Unknown；driver classification comes from rhapsody 0x140bda990; game formats suggestions/links；raw error tags and recommendation overrides preserved；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| `crates/tg-gpu-selector/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/lib.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | 全集/逐函数映射待核对 | candidate_present；B游戏私有设备选择/driver建议/graphics错误策略及Vulkan allowlist六项workaround状态，未重建枚举/驱动库。：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| `crates/tg-gpu-selector/src/vulkan_config.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | tg_gpu_selector::vulkan_config::Controls::new（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_gpu_selector::vulkan_config::toggle_allowlist_item（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_gpu_selector::vulkan_config::update（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_gpu_selector::vulkan_config::view（candidate_business_rule_mapping；业务等价以限定证据为准） | candidate_present；B游戏私有设备选择/driver建议/graphics错误策略及Vulkan allowlist六项workaround状态，未重建枚举/驱动库。：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_gpu_selector::vulkan_config::Controls::new：原定位 0x140df35e0；stable first appearance vendor grouping with None group；tagged layer/vendor retain-toggle; matching duplicate entries all removed；Apply snapshots allowlist word and six bools; Cancel close command；exact six English workaround labels retained；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_gpu_selector::vulkan_config::toggle_allowlist_item：原定位 0x140df3d10；stable first appearance vendor grouping with None group；tagged layer/vendor retain-toggle; matching duplicate entries all removed；Apply snapshots allowlist word and six bools; Cancel close command；exact six English workaround labels retained；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_gpu_selector::vulkan_config::update：原定位 0x140df3e70；stable first appearance vendor grouping with None group；tagged layer/vendor retain-toggle; matching duplicate entries all removed；Apply snapshots allowlist word and six bools; Cancel close command；exact six English workaround labels retained；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_gpu_selector::vulkan_config::view：原定位 0x140df4020；stable first appearance vendor grouping with None group；tagged layer/vendor retain-toggle; matching duplicate entries all removed；Apply snapshots allowlist word and six bools; Cancel close command；exact six English workaround labels retained；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/tg-gpu-selector/examples/tg_gpu_selector_native_probe.rs

[reconstruction/shared-foundation/crates/tg-gpu-selector/examples/tg_gpu_selector_native_probe.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/examples/tg_gpu_selector_native_probe.rs)

SHA256：`21cce796701d4a498b40bfd48438423ad4e7be2afc81363f9adc4571c940ff8b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `main` | 6 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-gpu-selector/src/fixtures/message_cases.rs

[reconstruction/shared-foundation/crates/tg-gpu-selector/src/fixtures/message_cases.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/fixtures/message_cases.rs)

SHA256：`77c8b23d45c43d049219cf1d8453df44cdef67983c96bcb276de3cd68664994e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs

[reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/gpu_selection.rs)

SHA256：`e860eca089ca48ff3662d0cd03f6f8823033e5b2d39cde81a426f59cc8d956a4`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `get_vendor_driver_update_links` | 17 | 未由扫描证明；以证据限定范围为准 |
| `device_driver_advice` | 54 | 未由扫描证明；以证据限定范围为准 |
| `localize_graphics_init_error` | 104 | 未由扫描证明；以证据限定范围为准 |
| `render_error_to_ui` | 137 | 未由扫描证明；以证据限定范围为准 |
| `key` | 167 | 未由扫描证明；以证据限定范围为准 |
| `new` | 187 | 未由扫描证明；以证据限定范围为准 |
| `with_reason` | 196 | 未由扫描证明；以证据限定范围为准 |
| `view` | 200 | 未由扫描证明；以证据限定范围为准 |
| `heading` | 203 | 未由扫描证明；以证据限定范围为准 |
| `update` | 210 | 未由扫描证明；以证据限定范围为准 |
| `program_outcome` | 226 | 未由扫描证明；以证据限定范围为准 |
| `view_native` | 233 | 未由扫描证明；以证据限定范围为准 |
| `update_native` | 314 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L10：Unknown,
- L35：Vendor::Other \| Vendor::Unknown => vec![],
### reconstruction/shared-foundation/crates/tg-gpu-selector/src/lib.rs

[reconstruction/shared-foundation/crates/tg-gpu-selector/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/lib.rs)

SHA256：`a4db367036959ed6d13dc45fb74f9e513d1d20c9e058740947c674039a357084`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `run_gpu_selection_gui` | 5 | 未由扫描证明；以证据限定范围为准 |
| `run_vulkan_configuration_gui` | 12 | 未由扫描证明；以证据限定范围为准 |
| `run_gpu_selection_gui_native` | 21 | 未由扫描证明；以证据限定范围为准 |
| `run_vulkan_configuration_gui_native` | 28 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs

[reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/vulkan_config.rs)

SHA256：`c8d47d94fa233915c9dae98fa31214cf4a63f88a1f0297583cd32e885833577d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `get` | 24 | 未由扫描证明；以证据限定范围为准 |
| `toggle` | 35 | 未由扫描证明；以证据限定范围为准 |
| `new` | 74 | 未由扫描证明；以证据限定范围为准 |
| `toggle_allowlist_item` | 92 | 未由扫描证明；以证据限定范围为准 |
| `allowlist_view` | 106 | 未由扫描证明；以证据限定范围为准 |
| `workarounds_view` | 109 | 未由扫描证明；以证据限定范围为准 |
| `view` | 129 | 未由扫描证明；以证据限定范围为准 |
| `update` | 136 | 未由扫描证明；以证据限定范围为准 |
| `program_outcome` | 146 | 未由扫描证明；以证据限定范围为准 |
| `view_native` | 153 | 未由扫描证明；以证据限定范围为准 |
