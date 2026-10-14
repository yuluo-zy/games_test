# resources/walls/curved_halftimber.rs

> 文件作用：弯曲半木墙组件（CurvedHalfTimber，含 16 段端帽掩码）与 drawable 集合资源；渲染域消费，无独立业务函数。

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables::remove_wall_id` | 0x140aa9380 | 19 | 删除/清理 | I | wall\decomp\decomp-0x140aa9380.txt |
| `country_core::resources::walls::curved_halftimber::impl$2::register_required_components` | 0x140aacda0 | 44 | 构造/注册/生成 | U |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables::clear`（行 14）— 删除/清理
