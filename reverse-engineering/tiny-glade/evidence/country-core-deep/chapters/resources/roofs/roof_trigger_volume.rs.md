# resources/roofs/roof_trigger_volume.rs

> 文件作用：**RoofTriggerVolume 组件（屋顶触发体/碰撞网格）**——存三角网格（triangles/normals/uvs 三个 cap-first Vec），
> 供物理交互与"从网格 UV 取世界位置"（悬停/编辑）。`set_*` 内置长度一致性断言。
> panic 串确认源码 `crates/country-core/src/resources/roofs/roof_trigger_volume.rs`。

函数总计 11：业务 7 / 内联无独立符号 1 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$2::clone_behavior` | 0x140002370 | 5 | 共享空桩 `xor eax,eax; ret`（None；跨模块去重体） | O（胶水） | 手解（rne-read --hex 0x140002370） |
| `impl$2::register_required_components` | 0x1408e6b00 | 5 | Bevy 注册胶水 | O（胶水） | roofs-rt/decomp/decomp-0x1408e6b00.txt |
| `RoofTriggerVolume::triangles` | 0x1408e4560 | 32 | 三角迭代器 `{start=ptr@+8, end=ptr@+8+len*0x24, idx}`（stride 0x24）；空表 panic "assertion failed: !self.triangles.is_empty()" | O | roofs-rt/decomp/decomp-0x1408e4560.txt |
| `RoofTriggerVolume::set_triangles` | 0x1408e45b0 | 41 | 替换 triangles：drop 旧（func_0x000140613c20）后整块拷入新 Vec（cap-first） | O | roofs-rt/decomp/decomp-0x1408e45b0.txt |
| `RoofTriggerVolume::set_normals` | 0x1408e45f0 | 45 | 替换 normals（stride 0x24）；断言 `self.triangles.len()==self.normals.len()`（UNK_142aa59e0） | O | roofs-rt/decomp/decomp-0x1408e45f0.txt |
| `RoofTriggerVolume::set_uvs` | 0x1408e4660 | 50 | 替换 uvs（stride 0x18）；断言 triangles.len()==uvs.len()（UNK_142aa5a38） | O | roofs-rt/decomp/decomp-0x1408e4660.txt |
| `RoofTriggerVolume::get_position_from_mesh_uv` | 0x1408e46d0 | 66 | 经 FUN_1416f34a0（utils position_from_mesh_uvs）由 uv+三角取位置，返回 `{found, pos}`，found=0 时输出丢弃 | O | roofs-rt/decomp/decomp-0x1408e46d0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `RoofTriggerVolume::new`（行 22）— 空构造（零化 3 个 Vec + 断言）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 3 项（triangles/get_position_from_mesh_uv 闭包 + impl$0::default 0x1408e4520）

## 布局/结构要点（已证实）

- `triangles` Vec {cap@+0x00, ptr@+0x08, len@+0x10}，stride 0x24（36B = 3×Vec3）
- `normals`   Vec {cap@+0x18, ptr@+0x20, len@+0x28}，stride 0x24
- `uvs`       Vec {cap@+0x30, ptr@+0x38, len@+0x40}，stride 0x18
- 三个 Vec 长度必须一致（set_* 断言），三角为 cap-first 游戏 Vec 布局（PLAYBOOK 事实 1）。
- 交叉域：物理交互（collision 域）与 roof uv 悬停（systems/collision/world.rs 的 collider registry）。

## 实现方式（算法级）

- `triangles()`（0x1408e4560）：`len@+0x10 == 0 → panic "assertion failed: !self.triangles.is_empty()"`（0x142aa5958, 0x2c=44 字符）；否则返回迭代器 `{start=ptr@+8, end=ptr@+8+len·0x24, idx}`（stride 0x24）。
- `set_triangles`（0x1408e45b0）：drop 旧（0x140613c20 系）→ 整块拷入新 Vec（cap-first）。
- `set_normals`（0x1408e45f0）：stride 0x24；断言 `triangles.len() == normals.len()`（0x142aa59e0）；`set_uvs`（0x1408e4660）：stride 0x18；断言 `triangles.len() == uvs.len()`（0x142aa5a38）。
- `get_position_from_mesh_uv`（0x1408e46d0）：经 FUN_1416f34a0（utils position_from_mesh_uvs）三角/uv → 位置；返回 {found, pos}，found=0 丢弃输出。
