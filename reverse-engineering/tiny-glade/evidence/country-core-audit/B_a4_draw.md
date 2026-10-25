# B 阶段 a4_draw：DrawBuilder / DrawStream / RasterPass / framebuffer_binding / raster_single

范围（reconstruction/country-core/src/render/ 下，仅这 5 个文件）：
`pass_builder/draw_builder.rs`、`draw_stream.rs`、`draw_list.rs`、`framebuffer_binding.rs`、`raster_single.rs`。
证据来源：`procs.tsv` 中 `country_core::render::pass_builder::{draw_builder,draw_stream,draw_list,framebuffer_binding}` 与 `render::raster_single`
共 192 个过程，按 `norm()` 归并后是下表 ~40 个函数体（`read_storage_buffer` 43 实例 = 1 个泛型，`optional_uniform` 3 个，`bind_texture_to_draw_builder` 6 个实例 = 4 类函数体，`raster_single_object` 3 个实例 = 1 个泛型）。
O = 机器码观测，I = 推断，U = 未知（未编造）。panic 位置（`draw_builder.rs:85/91/151/159/160/178/189/315/609/623/650/654/770`、`draw_stream.rs:204/262`）取自 `core::panic::Location` 静态数据。

## 1. 关键纠正（旧代码与机器码不符）

| 项 | 旧代码 | 机器码（O） |
|---|---|---|
| `DrawBuilder +0xe0` | `*const GameContext`（自造的 registry/uniform 表/placement map） | 是 **`&mut RasterPass`**：`+0x00` draws `Vec<DrawPacket(0xb8)>`、`+0x18` `Vec<RenderPassBinding(0x44)>`、`+0x60` `HashMap<&str,u8>`、`+0x90` PassBuilder、`+0x1a8` `PassResources`（`+0x1b8` ssbo 库、`+0x1c8` placement 表、`+0x1d0` `Mutex<DynamicConstants>`）、`+0x1e0` 设备、`+0x1f0` world、`+0x1f8` 视口 |
| `DrawBuilder` 本体 | `draw_commands/mode/draw_args/...` | `+0x00` `NoDropOnlyTake<DrawPacket>`（首 u64 = niche，2 = 已取走）、`+0xb8` `Option<DynamicConstantsBinding>`（`+0xde==8` = None）、`+0xe0` pass、`+0xe8` `Arc<CompiledShaderPipeline>`、`+0xf0` bool |
| `DrawPacket` | `{shader, blend, HashMap<TextureBindingKey,()>}` | 0xb8 B：`head[2]`@0、`Vec<(u32 binding,u8 slot)>`@0x10、`Vec<u8>`@0x28、**draw 种类 u32+载荷@0x40**、`HashMap<BindingIndex,GpuOwned<DescriptorSetBinding>>`@0x70、shader@0xa0、光栅状态 u32@0xa8、blend 1.0@0xac、variant@0xb0 |
| draw 种类 | `mode` 字段 1/3/4/5/7/8 | 0 NoVao、1 NoVaoIndexed、2 NoIndexBuffer、3 InstancedGlDrawFlag（`DrawPacket::new` 缺省 flag=1）、4 MultiInstanced(`Vec<[u32;2]>`)、5 MultiDrawIndirect、7 IndexedIndirect、8 IndexedIndirectCount |
| `draw_*` | 返回 `DrawBuilder` | **终结操作，无返回值**（取走 packet 压入 `pass.draws`，随后 drop self） |
| `TextureBindingKey` | `{texture:u32, view_hash:u64}` | **u64**：`0x4000_0000_0000_0000 \| id`，有 view 时 `^ calculate_hash(view)`（0x140867940 / 0x1408ee800） |
| `impl for &Texture` 键转换 | 返回 key | **恒 `unreachable!("Raw textures must not be used as rg resources")`**（0x1408eefd0，draw_stream.rs:204:9） |
| `DrawPassRenderCheck::drop` | `assert!(self.0, …)` | `!rendered && !thread::panicking()` → `panic!("RasterPass dropped without a call to render()")`（0x140896d80） |
| `ColorAttachment` | `{texture,word_1,word_2}`+5 槽绑定 | 0x20 B 枚举：tag0 引用@+8，tag1 带 16 B 视图字@+4、引用@+0x18；访问类型 tag0→0x36、tag1→0x2e（组 1）；`desc()` 两变体都返回 `ptr+4`（0x1409092f0） |
| `DepthAttachment` | `{texture, layer}` | tag0 可写（访问 0x2f）/ tag1 只读（0x17）/ 其它=无；`+4` 层、`+8` 纹理、`+0x10` u32 非 0 = 清除、`+0x14` f32 清除值 |
| `write_rg_storage_buffer` | 与 read 同 | 访问类型 **0x37、同步组 0**；read = 0x24、组 1（两函数体其余完全相同） |
| `vk_sync_fork::AccessType::is_write`（0x140c10e10） | — | `(access - 0x28) < 0x10`，即 0x28..=0x37 为写（因此 0x2e/0x2f/0x36/0x37 写、0x17/0x1d/0x1f/0x24/0x02 读） |

