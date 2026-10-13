# B 阶段 i_api：A4 pass_builder / A3 ssbo_library 公共 API 修正

范围（只改自己拥有的文件）：`render/pass_builder/{draw_builder,draw_stream,framebuffer_binding,compute_pass_builder,pass_resource_access,bind_resources,shader_uri,mod}.rs`、
新文件 `render/pass_builder/rg_resources.rs`、`render/ssbo.rs`、`asset_libraries/ssbo_library.rs`、`rhapsody_types.rs`（只追加）。
O = 机器码观测，I = 推断，U = 未知。方法：`dis.py` 逐条复核各组报告的断言；复核发现的**既有错误**见 §1。

## 1. 复核发现的既有错误（均已按机器码改正）

| # | 来源 | 旧断言 | 机器码（O） | 处理 |
|---|---|---|---|---|
| 1 | a4_draw `bind_common` | 第一个 uniform（`0x418a98441f59a54c`，bool）写 `flag` | 0x14090b4ea：`test r14d,r14d; je` → 非 0 路径 `xor r9d,r9d`（false），0 路径 `mov r9b,1`（true）。**值 = `flag == 0`**（预设 0x12/0x11 的选择方向对） | `draw_builder.rs` 改正 + 测试 |
| 2 | a4_draw / d_shaders | `WallColorId` 槽名 | `read_storage_buffer<WallColorIdSsbo>`（0x14162c290）`rustc_entry` 键 `mov r9d,0xc` = **"WallColorIds"**（复数；`xxh3_64` = SsboId 0xaf42315d263aa22a 吻合） | `WALL_COLOR_ID_SLOT_NAME` |
| 3 | a4_compute `bind_resources` 载体 | `rg_buffer_hash()` = binding 名哈希；NAME = "RtTrianglesSsbo" 等 | `get_rg_buffer<T>` 立即数（0x1412c7bb0/d1e40/d4150/cc500）= `xxh3_64(NAME)`，**NAME = "RtTriangles" / "RtTriangleMeta" / "RtBvh" / "WallColorIds"**（四个全部用 xxh3 复算吻合）；`ColorLibrarySsbo` 同样应返回 `xxh3("ColorLibrary")` | 载体改 NAME + `rg_buffer_hash = SsboId`，旧测试 `legacy_ssbo_hash_trait_matches_binding_hashes` 改成断言机器码常量 |
| 4 | a4_compute / c2 | `write_storage_buffer_concurrent` 无实例，访问码取 0x32 | 0x141608450（`NaniDrawListSsbo`）：`mov byte [rsp+0x20],0x37`、`mov [rsp+0x28],r9d` → 访问码 **0x37**，同步组由调用方 | `compute_pass_builder.rs` + 测试 |
| 5 | d_shaders §3.2 / B_i 之前 | 虚表槽 +0x18 "取出 InstanceBuffer（count 在 +0，handle 在 +0x68）" | 虚表 `[drop,size,align,+0x18 as_any,+0x20 as_any_mut,+0x28 buffer,+0x30 handle]`；**78 个 `dyn Ssbo` 虚表 = 四组实现**：`impl$0` `TypedStorageBuffer`（size 0x28，buffer/handle `mov rax,rcx`）×38、`impl$2` `InstanceBuffer`（0x30，`lea rax,[rcx+8]`）×19、`impl$1` `MappedStorageBuffer`（0x38，`lea rax,[rcx+0x10]`）×21、`sparse_instance_buffer::impl$12` `SparseInstanceBuffer`（0x90，`lea rax,[rcx+0x68]`，0x140005c50）。`buffer()` 与 `handle()` 在每组里是**同一个桩**（ICF）。+0x18 就是 `as_any`（返回 `(自身指针, Any 虚表)`）；调用点对**具体对象指针**直接读：`ivy_leaf/roof_spire/roof_support_beam` 读 `[rax]` = `InstanceBuffer.instance_count`；`indirect_draw_bricks` 的 `+0x68` 是 **`SparseInstanceBuffer<BrickTransformSsbo>`** 的内嵌缓冲，不是 InstanceBuffer。原版没有再比对 TypeId，只有名字断言（`assert_failed<&str,&str>`） | `ssbo_library.rs` 模块文档 + `AssetSsboLibrary::{get,instance_count,lookup}` |
| 6 | a3 / d_shaders §5 | `MappedStorageBuffer` 固定 0x40000 B | 21 个 `impl$4::new_storage_buffer` 实例：缓冲大小 = `BUFFER_LEN × size_of::<T>()`（0x40000 只是 6 个 u8 元素 staging 的巧合：Cobblestone1/2/3、Garden、Path、WaterRasterStaging；TerrainHeight 0x80000=4B×0x20000、Butterfly 0x300、Firefly 0x600、FireflyVisibility 0x180、BushCpu 0x12c08、Mouse 0x20、StairsCounter 4…）；memory 2 或 4；对齐 = `align_of::<T>()`（1/4/0x10）；返回 0x38 B = `{映射切片 ptr, **元素数**, Buffer(0x28)@0x10}`；上限比较 `len >= size+es`；`None` 时 `BUFFER_LEN` 个缺省元素（Butterfly）或零 | `render/ssbo.rs` 重写 |
| 7 | c3 §8.4 | `bind_texture` 第 5 个实参值 2 = "视图种类" | 第 5 实参是 `&Option<TextureViewDesc>`，`None` 的编码是首 dword = 2（Option<u32> 标签的 niche）；调用点（如 0x1409c01fc `mov dword [rbp+0x1c0],2`）传的就是 `None` | 见 §3.2 |

