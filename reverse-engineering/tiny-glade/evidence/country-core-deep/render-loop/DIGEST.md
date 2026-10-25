# render_loop 域 DIGEST（render_loop owner agent，tg_rl1）

> 覆盖 `country_core::systems::render_loop`（78 文件 / 159 业务函数 / 48 内联 / 121 胶水；SOURCE_MAP.json 统计）。
> 反编译落盘 `render-loop/decomp/`（156 个函数，146 OK + 10 NotFound 手解）。章节：`chapters/systems/render_loop/**`（78 章全填充，占位符清零）。

## 1. 模块清单与文件作用（章节内均含 O/I/U）

| 组 | 文件 | 作用 | 关键 O 事实 |
|---|---|---|---|
| 顶层 | mod.rs | 渲染公共工具 | `RenderContext::finish`（+0x168=2 终结态，释放 0x150/0x160/0x170）；`clear_rg_buffer`/`clear_rg_color_image`（0x40B ClearBuffer/ClearImage 命令包 push）；`rg_buffer_copy_from_slice<u32>`（slice→GPU 拷贝，断言 data.len()<=buffer.desc().size） |
| 顶层 | vermeer.rs | Vermeer 风格化笔触揭示链 | `VermeerState` enum2$ 判别+0x48（0 待采集/1 已采集/2 未启用）；is_active=`+0x48!=2`；should_freeze_camera=`(≠2)&&byte0x55`；笔触数 0xa00/0x8000、MAX_DT 0.0333、REVEAL 3s、WARMUP 2 帧 |
| 顶层 | painterly/mod.rs | painterly 渐进笔触链 | `PainterlyState` 判别+8（Default=2）；has_target=`byte[+8]&1`；STROKE_COUNT 1024、1638.4/s、INSTANT_TIMER 100000 |
| 顶层 | session_selection/mod.rs | 会话缩略图渲染 | `screenshot.jpg`/`meta.json`/`assets/gladestate`/`loading_background.jpg`；纹理池断言 idx<texture_pool.len、backed_range<=THUMBNAIL_POOL_SIZE |
| 顶层 | lut_gallery.rs / menu_background.rs | 摄影 LUT 画廊 / 菜单背景 | `lut_gallery_thumbnail`；`Menu bg` + `/gauss_blur.cs.hlsl:cs_main` |
| compute_passes | gpu_generate_draw_lists.rs（13） | **Nani GPU draw-list 生成（最高价值）** | 全链 pass 族 `/nani/{clear_draw_lists,clear_histogram,prepare_unsorted_draw_lists,cull_and_bucket,histogram_prefix_sum,group_draw_cmd_prefix_sum,generate_draw_lists,fill_sorted/unsorted_instance_indirection,draw_indexed_indirect_to_dispatch,…}.cs.hlsl`；round0/round≠0 双路径（get_rg_buffer 0x1412d1180/0x1412ce810 + clear_rg_buffer）；subset 槽 world+0x1e0/+0x1450；墙两轮 `/wall/generate_draw_lists.cs.hlsl:cs_pass1|cs_pass2` + `/wall/reset_pass2_inst_counts.cs.hlsl` |
| compute_passes | 其余 39 文件 | 各 1-2 系统（pass 组合 + dispatch 链，CPU 侧） | 每文件 shader URI + 绑定/分派见章节（wp5 §2-§23 全 O 复证） |
| draw_passes | draw_lists.rs（3/70） | CPU 绘制列表 | 21 图层（POST_TAA=17…WIREFRAME=20）；`build_draw_lists` 按材质名分派到 draw_shaders；`sort` 逐层（len<0x15 插入排序否则 std sort）；**DrawLists::new 机器码与 reconstruction default() 不符（错位候选）** |
| draw_passes | main_pass / transparency / common_draw_params / shadow / deferred_raster / ui 族 / wireframe / object_grass_push / foliage_exclusion / final_post / logo / feedback / custom_cursor | 各 draw pass | G-buffer/Forward/Velocity 目标名；水/冰透明度链（`/water/{deferred_rt,ice_*,refr_resolve,…}` + 软件 BVH bind_ray_tracing）；`bind_common` 3×3 旋转矩阵构造（2x 项公式） |
| draw_shaders | 16 文件 | 各 shader draw 构造 | 均 draw_lists 分派目标；ivy leaf/flower 为 name_hash+flag 薄 thunk；clouds/stars/billboard_forest build 为共享 tail-call thunk |

## 2. 关键 O 事实（跨文件）

