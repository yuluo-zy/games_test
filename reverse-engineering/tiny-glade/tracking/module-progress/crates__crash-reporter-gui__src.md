# crates/crash-reporter-gui/src

> 2026-10-14 用户明确排除：已从 tiny-glade 当前 workspace 删除，不需要继续逆向还原。以下为历史记录。



状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B诊断优先级、前8帧匹配、复制提示、一次callback/thread与Tick完成联接，内嵌完整性报告。

## 接线与验证

接线：shared_foundation_workspace_and_library_adapters_original_game_producer_consumer_pending。

共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。

## 已恢复的函数和类型范围

- 显卡driver帧/日志、前8帧OBS/FPS、Vulkan loader与integrity优先级
- more_info是建议URL；ShowDetails独立切换
- 复制传入诊断文本；成功提示严格<1秒，错误持续显示
- Sending不重复回调；Tick检查is_finished后join，状态/outcome与原iced联接
- 一次callback创建真实worker；内嵌integrity删除→状态链测试

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| crash_reporter_gui::Controls::new | [reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs)：`new` | B：原定位 0x14061f640；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| crash_reporter_gui::Controls::update | [reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs)：`update` | B：原定位 0x140620470；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| crash_reporter_gui::Controls::view | [reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs)：`view` | B：原定位 0x1406213c0；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| crash_reporter_gui::Controls::main_view | [reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs)：`main_view` | B：原定位 0x140623370；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| crash_reporter_gui::Controls::more_info_button | [reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs)：`more_info_button` | B：原定位 0x14062e080；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| crash_reporter_gui::program_outcome | [reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs)：`program_outcome` | B：原定位 0x14061f610；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| crash_reporter_gui::tick_message | [reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs)：`tick_message` | B：原定位 0x14061f620；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |

## 明确缺口

- 原panic报告producer/callback payload ABI与发送入口
- 完整原GUI样式/clipboard OS实效/真实崩溃端到端
- A/C未适配

下一闭包：原panic报告producer/callback payload ABI与发送入口；完整原GUI样式/clipboard OS实效/真实崩溃端到端；A/C未适配

## 证据

- [evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json)
- [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)
- [evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json](../../evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/crash-reporter-gui/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | crash_reporter_gui::Controls::new（candidate_business_rule_mapping；业务等价以限定证据为准）<br>crash_reporter_gui::Controls::update（candidate_business_rule_mapping；业务等价以限定证据为准）<br>crash_reporter_gui::Controls::view（candidate_business_rule_mapping；业务等价以限定证据为准）<br>crash_reporter_gui::Controls::main_view（candidate_business_rule_mapping；业务等价以限定证据为准）<br>crash_reporter_gui::Controls::more_info_button（candidate_business_rule_mapping；业务等价以限定证据为准）<br>crash_reporter_gui::program_outcome（candidate_business_rule_mapping；业务等价以限定证据为准）<br>crash_reporter_gui::tick_message（candidate_business_rule_mapping；业务等价以限定证据为准） | candidate_present；B诊断优先级、前8帧匹配、复制提示、一次callback/thread与Tick完成联接，内嵌完整性报告。：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>crash_reporter_gui::Controls::new：原定位 0x14061f640；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>crash_reporter_gui::Controls::update：原定位 0x140620470；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>crash_reporter_gui::Controls::view：原定位 0x1406213c0；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>crash_reporter_gui::Controls::main_view：原定位 0x140623370；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>crash_reporter_gui::Controls::more_info_button：原定位 0x14062e080；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>crash_reporter_gui::program_outcome：原定位 0x14061f610；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>crash_reporter_gui::tick_message：原定位 0x14061f620；11 graphics-driver frame patterns and device-lost log strings；VulkanOn12/gpu-hint priorities; first8 FPS monitor/OBS frame window; first frame Vulkan loader; integrity fallback；callback consumed once to create JoinHandle, tick checks is_finished then joins；clipboard uses supplied message String; success label strict <1sec, error persistent；more_info_button is advisory URL link, ShowDetails toggles separately；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/crash-reporter-gui/examples/crash_reporter_gui_native_probe.rs

[reconstruction/shared-foundation/crates/crash-reporter-gui/examples/crash_reporter_gui_native_probe.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/examples/crash_reporter_gui_native_probe.rs)

SHA256：`28abea07c1e20fac5093e138eacc102bc5e23425d1e8b7da7e84daa5c87105cf`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `main` | 3 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs

[reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/lib.rs)

SHA256：`3b2ad8448f33afcbaafd872bec155126839f83f3e5e4ad4aa2fc1c151972d6ac`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `diagnose` | 30 | 未由扫描证明；以证据限定范围为准 |
| `set_text` | 116 | 未由扫描证明；以证据限定范围为准 |
| `new` | 127 | 未由扫描证明；以证据限定范围为准 |
| `with_integrity_report` | 142 | 未由扫描证明；以证据限定范围为准 |
| `update_with_remover` | 149 | 未由扫描证明；以证据限定范围为准 |
| `copy_to_clipboard` | 165 | 未由扫描证明；以证据限定范围为准 |
| `more_info_button` | 175 | 未由扫描证明；以证据限定范围为准 |
| `clipboard_label` | 178 | 未由扫描证明；以证据限定范围为准 |
| `main_view` | 187 | 未由扫描证明；以证据限定范围为准 |
| `view` | 210 | 未由扫描证明；以证据限定范围为准 |
| `update_with_clipboard` | 234 | 未由扫描证明；以证据限定范围为准 |
| `can_change_advice` | 246 | 未由扫描证明；以证据限定范围为准 |
| `update` | 256 | 未由扫描证明；以证据限定范围为准 |
| `program_outcome` | 299 | 未由扫描证明；以证据限定范围为准 |
| `tick_message` | 302 | 未由扫描证明；以证据限定范围为准 |
| `run_crash_reporter_gui` | 306 | 未由扫描证明；以证据限定范围为准 |
| `set_text` | 323 | 未由扫描证明；以证据限定范围为准 |
| `update` | 333 | 未由扫描证明；以证据限定范围为准 |
| `program_outcome` | 336 | 未由扫描证明；以证据限定范围为准 |
| `tick_message` | 339 | 未由扫描证明；以证据限定范围为准 |
| `view_native` | 345 | 未由扫描证明；以证据限定范围为准 |
| `update_native` | 471 | 未由扫描证明；以证据限定范围为准 |
| `run_crash_reporter_gui_native` | 500 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L29：/// original_hint preserves the observed tri-state byte; its semantic producer is unknown.
