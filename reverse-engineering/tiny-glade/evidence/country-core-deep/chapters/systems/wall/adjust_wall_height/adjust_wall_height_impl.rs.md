# systems/wall/adjust_wall_height/adjust_wall_height_impl.rs

> 文件作用：墙高命令执行：AdjustWallHeightCmd 0x30B{ty u32@+0x10, height f32@+0x14, WallId@+0x18, follow-terrain}；clamp 回退命令高度；follow-terrain 采样 TerrainHeightsData；预置形状到 _try_update_state<update_wall_state_height>（×2）+ 0x28B 事件{WallId,pos+height,u32,t,3}，否则 0x20B RequestWallMaterialAudio。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl` | 0x140ac1f20 | 46 | adjust_wall_height_impl（0x30B 命令 + clamp 回退 + 地形采样 + _try_update_state<height>×2 + 0x28B/0x20B 事件） | O | wall\decomp\decomp-0x140ac1f20.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
