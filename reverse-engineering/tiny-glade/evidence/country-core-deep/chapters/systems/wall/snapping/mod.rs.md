# systems/wall/snapping/mod.rs

> 文件作用：吸附模块根：SnappingMode{mode u8@0, snapped bool@1}（mode 1=段吸附 2=端吸附）+ +0x20 声音状态字节；is_active/position_to_snap_to。

函数总计 4：业务 1 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::SnappingMode::new` | 0x140ade610 | 90 | SnappingMode::new（11B {mode@0,snapped@1}） | O | wall\decomp\decomp-0x140ade610.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::snapping::SnappingMode::is_active`（行 67）— 谓词/校验
- `country_core::systems::wall::snapping::SnappingMode::position_to_snap_to`（行 74）— 吸附/数值

## 派生胶水（derive/宏生成；不计独立业务）

`clone` 等 1 项
