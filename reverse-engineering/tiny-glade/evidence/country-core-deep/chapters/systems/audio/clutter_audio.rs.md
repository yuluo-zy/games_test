# systems/audio/clutter_audio.rs

> 文件作用：**杂物摆放音效：PlayClutterSoundCmd kind(0..0x26=38)→event:/SFX/Cosmetics/* 瞬时播放。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::clutter_audio::clutter_audio_system | 0x1408c02f0 | 66 | kind 表 0x142aa1688(39 项)→instantiate_audio+set_position+set_parameter+start_if_not_playing（串表 0x142aa0e4c：BirdsNest/Box/DoorBell/BoxFish/RackBrooms…） | O | audio-misc\decomp\decomp-0x1408c02f0.txt |
