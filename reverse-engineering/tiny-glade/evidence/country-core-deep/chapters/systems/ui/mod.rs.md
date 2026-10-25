# systems/ui/mod.rs

> 文件作用：country-core UI 面：GlobalId（u64 name-hash）、UiSystems::add_ui 插件（Update 调度链 + UiState/UiNavState(1e9ns)/UiSignifierStream(0x170B) 资源 + PlayUiSound/PlayPhotomodeSound 事件注册）。

函数总计 4：业务 1 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::impl$0::add_ui` | 0x140945c60 | 38 | 插件注册（add_ui） | O（wp15 合同）：schedule UI 系统（chain_ignore_deferred + in_set_inner）、插 3 资源、注册 2 事件 | wp15/analysis/b4_startup/add_ui_0x140945c60.c |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ui::signifier_config_hash`（行 65）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`eq`、`hash` 等 2 项

## 实现方式（算法级）

- `add_ui` @0x140945c60（O，wp15 合同）：插件注册主体——`chain_ignore_deferred` + `in_set_inner` 把 UI 系统组链进 Update 调度，插入 3 资源（UiState / UiNavState / UiSignifierStream），注册 2 事件（PlayUiSound / PlayPhotomodeSound）。这是整个 systems/ui 域被调度的唯一入口。
- `signifier_config_hash`（行 65，无独立符号，U）：调用点推断为把 signifier 配置对象散列成 u64，与 `WidgetId::from_name` = `utils::calculate_hash` 的 name-hash 同族；实现体被内联，无独立反编译符号。
- 派生胶水 eq/hash（O）：derive 生成，不计独立业务。
- 处理链（I 组合）：add_ui 注册 → Update 帧循环依次跑 hover_intersection → ui_update → signifier_*（create/animation/mark_for_delete）——各章见对应节。
