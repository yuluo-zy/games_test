# RESTORE — systems/audio/music（组 state）

> restorer-state，2026-10-15。music_player/music_system/music_debug_gui/native 多为既有
> 实现；本批核对并吸收 reviewer FIX。

## 本批改动

| 项 | Rust 位置 | 说明 |
|---|---|---|
| looped→PlayingLoopedOnce 不清累计时间（reviewer FIX-1） | `music_player.rs::update_with_random` | B1 0x1408b5615 只写 state=2、ticks=0，不清 +0x11c；测试同步改为 1.6 |
| `get_timeline_beat_properties` 存储顺序（reviewer FIX-2） | beat→tempo→count→upper→lower→**bar 最后写** | 消除跨原子读窗口差异 |

## 核对结论（既有实现已正确，未动）

- 播放器 4 状态机 + A/B 双跳表语义、洗牌弹出器（identity→shuffle→不重复守卫→弹尾）、
  MusicAudioState 三事件（Ambience 非 ×2 mus_GladePreview）、EscMenuState 门控、
  stop 模式映射（mode0=ALLOWFADEOUT/mode1=IMMEDIATE）均经 reviewer fresh 反编译核对，
  src 比章节更准。
- `MusicPlayer::current_track`（PDB 访问器）已由公开字段覆盖。

## 仍缺失/待取证

- **完整 GladeAudio 曲库生产**（HALLOWEEN_MUSIC_EVENTS → `load_tracks` 接线）：track 名由
  调用方提供，producer 未建。
- **live FMOD 集成**：native.rs 为真实 libfmod 2.222.6 适配；Studio 初始化/银行加载由
  宿主持有，未在 core 内接线。
- 全局注册（音乐系统加入 Schedule）经 `configure_music_consumers` 提供，宿主装配待接。

## 编译/测试状态

- `cargo check -p country-core --offline`：我的文件零错误零警告。
- `cargo test -p country-core --offline --lib audio::music`：1/1 通过（四状态转换命令序，
  含 FIX-1 后的 1.6 累计值断言）。

## 合同/状态建议

- reviewer 判 fix_pending → 本批已吸收 FIX-1/2，交回复验。
- 验收缺口：GladeAudio 曲库生产、live FMOD 集成、全局注册（合同已披露）。
