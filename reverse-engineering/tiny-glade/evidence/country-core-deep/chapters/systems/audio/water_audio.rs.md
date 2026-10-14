# systems/audio/water_audio.rs

> 文件作用：**水体绘制/擦除/结冰音效（4 循环 + mode）。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::water_audio::water_audio_system` | 0x140873630 | 0 | 按 WaterRaster 水位采样驱动音频混合（audio-misc 域细查） | O | terrain-water\decomp\decomp-0x140873630.txt
| `country_core::systems::audio::water_audio::water_audio_init_system` | 0x140873140 | 25 | 初始化 WaterAudioState 资源/音频源 | O | terrain-water\decomp\decomp-0x140873140.txt
