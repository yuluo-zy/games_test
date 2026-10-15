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

## 实现方式（算法级）（O，round2 capstone 双跳表全解）

**MusicPlayer::update @0x1408b5500（四状态机）**：
```
字段：曲目 Vec stride 0x30（[+0]==1 使能、[+8] 事件句柄）、主事件使能[+0x40]、主 fmod 事件[+0x48]、
      state@+0x118、time_acc@+0x11c、Duration@+0x120、last_position@+0x128
每帧：
  ① pos = get_timeline_position(当前曲目)，clamp ≥ 0
  ② state != 0 → 若 pos < last_position（曲目回卷）→ 清零节拍全局 0x143634c90 + 跳表 B
     （0x142a9fa34）；否则跳表 A（0x142a9fa24）；均写 last_position = pos，
     以 (state-1)*4 索引（4 有效条目 = 状态 1..4）：
     A1(0x1408b56ef) time_acc > 阈值1 且主使能 → 主事件 stop(立即)
     A2(0x1408b597d) Duration@+0x120(i64；负值 round-to-even (v>>1)|(v&1) 后 ×2)
                      超时 → cvttss2si 夹取 → 推进/切换路径 0x1408b5b52
     A3(0x1408b59b0) 节拍同步：全局 0x143634c68==1 且 0x143634c90&3==0 → state=4 + 主事件 start
     A4(0x1408b5826) acc > 阈值2 → 主事件 stop(ALLOWFADEOUT)；acc > 阈值3 → state=0（下一帧 switch）
     B1(0x1408b55d2)/B2(0x1408b5a75) 回卷路径（acc 累计后并入 A4/A1 续块）
     B3(0x1408b5975) 无条件跳 0x1408b59be（0x143634c68==1 → state=4+start；随后 stop 事件@+0x48）
     B4 = A4（同块）
     每 case 先 time_acc@+0x11c += dt
  ③ state == 0 → switch_track()（FUN_1408b70d0 推进到下一个 [+0]==1 曲目）
     + verbosity>4 log "snapshot:/Track Switching"（0x142a9f5d0，参数名 0x142a9f568）
     → 新曲目启动：主使能[+0x40]==1 → start(主)；否则 track[+0]==1 → start(track)；
        再 get_playback_state → state==Playing → set_paused(0) 恢复；state=1；
        失败链 unwrap panic "called Result::unwrap() on an Err value"（audio_resources.rs:43，
        上下文 0x142ad2bc0/0x142ad2c50/0x142ad2c68）
  ④ 每帧清零全局节拍计数 0x143634c90
```
**stop_immediately @0x1408b72e0（O，PDB 名 current_track 错位）**：遍历全部曲目 `[+0]==1 →
stop(1=ALLOWFADEOUT)`；主事件 [+0x10]==1 → stop(1)；`[+0]=0`（主事件使能清零）。
**get_timeline_beat_properties @0x1408b7620（O 手解，非 getter）**：输入结构 {i32,i32,f32,i32,i32}
→ 写 6 个进程全局 0x143634c68/c70(f32)/c78/c80/c88/c90（c88/c90 = 带一次修正的 running max；
c90 = update 每帧清零的节拍计数）——**发布器**，现有代码 `->(i32,f32)` 签名错。
**init @0x1408b63c0（O）**：3×instantiate_audio（`event:/Music/mus_GladePreview`×2 +
`event:/Amb/GladePreview`，串 0x142ad6318/0x142ad6350）+ start。
**pause @0x1408b7490**：set_paused(true)（曲目+主事件）。**load_tracks @0x1408b6b10**（I）：
曲库装载（Halloween/Tiny/Single/Forgotten/Enchanted 串）。

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::music::music_player::MusicPlayer::trigger_photomode_minus_fpv_snapshot`（行 162）— 待推断
- `country_core::systems::audio::music::music_player::MusicPlayer::trigger_fpv_snapshot`（行 167）— 待推断
- `country_core::systems::audio::music::music_player::MusicPlayer::current_track`（行 232）— 查询/计算
- `country_core::systems::audio::music::music_player::MusicPlayer::probabilistic_switch_track`（行 252）— 待推断
- `country_core::systems::audio::music::music_player::generate_track_playlist`（行 294）— 构造/注册/生成

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`fmt` 等 2 项
