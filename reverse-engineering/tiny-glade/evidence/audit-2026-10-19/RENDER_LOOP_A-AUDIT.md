# Render Loop A 审计：draw_passes/** 与 compute_passes/**

- 日期：2026-10-19
- 范围：`D:/game/tiny-glade/crates/country-core/src/systems/render_loop/{draw_passes,compute_passes}/**/*.rs`，共 92 个文件
- 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/render_loop/{draw_passes,compute_passes}/**/*.rs`，共 73 个文件
- 机械统计：`evidence/audit-2026-10-19/fn_audit.json`
- 性质：只读复核。未修改任何源码。
- 标注：O = 机器码/PDB 观测；I = 推断；U = 未知；“真实实现”指当前 crate 内有可执行逻辑并实际调用 pass/draw 构建 API，不等于已接入生产渲染图或已在 GPU 上执行。

## 0. 结论摘要

1. 当前 92 个文件、约 32,854 行；73 个骨架文件全部存在同名文件。机械审计范围内（130 个直接 PDB 过程）127 个同路径有名字或地址，`no_trace` 2 个、`name_only` 1 个。
2. 独立按骨架同路径复核后，另有约 13 个“内联/源码级函数”没有同名标识符：`blur_exclusion_mask`、`grass_precompute`、`logo_overlay::startup`、4 个 `DrawData::new_*`、`apply_topology`、5 个 `gpu_generate_draw_lists` 内联 helper。它们多数被合并进更大的实现（不是整段业务消失），但没有 1:1 函数；`blur_exclusion_mask`、`startup` 目前找不到等价实现。
3. 文件层面：71 个真实实现、9 个部分实现、4 个仅合同/视图、3 个仅模块声明、5 个测试/测试支撑。真正“函数级空壳”只有 `compute_water::water_level_at_xz`（直接返回 0.0，注释明确 U）；其余空函数/占位是原版分支、原版 `todo!`、测试 double 或已确认的原版 8 字节 `ret`。
4. 渲染链没有闭合：本簇完整的是 CPU 侧 pass 声明、绑定顺序、draw-list 构建和回调数据模型；`PassHost`/`PassFrame`/`RgResources`/`ComputeExec` 均无生产实现（只有测试 host），`FramePasses` 只有测试 `Rec`。`RecordedComputePass::execute` 还显式拒绝带 BDA 上传的 pass，要求走尚未安装的生产图命令所有者路径。
5. 综合完成度估计 **64%**（权重：函数覆盖 50%、链闭合 30%、非函数缺口 20%），置信度中等（约 0.65）。主要不确定性来自“内联函数是否等价合并”和视图类型字段布局。

## 1. 目录 / 子模块表

| 子目录 | 文件数 | 行数 | 生产函数 | 测试函数 | 骨架函数（fn_audit） | no_trace/name_only |
|---|---:|---:|---:|---:|---:|---:|
| `compute_passes/`（含 `mod.rs`、`pass_kit.rs`、`pass_kit/test_env.rs`） | 44 | 16,603 | 297 | 353 | 63 | 2 no_trace |
| `draw_passes/` 顶层（含 `mod.rs`、`d_main_support.rs`、`d_main_tests.rs`、callbacks/tests） | 23 | 10,016 | 361 | 63 | 32 | 1 name_only |
| `draw_passes/draw_shaders/` | 21 | 4,347 | 98 | 117 | 35 | 0 |
| `draw_passes/transparency_pass/` | 4 | 1,888 | 94 | 0 | 0（骨架拆在父文件） | 0 |
| **合计** | **92** | **32,854** | **850** | **533** | **130** | **2 / 1** |

说明：函数计数用简单的 Rust 大括号匹配；`#[cfg(test)] mod tests` 之前的算生产函数。`pass_kit/test_env.rs`、`draw_shaders/test_support.rs` 本身是测试支撑，其函数被计入“生产”列但不是业务实现。

### 文件性质汇总

| 判定 | 数量 | 文件 |
|---|---:|---|
| 真实行为实现（CPU 声明/组合层） | 71 | 见 §2 |
| 部分实现（有明确函数/接线缺口） | 9 | `compute_path_masks`、`compute_water`、`gpu_generate_draw_lists`、`grass`、`draw_callbacks_a`、`draw_callbacks_b`、`draw_lists`、`final_post`、`logo_overlay` |
| 仅数据合同 / 视图 / 标签 | 4 | `spawn_boats`、`draw_callbacks_a_views`、`material_id`、`ssbo_tags` |
| 仅模块声明 | 3 | `compute_passes/mod.rs`、`draw_passes/mod.rs`、`draw_shaders/mod.rs` |
| 测试 / 测试支撑 | 5 | `d_main_tests`、`draw_callbacks_a_tests`、`transparency_pass/surface_tests`、`draw_shaders/test_support`、`pass_kit/test_env` |

## 2. 每文件状态（92/92）

列含义：行数；“骨架”= fn_audit 的直接 PDB 过程数；“生产/测试”= 当前文件函数定义数；“判定”见上表图例。无同名骨架的文件是 crate 内拆分/测试/辅助文件。逐文件 `no_trace`：仅 `gpu_generate_draw_lists.rs` 为 2、`draw_lists.rs` 为 1（`name_only`），其余文件均为 0；记录在目录表与 §4。

### 2.1 compute_passes（44）

