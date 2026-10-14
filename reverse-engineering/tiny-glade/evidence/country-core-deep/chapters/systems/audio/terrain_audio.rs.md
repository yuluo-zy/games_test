# systems/audio/terrain_audio.rs

> 文件作用：**地形编辑音效（4 循环 + 编辑状态机）。**

函数总计 4：业务 2 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::terrain_audio::terrain_audio_system` | 0x1414618c0 | 0 | 消费 PlayTerrainSound 事件播放地形音效 | O | terrain-water\decomp\decomp-0x1414618c0.txt
| `country_core::systems::audio::terrain_audio::terrain_audio_init_system` | 0x1414613e0 | 52 | 初始化地形音频资源（7.6KB） | O | terrain-water\decomp\decomp-0x1414613e0.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::terrain_audio::EditStateAudio::new`（行 18）— 编辑状态音频构造
- `country_core::systems::audio::terrain_audio::EditStateAudio::update`（行 25）— 编辑状态音频更新
