# systems/history/replay_raster.rs

> 文件作用：光栅回放泛型系统 replay_raster<T,R,F>（Path/C1/C2/C3/Garden/Water 六实例）：过滤前桶命令，undo 清缓冲+重绘存储 stroke，redo 画 stroke 并发绘制事件；panic “!cmd.edit.stroke.points.is_empty()”。附 RasterReplayCmd（forward/backward、stroke 切段 get_stroke_from_0_to_t、时长 duration_based_on_stroke_length）与 RasterReplayResources SystemParam。

函数总计 22：业务 9 / 内联无独立符号 13 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_raster::RasterReplayCmd::forward` | 0x141700430 | 55 | 构造前向命令：{ptr,+0x8=0,duration f32@+0xc,+0x10=2} | O | decomp-0x141700430.txt |
| `country_core::systems::history::replay_raster::RasterReplayCmd::backward` | 0x141700620 | 85 | 构造反向命令（同构） | O | decomp-0x141700620.txt |
| `country_core::systems::history::replay_raster::_::impl$0::get_param` | 0x140ae5830 | 134 | RasterReplayResources SystemParam 的 get_param | O | decomp-0x140ae5830 待补 |
| `country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >` | 0x1409fc290 | 165 | replay_raster<Path> 领域包装 | I | history\decomp\decomp-0x1409fc290.txt |
| `country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >` | 0x1409f1cf0 | 165 | replay_raster<Path> 领域包装 | I | history\decomp\decomp-0x1409f1cf0.txt |
| `country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >` | 0x1409f9930 | 165 | replay_raster<Path> 领域包装 | I | history\decomp\decomp-0x1409f9930.txt |
| `country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>` | 0x1409f6ff0 | 165 | replay_raster<Garden> 薄包装 | I | history\decomp\decomp-0x1409f6ff0.txt |
| `country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >` | 0x1409febf0 | 165 | replay_raster<Path> 领域包装 | I | history\decomp\decomp-0x1409febf0.txt |
| `country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>` | 0x1409f4650 | 165 | replay_raster<Water> 薄包装 | I | history\decomp\decomp-0x1409f4650.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_raster::impl$6::eq`（行 39）— RasterEdit 判等（derive）（派生；derive）
- `country_core::systems::history::replay_raster::impl$0::from`（行 43）— From 转换（RasterEdit 构造）（I；内联）
- `country_core::systems::history::replay_raster::RasterReplayCmd::forward_instant`（行 71）— 构造前向命令：{ptr,+0x8=0,duration f32@+0xc,+0x10=2}（O；decomp-0x141700430.txt）
- `country_core::systems::history::replay_raster::RasterReplayCmd::backward_instant`（行 101）— 构造反向命令（同构）（O；decomp-0x141700620.txt）
- `country_core::systems::history::replay_raster::RasterReplayCmd::new`（行 115）— 命令构造（duration 默认）（I；内联）
- `country_core::systems::history::replay_raster::_::impl$0::init_state`（行 134）— SystemParam init_state（I；Bevy 胶水）
- `country_core::systems::history::replay_raster::_::impl$0::validate_param`（行 134）— SystemParam validate_param（I；Bevy 胶水）
- `country_core::systems::history::replay_raster::impl$8::default`（行 145）— RasterReplayResources 默认（I；内联）
- `country_core::systems::history::replay_raster::replay_raster::closure$0`（行 200）— 命令过滤/重绘闭包（I；宿主见行号）
- `country_core::systems::history::replay_raster::replay_raster::closure$1`（行 249）— 命令过滤/重绘闭包（I；宿主见行号）
- `country_core::systems::history::replay_raster::replay_raster::closure$2`（行 342）— 命令过滤/重绘闭包（I；宿主见行号）
- `country_core::systems::history::replay_raster::get_stroke_from_0_to_t`（行 435）— 按进度 t 切取 stroke 点段（I；内联；与 0x140859740 的 fold 同族）
- `country_core::systems::history::replay_raster::duration_based_on_stroke_length`（行 472）— 按 stroke 长度算动画时长（与 TerrainEdit::duration 同公式族）（I；内联）

<!-- 未覆盖函数数：0 -->
