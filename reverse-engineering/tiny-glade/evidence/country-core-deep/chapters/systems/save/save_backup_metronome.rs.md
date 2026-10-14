# systems/save/save_backup_metronome.rs

> 文件作用：备份节拍器（全内联文件，无独立符号）：累积活动游戏时间，达阈值触发备份。O 数值（内联于 save_game 0x140a3f470 行 507-591）：{counter u32@+0x140, 累计 f32@+0x144 clamp[0,1], 阈值 f32@+0x148}；仅阈值<60.0 时累计；档位表 {300,600,3600,7200}s（5min/10min/1h/2h）@0x142a9f348；断言 active_game_delta_seconds >= 0.0。

函数总计 4：业务 0 / 内联无独立符号 3 / 派生胶水 1

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::save::save_backup_metronome::SaveBackupMetronome::add_active_game_time`（行 18）— 构造/注册/生成
- `country_core::systems::save::save_backup_metronome::SaveBackupMetronome::consider_backing_up`（行 23）— 待推断
- `country_core::systems::save::save_backup_metronome::SaveBackupMetronome::new_from_backup_list`（行 77）— 构造

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
