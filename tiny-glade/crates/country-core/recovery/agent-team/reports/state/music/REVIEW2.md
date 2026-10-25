# REVIEW2 — systems/audio/music（组 state 二轮复验）

> 审查员：reviewer-state，2026-10-15。复核 round1 FIX-1/FIX-2 吸收。编译/测试实测通过（audio::music 1/1）。

## FIX 吸收核对

| round1 FIX | 吸收情况 | verdict |
|---|---|---|
| FIX-1：looped→PlayingLoopedOnce 不清 +0x11c | src `update_with_random` Playing 分支 looped 时 `PlayingLoopedOnce { elapsed_time: *elapsed_time, ticks: 0 }`（行 208-211，注释引 B1 0x1408b5615）；测试断言 `elapsed_time: 1.6`（行 411）。**与机器码一致**（B1 只写 state=2、+0x120=0，不清 +0x11c）。 | **OK** |
| FIX-2：get_timeline_beat_properties 存储顺序 beat→tempo→count→upper→lower→**bar 最后写** | src 行 311-316：beat→tempo→count→upper→lower→bar last；count 在写 bar 前用旧 bar 计算（行 307-310）。**与机器码一致**。 | **OK** |

## 其余核对（既有实现未动，维持 OK）
- 播放器 4 状态机 + A/B 双跳表、洗牌弹出（identity→shuffle→不重复守卫 len/2→弹尾）、
  MusicAudioState 三事件（Ambience 非 ×2 mus_GladePreview）、EscMenuState 门控、stop 模式
  映射（mode0=ALLOWFADEOUT / mode1=IMMEDIATE）——round1 fresh 反编译已逐字节核对，src 比章节更准。
- 验收缺口（合同已披露）：GladeAudio 曲库生产（HALLOWEEN_MUSIC_EVENTS→load_tracks）、live FMOD 集成、
  全局注册（configure_music_consumers 供宿主装配）。

## 结论与 DISPATCH 建议
- **建议：music → review_passed**（FIX-1/FIX-2 正确吸收；限定已实现范围）。完整模块验收仍缺
  曲库生产 / live FMOD / 全局接线。
