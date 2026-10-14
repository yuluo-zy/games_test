# systems/audio/garden_audio.rs

> 文件作用：**花园绘制/擦除音效（4 循环 + update_draw_audio 模板）。**

函数总计 6：业务 2 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::audio::garden_audio::garden_audio_init_system` | 0x140a88fa0 | 27 | 初始化花园音频资源 | O | terrain-water\decomp\decomp-0x140a88fa0.txt
| `country_core::systems::audio::garden_audio::garden_audio_system` | 0x140a89480 | 52 | 消费花园编辑/掩码事件驱动音频 | O | terrain-water\decomp\decomp-0x140a89480.txt

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
