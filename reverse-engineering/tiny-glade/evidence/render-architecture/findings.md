# Tiny Glade 原作自建渲染管线分析

研究日期：2026-10-12。目标仅为 `D:/game/ljxsj_92385/Tiny Glade` 当前发行文件。主要机器码载体为匹配发行清单的 `tiny-glade.rne`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；反编译复用 `workspace/input/tiny-glade.exe` 分析副本和持久 Ghidra 项目。身份沿用 `evidence/binary/baseline.json`，不能把 skeleton/C 的 1.16 后端证据迁移到当前构建。

这是静态架构分析，不是完整渲染器源码恢复或 GPU 动态验收。没有执行游戏。Bevy 为分叉是用户给定约束；版本 0.16.0 有原锁文件/PDB证据，分叉仓库和 commit 未确认。

1. **结论：Bevy 提供应用与 ECS 调度，自建渲染器通过 rhapsody 直接使用 Vulkan。**

原 `build-info/Cargo.lock` 有 `bevy_app/bevy_ecs/bevy_tasks/bevy_input/bevy_time` 等，没有 `bevy_render/bevy_pbr/bevy_core_pipeline/wgpu`。`tiny-glade` 依赖 `country-core`、`rhapsody`、`shader-compiler`、`shader-pipeline` 和 `shader-variants`；私有 path package `rhapsody 0.1.0` 依赖 `ash 0.38.0+1.3.281`、`ash-window`、`gpu-allocator`、`vk-sync-fork`、`gpu-profiler`、`raw-window-handle` 等。锁文件缺少某包是该输入范围内的负证据，结合实际帧入口和 Vulkan对象函数才构成架构结论。

分层：Bevy app/ECS → country_core 游戏渲染数据与专用 pass → rhapsody 渲染图/资源/命令执行 → ash/Vulkan。`rhapsody` 有自己的 `raw::instance/device/swapchain/command_buffer/shader`，不是单一材质或 shader 扩展。

公开一手说明与此相符：开发者官网明确只列 Bevy 的 ECS、调度等用途；开发者采访指出项目早期使用 Bevy 渲染，后来 GPU 程序化生成和大量 compute 需求超过当时框架能力，转向 Vulkan。这个历史理由不能说成今天 Bevy 仍不支持 compute。

- https://pouncelight.games/tiny-glade/info/
- https://80.lv/articles/exclusive-tiny-glade-developers-discuss-bevy-proceduralism-publishers-cozy-games （开发者本人受访回答，2024-05-30）
- https://graphicsprogrammingconference.com/archive/2024/ （开发者渲染讲座简介，支持 GI、GPU-driven、ray marching、DoF 等高层事实；未把未观看视频细节当作证据）

2. **CPU 游戏状态和 GPU 场景有专用连接层。**

PDB公开符号显示 `country_core::render::nani::gpu_world::{GpuWorld,OwnedGpuWorldSubset}`、`MeshAtlas`、`TypedStorageBuffer<T>`、`ShadowedBuffer<T>::gpu_flush` 和稀疏实例缓冲区。GpuWorld有创建、移除、更新实例网格和实例数据接口；Nani系统具有不同的类型化实例数据，如 RoofTile、CurvedHalfTimber、StairsFloor、Plant等。

这支持“把建筑/植被等状态整理为专用实例数据、mesh atlas和GPU缓冲区”的模型；没有恢复每个ECS组件到GPU字段的完整映射，不能说所有实体都采用一种布局。已完成的屋顶CPU逆向另有64字节瓦实例记录证据：`evidence/roof-tiles/`；渲染帧中还调用 `roof_tiles_gravity_compute`。因此CPU生成布局和GPU动画/渲染需要分别处理。

3. **游戏 pass 先声明资源访问，rhapsody 把这些声明编译为资源和同步计划。**

PDB存在 `ComputePass::{read_rg_storage_buffer,write_rg_storage_buffer,write_storage_buffer_concurrent,dispatch_indirect_with}`、`DrawBuilder::multi_draw_indexed_indirect`、`UniformBinder` 和 `PassResources`。这些是游戏专用的pass构造接口。下层为 `RenderGraph`、`PassBuilder`、`RenderGraphState::compile` 和 `CompiledRenderGraph`。

本轮反编译直接核实的函数：

| 原作VA | PDB名称 | 已观察行为 |
|---|---|---|
| `0x140530430` | `ResourceInfo::calculate` | 调用创建标志、导出用途扩充，处理访问记录，转换image/buffer访问为usage，连接访问链并计算生命周期事件 |
| `0x140bd47d0` | `link_up_next_access_chains` | 遍历20字节访问记录，依据已有索引建立反向相邻访问链接；完整字段名字未知 |
| `0x140bd4860` | `calculate_lifetime_events` | 对符合条件的资源生成一对带pass位置和资源索引的事件，区分端点bit，再排序 |
| `0x140bece70` | `PassResourceAccessType::can_be_concurrent_with` | 比较访问类别；部分类别还要求非零且一致的同步组值。访问类别枚举值尚未逐一命名 |
| `0x1402c3d00` | `CompiledRenderGraph::add_pre_pass_barriers` | 获取当前/先前访问，调用兼容性判断；不兼容时分别构造ImageBarrier和BufferBarrier |

