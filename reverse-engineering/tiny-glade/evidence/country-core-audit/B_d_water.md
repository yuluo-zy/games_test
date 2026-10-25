# B 阶段 d_water：水 / 冰表面 pass 的真实实现

范围：`systems/render_loop/draw_passes/transparency_pass.rs`（拥有）+ 新目录 `transparency_pass/`（`surface_support.rs`、`surface_water.rs`、`surface_ice.rs`、`surface_tests.rs`）。
没动 `draw_lists.rs`、`draw_shaders/`、其它 pass 文件。O = 机器码观测，I = 推断，U = 未知。未 commit。
验证：`/tmp/h_d_gap2` 副本（已删）全树 harness → **583 passed; 0 failed**（含 d_main 的 `transparency_passes_names_lists_and_state`、`water_and_ice_plans_are_selected_by_season`，新增 12 个 surface 测试）。五个文件已 `rustfmt --edition 2024 --config max_width=120`，无 `todo!`。

## 1. 完成了什么
| 地址 | Rust | 状态 |
|---|---|---|
| 0x1409bf800 `water_surface_pass`（4292 B）| `surface_water::water_surface_pass(ctx, env, &SurfaceInputs)` | **O 全部接线**：2 个 RG 缓冲 + `clear_rg_buffer`、`Water plane` 栅格 pass（列表 15）、2 个 `ComputePass`（args = `dispatch_1d(1)`、rt = `dispatch_indirect_raw(count,0)`）|
| 0x1409c08d0 `ice_surface_pass`（12711 B）| `surface_ice::ice_surface_pass(...)` | **O 全部接线**：3 纹理 + 4 缓冲（2 个 `rg_buffer_copy_from_slice<u32>`）、`Ice shadows` temporal pair、5 颜色附件的 `Ice plane`、半分辨率 2 纹理 + `clear_rg_color_image`、4 个 `ComputePass`、2 个 temporal pair |

`WATER_OPS` / `ICE_OPS` 保留为黄金序列：`SurfaceEnv::trace` 在每个真实调用的助手里记一个 `PlanOp`（调用不经助手就不会发生），测试断言一个绘制项时 trace **逐项 ==** 两张表；另有 `extra` 记录表词汇之外的步骤。空列表时 trace == 表去掉 per-item 段。
黄金表唯一改动：`ICE_OPS` 里 4 个 `key: None`（march 的 2 个 `WriteRgBuffer`、trace 的 2 个 `ReadRgBuffer`）补上 `Some(0xf0512b889439e93e / 0x06541d6cfdf253ab)`（`ice_rt_count_buffer` / `ice_rt_ray_buffer`，原静态提取漏了寄存器里的哈希）。长度不变。

## 2. 关键机器码事实（O）
- **绑定名全部解出**：重建 HLSL 的所有标识符做 FNV-1a 64 → 反查全部 30 个立即数（`names` 模块，测试里逐个重算）。`A`=`half_depth_tex`、`B`=`taa_history_tex`、`C`=`refraction_tex`、`D`=`reprojection_tex`、u8=`history_valid`（栈参顺序 A,B,u8,C,D,lists；与 `render_world.rs` 的 half/history/refraction/reprojection 对上）。
- 水：`bind_common` 的 flag 恒 false（`[rbp+0xb84]=0`），u8 参数只作 `history_valid` uniform；`Time` → `CommonDrawParams.time`。栅格每项：A,B,history_valid,D,write_rg(count),write_rg(ray),C；`state.byte3=(b3&0x9f)|0x40`（**没有** `byte0&=0xbf`，与 pre/post-TAA 不同）。
- 水 args pass 的 `PassBuilder::render` 环境是 `disc=1,(1,1,1)` = `dispatch_1d(1)`：常量 0x142958fe0 是四个 u32 `1`，反汇编器标成 float `[0,0,0,0]` 是误标（d_gap 若按“全零”解读需更正）。
- 冰 5 个颜色：`[tag0 color, tag1 roughness(视图字=四个 1.0f32), tag1 refr hits(0), **tag2** refr hit radiance, tag1 Ice shadows 当前帧(0)]`；上一帧 → `shadows_history_tex`。`history_valid`（栅格）= `u8!=0 ? pair标志 : 0`；march/trace = u8 原值；refl resolve = `(flag1 & flag2) & 1`。
- 冰计数缓冲初值 `[0,1,1,0]`（rodata 0x142abfce4；水的 count 用 `clear_rg_buffer` 清零）。射线尺寸：`n=w*h`(32 位) → 水 ray=`(n+(n>>4))<<4`；冰 px=`4*(n+(n>>4))`、ray=`(n+(n>>4))<<4`；均用 A 的 extent。
- 格式码：roughness 9、refr hits/radiance 0x62、Ice shadows 9(+sampled)、water refl hits 0x53、water refl hit radiance 0x62(半分辨率)、water refr temporal 0x7a(+sampled+storage)、ice refl ray len 0x53(+sampled+storage)。半分辨率 = `half_res(color.desc)`（O 0x140bd5730：`max(v-v/2,1)`）。
- 两个 resolve 是 `dispatch_2d(color 的 extent)`；两处 `*_hit_radiance_tex` 以 `Some(TextureViewDesc{format 0x7b, usage 4})` 绑定。blue noise 在 march/trace/水 rt 里**各绑两次**（机器码如此）。
- 磁盘纹理键 `0x579f39ab91a01865 = xxh3("blue_noise")`（B_c3.md 已证）；预设号：9 terrain_heightmap、0xa visual_terrain_heightmap、8 terrain_lighting_texture、0 brdf_fg_lut、0x11 shadow_map。

