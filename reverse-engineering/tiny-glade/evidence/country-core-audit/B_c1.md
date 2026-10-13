# B 阶段报告：c1（compute_passes：gpu_generate_draw_lists / compute_water / rasterize_terrain_stroke / dof / dof_autofocus / read_terrain_data / taa / mod）

标注：O = 机器码观测，I = 推断，U = 未知。工作文件均在 `reconstruction/country-core/src/systems/render_loop/compute_passes/`。

## 1. 结果摘要
- 所属 8 个文件内 `todo!` / `unimplemented!` 全部清除（grep 为 0）；新增 `pass_kit.rs`（c1 共用工具，已在 `mod.rs` 登记 `pub mod pass_kit;`）。
- 方法：绑定名哈希**反解成功**——`movabs r8` 立即数 = **FNV-1a-64(HLSL 里的资源/`$Globals` 字段名)**（`__alias_` 前缀去掉；`<name>_dims` 字段 = `hash(name)+1`）。
  对照 `reconstruction/shader-source/shaders/**.hlsl` 反解出 100+ 个绑定名，各文件测试逐个断言 `binding_hash("名字") == EXE 立即数`。
- 所有 pass 现在经 A4 的真实 `ComputePass::new / uniform / bind_texture / *_storage_buffer / dispatch_*` 声明，返回 `RecordedComputePass`，不再是 `Gap` 占位。
- 一个重要纠错：反汇编注释里 `v4=[0.0,…]` 的 `.rdata` 向量其实是**dispatch 描述符**（`{mode, x, y, z}` 的整数，被当成非规格化 f32 显示成 0.0）。
  例：autofocus 的 `0x142ac7e70 = {1,8,8,1}` → `dispatch_2d(8,8)`；全部 dispatch 形状都按字节读出。
- 测试（隔离测试台 `/tmp/h_c1`，挂载真实源文件）：本范围 54 个全通过；整个测试台 378 通过 / 1 失败——失败项 `mesh_atlas_library::default_has_all_27_subsets`
  是我为让他人文件（引用 `GpuWorldSubsetKind::Canopy`）编译而给测试台桩枚举加了第 28 个变体造成的，**与本范围无关**。
- `rustfmt --check --edition 2024 --config max_width=120` 对 8 个文件均通过。

