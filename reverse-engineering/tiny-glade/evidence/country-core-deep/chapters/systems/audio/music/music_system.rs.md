# systems/audio/music/music_system.rs

> 文件作用：**音乐系统：richness/NewGladeTime 全局参数 + 播放器驱动。**

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::music::music_system::music_audio_system` | 0x140a72810 | 0 | richness=clamp(time/120)三档 snap-off(0.24/0.49/0.74)+time_continuous_lerp 平滑(@+0x1c0/0x1c4)；set_parameter(MusicRichness/NewGladeTime)；AppMode (mode|8)==0xa→MusicPlayer::update 否则 pause | O | audio-misc\decomp\decomp-0x140a72810.txt |
| country_core::systems::audio::music::music_system::music_audio_init_system | 0x140a71f20 | 33 | GladeAudio::default→3×instantiate_audio(mus_GladePreview×2+Amb/GladePreview)→start→MusicPlayer::init→insert_resource<MusicAudioState>；参数串 MusicRichness/NewGladeTime/PreviewGladeType | O | audio-misc\decomp\decomp-0x140a71f20.txt |

## 实现方式（算法级）（O，multi-baseline §6 + audio-extra §4 反汇编）

**music_audio_init_system @0x140a71f20**：`GladeAudio::default` → 3×instantiate_audio
（`event:/Music/mus_GladePreview`×2 + `event:/Amb/GladePreview`）→ `start`（libfmod
0x1414372d0）→ `MusicPlayer::init` → `insert_resource<MusicAudioState>`；参数串表
MusicRichness/NewGladeTime/PreviewGladeType。（O）
**music_audio_system @0x140a72810**：
- **MusicRichness 计算**（update_music_richness_based_on_time 内联，行 136）：
  `accum = clamp(State+0x1c4 + time/120.0, 0, 1)`；三档 **snap-off** 避 0.25/0.5/0.75
  （[0.24,0.25)→max(x,0.251)、[0.49,0.5]→max(x,0.501)、[0.74,0.75]→max(x,0.751)）；
  `[arg1]+0==0x13` 时 target 强制 1.0；`time_continuous_lerp`（0x140c904e0）平滑 → State+0x1c0；
  `set_parameter("MusicRichness", [arg6]+0x100 置位? 1.0 : smoothed)`。（O）
- **NewGladeTime**：`t = clamp(time + State+0x1c8, 0, 0.8)` → State+0x1c8=t →
  `set_parameter("NewGladeTime", t)`。（O）
- **帧尾门禁**：`(AppMode|8)==0xa` → `MusicPlayer::update(player, [arg7]+0x58)`（0x1408b5500）；
  否则 `MusicPlayer::pause`（0x1408b7490）；随后 stop_photomode_snapshots（0x1408b6250）/
  start_if_not_playing。（O）

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::music::music_system::update_music_richness_based_on_time`（行 136）— 系统:周期性处理
