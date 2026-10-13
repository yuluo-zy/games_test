# crates/tg-iced/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B手写对话框选项/结果与文本入口；真实原iced/winit/cosmic-text后端可运行，窗口接线为重建适配。

## 接线与验证

接线：shared_foundation_workspace_and_library_adapters_original_game_producer_consumer_pending。

共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。

## 已恢复的函数和类型范围

- 手写level/按钮模式、标题/说明替换和关闭结果；原叶对照，原iced窗口适配
- Controls update与ProgramWithOutcome闭合到原iced窗口关闭
- 真实Application、命令/结果/Tick联接；16ms频率为重建选择
- 文本key交给lang-lib原本地化策略，standalone只读加载locale
- B原iced/winit/cosmic-text fork后端；窗口探针可运行

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| tg_iced::dialog::MessageDialog::new | [reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs)：`new` | B：原定位 0x140631ce0；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价 | scoped_business_comparison：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_iced::dialog::MessageDialog::set_level | [reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs)：`set_level` | B：原定位 0x140631d20；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价 | scoped_business_comparison：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_iced::dialog::MessageDialog::set_title | [reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs)：`set_title` | B：原定位 0x140631d70；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_iced::dialog::MessageDialog::set_description | [reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs)：`set_description` | B：原定位 0x140631ea0；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| tg_iced::dialog::Controls::view | [reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs)：`view` | B：原定位 0x1406331a0；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |

## 明确缺口

- 原GUI默认尺寸/调度频率/完整选项与精确widget样式
- 真实游戏启动消费者；A/C未适配

下一闭包：原GUI默认尺寸/调度频率/完整选项与精确widget样式；真实游戏启动消费者；A/C未适配

## 证据