## 3. 公共 API 变化（旧 → 新）
- `transparency_pass::water_surface_pass() -> &SurfacePlan` / `ice_surface_pass()` → **改名** `water_surface_plan()` / `ice_surface_plan()`（`surface_plan_for` 沿用）；`water_surface_pass` / `ice_surface_pass` 现在是真实 pass：`fn(&mut DrawPassCtx, &mut SurfaceEnv, &SurfaceInputs)`。grep：只有 d_main_tests 引用表/`surface_plan_for`，无调用者受影响。
- 新类型（`transparency_pass` 再导出）：`SurfaceInputs{color,depth,half_depth,taa_history,history_valid:u8,refraction,reprojection,lists}`（全是 `&RgTexture`）、`SurfaceEnv{rg:&mut dyn RgResources, graph:&mut dyn RenderGraph(rg_helpers), compute:&mut dyn SurfaceCompute, trace, extra}`、`trait SurfaceCompute{new_pass(name,uri)->ComputePass; record(RecordedComputePass)}`（ComputePassInit 的三接缝由调用方持有）。
- 为集成者：`render_world.rs` 的 `PassHost`（`water_surface_pass(color, depth, half, history, history_fresh, refraction, reprojection)`）实现里，把 `half.depth→half_depth`、`pair.prev→taa_history`、`history_fresh as u8→history_valid` 接到这里（`HalfResTargets` 的哪个字段是 `half_depth_tex` 我没核，I）。
- 新增 `#[path]` 子模块声明写在 `transparency_pass.rs` 里（解析到 `draw_passes/transparency_pass/*.rs`），**不需要改 `mod.rs`**。

## 4. 仍是 U / 缺口（不算完成）
1. `TextureDesc::sampled_with_different_format`（0x140bd5410：`first_qword |= 0x10800000004`，即 usage|=4 且 dword@+4 |= 0x108）与 `color_attachment()`（usage |= 0x10，用于 `Ice shadows`）在 `rhapsody_types::TextureDesc` 里无字段。现在 `sampled_with_different_format` 只设 `sampled`，`color_attachment` 缺省。**请求（共享文件，追加）**：给 `TextureDesc` 加 `color_attachment: bool` 与 `view_format_flags: u32`（或等价），并让 `surface_support::sampled_with_different_format` / `surface_ice` 使用。
2. 两个计数缓冲的初值 `[0,1,1,0]` 的语义（I：间接 dispatch 参数 x=计数,y=z=1）。
3. `RgResources::create_raw_*` / temporal 返回的 `TemporalTexturePair.fresh` 与原版“标志字节”的对应（I：`flag byte ≈ fresh`，`&` 与 `&1` 的使用是 O）。
4. `DrawPassCtx` 没有“水/冰”专用 ItemMode，本模块自带循环 `run_surface_raster`（与 `run_list_pass` 相比：无 `byte0&=0xbf`、5 色/自定义视图字、不通过 `ListPass`）；若 d_main_support 的 owner 愿意，可把它并入 `ItemMode`。
5. `ice` 里 `Ice plane` 在 `ICE_RESOURCE_NAMES` 第 13 项是 pass 名而非资源（表原样保留）。
6. 栅格 uniform 值（history_valid 等）在 `MockHost` 提交记录里不可观测（只有 `UniformBuffer{size}`），测试对栅格侧只断言资源访问/顺序；计算侧的 `history_valid` uniform 值已断言。
7. 未验证：真实 crate 整体编译（其它 agent 并发修改中）；`rhapsody` 真 `RenderGraph` / `ComputePassInit` 的接线由集成者提供 `SurfaceCompute` 实现。
