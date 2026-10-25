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

## 实现方式（算法级，2026-10-15 深挖）

- **debug 0x1408efd40（36KB，O）**：**21x 名式网格（FUN_140b1aca0）+ 14x 文件式（FUN_140b0eeb0）**，
  共 35 次网格载入（章节旧述 9 个为欠计）+ 8x FUN_140a07680 + 5x FUN_140a06b00（调试网格后处理/记录）。
  已证名：decorator_trigger_volume、cube、cube_grey、stair_step（0x142aa78d0 区）、leaf_particle_mesh、
  leaf_particle、texture_test_shader、debug_sphere_{white,red,blue,green,grey}（0x142aa79b5 区）、
  particle_plane（'assets/meshes/particle_plane.json' 0x142aa7914）、'assets/meshes/stair_step.json'（0x142aa7895）。
  15x unwrap panic（0x142aa6a30/0x142aa6a10 区）。