## 2. 任务 1：pass_builder draw 侧

| 项 | 改动 |
|---|---|
| `CommonDrawParams.flag` / `DrawBuilder +0xf0` | `bool` → `CommonDrawFlag`（`#[repr(u8)]`，0 Off / 1 On / 2..=4 `ShadowCascade0..2`；`from_byte`、`shadow_cascade(c)`=`c+2`、`is_set`、`From<bool>`）。`DrawBuilder::common_flag()` 返回该枚举（`as u8` 即原字节）。`bind_common`：`write_uniform<u32>` 写**整个字节**（2..=4 不再被截成 1）；bool uniform 修正为 `flag==0`（§1.1）。证据：`movzx r14d, byte [r8+4]`、`mov [rdx+0xf0], r14b`、`mov r9d, r14d`。 |
| `ColorAttachment` tag 2 | 新增 `ColorAttachment::Tag2(AttachmentTexture)`：引用在 +8（`from_iter` 0x14157b150 `rax = (tag==1)*0x10+8`）、访问 0x2e（`(tag==0)*8+0x2e`）、无视图字、描述 `+4` = 2。`RasterPass::new/build_render_pass_desc` 已按 `tag()` 工作。 |
| `PassHost` RG 资源创建 | 新文件 `rg_resources.rs`：trait `RgResources`（`create_raw_texture/buffer`、`get_or_create_temporal_texture/_pair/_buffer`，调用点 0x140be94d0 / 0x1404f6e50 / 0x1404f7450）+ 结果类型（`TemporalTexture/TemporalTexturePair/TemporalBuffer/RgError`）+ 记录型 `RecordingRg`。`PassHost` 追加**带缺省实现**的 `rg_resources()`（缺省 `None`）与 `ssbo_buffer_word()`（缺省 `None`），`require_rg_resources(host)`、`RasterPass::host_mut()`。已有 `PassHost` 实现者（`RecHost` 等）零改动即可编译。`MockHost` 增 `rg`、`buffer_words` 字段。 |
| `bind_ray_tracing<DrawBuilder>`（0x1416289d0） | `DrawBuilder::bind_ray_tracing::<Tri,Meta,Bvh,Wall>()`：先取 Tri/Bvh 缓冲字（经 `PassHost::ssbo_buffer_word`，缺失 `Option::unwrap`）→ 三个槽 "RtTriangles"/"RtTriangleMeta"/"RtBvh"（`read_access_buffer` 0x1f + binding 0x168ddb…/0x47ddeb…/0x339e76…）→ `read_storage_buffer<Wall>`（槽 "WallColorIds"）→ 两个 `write_uniform<u32>`（0x1c9f2eef0ac739ac / 0xc2b7f2fdf5e21521）。槽表复用 `read_storage_buffer` 的路径（已存在槽不重复登记访问）。 |
| `EntityDrawParams` entity 访问器 | `EntityDrawParams::entity()`（固有 impl 放在 `draw_builder.rs`，不碰 `draw_list.rs`；原版 `[rdx+8]`）。若另一个 agent 统一 `DrawItem` 时在 `draw_list.rs` 里也加了同名方法会重复定义，二选一删除。 |
| `INDEXED_INDIRECT_INDEX_COUNTS` 24 项 | 逐项 `rdva` 读 0x142aaaa78+i×0x14 复核：24 项全部与 d_shaders 写入值一致（第 10、11 项为 0；第 24 项起是别的数据）；加测试固化。 |
| TextureViewDesc | 见 §3.1（draw 侧改用真实类型，签名不变）。 |

