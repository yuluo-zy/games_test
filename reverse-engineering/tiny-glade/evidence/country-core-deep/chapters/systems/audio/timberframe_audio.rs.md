# systems/audio/timberframe_audio.rs

> 文件作用：**木骨架音效（PlayTimberframeSound）。**

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::timberframe_audio::timberframe_audio | 0x140a67b90 | 0 | 构造 TimberframeAudioState（瞬发+循环混合） | I | audio-misc\decomp\decomp-0x140a67b90.txt |
| country_core::systems::audio::timberframe_audio::timberframe_audio_init_system | 0x140a67810 | 30 | kind→TimberframeMove/Length/Width（+RectangleAppear 等）→instantiate+set_parameter(editing_speed) | O | audio-misc\decomp\decomp-0x140a67810.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
