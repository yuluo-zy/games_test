# systems/collision.rs

> 文件作用：**碰撞插件（systems/collision 模块入口）：impl$0::build 注册碰撞资源/系统（round3 O）**

函数总计 8：业务 1 / 内联无独立符号 0 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::collision::impl$0::build` | 0x140b685e0 | 27 | 系统:注册 | O：collision::impl$0::build——碰撞插件注册（round3 反编译 25KB 落盘；内部注册函数 0x140b660c0/0x140b66ff0/0x140b679c0 + bevy 注册胶水） | collision-eco\decomp\decomp-0x140b685e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`default`、`eq` 等 7 项
