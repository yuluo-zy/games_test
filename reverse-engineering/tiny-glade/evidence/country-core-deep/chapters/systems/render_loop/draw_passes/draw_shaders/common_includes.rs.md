# systems/render_loop/draw_passes/draw_shaders/common_includes.rs

> 文件作用：draw/compute 共用绑定 include：`bind_terrain_heightmap`、`bind_terrain_lighting`、`bind_water_object_push`（ComputePass/DrawBuilder 两单态）、`bind_garden_exclusion`。

函数总计 5：业务 3 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…common_includes::impl$2::bind_water_object_push<ComputePass>` | 0x14160e330 | 47 | 绑定水上物体推入缓冲（ComputePass 变体）：read_texture（水上物体 push buffer，供 compute 侧读） | I（名称 + 两单态结构） | render-loop\decomp\decomp-0x14160e330.txt |
| `…common_includes::impl$2::bind_water_object_push<DrawBuilder>` | 0x141631f90 | 47 | 绑定水上物体推入缓冲（DrawBuilder 变体）：read_texture（draw 侧读） | I（同型） | render-loop\decomp\decomp-0x141631f90.txt |
| `…common_includes::impl$3::bind_garden_exclusion<ComputePass>` | 0x14160e630 | 65 | 绑定花园排除掩码（ComputePass 变体）：read_texture（植被避开花园边界） | I（名称） | render-loop\decomp\decomp-0x14160e630.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `impl$0::bind_terrain_heightmap`（行 13）— 绑定地形高度纹理（全模块共享；各 compute/draw pass 复用）
- `impl$1::bind_terrain_lighting`（行 30）— 绑定地形光照纹理

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `bind_water_object_push` / `bind_garden_exclusion`（B_i_infra §2 逐条反汇编核对）
- `bind_water_object_push<ComputePass>`（0x14160e330，O 序列）：预设 0xf(0x19) → `read_access_buffer<RippleSourceSsbo>`(0x1a) → 预设 0x18(0x19)；`<DrawBuilder>`（0x141631f90）同型。
- `bind_garden_exclusion<ComputePass>`（0x14160e630，O）：预设 0xe/0x13/5/0x10 全 0x19（只读纹理序列）。
- 三份同构基础设施（c1 pass_kit / c2 c2_support / c3 pass_env）已由 i_infra 合并为**一份** `common_includes_compute.rs`；`spawn_vegetation_around_buildings`/`compute_garden`/`stairs`/`leaf_particles` 的调用方全部改用。
- 绑定名哈希：全部为 HLSL 名的 `binding_hash`（FNV-1a-64），如 `bind_terrain_heightmap` 键 0x61b08b5e0470c5f2（B_c2 §1）。
- 消费方：水/花园/地形各 compute/draw pass（`bind_terrain_heightmap`/`bind_terrain_lighting` 全模块共享）。

## 关键事实

- 这两个 include 是水/花园/地形各 pass 的公共绑定（wp5 全模块引用 bind_terrain_heightmap/lighting）。