## 2. 函数 → Rust 项
| 地址 | PDB 过程 | Rust 项 | 结论 |
|---|---|---|---|
| 0x140a17880 | `dof_autofocus::calculate_dof_autofocus` | `dof_autofocus::calculate_dof_autofocus` + `autofocus_uniforms` | O：5 个 ECS 资源；`point_uv` 三分支（mode 2 鼠标 / 3 / 其余）、门控 `mode<2 && view_kind==0x13`；7 个 uniform 顺序；2 SSBO + `depth_tex`(0x19)；pass 名 `"Mouse Depth Write to SSBO"`；`dispatch_2d(8,8)` |
| 0x140a5a020 | `dof::dof_prepare` | `dof::dof_prepare` | O：`CoC`(fmt 0x4c)、`CoC tiles`(`div_up_extent([8,8,1])`, fmt 0x53)，`DoF prepare` 绑定序、`dispatch_2d(depth)` |
| 0x140a5a570 | `dof::calc_dof`（含内联 multires） | `dof::calc_dof` / `calc_dof_fullres` / `calc_dof_multires` | O：全局开关 `*0x14362c080`；fused 单 pass；multires 9 个 pass 的顺序、绑定、间接偏移 0x60/0x40/0x50/0x30（全分辨率三项 I：`0x10*i`）；RG 缓冲名/大小；tile 偏移 = 独占前缀和 |
| 0x140a5d4f0 等 5 个 | `dof::impl$3/7/11/15/19::as_compute_pipeline` / `source_hash` | `FullResAllLod0Dof / FullResAllLod1Dof / FullResDof / HalfResDof / HalfResAllDof` | O：各带 `quality: u32`；`source_hash = calculate_hash(&("uri:entry", &self))`；define `PERM_quality`。**纠正旧文档**：`impl$3`=lod0、`$7`=lod1、`$11`=fullres |
| 0x140a3b2f0 | `taa::calculate_taa` | `taa::calculate_taa` | O：`"TAA"`，`history_valid = valid ? !G : 0`，5 个纹理访问序，`bypass_neighborhood_clamping`、`sharpen_amount`，`dispatch_2d(history_output)` |
| 0x140a3ba60 | `taa::calculate_tgsr` | `taa::calculate_tgsr` | O：`tgsr_aux` temporal 对（Err→`Result::unwrap` panic）、3 个 RG 纹理（0x4c/0x53/9）、`TGSR blur lum` → `TGSR reproject` → `TGSR`（10 个纹理访问）|
| 0x140a3e100 / 0x14145ac20 | `TgsrMain::as_compute_pipeline` / `calculate_hash` | `TgsrMain {wave32, sharpen}` | O：`PERM_wave32`、`PERM_sharpen`（u32 Display）；`wave32 = min_subgroup_size.is_some_and(<0x21)`；两字段进 hash |
| 0x14098c680 | `read_terrain_data::read_terrain_data` | `read_terrain_data::read_terrain_data` 等 | O：`Read terrain height`(1d 0x20000)/`rock mask`/`Read water`(demo 守卫 `*0x143634d90`)/`Read garden`(1d 0x2000)，预设槽 9/10/0xe/0x13/0x14；尾部 `on_gpu_copy_issued`（swap(0)，`\|=`）；光栅预渲染 `"Visual terrain height"` 固化为 `VisualHeightRasterPlan` |
| 0x140a12680 | `compute_water::water_rasterize_from_cpu` | `compute_water::water_rasterize_from_cpu` | O：**实为水 mask 4 遍 gauss 模糊**（`Blur water mask`，σ=10,10,30,30，`output_channel` 1,1,2,2，`water blur temp` 512²/0x61，`dispatch_2d(512,512)`），脏字节 +0x29b、`memset(+0x98, 0x200)` |
| 0x140a13970 | `compute_water::water_spawn_vegetation` | `compute_water::water_spawn_vegetation` | O：demo 守卫；`Spawn lilypads`（`is_autumn = session[+0x58]==3`，grid 225²，offset 8）/`Spawn reeds`（grid 150²，offset 9）；预设 9/10/0x13/5/0x14/0x17/0x16 |
| 0x1408c7120 | `rasterize_terrain_stroke::rasterize_terrain_stroke` | `rasterize_terrain_stroke::rasterize_terrain_stroke` + `upload_strokes` | O：早退条件、`width>0.01`、点数 0x400 上限、>0x3fff 一次性错误日志 `"Ran out of live terrain strokes"`、`StrokeInfo`(16 B，`(id<<10)\|(count&0x3ff)`、`pack_2x16f_uint`)、断言文案、5 个 pass |
| 0x1415fb550 | `rasterize_terrain_stroke::closure_env$1` | 栅格化 pass 的 `dispatch_with` 闭包 | O：每 tracker `track` + `dispatch_3d(512,512,..)` |
| 0x1408ca1c0 / 0x141459fc0 | `GaussBlur::as_compute_pipeline` / hash | `GaussBlur {fmt}` | O：`PERM_fmt`；`PERM_ty`="float"(I)；hash 含 fmt |
| 0x140987dd0 | `gpu_generate_draw_lists`（W22/W28） | `FRAME_PASS_ORDER` + 各 pass 函数 | 调用顺序 O；子集展开（`GpuWorld` 27 子集）U，见 §5 |
| 0x140a28950 等 | `create_cull_and_bucket_pass<T>` | `create_cull_and_bucket_pass` | O：`"Nani cull & bucket"` 8 步绑定序 + HZB 变体 |
| 0x1415fa2f0 | `cull_and_bucket::closure_env$1` | `cull_and_bucket_dispatch` | O：每项 `push_constants(20 B)` + `dispatch_3d(count,1,1)` |
| 0x140a27b10 / 280a0 | `cull_and_bucket_external<..>` | `cull_and_bucket_external` | O：再读 `instance_buffer`，`dispatch_with`（末尾 `assert_eq!(&str,&str)` 的两侧串 U，未复刻） |
| 0x140a29930/297b0/29640/29280 | `generate_draw_lists` / `group_draw_cmd_prefix_sum` / `prefix_sum_unsorted…` / `fill_sorted_instance_list` | 同名 `*_pass` 函数 | O：pass 名、URI、绑定序、RG 缓冲名；dispatch 的偏移/计数（`RenderSublistsIter`）作为参数 |
| 0x14098aaf0 / 0x14098b2a0 | `generate_wall_draw_lists` / `_pass2` | 同名 | O：`cs_pass1`、`reset_pass2_inst_counts`、`cs_pass2`（读 `hzb_tex`）绑定序 |
| 固定形状 | clear_draw_lists / clear_histogram / draw→dispatch / histogram_prefix | 同名 | O：`dispatch_1d` 108 / 0x1b000 / 1 / 0xd800（`.rdata` 描述符）；`Draw -> dispatch args` push `{slot, 4}` |

## 3. O / I / U 清单
- **O**：以上表中全部 pass 名、URI、uniform 名与类型、绑定序、访问码（0x19/0x32）、预设槽、dispatch 形状与常量（见每个文件头与测试）。
- **I**：autofocus 的 `use_dual_focus` 语义（只按 HLSL 字段名命名，值 = mode==2 分支标志）；dof multires 的 `halfres/output` 纹理 extent、`"DoF tile offsets"` 大小 `(12+4n)&!0xf`、全分辨率三项偏移 `0x10*i`；rasterize 的 blur#1 `img_in` 预设 9 与 blur#2/Derivative 的 dispatch 复用 `{1,512,512,1}`；`GaussBlur` 的 `PERM_ty`；`CULL_WRITE_GROUP=1`；`cull_and_bucket` 的 prev/next 两组合并。
- **U**：`GpuWorldSubsetKind` 27 变体与每子集 `PushConstants` 5 个 u32 的含义、`RenderSublistsIter` 内部与 `IndirectArgsType` 完整变体表、`gpu_world_subset` 跳表目的地；`ActiveRasterStroke` 记录里除 id/width/params/points 之外的字段；`water_level_at_xz`（依赖 `utils::float_grid::bilinear_sample_at_world_xz`，他人所有）；`DynamicConstantsChunk::into_uniform_buffer_binding` 返回的缓冲号（`bind_external_raw` 里占位 0）；`TextureDesc::storage()` 位。

