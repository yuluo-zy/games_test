# systems/save/load_game.rs

> 文件作用：读档主系统：LoadGameManager 状态机（LoadGameState::LoadGameData）→ 写 loading.tmp → NextSessionKind 解码（1=parse_saved_game_data 含 upgrade 备份、2=on_load 命令回放+内存快照）→ PostGameLoadCommand 回放（SwitchAppMode 忽略）→ 旧版颜色 ID 升级（{1→2,2→5,4→8,5→3,8→4}）→ extract_max_wall_id_from_history。

函数总计 14：业务 2 / 内联无独立符号 5 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::load_game::load_game`` | 0x14146d030 | 0 | IO/解析 | O：主系统（13+ 段 SystemParam）：状态机+loading.tmp+kind 解码+命令回放+升级+wall id 恢复（详见 DIGEST §3） | session-save\decomp\decomp-0x14146d030.txt  |
| ``country_core::systems::save::load_game::parse_saved_game_data`` | 0x141470d00 | 413 | IO/解析 | O：parse_saved_game_data（内联于 load_game）：Making save file backup for upgrade→Reading file data→Parsing JSON→Version→BackwardCompatibility→Camera→parsing camera→Time of day（串块 0x142c64450） | session-save\decomp\decomp-0x14146d030.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::save::load_game::LoadGameManager::new`（行 78）— 构造
- `country_core::systems::save::load_game::apply_parsed_game_data`（行 744）— 待推断
- `country_core::systems::save::load_game::apply_backward_compatibility_overrides`（行 870）— 待推断
- `country_core::systems::save::load_game::upgrade_window_color`（行 880）— 待推断
- `country_core::systems::save::load_game::extract_max_wall_id_from_history`（行 899）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 7 项
