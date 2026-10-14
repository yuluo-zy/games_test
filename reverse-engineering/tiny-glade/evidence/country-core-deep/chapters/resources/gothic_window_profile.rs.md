# resources/gothic_window_profile.rs

> 文件作用：**Gothic 窗/门拱剖面曲线（size 1/2/3→Vec<Vec2> 点列）。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::gothic_window_profile::gothic_door_profile_curve | 0x1409a4e70 | 0 | 0x1409a4e70：size∈{1,2,3} 分派→常量点列；越界 panic | O | audio-misc\decomp\decomp-0x1409a4e70.txt |
| country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve | 0x1409a4cc0 | 0 | 0x1409a4cc0：同构（半老虎窗剖面） | O | audio-misc\decomp\decomp-0x1409a4cc0.txt |
