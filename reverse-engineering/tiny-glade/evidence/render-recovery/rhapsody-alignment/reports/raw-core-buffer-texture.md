# raw-core 域·缓冲与纹理 复现报告（2026-10-15）

状态符号：✅ 原版对齐（括号内为证据地址）；⚠️ 候选/假设；❌ 读不出（写明读了什么）。
证据来源：`producer-closure/n1-device/*.c`、`n2-texture-buffer/*.c`、`/tmp/dis.py` 直接反汇编 `tiny-glade.exe`（patched）、`country-core-deep/.../raw-core/`。
验证：`cargo test --offline -p rhapsody` 全绿（lavapipe）；`cargo test -p tiny-glade --test render_host` 9/9。

## 0. 本轮对旧文档的更正（机器码为准）

| 旧说法 | 实际（证据） |
|---|---|
| `inner+0x30` 是 device_address 缓存 | 是 **VkBuffer**：`device_address`（0x140bd4c00）把 `inner+0x30` 放进 `VkBufferDeviceAddressInfo`（sType 0x3b9e8321=1000244001）调 `device+0x508`；`set_buffer_debug_name`（0x140beec30）同一个值作命名句柄（objectType 9）。**没有缓存**，每次都调设备函数 |
| `inner+0x10` 是 VkBuffer | `+0x10/+0x18` 是创建时传入的名字 `&str`（0x140bee69c 前 `[rbp+0xd0]=name ptr,[+0xd8]=len`，`+0xe0` 起是 BufferDesc 16B，`+0xf0` 才是 VkBuffer）→ 槽内：`+0x10/0x18` 名字、`+0x20` size、`+0x28` desc 尾、`+0x30` VkBuffer、`+0x38` 内存块 |
| `inner+0x28` 位 1 门控“直写路径” | `update_buffer_bytes` **根本没有直写路径**。`inner+0x28 & 2` = `usage & TRANSFER_DST`；缺它则 panic，文案 `Buffer {} must be created with vk::BufferUsageFlags::TRANSFER_DST in order to support updates`（0x142b159e0）。原版一律“暂存缓冲 + setup 命令缓冲 vkCmdCopyBuffer” |
| `create_mapped_buffer` 额外 24B | 额外 **16B**：返回 0x40B = tag 8 + Buffer 40 + `ptr`@0x30 + `len`@0x38（0x140beea80 `param_1[6]=impl+0x38, [7]=impl+0x40`）；`ptr==0` 则 `expect("mapped memory")`（0x142b158d0） |
| `Texture` 40B | **0x38B**：`{ctx@0, slot@8, offset u32@0x10, device_id u16@0x14, flag u8@0x16 = 0, pad, TextureDesc 32B@0x18}`（0x140c07950 的 `param_1[1..7]` 写入）；40B 是 Buffer。Result 80B，Ok 判别 `0xb` |
| `"GpuOnly texture"` 是默认名 | 只出现在**分配失败（Result Err 变体，niche 字节==2）**分支，作为错误载荷里的 String（cap=len=0xf，0x140c07950）；buffer 侧同位置放的是调用者传入的 name。不是默认调试名 |
| `all_mip_levels` = floor(log2(max(w,h)))+1 | 0x140bd54a0 三个 `bsr`，取 `max(bitlen(w), bitlen(h), bitlen(depth))` 截 u8；旧 Rust 实现恰好等价 |
| `half_res` depth `>>1` | 0x140bd5730：三个维度都是 `x - (x>>1)`，0→1（depth 也是 ceil，不是 `>>1`） |
| `init_texture_data` 的 `*desc&4` | 是 `usage.contains(SAMPLED)` 断言（0x142b16ff8 字串 `assertion failed: desc.usage.contains(vk::ImageUsageFlags::SAMPLED)`） |

## 1. 工作项状态

