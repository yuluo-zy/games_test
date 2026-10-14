# systems/audio/wall_audio_ui.rs

> 文件作用：**墙体 UI 音效（PlayWallUiSound kind 分派）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::wall_audio_ui::wall_audio_ui` | 0x1408b7690 | 60 | kind 分派（数据表 0x142a9f730 邻域）：ColorBricks/Gate/Halftimber/Window、ChangeWallStyle、WindowDrag、Lantern/TurnOn、Chimney/ToggleSmokeOn|Off（[+9] 字节二选一）、BuildingSupportsRemoveWooden、WallSplit 等→instantiate_audio_w_position | O | audio-misc\decomp\decomp-0x1408b7690.txt |
