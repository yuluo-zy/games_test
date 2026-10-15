# 第二批：纹理初值上传剩余分支 + 缓冲暂存拷贝（wave2-texture-upload）

工具：`/tmp/dis.py`（oracle `.rne` 反汇编）+ 只读 PE 字符串读取；Ghidra 导出 `producer-closure/n1-device/0x140bfeac0.c`、`0x140c04840.c` 辅助。
图例：✅原版对齐[证据] / ⚠️候选 / ❌读不出。

## 1. `init_texture_data` 全分派与 `update_texture_impl`

### 1.1 总体结构 ✅
* 0x140bfe270：`test byte [desc],4`（SAMPLED）→ 跳表 0x142b174fc 按 `*param_4`（`TextureInitialData` 判别）。desc 布局（`r15`）：`+8 format`、`+0xc w,h`、`+0x14 depth`、`+0x18 array_elements`、`+0x1c mip_levels(u8)`。
* 0 号（0x140bfe5af）= `unreachable!()`（texture.rs 第 648 行，文案 0x142b15fb0 "internal error: entered unreachable code"），没有对应变体（`create_texture` 只在标签非 0 才调用）。
* 1 号（0x140bfe2c0）= 整图清零（上一轮已落码 `init_texture_zero_fill`）。
* 2 号（0x140bfe30b）= **逐 mip 一块数据，仅非压缩格式**：`0x1417390e0(format) == 1`（否则 `assert_eq!(.., 1, "TODO")`，texture.rs 第 666 行；0x142b17090 就是字串 "TODO"）；切片数 `== desc.mip_levels`（第 667 行 `assert_eq!`）；`MipChainIter::new(extent, mips, bytes_per_block=1, block=(1,1))`（0x140bd5a80，实参 `r9b=1`、栈字 0x0101）；每个切片是 16B `(ptr,len)`，与 mip 链 zip（0x1418b77a0）。区域 `{data=切片, extent=该 mip 尺寸, mip, layer=0}`；**长度不校验**。
* 3 号（0x140bfe3aa）= **线性紧排数据，支持块压缩**：`vk_format_info`（0x141739060）取 `{bytes_per_block=word04, block=(byte08,byte09)}`；断言顺序 `array_elements < 0x10000`（texture.rs 659:52）→ `bytes_per_block < 0x100`（660:48）→ `!data.is_empty()` → `MipChainIter::new` → 收集 0x1418b8170。
* 两个收集点（0x1418b77a0 / 0x1418b8170）都是 `Vec<[u8;0x30]>`：`{ptr@0, len@8, 0@0x10(u64), 0@0x18(u32), extent{w,h,d}@0x1c/0x20/0x24, mip@0x28, layer@0x2c}`（3 号目标 +0x2c 为层号，2 号写 0）。
* 汇合点：`update_texture_impl(0x140bfeac0)(out, device, handle, desc, format, regions_ptr, regions_len, old_access=0, new_access=0x22)`。

### 1.2 `MipChainIter`（0x140bd5a80 / 0x140bd5bf0）✅
* 12B 状态 `{w u16,h u16,d u16,block[2] u8,cur u8,count u8,bytes_per_block u8}`；`new` 断言按机器码顺序：`bytes_per_block>0`、`block[0]>0`、`block[1]>0`、`extent[0..2]>0`、三次 `u16::try_from(extent[i]).unwrap()`（texture.rs 365/366/367 行）。文案全在 0x142b0fb20..0x142b0fc40。
* `next`：`cur >= count` → None；产出 `size = ceil(w/bw)*ceil(h/bh)*d*bytes_per_block`、当前 extent、`cur`；之后三维各 `>>1`，减半后三维**全 0**（当前级已是 1x1x1）置 `cur=0xff`（下一次结束），否则各取 `max(.,1)` 并 `cur+=1`。所以 `mip_levels` 大于真实链长时提前结束。