| 文件 | 行 | 骨架 | 生产/测试 | 判定 | 备注 |
|---|---:|---:|---:|---|---|
| `apply_screen_shadows.rs` | 379 | 2 | 4/9 | 真实实现 | 两个 pass + 两个 SSBO 合同；绑定序和 dispatch 有测试 |
| `apply_volumetrics.rs` | 153 | 1 | 2/3 | 真实实现 | 相机模式 → 尺度；单 pass |
| `bin_lights.rs` | 421 | 1 | 5/9 | 真实实现 | count==0 时两个 placeholder 缓冲是原版分支 |
| `blur_pyramid.rs` | 394 | 2 | 4/13 | 真实实现 | `rev_blur_pyramid` 同文件存在 |
| `compute_garden.rs` | 752 | 2 | 14/16 | 真实实现 | 4 个 spawn pass + blur + SSBO 构造器 |
| `compute_path_masks.rs` | 346 | 2 | 6/4 | 部分实现 | `blur_exclusion_mask` 无同名/无地址（见 §4） |
| `compute_water.rs` | 750 | 2 | 15/12 | 部分实现 | `water_level_at_xz` 返回 0.0（U，§3） |
| `displace_rocky_terrain.rs` | 128 | 1 | 2/5 | 真实实现 | 顶点数 /3 + push constants + dispatch |
| `dof.rs` | 826 | 2 | 13/8 | 真实实现 | fullres/multires 两条链均声明 |
| `dof_autofocus.rs` | 530 | 1 | 5/13 | 真实实现 | SSBO + uniform 计算 |
| `eval_gi_screen_radiance.rs` | 285 | 1 | 1/8 | 真实实现 | 三次 pass 声明 |
| `generate_hzb.rs` | 254 | 1 | 2/9 | 真实实现 | SPD/HZB 尺寸、mip 视图、间接缓冲 |
| `generate_mips.rs` | 321 | 2 | 7/12 | 真实实现 | 骨架函数名是泛型类型符号；实现为 mip pass 构造 |
| `gpu_generate_draw_lists.rs` | 1,771 | 10 | 31/25 | 部分实现 | 2 个 no_trace 单态化合并；5 个内联 helper 无同名；无生产 FramePasses |
| `grass.rs` | 731 | 2 | 9/14 | 部分实现 | `grass_precompute` 无同名；可能并入 `grass_compute_impl`（需人工） |
| `half_res.rs` | 158 | 1 | 1/3 | 真实实现 | 半分辨率深度/法线 |
| `highlight_active_terrain_stroke.rs` | 209 | 1 | 2/5 | 真实实现 | 单 pass + 索引 |
| `leaf_particles.rs` | 421 | 1 | 7/12 | 真实实现 | SSBO 构造 + compute |
| `mod.rs` | 50 | 0 | 0/0 | 仅模块声明 | 仅 `pub mod` |
| `mouse_depth_value.rs` | 136 | 1 | 2/3 | 真实实现 | 单 pass |
| `mouse_read_terrain_stroke_index.rs` | 228 | 1 | 5/7 | 真实实现 | 单 pass + SSBO |
| `pass_kit.rs` | 617 | 0 | 33/12 | 真实实现 | 非原版共用基础设施（原版内联）；PassEnv/绑定哈希 |
| `pass_kit/test_env.rs` | 659 | 0 | 49/0 | 测试支撑 | 记录型 PassFrame/Backend/World，不是生产实现 |
| `path_pebbles.rs` | 168 | 1 | 2/4 | 真实实现 | 单 pass + SSBO |
| `prefix_scan.rs` | 174 | 1 | 2/6 | 真实实现 | 三段扫描 |
| `primary_rt_debug.rs` | 78 | 1 | 1/1 | 真实实现 | 单 pass；光追绑定 |
| `rasterize_buildings.rs` | 398 | 1 | 5/10 | 真实实现 | 上传 + 多 pass 声明 |
| `rasterize_terrain_stroke.rs` | 364 | 1 | 7/3 | 真实实现 | 笔画上传 + 4 pass；`as_shader_def_value`/`hash` 为 trait/derive 级，未单列 |
| `ray_trace_gi.rs` | 334 | 1 | 2/7 | 真实实现 | 多 pass + 光追绑定 |
| `read_bush_positions_for_cpu.rs` | 224 | 1 | 3/4 | 真实实现 | 回读 pass |
| `read_terrain_data.rs` | 531 | 1 | 9/9 | 真实实现 | 4 个回读 pass + 直接光栅高度 |
| `reprojection.rs` | 187 | 1 | 1/6 | 真实实现 | 重投影 + 深度拷贝 |
| `reset_indirect_args.rs` | 376 | 1 | 6/14 | 真实实现 | 上传 + 单 pass |
| `restir_di.rs` | 213 | 1 | 1/6 | 真实实现 | lantern_count==0 的 placeholder 纹理是原版分支 |
| `roof_tiles_gravity.rs` | 383 | 1 | 5/18 | 真实实现 | push constants 多项 dispatch |
| `shader_refactoring.rs` | 496 | 1 | 4/14 | 真实实现 | 状态机 + capture/compare |
| `spawn_boats.rs` | 132 | 0 | 2/4 | 仅数据合同 | 骨架无过程，只有 `BoatSsbo`；当前仅类型/存储构造 |
| `spawn_vegetation_around_buildings.rs` | 216 | 1 | 3/6 | 真实实现 | 单 pass + 两个绑定 helper |
| `ssgi.rs` | 264 | 1 | 1/7 | 真实实现 | 3 个 pass |
| `stairs.rs` | 232 | 1 | 3/6 | 真实实现 | 单 pass + SSBO |
| `taa.rs` | 537 | 2 | 7/8 | 真实实现 | TAA + TGSR 3 pass |
| `terrain_lighting_texture.rs` | 235 | 1 | 5/5 | 真实实现 | 单 pass + 视图构造 |
| `upload_world_raster.rs` | 448 | 6 | 8/11 | 真实实现 | 格式分支的 `todo!` 是原版行为；6 个 raster kind |
| `visibility_queries.rs` | 94 | 1 | 1/2 | 真实实现 | 单 pass；SSBO 类型为泛型参数（NAME U） |

### 2.2 draw_passes 顶层（23）

