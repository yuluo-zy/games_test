# startup/debug.rs

> 文件作用：**debug 启动系统：载入调试可视化/测试网格（debug 球体、decorator 测试网格、stair_step、particle_plane）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::debug::debug` | 0x1408efd40 | 15 | `debug`：载入 `assets/meshes/{decorator_delete_scribble,decorator_test,decorator_test_turret,decorator_test_door,decorator_test_cupola,decorator_test_dormer,decorator_outline,stair_step,particle_plane}.json` + debug_sphere_{white,red,blue,green,grey} 等网格。 | **O** | decomp-0x1408efd40.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。