### 1.3 层×mip 迭代器（0x140bd5d30）✅ 排布顺序
**层主序、mip 次序**（外层 layer，内层 mip；mip 迭代耗尽后 layer+1、从 12B 模板复位）。数据依次从切片头部切：剩余为 0 → panic "Texture initial data underflow"（texture.rs 511:21）；`size > 剩余` → `split_at` panic（515:37）；全部迭代完后剩余 ≠ 0 → `assert_eq!(rest, 0, "Texture initial data has incorrect size; more bytes supplied than necessary")`（532:17）。
**层数取 `desc.array_elements`**（`[r15+0x18]`），不是 Vulkan 层数：Cube 的 `array_elements` 被 0x140bd5f4d 强制为 1，所以 Cube 初值只会切出 1 层（原版如此，保持）。

### 1.4 `update_texture_impl`（0x140bfeac0 + 0x1418b4080 + 0x140c08830）✅
1. `total = Σ region.len`（0x140bfeb10 起 4 路展开求和）；`total == 0` → `assert!(size > 0)`（0x142b0f830）。
2. `block_bytes = 0x1417390e0(format)`、`aspect = 0x141738fe0(format)`。
3. 暂存缓冲 `{size=total, usage=TRANSFER_SRC(1), alignment 0, memory_location=3}`，名 `"texture initial data"`（0x142b17168，20B）；没有映射指针 → `expect("mapped texture staging buffer memory")`（0x142b1717c）。**注意 memory_location=3（CpuToGpuLowPrio）**，旧 Rust 用 CpuToGpu(2)，已改。
4. 逐区域闭包 0x1418b4080：`memcpy(staging+offset, region.data, len)`；`offset % block_bytes != 0` → `assert_eq!` 失败（`block_bytes` 非压缩 =1，BC 为 8/16）；写 0x38B 区域 `VkBufferImageCopy{bufferOffset=offset, rowLength=0, imageHeight=0, subresource{aspect(整表值), mip, baseArrayLayer=layer, layerCount=1}, imageOffset=0, imageExtent}`；`offset += len`（**紧排、无额外对齐**）。
5. 命令闭包 0x140c08830（经 setup CB，0x140c04560 同步状态机）：整图屏障（`level_count=-1, layer_count=-1`，访问码 `old(0) → 0x34`）→ `vkCmdCopyBufferToImage`（`device+0x388`，dstLayout 7 = TRANSFER_DST_OPTIMAL，区域数 = Vec 长度）→ 屏障 `0x34 → 0x22`。
6. 之后 `FUN_140be24b0` 释放暂存（延后销毁）。

**`block_bytes`（0x1417390e0）的真实值域**：格式 1..130 返回 1；131..146（BC）只放行 `{131,132,139→8B；137,138,141,142,145,146→16B}`，其余 BC（BC1_RGBA、BC2、BC4_SNORM、BC6H）落到 panic。所以原版 3 号目标对这些 BC 格式 panic（Rust 返回 `Err`）。
**偏差记录**：(a) 访问码表（0 / 0x34 / 0x22）以公开 Vulkan 访问掩码 + 布局 SHADER_READ_ONLY_OPTIMAL 近似（访问码 0x22 的精确语义未读，沿用上一轮 zero-fill 的同一假设）；(b) 同步版 setup CB（既有偏差）；(c) 2 号目标过短切片原版不校验，这里拒绝；(d) 深度+模板格式的 aspect 整表值（DEPTH|STENCIL）直接进 `imageSubresource`，与 Vulkan 规则冲突，原版如此，未改；(e) 初始 old_layout 用 UNDEFINED（图像是 PREINITIALIZED 创建，原版屏障的 old layout 在访问码表里，未读）。

### 落码
* `texture_upload.rs`：`MipChainIter`、`MipLevelItem`、`TextureUpdateRegion`、`linear_regions`（3 号）、`per_mip_regions`（2 号）、`plan_texture_copies`（0x38B 区域 + 对齐/总长断言）、`copy_format_params`。纯函数。
* `setup.rs`：`TextureInitialData{Zeroed,PerMip,Linear}`（变体名候选）、`upload_regions`（update_texture_impl）、`init_texture_data_with`（完整分派）、`init_texture_data`（= Linear，旧签名保留）、`update_texture`（整纹理线性，签名保留，现在支持多 mip 多层）、`update_texture_regions`（显式区域）。
* `device_api.rs`：`GraphicsDevice::create_texture_with_data(desc, name, TextureInitialData)`。

