# systems/render_loop/render_world.rs

> 文件作用：帧渲染入口 `render_world`（H 层，0x1404ec110，54 个 pass 逐条调用序）。详见 `evidence/country-core-audit/B_s_h.md` H 层。

## 全局门控（render-gap §3.4，2026-10-14 补充；round3 2026-10-15 写入者/链闭合）

| 全局 | 指向 | 语义（O 地址 / I 含义） | 用法（O） |
|---|---|---|---|
| `*0x14362c0d8`（qword，.data 初值） | `0x143634da0` | `freeze_occlusion_culling` 调试开关（O：grass 以该字节为 uniform 值写 `fnv1a("freeze_occlusion_culling")=0x6d7707561ec0da2b`，round3） | 0x1404ec5b1 `mov rax,[0x14362c0d8]; cmp byte ptr [rax],0; jne skip`——非 0 跳过 `generate_wall_draw_lists_pass2`（0x14098b2a0）+ `gpu_generate_draw_lists`（0x140987dd0）。绑定方 = `do_gui_debug` closure_env$5（0x1400b68e6）字节指针传入 `egui::widgets::button::ui`（调试窗口按钮，点击处理为唯一写入路径，round3 O） |
| `*0x14362c098`（qword，.data 初值） | `0x143634d90` | 进程级 demo 开关（同 `lut_gallery`/`gpu_lut`/`vermeer`；~50 处读取，round3 穷举） | 0x1404eca6c 同形读——非 0 跳过 `refraction_copy_pass`（0x1409bf320）+ `create_raw_resource("refraction source")`（0x1429ec028，0x7a）+ `water_surface_pass`/`ice_surface_pass`（0x1409bf800/0x1409c08d0，is_winter 选变体）。写入者 = `tiny_glade::main` 0x1403dcf78（值 0，round3 O） |

- 两者都是**指针全局**（`.data` 存 qword 指针 → 指向 `.data` 零填充区字节标志），与 `CENTERED_PATH`（`*0x14362c308`→`0x143638fac`）同形。
- round3 闭合：写入者、被门控 pass 链（wall draw list 第二轮 / 水面折射链）、demo/freeze 名经 uniform 哈希（`0x6d7707561ec0da2b`）证实。详见 `round3/RENDER-U-CLOSURE.md` §1。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `render_world`（0x1404ec110）—— 54 pass 编译期注册序（O，sequence.md + CALL-CHAIN §2.3）
帧序 W1-W54（条件分支见真值表；每 pass 算法见对应 compute_passes/draw_passes 章）：