### 1. Buffer 原版形状
- ✅ `Buffer::device_address`（0x140bd4c00）：三查顺序 设备代号→flag==1→槽偏移；无缓存。落码 `BufferRecord::device_address(device_id, slot, query)`、`GraphicsDevice::buffer_device_address`。
- ✅ `Buffer::mapped_slice_mut`（0x140bd4cd0）：三查 → `inner+0x7c==2` panic → 块映射指针空则 `None` → `Buffer+0x18 > 块大小` panic（slice_end_index_len_fail）→ 全长切片 `(ptr, Buffer+0x18)`。落码 `BufferRecord::mapped_slice_parts`、`Buffer::mapped_full_slice_mut`、`GraphicsDevice::buffer_mapped_slice_mut`。旧 `(offset,size)` 形态的 `mapped_slice_mut(&mut self,..)` 保留未动。
  - ⚠️ 为让专用分配也“创建即映射”（原版 0x140bee5f0 创建时就取 ptr+len），`Buffer::new` 对 HOST_VISIBLE 且无子分配指针的缓冲一次性 `vkMapMemory`，`transfer()` 改为优先用该指针（避免二次 map）。行为超集，现有测试通过。
- ✅ `create_mapped_buffer`（0x140beea80）：断言 `memory_location ∈ {2,4}`、创建后 ptr 非空；落码 `GraphicsDevice::create_mapped_buffer_checked`（**名字带 `_checked` 是因为 `device.rs` 已有同名 `create_mapped_buffer(size,usage)`，我无权改它**，见遗留）。
- ✅ `inner+0x28` 位 1：见 §0，= `usage & TRANSFER_DST`，`BufferSlotView::has_transfer_dst`。
- ✅ `update_buffer_bytes`（0x140beefa0 + 闭包 0x140c03a00）：判定顺序 句柄三查 → 逐区间（`start<=end`、`end<=data.len`、累计后 `total+dst_offset <= capacity`，断言文案 0x142b15b00）→ `TRANSFER_DST` → `total>0`（`assert!(size>0)`）；暂存 `{size=total, TRANSFER_SRC, alignment 0, memory_location 3}` 名 `"Buffer update data"`；区域 `{src=累计, dst=start+dst_offset, size}`（0x140c03a00 写 0x18B 项）；闭包前后各一次全量内存屏障（stage 0x10000，access 0x1800000018000）。落码 `plan_buffer_update`（纯函数，可测）+ `GraphicsDevice::update_buffer_bytes`（重写，去掉直写）。
  - ⚠️ 原版只比较 `total + dst_offset`，不查每区域 `dst_end`；我额外拒绝 `dst_offset + range.end > capacity`（有意超集）。断言失败原版 panic，这里 `Err`（`Error::Invalid(静态文案)`）。
  - ⚠️ 屏障：本实现的 `with_setup_cb` 只在尾部补一次全量屏障且同步等 fence；原版闭包内前后两次屏障未复刻（同步版本下无差别）。
- ✅ `set_buffer_debug_name`（0x140beec30）：debug-utils 为空直接返回（先于校验）→ 校验 → 取分配器锁 → `inner+0x7c==2` panic → `rename_allocation(gpu-allocator)` → `vkSetDebugUtilsObjectNameEXT(type 9, handle=inner+0x30)`；名字含 NUL 则 `unwrap()` panic。现有 `set_buffer_debug_name` 语义等价（gpu-allocator 重命名在依赖里，不复刻）。
- ⚠️ `inner+0x7c == 2` 的语义：机器码证实“==2 时 panic/不命名”，`create_buffer_impl` 里同一偏移的 `==2` 是分配结果 Result 的 Err niche → 推断为“Option<Allocation> 被取走”，**未直接证实**。Rust 侧用 `SLOT_ALLOCATION_TAKEN=2`、活槽为 0。

### 2. create_buffer / create_buffer_impl（0x140c03460 / 0x140bee200）
- ✅ Result 展开：`out+0x00=0xb` 是 Ok 判别，Buffer = `impl.out+0x10..0x37`（40B）；impl 的 0x48B = `{tag@0, slot+0x10@8(额外), Buffer@0x10..0x37, mapped ptr@0x38, size@0x40}`。`0x140bee5f0` 的 `call 0x140c113d0` 返回 `rax=ptr, rdx=len`，随后 `if ptr!=0 && size>len → slice_end_index_len_fail`，**确认 0x38/0x40 是 (ptr, len=desc.size) 切片**。
- ✅ 对齐：`max(memreq.alignment, desc.alignment(u16), usage&0x400 ? 0x40 : 0x10)`（`buffer_alignment`，已用于 `Buffer::new` 的分配请求）。`0x400` 在公开 Vulkan 里无名，保持原始数值。
- ✅ usage 自动加 TRANSFER_DST：仅当初始数据非空（0x140c03460 开头 `if len!=0 && ptr!=0 { desc.usage |= 2 }`）。
- ✅ 初值上传链：`assert!(initial_data.byte_count() <= desc.size)`（0x142b186a0）→ 目标已映射则直接 memcpy（不经暂存）→ 否则暂存 `{size=desc.size（不是数据长度！）, usage=TRANSFER_SRC, align 0, memory_location=3}` 名 `"buffer initial data"`（0x142b18730，19B）→ `expect("mapped buffer memory")` → memcpy → `with_setup_cb`（0x140c04840）→ 暂存丢弃。落码 `GraphicsDevice::create_buffer`（含 `upload_via_staging`）。
  - ⚠️ 0x140c04840 的拷贝闭包体没反汇编，区域大小按“数据长度”写（与 0x140c03a00 同族，**未证实**）。
