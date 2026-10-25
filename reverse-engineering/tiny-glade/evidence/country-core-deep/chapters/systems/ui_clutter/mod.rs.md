# systems/ui_clutter/mod.rs

> 文件作用：**待推断**（由章节 agent 按函数名+证据填写）

函数总计 1：业务 0 / 内联无独立符号 0 / 派生胶水 1

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项

## 实现方式（算法级）

- 本文件无独立业务函数（仅 1 项 default 派生胶水，O）：`mod.rs` 为 ui_clutter 子模块的聚合容器。
- 聚合 6 个 clutter 系统（I，由子模块名 + DIGEST 串 "systems/ui_clutter/rbm_autoclutter.rs" 推断）：debug_spawn_clutter、decoclutter_show_pins、defered_autoclutter_history、rmb_autoclutter、ui_autoclutter、ui_focus_decoclutter——由 add_ui 一并挂入 UI 调度（O：add_ui 链见 systems/ui/mod.rs 章）。
- 处理链（I）：autoclutter 生成/移除（rmb/ui_autoclutter）→ 历史记录（defered_autoclutter_history）→ 图钉显示（decoclutter_show_pins）→ 焦点移动/旋转（ui_focus_decoclutter）。
