# B 阶段 i_infra：compute / draw_shaders 侧重复基础设施合并，统一落在 i_api 重写后的 A4 真实 API 上

范围：`systems/render_loop/compute_passes/*`、`indirect_params.rs`、`draw_passes/draw_shaders/*`、`resources/render_ssbo`（拆成独立文件）、
`consume::buffers`（经 `render_ssbo` 再导出，`lib.rs` 未改）、`systems/render_loop/resources/critters.rs` + `resources/critters.rs`、
`utils/components/transform.rs`、`resources/image_formation.rs`。O = 机器码观测，I = 推断，U = 未知（未编造）。
验证：全树测试台 `/tmp/h_all`（挂 render 全部 + compute_passes 全部 42 个文件 + indirect_params + draw_shaders 全部 + `resources::render_ssbo` +
`image_formation` + `transform`）**785 passed / 0 failed / 0 警告**。

## 1. 三份 pass 环境合并为一份（任务 1）

唯一环境 = `compute_passes/pass_kit.rs`（+ `pass_kit/test_env.rs`，`#[path]` 挂载，真实 crate 与测试台都能解析）。删除：c1 旧 `pass_kit` 内容、
c2 `c2_support.rs` / `c2_testenv.rs`、c3 `indirect_params/pass_env.rs` / `pass_env/test_support.rs`；`indirect_params.rs` 摘掉 `pub mod pass_env`，
`compute_passes/mod.rs` 摘掉 c2 的两个模块。

| 旧（三份） | 新（唯一） |
|---|---|
| c1 `PassEnv{world,backend,uniforms}` + `RgCreator` 单独传参 / c2 `PassCx{.., frame}` / c3 `PassEnv{.., graph: RgFactory}` | `PassEnv{world, backend, uniforms, frame: &mut dyn PassFrame}`；`PassFrame: RgResources + RenderGraph { fn record(RecordedComputePass) }`（RG 创建 = i_api 的**唯一** `RgResources`） |
| 三套 `binding_hash`（c2 含 `_dims` 规则，c1/c3 不含） | **一份 `const fn binding_hash`**（O 0x140c87660：`_dims` 后缀 → `fnv(前缀)+1`；`const` 是为了写 `const H_X: u64 = binding_hash("x")`，`fnv` crate 不能在常量里用）；`imm`、`binding::*`、`TEX_READ/WRITE/STORAGE_WRITE` 同在此处 |
| `Texture2dDesc`/`RgTextureRef`/`RgBufferRef`/`TemporalPair`×2/`RgTextureSpec`/`rhapsody new_2d(u8)` | 真实 `TextureDesc::new_2d(..).storage().sampled()` / `RgTexture` / `RgBuffer` / `TemporalTexturePair{current, previous, fresh}` |
| `env.graph.create_texture` / `create_rg_texture(graph,..)` / `frame.create_raw_buffer(name,size,usage)` | `env.create_texture(name, desc)` / `env.create_buffer(name, size, usage)` / `env.temporal_pair` / `env.temporal_buffer`（`Err` → `Result::unwrap` panic，O） |
| `frame.preset_texture_desc` / `frame.ssbo_instance_count(name)` 接缝 | 经 world 里的 `PassResources`：`env.preset_desc(p)`（O 0x14093cf30 `get_rg_texture(..).desc`）、`env.ssbo_instance_count::<T>()`（`AssetSsboLibrary::get::<T>()`，i_api 新增） |
| 三套测试环境（`Env`/`TestWorld`+`TestFrame`/`Env`+`FakeGraph`） | `test_env`：`Reflection`（c2 约定 `new().bind().uniforms().dims()` + c3 约定 `for_names`）、`TestWorld`（含 `with_presets`）、`TestFrame`（全部 RG 创建/temporal/记录）、`Env{world,frame,offsets}`+`run`、`with_env`、`threads/accesses/rgtex/uniform_u32`、`test_carrier!` |

