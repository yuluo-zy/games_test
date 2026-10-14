# systems/paths/draw_path.rs

> 文件作用：**路径绘制：UI 拖拽→DrawState 状态机→WorldRaster<512> 四变体绘制+互擦**

函数总计 27：业务 9 / 内联无独立符号 4 / 派生胶水 14

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::paths::draw_path::DrawState::update` | 0x140ae4bd0 | 64 | 系统:状态机 | O：DrawState::update——raw=ui_mode-2 夹取 0..3；raw∈{1,2}→mode=min(mode-1,raw)；返回 0/3 | collision-eco\decomp\decomp-0x140ae4bd0.txt |
| `country_core::systems::paths::draw_path::_::impl$0::apply` | 0x14003bb80 | 79 | Bevy 参数胶水 | O：SystemParam 胶水（apply/queue/validate） | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::systems::paths::draw_path::_::impl$0::get_param` | 0x141507a40 | 79 | Bevy 参数胶水 | O：SystemParam 胶水（apply/queue/validate） |  |
| `country_core::systems::paths::draw_path::_::impl$0::init_state` | 0x140ae5b20 | 79 | Bevy 参数胶水 | O：SystemParam impl | collision-eco\decomp\decomp-0x140ae5b20.txt |
| `country_core::systems::paths::draw_path::_::impl$0::new_archetype` | 0x14003bb80 | 79 | Bevy 参数胶水 | O：SystemParam 生命周期胶水 | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::systems::paths::draw_path::_::impl$0::queue` | 0x14003bb80 | 79 | Bevy 参数胶水 | O：SystemParam 胶水（apply/queue/validate） | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::systems::paths::draw_path::_::impl$0::validate_param` | 0x140b4b430 | 79 | Bevy 参数胶水 | O：SystemParam 胶水（apply/queue/validate） |  |
| `country_core::systems::paths::draw_path::PathRastersResources::as_float_grid` | 0x140ae4c20 | 88 | 查询/计算 | O：PathRastersResources::as_float_grid——4 路 f32 采样视图（CPU+8/+0x10 vs GPU 暂存 +0x20/+0x28，按 +0x298 标志） | collision-eco\decomp\decomp-0x140ae4c20.txt |
| `country_core::systems::paths::draw_path::draw_path` | 0x140ae4d00 | 129 | 系统:绘制 | O：draw_path——UI 模式串 "eraser"/"path"/"cobblestone" 分派 9 个 world_raster::draw_raster 单态（每变体绘制+擦除组合） | collision-eco\decomp\decomp-0x140ae4d00.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::paths::draw_path::DrawState::finish`（行 43）— 生命周期控制
- `country_core::systems::paths::draw_path::DrawState::start_or_continue`（行 50）— 生命周期控制
- `country_core::systems::paths::draw_path::impl$2::bilinear_sample_at_world_xz`（行 108）— I：世界 xz 双线性采样（路径栅格；draw_path 内联）
- `country_core::systems::paths::draw_path::erase_other_path_variants`（行 429）— 删除/清理

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 14 项
