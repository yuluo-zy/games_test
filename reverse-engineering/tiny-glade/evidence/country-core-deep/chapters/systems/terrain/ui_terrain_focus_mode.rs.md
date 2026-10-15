# systems/terrain/ui_terrain_focus_mode.rs

> 文件作用：**地形聚焦模式 UI：进入/退出地形聚焦的状态机（167KB 反编译几乎全内联），4 个闭包（曲线采样辅助/高亮/OutlineWallCmd 状态转换）。ui-input 域共享。**

函数总计 11：业务 1 / 内联无独立符号 0 / 派生胶水 9

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode` | 0x1409a4fb0 | 0 | 聚焦模式状态机：alloc/memcpy 为主 + HashMap helper 0x140c9cde0×9；闭包 0x9a3620（FUN_140c97e10 曲线采样，0.5 参数）/0x9a8640/0x9a88c0/0x9a8b40（状态转换，OutlineWallCmd/PublicWalls 相关） | O | terrain-water\decomp\decomp-0x1409a4fb0.txt

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 9 项

## 实现方式（算法级）

`ui_terrain_focus_mode`（0x9a4fb0）：167KB 近全内联（alloc/memcpy + HashMap helper 0x140c9cde0×9）；闭包 4 个 O 地址：0x9a3620（FUN_140c97e10 曲线采样，0.5 参数）/ 0x9a8640 / 0x9a88c0 / 0x9a8b40（状态转换，OutlineWallCmd/PublicWalls 相关）。内部状态名 U（ui-input 域 owns）。