c1 / c2 / c3 三个家族的 42 个 pass 文件全部改用它（c2、c3 由 fork 的 worker 完成，c1 + 集成由我完成）；每个文件的 `#[cfg(test)]` 都迁到新环境，
**机器码常量 / 文案 / 分支断言全部保留**，只改取值方式（资源号按“纹理/缓冲/temporal 共用一个计数器，100+序号”重算；并发写访问码 0x32→0x37 是 i_api 的 O 0x141608450，
不是弱化）。公共签名变化（旧→新）：所有 pass 函数的 `cx: &mut PassCx` / `(env, graph: &mut dyn RgCreator|TemporalGraph)` → `env: &mut PassEnv`；
`taa::calculate_tgsr` 去掉 `TemporalGraph` 参数（A4 temporal 对）；`dof::{calc_dof, calc_dof_fullres, calc_dof_multires}` 去掉 `blue_noise: &RgTexture`
（改用真实 `TextureFromDisk`，见 §4）；`gpu_generate_draw_lists::{prefix_sum_unsorted_instances_per_atlas_entry, fill_sorted_instance_list}` 的字节数 `u64→u32`；
`bin_lights<L: LibrarySsbo>` 泛型化（`L::Storage: InstanceCounted`）；`grass_compute` 去掉 `scan` 闭包参数。

## 2. `bind_garden_exclusion` / `bind_water_object_push` 统一（任务 2）

* 任务描述里 `common_includes.rs` 的 `unimplemented!` 桩在本轮开始前已被 d_shaders 恢复为真实 `DrawBuilder` 版（O 0x141631f90）；ComputePass 版（O 0x14160e330 /
  0x14160e630，逐条反汇编核对过的序列：预设 0xf(0x19)→`read_access_buffer<RippleSourceSsbo>`(0x1a)→预设 0x18(0x19)；预设 0xe/0x13/5/0x10 全 0x19）
  **只剩 `common_includes_compute.rs` 一份**：删除 `c2_support` 的 `bind_water_object_push`/`bind_garden_exclusion`/`GARDEN_EXCLUSION_HASHES`，
  `spawn_vegetation_around_buildings.rs` 的内联 `bind_garden_exclusion`，调用方（compute_garden、stairs、leaf_particles、spawn_vegetation）全部改用它。
* 新增 3 个行为测试（此前 `common_includes_compute` 无行为测试）：常量 = HLSL 名字的 `binding_hash`、四预设只读序列、水推送三段访问码序列。

## 3. 间接缓冲类型（任务 3）

* `resources::render_ssbo` 拆成 `resources/render_ssbo.rs`（`resources/mod.rs` 只把内联模块换成 `pub mod render_ssbo;`）：`pub use indirect_params::*`，
  旧 `AtomicCounterBuffer/DrawIndirectBuffer/DispatchIndirectBuffer/DrawIndexedIndirectBuffer(Gap)` 占位删除；`consume::buffers` 因此指向真实类型（`lib.rs` 未改）。
* `ssbo_tags.rs`：四个零大小标签（`AtomicCounterTag`/`DrawIndirectTag`/`DrawIndexedIndirectTag`）换成真实类型的 `SsboType` + `DrawSsbo` 实现（SsboId = `indirect_params::ssbo_ids`
  的 O 立即数，测试 `xxh3(NAME)==id` 覆盖四个类型）；旧标签名保留为**类型别名**（`draw_callbacks_b.rs` 属他人，仍按旧名引用）。
  `grass_shader`/`draw_bricks`/`stairs_draw`/`path_pebble_shader`/`water_planes_draw` 改用真实类型；`gpu_generate_draw_lists` 本就用 `indirect_params` 的真实类型（经 `NaniSsbos` 关联类型，c1 已做）。
* c2 的 `DrawIndexedIndirectBuf/DrawIndirectBuf/DispatchIndirectBuf/AtomicCounterBuf` 载体删除，`pass_kit` 里只剩仍无真实类型的 `RippleSourceBuf`、`RockyTerrain*Buf`、`StairsCounterBuf`。

## 4. 6 个缺 `SsboType` 的 SSBO（任务 4）

