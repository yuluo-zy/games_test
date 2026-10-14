# systems/audio/music/music_system.rs

> 文件作用：**音乐系统：richness/NewGladeTime 全局参数 + 播放器驱动。**

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::music::music_system::music_audio_system` | 0x140a72810 | 0 | richness=clamp(time/120)三档 snap-off(0.24/0.49/0.74)+time_continuous_lerp 平滑(@+0x1c0/0x1c4)；set_parameter(MusicRichness/NewGladeTime)；AppMode (mode|8)==0xa→MusicPlayer::update 否则 pause | O | audio-misc\decomp\decomp-0x140a72810.txt |
| country_core::systems::audio::music::music_system::music_audio_init_system | 0x140a71f20 | 33 | GladeAudio::default→3×instantiate_audio(mus_GladePreview×2+Amb/GladePreview)→start→MusicPlayer::init→insert_resource<MusicAudioState>；参数串 MusicRichness/NewGladeTime/PreviewGladeType | O | audio-misc\decomp\decomp-0x140a71f20.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::music::music_system::update_music_richness_based_on_time`（行 136）— 系统:周期性处理
