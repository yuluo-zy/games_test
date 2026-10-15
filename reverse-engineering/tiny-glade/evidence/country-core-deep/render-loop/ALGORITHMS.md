# render 域算法级汇总（ALGORITHMS.md，2026-10-15）

> 任务：把 render 顶层 37 章 + render_loop 79 章全部升级到**算法级**（处理链/算法/数据结构/详细行为/panic 分支）。
> 本文 = 帧编排链图、pass 声明→执行链、nani 实例化链、dispatch 参数表、跨模块调用 五部分汇总。
> 标注 O=机器码/反编译/哈希复算，I=推断，U=未知。GPU shader 内部算法不挖（wave-2 HLSL 覆盖）。
> 各 pass 逐函数算法见对应章节的「实现方式（算法级）」节。

---

## 1. 帧编排链图（W1-W54 顺序 + 条件）

### 1.1 帧外层 `tiny_glade::render_loop::render`（0x1404e8010，R1-R20）
```
ECS 资源获取（R1）→ DrawLists::new/build/sort（R2）→ Swapchain format/extent（R3）
→ RenderGraphState::compile（R4/R4'：settings+0xac 模式字节选 0x1401647b0 加载图 / 0x140161cd0 世界图）
→ GraphExecutor::begin_frame（R5）→ profiler retrieve（R6）
→ PassResources::new_from（R7，0x140909310，唯一数据契约）→ execute_compiled（R8，0x1400fa190）
→ retire/take_resource/from_owned_rg_resource（R9）
→ validate_swapchain_and_acquire_image（R10，结果字节==7 本帧跳过 present）
→ PassResources::new_from（R12，展示图）→ GraphExecutionRecorder::execute（R13，内部 compile 0x140166510）
→ into_present_source（R14）→ end_frame（R15）→ Swapchain::present（R16）
→ finish_screenshot_capture（R17）→ RenderStats::snapshot_and_reset（R18）→ refresh_memory_budget（R19）→ wait_gpu_queue_trim（R20）
```
编译失败 = 一次性 OOM 降级：`scale += -0.1f32` → Error 日志 → `ResourcePool::clear` → temporal 表换新 → `wait_idle_and_recycle_resources` → 降级状态 u64:=2 → `[32,32]` 兜底编译 → 降级路径不取图不 present。帧节奏 = GPU/CPU 各 4 项环 `wait = max(min(GPU)-max(CPU),0)`。

### 1.2 世界图闭包 `render_world`（0x1404ec110，W1-W54）
完整 54-pass 序表见 `chapters/systems/render_loop/render_world.rs.md`。摘要链图：

```
W1-W4  上传/程序化：upload_world_raster×6 → water_rasterize → rasterize_terrain_stroke → displace_rocky
W5-W8  回读/高亮：read_terrain_data → mouse_read_terrain_stroke_index → foliage_exclusion(draw) → highlight_stroke
W9-W20 植被/程序化：object_grass_push(draw) → path_masks → rasterize_buildings → garden_rasterize →
       water_spawn_vegetation(demo) → spawn_veg_around_buildings → read_bush → garden_spawn → path_pebbles →
       leaf_particles → roof_tiles_gravity → stairs
W21-W22 墙 draw list 第 1 轮 + gpu_generate_draw_lists(round0)
W23-W26 shadow_pass(draw) → terrain_lighting → main_pass(draw) → generate_hzb
W27-W34 第 2 轮（freeze==0）：wall_draw_lists_pass2 → gpu_generate_draw_lists(round≠0,hzb) → grass →
       main_pass_oc2(draw) → visibility_queries → dof_autofocus → mouse_depth → late_forward+deferred(draw)
W35-W43 GI：extract_half_res → reprojection → ssgi → ray_trace_gi → eval_gi_screen_radiance → bin_lights →
       restir_di → apply_screen_shadows → deferred_raster(draw)
W44-W54 透明/后处理：refraction_copy(draw,demo) → water/ice_surface(draw+compute,demo,is_winter) →
       pre_taa_transparency(draw) → shadow_mask_mean → apply_volumetrics → dof_prepare → generate_mips+calc_dof(DoF门) →
       taa/tgsr(param_6[0xab]) → post_taa_transparency(draw) → render_shader_refactoring
```