| 类型 | 位置 | 机器码（O） | 实现 |
|---|---|---|---|
| `FireflySsbo` | `render_loop/resources/critters.rs` | `impl$4::new_storage_buffer` 0x14161f1e0：16 B × 96 = 0x600、memory 2 | `MappedStorageBuffer<'static,T>`（i_api 的 `BUFFER_LEN×size`），缺省全零 |
| `FireflyVisibilitySsbo` | 同上 | 0x14161fea0：4 B × 96 = 0x180、memory **4** | 同上 + `MAPPED_MEMORY = READBACK` |
| `ButterflySsbo`（顺带） | 同上 | 0x14161b0f0：32 B × 24 = 0x300 | 同上；`default_value()` = 常量元素（`forward=(-0,-0,-1)`，O 0x14161b172 起） |
| `BushSsbo` | `compute_garden.rs` | `impl$0` 0x141610b60：`BufferDesc::new(0x70800, 0x22, 1).alignment(4)`，名 "Bushes"，元素 48 B（`BushXform`=`Affine3Packed`）→ 容量 9600，上限 `cmp r9, 0x70830` | `TypedStorageBuffer`，经 `pass_kit::fixed_len_typed_storage`（见下） |
| `FlowerSsbo` | `compute_garden.rs` | 0x141613b90：0x33e000、名 "FlowerSsbo"，元素 64 B（`{Affine3Packed, color Vec3, flower_type u32}`）→ 容量 **53120**，上限 `cmp r14, 0x33e040` | 同上 |
| `LanternSsbo` | `resources/render_ssbo.rs` | `TypedStorageBuffer::new` 0x141615bb0：`BufferDesc::new(0x80000, 0x22, 1).alignment(4)`，名 "LanternSsbo"，元素 16 B（`LanternPacked{float3 position; uint intensity__color_id}`）→ 容量 0x8000；存储是 `InstanceBuffer<LanternSsbo>`（`type_id` 0x14160ec30） | `InstanceBuffer`；SsboId `0x8b2da1ef1bff9619` = xxh3("LanternSsbo") |
| `MouseDepthValueSsbo` | `resources/render_ssbo.rs` | 0x14161e8f0（映射）：`BufferDesc::new(0x40, 0x22, 4).alignment(4)`，名 "MouseDepthValue"，元素 16 B（`float4(world.xyz, depth)`，16 对齐断言 `and rax,0xf`）→ 容量 4，无数据写 0x40 B 零 | `MappedStorageBuffer`（`#[repr(align(16))]`），memory 4 |

新增测试：critters 库注册后缓冲字节数 = 0x300/0x600/0x180、映射区元素数；Bush/Flower 元素与容量、库内字节数、超容量 `Excessive data size` panic、非整数个元素断言；
Lantern 实例缓冲（`instance_count`=0、字节 0x80000）与 SsboId 立即数；MouseDepthValue 映射/回读。

**对 i_api 的缺口 / 偏差报告（U 或 I，未改它的文件）**
1. `TypedStorageBuffer::new_storage_buffer` 只建模了 `BUFFER_LEN==0`（Sheep：memory 2、大小随数据）；`impl$0::new_storage_buffer` 在 `BUFFER_LEN>0` 时是**固定大小、memory 1、对齐 4、无数据则不上传**
   （Bush / Flower 机器码）。我在 `pass_kit::fixed_len_typed_storage` 里经 `TypedStorageBuffer::new` 落地，唯一差别：`BUFFER_LEN>1` 且无初始数据时 `new` 上传全零，原版不上传（U）。建议 i_api 把该形态并进 `new_storage_buffer`。
2. `MappedStorageBuffer` 用 `.alignment(align_of::<T>())`，而 `MouseDepthValueSsbo` 的 `BufferDesc::alignment` 实参恒为 4（O 0x14161e9cd），元素指针却按 16 断言。
3. `TemporalTexturePair.fresh` 是 I 命名：机器码里该字节是**直接喂给 `history_valid` uniform**（c2/c3 一致，O），“本帧新建”的语义无证据，两者可能相反。
4. `LanternSsbo` 的 `type_name` 是 `country_core::resources::render_ssbo::LanternSsbo`，原版是 `…::resources::render::lantern::LanternSsbo`（0x35 B）；`AssetSsboLibrary::get` 的名字断言因此是内部自洽的，若要字面一致需把类型挪到 `resources/render/lantern.rs`（该文件不归我）。

## 5. B_i_api §5 跟进项（任务 5）

