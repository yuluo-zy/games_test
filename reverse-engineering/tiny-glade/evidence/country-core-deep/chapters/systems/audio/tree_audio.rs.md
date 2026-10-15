# systems/audio/tree_audio.rs

> 文件作用：**树木音效（4 循环 + 最近树距离排序）。**

函数总计 5：业务 2 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::tree_audio::tree_audio_system | 0x1414638e0 | 0 | PlayTreeSoundCmd（tree-tool 消费）+Query<&TreeAmbience> (dist,pos) 稳定排序→最近树 set_position/音量 | O | audio-misc\decomp\decomp-0x1414638e0.txt |
| country_core::systems::audio::tree_audio::tree_audio_init_system | 0x1414633e0 | 24 | 4×instantiate_loop：Tree/Amb、TreeMove、TreePlace、TreeGizmoHover/Release | O | audio-misc\decomp\decomp-0x1414633e0.txt |

## 实现方式（算法级）（O，decomp-0x1414633e0/0x1414638e0）

**tree_audio_init_system @0x1414633e0**：4×`instantiate_loop`：`Tree/Amb`、`TreeMove`、
`TreePlace`、`TreeGizmoHover/Release`。（O）
**tree_audio_system @0x1414638e0**：
- PlayTreeSoundCmd 消费 + `Query<&TreeAmbience>` `(dist,pos)` 稳定排序 → 最近树
  set_position / 音量。（O）
- 瞬发路径：instantiate_audio（0x140a54440，事件串 0x142c62fca/0x142c62fa9 len 0x1a/0x19）
  → set_parameter（0x140a57670）→ set_position（0x140a57da0）→ start_if_not_playing
  （0x140a57300）。（O）
- 越界 panic 路径 FUN_1428d9518（index out of bounds，0x142ad2f28 上下文）。（O）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项