## 2. `create_buffer` 暂存拷贝闭包 0x140c04840 ✅
* 命令只有一条 `vkCmdCopyBuffer`（`device+0x370`，count=1）：`{srcOffset=0, dstOffset=0, size = data.ptr != 0 ? data.len : 0}`——**区域大小 = 数据长度，不是 `desc.size`**；暂存缓冲大小才是 `desc.size`（0x140c03460 里 `uStack_88 = *param_5`）。前面 memcpy `data.len` 进映射，`len > 映射长` → 切片越界 panic（0x142b18770）；暂存映射为空 → `expect("mapped buffer memory")`（0x142b18743，20B）。
* 闭包体本身无屏障（只有 with_setup_cb 末尾的全量屏障）；资源 +0xc 校验位后用原子 max 更新“最近使用”时间戳（两个资源各一次，0x140c04840 里的 CAS 循环）。
* 核对 `upload_via_staging`：区域 `(0,0,len)` 单区间、暂存 `new_cpu_to_gpu_low_prio(desc.size)`——**已一致**，补了注释。`plan_buffer_update`/0x140c03a00 的多区间路径此前已对齐，未变。
* 小修：`update_buffer_bytes` 的暂存名应是 `"Buffer update data"`（旧代码两路都叫 "buffer initial data"），`upload_via_staging` 加了 `name` 形参。

## 3. `inner+0x7c==2` 与 `flag@0x16=0`
* `inner+0x7c` ✅（部分）：`inner+0x38` 起是内存块（0x140c02280 读 `+0x10`），所以 `+0x7c` 恰是块内 `+0x44` 字节；创建纹理（0x140bdc2d0）对同一字节判 `!= 2`（决定要不要释放分配，错误路径写 3），故是 **`Option<分配>` 的 niche：2 = None**。`mapped_slice_mut` 命中时走 `managed/buffer.rs:40:34`（0x142d02388）的单参 `unwrap_failed`（0x140bd4dc9）。❌ “谁把它置成 2”：全 .text 扫描 `mov byte/dword [reg+0x7c],2`，只有 5 处栈局部写，没有写槽的立即数，取走分配的函数未定位。
* `flag@0x16` ✅（用途）：消费方都做种类断言——`mapped_slice_mut`（0x140bd4cef）`cmp [h+0x16],1`，`update_texture` 校验包装 0x140bfe9c0/0x140bfe650 `cmp byte [h+0x16],0`，不符 `assert_eq!`（0x1428ca902，managed/resources.rs）。所以是托管句柄“资源种类”判别（纹理 0 / 缓冲 1 / 描述符集 6）。⚠️ 种类枚举名未取得。已写入 `TextureRecord` 注释。

## 4. 纹理视图缓存与“全局 id 计数器” ✅（纠正此前候选）
* 视图缓存就是 `RwLock<std HashMap<TextureViewDesc, 视图句柄>>`（hashbrown + RandomState SipHash-1-3，`#[derive(Hash/PartialEq)]`），**没有私有位图/校验和**：“校验和”= hashbrown 的 h2（哈希高 7 位 `>>0x39`），“位图”= SSE2 组匹配掩码，“0x48B 槽”= 桶（键 0x3c→0x40 + 值 u64，位于控制字节之下）。0x140bdc5d0 / 0x1418bd030 / 0x1418bd600。
* “纹理全局 id 计数器”不存在：0x140bdc2d0 里递增的线程局部 `{inited,k0,k1}` 是 std `RandomState::new()` 的每线程密钥（首次用 OS 随机 0x140d64e00，之后 `k0+=1`）。
* ManagedTexture 内部字段表（0xC0B）已写入 `managed/texture.rs` 的 `ViewCache` 注释：`+0x30 设备上下文 / +0x38 RwLock / +0x40 ctrl / +0x48 bucket_mask / +0x58 items / +0x60 RandomState / +0x70 默认视图 / +0x78 内存块`。现有 `HashMap<ViewKey,..>` 语义等价（仅哈希器不同），不改。