- ✅ 默认名：buffer 没有默认名，name 由调用方传入并写入槽 `+0x10/0x18`；分配失败时 Err 载荷带这个 name。
- ❌ 40B 载荷里 `+0x00`（ctx = `*(context)` 退役队列头）、`+0x08`（slot）在本实现是 `BufferRecord` 调用方槽表提供的标识；Arc 模型里无真实指针，未对 Mutex 头做任何模拟。

### 3. Texture
- ✅ `create_texture`（0x140c07950）：`assert!(usage != 0)`（已有）→ 有数据 `usage|=2` → `get_image_create_info(desc, has_data)`（**第三参 → `initialLayout = has_data<<3` = PREINITIALIZED(8)**，0x140bd5f89 `shl r8d,3`/`mov [rcx+0x50]`；旧 Rust 恒 UNDEFINED，已修：`get_image_create_info_with_data`、`Texture::new_with_initial_data`）→ extent 为零三条断言（已有）→ vkCreateImage(+0x1a0)→命名→内存需求(+0xf0)→分配→bind(+0xe0)→`ManagedTexture::new`（0x140bdc2d0）→ tag≠0 则 `init_texture_data`。
- ✅ `get_image_create_info`（0x140bd5ed0）逐字段：sType 14、`flags=desc+4` 原样、imageType（1D 两种 0/ 3D 2/ 其余 1）、extent 随 kind（1D 类 `(w,1,1)`；2D/2DArray/Cube/CubeArray `(w,h,1)`；3D `(w,h,d)`）、layers（Cube 6，CubeArray n*6，kind 0/2/5 且 array≠1 panic）、samples=desc+0x1d 原始数、tiling 0、sharing 0。已落码 `TextureKind::image_extent`。
  - ⚠️ 有意偏差：cube 种类仍自动补 `CUBE_COMPATIBLE`（原版 flags 原样透传，原版调用方如何给出该位未证实）。
- ✅ `Texture` 0x38B（U#14）：`TextureRecord` + `Texture::record`，偏移断言。`flag@0x16 = 0`（Buffer 为 1）——数值已证，“资源种类标记”的解读 ⚠️ 未证实。
- ✅ `init_texture_data`（0x140bfe270）：门控、跳表 0x142b174fc 四目标（已用 EXE 解出 0x140bfe5af/2c0/30b/3aa 并写成测试）；`*param_4` 为 `TextureInitialData` 判别：
  - 0（0x140bfe5af）：`unreachable` 文案（0x142b15fb0）= “无数据”，`create_texture` 里 tag==0 根本不调用；
  - **1（0x140bfe2c0 → 闭包 0x140c08560）= 整图清零**：`aspect=0x141738fe0(format)`（= `format::observed::vk_format_valid_aspect_flags`，表 0x142ccd890，已用 EXE 表逐项比对），setup CB 内访问码 `0→0x34(TransferWrite)` 整图屏障 → `aspect&6==0` 用 `vkCmdClearColorImage(+0x3a8)` 否则 `vkCmdClearDepthStencilImage(+0x3b0)`，布局 7，清除值 0，范围 `{aspect, 0.., level_count=desc.mip_levels(byte), 0.., layer_count=desc.array_elements}` → `0x34→0x22` 屏障。落码 `setup::init_texture_zero_fill` / `GraphicsDevice::create_texture_zeroed`（⚠️ layer_count 取 `array_layers()` 而非 `array_elements`，避免 cube 漏面；屏障用公开访问掩码而非原访问码表）。
  - 2（0x140bfe30b）：`0x1417390e0(format)==1`（= `observed::vk_format_block_bytes`，非块压缩；否则落 0x140bfe52c panic 文案）、`slice 元素数 == desc.mip_levels`（0x140bfe33d 否则 panic）、`MipChainIter::new(extent, mips)`、每元素 16B `(ptr,len)`（`shl r12,4`）→ 推断为“逐 mip 一块数据”。⚠️ 后续区域生成（0x1418b77a0）未读。
  - 3（0x140bfe3aa）：`0x141739060(format)` 取 `{word00, word04(块字节,<0x100), byte08}`（= `observed::vk_format_info`）、`array_elements<0x10000`、`data.len!=0`（`assert!(!data.is_empty())`，0x142b0fae0）、`MipChainIter::new(...)`、生成 0x30B 区域表（0x1418b8170），再调 0x140bfeac0。推断为“线性紧排数据”。⚠️ 区域排布顺序（mip 主序 / layer 主序）未读。
  - ❌ 2/3 号的逐格式写入（区域表与 update_texture_impl 0x140bfeac0 的访问码 0x22/stage 0x22）未落码：读了 3 个分支的入口与助手，但 0x1418b77a0/0x1418b8170/0x140bfeac0 未展开。现有 `init_texture_data` 仍只支持单 mip 单层、长度精确匹配；新增 `usage.contains(SAMPLED)` 与 `!data.is_empty()` 断言（返回 `Err`）。
  - 助手 0x141738fe0/0x1417390e0/0x141739060 原来已在 `format::observed`（别人建的），本轮用 EXE 表逐项交叉验证了 `format_table.rs` 的 `OBSERVED_*` 常量。