## 3. 任务 2：compute 侧

### 3.1 `rhapsody_types.rs`（只追加，共享）
- `TextureDesc` 追加 `storage: bool`、`format: u32` 与 `storage()`、`new_2d(extent2, format)`、`with_format`、`extent_2d()`（0x140bd5390 / 0x140bd52a0 / 0x140bd5430 / 0x140bd5790）。**唯一的被动适配**：`render/load_texture.rs:182` 的测试里 `TextureDesc{..}` 全字段字面量加了 `..Default::default()`（一行）。
- 新增真实 `TextureViewDesc`（0x3c B；逐个反汇编 `new/view_type/usage/format/aspect_mask/base_mip_level/base_layer/level_count/layer_count/bind_individual_mips`）：`view_type/usage/format/aspect_mask/level_count/layer_count: Option<u32>`（dword 标签+值）、`base_mip_level/base_layer: u32`、`bind_individual_mips: bool`；`level_count/layer_count` 的参数是 `Option<u32>`（`r8d` 标签、`r9d` 值），其余构造器取 `u32`。`aspect_mask(4)` = stencil（c3 的 `read_texture_view`）。
- **需要集成者做的唯一共享改动**：`lib.rs:188-190` 的 `rhapsody_contract::TextureViewDesc`（单元结构体）改成 `pub use crate::rhapsody_types::TextureViewDesc;`。pass_builder 里全部 `TextureViewDesc` 现在取自 `rhapsody_types`；不改的话 `ui_2d_pass` 等传 contract 单元类型的调用点会类型不符。

### 3.2 `bind_texture`
O（0x140363db0 / 0x140155400 / 0x1402af750 / 0x1408fcd40 带视图的 3 标签实例；0x1409056d0 `TextureFromDisk`；0x1408ee240 `&Handle`、0x1408edd00 `&mut Handle`、0x140905260 `TexturePreset` 的无视图实例）：
- `as_texture` 返回 **3 个标签**：0 = RG（`{extent,id,desc}`）、1 = 受管（`{extent,vk view,句柄克隆}`）、2 = 按名字（`cmp eax,2; je` → `AssetTextureLibrary::get_by_name`，`extent_2d(desc@+0x18)`、`get_vk_image_view(.,0)`、`handle.clone()`）。
- RG 路径：绑定数检查 → `add_resource_access(res, access, write?0:1)` → `Ref::bind_view`（读 0x140bd6c00 / 写 0x140bd6c30，**整块 `TextureViewDesc` 进入绑定**，+0x3c 资源号、+0x40 写标志）→ `AccessRange` → 维度 uniform。
- 外部路径（标签 1/2）：`assert!(!access.is_write())`（"assertion failed: !access.is_write()"，0x24 字节）→ 视图必须为 `None`，否则 `panic!("TODO?")`（0x142aaa7c8，已读出字符串）→ 追加 0x48 B 外部记录 `{binding@0, 判别 0@8, sampler 0@0x10, image_view@0x18, layout 5@0x20, 句柄克隆@0x28, 标志 0@0x40}` → 维度 uniform；**不登记 RG 访问、不追加 AccessRange**。
- 只读类型（`TextureFromDisk`、`&Handle`、`&ReadOnlyHandle`）在**注册表查找之前**：`is_write(access)` → `panic!("Trying to bind {:?} for writing, but it's not writable")`（0x142aaa898 的两段 pieces 已读出）。
- 签名：`bind_texture(name, &tex, access)` 不变（= `bind_texture_with_view(.., None)`）；新增 `bind_texture_with_view(name, &tex, access, Option<&TextureViewDesc>)`。`PassBinding::ImageView` 增 `view: TextureViewDesc`；`TextureBinding` 增 `ByName{name_hash}`（标签 2）；`ExternalSlot` 增 `Image{sampler,image_view,layout,handle,flag}`；`ComputeTexture` 增带缺省实现的 `target()`/`write_guard()`，`rg_resource()` 改为带缺省；新增实现：`TextureFromDisk`、`&Texture`（原生，标签 1）。`&RgHandle`/`&ReadOnlyHandle` 的目标改为 RG（句柄自带 id/desc；旧实现经受管 `TextureAccess` 走偏）。
- `ImportedTexture` 带 desc：O 0x140a59800 里 `get_rg_texture` 返回 36 B `{id:u32, desc:TextureDesc}`。`texture_library.rs::ImportedTexture` 追加 `desc`（**改了非我拥有的文件，仅 +1 字段、+1 处构造**）；`preset_texture` 改用 `imported.desc`（不再回头查 `PresetTextureState.texture`）。