## 5. create 失败路径 panic 文案 ✅（落常量，不改行为）
`create_texture`：`"create_image"`（0x142b18af8）、`"bind_image_memory"`（0x142b18b70）、`"set_debug_utils_object_name"`（0x142b18b38）；`create_buffer_impl`：`"create_buffer"`（0x142b157f0）、`"bind_buffer_memory"`（0x142b15888）、同名 debug 文案（0x142b15850）。均为 `.expect(msg)`（格式 `"{msg}: {Debug(vk::Result)}"`）。落 `texture::create_failure` / `buffer::create_failure` 常量 + `texture::create_failure_text`；创建路径仍返回 `Err(Error::Vulkan)`（改 `Error` 变体要动 `error.rs`，不归我，且有 OOM 处理依赖）。断言文案（`desc.usage != empty`、`extent.* > 0`、`initial_data.byte_count() <= desc.size`、`size > 0`、SAMPLED 等）此前/本轮均已落码，测试对照 EXE 字节。

## 6. ALGORITHMS §4 其余 U
U#2/#3/#4/#14/#15 与上面各项重叠部分已闭合（#4 = §3 niche；#2/#3 = §1 排布与区域大小；#14/#15 = §4 视图缓存/计数器）。其余未在本轮范围内再读。

## 改动文件
* `src/raw/texture_upload.rs`（新增 ~270 行纯函数，不动原有批量区域上传）
* `src/raw/setup.rs`（替换 `upload_size/upload` 为区域核心 + 新 API；`update_texture` 重写）
* `src/raw/device_api.rs`（`create_texture_with_data`；`upload_via_staging` 名字形参）
* `src/raw/texture.rs`（`create_failure` 常量与 `TextureRecord.flag` 注释）、`src/raw/buffer.rs`（`create_failure` 常量、`SLOT_ALLOCATION_TAKEN` 注释）
* `src/raw/managed/texture.rs`（`ViewCache` 字段表注释）
* 新增测试 `tests/wave2_texture_upload.rs`（18 项，下）。`tests/raw_core_buffer_texture.rs` 未改，仍通过。

## 新增测试（均通过，lavapipe）
纯函数：`mip_chain_*`（1x1 收敛/提前结束/BC 块向上取整/断言顺序）、`linear_regions_are_layer_major_then_mip`、`linear_regions_data_size_checks_*`（underflow / 多余 / 空 / 越界）、`cube_uses_array_elements_*`、`per_mip_regions_*`、`copy_plan_*`；EXE 锚点 `panic_strings_and_names_*`、`region_table_and_copy_closure_machine_code_anchors`、`create_failure_expect_messages_*`。
GPU 读回（上传 → `vkCmdCopyImageToBuffer` → 逐字节比对）：`gpu_multi_mip_multi_layer_linear_upload_*`（8x4 RGBA8，3 mip × 4 层）、`gpu_per_mip_slices_*`、`gpu_block_compressed_mip_chain_*`（BC1_RGB / BC3 / BC7，16x8 三级 mip；lavapipe 实测支持，非跳过）、`gpu_create_texture_with_plain_slice_*`、`gpu_update_texture_regions_*`（只覆盖指定 mip/层，其余不变）、BC 被 2 号路径拒绝、原表外 BC 被拒绝。

验证：`cargo test --workspace --all-features --no-fail-fast`：rhapsody、country-core、tiny-glade 全绿；失败的是 `asset`/`asset-pipe`/`shader-compiler`/`tg-text` 里与本改动无关的既有项（缺外部数据 / 文本布局 `fragment.rs:583 unwrap None`），未碰。

## 对其他 agent 的需求
* structure agent：若要把 `inner+0x7c`/`+0xbc` niche 或“种类 flag 断言”做成托管句柄的真实检查，在 `managed/{handle,resources}.rs` 里落（我只落了文档）。
* `device.rs` 的 `GraphicsDevice::init_texture_data/update_texture` 仍转调旧签名（行为已扩展到多 mip/层），无需改；若要暴露 `init_texture_data_with` 可加一行转调（`device.rs` 不归我）。
* 访问码表（0 / 0x34 / 0x22）精确语义、setup CB 并入图提交（非同步）仍是未闭合项，属同步/提交相关 agent。
