# render/pass_builder/framebuffer_binding.rs

> 文件作用：ColorAttachment（0x20B 枚举：tag0 引用@+8 / tag1 视图字@+4 引用@+0x18；访问 0x36/0x2e）/ DepthAttachment（tag0 可写 0x2f / tag1 只读 0x17 / 其它无深度；+0x10 清除 u32、+0x14 f32）。

函数总计 9：业务 1 / 内联无独立符号 8 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc` | 0x1409092f0 | 89 | 21B：ptr = *(self + 8 + (tag==1)*0x10); return ptr+4（两变体都指向 {id:u32@0, TextureDesc@4}，引用偏移不同） | O | B_a4_draw §1（0x1409092f0） |

## 内联/无独立符号函数

- `country_core::render::pass_builder::framebuffer_binding::DepthAttachment::layer`（行 36）— 层字段读取（inline 1）—— I（+4 = 层，O）
- `country_core::render::pass_builder::framebuffer_binding::ColorAttachment::handle_mut`（行 81）— 纹理句柄可变访问（inline 4）—— I
- `country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth`（行 132）— 旧 API：机器码无对应物（B_a4_draw 已删 5 槽/color_count 字段）—— 已删除
- `country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color`（行 145）— 旧 API：同上 —— 已删除
- `country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_depth`（行 157）— 旧 API：同上 —— 已删除
- `country_core::render::pass_builder::framebuffer_binding::impl$4::from`（行 171）— 附件转换（行 171）—— I
- `country_core::render::pass_builder::framebuffer_binding::impl$5::from`（行 179）— 附件转换（行 179）—— I
- `country_core::render::pass_builder::framebuffer_binding::impl$6::from`（行 185）— 附件转换（行 185）—— I

## 证据注记（tg_rn1）
- **ColorAttachment 0x20B**（O）：tag0 引用@+8 / tag1 视图字@+4（16B TextureViewDesc）+ 引用@+0x18；`desc()` 两变体都返回 `ptr+4`；访问类型 tag0→0x36、tag1→0x2e（组 1）。
- **DepthAttachment**（O）：tag0 可写 0x2f / tag1 只读 0x17 / 其它 = 无深度；+4 层、+8 纹理、+0x10 u32 非 0 = 清除、+0x14 f32 清除值。
- **RasterPass::new_impl** 迭代步长 = count<<5（0x20B/附件），上限 5（ArrayVec）。
- 纠错：旧 `RasterFramebufferBinding` 的 5 槽/`color_count` 字段在机器码里没有对应物（B_a4_draw 已删）；`AttachmentTexture` 是 `rhapsody::raw::texture::Texture` 被读字段（id@0、format@0xc、sample_count@0x21、extent）的最小替身。
- U：访问类型数值对应的 `vk_sync_fork::AccessType` 变体名；`RenderPassAttachmentDesc` 0x10B 内部布局（valid=format!=0，I）。
