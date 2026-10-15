# systems/audio/wall_audio_path.rs

> 文件作用：**墙体路径绘制音效。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::wall_audio_path::wall_audio_path | 0x140a89bb0 | 13 | 路径绘制循环驱动（0x140a89bb0；细节 U） | I | audio-misc\decomp\decomp-0x140a89bb0.txt |

## 实现方式（算法级）（decomp-0x140a89bb0，U 为主）

**wall_audio_path @0x140a89bb0**：墙体路径绘制循环驱动。decomp 仅见两处
`FUN_140562c30(元素+0x20, &0x142c3a148)`（event/日志写入路径），完整循环应播/参数链
未展开——事件→WallAudioState 循环 player 的驱动语义 **U**。I
