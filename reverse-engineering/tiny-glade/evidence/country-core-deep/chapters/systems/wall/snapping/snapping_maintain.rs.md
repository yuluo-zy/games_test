# systems/wall/snapping/snapping_maintain.rs

> 文件作用：吸附维护：snapping_maintain = mod key 切换 + NotifyHintSystemSnapping（跳表分发）；check_for_disable_snapping = mode 在 {2,3} 到写事件；snapping_signifiers 双查询画段/端预览线。

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_maintain::check_for_disable_snapping` | 0x140a6a560 | 19 | check_for_disable_snapping（mode∈{2,3} → 写事件） | O | wall\decomp\decomp-0x140a6a560.txt |
| `country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers` | 0x140a6a650 | 40 | snapping_signifiers（段/端双查询预览线） | O | wall\decomp\decomp-0x140a6a650.txt |
| `country_core::systems::wall::snapping::snapping_maintain::snapping_maintain` | 0x140a6ad00 | 93 | snapping_maintain（跳表分发 + mod key 切换 + NotifyHintSystemSnapping） | O | wall\decomp\decomp-0x140a6ad00.txt |
