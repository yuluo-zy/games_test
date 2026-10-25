# systems/audio/mod.rs

> 文件作用：**音频模块注册：16 个事件类型 + 运行期/启动期系统。**

函数总计 4：业务 3 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::AsyncLoadedAudioResources::new | 0x140a7abb0 | 86 | 跨线程 mpmc 异步资源加载包构造（24B；AudioSamplesLoaded 通道泛型） | O |  |
| country_core::systems::audio::impl$1::add_audio_systems | 0x1409411a0 | 95 | music_debug_gui + water_audio_system/water_ambience_audio_system 两个 run_if 包（0x26 SystemSet 归属） | O | audio-misc\decomp\decomp-0x1409411a0.txt |
| country_core::systems::audio::impl$1::add_audio_events | 0x140941450 | 179 | 16×register_event：PlayTerrainSound/SuppressPathSound/PlayReplayAudio/PlayDecoratorSoundCmd/PlayStairsAudioCmd/PlayWallUiSound/PlayWallAudio/RequestWallMaterialAudio/WallBendAudioParamReset/PlayTimberframeSound/PlaySheepReactSound/PlayMenuUiSound/PlayBuildingAudio/PlayElevationSupportSoundCmd/PlayClutterSoundCmd/PlayTreeSoundCmd（PDB 名 add_audio_startup_systems 错位） | O | audio-misc\decomp\decomp-0x140941450.txt |

## 实现方式（算法级）（O，multi-baseline §1 + DIGEST §2.13）

**add_audio_events @0x140941450（PDB 名 add_audio_startup_systems 错位）**：16 段同构块
（TypeId 查组件表未注册则 `register_event<Type>`），注册顺序即：
PlayTerrainSound/SuppressPathSound/PlayReplayAudio/PlayDecoratorSoundCmd/PlayStairsAudioCmd/
PlayWallUiSound/PlayWallAudio/RequestWallMaterialAudio/WallBendAudioParamReset/
PlayTimberframeSound/PlaySheepReactSound/PlayMenuUiSound/PlayBuildingAudio/
PlayElevationSupportSoundCmd/PlayClutterSoundCmd/PlayTreeSoundCmd。（O）
**add_audio_systems @0x1409411a0**：两次 add_systems（bevy 依赖）+ 两个 run_if 条件包：
`music_debug_gui`（0x140b66c20）、`water_audio_system`（run_if 0x140985d30）、
`water_ambience_audio_system`（run_if 0x1412ec640）；`mov [rbp+0x1d8],0x26` +
intern<SystemSet> → 某系统入 SystemSet 0x26。（O）
**AsyncLoadedAudioResources::new @0x140a7abb0**：24B mpmc 异步资源加载包构造（跨线程通道，
AudioSamplesLoaded 泛型实例化）。O

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::impl$1::add_audio_startup_systems`（行 144）— 构造/注册/生成
