# LOGIC — systems/audio/music（逐函数逻辑判等审计）

> 审计员: logic-verifier-session。日期: 2026-10-17。
> 依据: 2026-10-16-logic-verify-protocol.md；Ghidra shared 只读 fresh 反编译 + radare2(RNE) 定点反汇编核对 jumptable/常量/模式参数。

## music_player.rs

| VA | 原函数 | verdict | 证据 |
|---|---|---|---|
| 0x1408b5500 | MusicPlayer::update | **判等** | 状态机 5 态（0=SwitchTrack 单独、1=Playing、2=PlayingLoopedOnce、3=SwitchAtNextOpportunity、4=Switching）逐分支核对：位置取 max(0,timeline)；looped(现位<旧位)→BAR_COUNT=0；SwitchTrack 体（last=0+BAR_COUNT=0+switch_track+fade_in.start+track.start+state=1）；jumptable 0x142a9fa24(非循环)/0x142a9fa34(循环) 目标逐一解出（超出第 4 项即 wall_audio_ui 数据，状态仅 5 态）；状态体常量 1.5(0x1429258d0)/18.5(0x142a9f400)/19.0(0x142a9f404)/18.0(0x142a7ba80)/120.0(0x1429cd0f8) 全符；state2 prob²>random(0x140caf3c0)；Switching 顺序=track_switching.stop(>18.5, Immediate)→state0(>19.0)→fade_in.stop→track.stop(>18.0, Immediate)；末尾反暂停快照块（当前曲目 paused→set_paused(false)+track_switching set_paused(false)）一致。**小分歧**: 无曲目日志文本 原 "Failed to switch music track; will retry!"(0x1408b57c8) vs Rust "No music track available"（仅日志串） |
| 0x1408b6250 | stop_photomode_snapshots | **判等** | +0x70(photomode)/+0xa0(first_person) disc==1 时 stop(**R8=0 AllowFadeout**，r2 0x1408b627a/62a9 核对) |
| 0x1408b63c0 | init | **判等** | 4 快照名 "snapshot:/Track Switching"/PhotoMode/FirstPerson/MusicFadeIn 逐串核对；disabled/active 分支；状态=0/current=None/playlist cap8 |
| 0x1408b6b10 | load_tracks | **判等** | 清 tracks+track_names→逐 path instantiate→Active 且 set_callback is_ok → push tracks+track_names→state=0 |
| 0x1408b70d0 | switch_track | **判等** | **洗牌弹出器**: playlist 空→identity 0..n→fastrand::shuffle(0x14132b1a0)→首元素==当前曲目时 swap(0, len/2)（机器码 `uVar4&~1` 折算 middle×8，非弹尾换首）→pop 尾元素→current_track=Some(pop)。与 Rust 逐句符 |
| 0x1408b72e0 | stop_immediately | **判等** | 全部 tracks disc==1 stop(**R8=1 Immediate**)+current=None+track_switching stop |
| 0x1408b7490 | pause | **判等** | current 曲目 set_paused(true)+track_switching set_paused(true)（R8=1 核对） |
| 0x1408b7620 | get_timeline_beat_properties | **判等** | 忽略 null；存储序 beat→tempo→count(wrapping_add+max(bar))→upper→lower→**bar 最后写**（reviewer FIX-2 确认） |

glue: impl$2::fmt(0x14083fec0, Debug 变体名) 为派生胶水。

## music_system.rs

| VA | 原函数 | verdict | 证据 |
|---|---|---|---|
| 0x140a71f20 | music_audio_init_system | **判等** | MusicAudioState 装配: ambience 路径来自 load_glade_audio::default(0x1408bf2c0) 第二串="event:/Amb/Ambience"(19 字符已解码)并 start；preview_music="event:/Music/mus_GladePreview"(0x1d)、preview_ambience="event:/Amb/GladePreview"(0x17)；player init；insert_resource。与 Rust 全符 |
| 0x140a72810 | music_audio_system | **未实现-需重做** | 大部分判等: target_richness=snap(clamp(+delta/120))、**mode==0x13(19)门 target=1.0**(r2 0x140a72b67 `cmp eax,0x13; je`)、time_continuous_lerp(rate=1)+0.99→1.0+snap、参数名 MusicRichness/NewGladeTime(0.8 钳)/PreviewGladeType(audio_id 表 0x142ad67f0=[0,3,1,2,4])、EscMenuState tag|8==10 门、update 用 bevy Time delta(+0x58)、pause/stop_photomode、preview 分支全符。**分歧 1 处**: photomode 分支 start 目标反了——原 mode==2 分支 `lea rcx,[rbx+0x70]` 起 **photomode** 且 `cmp [rbx+0xa0],1` 停 **first_person**；else 分支起 first_person(+0xa0) 停 photomode(+0x70)。**Rust mode==2 起 first_person、else 起 photomode（start 实例互换）**。stop 目标两版一致 |
| (inline) | update_music_richness_based_on_time | 内联 | 无独立 VA；richness/target_richness 更新逻辑已随 music_audio_system fresh 核对一致 |

## music_debug_gui.rs

| VA | 原函数 | verdict | 证据 |
|---|---|---|---|
| 0x14083c4a0 | music_debug_gui | **判等** | context None→return；ResMut changed 标记；Window::new "DJ station"(0x142a9350b,len 0xa)+`.open(&mut debug.show_music_info)`(+0xf6)+show_dyn 闭包。GUI 按钮内容在 closure 0x14083b480(胶水) |

## 汇总

- music_player 8 业务函数全 **判等**（含 switch_track 洗牌弹出/状态机逐分支）；1 处日志文本小分歧。
- music_system: init_system **判等**；music_audio_system **未实现-需重做**（photomode 分支 start 实例反了，1 处数据流分歧）。
- music_debug_gui 主函数 **判等**。
- 未实现清单: 无（全部已实现函数均过，除上述 1 处）。