### 3.3 `ComputeShaderSource` defines / `ComputePass::new`
O：`ComputePass::new<TgsrMain/BinLights/HalfResDof/…>`（2311 B）与 `new<&str>`（1943 B）形态不同——带 `PERM_*` 的源在 `as_compute_pipeline` 里直接产出带 `defines: Vec<(String,String)>` 的 `ComputePipelineDesc`。
`ComputeShaderSource.defs: ShaderDefValue(u64)` 保持不变（既有 5 个实现用字面量构造，改类型会破坏它们），改为在 `AsComputeShaderSource` 增**带缺省实现**的 `fn pipeline_defines(&self) -> Vec<(String,String)>`（缺省空，`Box<dyn>` 转发）；`ComputePass::new` 的 `compute_desc` 把它放进 `ComputePipelineDesc.defines`（原先恒空）。另有 `shader_uri::{ShaderDefine, perm_define}`、`ComputePipelineDesc::with_define`。

## 4. 任务 3：ssbo / ssbo_library

- `StorageElement` 追加 `const MAPPED_MEMORY: u8 = 2`（常量 `MAPPED_MEMORY_UPLOAD=2`/`MAPPED_MEMORY_READBACK=4`，语义 U/I）；`InstanceCounted` trait（`InstanceBuffer` 实现）。
- `MappedStorageBuffer<'a,T>`：`{ mapped: &'a mut [T], buffer: Buffer }`（旧：`mapped: &mut [u8]`）；`new_storage_buffer(device: &dyn GraphicsDevice, Option<&[u8]>)`（旧：`<D: GraphicsDevice + MappedBuffers>(&'a D, ..)`）——`mapped_slice_mut` 并入 `GraphicsDevice` trait（带缺省 `None`，与原版 `Option::unwrap` 路径一致），`MappedBuffers` trait 删除（全 crate 无引用）。契约设备占位 `mapped_slice_mut` 返回泄漏的 16 B 对齐零页。新增 `cpu_write(&mut self, &[T])`（0x14230edb0/0x14230f1e0：`n=min(len,mapped.len())` + `copy_from_slice`，数据过长触发长度不匹配 panic；puffin scope 略）。`MAPPED_BUFFER_BYTES` 保留为历史常量（6 个 u8 staging 的大小）。
- 因此**蝴蝶/萤火虫/可见性/BushCpu/MouseTerrainRaycast/…都能实现库侧 `SsboType`**：`type Storage = MappedStorageBuffer<'static, T>; fn new_storage_buffer(d, i) { MappedStorageBuffer::new_storage_buffer(d, i) }`（测试 `Fly` 同构；`ssbo_library` 的 `mapped_buffers_can_be_library_ssbos…`）。`FireflyVisibilitySsbo` 需 `const MAPPED_MEMORY = MAPPED_MEMORY_READBACK`。
- `AssetSsboLibrary` 新增：`get::<T>() -> Option<&T::Storage>`（名字断言 + 安全 `downcast_ref`）、`contains::<T>()`、`instance_count::<T>() -> usize`（`T::Storage: InstanceCounted`；未注册 0，O `xor esi,esi`）、类型擦除 `lookup(SsboId) -> Option<(name, Option<InstanceInfo{instance_count, buffer}>)>`；`Ssbo` trait 追加带缺省的 `instance_count()`；`BufferState::instance_count()`。c2 的 `AssetSsboLibrary::get` 缺口就是 `get`/`instance_count`。
- 与另一个 agent 在同一文件里加的 `Ssbo::cpu_shadow_words`（它把"+0x18 返回 (ptr,len)"当独立虚方法）不冲突；§1.5 说明该槽就是 `as_any`，它读到的 `(ptr,len)` 是具体对象（`ShadowedBuffer`）的字段，槽位序号标注可更正为 O。