| 段 | W# | pass | 地址 | 条件 |
|---|---|---|---|---|
| 上传 | W1 | upload_world_raster ×6 | 0x1403d8bd0 系 | 无条件 |
| | W2 | water_rasterize_from_cpu | 0x140a12680 | |
| | W3 | rasterize_terrain_stroke | 0x1408c7120 | |
| | W4 | displace_rocky_terrain | 0x140a3ee80 | |
| 回读/高亮 | W5 | read_terrain_data | 0x14098c680 | |
| | W6 | mouse_read_terrain_stroke_index | 0x1409768b0 | |
| | W7 | rasterize_foliage_exclusion_mask | 0x1409cb7b0 | draw |
| | W8 | highlight_active_terrain_stroke | 0x140a58450 | |
| 植被 | W9 | object_grass_push_pass | 0x1409daaa0 | draw |
| | W10 | compute_path_masks | 0x1409014e0 | |
| | W11 | rasterize_buildings | 0x140a32770 | |
| | W12 | garden_rasterize_from_cpu | 0x140a22140 | |
| | W13 | water_spawn_vegetation | 0x140a13970 | demo 守卫 |
| | W14 | spawn_vegetation_around_buildings | 0x1408e4cc0 | |
| | W15 | read_bush_positions_for_cpu | 0x140866540 | |
| | W16 | garden_spawn_vegetation | 0x140a22b80 | |
| | W17 | path_pebbles_compute | 0x140958f70 | |
| | W18 | leaf_particles_compute | 0x1408f4d00 | |
| | W19 | roof_tiles_gravity_compute | 0x140a29b10 | |
| | W20 | stairs | 0x140a68a50 | |
| 绘制列表 1 | W21 | generate_wall_draw_lists | 0x14098aaf0 | |
| | W22 | gpu_generate_draw_lists（round0） | 0x140987dd0 | 第三参 0 |
| 可见性 1 | W23 | shadow_pass | 0x1409cc270 | draw |
| | W24 | calc_terrain_lighting_texture | 0x140a51440 | |
| | W25 | main_pass | 0x140833bc0 | draw |
| | W26 | generate_hzb | 0x1409286f0 | |
| 可见性 2 | W27 | generate_wall_draw_lists_pass2 | 0x14098b2a0 | **freeze==0** |
| | W28 | gpu_generate_draw_lists（round≠0） | 0x140987dd0 | **freeze==0**；带 hzb |
| | W29 | grass_compute | 0x140a60280 | |
| | W30 | main_pass_oc2 | 0x140835a40 | draw |
| | W31 | run_visibility_queries | 0x140a2f2e0 | |
| | W32 | calculate_dof_autofocus | 0x140a17880 | |
| | W33 | mouse_depth_compute | 0x140a01620 | |
| | W34 | late_forward_and_deferred_pass | 0x1408373b0 | draw |
| GI | W35 | extract_half_res_depth_normals | 0x1408f6180 | |
| | W36 | calculate_reprojection_map | 0x140a0d190 | |
| | W37 | calculate_ssgi | 0x1408743b0 | |
| | W38 | ray_trace_gi | 0x140a01cf0 | GI history pair 建成功 |
| | W39 | eval_gi_screen_radiance | 0x140a44cd0 | |
| | W40 | bin_lights | 0x1409e7770 | |
| | W41 | restir_sample_direct_lights | 0x140a6fec0 | 灯笼数>0 |
| | W42 | apply_screen_shadows | 0x140a39e70 | |
| | W43 | deferred_raster | 0x1409d20b0 | draw |
| 透明 | W44 | refraction_copy_pass | 0x1409bf320 | draw；**demo==0** |
| | W45 | water_surface_pass / ice_surface_pass | 0x1409bf800 / 0x1409c08d0 | **demo==0**；is_winter 选 |
| | W46 | pre_taa_transparency_pass | 0x1409c3a80 | draw |
| | W47 | compute_shadow_mask_mean | 0x140a3a6f0 | |
| | W48 | apply_volumetrics | 0x1408a44c0 | |
| | W49 | dof_prepare | 0x140a5a020 | |
| | W50 | generate_mips | 0x1400fb890 | **DoF 字节≠0** |
| | W51 | calc_dof | 0x140a5a570 | 同上 |
| | W52 | calculate_taa / calculate_tgsr | 0x140a3b2f0 / 0x140a3ba60 | **param_6[0xab]** 选择 |
| | W53 | post_taa_transparency_pass | 0x1409c40b0 | draw |
| | W54 | render_shader_refactoring | 0x14098f9f0 | |

### 2. 条件真值表（O，sequence.md C 节 + round3）
| 分支 | 判定 | 效果 |
|---|---|---|
| 整帧主体 | `cStack_154 != '\x02'` | ==2 → unwrap_failed panic（错误标签，I） |
| 第二轮回合 | `*0x14362c0d8==0`（freeze_occlusion_culling） | W27+W28 执行 |
| 折射/水面 | `*0x14362c098==0`（demo） | W44+W45 执行（demo 下不渲染水面折射） |
| DoF 主体 | `param_6[0xa9+(res_type==0x13?1:0)] != 0` | W50+W51 |
| TAA vs TGSR | `param_6[0xab]==0 / !=0` | W52 互斥选择 |
| compile 实例 | settings+0xac 模式字节 | 0x1401647b0（加载图）↔ 0x140161cd0（世界图） |
| present 跳过 | acquire 结果字节==7 | 本帧不 present |

### 3. 资源创建与闭包尾部（compile 闭包内，O）
- `create_raw_resource("gbuffer", new_2d(dims,0x65))` / `"velocity"(0x61)` / `"refraction source"(0x7a, demo==0)`；HZB 的 "main" 只是 generate_hzb 的 name 参数；
- temporal 对：`taa_history`（尺寸取 param_8.desc extent）/`tgsr_history`（尺寸取分辨率资源 +8），`new_2d(..,0x7a).sampled().storage()`；输出 "taa_output"(0x7a)；
- `is_winter`（0x140ae8320，GladeType::is_winter）选水/冰；
- 参数：param_4=DrawLists、param_5=world_render_dims（断言两维>0）、param_6=设置字节（+0x9c/+0xa9/+0xaa/+0xab）、param_7=深度、param_8=场景颜色；返回体 0x6cB={深度拷贝, dof_prepare 输出, taa 输出}；panic 行号 965/967/978/1113/1302/1143/1254。
- 闭包尾部（compile 0x140161cd0 内，CALL-CHAIN §2.2）：render_menu_bg/painterly/vermeer/logo/ui/capture → `RenderContext::finish` → export 链 → **`GpuWorld::export_from_rg`（最后一个 country_core 调用）** → rhapsody `TemporalRenderGraphRecorder::export_temporal` → `ResourceInfo::calculate` → `ResourceRegistry::populate_resources` → `CompiledRenderGraph::new`。
- U：36B `PassHandle` 内部字段；半分辨率结果两张纹理角色名；taa/tgsr 历史对 cur/prev 具体次序（I）。