| 项 | 结果 |
|---|---|
| `pass_env` 的 `FromDisk` 旁路（多登记一条 RG 访问，偏差） | 删除 → 真实 `TextureFromDisk`（A4 外部描述符，`ExternalSlot::Image`，不登记 RG 访问、不追加 AccessRange）。受影响的访问序列断言按新语义重算（apply_volumetrics RG 写序 `[F,F,T,F]→[F,T,F]`；primary_rt_debug 访问数 10→9；restir_di 9→8；各自加 `external_bindings.len()==1`；ray_trace_gi/eval_gi 同）。DoF 的 `blue_noise` 也改走它（halfres pass 带 1 条外部绑定） |
| c2 `SpdViewBinding` / `DiskTextureBinding` / `record_with_views` | 全删 → `bind_texture_with_view(.., Some(&TextureViewDesc…))`（真实 `rhapsody_types::TextureViewDesc`，`Option<u32>` 字段）；generate_mips/hzb/blur_pyramid 测试断言 `PassBinding::ImageView.view`；grass 的磁盘纹理走真实 `bind_texture` |
| `pipeline_defines()`：5 个 PERM 源 | `TgsrMain`（taa）、`FullResAllLod0/Lod1/FullRes/HalfRes/HalfResAll`（dof，原 `DofSource::pipeline_defines` 与 trait 方法二义，已并入 `AsComputeShaderSource`）、`BinLights`、`GaussBlur`、`GenerateStrokes`（`render_loop/vermeer.rs`，**范围外文件，只加了 `ShaderDefine` 导入 + 一个转发方法**）全部覆写；测试断言 `PERM_*` 到达 `ComputePipelineDesc.defines`；`DynSource` 也转发 `pipeline_defines`（此前丢 define） |
| `water_planes_draw` 闭包 | 新增 `water_planes_draw_rt(b, inputs)` = 真实 `DrawBuilder::bind_ray_tracing::<RtTriangles,RtTriangleMeta,RtBvh,WallColorId>()`；**保留闭包版 `water_planes_draw` 作兼容入口**，因为 `draw_callbacks_b.rs`（他人所有）仍按闭包签名调用——请该文件的 owner 改调 `_rt` 后再删闭包参数。`test_support::RecHost` 补 `ssbo_buffer_word` 转发 + `Rig` 预置 Tri/Bvh 缓冲字；新测试：RT 四槽排在雪纹理之前、两个 `write_uniform<u32>` 写入缓冲字、两个入口等价 |
| critters | 见 §4；`resources/critters.rs` 里已无引用的 `ButterflySsbo/FireflySsbo/FireflyVisibilitySsbo(Gap)` 占位删除 |
| `ObjectGrassPush` 是单个 f32 | `pub struct ObjectGrassPush(pub f32)`（O：`register_component` 0x140a5fc10 的 Layout size 4/align 4；`object_grass_push_pass` map 闭包 `movss xmm1,[r8]`），加 size/align 测试 |
| grass 的 prefix scan 闭包接缝 | 接成直接调用 `prefix_scan::inclusive_prefix_scan_u32_1m(env, &SizedRgBuffer{bundle_index, 0x80000})`，三个 pass 依次 `env.record`；grass 现有 11 个 pass（插在 gen 与 dispatch args 之间，O 序），子和缓冲 `subtotal_sum_buf` 0x1000 B；测试重算 |

## 6. 两处文档与机器码不符（任务 6）

* **`Transform`**（`utils/components/transform.rs`）：O 0x140c9d180 `Default` 写 `[+0]=(0,0,0,1)`（旋转恒等）、`[+0x10..0x1c]=0`、`[+0x1c..0x28]=1`；`compute_affine` 0x1407a45b0 读 `[rdx]`
  四元数 / `[rdx+0x10]` 平移 / `[rdx+0x1c]` 缩放。字段改为 `#[repr(C)] { rotation: Quat @0, translation: Vec3 @0x10, scale: Vec3 @0x1c }`（有效 0x28 B，`Quat` 16 对齐占 0x30 B）；
  **同时修了一个行为错误**：`derive(Default)` 给出零缩放，O 是恒等变换，改成 `impl Default = identity()`。新增 3 个测试（`offset_of!`、默认恒等、TRS）。
  该文件里 `look_at` / `abs_diff_eq` 仍是 `todo!`（任务限定“只改这两处”，未动）。
* **`ImageFormationConfigs`**（`resources/image_formation.rs`）：O `current_mut` 0x1408aea50 / `get` 0x1408aea80（`kind << 6`）→ 单档 **0x40 B**（不是 0xc0），三档 0xc0 B，`kind` @0xc0，整个资源 0xc8 B；
  `default` 0x1408ae7f0 逐档写入的字段已按偏移落地（`String "default"` @0、`(1,0,1,1)` @0x18、u8 @0x28、2×f32 @0x30（档 2 = `(0, 0.015)`）、u32 @0x38、u8 @0x3c、u8 @0x3d（档 0/1 = 1，档 2 = 进程级布尔 `.data 0x143634d90`））；
  **缺省 `kind` 是 1**（旧源码是 0）。字段语义 U，按偏移命名；`iter_non_default_mut` 的“non_default”过滤条件 U。新增 3 个测试（`offset_of!`/`size_of`、缺省值、`switch_kind`）。

## 7. 对共享文件 / 他人文件的改动（请集成者确认）