## 2. 过程 → Rust 项

| 地址 | 原符号 | Rust |
|---|---|---|
| 0x140867850 | `DrawPacket::new` | `DrawPacket::new(shader, head, default_state)`（`+0x40 = 0x1_0000_0003`、`+0xac=1.0`、`+0xb0=0`） |
| 0x140867940 | `TextureBindingKey::with_view` | `TextureBindingKey::with_view` |
| 0x1408eefd0 | `impl$8::to_texture_binding_key` | `impl ToTextureDrawStreamBindingKey for &Texture`（panic）；Handle/ReadOnlyHandle 实现为 `0x4000.. \| id`（I，取自 bind 函数体） |
| 0x140896d80 | `drop_in_place<DrawPassRenderCheck>` | `impl Drop for DrawPassRenderCheck` |
| 0x140867970 | `RasterPass::new_impl`（`new` inline） | `RasterPass::new`：颜色附件 ≤5（ArrayVec 溢出 panic）→ 每个 `add_resource_access(id, 0x36/0x2e, 1)`；深度 0x2f/0x17、组 1；其余（PassResources、描述符集克隆、两张空表）由宿主边界承担 |
| 0x140868370 | `RasterPass::render` | `RasterPass::build_render_pass_desc` + `render`：置已渲染；颜色描述 `new(format).sample_count(n)`、`+4` 覆盖为 tag，`+0xc` 无效即停止收集（上限 8，溢出 panic）；深度 `clear_input`、深度字节（可写标志，无深度=3）；设备缓存 `rustc_entry` 去重（命中克隆 / 未命中 `create_render_pass` 后插入）；`PassBuilder::render` 提交 |
| 0x140905be0 | `DrawBuilder::new` | `DrawBuilder::new(pass, packet, variant)`：写 variant、取反射（缺失 `Option::unwrap` panic）、**仅当反射有 uniform 块**才分配动态常量并 `insert` uniform 绑定，否则 `None` |
| 0x14089a140 | `impl Drop::drop` | `impl Drop for DrawBuilder`：未 draw/discard 且未在 panic → `panic!("The DrawBuilder for {name} was not drawn or discarded.")` |
| 0x140906150 / 0x1409067e0 | `read_rg_storage_buffer` / `write_rg_storage_buffer` | 同名；名字表空/未命中 → 原样返回；`set != 0` → `assert_failed<SetIndex>`；pass 绑定 ≤0x100 |
| 0x140906400 | `bind_external_storage_buffer` | 同名：`{vk_buffer,0,u64::MAX}`+句柄克隆 `insert`；`get_vk_buffer` 为 None → `Option::unwrap` panic |
| 0x140906a90 / 0x140906cb0 / 0x1409083f0 / 0x140907490 / 0x140907050 | `draw_no_vao`/`_indexed`/`draw_no_index_buffer`/`draw_instanced_w_gl_draw_flag`/`draw_multi_instanced` | 种类 0/1/2/3/4 |
| 0x1409076b0 / 0x140907990 / 0x140907cc0 / 0x140907f90 | `multi_draw_indirect` / `multi_draw_indexed_indirect` / `draw_indexed_indirect_raw` / `draw_indexed_indirect_count` | 种类 5/7/7/8；访问 `add_resource_access(res, 2, 1)`；`multi_draw_indexed_indirect` 取静态表首字（`0x142aaaa78 + idx*0x14`，已核对前 10 项） |
| 0x140906f40 / 0x1409082c0 | `draw` / `discard` | `draw` 不改种类；`discard` 把所有 `GpuOwned` 状态置 2 |
| 0x140907280 | `draw_instanced_from_placement_data` | placement 表查找（缺失 `expect("This Handle for PlacementDataSSBO doesn't have a corresponding buffer")`）→ `bind_external_storage_buffer(0x30e6778252531a83)` → `draw_instanced_w_gl_draw_flag(flag)` |
| 0x141629a20/b90/d20 | `optional_uniform<f32/Mat4/[u32;2]>` | `optional_uniform` / `uniform`（行为相同，0x14096b850 核对：未命中静默、块外越界 panic、`bool` 以 u32 写） |
| 0x141629e90 … | `read_storage_buffer<T>`（43 实例） | `read_storage_buffer<T: SsboType>(slot_name, binding_hash)`：槽表 entry（空位 `read_access_buffer`：访问 0x1f、组 1）→ 名字表 → `packet.accesses.push` |
| 0x1403728a0/0x1408ee800/0x1408fd6c0/0x140908610/0x140908c50/0x14096f650 | `bind_texture_to_draw_builder<T>` | `bind_texture_to_draw_builder(BindTexture::{RgHandle,Preset,FromDisk,Raw})`：RG 路径（Handle=ReadOnlyHandle 同体，写访问 panic "Trying to bind {:?} for writing, but it's not writable"）按 `TextureBindingKey` 去重槽并登记访问/绑定；Preset 允许写（组 0）；FromDisk/Raw：`assert!(!access.is_write())`，Raw 另 `assert!(view.is_none())`，`get_vk_image_view` 后 `{0, view, 5}` 绑定；末尾 `with_suffix_dims` 维度 uniform |
| 0x14090b460 | `bind_common` | `bind_common<C>(params, transform)`：flag→`+0xf0`；`write_uniform<bool>(0x418a…)`；预设 flag?0x12:0x11 绑到 0x11651f4d…（访问 0x1d）；`optional_uniform<Mat4>(0x14658de1…, compute_matrix)`；`write_uniform<u32>(…flag)`；时间 f32、视口 `[u32;2]`；`bind_color_library`（槽 "ColorLibrary"，binding 0x52ec6a17beb8fef0） |
| 0x14090bb70 / 0x14090c300 / 0x14090c980 | `bind_wall_color_customization` / `bind_stone_floor_textures` / `bind_wooden_floor_textures` | 磁盘纹理表 4 / 12 / 12 项（常量由反汇编脚本批量抽取，非手抄）；墙色末尾 `read_storage_buffer<WallColorIdSsbo>`（槽 "WallColorId"） |
| 0x1409cdec0 / 0x1409ce780 / 0x1409d00a0 | `raster_single_object<1,[ColorAttachment;1],CLOSURE>` ×3 | `raster_single_object<N,F>`：视口=首附件 extent → `RasterPass::new`（无深度）→ `DrawPacket::new` → `DrawBuilder::new(variant 0)` → 闭包 → `draw_no_vao(3)` → `render`；三个实例都是 `edx=3` |

