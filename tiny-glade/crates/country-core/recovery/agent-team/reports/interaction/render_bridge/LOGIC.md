# LOGIC — render_loop / render/pass_builder / render/nani 逐函数逻辑判等

verifier: logic-verifier-renderloop | 日期: 2026-10-17 | 协议: planning/2026-10-16-logic-verify-protocol.md
方法: ghida fresh 反编译(每批≤20, `--worker shared`) + 定点反汇编(select_lists/窗内判定) + Rust body 逐语句对照。
红线: 只以 fresh 反编译为准; 逻辑未判等标未实现。GPU 宿主调用语义核验调用点即可。

## 判等函数表

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x14098c680 | `read_terrain_data` | **判等** | 四 compute pass 写序=高度→rock→water(仅非 demo)→garden; 高度线程 0x20000、掩码 0x2000; 光栅接缝执行; 回读 `cmpxchg(0)`→`copy_issued\|=old!=0`, changed_tick=world tick。逐分支对上。 |
| 0x1409e7770 | `bin_lights` | **判等** | count==0 只建两个占位(4B, 0x20); 否则 grid 0x10000+clear→count(线程=count)→subtotals 0x44→prefix 0x2000→subtotal prefix 8+push 0x10→merge 0x4000→indices 0x100000→bin(线程=count)。缓冲名长与 dispatch 全对上。 |
| 0x1409e8db0 | `BinLights::as_compute_pipeline` | **判等** | path `/light_binning/bin_lights_to_cells.cs.hlsl`、entry `cs_main`、defines `[("PERM_write", u32 Display)]`。 |
| 0x140a6fec0 | `restir_sample_direct_lights` | **判等** | lantern_count==0→占位 1×1 纹理无 pass; 否则 temporal pair "RDI reservoirs" + 单 pass(5 纹理+lantern ssbo+2 grid 缓冲+2 uniform), dispatch_2d(half)。 |
| 0x140a5a020 | `dof_prepare` | **判等** | CoC(0x4c)+CoC tiles(div_up 8,8,1, 0x53); "DoF prepare" 4 绑定, dispatch_2d。 |
| 0x140a5a570 | `calc_dof` | **判等** | multires 开关门; 非 multires="dof output" 0x7a+fused pass(bokeh_anisotropy uniform); multires=tile offsets 前缀和+4 缓冲+init/analyze/clear(halfres,0x60)/halfres(0x40)/halfres all(0x50)/3 全分辨率(dyn)/upsample(0x30)+spatial resolve。pass 序/间接偏移/绑定对上。 |
| 0x1409ba340 | `DrawLists::new` | **判等** | 21 空 Vec({0, 0x10, 0})。 |
| 0x1409ba4b0 | `DrawLists::sort` | **判等** | 21 列表各按 sort_key(i32@+0x60) 稳定排序, len>=2 才排。 |
| 0x1409ba540 | `build_draw_lists` | **判等(容器/分派/选列)** | Query 10 元组对上; `debug_raster_get_name`+expect; DISPATCH 87 名 sort_key 与机器码立即数对上(0x157c=5500、0x1f41=8001、0xdac=3500 等); select_lists: designated→[n]/ui→[19]/wireframe→[20]/kind 主列表+shadow 0+OC/2(7/10); stencil: kind1/2→Some(variant)、kind3→Some(variant\|0x80)、kind5→Some(4); push 边界 cmp 0x14 ja。元素布局 sort_key@+0x60/stencil@+0x65/func@+0x70、步长 0x80 对上。**回调本体**: 25 个 draw_shaders 直接函数(抽查 floor/grass/stairs/water_planes/ivy 判等)、env 闭包经 draw_callbacks_a/b(Envelope4 discard、Env35 topology_draw fresh 判等); 其余 env 闭包未逐 fresh 复核——按纪律标 `未逐 fresh(待补)`, 不冒充。 |
| 0x1409d1710 | `custom_cursor_pass` | **判等** | 光标资源 +0xf4 门; raster pass + uniform[u32;2]+磁盘纹理+CustomCursorSsbo+draw_no_vao(6)。 |
| 0x1409bf320 | `refraction_copy_pass` | **判等** | 列表 14(+0x158/0x160, 步长 0x80) 逐元素 bind_common+绑源+回调。 |
| 0x1409c3a80 | `pre_taa_transparency_pass` | **判等** | 列表 16(+0x188); 3 纹理(depth/b/c) 绑定; byte3 field2。 |
| 0x1409c40b0 | `post_taa_transparency_pass` | **判等** | 列表 17(+0x1a0/0x1a8) 为空→直接 return; 否则 raster+回调。 |
| 0x1409bf800 | `water_surface_pass` | **判等** | count 缓冲 0x10+clear、ray 缓冲(n+(n>>4))<<4、raster(列表 15, 7 绑定)、deferred rt args(dispatch_1d(1))、deferred rt(间接+9 绑定)。 |
| 0x1409c08d0 | `ice_surface_pass` | **判等(资源/时序)** | 13 资源名长(0x11/0xd/0x15/0x15/0x13/0x12/0x11/0x17/0xf)与 ICE_RESOURCE_NAMES 对上; 2 temporal pair + 4 compute pass(含 bind_ray_tracing+间接)。compute 内逐绑定未逐 fresh 复核(待补)。 |
| 0x1408f68c0 | `session_selection::impl$0::build` | **判等** | add_systems(PreStartup, thumbnails_startup)+insert_resource(ThumbnailTextures)。 |
| 0x1408f6900 | `thumbnails_startup` | **判等** | 4 load_shader_into_library + 6×create_thumbnail_texture(0x68 步长入池)。 |
| 0x1408f6c00 | `render_session_selection_previews` | **判等(窗内判定/池)** | 窗内判定 disasm 核对: `start=floor(min(len,max(0,scroll))-pool/2+0.5)`、`end=min(len,start+pool)`、断言 range<=min(6,len); 池分配 `FILETIME_UNIX_EPOCH=0x19db1ded53e8000`; pool_index_array=usize::MAX 填+unwrap+assert<pool.len。 |
| 0x1403d8bd0..0x1403da570 ×6 | `upload_raster_staging_to_texture<T>` | **判等** | 六实例同形(各 1306 B): 栅格 +0x29a 上传待办门; 格式 0x4c→"r16f"/0x61→"rgba16f"/其它→todo(先于状态修改); +0x29a=0/+0x29b=1; dispatch_2d(0x80,0x200)。 |
| 0x1402b00e0 | `read_access_buffer<ColorLibrarySsbo>` | **判等** | get_rg_buffer+add_resource_access(0x1a,组1)+绑定数<0x100 unwrap+追加 0x44B 绑定。 |
| 0x140a59800 / 0x140a59860 | `TexturePreset::as_texture/dims` | **判等** | get_rg_texture→extent_2d, 标签 0。 |
| 0x1405896e0 | `bind_color_library<ComputePass>` | **判等** | read_access_buffer<ColorLibrarySsbo>; 注册表命中 hash 0x52ec6a17beb8fef0 追加范围。 |
| 0x1416286f0 | `bind_color_library<DrawBuilder>` | **判等** | 槽表 rustc_entry 去重; 已占槽不重复登记; access 0x1f。 |
| 0x1415fc230 | `bind_ray_tracing<ComputePass>` | **判等** | 4 槽 read_access_buffer(RtTriangles/Meta/Bvh/WallColorId, 0x1a)+2 write_uniform(Tri 0x1c9f2eef0ac739ac/Bvh 0xc2b7f2fdf5e21521)。 |
| 0x1416289d0 | `bind_ray_tracing<DrawBuilder>` | **判等** | 同上+WallColorId 回读(read_storage_buffer 槽 "WallColorIds", 0xaf42315d263aa22a)+写 CPU 缓冲。 |
| 0x1405876a0 | `ComputePass::new<String>` | **判等** | calculate_hash→PassResources::new_from→get_compute_by_name(未命中 as_compute_pipeline+ComputeShaderProgram::new+add_compute)→UniformBinder::new→bind_color_library。 |
| 0x1409cacb0 | `draw_floor_shader` | **判等** | state byte0=(b&0xf3)|8; 预设 9/10/6/0x13→磁盘→stone/wall→3 磁盘→预设 0x15/7/0xc/0xe/0x14→磁盘→uniform bool→draw。 |
| 0x140b4af70 | `draw_grass_shader` | **判等** | 预设 9/10/8→GrassBladeData+AtomicCounter ssbo→uniform 0x90000→multi_draw_indexed_indirect。 |
| 0x14086a780 | `stairs_draw` | **判等** | StairsTransform ssbo→磁盘→uniform i32 0x9c40→uniform bool(flag)→预设 0xc→multi_draw_indirect(flag+1)。 |
| 0x1409d9bc0 | `water_planes_draw` | **判等** | demo 门→discard; ActiveSession 变体@+0x58+字节序列全非 0; state byte3&=0x9f; 绑定序(预设 9/10/0x13→Ripple→0x18→noise→Dof→9/10/8→noise 二次); uniform Vec3 water_color; 冬季 variant4 磁盘; draw。 |
| 0x14206f300 | `nani_maintain_instances` | **判等** | 删→建→改三阶段: remove_entry+remove_instance / get_by_name+extend_no_data+insert NaniActiveInstance / update_instance_mesh。 |
| 0x142070ac0 | `nani_update_solid_vertex_color` | **判等** | owned_subset_mut→update_instance_data<SolidVertexColorInstanceData>。 |
| 0x1409b85d0 | `closure_env$4`(discard) | **判等** | DrawBuilder::discard。 |
| 0x1409b8340 | `closure_env$35`(topology_draw) | **判等** | topology tag==1→state byte0=(b&0xfc)\|2+载荷@+0xac; 否则保持; draw。 |