| 文件 | 改动 |
|---|---|
| `resources/mod.rs` | 内联 `render_ssbo` 模块 → `pub mod render_ssbo;`（新文件 `resources/render_ssbo.rs`）。`lib.rs` 未改（`consume::buffers` / `consume::core` 的再导出仍成立） |
| `render_loop/vermeer.rs` | `GenerateStrokes` 加 `pipeline_defines`（+1 个导入） |
| `draw_shaders/test_support.rs`（我的文件，但被他人测试使用） | `RecHost::ssbo_buffer_word` 转发；`Rig::new` 预置 `RT_TRIANGLES_WORD/RT_BVH_WORD` |
| `draw_callbacks_b.rs`（他人） | **未改**；仍编译（闭包版保留、`*Tag` 别名保留） |

## 8. O / I / U

* O：`binding_hash` 的 `_dims` 规则、全部访问码、BushSsbo/FlowerSsbo/LanternSsbo/MouseDepthValueSsbo/Firefly* 的缓冲大小/memory/名字/元素大小/上限、`ObjectGrassPush` 大小、`Transform` 偏移与缺省、
  `ImageFormationConfigs` 步长/偏移/缺省、grass 的 pass 序。
* I：Bush/Flower/Mouse/Lantern 的字段名（取自重建 HLSL）；`FireflyVisibilitySsbo`/`MouseDepthValueSsbo` 的 memory 4 = 回读；`ImageFormationConfig` 字段名（按偏移）。
* U：`Flower03Ssbo`/`GardenStoneSsbo` 的元素布局与 NAME（仍是 `ssbo_carrier!` 载体）；`RippleSourceSsbo` 等非本范围类型无库侧实现（`RippleSourceBuf` 载体）；grass 的 `bg_terrain_heightmap`
  纹理名（键 0x8991bdb39a048ac5，测试里无法按该哈希注入纹理，故磁盘纹理在 grass 里用 `should_panic("No such texture")` 断言而不是正向断言 `ExternalSlot::Image`）；`BufferDesc` 无数据时设备初值。

## 9. 验证

```
# 全树测试台（render 全部 + compute_passes 42 个文件 + indirect_params + draw_shaders + render_ssbo + image_formation + transform）
export PATH=$HOME/.cargo/bin:$PATH CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0
L0H_REAL=$PWD/reconstruction/country-core/src L0H_WORK=/tmp/w_all CARGO_TARGET_DIR=/tmp/l0h-target /tmp/h_all/run.sh
→ test result: ok. 785 passed; 0 failed
```
各阶段分家测试台：c1 509、c3 542、c2 560、draw 侧 558（均 0 失败）。`rustfmt --check --edition 2024`：c1 文件与 pass_kit/test_env 用 `max_width=120`，draw_shaders / c2 文件用默认 100（保持各文件原风格）；
`compute_passes/mod.rs`、`draw_shaders/mod.rs` 的 `--check` 失败来自他人文件（递归子模块），非本次改动。
工具脚本：`/tmp/mkharness.py`（按模块清单生成裁剪测试台）、`/tmp/prune_draw.py`；旧环境备份 `/tmp/old_infra/`（仅供对照，可删）。注意 `mkharness.py` 以 `/tmp/h_i_infra_d` 为底，已被我删除，需要时以 `/tmp/h_all` 为底。

## 10. 遗留

1. `visibility_queries.rs`（c3）文档仍写“Firefly 的 `SsboType` 尚未恢复”，测试用 `test_carrier`；现在有真实类型，可换（行为不变，只是文档/替身）。
2. `draw_callbacks_b.rs` 改调 `water_planes_draw_rt` 后可删 `water_planes_draw` 的闭包参数与 `*Tag` 别名。
3. `render_world.rs` 等调用方尚未接 `bin_lights::<LanternSsbo>`、`calc_dof`（无 `blue_noise` 参数）等新签名——这些函数此前没有其它调用者（grep 过）。
4. `Flower03Ssbo`、`GardenStoneSsbo`、`RippleSourceBuf`、`RockyTerrain*Buf`、`StairsCounterBuf` 仍是载体；`BoatSsbo`、`GrassBladeDataSsbo`、`RoofTileSsbo`、`LeafParticleSsbo`、`ParticleEmitterSsbo`、`StairsTransformSsbo`、`ShadowMask*Ssbo` 也是 `ssbo_carrier!`（PDB 里 `new_storage_buffer` 实例存在，可按 §4 的方法继续逐个恢复）。
5. `transform.rs` 的 `look_at` / `abs_diff_eq` 仍是 `todo!`（超出“只改这两处”）。