原作同步判断不是“每两个pass之间无条件放屏障”。它查看具体资源先前如何被访问、下一次如何被访问，然后构造相应屏障。生命周期分析加 ResourcePool 的take/insert接口支持资源复用体系。**尚未证明同一帧内不同资源共用一段显存、完整拓扑排序算法、自动pass消除或多队列实际并发，不将这些常见render graph能力自动归给它。**

概念例子（解释用途，不是原源码）：compute写实例缓冲 → 图登记storage写 → draw登记实例/间接参数读 → 编译器建立访问链与同步 → 命令录制后供GPU执行。实际枚举和所有屏障参数仍需逐项恢复。

4. **原作的帧工作包含程序化生成、GPU剔除、光栅绘制和计算光照。**

`tiny_glade::render_loop::render_world` 位于 `0x1404ec110`。下表概括该函数反编译中可确认的构图调用段落；这是CPU构图关系，不是完整动态GPU时间线。分支条件、资源依赖、循环和间接调用影响某一帧的实际执行。

| 构图段落 | 已核实的调用 |
|---|---|
| 场景数据与程序化更新 | upload_world_raster；water_rasterize_from_cpu；rasterize_terrain_stroke；displace_rocky_terrain；compute_path_masks；rasterize_buildings；garden_spawn_vegetation；path_pebbles；leaf_particles；roof_tiles_gravity_compute；stairs |
| 第一轮可见性与绘制 | generate_wall_draw_lists；gpu_generate_draw_lists；shadow_pass；calc_terrain_lighting_texture；main_pass |
| 当前深度后的补充阶段 | generate_hzb；受条件控制的generate_wall_draw_lists_pass2/gpu_generate_draw_lists；grass_compute；main_pass_oc2；visibility queries、autofocus和mouse depth；late_forward_and_deferred_pass |
| 光照输入与求解 | extract_half_res_depth_normals；calculate_reprojection_map；calculate_ssgi；ray_trace_gi；eval_gi_screen_radiance；bin_lights；restir_sample_direct_lights；apply_screen_shadows；deferred_raster |
| 透明与图像处理 | 条件控制的refraction copy和水/冰绘制；pre_taa_transparency_pass；shadow mask mean；apply_volumetrics；dof_prepare；条件控制的generate_mips/calc_dof；calculate_taa或calculate_tgsr；post_taa_transparency_pass |

**TAA与TGSR在所见分支上是选择关系；不能把二者画成每帧无条件串行运行。DoF主体也存在开关分支。** 主绘制、late forward/deferred和deferred raster同时存在，故应称有混合阶段的专用管线，不能简化成全部物件统一走纯forward或纯deferred。

`RenderGraphState::compile`的此编译实例 `0x140161cd0` 含有游戏构图闭包被单态化/内联后的代码：调用render_world、后处理、UI、RenderContext::finish、export_temporal，随后 ResourceInfo::calculate → ResourceRegistry::populate_resources → CompiledRenderGraph::new。这不意味着通用rhapsody源码本来直接依赖所有游戏pass。

5. **大量小物件的提交通过GPU实例筛选和分桶组织。**

`gpu_generate_draw_lists` 的原作入口 `0x140987dd0` 创建GPU-only临时缓冲，清零图缓冲，读取游戏状态并构造 `create_cull_and_bucket_pass`。帧入口有HZB之前/之后两轮相关调用。

对原 `compiled-shaders` 的SPIR-V解析进一步确认：

- `_nani_cull_and_bucket...` 为compute入口，workgroup `64×1×1`。
- 通用shader变体声明 `atlas_part_info`、filtered instance/allocator/meta、histogram、draw_list_info、prev/next visibility bitmask；其中一个变体还有 `hzb_tex`，另一个没有。
- 瓦片专用变体声明 `instance_buffer`；不能把通用变体和瓦片变体的取数方式等同。
- `_nani_draw_indexed_indirect_to_dispatch` 是 `1×1×1` compute，声明 `DrawIndexedIndirectCommand` 载体、`draw_buffer` 和 `args_buffer`。
- PDB还确认 `DrawBuilder::multi_draw_indexed_indirect`、dispatch_indirect接口，以及实例mesh atlas。

结合这些证据，可确认它具有GPU-driven实例筛选/分桶和间接绘制基础设施。工程意义是把大量砖瓦/植被实例的可见性和提交组织放在GPU批量处理，避免CPU逐物件发起绘制。**准确LOD选择规则、分桶算法、两轮剔除对新显露物件的完整策略及真实draw-call数量尚未恢复。**

