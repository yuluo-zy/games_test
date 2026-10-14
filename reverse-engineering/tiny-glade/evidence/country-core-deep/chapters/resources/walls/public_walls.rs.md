# resources/walls/public_walls.rs

> 文件作用：公共墙集合：HashMap<WallId, PublicWallState>（0x2a8 步长）+ wall_entities_map@+0x30 + roofs map + simplified_curve_2d 简化 2D 曲线（publish/spatial_hash/clamp 三处断言 is_valid）；WallEntities/WallPlasters 资源。

函数总计 13：业务 8 / 内联无独立符号 0 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::public_walls::WallEntities::wall_entity` | 0x140a91e00 | 32 | WallEntities::wall_entity（hashbrown 查表 → Option<Entity>） | O | wall\decomp\decomp-0x140a91e00.txt |
| `country_core::resources::walls::public_walls::PublicWalls::get` | 0x140a91ef0 | 39 | 访问器/转换 | I | wall\decomp\decomp-0x140a91ef0.txt |
| `country_core::resources::walls::public_walls::PublicWalls::get_unchecked` | 0x140a92010 | 52 | 访问器/转换 | I | wall\decomp\decomp-0x140a92010.txt |
| `country_core::resources::walls::public_walls::PublicWalls::wall_ids` | 0x140a1b100 | 60 | PublicWalls::wall_ids | I | wall\decomp\decomp-0x140a1b100.txt |
| `country_core::resources::walls::public_walls::PublicWalls::has_roof` | 0x140a92100 | 65 | 谓词/校验 | I | wall\decomp\decomp-0x140a92100.txt |
| `country_core::resources::walls::public_walls::PublicWalls::roof_entity` | 0x140a922b0 | 73 | PublicWalls::roof_entity（map@+0x48 区查表，与 0x140a923a0 并存实例） | O | wall\decomp\decomp-0x140a922b0.txt |
| `country_core::resources::walls::public_walls::PublicWalls::roof_component` | 0x140a923a0 | 78 | PublicWalls::roof_entity（hashbrown 查表 Option<Entity>） | O | wall\decomp\decomp-0x140a923a0.txt |
| `country_core::resources::walls::public_walls::PublicWalls::get_shape_params` | 0x140a92530 | 86 | 访问器/转换 | I | wall\decomp\decomp-0x140a92530.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 5 项