## 5. 对公共 API 的改动清单与调用方要做的事（我没有改这些文件）

grep 了全 crate（`systems/render_loop/**`、`render/nani/**`、`resources/**`）。**会编译失败的**：
1. `lib.rs:188-190` `rhapsody_contract::TextureViewDesc` → `pub use crate::rhapsody_types::TextureViewDesc;`（§3.1，必须）。
2. `systems/render_loop/compute_passes/dof_autofocus.rs:324` 测试里的模式 `PassBinding::ImageView { resource: 55, write: false }` → 末尾加 `, ..`。
3. `systems/render_loop/draw_passes/common_draw_params.rs:25`、`d_main_support.rs:151`：`CommonDrawParams::new(elapsed, flag)` 的 `flag` 现为 `CommonDrawFlag`（d_main 已改成 `impl Into<CommonDrawFlag>`，与本文一致；`ItemMode::Shadow` 的覆盖写可用 `CommonDrawFlag::shadow_cascade(c)`）。
4. `d_main_support.rs:170-182` 的 tag2 补丁可删，直接 `ColorAttachment::Tag2(tex)`。
**仍能编译但应该跟进的**：
5. `systems/render_loop/indirect_params/pass_env.rs:185` 临时 `impl ComputeTexture for FromDisk`（多登记一条 RG 访问，偏差）→ 改用真实 `TextureFromDisk`；c2 `c2_support::{SpdViewBinding, DiskTextureBinding, record_with_views}`、c3 `half_res` 的 stencil 视图旁路 → 改用 `bind_texture_with_view(.., Some(&TextureViewDesc::new()…))`。c2 自带的 `TextureViewDesc{u8 view_type/usage…}` 与真实类型字段不同（真实：`Option<u32>`），换用时注意。
6. `AsComputeShaderSource` 的 5 个带 `PERM_*` 实现（`taa.rs::TgsrMain`、`dof.rs` 五个、`bin_lights.rs::BinLights`、`rasterize_terrain_stroke.rs::GaussBlur`、`vermeer.rs::GenerateStrokes`）各加 `fn pipeline_defines(&self)`（它们已有同内容的固有方法 `pipeline_defines()/pipeline_desc()`，直接转发）。
7. `draw_shaders/water_planes_draw.rs`：把闭包参数换成 `DrawBuilder::bind_ray_tracing::<RtTrianglesSsbo,RtTriangleMetaSsbo,RtBvhSsbo,WallColorIdSsbo>()`（`bind_resources` 里的载体已改 NAME/哈希）；`stairs_draw`/`draw_bricks` 取实体用 `params.entity()`；`ssbo_tags::SsboInstanceView` 可由 `AssetSsboLibrary::lookup(SsboId(NameHash(id)))` 实现（`InstanceInfo::buffer` → 它的 `ManagedHandle`）；其文档里"缓冲内句柄在 +0x60 / +0x18 取 InstanceBuffer"的表述按 §1.5 更正（砖块是 Sparse，+0x68）。
8. `systems/render_loop/resources/critters.rs`（及 `read_bush_positions_for_cpu.rs`、`mouse_read_terrain_stroke_index.rs` 等 Mapped 家族）：补库侧 `SsboType`（§4）；`FireflyVisibilitySsbo` 设 `MAPPED_MEMORY`。
9. `render/nani/draw_ext.rs`：`impl NaniDrawExt for DrawBuilder<'_>` 已是新形态，API 未再变；`PassHost` 新方法带缺省，不影响。
10. `lib.rs::consume::pass_builder` 可选再导出 `rg_resources::{RgResources, RgError, TemporalTexture, TemporalTexturePair, TemporalBuffer}`。

## 6. stub / 接缝
- `PassHost::ssbo_buffer_word`（`AssetSsboLibrary::get_buffer` + 名字断言，宿主实现）、`PassHost::rg_resources`（rhapsody `RenderGraph`）——缺省 `None`。
- 契约设备 `mapped_slice_mut` 占位（泄漏零页）。
- `ManagedHandle(texture.id)`、`TextureDesc::new_2d([w,h],format)` 由占位 `Texture` 推出（I；真实 `Texture+0x18` 是整块 `TextureDesc`，U）。

