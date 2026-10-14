# systems/audio/menu_audio.rs

> 文件作用：**菜单 UI 音效：PauseMenu 快照 + 13 个 UI 事件。**

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::menu_audio::menu_audio` | 0x140a0b4c0 | 0 | AppMode (mode|8)==0xa 门禁停/播快照；PlayMenuUiSound kind 表 0x142ac706c(0..12)→Menu/Hover、Ui2dHover、Menu/Select、Menu/SettingsTab、Menu/NewSession、Menu/StartSession、Menu/Back、Menu/Close、Ui2dLockedClickNew 等 | O | audio-misc\decomp\decomp-0x140a0b4c0.txt |
| country_core::systems::audio::menu_audio::menu_audio_init_system | 0x140a0b340 | 35 | instantiate_audio(snapshot:/PauseMenu) | O | audio-misc\decomp\decomp-0x140a0b340.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