### 1.3 条件真值表（O）
| 分支 | 判定 | 效果 |
|---|---|---|
| 整帧主体 | `cStack_154 != '\x02'` | ==2 → unwrap_failed panic |
| 第 2 轮 draw lists | `*0x14362c0d8==0`（freeze_occlusion_culling，round3 闭合：do_gui_debug 按钮写入） | W27+W28 |
| 折射/水面 | `*0x14362c098==0`（demo，round3 闭合：main 写 0 恒禁用） | W44+W45 |
| 水 vs 冰 | `is_winter`（0x140ae8320，GladeType::is_winter） | W45 选变体 |
| DoF | `param_6[0xa9+(res_type==0x13?1:0)] != 0` | W50+W51 |
| TAA vs TGSR | `param_6[0xab]==0 / !=0` | W52 互斥 |
| compile 实例 | settings+0xac | 加载图/世界图 |
| present 跳过 | acquire 结果==7 | 本帧不 present |

---

## 2. pass 声明 → 执行链

### 2.1 统一声明链（O）
```
ComputePass::new<S>(name, uri)          // 0x1405876a0 系 14 实例；get_compute_by_name 缓存（着色器只编译一次）
  ├─ ShaderUri::parse（0x140968960：':'→entry 可选、'/'→文件名、'.'→stem，无点 unwrap）
  ├─ as_compute_pipeline（18 游戏实现：9 nani cull + DoF×5 + BinLights + GaussBlur + TgsrMain + GenerateStrokes；PERM_* defines）
  ├─ PassResources::new_from（0x140909310：7 个 128 位 TypeId，A/C/D/E/G/F 必选 + B 可选）
  ├─ UniformBinder::new（反射 +0xd8==0 → None；否则 DynamicConstants::allocate）
  └─ bind_color_library（无槽表，access 0x1a）
→ 绑定链（按 pass）：
  read/write_storage_buffer<T>（access 0x1a/0x32/0x37，**不经注册表**；资源号=handle+0x10；绑定数≥0x100 panic）
  read/write_rg_storage_buffer（经注册表 {set,resource}；未命中静默；assert set==0）
  bind_texture_to_*（RG 路径 = assert+add_resource_access+Ref::bind_view+维度 uniform；外部路径 = !is_write+记录 0x48B）
  uniform（UniformBinder 四道守卫：反射有效/表非空/名字哈希命中/偏移≤len；越界 slice_start_index_len_fail）
→ dispatch_*（模式 0 组数 / 1 线程总数 / 2 间接 / 3 闭包 / 4 间接+闭包）
  → RecordedComputePass::execute（0x14169af70 渲染回调：组模式 [dev+0xaba]==1 先屏障+计数+1+vkCmdDispatch；
     间接模式 "Resource does not exist"+vkCmdDispatchIndirect；闭包模式给 ComputeDispatchApi）
```
```
DrawBuilder::new（0x140905be0；0xf8B 按值消费；NoDropOnlyTake<DrawPacket> niche 2=已取走）
  → read_storage_buffer<T>（槽表 entry 去重，access 0x1f）/ bind_texture_to_draw_builder（TextureBindingKey=0x4000…|id^hash(view)）
  → bind_common（0x14090b460：Transform 3×3 旋转矩阵 2x 项公式 + shadowmap + 帧时间；CommonDrawFlag 0/1/2..=4）
  → 状态改写（state byte0/byte1/byte2/byte3 位掩码）→ draw 种类 0..8（终结操作，无返回值）
  → RasterPass::render（0x140868370：build_render_pass_desc 上限 8 → RenderPassDesc 去重缓存 → PassBuilder::render）
```
- 命令包 0x40B（render_loop/mod.rs）：`clear_rg_buffer`（"Clear buffer"）/`clear_rg_color_image`（"Clear image"）/`rg_buffer_copy_from_slice<u32>`（"Buffer memcpy"；断言 data.len()<=desc.size；加锁写 Mutex<DynamicConstants>）。

### 2.2 dispatch 参数表（O，`.rdata` 描述符实读；c1 纠错"全零向量"误读）