## 3. 未做 / U

- **U `DrawBuilder::bind_brick_erase_highlight`（0x14090be10，1253 B）**：只确认读 4 个 world 资源（`get_resource_id` 4 个 TypeId + `unwrap_failed`×4）、3 个 uniform（hash 0x203fd61c57064e54 / 0x06bdce4199980bf7 / -0x4a7a31de4938d73c）、`CursorTerrainRaycast::position` 参数；依赖 `world` 资源布局，本轮未恢复（本文件没有该函数，未写占位）。
- U `RasterState` 各位名称（只知 `byte0 &= 0x7f`、`byte3 = (byte3 & 0x9f) | 0x40` 出现在 mei_pass 实例闭包内）；`vk_sync_fork::AccessType` 数值只核对了 `is_write` 范围与 0x02=IndirectBuffer，其余名称 U。
- U `DrawPacket.head[2]`（`+0`/`+8`，`new` 的 r8/r9）、`+0x28` 的 `Vec<u8>`、`+0xb1`。
- U `multi_draw_indirect`/`IndexedIndirect*` 里各 u64 的语义（只记偏移：`arg_48/arg_50/arg_58`）；`INDEXED_INDIRECT_INDEX_COUNTS` 只核对前 10 项（原函数无边界检查，表长未知）。
- U `RenderPassAttachmentDesc` 0x10 B 内部布局（`valid` 标志位 I：`format != 0`）、`DescriptorSetBinding::UniformBuffer` 的原始判别值。
- `draw_list.rs`：`DrawListItem::to_draw_packet` 全 inline（22 宿主，无 PDB 过程）。机器码只证明列表元素是 `(DrawListItem, fn(DrawBuilder,&EntityDrawParams,&mut World))` 元组 + 稳定排序；字段 `sort_key/shader_id`、`to_draw_packet` 形态与排序键均为 **I**。
- puffin profile scope（`raster_single_object` 开头）未复刻（离线缺失，不影响游戏可见行为）。
- 动态常量里 `with_suffix_dims`、`DynamicConstants::allocate`、`FrozenMap`、`get_vk_*`、`create_render_pass`、`PassBuilder::render`、纹理库查找全部经 `PassHost` 边界 trait 表达（调用点即 O，实现者是 rhapsody / 资源库）。

