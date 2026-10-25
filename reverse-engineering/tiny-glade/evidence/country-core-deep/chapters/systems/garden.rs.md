# systems/garden.rs

> 文件作用：**花园系统插件：build 注册 draw_garden/record_garden_history/replay 等系统与 GardenRaster 资源。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::garden::impl$0::build` | 0x140b6d290 | 26 | 插件 build：注册花园绘制/历史记录/回放系统（FunctionSystem 记录佐证：RasterReplayResources+EventWriter<GardenDrawEvent> 等） | O | terrain-water\decomp\decomp-0x140b6d290.txt
