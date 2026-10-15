# systems/audio/wall_audio_ui.rs

> 文件作用：**墙体 UI 音效（PlayWallUiSound kind 分派）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::wall_audio_ui::wall_audio_ui` | 0x1408b7690 | 60 | kind 分派（数据表 0x142a9f730 邻域）：ColorBricks/Gate/Halftimber/Window、ChangeWallStyle、WindowDrag、Lantern/TurnOn、Chimney/ToggleSmokeOn|Off（[+9] 字节二选一）、BuildingSupportsRemoveWooden、WallSplit 等→instantiate_audio_w_position | O | audio-misc\decomp\decomp-0x1408b7690.txt |

## 实现方式（算法级）（O，DIGEST §2.13 事件串补全）

**wall_audio_ui @0x1408b7690**：PlayWallUiSound kind 分派（数据表 0x142a9f730 邻域）：
- 瞬发/参数事件：`event:/SFX/ChangeWallStyle`、`Lantern/TurnOn`、`Chimney/ToggleSmokeOn|Off`
  （按事件 +0x9 字节二选一）、`BuildingSupportsRemoveWooden`、`WallSplit` 等。（O）
- 分派目标统一走 `instantiate_audio_w_position`（roof_audio @0x140aa3360）。I（接线）
- kind→事件名逐条完整映射 U（仅串表邻域已录）。O/I 混合
