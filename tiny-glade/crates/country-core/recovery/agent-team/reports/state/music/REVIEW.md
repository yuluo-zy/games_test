# REVIEW — systems/audio/music（组 state）

> 审查员：reviewer-state，2026-10-15。独立核对，以原二进制证据为准。
> 范围：`src/systems/audio/music/{mod,music_player,music_system,music_debug_gui,native}.rs`。
> 证据：audio-misc/decomp/*、chapters/systems/audio/music/*、本审查 fresh 反编译（ghidra shared）
> + RNE 跳表/常量/串直读 + 机器码反汇编（capstone）。

## 逐函数判定

### music_player.rs
| 函数 | verdict | 说明 |
|---|---|---|
| `MusicPlayer::init` | **OK** | fresh 核对 0x1408b63c0：4×instantiate **snapshot:/Track Switching / PhotoMode / FirstPerson / MusicFadeIn**（串 0x142a9f601/638/668/698，len 25/19/21/21）。**章节"mus_GladePreview×2+Amb/GladePreview"错误**，src 正确。 |
| `MusicPlayer::load_tracks` | OK | 仅 push 已装 beat callback 的实例（对应 0x1408b6b10 曲库装载；track 名由调用方给出，未硬编码）。 |
| `MusicPlayer::switch_track` | **OK** | 机器码 0x1408b70d0：playlist 空→建 0..n→fastrand shuffle(0x14132b1a0)→若 playlist[0]==previous 则 swap(0, **len/2**)（`and ~1` + `*4` 折算 = len/2）→pop 尾；非空直接 pop。src 逐一吻合（含 1 曲 no-op）。 |
| `MusicPlayer::update`/`update_with_random` | **FIX(1)** | A/B 双跳表（0x142a9fa24/0x142a9fa34）+ 4 状态机 fresh 反汇编核对：A1(0x1408b56ef) `acc>1.5 && fade_in 使能 → fade_in.stop(mode0=ALLOWFADEOUT)`；A2(0x1408b5b84 区) `elapsed/120 → prob²>random → state=3` + fade_in.stop；A3(0x1408b59b0) `BAR_COUNT&3==0 且 CURRENT_MUSIC_BEAT==1 → state=4+track_switching.start`；A4(0x1408b5826) `acc>18.5→track_switching.stop(立即)；acc>19→state=0；fade_in.stop(ALLOWFADEOUT)；acc>18→当前曲 stop(立即)`；B1(0x1408b55d2) looped 态1：acc+=dt→fade_in.stop→**state=2, ticks=0（不重置 +0x11c）**。src 顺序全对，但见 FIX-1。 |
| `PlayerState::Playing` looped 转换 | **FIX(1)** | B1 0x1408b5615 `mov [rsi+0x118],2; mov [rsi+0x120],0` **不清 +0x11c**；src `PlayingLoopedOnce { elapsed_time: 0.0, … }` 将累计时间清零。原版进入态 2 时携带累计 elapsed（例 1.6s），概率=elapsed/120 首次即采样且值更大；src 从 0 起步，概率与采样时机偏移。 |
| `PlayerState::Playing` fade_in stop 模式 | OK | A1 `xor r8d,r8d`=mode 0=**ALLOWFADEOUT**；src 用 AllowFadeout。章节"A1 stop(立即)"误标（1=IMMEDIATE）。 |
| `stop_immediately` | OK | 0x1408b72e0：全曲目 stop(1=IMMEDIATE)+current=None+track_switching.stop(1)。章节"(1=ALLOWFADEOUT)"误标。 |
| `pause` | OK | 当前曲+track_switching set_paused(true)（0x1408b7490）。 |
| `stop_photomode_snapshots` | OK | photomode+first_person stop(ALLOWFADEOUT)（0x1408b6250）。 |
| `MusicInstance`/`MusicBackend`/`MusicEvent` trait 族 | OK | Disabled/Active 双形态对应原 Disabled AudioCore 分支；`.unwrap()` 对应原 unwrap_failed panic。 |
| `get_timeline_beat_properties` | **PARTIAL(低)** | 语义全对（None 忽略；beat/bar/tempo/signature→6 全局；BAR_COUNT 增量+max(bar)）。**存储顺序不同**：原版 beat→tempo→count→upper→lower→bar（bar 最后）；src count→beat→bar→tempo→upper→lower。跨原子读一致性窗口略异（原版无原子，src 用原子已更强）。 |
| `timeline_snapshot` | OK | 读 6 原子组装 TimelineSnapshot。 |

### music_system.rs
| 函数 | verdict | 说明 |
|---|---|---|
| `MusicAudioState::init`/`music_audio_init_system` | **OK** | fresh 核对 0x140a71f20：GladeAudio::default 串 **event:/Amb/Ambience**（0x142a9f331 实测）+ **event:/Music/mus_GladePreview**(0x1d) + **event:/Amb/GladePreview**(0x17)，首个 start。**章节"mus_GladePreview×2"错误**，src 正确。 |
| `update_music_audio`/`music_audio_system` | **OK** | fresh 核对 0x140a72810 十参数：richness=clamp(target+game_delta/120,0,1) 后三档 snap-off(0.24..0.25→0.251/0.49..=0.5→0.501/0.74..=0.75→0.751)；target 在 AppMode==0x13 时强 1.0（param_1 是 AppMode）；time_continuous_lerp(rate=1)→richness snap-off 且 >0.99→1.0；set_parameter("MusicRichness", force?1.0:richness)（DebugGui+0x100）；NewGladeTime=clamp(delta+acc,0,0.8)（0x142a16cb8=0.8）；**门 `(menu.tag|8)==10` 用 param_4（与 AppMode=param_1 分离，src 用 EscMenuState 正确，章节"AppMode 门"过期）**；开启分支 photomode(camera_mode==2)/first_person 切换、preview_music/ambience、PreviewGladeType=表[theme]（0x142ad67f0 实测 [0,3,1,2,4]）。参数名串 MusicRichness/NewGladeTime/PreviewGladeType 实测。 |
| `configure_music_consumers` | OK | 调度接线（chain music_audio_system→music_debug_gui）。 |
| `preview_glade_parameter` | OK | theme.audio_id() 与表一致。 |

### music_debug_gui.rs
| 函数 | verdict | 说明 |
|---|---|---|
| `music_debug_gui` | **OK**（按钮部分 I） | 0x14083c4a0：Window "DJ station"(0x142a9350b, len 10)；open 读 DebugGui **+0xf6**（src show_music_info）；force 读 **+0x100**（src force_music_richness，与 music_audio_system 同一字节）；richness 标签读 player+0x1c0。按钮行为（Force switch/Switch immediately）在 egui 闭包内，未独立反编译，标 I。 |

### native.rs
| 函数 | verdict | 说明 |
|---|---|---|
| `FmodMusicBackend`/`FmodMusicEvent`/`timeline_callback` | **OK** | 真实 libfmod 2.222.6 适配；None=Disabled 分支；Drop release()（对应 instantiate_audio stop-on-drop None +0x28=2）；set_callback(TIMELINE_BEAT 0x1000)→get_timeline_beat_properties。Studio 由调用方持有，不伪造播放。 |

## FIX 清单（交 restorer）
1. **FIX（中）`update_with_random` 的 looped→PlayingLoopedOnce 转换清零 elapsed_time**：原版 B1(0x1408b5615) 只写 `state=2, +0x120(ticks)=0`，**不清 +0x11c**。src 应保留累计 elapsed（`PlayerState::PlayingLoopedOnce { elapsed_time: <carry>, ticks: 0 }`），否则概率 `elapsed/120` 与随机采样时机偏移。src 测试当前断言 `elapsed_time: 0.0` 需同步修正（依据机器码：进入态 2 时 elapsed 应为 1.6 例值）。
2. **PARTIAL(低) `get_timeline_beat_properties` 存储顺序**：改与机器码一致（beat→tempo→count→upper→lower→bar 最后写），消除跨原子读窗口差异。

## 核对要点（src 优于既有证据）
- 章节多项过期/错误，src 反而正确：init 四快照、init 三事件（Ambience 非 ×2 mus_GladePreview）、门资源是 EscMenuState 而非 AppMode、stop 模式映射（mode0=ALLOWFADEOUT/mode1=IMMEDIATE，章节两处误标）。
- 状态机 A/B 双跳表全部经 fresh 反汇编逐 case 实证，src 的 `looped || (BAR_COUNT&3==0)` 恰为 A3/B3 合并语义。

## 过程问题
- contracts.json 的 `evidence/core-recovery-20261015/music/REPORT.md` 在磁盘不存在；完成度引用缺失证据。

## 结论
实现层面：播放器状态机、切换/回卷、队列、richness/NewGladeTime 参数、调试面板、FMOD 适配均为真实实现且**除 FIX-1 外逐字节核对通过**。验收层面：完整 GladeAudio 曲库生产（HALLOWEEN_MUSIC_EVENTS→load_tracks 接线）、live FMOD 集成与全局注册仍未闭合（合同已披露）。状态建议：`in_review → fix_pending`（FIX-1、FIX-2 提交 restorer）。
