# systems/audio/roof_audio.rs

> 文件作用：**屋顶编辑音效：3 循环 + 位置/音量即时实例辅助。**

函数总计 4：业务 4 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::roof_audio::roof_audio_system | 0x140aa3860 | 0 | 按 RoofAudioState 驱动循环音量/位置 | O | audio-misc\decomp\decomp-0x140aa3860.txt |
| country_core::systems::audio::roof_audio::roof_audio_init_system | 0x140aa2fd0 | 28 | 3×instantiate_loop：RoofShape/RoofTipMove(0x142adcb38)/InterRoofMerlonsRearrange(0x142adcb68)（重建代码 RoofAppear/RoofSwitchDirection 为错） | O | audio-misc\decomp\decomp-0x140aa2fd0.txt |
| country_core::systems::audio::roof_audio::instantiate_audio_w_position | 0x140aa3360 | 50 | 真实签名 4 参(core,name&str,pos)：instantiate_audio+set_position（调用点 0x1408b7788 O） | O | audio-misc\decomp\decomp-0x140aa3360.txt |
| country_core::systems::audio::roof_audio::instantiate_audio_w_position_and_volume | 0x140aa3590 | 60 | instantiate_audio→set_position→start_if_not_playing→EventInstance::set_volume | O | audio-misc\decomp\decomp-0x140aa3590.txt |

## 实现方式（算法级）（O，DIGEST §2.13 + decomp-0x140aa2fd0/0x140aa3360/0x140aa3590）

**roof_audio_init_system @0x140aa2fd0**：3×`instantiate_loop`：`RoofShape`、
`RoofTipMove`(0x142adcb38)、`InterRoofMerlonsRearrange`(0x142adcb68)——重建代码
RoofAppear/RoofSwitchDirection 事件名**错误**（O 串纠正）。（O）
**roof_audio_system @0x140aa3860**：按 RoofAudioState 驱动各循环的音量/位置（无 kind 表）。O
**instantiate_audio_w_position @0x140aa3360**：真实签名 4 参 `(core, name&str, pos)`：
instantiate_audio（0x140a54440）+ set_position（0x140a57da0）；调用点 0x1408b7788 O。
**instantiate_audio_w_position_and_volume @0x140aa3590**：instantiate_audio → set_position
→ start_if_not_playing → `EventInstance::set_volume`——**被 wall_audio_fmod 瞬发类复用**。O
