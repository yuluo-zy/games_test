# resources/gothic_window_profile.rs

> 文件作用：**Gothic 窗/门拱剖面曲线（size 1/2/3→Vec<Vec2> 点列）。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::gothic_window_profile::gothic_door_profile_curve | 0x1409a4e70 | 0 | 0x1409a4e70：size∈{1,2,3} 分派→常量点列；越界 panic | O | audio-misc\decomp\decomp-0x1409a4e70.txt |
| country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve | 0x1409a4cc0 | 0 | 0x1409a4cc0：同构（半老虎窗剖面） | O | audio-misc\decomp\decomp-0x1409a4cc0.txt |

## 实现方式（算法级）（O 分派 + U 常量）

**gothic_door_profile_curve @0x1409a4e70 / gothic_half_dormer_profile_curve @0x1409a4cc0**：
同构——`size∈{1,2,3}` 分派 → 常量点列（Vec<Vec2> 剖面曲线），越界 size panic。（O）
- 点列常量表为独立 .rdata 区，WP21 逐 f32 转录**未重验**（U）；曲线被 window/decorator
  域消费（本域只确认函数存在与分派）。O/I 混合
