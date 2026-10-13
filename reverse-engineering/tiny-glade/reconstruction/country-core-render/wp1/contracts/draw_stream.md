# draw_stream.rs 业务契约（RasterPass / DrawPacket / DrawStream）

标注：**O**=反编译直接支持；**I**=调用链/行号组合；**U**=未恢复。

## 1. `DrawPacket`（O，行 106；0x140867850）

`DrawPacket::new(shader: …)` 返回约 0xb0+B 结构（尺寸 U）。字段：
- +0x00/+0x08：shader 句柄（`rhapsody::raw::shader::impl$7::default()` 为默认 shader）与次级句柄（in_R8/in_R9）。
- +0xa0（in_RCX[0x14]）：in_RDX（推测 pass 标记/纹理列表）。
- +0xa8..+0xac：`RandomState`（两 key）+ `0x3f800000`（blend 1.0）。
- +0x70（in_RCX[0xe]）：回调/判别指针 `&anon_65ceff…`；+0x90/+0x98（in_RCX[0x12]/[0x13]）：RandomState key 对。
- +0x40（in_RCX[8]）：0x100000003（打包判别+计数）。
- +0x30/+0x38（in_RCX[6]/[7]）：1 / 0。

方法：`variant`（inline 1）、`stencil_ref`（inline 3）。

## 2. `RasterPass`（O，尺寸约 0x3e8B，`new_impl` 写至 +0x3e4）

- `new`（inline 42）→ `new_impl`（0x140867970，行 352）。
- `new_impl`（O）：`arrayvec::from_iter<RasterPassColorAttachment, 5, Map<IterMut<ColorAttachment>, new_impl::closure_env_0>>` 把至多 5 个 `ColorAttachment`（0xcB）转成 raster 附件（max 5，arrayvec 上限）；每附件读纹理句柄 → `PassBuilder::add_resource_access<PassResources>` + `TextureViewDesc::{level_count,layer_count,base_layer}`；`PassResources::new_from()`；clone 2 个 `DescriptorSetBinding`；建两个 `RandomState` hash map；写附件计数@+0x7c、深度标志@+0x1c、7 个附件视图描述@+0xe4..+0x114、纹理句柄@+0x124、mip/层标志@+0x12c。
- `get_render_pass`（I，inline 1）：取/建 `HashMap<RenderPassDesc, RenderPass>` 中本 pass 的 RenderPass。
- `render`（0x140868370，行 450，O）：
  1. `*(self+0x3e4) = 1`（DrawPassRenderCheck 置已渲染）。
  2. 由 +0x40 处附件数组（`+0x7c` 计数，每项 0xcB）构造 `RenderPassAttachmentDesc` 列表（arrayvec，上限 0x80）。
  3. 若 `+0x1c != 2`：追加深度附件；若 `+0x11c != 0`：调 `RenderPassAttachmentDesc::clear_input`。
  4. `HashMap::rustc_entry<RenderPassDesc, RenderPass>` 去重：命中直接 clone 已有 RenderPass；未命中 `GraphicsDevice::create_render_pass()` + hashbrown 插入。
  5. `PassBuilder::render<PassResources, render::closure_env_0>` 注册图 pass。
  6. 结束时若 `+0x3e4 == 0`（未渲染）且未 panic → `panic_fmt`（DrawPassRenderCheck drop 断言）。

## 3. `DrawPassRenderCheck` / `DrawKind`（O/I，行 260）

- `DrawPassRenderCheck`：drop（0x140896d80，98B）：断言 pass 已 render（否则 panic）。
- `DrawKind`（enum2，module 253/485 drop_in_place）：DrawBuilder mode@+0x40 判别值的枚举载体（值 1/3/4/5/7/8 见 draw_builder §4）。

## 4. `TextureBindingKey` / `ToTextureDrawStreamBindingKey`（O，行 147/201）

- `with_view`（0x140867940，行 147）：若 view 非 0，`utils::calculate_hash<TextureViewDesc>` 计算视图描述 hash 并入键。
- `ToTextureDrawStreamBindingKey` trait：`&Texture` 的 `to_texture_binding_key`（0x1408eefd0，行 201）。`impl$2/3/4::to_texture_binding_key`（inline）。
- 语义：纹理→纹理绑定键（texture handle + view desc hash），供 `bind_texture_to_*` 与纹理槽表使用。

## 5. `DrawStream`（I/U）

高层封装（无独立过程，仅从 trait/类型关系确认）：聚合 DrawPacket + RasterPass，向图录制绘制流。
