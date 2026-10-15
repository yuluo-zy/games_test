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
## 实现方式（算法级）

### PublicWalls 结构（O，DIGEST §1.12 深化）

- 主 map HashMap<WallId, PublicWallState>（**0x2a8 步长** = 8B key + 0x2a0 value）。
- **wall_entities_map @+0x30**（delete_wall 0x140aac1e0 从 *self+0x30 remove_entry）。
- roofs map @+0x48 区（roof_entity 0x140a922b0；与 0x140a923a0 并存实例）。
- simplified_curve_2d 字段 + `assertion failed: w.simplified_curve_2d.is_valid()`（public_walls.rs:50，0x142ada193；publish/spatial_hash/clamp 三处内联）。

### 查表方法（O）

- `WallEntities::wall_entity` 0x140a91e00：hashbrown 查表 → Option<Entity>。
- `PublicWalls::roof_entity` 0x140a922b0 / 0x140a923a0：map@+0x48 区查 roof → Option<Entity>。
- `get` 0x140a91ef0 / `get_unchecked` 0x140a92010：主 map 访问（Option/unwrap）。
- `wall_ids` 0x140a1b100：主 map 遍历收集（与 Decorators::iter_removed_ids 同址复用，I）。
- `has_roof` 0x140a92100：roof map 存在性谓词。
- `get_shape_params` 0x140a92530：形状参数访问。

### 跨模块调用

PublicWalls 消费于 delete_wall 0x140aac1e0、publish_public_wall_state 0x140a8b530、spatial_hash_update 0x140a8fd90、clamp_wall_height 0x140ac7f30、wall_roof_hashmap 0x140a94cd0、wall_eraser 0x140accab0（roof_entity 平屋顶补偿）。WallEntities/WallPlasters 双 map（各 +0x0/+0x20 区，delete_wall 用 *+0x20 作 map/哈希态）供实体映射。

### 遗留 U

- WallEntities/WallPlasters 双 map 内部结构（两 map 哈希态互相引用，具体布局 U）。
- simplified_curve_2d 存储格式 U（仅断言文案证存在）。
