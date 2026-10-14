# systems/wall/wall_state/spatial_hash_update.rs

> 文件作用：空间哈希同步系统：OnWallChanged 到按 simplified_curve_2d 逐点 get_pos_at_coord/sample_at_coord 建段到 add_segment（缺墙 panic 'SpatialQuery received OnWallChanged with wall index'）；OnWallDeleted 到 remove_wall_id。断言 (0.0..=1.0).contains(&t)。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::spatial_hash_update::spatial_hash_update` | 0x140a8fd90 | 0 | spatial_hash_update（EventReader×2+Res<PublicWalls>+ResMut<WallSpatialHash>；simplified_curve_2d→add_segment/remove_wall_id） | O | wall\decomp\decomp-0x140a8fd90.txt |
