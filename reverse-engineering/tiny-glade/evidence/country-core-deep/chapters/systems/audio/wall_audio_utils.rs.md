# systems/audio/wall_audio_utils.rs

> 文件作用：**墙体循环播放器（AudioLoopPlayer，10 循环的公共实现）。**

函数总计 3：业务 1 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play | 0x1409285a0 | 14 | +0x38=1(应播)；+0x28<+0x10 否则 panic；instances[current]→set_position→set_parameter(editing_speed)→set_volume | O | audio-misc\decomp\decomp-0x1409285a0.txt |

## 实现方式（算法级）（O，decomp-0x1409285a0 + DIGEST §2.5）

**AudioLoopPlayer @0x1409285a0**（stride 0x40）：
`{instances Vec cap-first（cap@+0, ptr@+8, len@+0x10）, 参数 Vec@+0x18, current usize@+0x28,
should_play u8@+0x38}`。（O）
**request_play**：
1. `should_play=1`（置应播标志，帧尾由 wall_audio_fmod 统一 start/stop）。（O）
2. `current(+0x28) < len(+0x10)` 否则 **panic**（index out of bounds，wall_audio_utils.rs）。（O）
3. `instances[current]`（0x30B AudioInstance）→ `set_position` → `set_parameter("editing_speed")`
   → `EventInstance::set_volume`。（O）
- 帧尾配对：`AudioLoop::stop`（0x140a59430）/`start_if_not_playing`（0x140a596b0）由
  wall_audio_fmod 调用。（O）
- `resolve`（行 32）/`new`（行 46）无独立符号（内联）。I

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::resolve`（行 32）— 查询/计算
- `country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::new`（行 46）— 构造