- ⚠️ `update_texture`：已有 `setup::update_texture`（单 mip，layout SHADER_READ_ONLY）；原版是 `update_texture_impl`（0x140bfeac0），未对照，不动。
- ✅ `set_texture_debug_name`（0x140bfe650）：门控/类型 10=IMAGE，现有等价。

### 4. `image_aspect_mask_from_format`（0x140bd6630）
✅ 原来 rhapsody 里**没有**这个函数（只有 render_pass.rs 的注释引用）。新增 `texture::image_aspect_mask_from_format`：`format-0x7c`（无符号）`>6 → 1` 否则表 0x142b0ffb8 = `{2,2,2,4,6,6,6}`（已从 EXE 读出并测试，含机器码字节核对）。`format_table.rs` 里 observed 表与 EXE 全量一致；公开 vk.xml 注册表的 `block_bytes/block_extent` 对所有 COLOR 格式 ≤146 与原表一致（深度模板格式原表按分量写，未比较）。

### 5. U 卡点落点
| U | 结论 |
|---|---|
| #1 0xb tag | ✅ Result Ok 判别（0x140c035c3 `mov [rsi],0xb`；texture 同） |
| #2 80B 映射 | ✅ Buffer=out+8..0x2f；Texture=out+8..0x3f（0x38B）；Err 写到 `[9]`=80B |
| #3 `Buffer+0x20` | ✅ BufferDesc 尾 `{usage,align,mem}`；create_mapped_buffer 的额外项是 `(ptr,len)` 不在 Buffer 内 |
| #4 `inner+0x28` vs `+0x7c` | ✅ 两个字段：`+0x28`=usage 低字节含 TRANSFER_DST；`+0x7c`=分配状态字节（==2 取走）。⚠️ `+0x7c` 的 Option niche 解释未直证 |
| #9 init 跳表 | ✅ 4 目标；1 号完整落码；2/3 号入口读完、区域生成 ❌ 未读（见上） |
| #14 Texture 布局 | ✅ 0x38B |
| #15 微 getter | ✅ 逐字节复核：TextureDesc 全部（half_res / div_up / div / all_mip / array_elements 掩码 0x4a / sampled_with_different_format `or [rdx],0x108000004` / sample_count / mip_levels / usage 位 4,8,2,0x10,0x20）；TextureViewDesc 十个微函数（0x3cB，Option=`{tag,value}`）。旧 Rust 偏差已修：half_res 漏 depth；`sampled_with_different_format` 原为谓词（现为原版 builder，旧谓词改名 `is_sampled_color`）；`sample_count` getter 改为原版 builder（`samples()` 保留）；`div_up_extent/div_extent` 原为单除数返回 Extent3D，新增原版三除数、至少 1、除零 panic 的 `*_by` |

