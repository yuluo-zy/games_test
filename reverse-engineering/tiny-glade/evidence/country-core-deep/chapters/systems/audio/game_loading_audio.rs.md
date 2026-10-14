# systems/audio/game_loading_audio.rs

> 文件作用：**加载界面音频：snapshot:/Loading 快照 + event:/UI/Loading。**

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::game_loading_audio::init_game_loading_audio | 0x140b1ae70 | 18 | 创建并立即 start snapshot:/Loading；状态装填 | O |  |
| country_core::systems::audio::game_loading_audio::loading_audio | 0x140b1b1b0 | 28 | 加载进行时播 event:/UI/Loading；完成→停快照(状态→2、清标志)+1s 定时器二次保护 | O | audio-misc\decomp\decomp-0x140b1b1b0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