| 文件 | 行 | 骨架 | 生产/测试 | 判定 | 备注 |
|---|---:|---:|---:|---|---|
| `common_draw_params.rs` | 149 | 5 | 8/2 | 真实实现 | `CommonDrawParams::new` + 擦除高亮入参；2 个骨架符号属 DrawBuilder 方法/closure，不是此文件缺口 |
| `custom_cursor_pass.rs` | 163 | 1 | 7/0 | 真实实现 | 真实 pass；`_unused` 为 dead helper（150 行） |
| `d_main_support.rs` | 301 | 0 | 14/0 | 真实实现 | 非原版共用宿主缝；`run_list_pass` 是 5 个 pass 的真实共用逻辑 |
| `d_main_tests.rs` | 1,097 | 0 | 49/0 | 测试 | 文件本身由 `#[cfg(test)] mod d_main_tests` 引入 |
| `deferred_raster.rs` | 263 | 1 | 8/0 | 真实实现 | 5 个 draw entry + stencil 状态 |
| `draw_callbacks_a.rs` | 1,168 | 0 | 88/0 | 部分实现 | 回调本体真实；大量 `*View` 视图类型 + `RenderGraph::default()` 占位（205 行） |
| `draw_callbacks_a_tests.rs` | 1,162 | 0 | 53/0 | 测试 | 断言绑定顺序/uniform 值 |
| `draw_callbacks_a_views.rs` | 157 | 0 | 0/0 | 仅合同/视图 | 18 个只含被读字段的视图/stub 类型 |
| `draw_callbacks_b.rs` | 1,620 | 0 | 45/42 | 部分实现 | 回调真实；view/stub 类型和 `RenderGraph::default()`（27 行） |
| `draw_lists.rs` | 1,785 | 4 | 29/18 | 部分实现 | 4 个 `DrawData::new_*` + `apply_topology` 无同名；2 个回调 `todo!("TODO(vulkan)")` |
| `feedback_report_bg.rs` | 74 | 1 | 1/0 | 真实实现 | 门控 + draw_no_vao(3) |
| `final_post.rs` | 382 | 2 | 10/0 | 部分实现 | final blit 完整；`msaa color out` 由调用方提供（PassHost 无 create_raw_resource，40 行） |
| `foliage_exclusion_mask.rs` | 79 | 1 | 1/0 | 真实实现 | 两条路径 + 全局脏标志 |
| `logo_overlay.rs` | 59 | 2 | 2/0 | 部分实现 | `render_logo_overlay` 真实；`LogoOverlayPlugin::build` 空体是原版 8B `ret`；`startup` 缺失 |
| `main_pass.rs` | 246 | 3 | 6/0 | 真实实现 | 6/5/3 子 pass；`with_stencil_write` 移到 `d_main_support` |
| `material_id.rs` | 51 | 0 | 0/1 | 仅数据合同 | `MaterialId` 枚举；骨架 `impl$6::fmt` 由 derive Debug 覆盖 |
| `mod.rs` | 29 | 0 | 0/0 | 仅模块声明 | 仅 `pub mod` |
| `object_grass_push.rs` | 157 | 1 | 9/0 | 真实实现 | 半径分支 + 每 Vec4 一个六顶点 draw |
| `shadow_pass.rs` | 48 | 1 | 1/0 | 真实实现 | 3 层级联列表 pass |
| `transparency_pass.rs` | 507 | 5 | 11/0 | 真实实现 | 3 个列表 pass + 水/冰表面入口 |
| `ui_2d_pass.rs` | 439 | 2 | 15/0 | 真实实现 | UI/mei pass + 文本/形状缓冲 |
| `ui_pass.rs` | 46 | 2 | 3/0 | 真实实现 | 两个列表 pass 共用 `run` |
| `wireframe_pass.rs` | 34 | 1 | 1/0 | 真实实现 | 单列表 pass |

### 2.3 draw_passes/draw_shaders（21）

| 文件 | 行 | 骨架 | 生产/测试 | 判定 | 备注 |
|---|---:|---:|---:|---|---|
| `billboard_forest.rs` | 92 | 2 | 2/3 | 真实实现 | startup 宿主缝；无生产 StartupHost 实现 |
| `clearing_shaders.rs` | 317 | 2 | 7/9 | 真实实现 | tree trunk/canopy + cull |
| `clouds.rs` | 78 | 2 | 2/2 | 真实实现 | startup 宿主缝 |
| `common_includes.rs` | 180 | 2 | 6/5 | 真实实现 | 绑定 helper |
| `common_includes_compute.rs` | 115 | 0 | 2/3 | 真实实现 | crate 内拆分，非骨架同名模块 |
| `draw_bricks.rs` | 547 | 2 | 5/20 | 真实实现 | gate/LOD 位运算 + indirect draw |
| `floor_shader.rs` | 157 | 1 | 2/5 | 真实实现 | draw builder |
| `grass_shader.rs` | 170 | 2 | 3/6 | 真实实现 | 两个 LOD |
| `instanced_critters.rs` | 356 | 5 | 7/9 | 真实实现 | 5 个 critter shader |
| `instanced_ivy.rs` | 270 | 4 | 5/7 | 真实实现 | 枝/叶/花 |
| `instanced_roof.rs` | 266 | 3 | 5/7 | 真实实现 | nani/spire/support beam |
| `leaf_particle_draw.rs` | 68 | 1 | 1/2 | 真实实现 | 单 draw |
| `mod.rs` | 29 | 0 | 0/0 | 仅模块声明 | 仅 `pub mod` |
| `path_pebble_shader.rs` | 69 | 1 | 1/2 | 真实实现 | 单 draw |
| `rocky_terrain_draw.rs` | 104 | 1 | 2/4 | 真实实现 | 单 draw + 状态 |
| `ssbo_tags.rs` | 421 | 0 | 7/7 | 仅合同/标签 | SsboType/DrawSsbo 载体与查询辅助，非渲染业务 |
| `stairs_draw.rs` | 131 | 1 | 2/4 | 真实实现 | 单 draw |
| `stars.rs` | 85 | 2 | 2/2 | 真实实现 | startup 宿主缝 |
| `startup_quad.rs` | 195 | 0 | 6/3 | 真实实现（接缝） | 三 startup 共用骨架；无生产 StartupHost |
| `test_support.rs` | 225 | 0 | 23/0 | 测试支撑 | `#[cfg(test)]` 记录型 PassHost |
| `water_planes_draw.rs` | 472 | 4 | 8/17 | 真实实现 | 水面/睡莲/花/芦苇 |

