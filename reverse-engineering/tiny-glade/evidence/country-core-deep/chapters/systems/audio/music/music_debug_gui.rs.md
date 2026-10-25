# systems/audio/music/music_debug_gui.rs

> 文件作用：**音乐调试 egui 面板（开发用）。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::music::music_debug_gui::music_debug_gui | 0x14083c4a0 | 10 | (Res<EguiContextResource>,ResMut<MusicAudioState>,ResMut<DebugGui>)：读 [+0x100] 开关字节与 MusicPlayer+0x1c0 richness 渲染调试窗 | O | audio-misc\decomp\decomp-0x14083c4a0.txt, ui-input\decomp\decomp-0x14083c4a0.txt |

## 实现方式（算法级）（O，decomp-0x14083c4a0）

**music_debug_gui @0x14083c4a0**：egui 调试窗，签名
`(Res<EguiContextResource>, ResMut<MusicAudioState>, ResMut<DebugGui>)`：读 `[+0x100]`
开关字节（DebugGui 域）与 `MusicPlayer+0x1c0` richness 平滑值渲染面板；仅开发/调试面，
无 FMOD 调用。O

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
