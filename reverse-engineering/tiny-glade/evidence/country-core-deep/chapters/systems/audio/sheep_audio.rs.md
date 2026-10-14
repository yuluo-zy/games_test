# systems/audio/sheep_audio.rs

> 文件作用：**绵羊反应音效（kind 0-7 + 参数 FirstPersonOff）。**

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::sheep_audio::sheep_audio_system | 0x1409d5ea0 | 49 | kind 表 0x142ac2cc8(0..7)：Aww/Think/So/Angry/Idea/Eyes/Confused→instantiate+set_parameter+start；kind4=帧尾 start/stop 分支；参数 FirstPersonOff | O | audio-misc\decomp\decomp-0x1409d5ea0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::sheep_audio::sheep_audio_init_system`（行 26）— 待推断