### 2.4 draw_passes/transparency_pass（4）

| 文件 | 行 | 骨架 | 生产/测试 | 判定 | 备注 |
|---|---:|---:|---:|---|---|
| `surface_ice.rs` | 295 | 0（父文件 5） | 5/0 | 真实实现 | 冰表面准备/光栅/四个 compute pass 的声明 |
| `surface_support.rs` | 514 | 0 | 42/0 | 真实实现 | 水/冰共用 SurfaceEnv/PlanOp/绑定助手 |
| `surface_tests.rs` | 948 | 0 | 43/0 | 测试 | 记录型 SurfaceEnv + 黄金序列断言 |
| `surface_water.rs` | 131 | 0 | 4/0 | 真实实现 | 水表面准备/光栅/2 个 compute pass |

## 3. 空壳与“水分”清单

### 3.1 真实函数级空壳（业务缺口）

| 位置 | 内容 | 判定 |
|---|---|---|
| `compute_passes/compute_water.rs:436-443` | `water_level_at_xz(_raster, _world_xz) -> f32 { 0.0 }`，注释明确“`WorldRaster<512>` 是 typed gap，读不出字节，仍返回 0.0（U）” | 唯一真正的业务空壳；水位双线性采样未恢复。依赖 `utils::float_grid`（他人所有、尚未恢复） |

### 3.2 原版就未实现的 `todo!`（不是本次还原遗漏，但仍未复现）

| 位置 | 内容 |
|---|---|
| `draw_passes/draw_lists.rs:406-413` | `closure_env$58/59` 原版 `todo!("TODO(vulkan)")`（normal_debug_shader / ivy_debug_lines_shader），当前保留 `panic!("not yet implemented: TODO(vulkan)")` |
| `compute_passes/upload_world_raster.rs:145` | 非 r16f/rgba16f 格式直接 `panic!("not yet implemented: Format(...)")`，与原版 `todo!` 一致 |

### 3.3 空函数体 / 只返回常量（非缺口）

| 位置 | 内容 | 说明 |
|---|---|---|
| `draw_passes/logo_overlay.rs:30` | `LogoOverlayPlugin::build(&self) {}` | 原版 0x1408a5f40 只有 8 字节 `ret`，空体是忠实还原 |
| `draw_passes/custom_cursor_pass.rs:150` | `_unused(...) {}` | `#[allow(dead_code)]` 的遗留 helper，不参与业务 |
| `compute_passes/bin_lights.rs:140-145` | `"Light indirection grid placeholder"` / `"Light indices placeholder"` | `count == 0` 时的原版占位缓冲，不是未实现 |
| `compute_passes/restir_di.rs:8-9,27,50` | `"RDI reservoirs placeholder"` 1×1 纹理 | `lantern_count == 0` 时的原版行为，不是未实现 |
| `compute_passes/pass_kit/test_env.rs:172,188` | `load_compiled_raster` / `create_raster_pipeline` → `unreachable!()` | 测试 double |
| `draw_passes/transparency_pass/surface_tests.rs:159,175` | 同上 | 测试 double |
| `draw_passes/d_main_tests.rs:492`、`transparency_pass/surface_tests.rs:356` | `raster_handle_by_name -> None` | 测试 host |
| `compute_passes/roof_tiles_gravity.rs:188-197` | 测试 `ComputeExec` 的 `false`/空体/`None` | 仅测试 |
| `compute_passes/gpu_generate_draw_lists.rs:1628,1646-1653` | 测试 `ComputeDispatchApi` 的 `track_gpu_lifetime` 空体、`unreachable!()` | 仅测试 |

### 3.4 注释标记的占位 / 待接线

| 位置 | 内容 |
|---|---|
| `compute_passes/gpu_generate_draw_lists.rs:30` | “原生产调用由 render_world 首轮及 HZB 后各一次；完整 FramePasses 场景闭包尚未安装。” |
| `draw_passes/draw_lists.rs:26-27` | 其余回调为 typed gap，调用时 panic；依赖 nani `GpuWorld`/`World` 资源访问器 |
| `draw_passes/draw_callbacks_a.rs:205` | `RenderGraph` 是 crate 内占位类型，使用一次性 `RenderGraph::default()` |
| `draw_passes/draw_callbacks_b.rs:27` | 同上（`get_rg_buffer` 命中已导入句柄时不碰它，I） |
| `draw_passes/draw_callbacks_b.rs:22,144` | `RoofHighlight`/`InstancedDrawFromPlacementData` 等类型按 PDB 名放 stub |
| `draw_passes/draw_callbacks_a_views.rs:1-13` 等 | 组件字段布局仍为 typed gap，使用只含被读字段的 `*View` |
| `draw_passes/final_post.rs:40` | `RenderGraph::create_raw_resource` 在 A4 无 API，`msaa color out` 由调用方给出 |
| `compute_passes/visibility_queries.rs:8` | 两个 SSBO 类型 `SsboType` 实现尚未恢复，NAME U，故作泛型参数 |
| `draw_passes/deferred_raster.rs:38` | `LanternSsbo` 是 typed gap，用本文件 `LanternBuffer` 替代 |

## 4. 骨架同路径函数对照与 no_trace

### 4.1 机械统计的 3 个异常

