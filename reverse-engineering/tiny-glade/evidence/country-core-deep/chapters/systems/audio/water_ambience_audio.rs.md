# systems/audio/water_ambience_audio.rs

> 文件作用：**水面环境音：按水网格逐格装配的 Amb 循环播放器。**

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::water_ambience_audio::impl$0::load_index_unchecked` | 0x1409ebe80 | 37 | 网格单元装载（音频网格索引访问） | O | terrain-water\decomp\decomp-0x1409ebe80.txt
| `country_core::systems::audio::water_ambience_audio::water_ambience_audio_init_system` | 0x1409ebf20 | 68 | 初始化 WaterAmbienceAudio/网格资源 | O | terrain-water\decomp\decomp-0x1409ebf20.txt
| `country_core::systems::audio::water_ambience_audio::water_ambience_audio_system` | 0x1409ecea0 | 107 | 消费 WaterAudioGrid 入水计数驱动水面环境音 | O | terrain-water\decomp\decomp-0x1409ecea0.txt

## 实现方式（算法级）

`water_ambience_audio_system`（0x1409ecea0）：消费 WaterAudioGrid 入水计数（24B 单元/64 位滚动窗/6400.0² 阈值/0.5 水位阈值 prior O）驱动逐格 Amb 循环播放器；`impl$0::load_index_unchecked`（0x1409ebe80）网格单元装载；init 0x1409ebf20。
