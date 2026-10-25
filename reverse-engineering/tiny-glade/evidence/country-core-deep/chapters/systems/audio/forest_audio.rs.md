# systems/audio/forest_audio.rs

> 文件作用：**森林生长音效（1 个循环）。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::forest_audio::forest_audio_init_system | 0x140aa18f0 | 32 | 1×instantiate_loop(event:/SFX/ForestGrow) | O | audio-misc\decomp\decomp-0x140aa18f0.txt |
| country_core::systems::audio::forest_audio::forest_audio_system | 0x140aa1f00 | 65 | 森林生长事件驱动循环 start/stop/set_position | O | audio-misc\decomp\decomp-0x140aa1f00.txt |

## 实现方式（算法级）（O，decomp-0x140aa18f0/0x140aa1f00）

**forest_audio_init_system @0x140aa18f0**：1×`instantiate_loop`（`event:/SFX/ForestGrow`）。
**forest_audio_system @0x140aa1f00**：森林生长事件 → 单个循环 player 的
start_if_not_playing / stop / set_position 驱动（生长位置跟随）。单循环模块，无 kind 分派。O
