# framebuffer_binding.rs 业务契约

标注：**O**=反编译直接支持；**I**=调用链/行号组合；**U**=未恢复。

## 1. `ColorAttachment`（O/I，0xcB = 3×u32）

- `desc`（0x1409092f0，行 90；反编译为空——真实逻辑被内联，**U** 真实体）：生成 `RenderPassAttachmentDesc`（load/store/format）。
- `handle_mut`（inline 4）：纹理句柄可变访问。
- `handle` 字段 + 加载/存储操作字段（+0x04/+0x08）语义 U。

## 2. `DepthAttachment`（O/I）

- `layer`（inline 1）。

## 3. `RasterFramebufferBinding`（I，行 90 附近）

- `new_color`（inline 22）/ `new_depth`（inline 1）/ `new_color_depth`（inline 19）。
- `impl$5/impl$4/impl$6::from` 转换。

## 4. 与 draw_stream 的接缝（O）

`RasterPass::new_impl`（0x140867970）用 `arrayvec::from_iter<RasterPassColorAttachment, 5, Map<IterMut<ColorAttachment>, new_impl::closure_env_0>>` 把至多 5 个 `ColorAttachment`（0xcB）转成 `rhapsody::rg::pass_api::RasterPassColorAttachment`；每附件读纹理句柄 → `PassBuilder::add_resource_access` + `TextureViewDesc::{level_count,layer_count,base_layer}`。