| 骨架文件 | 函数 | VA | 分类 | 独立复核 |
|---|---|---:|---|---|
| `compute_passes/gpu_generate_draw_lists.rs` | `NaniCullAndBucketBush>` | 0x140a28c60 | no_trace | 已合并：`render/nani/cull_shaders.rs` 的 `NaniCullAndBucketFamily::Bush` + `create_cull_and_bucket_pass` 泛型；无独立单态化符号。语义有表示，但不是同名函数 |
| `compute_passes/gpu_generate_draw_lists.rs` | `NaniCullAndBucketFlowers>` | 0x140a28f70 | no_trace | 同上，`NaniCullAndBucketFamily::Flowers` |
| `draw_passes/draw_lists.rs` | `fmt` | 0x1409bdc60 | name_only | 本文件无 `fmt`、无地址；属格式化/Debug 类符号，未发现业务调用点；不计业务缺口 |

### 4.2 机械审计未覆盖、但骨架同路径存在的内联/源码级函数

| 骨架文件:行 | 函数（源码行） | 当前 crate | 判定 |
|---|---|---|---|
| `compute_passes/compute_path_masks.rs:506` | `blur_exclusion_mask`（src 158） | 无同名、无地址、无等价独立函数 | **整体缺失**；其依赖包含 `ComputePass::dispatch_2d`、`read_texture`、`write_texture`、`uniform`、`bind_terrain_heightmap`。`blur_path_mask` 不是同一绑定/分支 |
| `compute_passes/grass.rs:466` | `grass_precompute`（src 132） | 无同名 | **疑似合并**进 `grass_compute_impl` 前 4 段（tile/prefix/bundle）；需人工按原指令确认是否等价 |
| `draw_passes/logo_overlay.rs:62` | `startup`（src 33） | 无同名；`startup/startup_blit.rs` 只加载 `blit_stretch_decode_srgb`，不是 `logo_overlay` | **缺失**：原函数加载 `/logo_overlay.raster.hlsl` 到 shader 库并 spawn 面片；当前只有绘制函数，没有注册 |
| `draw_passes/draw_lists.rs:1071` | `DrawData::new_main_pass`（src 189） | 无同名 | **改名/合并**：`push_row` 直接构造 `DrawItem` + `select_lists`；语义大部分存在 |
| `draw_passes/draw_lists.rs:1089` | `DrawData::new_late_forward`（src 204） | 无同名 | 同上，`PassKind::LateForward` + 列表 13 |
| `draw_passes/draw_lists.rs:1107` | `DrawData::new_main_with_velocity`（src 212） | 无同名 | 同上，`OpaqueVelocity` 分支 |
| `draw_passes/draw_lists.rs:1125` | `DrawData::new_velocity_only`（src 227） | 无同名 | 同上，`VelocityOnly` + 列表 6/11 |
| `draw_passes/draw_lists.rs:6126` | `apply_topology`（src 1452） | 无同名；仅注释提到 topology | **未确认**：当前 `DrawTopology` 直接存入 `EntityDrawParams`，由 `bodies::topology_draw` 在绘制时改写状态；是否等价需人工 |
| `compute_passes/gpu_generate_draw_lists.rs:2442` | `generate_draw_list_dispatch_args`（src 687） | 无同名 | **内联合并**：`gpu_generate_draw_lists` 的 `"Nani generate dispatch args"` pass |
| `compute_passes/gpu_generate_draw_lists.rs:2895` | `clear_draw_cmd_histogram`（src 871） | 无同名 | **内联合并**：`"Nani draw cmd histogram clear"` pass |
| `compute_passes/gpu_generate_draw_lists.rs:2926` | `clear_draw_cmd_subtotals`（src 891） | 无同名 | **内联合并**：`"Nani draw cmd subtotals clear"` pass |
| `compute_passes/gpu_generate_draw_lists.rs:3034` | `allocate_draw_lists`（src 911） | 无同名 | **内联合并**：`"Nani allocate draws"` pass |
| `compute_passes/gpu_generate_draw_lists.rs:4367` | `reset_wall_draw_indirect_args`（src 1287） | 无同名 | **内联合并**：`"Wall pass2 reset inst counts"`（`wall_draw_lists_pass2` 的 reset 部分） |
| `draw_passes/main_pass.rs:35` | `with_stencil_write`（src 22） | 同名函数在 `d_main_support.rs` | 已移动，不是缺口 |
| `draw_passes/common_draw_params.rs:361,383` | `bind_roof_color_customization` / `bind_window_color_customization_textures` | 仅注释提及；PDB 前缀在 `draw_builder` | 属 DrawBuilder 方法/closure 内联，不在本文件恢复；不是业务缺口 |
| `compute_passes/rasterize_terrain_stroke.rs:47,88` | `impl$5::hash` / `impl$0::as_shader_def_value` | `hash` 由 derive Hash 覆盖；`as_shader_def_value` 无同名 | 类型/trait 级符号；`pipeline_defines` 已覆盖 shader defs；标记“证据不足”，不当作独立业务函数 |
| `compute_passes/read_terrain_data.rs:533,574,610,646` | `read_terrain_height` / `read_terrain_rock_mask` / `read_water_mask` / `read_garden_mask` | 同文件全部存在 | 已恢复 |
| `compute_passes/blur_pyramid.rs:383` | `rev_blur_pyramid` | 同文件存在 | 已恢复 |
| `compute_passes/dof.rs:814,1037` | `calc_dof_fullres` / `calc_dof_multires` | 同文件存在 | 已恢复 |

### 4.3 结论

- 机械 `no_trace=2` 不是“整段业务消失”，而是泛型单态化被合并；`name_only=1` 无业务影响。
- 真正需要补的骨架函数缺口：`blur_exclusion_mask`（明确缺失）、`logo_overlay::startup`（明确缺失）、`grass_precompute`（疑似缺失）、`apply_topology`（需确认）。其余内联函数已有等价实现但没有 1:1 名字，交付时应登记“合并等价”而不是计入未恢复函数。

## 5. 最大 15 个文件的关键函数抽读