- [evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json)
- [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)
- [evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json](../../evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/tg-iced/src/dialog.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | tg_iced::dialog::MessageDialog::new（scoped_business_comparison；业务等价以限定证据为准）<br>tg_iced::dialog::MessageDialog::set_level（scoped_business_comparison；业务等价以限定证据为准）<br>tg_iced::dialog::MessageDialog::set_title（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_iced::dialog::MessageDialog::set_description（candidate_business_rule_mapping；业务等价以限定证据为准）<br>tg_iced::dialog::Controls::view（candidate_business_rule_mapping；业务等价以限定证据为准） | candidate_present；B手写对话框选项/结果与文本入口；真实原iced/winit/cosmic-text后端可运行，窗口接线为重建适配。：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_iced::dialog::MessageDialog::new：原定位 0x140631ce0；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_iced::dialog::MessageDialog::set_level：原定位 0x140631d20；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_iced::dialog::MessageDialog::set_title：原定位 0x140631d70；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_iced::dialog::MessageDialog::set_description：原定位 0x140631ea0；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json)<br>tg_iced::dialog::Controls::view：原定位 0x1406331a0；empty title/description, caller supplied u32 dimensions, level0/buttons0；fluent string replacement；three button modes Ok/OkCancel/Continue and icons sheep/warning/cross；静态/叶/候选范围分开，不含完整原GUI等价：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| `crates/tg-iced/src/gui_runner.rs` | A,B | [reconstruction/shared-foundation/crates/tg-iced/src/gui_runner.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/gui_runner.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | 全集/逐函数映射待核对 | candidate_present；B手写对话框选项/结果与文本入口；真实原iced/winit/cosmic-text后端可运行，窗口接线为重建适配。：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |
| `crates/tg-iced/src/gui_runner_sdl.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/tg-iced/src/gui_runner_winit.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/tg-iced/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-iced/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/lib.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | 全集/逐函数映射待核对 | candidate_present；B手写对话框选项/结果与文本入口；真实原iced/winit/cosmic-text后端可运行，窗口接线为重建适配。：[evidence/foundation-unstarted-2026-10-13/ui/manifest.json](../../evidence/foundation-unstarted-2026-10-13/ui/manifest.json), [evidence/foundation-unstarted-2026-10-13/ui/validation.json](../../evidence/foundation-unstarted-2026-10-13/ui/validation.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/tg-iced/examples/tg_iced_native_probe.rs

[reconstruction/shared-foundation/crates/tg-iced/examples/tg_iced_native_probe.rs](../../reconstruction/shared-foundation/crates/tg-iced/examples/tg_iced_native_probe.rs)

SHA256：`8914fc0f243e5261542ed99f7cb191da879577a2448ab74aab36d3e1717ac365`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `main` | 5 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs

[reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/dialog.rs)

SHA256：`0eca51dbf7544abb4184638f1fe0afe730bc6a36788e5f58de14957925585465`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `icon` | 10 | 未由扫描证明；以证据限定范围为准 |
| `new` | 34 | 未由扫描证明；以证据限定范围为准 |
| `set_level` | 44 | 未由扫描证明；以证据限定范围为准 |
| `set_title` | 48 | 未由扫描证明；以证据限定范围为准 |
| `set_description` | 52 | 未由扫描证明；以证据限定范围为准 |
| `set_buttons` | 56 | 未由扫描证明；以证据限定范围为准 |
| `show` | 60 | 未由扫描证明；以证据限定范围为准 |
| `new` | 83 | 未由扫描证明；以证据限定范围为准 |
| `view` | 89 | 未由扫描证明；以证据限定范围为准 |
| `update` | 109 | 未由扫描证明；以证据限定范围为准 |
| `program_outcome` | 113 | 未由扫描证明；以证据限定范围为准 |
| `view_native` | 120 | 未由扫描证明；以证据限定范围为准 |
| `show_native` | 153 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-iced/src/fixtures/dialog_cases.rs

[reconstruction/shared-foundation/crates/tg-iced/src/fixtures/dialog_cases.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/fixtures/dialog_cases.rs)

SHA256：`2ff9bc9d8d39d58b946ddf8395845b0532f1dfb9bd9082d4dd85f03865a95e52`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/shared-foundation/crates/tg-iced/src/gui_runner.rs

[reconstruction/shared-foundation/crates/tg-iced/src/gui_runner.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/gui_runner.rs)

SHA256：`08a53c11e629aab7100cb4946ac8c7c2b1649dc2d5712f244a0b88796c1f92c3`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 10 | 未由扫描证明；以证据限定范围为准 |
| `update` | 30 | 未由扫描证明；以证据限定范围为准 |
| `program_outcome` | 31 | 未由扫描证明；以证据限定范围为准 |
| `tick_message` | 32 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 43 | 未由扫描证明；以证据限定范围为准 |
| `run` | 50 | 未由扫描证明；以证据限定范围为准 |
| `run_gui_with_options` | 52 | 未由扫描证明；以证据限定范围为准 |
| `run_gui` | 60 | 未由扫描证明；以证据限定范围为准 |
| `view_native` | 81 | 未由扫描证明；以证据限定范围为准 |
| `update_native` | 82 | 未由扫描证明；以证据限定范围为准 |
| `platform_command` | 92 | 未由扫描证明；以证据限定范围为准 |
| `new` | 121 | 未由扫描证明；以证据限定范围为准 |
| `title` | 130 | 未由扫描证明；以证据限定范围为准 |
| `update` | 133 | 未由扫描证明；以证据限定范围为准 |
| `view` | 148 | 未由扫描证明；以证据限定范围为准 |
| `subscription` | 155 | 未由扫描证明；以证据限定范围为准 |
| `run` | 168 | 未由扫描证明；以证据限定范围为准 |
| `run_native_gui` | 187 | 未由扫描证明；以证据限定范围为准 |
| `update` | 213 | 未由扫描证明；以证据限定范围为准 |
| `program_outcome` | 216 | 未由扫描证明；以证据限定范围为准 |
| `tick_message` | 219 | 未由扫描证明；以证据限定范围为准 |
| `view_native` | 228 | 未由扫描证明；以证据限定范围为准 |
| `update_native` | 231 | 未由扫描证明；以证据限定范围为准 |
| `capture_native_gui` | 264 | 未由扫描证明；以证据限定范围为准 |
| `write_native_capture` | 289 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-iced/src/lib.rs

[reconstruction/shared-foundation/crates/tg-iced/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-iced/src/lib.rs)

SHA256：`3d03da8562e44405335438f3e00555c078dc15286e51b5ceaefb48b0ac10f91d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `text` | 5 | 未由扫描证明；以证据限定范围为准 |
| `load_gui_locale` | 14 | 未由扫描证明；以证据限定范围为准 |