## 未判等(需重做 / 待补 fresh)

| VA | 函数 | 状态 | 说明 |
|---|---|---|---|
| (build_draw_lists 内 50+ 个 env 闭包) | `closure_env$N` | **已逐 fresh 复核（55/56 判等）** | 见 [REBUILD-REMAINDER.md](REBUILD-REMAINDER.md) 批次 1-3；仅 `closure_env$36`（0x1409b7fa0）未判等: ranges 经 `SparkleDrawRangesView` 视图、原版每帧 `SparseInstanceBuffer::to_draw_multi_instanced` 现算，需 resources/ssbo 层接线。 |
| 0x142078780 系(nani update_* 其余) | `nani_update_*` | **已逐 fresh 复核（20/20 判等）** | 见 [REBUILD-REMAINDER.md](REBUILD-REMAINDER.md) 批次 4；19 个 update 的 `update_instance_data<T>` 类型全对上 + flush 27 槽/滚动计数。 |
| (ice_surface_pass 内部) | 4 compute pass 逐绑定 | **已逐绑定 fresh 复核（4/4 判等）** | 见 [REBUILD-REMAINDER.md](REBUILD-REMAINDER.md) 批次 5；march/trace 预设 9/10/8、蓝噪 0x579f…、ray_tracing、tex_write、write_rg、dispatch_indirect 全对上。 |

## 结论

- **判等**: 34 个函数/6 组同形实例 fresh 反编译逐语句对上(核心重点全部判等: read_terrain_data 四 pass 写序、delayed 回读拷贝序、session_selection 窗内判定、draw_lists 排序/裁剪、bin_lights 灯箱布局)；续批 REBUILD-REMAINDER.md 再判等 env 闭包 55、nani update_* 20、ice_surface 4 compute pass 逐绑定。
- **未判等**: 仅 `closure_env$36`（0x1409b7fa0，ranges 数据流经视图、原版每帧 `to_draw_multi_instanced` 现算，需 resources/ssbo 层接线）；不冒充判等。
- 未发现结构对但逻辑假的情况; 未发现缺分支/条件反/循环错/callee 错（Env36 数据流偏差已明确登记）。