按行数取前 15 名（含测试文件），逐个判断“真实逻辑 / 仅签名 / 仅注释”。

| 排名 | 文件 | 行 | 抽读结论 |
|---:|---|---:|---|
| 1 | `draw_passes/draw_lists.rs` | 1,785 | **真实逻辑为主，部分缺口**。`build_draw_lists` 真正查询 ECS、按 shader 名查 85 叶分发表、`select_lists` 做 kind/variant/OC 分支、`push_row` 写 21 个列表；`DrawFnId::addr/is_recovered/func` 是完整地址表和分派。缺口：4 个 `DrawData::new_*`、`apply_topology` 无同名；`Env58/59` 是原版 `todo!`。 |
| 2 | `compute_passes/gpu_generate_draw_lists.rs` | 1,771 | **真实逻辑，部分缺口**。`gpu_generate_draw_lists` 逐段声明 clear/histogram/prefix/cull/sort/allocate/final draw，`create_cull_and_bucket_pass` 有 11 步绑定，`owned_cull_batch`/`record_external_gpu_cull`/`record_final_draw_commands` 都有实际调用序。缺口：5 个原内联 helper 无 1:1 函数；2 个单态化合并；生产 FramePasses 未安装。 |
| 3 | `draw_passes/draw_callbacks_b.rs` | 1,620 | **真实回调本体，部分类型为视图**。`env21`/`env36`/`env38`/`tree_trunk_cb`/`tree_canopy_cb`/`roof_nani_cb` 等真正读 World 资源、绑定 uniform/texture、调用 draw；但 `Weather`/`RoofFlagCloth`/`ActiveSession`/`MainCamera` 等仍是 `*View` 或 stub，`RenderGraph` 是 `default()` 占位。 |
| 4 | `draw_passes/draw_callbacks_a.rs` | 1,168 | **真实回调本体，部分类型为视图**。`bricks_cb` 实现 wall brick 门控/代理路径/擦除高亮，`read_erase_highlight` 读四个资源，ivy/stairs 等回调调用真实 shader 组合；视图类型布局来自机器码，但真实组件仍多为 `Gap`。 |
| 5 | `draw_passes/draw_callbacks_a_tests.rs` | 1,162 | **仅测试**。53 个测试函数断言绑定顺序、uniform、cull mode、失败文案；不是实现。 |
| 6 | `draw_passes/d_main_tests.rs` | 1,097 | **仅测试**。记录型 PassHost + 每个 draw pass 的序列断言；不是实现。 |
| 7 | `draw_passes/transparency_pass/surface_tests.rs` | 948 | **仅测试**。记录型 SurfaceEnv/Compute host，断言 water/ice 的黄金序列。 |
| 8 | `compute_passes/dof.rs` | 826 | **真实逻辑**。`dof_prepare` 建 CoC/tiles 两纹理并 dispatch；`calc_dof_multires` 逐步声明 7 槽前缀和、init/analyze/clear/halfres/fullres/upsample 全部 pass，含间接偏移与 uniform 上传；不是只签名。 |
| 9 | `compute_passes/compute_garden.rs` | 752 | **真实逻辑**。`garden_rasterize_from_cpu` 消费脏标志并声明 blur；`garden_spawn_vegetation_with_building_prefix` 声明 bushes/flowers/marching squares/small flowers 4 个 pass，网格尺寸、preset、间接槽位均有据。 |
| 10 | `compute_passes/compute_water.rs` | 750 | **真实逻辑 + 1 个空壳**。`water_rasterize_live`、`declare_water_blur`、`water_spawn_vegetation` 有完整 pass/绑定；`water_level_at_xz` 直接返回 0.0，是明确 U 的业务缺口。 |
| 11 | `compute_passes/grass.rs` | 731 | **真实逻辑，疑似缺一函数**。`grass_compute_impl` 声明 8 个 pass + 3 段 prefix scan，含 `AppMode` 到 brush index 的映射；`grass_precompute` 无同名，若原版是独立预扫描段则需补。 |
| 12 | `compute_passes/pass_kit/test_env.rs` | 659 | **测试支撑**。实现 `PassFrame`/`ShaderBackend`/`WorldAccess` 的记录型替身；`TestFrame` 是 crate 内唯一 `PassFrame` 实现，不能当作生产接线。 |
| 13 | `compute_passes/pass_kit.rs` | 617 | **真实基础设施**。`PassEnv`、`binding_hash`（FNV-1a `_dims` 规则）、资源读取、temporal/RG 创建、`record` 都是实际代码；它是本簇 pass 声明的公共入口，不是占位。 |
| 14 | `draw_passes/draw_shaders/draw_bricks.rs` | 547 | **真实逻辑**。`gate<LOD>` 的 mode/代理分支、`lod1_mode_bits` 的硬件移位语义、`indirect_draw_bricks_with_mode` 的绑定与 `multi_draw_indexed_indirect` 索引均为实际实现。 |
| 15 | `compute_passes/taa.rs` | 537 | **真实逻辑**。`calculate_taa` 单 pass；`calculate_tgsr` 声明 blur-lum/reproject/main 3 个 pass，aux temporal pair、history_valid 组合、inactive/undefined 内容标志都有实际代码。 |

至少 5 个具体判定已覆盖：draw_lists（真实+缺口）、gpu_generate_draw_lists（真实+合并缺口）、draw_callbacks_b（真实+视图）、dof（真实 pass 链）、compute_water（真实+空壳）、compute_garden（真实）、taa（真实）、测试文件（非实现）。

## 6. 渲染链未复现内容分层

### 6.1 Pass 执行

