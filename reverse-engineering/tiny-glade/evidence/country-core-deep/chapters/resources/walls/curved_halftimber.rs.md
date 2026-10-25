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
## 实现方式（算法级）

### 该文件无独立业务函数（依据：sysres-procedures.json 不含 curved_halftimber.rs）

sysres-procedures.json 无 `resources/walls/curved_halftimber.rs` 键。CurvedHalfTimber 组件（含 16 段端帽掩码）与 CurvedHalfTimberDrawables 由渲染/构造系统内联消费。索引外补充确认以下方法行为：

- `CurvedHalfTimberDrawables::remove_wall_id` 0x140aa9380（O）：按 WallId 键从 drawables 集合 hashbrown **erase**（SIMD 探针 + 删除），墙删除时清理半木 drawables。
- `CurvedHalfTimberDrawables::clear`（行 14，内联）：整集清空。
- `register_required_components` 0x140aacda0（U）。

### 跨模块调用

remove_wall_id 与 delete_wall 0x140aac1e0 联动（墙实体删除路径）。

### 遗留 U

- register_required_components 0x140aacda0 组件表内容（U）。
- 16 段端帽掩码的位布局（渲染域另行恢复）。