## 2. 改动文件
- `crates/rhapsody/src/raw/buffer.rs`：`buffer_alignment`、`BufferSlotView`、`BufferAccessError`、`BufferUpdateError`、`BufferRegion`/`BufferUpdatePlan`/`plan_buffer_update`、`BufferRecord::{validate,device_address,mapped_slice_parts}`、`Buffer::{slot_view,mapped_full_slice_mut}`、`MappedBufferParts`；`Buffer::new` 用原对齐并对专用主机可见分配创建即映射；`transfer` 复用该映射。
- `crates/rhapsody/src/raw/device_api.rs`：`create_buffer` 重写（原上传链）、`create_mapped_buffer_checked`、`buffer_mapped_slice_mut`、`buffer_device_address`、`update_buffer_bytes` 重写（无直写）、`create_texture_zeroed`、`upload_via_staging`（取代 `upload_buffer_bytes`）；`create_texture` 传 `has_data`。
- `crates/rhapsody/src/raw/texture.rs`：`image_aspect_mask_from_format`、`get_image_create_info_with_data`、`TextureKind::image_extent`、`TextureRecord`、`OptionU32`/`TextureViewDescRaw`（0x3c）、`Texture::{new_with_initial_data,record}`、TextureDesc 的 `half_res`/`sampled_with_different_format`/`sample_count`/`div_*_by` 修正。
- `crates/rhapsody/src/raw/setup.rs`：`init_texture_zero_fill`、`init_texture_data` 加 SAMPLED / 非空断言。
- 未改：`format_table.rs`、`texture_upload.rs`、`managed/buffer.rs`、`managed/texture.rs`（审过，无需改；`managed::Buffer::mapped_slice_mut(offset,size)` 保持）。
- 工作树换行：`texture.rs` 实测为 LF（简报写 CRLF，与实际不符；HEAD 同为 LF），未引入换行改动。

## 3. 新增测试（`crates/rhapsody/tests/raw_core_buffer_texture.rs`，22 项）
纯逻辑：对齐三因子；句柄三查顺序；device_address 不缓存/校验失败不调设备；mapped_slice_parts 四种分支；update 计划（区域、各断言顺序、稀疏越界超集、空更新）；Buffer 40B / Texture 0x38 / TextureViewDescRaw 0x3c 偏移；TextureDesc builders（half_res 3D、div_*_by、all_mip、array 掩码）；`get_image_create_info`（PREINITIALIZED、extent 随 kind、cube 层数）。
EXE oracle（EXE 缺失则跳过）：aspect 表 0x142b0ffb8 + 函数头字节；`observed` 四张格式表对 EXE 全量逐项；公开注册表对原块表的一致性；init 跳表四目标地址与 `test byte [r8],4`、`shl r8d,3`。
lavapipe：create_buffer 初值两路径（可映射直写 / GpuOnly 暂存）与越界断言；映射切片全长；create_mapped_buffer 位置断言；update 需 TRANSFER_DST 且走暂存；BDA 每次转发且跨设备拒绝；create_texture_zeroed（颜色与深度）、SAMPLED 门控。
注：lavapipe 上 GpuOnly 也 HOST_VISIBLE，故只断言“有映射 ⇔ 有切片”，没有断言 GpuOnly 必为 None。

## 4. 遗留 / 需要别人配合
1. `device.rs`（非我拥有）已有 `GraphicsDevice::create_mapped_buffer(size, usage)`，与原版签名 `(desc, name)` 不同；我只能提供 `create_mapped_buffer_checked`。device.rs 所有者可改名后让我换回。
2. `init_texture_data` 2/3 号目标的区域生成（0x1418b77a0 / 0x1418b8170）与 `update_texture_impl`（0x140bfeac0）未读 → 多 mip / 多层 / 压缩纹理初值上传仍不支持，也无法证实 `LinearInitialDataIter` 的排布。
3. 0x140c04840（create_buffer 暂存拷贝闭包）区域大小未读；0x140c04280 是 `with_setup_cb` 本体（锁 `+0x9f0`，状态 4=无→新建命令缓冲 0x140bef4a0 → begin 0x140bed230 → 跑闭包），setup CB 并入图提交仍是同步版（setup.rs 既有偏差，非本轮范围）。
4. `+0x7c==2` 的 niche 解释、`Texture flag@0x16=0` 的含义均未直证。
5. `country-core` 的 `TextureDesc::new([..], kind, format)` 等占位类型仍独立于 rhapsody 的 `TextureDesc`，本轮未接线（不在我的文件内）。