## 7. O/I/U
- O：§1 七项、§2/§3 全部地址与常量、`bind_ray_tracing<DrawBuilder>` 的槽名/哈希/顺序（槽名字符串已从 rdata 读出）、`TextureViewDesc` 布局、外部图像记录布局、`write_storage_buffer_concurrent` 访问码、`cpu_write` 语义。
- I：`CommonDrawFlag` 变体名；`MAPPED_MEMORY` 2/4 的含义；`bind_texture` RG 路径遇 `None` 视图用缺省描述（机器码在 RG 路径不检查 `None`，调用点保证 `Some`）；缓冲字 = `Buffer+0` 低 32 位；`ColorAttachment::Tag2` 命名。
- U：`TextureViewDesc.view_type` 的真实枚举类型（按 u32）；`RenderPassBinding` 判别 2 的含义；`ManagedHandle` 与 `Texture` 的真实关系。

## 8. 验证
```
cp -r /tmp/h_d_shaders /tmp/h_i_api   # 以 d_shaders 测试台为底，增挂 rg_resources / draw_list / raster_single / uniform / indirect，rc_body 的 TextureViewDesc 改为 re-export
L0H_REAL=$PWD/reconstruction/country-core/src L0H_WORK=/tmp/w_i_api CARGO_TARGET_DIR=/tmp/l0h-target /tmp/h_i_api/run.sh
→ test result: ok. 516 passed; 0 failed   （基线 481；本轮净增约 35 个，其余为新挂载的 raster_single/uniform/indirect）
```
被挂载的 draw_shaders 全部文件与 `draw_stream/draw_builder/…` 在该测试台里一起编译、通过（证明 `common_flag() as u8`、`WALL_COLOR_ID_SLOT_NAME` 等外部引用未被破坏）。`render/nani/*` 真实文件依赖 `half`/`components` 等离线缺失项，无法挂载，改用 grep 审计（§5.9）。
新增/修改测试覆盖：bind_common 0..=4 全值（bool=`flag==0`、u32 整字节、预设 0x11/0x12）、`CommonDrawFlag` 字节表、`bind_ray_tracing`（四槽顺序/访问码/复用已有槽/缓冲字缺失 panic）、24 项表、tag2（访问 0x2e、描述 kind_tag 2）、RG 资源创建（含 pass 创建前后、temporal 复用）、`bind_texture_with_view`（视图进入绑定）、`TextureFromDisk` 外部绑定（不登记 RG 访问、记录布局、维度 uniform）、注册表未命中惰性 extent、入口写守卫文案、`!access.is_write()` 与 `TODO?`、PERM defines 到达着色器后端、`write_storage_buffer_concurrent` 0x37、`MappedStorageBuffer`（大小 = BUFFER_LEN×es、memory/对齐、缺省元素、上限/整除断言、`cpu_write`、无映射 unwrap）、`AssetSsboLibrary::{get,instance_count,lookup}`、Mapped 作库侧 SsboType、载体 SsboId 常量。
`rustfmt --check --edition 2024`：`draw_builder/draw_stream/framebuffer_binding/ssbo_library/rhapsody_types` 在 max_width=100 下通过；`rg_resources/bind_resources/pass_resource_access/compute_pass_builder/shader_uri/ssbo` 在 max_width=120 下通过。本范围无 `todo!/unimplemented!`，无编译警告。

## 9. 遗留
1. `draw_builder.rs` 2258 行 / `compute_pass_builder.rs` 1848 行（测试占大头，业务约 700/700 行），超过 800 行指引；可把 `#[cfg(test)]` 拆成子模块文件。
2. `PassHost` 的真实实现（world → `PassResources`/`GraphicsDeviceResource`/`RenderGraph`）仍属集成层。
3. 带视图的 `bind_texture` 与 `write_storage_buffer_concurrent` 之外，`bind_texture<&mut Handle>`（0x1408edd00，无写守卫）没有对应的 Rust 类型（现只有 `&RgHandle`）。
4. `SparseInstanceBuffer` 的实例数（`[rax]`）未核对（砖块调用点只用 +0x68），`Ssbo::instance_count()` 对它返回 `None`。
