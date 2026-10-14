# systems/save/mod.rs

> 文件作用：save 系统族入口：add_save_load_systems / add_save_load_events_and_resources 注册存档系统与事件。

函数总计 5：业务 2 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::impl$1::add_save_load_systems`` | 0x1409443b0 | 94 | 构造/注册/生成 | O：add_save_load_systems = 注册 save_game/load_game/save_thread/check_load_completion 等 | round3\decomp-0x1409443b0.txt  |
| ``country_core::systems::save::impl$1::add_save_load_events_and_resources`` | 0x1409447e0 | 114 | 构造/注册/生成 | O：add_save_load_events_and_resources = 注册事件/资源（SaveGameCmd/SavingGameComplete/LoadingGameComplete 等） | session-save\decomp\decomp-0x1409447e0.txt  |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`deserialize`、`serialize` 等 3 项
