# resources/ivy/wall_coord.rs

> 文件作用：**墙坐标→世界空间（PublicWallState 曲线采样+法向偏移）**

函数总计 3：业务 1 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::ivy::wall_coord::WallCoord::into_world_space` | 0x140b199e0 | 0 | 转换 | O：WallCoord::into_world_space——PublicWallState 曲线采样+法向偏移 0.63*0.5（按侧±） | collision-eco\decomp\decomp-0x140b199e0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::ivy::wall_coord::WallCoord::from_wall_u`（行 24）— O：WallCoord::from_wall_u——u 参数构造（v=0）
- `country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height`（行 60）— O：墙最大藤蔓生长高度 getter（grower 引用，跨域）