- **已复现（CPU 声明层）**：每个 pass 函数都实际构造 `ComputePass`/`RasterPass`，按机器码顺序声明读写、uniform、push constants、dispatch 模式；测试可回读 `RecordedComputePass`。
- **未复现（生产执行）**：`FramePasses` 只有 `render_world.rs:686` 的测试 `Rec`；没有生产实现。`gpu_generate_draw_lists.rs:30` 明确“完整 FramePasses 场景闭包尚未安装”。`RecordedComputePass::execute`（`compute_pass_builder.rs:384`）要求 `device_address_reads` 为空，带 BDA 的 pass 必须走“production graph command owner path”，该路径在本簇不存在。
- **结论**：pass 执行是“可录制的数据模型”，不是“每帧真正提交”。

### 6.2 资源绑定

- **已复现**：绑定名哈希、访问码（0x19/0x32/0x37/0x1a）、SSBO 类型/tag、纹理 preset/disk 绑定、preserve/inactive/undefined 内容意图、动态常量上传、BDA 范围（`rasterize_terrain_stroke`）都已建模；`RasterPass::render` 会把 `RasterPassSubmission` 交给 `PassHost`。
- **未复现**：生产 `PassHost` 不存在。全 crate 的 `impl PassHost for` 只有 `draw_stream.rs:779`（测试 `MockHost`）、`draw_shaders/test_support.rs:57`（测试 `RecHost`）、`resources/walls/dormers.rs:229`（局部 `LibraryHost`）、`painterly.rs:691`/`vermeer.rs:660`（测试 `Rec`）。因此 descriptor set / render pass / framebuffer 的真实提交没有落到 `rhapsody_contract::GraphicsDevice`。
- **结论**：资源 bind 是完整 CPU 数据模型，未接到真实 Vulkan 描述符/帧缓冲。

### 6.3 Draw list 生成

- **已复现**：`build_draw_lists` 有真实 ECS 查询、shader 名分发表、21 个列表选择、排序键、阴影/OC 追加、模板参考值；GPU 侧 `gpu_generate_draw_lists` 有 Nani clear/histogram/cull/sort/allocate/final draw 的完整 pass 声明和 `NaniGraphOutputs` 数据模型。
- **未复现**：CPU draw list 的 GPU 执行未接线；GPU draw list 的 pass 也未执行。`draw_lists` 的 4 个 `DrawData` 构造器和 `apply_topology` 没有 1:1 函数；2 个 debug 回调仍是原版 `todo!`。

### 6.4 Compute dispatch

- **已复现**：`dispatch_1d/2d/groups/indirect_raw/with` 都编码为 `DispatchMode`；`ComputeExecution::execute` 有 `vkCmdDispatch`/`vkCmdDispatchIndirect` 的语义实现（含 local size、屏障、计数）。
- **未复现**：生产 `ComputeExec` 只有测试实现（`compute_pass_builder.rs:2096`、`roof_tiles_gravity.rs:188` 的测试 `Exec`）。没有把 `RecordedComputePass` 接到真实命令缓冲的生产宿主。

### 6.5 GPU 实际后端调用（rhapsody）

- **已存在的真实调用面**：`rhapsody_contract::GraphicsDevice` 包装了真实 `rhapsody::device::Device`，有 `issue_full_pipeline_barrier`、`create_command_buffer`、`create_buffer` 等真实方法；`render/shader.rs:212,288` 为 `GraphicsDevice` 实现 `ShaderBackend`；`rasterize_terrain_stroke` 直接使用 `rhapsody::raw::dynamic_constants`。
- **本簇未接线**：这些后端调用没有被 `PassHost`/`FramePasses`/`ComputeExec` 的生产实现串起来。scope 内没有直接 `vkCmd*` 调用；只有数据模型和测试 double。
- **结论**：rhapsody 后端“有接口、有局部真实调用”，但 render_loop 这一簇没有形成从 pass 声明到 GPU 命令提交的闭环。

## 7. 仍未复现清单（≥10）

