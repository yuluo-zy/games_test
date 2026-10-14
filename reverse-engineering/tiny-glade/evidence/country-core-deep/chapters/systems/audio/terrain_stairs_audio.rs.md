# systems/audio/terrain_stairs_audio.rs

> 文件作用：**地形楼梯音效（SuppressPathSound + StairsCounter 状态 + StairDraw_OneShot 瞬发）。**

函数总计 3：业务 1 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::terrain_stairs_audio::terrain_stairs_audio_system` | 0x141462dc0 | 64 | 消费楼梯地形交互事件驱动音频 | O | terrain-water\decomp\decomp-0x141462dc0.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::terrain_stairs_audio::terrain_stairs_audio_init_system`（行 49）— 初始化地形楼梯音频资源

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
