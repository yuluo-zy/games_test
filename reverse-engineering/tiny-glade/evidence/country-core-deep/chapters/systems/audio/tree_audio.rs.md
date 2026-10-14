# systems/audio/tree_audio.rs

> 文件作用：**树木音效（4 循环 + 最近树距离排序）。**

函数总计 5：业务 2 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::tree_audio::tree_audio_system | 0x1414638e0 | 0 | PlayTreeSoundCmd（tree-tool 消费）+Query<&TreeAmbience> (dist,pos) 稳定排序→最近树 set_position/音量 | O | audio-misc\decomp\decomp-0x1414638e0.txt |
| country_core::systems::audio::tree_audio::tree_audio_init_system | 0x1414633e0 | 24 | 4×instantiate_loop：Tree/Amb、TreeMove、TreePlace、TreeGizmoHover/Release | O | audio-misc\decomp\decomp-0x1414633e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项
