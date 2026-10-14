# systems/audio/wall_audio_utils.rs

> 文件作用：**墙体循环播放器（AudioLoopPlayer，10 循环的公共实现）。**

函数总计 3：业务 1 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play | 0x1409285a0 | 14 | +0x38=1(应播)；+0x28<+0x10 否则 panic；instances[current]→set_position→set_parameter(editing_speed)→set_volume | O | audio-misc\decomp\decomp-0x1409285a0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::resolve`（行 32）— 查询/计算
- `country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::new`（行 46）— 构造
