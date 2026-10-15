# systems/color 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/color` |
| package | `system-color` |
| 规模 | 17 文件 / 31 本 ns 函数 / 6 闭包 / **21 业务函数** |
| 难度 | 中 |
| 批次 | 2 |
| 上游依赖 | `country-core`、`egui`、`rhapsody`、`glam`、`utils` |
| 复现工程现状 | MODULE_PROGRESS：**未逐项评估**，无本机源码；仅索引 |

## 任务表（按 B src 层级）

### 颜色 ID 写入（状态 → GPU / 权威）

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-COLOR-01 | `src/set_wall_color_id.rs` | 恢复 `set_wall_color_id`、`init_color_ids_buffer`：墙颜色 ID 写入。 | core 墙权威 | 无 | 原指令样例 + 缓冲区布局一致 | 未开始 |
| UM-COLOR-02 | `src/write_roof_color_id_to_gpu.rs` | 恢复 `write_roof_color_id_to_gpu`：屋顶颜色 ID 上传 GPU。 | rhapsody | 无 | 原入口对照；GPU 后置登记 | 未开始 |
| UM-COLOR-03 | `src/set_decorator_color_id.rs` | 恢复 `set_decorator_color_id`。 | decorator | 无 | 原指令样例 | 未开始 |
| UM-COLOR-04 | `src/set_gate_color_id.rs` | 恢复 `set_gate_color_id`。 | 门几何 | 无 | 原指令样例 | 未开始 |
| UM-COLOR-05 | `src/set_halftimber_color_id.rs` | 恢复 `set_halftimber_color_id`。 | 半木结构 | 无 | 原指令样例 | 未开始 |
| UM-COLOR-06 | `src/set_flag_connection_n_chimney_smoke.rs` | 恢复 `set_flag_connection_and_chimney_smoke`：旗帜连接与烟囱烟雾的颜色。 | decorator | 无 | 原指令样例 | 未开始 |
| UM-COLOR-07 | `src/set_window_light.rs` | 恢复 `set_window_light`：窗光颜色/状态。 | decorator/时间 | 无 | 原指令样例 | 未开始 |
| UM-COLOR-08 | `src/set_roof_customization.rs` | 恢复 `set_roof_customization`：屋顶自定义（颜色/材质）。 | 屋顶 | 无 | 原指令样例 | 未开始 |

### 菜单 UI

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-COLOR-09 | `src/radial_menu/mod.rs` | 恢复 `ColorRadialMenu::create`、`IconInfo::new_flat_roof_rim_ty`、`IconInfo::new_separator`：径向颜色菜单。 | UI 框架 | 无 | 原指令样例 | 未开始 |
| UM-COLOR-10 | `src/color_menu.rs` | 恢复 `color_menu`、`color_button_animations`、`delete_color_icon_entities`。 | 径向菜单 | 无 | 原指令样例 | 未开始 |
| UM-COLOR-11 | `src/show_color_menu.rs` | 恢复 `show_color_options`、`hide_color_menu`。 | 菜单状态 | 无 | 原指令样例 | 未开始 |
| UM-COLOR-12 | `src/autoclose_color_menu.rs` | 恢复 `autoclose_color_menu`。 | 输入层 | 无 | 原指令样例 | 未开始 |
| UM-COLOR-13 | `src/ui_color_wall_and_roof.rs` | 恢复 `ui_color_wall_and_roof`、`color_cursor_icon`。 | UI/egui | 无 | 原指令样例 | 未开始 |
| UM-COLOR-14 | `src/ui_color_gates.rs` | 恢复 `ui_color_gates`。 | UI/egui | 无 | 原指令样例 | 未开始 |
| UM-COLOR-15 | `src/ui_color_decorator.rs` | 恢复 `ui_color_windows_and_flags`。 | UI/egui | 无 | 原指令样例 | 未开始 |

### 回放 / 插件

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-COLOR-16 | `src/replay_color.rs` | 恢复 `replay_color`：颜色编辑回放。 | core 历史回放 | 无 | 原入口对照 | 未开始 |
| UM-COLOR-17 | `src/lib.rs` | 恢复插件注册 `add_color`。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |

## 卡点

- 颜色 ID 缓冲区布局（wall/roof/decorator/gate）与 GPU 写入格式需原侧证据。
- UI 部分依赖 `egui`（原 fork），UI 框架接线属批次 3/4 范围；本批次先恢复颜色逻辑与状态。

## 验收标准

- 每函数 ≥ 原指令样例；颜色 ID 映射表与缓冲区布局有原侧证据；回放与 core 历史 schema 一致。
