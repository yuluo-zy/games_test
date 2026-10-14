# systems/audio/music/music_player.rs

> 文件作用：**音乐播放器：曲目池 + 4 状态机（含节拍同步）+ 快照触发。**

函数总计 15：业务 8 / 内联无独立符号 5 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::music::music_player::MusicPlayer::update | 0x1408b5500 | 51 | 状态机（state@+0x118∈0..4，双跳表 0x142a9fa24/A 0x142a9fa34/B 各 4 有效条目）：每 case time_acc@+0x11c+=dt；A1 超阈值停主事件；A2 Duration@+0x120(round-to-even)超时→推进；A3 节拍同步(0x143634c68==1 且 0x143634c90&3==0→state=4+start)；A4 超阈值 fadeout 停/超阈值3→state=0；B 表=回卷路径；state==0→switch_track；公共尾 get_paused→恢复/切；每帧清零 0x143634c90 | O | audio-misc\decomp\decomp-0x1408b5500.txt |
| country_core::systems::audio::music::music_player::MusicPlayer::stop_photomode_snapshots | 0x1408b6250 | 172 | 停 PhotoMode 快照（snapshot:/PhotoMode O） | I（内联，名推断） | audio-misc\decomp\decomp-0x1408b6250.txt |
| country_core::systems::audio::music::music_player::MusicPlayer::init | 0x1408b63c0 | 177 | 3×instantiate_audio(mus_GladePreview×2+Amb/GladePreview)→start | O | audio-misc\decomp\decomp-0x1408b63c0.txt |
| country_core::systems::audio::music::music_player::MusicPlayer::load_tracks | 0x1408b6b10 | 200 | 曲库装载（Halloween/Tiny,Single,Forgotten,Enchanted 串 O） | I | audio-misc\decomp\decomp-0x1408b6b10.txt |
| country_core::systems::audio::music::music_player::MusicPlayer::switch_track | 0x1408b70d0 | 223 | 切曲（track 下标推进+start/stop） | I | audio-misc\decomp\decomp-0x1408b70d0.txt |
| country_core::systems::audio::music::music_player::MusicPlayer::stop_immediately | 0x1408b72e0 | 237 | 0x1408b72e0（PDB 名 current_track 错位）：遍历全部曲目[+0]==1→stop(1=ALLOWFADEOUT)+主事件[+0x10]==1→stop(1)；[+0]=0 | O | audio-misc\decomp\decomp-0x1408b72e0.txt |
| country_core::systems::audio::music::music_player::MusicPlayer::pause | 0x1408b7490 | 245 | set_paused(true)（曲目+主事件） | I | audio-misc\decomp\decomp-0x1408b7490.txt |
| country_core::systems::audio::music::music_player::get_timeline_beat_properties | 0x1408b7620 | 269 | 0x1408b7620：非 getter——把输入结构{i32,i32,f32,i32,i32}写入 6 个进程全局 0x143634c68/c70(f32)/c78/c80/c88/c90（c88/c90 running max；c90=update 每帧清零的节拍计数） | O | audio-misc\decomp\decomp-0x1408b7620.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::music::music_player::MusicPlayer::trigger_photomode_minus_fpv_snapshot`（行 162）— 待推断
- `country_core::systems::audio::music::music_player::MusicPlayer::trigger_fpv_snapshot`（行 167）— 待推断
- `country_core::systems::audio::music::music_player::MusicPlayer::current_track`（行 232）— 查询/计算
- `country_core::systems::audio::music::music_player::MusicPlayer::probabilistic_switch_track`（行 252）— 待推断
- `country_core::systems::audio::music::music_player::generate_track_playlist`（行 294）— 构造/注册/生成

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`fmt` 等 2 项