| pass | dispatch 形状 | 常量 |
|---|---|---|
| clear_draw_lists | dispatch_1d **108** | 固定 |
| clear_histogram | dispatch_1d **0x1b000** | 固定 |
| histogram_prefix_sum | dispatch_1d 1 | 固定 |
| draw→dispatch args | dispatch_1d **0xd800**；push `{slot, 4}` | 固定 |
| dof_autofocus | dispatch_2d **8,8** | 0x142ac7e70={1,8,8,1} |
| leaf_particles | dispatch_1d(n)，n=min(0x340) | |
| displace_rocky | dispatch_1d(首 qword /3) | |
| read_bush_positions | dispatch_1d **0x2581** | |
| path_pebbles | dispatch_1d **0x240** | |
| visibility_queries | dispatch_1d **0x60** | |
| compute_shadow_mask_mean | dispatch_1d 256 | |
| generate_hzb | dispatch_groups(ceil(w/64),ceil(h/64),1) | push 24B |
| generate_mips | dispatch_groups(ceil(w/64),ceil(h/64),1) | push 16B；mips≤13/Format 0x7a 断言 |
| grass_compute | 2×dispatch_indirect_raw（偏移 0x10/0） | 11 pass 9 缓冲 |
| water deferred rt | dispatch_indirect_raw(count,0)；args=dispatch_1d(1) | |
| roof_tiles_gravity | dispatch_with 闭包逐项 push 20B + dispatch_3d(count,1,1) | 空表也登记 |
| nani cull_and_bucket | dispatch_with 闭包 push_constants(20B)+dispatch_3d(count,1,1) | |
| prefix_scan_u32_1m | 三段 n-n/2、blocks-blocks/2、n-n/2 | assert n&0x3ff==0 |
| DoF tile offsets | 独占前缀和；间接偏移 0x60/0x40/0x50/0x30 | |
| stairs | two passes；contouring_grid_dims=(80,80)；pebble_offset=0x9c40 | |

### 2.3 访问码表（O，`is_write=(a-0x28)<0x10` @0x140c10e10）
compute 读 SSBO 0x1a / 写 0x37 / `*_storage_buffer` 与 cpu-mapped 写 0x32 / 间接参数读 0x02 / 传输写 0x34 / DrawBuilder 读 0x1f / 采样图像 0x1d / 颜色附件 0x36/0x2e / 深度 0x2f/0x17。同步组：读 1 / 写 0 / 并发写=调用方实参。

---

## 3. nani 实例化链（27 槽全链，O）

```
nani_maintain_instances（0x14206f300，删→建→改三阶段）
  ├─ 删除：RemovedComponents → Local map remove_entry（缺 unwrap）→ GpuWorld::remove_instance（swap-remove）
  ├─ 新建：NaniInstanceDesc{mesh(bit0: AtlasPart/Named), subset_kind} → MeshAtlasLibrary::get_by_name（Named 需 part.subset==kind）
  │   → extend_no_data（回调收 12B {u32,u32,kind}）→ total_instance_count+=1（无条件）→ Insert<NaniActiveInstance>
  └─ 变更：Changed<NaniInstanceDesc> → parts.len()==1 assert + kind 相等 → update_instance_mesh(+0xe u16)
20 个 nani_update_*（kind 3..0x1a）→ OwnedGpuWorldSubset::update_instance_data<T>
  （fast-path compact[index].id==key.id 否则 HashMap 回写；写 InstanceCullData{Vec3,f16 radius,u16 mesh_part} + T 数据；CAS min/max）
nani_flush_instance_updates（27 槽 stride 0xc0；仅 Owned 调 ShadowedBuffer::gpu_flush + 数据 vtable+0x30；+0x1448=+0x1440; +0x1440=0）
nani_draw（0x140904ae0，9 步）：
  list_index = kind*4 | PASS_VARIANT_TABLE[pass_variant]（0x302010000→[0,0,1,2,3]）
  → list_index_to_cmd_offset（params 第二个 Vec，越界 panic_bounds_check）
  → 3 个间接 SSBO 绑定（DrawListLength/DrawList 经 RG + InstanceIndirection 槽）
  → count_path = FORCE_FINE(0x143634d9c)||(kind<=1 && !FORCE_NO_FINE(0x143634d9d))
    → draw_indexed_indirect_count（max=MAX_DRAWS_PER_LIST[kind] 表 0x142aab288）或 _raw（cmd_slot*20）
gpu_generate_draw_lists（0x140987dd0）：round0/round≠0 双路径 + 15 个 nani pass 链 + 墙两轮（见 gpu_generate_draw_lists 章）
```
- 数据仓库：`GpuWorld` 27×0xc0 `RwLock<Option<GpuWorldSubset>>` + total_instance_count@+0x1440 + Box<NaniDrawListParams>@+0x1450 + 两 TSB（NaniMeshAtlasPartInfo@+0x1458 / NaniDrawListInfo@+0x1480）。
- 容量用尽 **warn-once 跳过（非 panic）**；`InstanceKey` 12B `{id,index,kind}`；`NaniInstanceData` vtable（type_name/as_any/gpu_flush/rg_handle/import/export/copy）。

---