- **cap-first Vec**（`{cap@0,ptr@8,len@0x10}`）贯穿（playbook 事实 1）：命令队列扩容 FUN_1405627b0、sort 21 层步进 0x18。
- **PDB 名错位候选**：`DrawLists::new`（0x1409ba340）机器码为含 2 内联 Vec+2 计数器=1 的小结构初始化，与 reconstruction `DrawLists::new=default(21 空 Vec)` 不一致（PLAYBOOK 事实 7）；`grass::clone_behavior` 与 bevy `is_exclusive` 共享 0x140002370。
- **pass 注册模式**：`FUN_1416032a0(&builder, &name, len, &uri, len, world, ctx)`（ComputePass::new）；`FUN_1409dd4d0` 收尾；dispatch 尾链 `FUN_14160a2e0`/`FUN_14160afd0`。
- **枚举判别**：vermeer +0x48、painterly +8、apply_volumetrics 资源 +0x4d0 状态字节。
- **字符串表**（`render-loop/_string_table.txt`，1579 地址）：每文件 pass/shader/resource 名已解（`_per_file_strings.json`）。
- **帧序 W#**：W1 上传光栅 → W2/W3/W4 水/地形 → W5-W8 回读/高亮 → W9 object_grass → W10-W20 植被/路径/屋顶/草/树 → W21/W22 nani round0 → W23 shadow → W24 地形光照 → W25 主 pass → W26 hzb → W27/W28 nani round1 → W29 草 → W30 oc2 → W31 visquery → W32-W41 GI/DoF/SSGI/ReSTIR/屏幕阴影 → W42-W53 透明/延迟/后处理 → W54 shader_refactoring。

## 3. 与 wp5 合同/代码差异

- **draw_lists.rs DrawLists::new**：代码写 `DrawLists::new=DrawLists::default()`（21 空 Vec，`reconstruction/country-core/src/systems/render_loop/draw_passes/draw_lists.rs`），证据为 0x1409ba340 小结构初始化（内联 Vec ×2 + 计数器=1）→ **证据为 X，代码为 Y，待集成裁决**（可能地址→符号错位）。
- wp5 draw-passes 对 `DrawData::new*`（71 单态）与 draw_shaders body 记 U —— 本域维持 U（GPU 后置规则，不挖 shader 内部）。
- `session_selection` 三层索引（build/thumbnails_startup/render_session_selection_previews）与 wp5 无冲突。

## 4. 跨领域线索

- 资源：`GraphicsDeviceResource`（ui_2d mei_pass panic 串）、`ActiveSession`/`GameConfig`（gpu_generate_draw_lists panic 串）、`AssetSsboLibrary`/`AssetTextureLibrary`、`ShaderRefactoringState`（audio-misc 域）、`GradingLut`（lut_gallery/final_post）、`MouseDepthValueSsbo`/`MouseTerrainRaycastSsbo`/`CursorTerrainRaycast`（terrain-water/ui 域）、`WallColorIdSsbo`（bind_ray_tracing）。
- 回读：`ScheduledTerrainReadback`（read_terrain_data）、`BushCpuSsbo`、`DofFocusReadbackSsbo`。
- 共享原语：`inclusive_prefix_scan_u32_1m`（grass/nani）、`blur_pyramid`/`rev_blur_pyramid`（GI/nani）、`rg_buffer_copy_from_slice`（mod.rs）。

## 5. 错位/纠错

- 0x1409ba340 名 `draw_lists::new` 实行为与 21 层 DrawLists 不符的小结构初始化 → 中央裁决。
- 0x140002370 同址挂 `is_exclusive<FunctionSystem>` 与 `grass::clone_behavior` → 后者为 bevy 派生胶水。
- `apply_volumetrics`（0x1408a44c0）串含 `draw_bricks.rs` 路径（邻接数据污染），已按实际 pass 名 "Volumetrics" 判定。

## 6. U 清单（最大项）

1. **`DrawLists`/`DrawData` 字段布局**（21 层 Vec<DrawEntry> 内容、DrawEntry 定义）—— wp5 同 U。
2. **draw_shaders 各 shader 内部算法**（GPU 后置，wave-2 HLSL 域）。
3. **`DeferredRasterResources`/`FinalPostResources`/`ShapeStreamSsbo`/`TextPrimSsbo`/各 `*Ssbo` 字段布局**。
4. **handle hash → 资源名映射**（`PassResources::new_from` 7 hash）。
5. **dispatch 模式判别 2/3/4（ComputePass）/1/3/4/5/7/8（DrawBuilder）→ Rust 枚举名**。
6. **`IndirectArgsType`/`CommonDrawParams` 结构体字段**。
7. **`RenderSublistsIter` 迭代次序细节与 `GpuWorldSubsetKind::use_fine_grained_sorting` 判定**（O 名，语义 I）。