## 4. Stub / 对公共 API 的改动
- 本范围没有改任何共享文件；没有新增 stub 到 `lib.rs`/`rhapsody_types.rs`/`rhapsody_contract`。新增接缝全在自有文件里：`pass_kit::{PassEnv, RgCreator, DynSource}`、`taa::TemporalGraph`、`read_terrain_data::VisualHeightRaster`、`rasterize_terrain_stroke::StrokeAllocator`。
- 公共 API 签名改动（旧 → 新，原因）：
  - `dof_autofocus::calculate_dof_autofocus(world:&mut Gap, ..)` → `(env:&mut PassEnv, &AutofocusInputs, &RgTexture) -> RecordedComputePass`，泛型 SSBO 参数 `<A, R>`（真实载体在 `resources` 里是 typed gap，不满足 `LibrarySsbo`）。**同时移除** `dof_autofocus::{DofAutofocusSsbo, DofFocusReadbackSsbo}` 本地占位（`resources::DofAutofocusSsbo` 才是共享的那份）。
  - `dof::{dof_prepare, calc_dof}`、`taa::{calculate_taa, calculate_tgsr}`、`read_terrain_data::*`、`compute_water::{water_rasterize_from_cpu, water_spawn_vegetation}`、`rasterize_terrain_stroke::rasterize_terrain_stroke`、`gpu_generate_draw_lists::*`：全部由 `world: &mut Gap` 占位改为真实输入；**无其他文件调用这些函数**（已 grep；`render_world.rs` 里的同名项是它自己的 trait 方法）。
  - `gpu_generate_draw_lists::PushConstants(u32,u32)` → `PushConstants([u32;5])`（O：push 20 B）；删除旧的 `IndirectArgsType::Slot(Gap)` / `RenderSublistsIter` / `GpuWorld`-依赖的 `build_push_constants`（均为臆造的占位，原无机器码依据）。
  - `dof::IndirectArgsType` 补全 7 个变体；`dof::{FullResDof 等}` 增加 `quality` 字段（O）。
  - `rasterize_terrain_stroke::GaussBlur` 由单元结构体改为 `GaussBlur{fmt}`；删除空实现的 `as_shader_def_value`/`hash`。
  - `compute_water`、`read_terrain_data` 去掉对 `resources::{session,world_raster_defs,water_mask::WaterMaskSsbo…}` 的无用导入。
- **请求集成者**：(1) A4 的 `ComputePass::new` 的 `compute_desc` 把 `defines` 恒置空，而 `TgsrMain`/5 个 DoF 源/`GaussBlur` 的 `as_compute_pipeline` 都带 `PERM_*` define（本范围经各类型 `pipeline_defines()` 暴露，未接入）——需要 A4 在 `AsComputeShaderSource` 加可选 `defines()`。(2) `rhapsody_types::TextureDesc` 缺 `storage()` 位。(3) `Cargo.toml` 已有 `half`，无需追加。
- `water_planes_draw.rs`（他人）仍引用 `compute_water::{LilypadSsbo, LilypadFlowerSsbo, ReedSsbo}`，这三个类型保留未动。

## 5. 遗留
1. `gpu_generate_draw_lists`（主函数）**没有**端到端的"遍历 `GpuWorld` 子集并展开所有 pass"的 Rust 函数：只恢复了每个 pass 的声明、dispatch 闭包、固定形状、调用顺序（`FRAME_PASS_ORDER`）。原因：展开依赖 `GpuWorld`/`GpuWorldSubsetKind`（`render::nani::gpu_world`，他人所有）与 27 路跳表，需要先恢复这些类型。
2. `read_terrain_data` 的光栅预渲染（`"Visual terrain height"` + 两个 `DrawBuilder`）只固化了参数，执行走 `VisualHeightRaster` 接缝（依赖 ECS 实体查询 + A4 draw_builder）。
3. `rasterize_terrain_stroke` 的活动笔画表是 hashbrown SwissTable 遍历（SIMD 控制字节扫描）；这里由调用方给出遍历后的 `&[RasterStroke]`。
4. `water_level_at_xz` 仍返回 0.0（U，依赖他人的 `bilinear_sample_at_world_xz`）。
5. 测试台的 `mesh_atlas_library` 失败项见 §1（测试台桩枚举，非真实 crate 问题）。
