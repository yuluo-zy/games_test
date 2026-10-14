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