6. **GI与跨帧处理是渲染图里的实质算法。**

半分辨率depth/normal提取shader是 `8×8×1` compute，声明depth、G-buffer、stencil输入及depth/normal输出。重投影shader声明当前depth、上一帧depth、velocity和输出图。

GI候选shader `_gi_trace_gi_candidates` 为 `8×8×1` 普通compute；声明 `rt_bl_bvh`、`PackedBlBvhNode`、`rt_triangles`、`rt_triangle_meta`，还声明terrain heightmap、terrain lighting、shadow map、blue noise、TAA历史、重投影、半分辨率depth/normal与candidate输出。此模块未声明RayTracing/RayQuery能力，opcode histogram无对应硬件光追操作。**软件BVH/三角形追踪是这些声明的强推断，尚未恢复完整遍历/采样算法；结论仅限这个模块，不能扩展成全程序没有硬件光追路径。** PDB另有 `build_gpu_bvh` 和 `bvh_builder_worker`，目前为符号观察。

`_gi_update_gi_radiance`声明candidate输入和 `restir_radiance_out_tex/restir_ray_dir_out_tex/restir_reservoir_out_tex`。GI evaluation声明对应radiance/ray_dir/reservoir与SSGI输入；重投影和blur阶段有history、normal、depth与temporal stats。帧入口亦调用独立的 `restir_sample_direct_lights`。这支持有候选/蓄水池、时间重用与滤波的GI/直接光照方案，**尚不能确定ReSTIR变体、权重、偏差修正或具体论文实现**。

`TemporalRenderGraphRecorder::get_or_create_temporal_pair/export_temporal`与history读写shader声明共同说明跨帧资源由专用图层管理。TAA/TGSR、GI及可见性会使用跨帧数据，render graph并非每帧只创建一套完全独立的临时纹理。缓存reset规则、相机突变处理、缓冲数量与帧延迟仍未知。

7. **图的执行和最终呈现也由原作控制。**

`GraphExecutionRecorder::execute_compiled` (`0x1400fa190`) 实际调用 BatchCompiler::new/flush_batch、submit_pending_cbs、ensure_active_cb、CompiledRenderGraph::record_batch，最后导出资源并创建 ExecutedRenderGraph。

帧外层 `tiny_glade::render_loop::render` (`0x1404e8010`) 有如下CPU路径：begin_frame → execute_compiled → retire/take exported resource → validate_swapchain_and_acquire_image → 构造并execute最终图 → end_frame → Swapchain::present。获取交换链图像出现在已观察的主体图执行调用之后，所以不能概括成所有场景工作都必须先acquire交换链再开始。是否与GPU执行重叠、具体等待信号和提交队列还需动态证据。

PDB确认ResourcePool、ManagedResourceStore、CommandBufferDisposalBuckets、SequentialDisposal、timeline semaphore及queue pending submission/reclaim等设施。存在显存预算刷新与根据显存估算render scale的函数。它确实接管资源创建、生命周期、shader/pipeline、命令录制、提交和呈现；资源释放安全性的完整证明和具体帧在途数量未完成。

8. **自建的动机与恢复时的边界。**

它的特点是建筑生成、草木生成、可见性、物件动画、光照和图像处理共享一套GPU资源与调度机制。程序化系统输出的不仅是最终mesh，还可能是供后续compute和raster pass共同使用的状态/缓冲。这个需求解释了为什么开发者需要控制从资产生成到最后呈现的整条流水线。

若后续恢复实现，应继续直接使用原版ash、gpu-allocator、Bevy、glam等依赖；恢复的是rhapsody的私有图框架、游戏GPU数据布局、专用pass与shader算法。框架骨架已定位，但完整资源版本图、descriptor实际句柄、格式/extent、队列所有权和shader算法尚未完整核实。此次请求完成架构分析，不把项目M12标成已验收，也不把这些反编译伪代码称为作者原Rust源码。

原始与派生证据：

- `dependencies.json`：原作选定依赖条目。
- `public-symbols.json`：LLVM公开符号与demangle，保留同地址别名；`symbol-seeds.json`为相关种子。
- `mcp-report.json`、`mcp-second-report.json`：两次实际RE-MCP连接、持久数据库、10个函数反编译和引用结果；首次create_function错误是函数已存在，随后decompile成功，未删除数据库。
- `0x*.c`：从工具返回保留的Ghidra伪代码，只规范空行，不修造ABI或类型。
- `resolved-calls.json`：伪代码内函数名称与PDB公开地址映射；不等同完整机器码callsite或动态调用图。
- `shader-reflection.json`：20个原shader文件的SPIR-V入口、workgroup、能力、绑定声明、opcode计数和哈希；不证明具体帧绑定了哪个变体。

问题验收：Bevy/自建边界已回答；自建框架组成与关键调用已回答；帧构图与GPU-driven机制有静态证据；完整GPU依赖/执行时序、shader算法和运行画面未恢复。
