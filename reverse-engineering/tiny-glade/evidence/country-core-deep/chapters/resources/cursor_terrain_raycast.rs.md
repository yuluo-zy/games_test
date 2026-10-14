# resources/cursor_terrain_raycast.rs

> 文件作用：**光标地形射线资源：{tag u8@0, pos Vec3@4}，Option<Vec3> 语义。地形类工具（water/garden/path/terrain）读取光标落点。**

函数总计 3：业务 1 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position` | 0x140a12650 | 35 | tag&1 非零返回 pos（+4/+0xc），否则 {0,0,0} | O | terrain-water/DIGEST.md

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::update`（行 20）— 由射线更新写入命中位置（Some→tag=1+pos；None→tag=0）
- `country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::new`（行 39）— 构造空资源（tag=0, pos=0）
