# systems/audio/ivy_audio.rs

> 文件作用：**常春藤生长音效（单瞬发）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::ivy_audio::ivy_audio_init_system | 0x140ad5350 | 13 | instantiate_audio(event:/SFX/IvyGrow) | O | audio-misc\decomp\decomp-0x140ad5350.txt |

## 实现方式（算法级）（O，decomp-0x140ad5350）

**ivy_audio_init_system @0x140ad5350**：单次 `instantiate_audio`（`event:/SFX/IvyGrow`）
→ 无循环、无 kind 分派；常春藤生长触发即播（瞬时一次）。O
