# systems/wall/shapes/circle/signifiers/edit_arrow.rs

> 文件作用：圆形编辑箭头 signifier：Query 驱动箭头位姿（edit_circle_ 前缀）。

函数总计 5：业务 3 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::signifiers::edit_arrow::impl$0::clone_behavior` | 0x140002370 | 14 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::systems::wall::shapes::circle::signifiers::edit_arrow::impl$0::register_required_components` | 0x140b35ba0 | 14 | 构造/注册/生成 | I | wall\decomp\decomp-0x140b35ba0.txt |
| `country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow` | 0x140b35120 | 18 | circle edit_arrow signifier | O | wall\decomp\decomp-0x140b35120.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
