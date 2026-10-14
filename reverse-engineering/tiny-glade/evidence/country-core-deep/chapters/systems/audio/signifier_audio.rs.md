# systems/audio/signifier_audio.rs

> 文件作用：**UI 符号流音效（1 循环 + 4 瞬发 + 21 参数更新）。**

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::signifier_audio::signifier_audio_system | 0x1409d6b90 | 0 | UiSignifierStream→set_parameter×21（含 FirstPersonOn/Off、Tool 参数） | O | audio-misc\decomp\decomp-0x1409d6b90.txt |
| country_core::systems::audio::signifier_audio::signifier_audio_init_system | 0x1409d6320 | 38 | 1×loop+4×instantiate：UI Boop(0x142ac2465)/Cursor/Grab/Cursor/Release/HoldTimeSiginfier(原版拼写)/UiRotateGizmoMovingBack | O | audio-misc\decomp\decomp-0x1409d6320.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