## 4. stub 清单（均在本组文件内；没有新增第二份 contract）

- `draw_stream.rs::PassHost`（外部调用面，10+ 方法，含 O 调用点注释）、`ShaderReflection`（`CompiledShaderPipeline` 反射字段最小替身：名字表 +0x98、uniform 块 +0xe8、偏移表 +0x118、块大小 +0x138、块 binding +0x140）、`DynamicConstantsBinding`、`RasterRenderPassDesc`/`AttachmentDesc`（`RenderPassAttachmentDesc` 的最小替身）、`RasterPassSubmission`。
  原因：`rhapsody_types::CompiledShaderPipeline` 是不透明 `Vec<u8>`、`rhapsody_contract::RenderPassDesc{selector}` / `RenderPassAttachmentDesc` 是单元/单字段占位，承载不了机器码里读写的字段，且我不能改共享文件。
- `framebuffer_binding.rs::AttachmentTexture`：`rhapsody::raw::texture::Texture` 被读取部分（`id`@0、`format`@0xc、`sample_count`@0x21、extent），带 `From<&rhapsody_types::Texture>`。
- 复用已有：`rhapsody_contract::{Handle, ManagedHandle, ReadOnlyHandle, TextureViewDesc, RenderPassBinding}`、`rhapsody_types::Texture`、`render::utils::calculate_hash`、`resources::texture_presets::{TexturePreset, TextureFromDisk}`、`pass_resource_access::SsboType`（仅 `rg_buffer_hash()`）、`render::shader::RasterShaderProgram`。

## 5. 对公共 API 的改动（旧 → 新，原因均为机器码）

