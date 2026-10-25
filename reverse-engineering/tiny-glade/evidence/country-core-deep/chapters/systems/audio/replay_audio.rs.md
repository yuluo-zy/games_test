# systems/audio/replay_audio.rs

> 文件作用：**撤销/重做/拒绝输入音效。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::replay_audio::replay_audio_init_system | 0x140ad54d0 | 23 | 2×instantiate_audio(event:/UI/Undo、event:/UI/Redo) | O | audio-misc\decomp\decomp-0x140ad54d0.txt |
| country_core::systems::audio::replay_audio::replay_audio_system | 0x140ad5800 | 30 | kind 0/1→Undo/Redo、2→event:/UI/DenyInput；AppMode==0xd 设全局参数 | O | audio-misc\decomp\decomp-0x140ad5800.txt |

## 实现方式（算法级）（O，decomp-0x140ad54d0/0x140ad5800）

**replay_audio_init_system @0x140ad54d0**：2×`instantiate_audio`（`event:/UI/Undo`、
`event:/UI/Redo`）。（O）
**replay_audio_system @0x140ad5800**：PlayReplayAudio kind 分派：0→Undo、1→Redo 瞬发；
2→`event:/UI/DenyInput`（拒绝输入提示音）；AppMode==0xd（摄影/特定模式）时另设全局参数。O