## 4. 跨模块调用（核心接线）

### 4.1 country_core ↔ rhapsody 边界（CALL-CHAIN 全链）
- 编译链：`render` → `RenderGraphState::compile`（0x140161cd0 世界图）→ 闭包 `render_world`（54 pass）→ 闭包尾部 **`GpuWorld::export_from_rg`（最后 country_core 调用）** → rhapsody `TemporalRenderGraphRecorder::export_temporal`（0x1404f7ca0）→ `ResourceInfo::calculate`（0x140530430）→ `ResourceRegistry::populate_resources`（0x140be7ba0）→ `CompiledRenderGraph::new`（0x1402c4280）。
- 录制/提交链：`render` → **`PassResources::new_from`（0x140909310，唯一共享数据契约）** → `GraphExecutionRecorder::execute_compiled`（0x1400fa190）→ `BatchCompiler::flush_batch`（0x140be9600）→ `submit_pending_cbs`（0x140bebdf0）→ `ensure_active_cb`（0x140bebc90）→ `CompiledRenderGraph::record_batch`（0x1402c1d40，bevy_tasks 并行）→ `ResourceRegistry::into_exports` → `ExecutedRenderGraph::new`。
- 展示图：`GraphExecutionRecorder::execute`（0x1400fb5b0）内部 compile 0x140166510，闭包 = `execute_presentation_render_graph`（final_post/UI 链）。

### 4.2 关键资源/句柄流
- `GpuWorld` ↔ 软件 BVH：`Trimesh::build` → `RtTriangles/RtTriangleMeta/RtBvh`（SsboId=xxh3("RtTriangles"/…)) → `bind_ray_tracing`（ComputePass 不写 CPU 缓冲 / DrawBuilder 回读墙色到 CPU 缓冲）→ ray_trace_gi/water/ice/deferred_raster/primary_rt_debug。
- `ShadowedBuffer` 脏区间（fetch_min/fetch_max，两端含，exclusive_end=end+1）→ gpu_flush → nani 实例数据每帧改写→冲刷。
- `SparseInstanceBuffer`（32 实例/块分块 + 双位图 + 自由块栈）→ `allocation_to_draw_multi_instanced`（返回 (0,4,count)）→ DrawBuilder 多绘制；砖块间接参数 = 内嵌缓冲 @+0x68。
- `reset_indirect_args`（0x1e0 = 24×20B DrawIndexedIndirectCommand index_count 初值表，memcpy 0x142aaaa78，round3 闭合）→ 全间接绘制缓冲初始化。
- 回读：`ScheduledTerrainReadback` / `DofFocusReadbackSsbo` / `MouseDepthValueSsbo` / `BushCpuSsbo` / `MouseTerrainRaycastSsbo`（MappedStorageBuffer memory 4）。

### 4.3 帧级 trait 接线（现状：无生产实现）
`FramePasses`（54 个 pass 入口）/`RenderFrameSurface`（compile/begin/execute/present）/`PassHost`（raster_shader/allocate_dynamic_constants/rg_resources/vk_buffer/vk_image_view/ssbo_buffer_word）/`RgResources`（create_raw_*/temporal）——reconstruction 只有测试 Mock；`PassResources::new_from(&World)` 是 typed gap（恒 None → 走原版 unwrap panic）。见 RENDER_GAP_ANALYSIS §5.1 与 CALL-CHAIN §1.1。

---

## 5. 最大 U 项（跨模块）
1. `DrawLists::new`（0x1409ba340）机器码 = 小结构初始化，与 reconstruction `default(21 空 Vec)` 不符（PDB 名错位候选，中央裁决）。
2. `PassResources::new_from(&World)` typed gap（22+33 真名已建 TypeId 表，`typeid_resolve.py` 可闭合）。
3. `Flower03Ssbo`/`IvyFlowerSsbo`/`LilypadFlowerSsbo` 的 NAME（SsboId 立即数 O，名字编译期常量穷尽无命中）。
4. 各 `*Ssbo` 字段布局（ShapeStream/TextPrim/CustomCursor/DeferredRasterResources/FinalPostResources 等）。
5. dispatch 模式/枚举名映射（ComputePass 0..4、DrawBuilder 0..8 → Rust 枚举名）；`RenderPassBinding` 判别 2。
6. `bind_texture` 受管/FromDisk 外部绑定路径细节（标签 1/2 记录布局 O 于 B_i_api §3.2）。
7. GPU shader 内部算法（wave-2 HLSL 覆盖，不在本文范围）。