- `DrawBuilder` → `DrawBuilder<'p>`（借用 `&'p mut RasterPass`）。**`render/nani/draw_ext.rs`（另一 agent）里 `impl NaniDrawExt for DrawBuilder` 需要补 `<'_>`**，并且它依赖的旧签名（`draw_*` 返回 builder、`read_storage_buffer::<T>()` 无参）已变：`draw_*` 现在是终结操作返回 `()`；`read_storage_buffer<T>(slot_name, binding_hash)`；`read_rg_storage_buffer(name_hash, buffer_resource)`；`draw_indexed_indirect_count(buffer_res, count_res, args[4])`；`self.variant` → `DrawBuilder::variant()`。
- `DrawBuilder::new(&World, Handle, u8)` → `new(&mut RasterPass, DrawPacket, u8)`。
- `RasterPass::new(&[ColorAttachment], &GraphicsDeviceResource, &World)` → `new(name, &[ColorAttachment], Option<&DepthAttachment>, Box<dyn PassHost>)`（+ `with_viewport`）；`render(self, device)` → `render(self)`；删除 `get_render_pass`。
- `DrawPacket::new()` → `new(shader, head, default_state)`；`DrawPacket` 字段全变。`TextureBindingKey` 变为 u64 新类型；`DrawKind::KindN` 枚举换成带载荷的真实枚举。
- `ColorAttachment`/`DepthAttachment` 重写；删除 `RasterFramebufferBinding`、`AsTexture`、`DrawArgs`、`GameContext`、`UniformValue::write_at`（改 `write_bytes`）、`RasterSsbMarker`、`impl BindGameResources for DrawBuilder`（`bind_color_library` 已直接是 `DrawBuilder::bind_color_library<C>()`，原版是内联展开的泛型实例）。
- `DrawCommand` 保留为 `pub type DrawCommand = DrawPacket`（`lib.rs::consume::pass_builder` 的 re-export 不用改）。
- `pass_builder/mod.rs` 的 `pub use draw_stream::{DrawKind, …}` 仍有效；`DrawStream` 保留（无 PDB 过程，空壳）。
- 未触碰共享文件（`lib.rs`、`rhapsody_types.rs`、`rhapsody_contract`、`mod.rs`）。集成需要：`lib.rs:488` 无需改（别名）；`draw_ext.rs` 需跟进（见上）；`DrawBuilder::bind_common` 的真实调用者 `common_draw_params.rs`（还在 `unimplemented!`）可改调 `DrawBuilder::bind_common::<ColorLibrarySsbo>` 等。

## 6. 测试与验证

- 隔离测试台 `/tmp/h_a4_draw`（`#[path]` 挂载 5 个真实源文件 + 真实 `rhapsody_types.rs`、`render/utils.rs`、`texture_presets.rs`；`rhapsody_contract`、`shader::RasterShaderProgram`、`SsboType` 为最小替身）：
  `L0H_REAL=$PWD/reconstruction/country-core/src L0H_WORK=/tmp/w_a4_draw CARGO_TARGET_DIR=/tmp/l0h-target /tmp/h_a4_draw/run.sh`
  → **67 passed, 0 failed, 0 warnings**。5 个文件 `rustfmt --check --edition 2024` 通过。
- 覆盖：draw_stream 15（`DrawPacket::new` 缺省、种类判别、键标记/view 异或、`&Texture` 转换 panic、`DrawPassRenderCheck` 两分支及 panic 文案、`is_write` 范围、状态位改写、new 的访问登记、>5 附件 panic、render 缓存命中/未命中、take_while 终止、drop 未渲染 panic）；framebuffer_binding 5；draw_list 2；draw_builder 40 余个（new 带/不带 uniform 块、未知 shader panic、Drop 文案、NoDropOnlyTake、全部 draw 种类与载荷、间接绘制访问与静态表、uniform 偏移/Mat4/bool/越界、rg 读写访问类型与组、set≠0 断言、槽复用、外部缓冲绑定与 vk None panic、`discard`、placement 命中/缺失文案与长度 0x45、RG/Preset/FromDisk/Raw 纹理路径及全部断言、bind_common 两个 flag 值逐字节、bind 表形状与常量、>256 绑定 panic）；raster_single 3（闭包→draw_no_vao(3)→render 的顺序与提交内容、视口 = 首附件 extent、未消费 builder 的 drop 检出）。
- 本文件组内已无 `todo!`/`unimplemented!`（`grep` 无命中）。

## 7. 遗留

1. `bind_brick_erase_highlight`（见 §3）。
2. `draw_builder.rs` 含大量内联测试后约 1.8k 行，超过 800 行指引；常量表（rustfmt 纵向展开）和测试占大头，业务代码约 600 行。如需拆分可把 `#[cfg(test)]` 移到 `draw_builder/tests.rs`。
3. `PassHost` 的真实实现（world 资源 → `PassResources`、`GraphicsDeviceResource`）属于集成层，待与 a4_compute 的 `pass_resources.rs` 合并时落到同一个 host。
4. `draw_ext.rs`（nani）与各 `draw_passes/*`、`common_draw_params.rs` 需要按 §5 的新签名接线。