1. **水位采样空壳**：`compute_passes/compute_water.rs:436-443` `water_level_at_xz` 返回 0.0；`WorldRaster<512>` typed gap + `utils::float_grid` 未恢复。
2. **生产 FramePasses 缺失**：`systems/render_loop/render_world.rs:686` 只有测试 `Rec`；`compute_passes/gpu_generate_draw_lists.rs:30` 注释“完整 FramePasses 场景闭包尚未安装”。`render_world` 的 54 个 pass 调用没有生产实现。
3. **生产 PassHost 缺失**：`render/pass_builder/draw_stream.rs:420` trait；仅有 `draw_stream.rs:779`、`draw_shaders/test_support.rs:57`、`resources/walls/dormers.rs:229`、`painterly.rs:691`、`vermeer.rs:660` 等测试/局部实现。`RasterPass::render` 的提交无生产落点。
4. **生产 PassFrame/RgResources 缺失**：`compute_passes/pass_kit/test_env.rs:492` 是唯一 `impl PassFrame for`；`PassEnv::create_texture/create_buffer/temporal_*` 在生产中没有真实渲染图实现。
5. **生产 ComputeExec 缺失**：`compute_pass_builder.rs:2096`、`roof_tiles_gravity.rs:188` 只有测试 `ComputeExec`；`ComputeExecution::execute` 没有接到真实命令缓冲。
6. **`blur_exclusion_mask` 缺失**：骨架 `compute_passes/compute_path_masks.rs:506`（源码行 158，inline instances=2）；当前 crate 无同名、无地址、无等价独立函数。
7. **`logo_overlay::startup` 缺失**：骨架 `draw_passes/logo_overlay.rs:62`（源码行 33）；应加载 `/logo_overlay.raster.hlsl` 到 shader 库并 spawn，当前只有 `render_logo_overlay`。
8. **`grass_precompute` 缺失/待确认**：骨架 `compute_passes/grass.rs:466`（源码行 132）；当前无同名，可能并入 `grass_compute_impl` 前 4 段，需原指令确认。
9. **`DrawData::new_main_pass/new_late_forward/new_main_with_velocity/new_velocity_only` 无 1:1 函数**：骨架 `draw_passes/draw_lists.rs:1071/1089/1107/1125`；当前由 `push_row`/`select_lists` 合并实现，需登记等价或补名。
10. **`apply_topology` 未确认**：骨架 `draw_passes/draw_lists.rs:6126`（源码行 1452）；当前只有 `DrawTopology` 字段和 `bodies::topology_draw`，无同名函数。
11. **GPU draw-list 内联 helper 无 1:1 函数**：`generate_draw_list_dispatch_args`（骨架 2442）、`clear_draw_cmd_histogram`（2895）、`clear_draw_cmd_subtotals`（2926）、`allocate_draw_lists`（3034）、`reset_wall_draw_indirect_args`（4367）；行为已内联到对应 pass，但函数级对照缺失。
12. **NaniCullAndBucketBush/Flowers 无独立单态化**：VA 0x140a28c60 / 0x140a28f70；合并为 `NaniCullAndBucketFamily::{Bush,Flowers}` + 泛型 `create_cull_and_bucket_pass`。
13. **draw_callbacks 的组件布局仍为视图/stub**：`draw_callbacks_a_views.rs:1-13`（AlbedoMap/NormalMap/WallFlagCloth/WoodenGate/LoadingScreenProgress/Weather 等真实类型是 `(pub Gap)`）；`draw_callbacks_b.rs:22,144`（RoofHighlight/InstancedDrawFromPlacementData/Duck 等 stub）。
14. **`RenderGraph` 占位**：`draw_callbacks_a.rs:205`、`draw_callbacks_b.rs:27` 使用一次性 `RenderGraph::default()`；`rhapsody_types.rs:296` 的实现不是生产渲染图。
15. **startup 宿主未接生产**：`draw_shaders/startup_quad.rs:66-88` 定义 `StartupHost`/`AddStartupSystem`，但只有 `test_host` 实现；`billboard_forest/clouds/stars` 的 `startup_*` 没有生产 `load_mesh_into_library`/`load_shader_into_library`/spawn 调用。
16. **`final_post` 的 MSAA 目标仍需外部创建**：`draw_passes/final_post.rs:40` 明确 `PassHost` 无 `create_raw_resource`，`msaa color out` 由调用方提供；与第 3 条同源。
17. **两个 debug 回调未实现**：`draw_passes/draw_lists.rs:406-413` 的 `normal_debug_shader` / `ivy_debug_lines_shader` 保留原版 `todo!("TODO(vulkan)")`。

## 8. 完成度评估

### 8.1 权重与打分

| 维度 | 权重 | 估计 | 依据 |
|---|---:|---:|---|
| 函数覆盖 | 50% | 88% | 130 个直接 PDB 过程：127 个同路径有名字/地址；2 个 no_trace 已合并、1 个 name_only 无业务影响。独立发现约 12 个内联函数无 1:1 名字，其中 4 个为明确/疑似缺失，其余多数有合并等价。 |
| 链闭合 | 30% | 20% | 所有 pass/draw 声明与数据模型完整，但 `FramePasses`、生产 `PassHost`、生产 `PassFrame/RgResources`、生产 `ComputeExec` 全部缺失；GPU 提交未闭环。 |
| 非函数缺口 | 20% | 70% | 大部分 SSBO/纹理/preset/绑定名/格式已恢复；但 `water_level_at_xz` 空壳、组件视图/typed gap、`RenderGraph` 占位、startup host 未接生产仍在。 |
| **加权** | 100% | **64%** | `0.5×88 + 0.3×20 + 0.2×70 = 44 + 6 + 14 = 64` |

### 8.2 结论

- **本簇完成度：约 64/100**。
- **置信度：中等（约 0.65）**。
  - 高置信：文件/行数/函数数、pass 声明是否存在、机械 no_trace、测试覆盖、生产 host 缺失。
  - 中置信：内联函数是否等价合并（`grass_precompute`、`DrawData::new_*`、`apply_topology`）。
  - 低置信/需人工：视图类型字段布局是否与原组件完全一致；`NaniCullAndBucketBush/Flowers` 合并后是否覆盖全部单态化差异；`water_level_at_xz` 依赖的 `float_grid` 语义。

### 8.3 需人工/证据不足

1. `grass_precompute`、`apply_topology`、4 个 `DrawData::new_*`：需按原函数地址/调用点确认当前合并实现是否逐指令等价。
2. `draw_callbacks_a/b` 的 `*View`/stub 类型：字段偏移来自机器码，但真实组件布局仍为 `Gap`；不能宣称类型合同完成。
3. `NaniCullAndBucketBush/Flowers`：无独立单态化符号，需确认泛型版本是否覆盖原版所有绑定/dispatch 差异。
4. `water_level_at_xz`：`utils::float_grid` 属他人范围且未恢复，当前 0.0 不能作为行为等价。
5. 生产渲染图/命令所有者路径不在本簇内：本报告只能确认“未接线”，不能判断其他簇是否已有实现；需跨簇人工核对。

## 9. 复核方法

- 行数：逐文件 `splitlines` 计数。
- 函数：正则 `fn NAME` + 大括号匹配，区分 `#[cfg(test)] mod tests` 之前/之后。
- 空壳：匹配空体、单字面量返回、`todo!/unimplemented!/unreachable!`，并逐条人工读上下文。
- 骨架对照：解析 skeleton 中 `// 函数符号:` / `// 函数:` 行，先与 `fn_audit.json` 的 130 条直接过程对照，再单独列出未覆盖的 inline/enrichment 符号；对每个名字在 crate 全文做词边界搜索并检查十六进制地址字面量。
- “真实逻辑”判定：要求函数体内实际出现 `ComputePass::new`/`env.pass`/`RasterPass`/`DrawBuilder`/`bind_*`/`uniform`/`dispatch_*`/`draw_*` 等调用，且绑定/顺序/常量有机器码注释或测试支撑；只含签名、注释、常量或 `Default::default()` 的不计。
