# render/pass_builder/draw_stream.rs

> 文件作用：DrawPacket（0xb8B 绘制包，种类判别@+0x40）/ RasterPass（~0x3e8B 光栅 pass：附件数组 + RenderPass 去重缓存）/ DrawStream（无过程空壳）+ TextureBindingKey（u64：0x4000_0000_0000_0000|id，视图异或 calculate_hash）。

函数总计 27：业务 5 / 内联无独立符号 7 / 派生胶水 14

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::pass_builder::draw_stream::DrawPacket::new` | 0x140867850 | 106 | 0xb8B 绘制包：+0x40 种类 u32=0x1_0000_0003（3 变体 + 计数）、+0x70 HashMap<BindingIndex,GpuOwned<DescriptorSetBinding>>、+0xa0 shader、+0xa8 光栅状态、+0xac blend 1.0、+0xb0 variant=0 | O | B_a4_draw §1/§2；wp1/contracts/draw_stream.md §1（0x140867850） |
| `country_core::render::pass_builder::draw_stream::TextureBindingKey::with_view` | 0x140867940 | 147 | u64 键 = 0x4000_0000_0000_0000|id；有视图时 ^ calculate_hash(view)（0x140867940） | O | B_a4_draw §1；wp1/contracts/draw_stream.md §4 |
| `country_core::render::pass_builder::draw_stream::impl$8::to_texture_binding_key` | 0x1408eefd0 | 201 | &Texture → 恒 unreachable!("Raw textures must not be used as rg resources")（draw_stream.rs:204:9） | O | B_a4_draw §1；wp1/contracts/draw_stream.md §4（0x1408eefd0） |
| `country_core::render::pass_builder::draw_stream::RasterPass::new_impl` | 0x140867970 | 352 | 颜色附件 ≤5（ArrayVec 溢出 panic）→ 每附件 add_resource_access(id, 0x36/0x2e, 1)（tag 决定）；深度 0x2f/0x17 组 1；PassResources::new_from + 2 描述符克隆 + 两张空表 | O | B_a4_draw §2；wp1/contracts/draw_stream.md §2（0x140867970） |
| `country_core::render::pass_builder::draw_stream::RasterPass::render` | 0x140868370 | 450 | 置已渲染 → build_render_pass_desc（上限 8 附件）→ 深度 clear_input → HashMap<RenderPassDesc,RenderPass> 去重（命中克隆/未命中 create_render_pass+插入）→ PassBuilder::render 提交 | O | B_a4_draw §2；wp1/contracts/draw_stream.md §2（0x140868370） |

## 内联/无独立符号函数

- `country_core::render::pass_builder::draw_stream::DrawPacket::variant`（行 122）— variant 读取（行 122）—— I
- `country_core::render::pass_builder::draw_stream::DrawPacket::stencil_ref`（行 127）— stencil_ref 读取（行 127）—— I
- `country_core::render::pass_builder::draw_stream::impl$2::to_texture_binding_key`（行 161）— Handle<Texture> → 0x4000_0000_0000_0000|id（行 161）—— I（取自 bind 函数体）
- `country_core::render::pass_builder::draw_stream::impl$3::to_texture_binding_key`（行 167）— ReadOnlyHandle → 同上（行 167）—— I
- `country_core::render::pass_builder::draw_stream::impl$4::to_texture_binding_key`（行 174）— TexturePreset → 同上（行 174）—— I
- `country_core::render::pass_builder::draw_stream::RasterPass::get_render_pass`（行 268）— 取/建 HashMap<RenderPassDesc,RenderPass> 缓存中的本 pass RenderPass（inline 1）—— I
- `country_core::render::pass_builder::draw_stream::RasterPass::new`（行 337）— new → new_impl（inline 42）—— O（见 new_impl）

## 派生胶水（14 项：closure、default、drop、eq 等）

14 项 glue：impl$9::drop（0x140896d80 = **DrawPassRenderCheck::drop**，O：!rendered && !panicking → panic "RasterPass dropped without a call to render()"）+ new_impl/render/get_render_pass 的 9 个 closure + impl$14/17/22 的 default/eq 派生。

## 证据注记（tg_rn1）
- **DrawPacket 0xb8B**（O）：head[2]@0、Vec<(u32 binding,u8 slot)>@0x10、Vec<u8>@0x28、种类 u32+载荷@0x40、HashMap<BindingIndex,GpuOwned<DescriptorSetBinding>>@0x70、shader@0xa0、光栅状态@0xa8、blend 1.0@0xac、variant@0xb0。
- **RasterPass ~0x3e8B**：new_impl 附件计数@+0x7c、深度标志@+0x1c、7 附件视图描述@+0xe4..、纹理句柄@+0x124、mip/层@+0x12c、已渲染@+0x3e4；render 置已渲染 → 附件描述（上限 8）→ create_render_pass 缓存 → PassBuilder::render 提交。
- **TextureBindingKey**（O）：u64 = 0x4000_0000_0000_0000|id，有 view `^ calculate_hash(view)`；`&Texture` 转换恒 panic。
- **DrawPassRenderCheck::drop**（0x140896d80）：`!rendered && !thread::panicking()` → panic。
- U：`DrawStream` 无 PDB 过程（空壳）；`get_render_pass` 形态 I；DrawPacket +0x28 Vec<u8> 语义 U。
