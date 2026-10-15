# vk_sync_fork::get_access_info 64 字符全表（agent=rhapsody-crates-closure，2026-10-15）

来源：rne-read 0x142b1a6c8（256×u32 数组）+ capstone 手解 get_access_info（0x140c10dd0）。
get_access_info(c) 写 3 个 u32：out[0]=stage（表 @0x142b1a6c8）、out[1]=access（表 @0x142b1a7c8）、out[2]=layout（表 @0x142b1a8c8）。
is_write=(c-0x28)<0x10（0x140c10e10）→ 写变体 0x28..0x37。变体名 = 公开 h3r2tic/vk-sync-rs 枚举序 + country-core 常量锚点 + layout 列（O/I）。

| char | stage 0x... | access 0x... | layout | 变体（I=锚点推断名） |
|---|---|---|---|---|
| 0x00 | 00000000 | 00000000 | 0 | Nothing |
| 0x01 | 00020000 | 00020000 | 0 | CommandBufferReadNVX |
| 0x02 | 00010000 | 00000021 | 0 | IndirectBuffer（country-core ACCESS_INDIRECT_READ=0x02） |
| 0x03 | 00000004 | 00000002 | 0 | IndexBuffer |
| 0x04 | 00000004 | 00000004 | 0 | VertexBuffer |
| 0x05 | 00000008 | 00000020 | 0 | VertexShaderReadUniformBuffer |
| 0x06 | 00000008 | 00000020 | 5 | VertexShaderReadSampledImageOrUniformTexelBuffer |
| 0x07 | 00000008 | 00000020 | 1 | VertexShaderReadOther |
| 0x08 | 00000010 | 00000020 | 0 | TessControlShaderReadUniformBuffer |
| 0x09 | 00000010 | 00000020 | 5 | TessControlShaderReadSampledImageOrUniformTexelBuffer |
| 0x0a | 00000010 | 00000020 | 1 | TessControlShaderReadOther |
| 0x0b | 00000020 | 00000020 | 0 | TessEvalShaderReadUniformBuffer |
| 0x0c | 00000020 | 00000020 | 5 | TessEvalShaderReadSampledImageOrUniformTexelBuffer |
| 0x0d | 00000020 | 00000020 | 1 | TessEvalShaderReadOther |
| 0x0e | 00000040 | 00000020 | 0 | GeometryShaderReadUniformBuffer |
| 0x0f | 00000040 | 00000020 | 5 | GeometryShaderReadSampledImageOrUniformTexelBuffer |
| 0x10 | 00000040 | 00000020 | 1 | GeometryShaderReadOther |
| 0x11 | 00000080 | 00000020 | 0 | FragmentShaderReadUniformBuffer |
| 0x12 | 00000080 | 00000020 | 5 | FragmentShaderReadSampledImageOrUniformTexelBuffer |
| 0x13 | 00000080 | 00000020 | 5 | FragmentShaderReadColorInputAttachment |
| 0x14 | 00000080 | 00000020 | 4 | FragmentShaderReadDepthStencilInputAttachment |
| 0x15 | 00000080 | 00000020 | 1 | FragmentShaderReadOther |
| 0x16 | 00000400 | 00000080 | 2 | ColorAttachmentRead |
| 0x17 | 00000300 | 00000200 | 4 | DepthStencilAttachmentRead |
| 0x18 | 00000800 | 00000020 | 0 | ComputeShaderReadUniformBuffer |
| 0x19 | 00000800 | 00000020 | 5 | ComputeShaderReadSampledImageOrUniformTexelBuffer |
| 0x1a | 00000800 | 00000020 | 1 | ComputeShaderReadOther（country-core ACCESS_READ=0x1a） |
| 0x1b | 00000008 | 00000020 | 0 | AnyShaderReadUniformBuffer |
| 0x1c | 0000000c | 00000020 | 0 | AnyShaderReadUniformBufferOrVertexBuffer |
| 0x1d | 00000020 | 00000020 | 5 | AnyShaderReadSampledImageOrUniformTexelBuffer |
| 0x1e | 00000220 | 00000020 | 4 | AnyShaderReadOther（layout 4=深度相关读，fork 新增，名 I） |
| 0x1f | 00000020 | 00000020 | 1 | fork 新增读变体（country-core ACCESS_DRAW_READ=0x1f，名 I） |
| 0x20 | 00000008 | 00010000 | 0 | fork 新增读变体（access=MEMORY_WRITE 语义组，名 I） |
| 0x21 | 0000000c | 00010000 | 0 | fork 新增读变体（名 I） |
| 0x22 | 00000020 | 00010000 | 5 | fork 新增读变体（名 I） |
| 0x23 | 00000220 | 00010000 | 4 | fork 新增读变体（名 I） |
| 0x24 | 00000020 | 00010000 | 1 | fork 新增读变体（名 I） |
| 0x25 | 00000800 | 00001000 | 6 | TransferRead（layout6=TRANSFER_SRC） |
| 0x26 | 00002000 | 00004000 | 1 | HostRead |
| 0x27 | 00000000 | 00000000 | 3b9acdea | Present（layout 哨兵，access/stage 0） |
| 0x28 | 00020000 | 00040000 | 0 | CommandBufferWriteNVX |
| 0x29 | 00000008 | 00000040 | 1 | VertexShaderWrite |
| 0x2a | 00000010 | 00000040 | 1 | TessControlShaderWrite |
| 0x2b | 00000020 | 00000040 | 1 | TessEvalShaderWrite |
| 0x2c | 00000040 | 00000040 | 1 | GeometryShaderWrite |
| 0x2d | 00000080 | 00000040 | 1 | FragmentShaderWrite |
| 0x2e | 00000400 | 00000100 | 2 | ColorAttachmentWrite |
| 0x2f | 00000600 | 00000300 | 3 | DepthStencilAttachmentWrite |
| 0x30 | 00000600 | 00000300 | 3b9c9309 | DepthAttachmentWriteStencilReadOnly（哨兵） |
| 0x31 | 00000600 | 00000300 | 3b9c9308 | StencilAttachmentWriteDepthReadOnly（哨兵） |
| 0x32 | 00000800 | 00000040 | 1 | ComputeShaderWrite（country-core ACCESS_CPU_MAPPED_WRITE=0x32） |
| 0x33 | 00010000 | 00000040 | 1 | AnyShaderWrite |
| 0x34 | 00001000 | 00001000 | 7 | TransferWrite（country-core ACCESS_TRANSFER_WRITE=0x34） |
| 0x35 | 00004000 | 00004000 | 1 | HostWrite |
| 0x36 | 00000400 | 00000180 | 2 | ColorAttachmentReadWrite |
| 0x37 | 00010000 | 00018000 | 1 | General（country-core ACCESS_WRITE=0x37） |
| 0x38..0x3f | 高掩码（0x200000/0x2000000 组） | 0x20/0x10/0x200000/0x400000/0x1000 | 5/4/0/1 | 高位掩码读变体（名 U；0x3d/0x3e access=0x2000000=SHADER_STORAGE_READ 类） |

stage 位值 = VkPipelineStageFlagBits（stage-1：0x8=VS、0x10=TC、0x20=TE、0x40=GEOM、0x80=FS、0x200=LATE、0x400=COLOR_OUT、0x800=COMPUTE、0x1000=TRANSFER、0x2000=BOTTOM_OF_PIPE、0x4000=HOST、0x10000=ALL_COMMANDS）。
access 位值 = VkAccessFlagBits（stage-1：0x2=INDEX_READ、0x4=VERTEX_ATTRIBUTE_READ、0x20=SHADER_READ、0x40=SHADER_WRITE、0x80=COLOR_READ、0x100=COLOR_WRITE、0x200=DEPTH_READ、0x400=DEPTH_WRITE、0x800=TRANSFER_READ、0x1000=TRANSFER_WRITE、0x4000=HOST_WRITE、0x8000=MEMORY_READ、0x10000=MEMORY_WRITE）。
