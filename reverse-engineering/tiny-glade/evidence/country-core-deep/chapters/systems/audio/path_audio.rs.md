# systems/audio/path_audio.rs

> 文件作用：**小路/卵石绘制音效（CobblestoneDraw 循环 + 4 个 snappy 循环）。**

函数总计 9：业务 4 / 内联无独立符号 2 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::audio::path_audio::update_draw_audio<country_core::systems::audio::garden_audio::garden_audio_system::closure_env$1> | 0x14166d9f0 | 0 | update_draw_audio<garden_closure> 模板实例：花园栅格索引→loop 应播标志/stop | O | audio-misc\decomp\decomp-0x14166d9f0.txt |
| country_core::systems::audio::path_audio::update_draw_audio<country_core::systems::audio::path_audio::path_audio_system::closure_env$5> | 0x14166d890 | 0 | update_draw_audio<path_closure> 模板实例：路径栅格→loop 驱动 | O | audio-misc\decomp\decomp-0x14166d890.txt |
| country_core::systems::audio::path_audio::path_audio_init_system | 0x1409d08d0 | 39 | instantiate_loop(CobblestoneDraw)+instantiate_loop_snappy×4：Path Draw(0x142ac1660)/Erase Draw(0x142ac1690)/PathWaterDraw/PathWaterErase | O | audio-misc\decomp\decomp-0x1409d08d0.txt |
| country_core::systems::audio::path_audio::path_audio_system | 0x1409d0f80 | 107 | 事件栅格→update_draw_audio 模板（行 69） | O | audio-misc\decomp\decomp-0x1409d0f80.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::audio::path_audio::update_draw_audio`（行 69）— 系统:周期性处理
- `country_core::systems::audio::path_audio::brush_editing_speed_01`（行 234）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项
