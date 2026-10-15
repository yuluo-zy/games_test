# rhapsody raw-core 域 审查：未补全/未探究清单（2026-10-15）

- 审查 agent：`rhapsody-rawcore-review`。任务：对已交付的 `ALGORITHMS.md`（397 行）做二次审查，逐函数对照骨架清单，找出未覆盖/表面分析/业务逻辑缺口/rebuild 对照缺口。**只产出清单，不改任何源码与既有交付文档。**
- 核对基准：`domains/raw-core.json`（184 compiled / 95 有地址）、`domains/raw-core/decomp/`（95 份）、`evidence/render-recovery/producer-closure/{n1-device,n2-texture-buffer,a5-descriptor-shader,a6-allocator-swapchain,r2-graph-barrier}`、`reconstruction/rhapsody-rebuild/src/raw/**`、`CALL-CHAIN.md`、`PLAYBOOK.md`。
- 标注沿用：**O**=机器码/PDB 实测；**I**=推断；**U**=未知。NotFound 反编译（82B 错误 JSON）不作 O。

## 0. 核对统计（机器核验）

| 项 | 数值 | 说明 |
|---|---|---|
| 骨架条目总数 | 340 | 184 compiled_procedure / 154 inline_instance / 2 inlinee_declaration |
| compiled 有地址 | **94** | 另 1 个非 compiled 有地址（`Queue::with_raw` inline，0x140c03d70）→ 合计 **95 有地址**，与 ALG"95 有地址"一致 |
| decomp 文件 | 95 | 覆盖全部 95 地址；**47 份是 NotFound 错误文件（82B）**，48 份真实反编译 |
| 有地址函数的 verdict 分布（ALG 表） | O≈71 / I≈46 / U=17 / O(derive)≈8 / O(部分)≈7 / O(消费端)≈5 | 17 行 verdict 仍为 **U** |
| 行级覆盖 | 184 行几乎全有（多为**并组行**） | 真正"无行"的 compiled 函数极少（见 §1） |

> 结论：ALGORITHMS.md 的**覆盖度（row 级）很高**，但**深度不均**：大量行是"一个并组行覆盖 N 个单态/闭包实例"、或"NotFound 反编译靠消费端/rebuild 推断"。以下逐节列出未补全项。

---

## 一、未覆盖函数（骨架 compiled 函数无对应分析行 / 只有并组引用）

### 1.1 完全无行

| 函数 | 形式 | 地址 | 卡点 | 建议补法 |
|---|---|---|---|---|
| `rhapsody::raw::device::query_device_extension_support::closure$0` | compiled | — | ALG 无行。父函数 `query_device_extension_support`（inline，line 1041）只在 `GraphicsDevice::create` 行内一笔带过（"扩展筛选"），其内部**按扩展名逐项查询设备扩展支持的算法**（是否走 `vkEnumerateDeviceExtensionProperties` / 内置名表 / 返回 Vec<bool>）未解。 | 从 `GraphicsDevice::create`（0x140bf2560）内部定位内联点，rne-read 该扩展名表与查询循环；对照 rebuild `query_device_extension_support`（device.rs:499 已有，需逐语义核对） |

### 1.2 并组覆盖但未逐实例（编译单态实例 ≥2 个、仅定位部分）

| 函数 | 实例数 | 已定位地址 | 缺什么 | 建议补法 |
|---|---|---|---|---|
| `GraphicsDevice::with_setup_cb<*>` | 5 | 0x140c03fa0 / 0x140c04840（2 个） | 另 3 实例（`update_buffer_bytes`/`init_texture_data`/`update_texture_impl` 闭包）地址与各自录制内容未逐一核 | rne-read 未定位实例；或确认 ICF 折叠到已定位 2 地址并注记 |
| `GraphicsDevice::with_global_allocator<*>` | 4 | —（无地址） | 四游戏闭包实例（commander_update/shut_down_everything/render_gpu_allocator_gui/report_bug）仅并组 1 行 O(部分)，锁拓扑/返回值差异未逐实例 | 从四个调用点反推内联地址；重点核对 `report_bug` 返回 `AllocatorReport`（与其它实例 tuple 返回不同） |
| `GraphicsDevice::create_buffer<*>` | 3 | 0x140c03460（1 个） | `&[u8]` / `()` 另两实例地址未列 | 同上，rne-read 逐实例（区别在于带不带暂存上传链） |
| `GraphicsDevice::create_texture<*>` | 2 | 0x140c07950（1 个） | `&[u8]` 实例未列 | 同上 |
| `GraphicsDevice::bind_descriptor_set<*>` | 4 | 0x1416a5380/0x1416a5910/0x1416a6010 | **第 4 实例地址未列**（ALG §4 #7 已认领，未闭合）；各实例 `vkCmdBindDescriptorSets` 的 firstSet/count 未核 | rne-read `draw_stream::impl$11::render::closure$0` 与 `compute_pass_builder::dispatch_impl::closure$0` 内联调用点（CALL-CHAIN §4 #4） |
| `impl$9::update_texture_impl::closure$2` | 1（compiled） | — | 行内仅提"closure$1/2"，闭包（setup CB copy image 细节）未单独分析 | 从 `update_texture_impl` 调用点（n2 侧）定位 |
| `Queue::with_raw<present_image closure_env,...>` | 1（compiled） | — | ALG 行是 inline 基类 `Queue::with_raw@0x140c03d70`，**present_image 单态实例**（swapchain present 闭包）未单独核 | 从 `Swapchain::present`/`present_image` 调用点定位 |

---

## 二、表面分析（有行但 role/算法空泛、证据弱、业务逻辑未讲清）

### 2.1 大函数只浓缩一行（O 但实质未展开）

| 函数 | 地址 | 反编译体量 | ALG 现状 | 缺什么 | 建议补法 |
|---|---|---|---|---|---|
| `GraphicsDevice::create` | 0x140bf2560 | **2625 行 / 155KB** | 1 行（队列族 stride 0x1c、特性/扩展筛选、SubmissionEngine） | **未逐块解**：物理设备枚举、queue-family 遍历与 `QueuePlan::select` 判据、feature/extension 挑选逻辑、验证层/debug utils 装配、allocator 初始化、device_uuid 提取、设备代号（`+0xa50`）来源。这是 raw-core 域最大的业务主体。 | 按段 grep 大函数（错误串/扩展名/常量），与 raw-init 域 `create_device 0x140bdb940`、n3-init-instance 交叉；建议产出 create 的分段算法纪要 |
| `Queue::reclaim_command_buffer_resources` | 0x140bf0cc0 | 748 行 / 25KB | 1 行（"按时间线/序号回收…交接资源所有权"） | 未展开：fence/timeline 完成判定、弹环机制、命令缓冲回池与 reset 时机、跨队列桶（`+0x9d8` 附近 vec）操作、大量 panic 路径（FUN_1428d9xxx）。 | 分段反编译，输出回收状态机（哪些提交可回收、归还/销毁动作序列） |
| `Swapchain::acquire_next_image` | 0x140c0eb40 | 7KB | result 分派 + 产出租约 | 未解：结果编码（成功分支 vs 失败分支 `*(+0x5e)=7`）、租约结果 0x50B 布局逐字段、present_image 闭包日志路径（UTF-8 解析 `0x142b19bb8`）。 | 反编译全文逐字段回填租约结果结构 |
| `GraphicsDevice::wait_idle` | 0x140bf8980 | 6.9KB | "vkDeviceWaitIdle + 清空待决" | 未展开清空待决的具体动作（哪些队列/时间线/提交被 drain） | 反编译全文 |
| `GraphicsDevice::wait_idle_and_recycle_resources` | 0x140bf8e60 | 7.8KB | "wait_idle + 遍历退役桶" | 未展开：`FUN_1418c0a70(dev+0x648, dev)` 注册表钩子、`+0x718` 回收锁、`FUN_1418c3300(+0x720)` 逐队列推进、**全局退役哈希表（0x1436308f0）遍历**——与 U#8 同源 | 反编译全文 + 与 raw-init `disposal.rs` 桶结构交叉 |

### 2.2 有行但证据为"消费端推断"（未机器码复验，标 O(消费端)/I 面大）

| 函数族 | 数量 | 现状 | 风险 | 建议补法 |
|---|---|---|---|---|
| `TextureDesc::usage/sampled/storage/transfer_dst/color_attachment/depth_stencil_attachment/sampled_with_different_format/format/mip_levels/all_mip_levels/sample_count` 等 | ~13 | **全 NotFound**，ALG 标 O(消费端)/U（`storage=usage\|=8`、`transfer_dst=usage\|=2` 等来自消费端） | 位值未逐字节核；`sampled_with_different_format`/`all_mip_levels` 语义靠推断 | rne-read `--hex` 批量复核（PLAYBOOK §3.2）；至少核 usage 位掩码与 format/mip_levels 的读偏移 |
| `TextureViewDesc::{new,view_type,usage,format,aspect_mask,base_mip_level,level_count,base_layer,layer_count,bind_individual_mips}` | 11 | **全 NotFound**，0x3cB 布局来自 country-core `rhapsody_types.rs`（消费端） | `Option<u32>` 标签/值写法的精确 ABI（r8d 标签/r9d 值）未机器码确认 | rne-read 构造函数字节；与 `Ref::bind_view` 0x3cB 整拷交叉 |
| `RenderPassAttachmentDesc::{new,clear_input,sample_count}` | 3 | 全 NotFound，O(消费端) | 0x10B 布局与 `depth_stencil` 字段存在性 U（见 §4 #6） | rne-read + `create_render_pass` 键构造点反推 |
| `ImageBarrier::new` / `BufferBarrier::new` | 2 | 全 NotFound，**verdict U** | 结构字段（src/dst stage、layout、aspect）纯猜测 | rne-read；`SynchronizationScope1` 消费点反推 |
| `TextureDesc::half_res` | 1 | NotFound，**verdict U** | 奇尺寸修正语义未核 | rne-read + `div_extent` 交叉 |

### 2.3 有行但 verdict 仍 U（反编译 = NotFound 错误文件，无法当 O）

见 §5 清单（17 个 U 行 + 若干 O(消费端)），其中**设备微函数 5 个**（`device_uuid`/`physical_device`/`debug_utils`/`refresh_memory_budget`/`min_subgroup_size`）与**动态常量 2 个**（`DynamicConstantsChunk::device_address`/`into_gpu_lifetime_tracker`）、**`get_vk_blend_mode`**、**`RenderStats::snapshot_and_reset`**、**`Swapchain::{format, any_present_successful}`** 都是"反编译了但结论仍 U"。

---

## 三、业务逻辑缺口（处理链断点 + 未深挖算法）

### 3.1 Buffer 生命周期链（最高价值，U#1~#4）

```
create_buffer(0x140c03460) → create_buffer_impl(0x140bee200) 0x48B 结果
  → 40B Buffer 截取（U#2：+0x30 usage/+0x38 mapped/+0x40 size 是否在 Buffer 外？）
  → 游戏面 TypedStorageBuffer 整块搬运 40B（CALL-CHAIN #7）
  → device_address(0x140bd4c00) / mapped_slice_mut(0x140bd4cd0) 三校验
  → update_buffer_bytes(0x140beefa0) 多区间上传
  → 暂存+with_setup_cb → 提交 → reclaim
```
- 断点 1（U#1）：`+0x00` 的 `0xb` tag 是 Result 判别还是 ManagedHandle 字段——未定。建议：反编译 0x140c03460 的 Result 展开 + 游戏 `SsboType::new_storage_buffer` 消费点。
- 断点 2（U#2）：80B Result → 40B Buffer 的字段映射未定。建议：`import_unowned 0x140bd4fc0` 整拷对照。
- 断点 3（U#3）：`Buffer+0x20` 字段（mapped 基址 vs device_address 缓存）未定。建议：rne-read `create_mapped_buffer 0x140beea80` 字段回填。
- 断点 4（U#4）：`inner+0x7c`（set_buffer_debug_name 用）与 `inner+0x28`（update_buffer_bytes 用）双属性位并存，关系未定。建议：rne-read 分配器返回结构（0x140c113b0/0x140c02280 契约）。
- 断点 5：`get_vk_buffer`（0x140bef470，注册表 thin lookup）与 `get_vk_image_view`（无地址）——注册表条目布局（`lVar6+0x20` 区）未定；`get_vk_image_view` 消费点 `Result::unwrap` 的 Err 载荷未恢复。

### 3.2 GraphicsDevice 方法族与结构（断点）

- 微函数 5 个（0x140bf84c0/8950/8960/8f80/8fa0）全部 NotFound；`refresh_memory_budget` 有 raw-init 侧 0x140c01620 的滞回算法但**GraphicsDevice 方法本身（0x140bf8f80）未解**；`physical_device`/`debug_utils` 返回的字段位置（`+0x78+0x5c8` 区）未定。
- 结构未用字段：`+0x168`/`+0x3f8`/`+0x400`/`+0x408`（n1 §6）ALG 未登记；ash 函数表内部精确排序未核；`dev+0x808`（管线缓存）身份已用于 create_raster_pipeline 行，但 `+0x3a0/+0x200/+0x210/+0x1c8/+0x320` 的 Vulkan 身份仍 U。

### 3.3 命令缓冲/提交链（断点）

- `ensure_active_cb`(0x140bebc90) / `submit_pending_cbs`(0x140bebdf0) / `queue_submit`（0x90B SubmitInfo 组装、setup CB 并入图提交）只有"O(部分)"（MODULE_STATUS / CALL-CHAIN），**命令缓冲 end→submit→fence/timeline 信号的完整收尾序列未展开**。
- `wait_submit_done` / `wait_duration_or_until_submit_done`（skeleton line 363/394）全 I：`vkWaitForFences` 与超时返回语义未机器码核。
- `SubmissionEngine::update_pending_submission_timestamp`（line 107）I，未核队首时间线更新。
- `CommandBuffer::begin` 的级别派生 `(val==0)?1:4`（来自 `*(dev+0x658)+0x78`）标记 U，未核。

### 3.4 屏障链（断点）

- `issue_pipeline_barrier`(0x140bee030) / `issue_full_pipeline_barrier`(0x140bee090)（地址来自 n1，**本域 decomp 无文件**——引用路径需改为 `n1-device/0x140bee0*.c`）：作用域构建（src/dst stage 选择、image layout 转换、`0x3e0` 分派 10 参）已列，但 `SynchronizationScope1` 各屏障条的**实际填充来源**（谁算 old/new layout、aspect）未深挖。
- `image_aspect_mask_from_format`(0x140bd6630) NotFound、O(消费端)：VkFormat→aspect 内建表的具体条目未机器码核。

### 3.5 纹理上传链（断点）

- `init_texture_data`(0x140bfe270)：`*desc&4` 走 **jumptable（0x140bfe2be，基址 0x142b174fc，Ghidra 无法恢复）**——U#9 未闭合；各 sample/格式路径（逐 mip copy / 直接写）未解。建议：rne-read 跳表基址逐目标反编译。
- `update_texture`/`update_texture_impl`：无地址、O(部分)；`LinearInitialDataIter`/`MipChainIter` 的 mip 切块与对齐算法为 I。
- `get_image_create_info`(0x140bd5ed0) O：kind 跳表 0..6 与 `+0x50` has_data<<3 已列，但 kind=5/6（Cube/CubeArray）与 3D 的 arrayLayers 换算细节、`0xd/0xe/0xf/0x10` 坏指令区成因未核。

### 3.6 descriptor 链（断点）

- `DescriptorSetState::allocate_descriptor_set`(0x140be5a40) O：空闲池+回收重试已定，但 **`DescriptorSetBinding` 二进制布局未定**（into_uniform_buffer_binding 产出 `{tag6 + 16B managed handle + device_address + offset + size + byte}`，0x140c022b0 反编译可见；rebuild 是 `enum DescriptorSetBinding::Buffer{ty,info,owner}`——两侧结构未对照）。
- `create_persistent_descriptor_set`(0x140bfef70) 0x18B 结果、`maxSets=1 flags=2`、tag=6 已列，但池尺寸表累加逻辑与 `ManagedResourceStore::allocate` 接线为 a5 摘要级。
- `bind_descriptor_set` 第 4 实例地址（U#7）未闭合。
- `get_vk_descriptor_set` 校验（`+0x16==6`、`+0x14==store+0xa50`）已 O，但 store/注册表内部（0x1418aead0/0x1418aeba0/0x1418aee50 族）未深挖（属 managed 域，交叉登记即可）。

### 3.7 DynamicConstants 链（断点）

- `DynamicConstants::allocate`(0x140c02740) O：256KB 上限/对齐游标/溢出建新 chunk 已列；但 **U#11：chunk 双 size（+0x08/+0x18）双 offset（+0x30/+0x40）语义未定**——是 `{uniform,storage}` 双 region 还是对齐冗余。建议：`into_uniform_and_storage_buffer_binding 0x140c02370` 的绑定区域读取对照（反编译显示绑定 A 用 `param_2+0x40` offset、`+0x18` size；绑定 B 同源——双绑定共享同一 region 的可能性高，需定案）。
- `DynamicConstantsChunk::device_address`(0x140c02280) / `into_gpu_lifetime_tracker`(0x140c02290) 全 NotFound、U：chunk 的 device_address 缓存与 lifetime tracker 语义未解。

### 3.8 swapchain 各链（断点）

- **U#8：全局退役哈希表（0x1436308f0）**——`mark_stale`/`on_focus`/`wait_idle_and_recycle_resources` 尾部都遍历它（fibonacci 散列 + refcount + marker 桶，条目 0x40B），具体结构（条目字段、与 disposal 桶的关系、何时刷新交换链图像）未定。建议：构造入口 `0x14290b400/0x14290b740/0x14290b9d0` 族反编译；与 raw-init disposal.rs（HashMap<CommandBufferSeq, Mutex<DisposalBucket>>）交叉。
- `validate_swapchain_and_acquire_image`(0x140bfd9d0)：SUBOPTIMAL(2)/OUT_OF_DATE(4)/OK 分派已列，但 switch 各分支动作（retire/重建/重试）未展开。
- `Swapchain::format`/`any_present_successful`/`TrackedSwapchainTextureLease::image` 三个 NotFound、U。
- `enumerate_surface_formats`(0x140c0e650)：从配置管理器读 cap/格式，但格式挑选（哪几个被实际接受）未核。
- `create_swapchain`：无地址、O(部分)，present_mode 筛选与 `old_swapchain` 语义未展开。

### 3.9 其它

- `RenderStats::snapshot_and_reset`(0x140c02da0) NotFound、U：统计字段布局未解（建议 `render_frame.rs::snapshot_render_stats` 消费点反推）。
- `Surface::create`/`TempList` 全 I：winit→vkCreateSurfaceKHR 与 pNext 临时链仅形状。

---

## 四、rebuild 对照缺口（原版 ↔ rebuild）

### 4.1 对照表本身有 **过时/失实** 项（重点）

| 原版 | ALG §3 判定 | 当前 rebuild 实际（2026-10-15 复核） | 结论 |
|---|---|---|---|
| `DescriptorSetState::allocate_descriptor_set`（空闲池复用） | **✗**："rebuild 用按写入内容寻址的缓存（BTreeMap<Vec<u64>,Arc<Set>>）——候选自造机制（a5 §7）" | rebuild `descriptor_set.rs` 现已是 **空闲池复用 + 来源返还 + OUT_OF_POOL/FRAGMENTED_POOL 循环重试**（`DescriptorPoolQueue`/`DescriptorPoolSource.returned`/`RetiredDescriptorPool`，`cached_set_count()` 注释"内容缓存已经移除"） | **STALE**：a5 §7 与 ALG 都抄了旧代码。当前 rebuild 行为与原版一致，应改判 **✓/半**（剩余差异 = 原 `+0x60` 空闲池 Vec vs rebuild 池栈 + 来源批处理细节） |
| `bind_persistent_descriptor_set` | **✗**："rebuild 缺" | rebuild `binding.rs:89` 已有 `bind_persistent_descriptor_set` free fn（同底层 cmd_bind_descriptor_sets） | **STALE（半）**：缺的是"GraphicsDevice 方法"身份，不是功能 |
| `bind_descriptor_set` ×4 | **✗**（#4）："rebuild 只有 rg/binding free fn" | rebuild `binding.rs:112` 有 `bind_descriptor_set` free fn | 差异说明基本成立，但"缺"措辞过强；真缺的是 4 个游戏闭包单态 + 设备方法 |

### 4.2 对照表未登记的结构/行为差异（新增）

| 原版 | rebuild | 现状 | 建议 |
|---|---|---|---|
| `DynamicConstantsChunk` = **0x48B**（device_ptr/size×2/inner+offset/buffer/chunk/offset×2/gen/flag） | rebuild `{Arc<Buffer>, offset, size}`（32B） | ALG 标 **✓** | 布局不一致未登记；建议按 §3.7 先定双 region 语义再改对照 |
| `DynamicConstants::new`（0x140c02570）**预建 ~256KB chunk** | rebuild `DynamicConstants::new` **惰性**（pages 空，首个 allocate 才建页） | ALG 标 ✓ | 初始页存在性行为差异未登记 |
| 原 `DescriptorSetBinding` 产出（into_uniform_buffer_binding 0x140c022b0）= `{tag=6, 16B managed handle, device_address, offset, size, byte}`（0x2cB 级） | rebuild `enum DescriptorSetBinding::Buffer{ty, vk::DescriptorBufferInfo, owner}` | 未对照 | 两侧结构完全不同，需要统一的绑定载荷 ABI 登记（CALL-CHAIN INDEX §5.9 RenderPassBinding 同源） |
| `into_uniform_and_storage_buffer_binding` 原产出 **两个含 managed tag 6/5 的绑定** | rebuild 返回 `(uniform, storage)` tuple | ALG 标 ✓ | tag 5/6 编码 vs rebuild 枚举变体未对照 |
| `Buffer` 40B 句柄 | 16B desc+句柄对象 | ✗（#7）已知 | 未闭合；需游戏面转换或改 game 侧 |
| `Texture` 40B 句柄 / 0x38B | rebuild `Texture{Arc, handle, desc, allocation}` 对象 | 半（#14 U） | 0x38B vs 40B 关系待对齐 |
| `GraphicsDevice` 真实结构 | `Arc<VulkanDevice>` 包装 | 半（#18） | 已知 |
| `RenderPassDesc` 缓存键 `(ManagedHandle,u32)+ArrayVec<AttachmentDesc,8>` | `RenderPassKey(Vec<AttachmentKey>)` | 半（#6/#14） | `RenderPassAttachmentDesc` 0x10B 的 `depth_stencil` 字段存在性 U 未闭合 |
| `mapped_slice_mut(&self, device)` / `device_address(&self, device)` | `(&mut self, offset, size)` / `(&self)` | 半（#8/#9） | 已知 |
| `get_vk_buffer` / `get_vk_image_view` | **无** | ✗（#6） | 注册表 thin lookup 未恢复 |
| `create_framebuffer` | 无 GraphicsDevice 方法（framebuffer.rs 仅 desc 构建） | ✗（#6） | 已知 |
| `create_command_buffer` | `VulkanCommandPool::allocate` + `VulkanCommandBuffer` | 半 | 原池空闲链（dev+0x9b0 锁）vs rebuild 每池互斥+新分配——行为差异未验证 |
| `GraphicsDevice::create_swapchain` | `Swapchain::create`（swapchain.rs:280，对象方法） | ✓（对象级） | 原为 GraphicsDevice 方法（swapchain.rs），身份差异未登记 |

### 4.3 CALL-CHAIN 一致性

CALL-CHAIN §3.2/§3.3（19 对上/11 缺）与 ALG §3 基本一致；但 CALL-CHAIN 的"缺 get_vk_buffer/get_vk_image_view/create_render_pass/create_framebuffer/create_command_buffer/bind_*"清单与 ALG 的"半"判定有措辞差（如 create_command_buffer 在 ALG 判半、CALL-CHAIN 判缺）。建议以 ALG §3 为准补一条"rebuild 有 free fn 但无设备方法"的分类。

---

## 五、反编译了但结论仍 I/U 的清单（95 地址分类）

### 5.1 47 份 NotFound（82B）→ 对应行 verdict

**verdict 仍 U 的 17 行**（数据存在、Ghidra 无函数体、未 rne-read 补证）：

| VA | 函数 | 
|---|---|
| 0x140bf84c0 / 0x140bf8950 / 0x140bf8960 / 0x140bf8f80 / 0x140bf8fa0 | `device_uuid` / `physical_device` / `debug_utils` / `refresh_memory_budget` / `min_subgroup_size` |
| 0x140be67f0 | `get_vk_blend_mode` |
| 0x140bd5730 | `TextureDesc::half_res` |
| 0x140c02280 / 0x140c02290 | `DynamicConstantsChunk::device_address` / `into_gpu_lifetime_tracker` |
| 0x140c02da0 | `RenderStats::snapshot_and_reset` |
| 0x140c0e7e0 / 0x140c0ef70 | `Swapchain::format` / `any_present_successful` |
| 0x14003bb80 / 0x140be6760 | `ComputePipelineDesc::new` / `RasterPipelineDesc::new` |
| 0x140bd65e0 / 0x140bd6610 | `ImageBarrier::new` / `BufferBarrier::new` |
| 0x140c0f120 | `TrackedSwapchainTextureLease::image` |

**verdict 为 O(消费端)/I 的 ~30 行**（消费端推断，未机器码复验）：
- TextureDesc getter 13 个（0x140bd5270..55a0 区）
- TextureViewDesc 11 个（0x140bd57a0..59f0 区）
- `extent_2d`、`image_aspect_mask_from_format`、`RenderPassAttachmentDesc` 3 个

> 建议：这批 ~30 个"消费端推断"是 ALG 中证据链最弱的一块，全部可用 `rne-read --hex` 批量复核（PLAYBOOK §3.2），一次跑完即可把 O(消费端) 升级为 O。

### 5.2 48 份真实反编译 → 表面分析（见 §2.1）

大函数（create/reclaim/wait_idle/wait_idle_and_recycle/acquire_next_image/mark_stale/on_focus）只浓缩，实质未展开。

---

## 六、补全优先级排序

### P0（阻塞游戏面接线 / 高价值未闭合）

1. **Buffer 40B 生命周期定案**（U#1/#2/#3/#4）：create_buffer 80B→40B 截取、`+0x00` tag 0xb 语义、`+0x20` 字段、`inner+0x7c`/`+0x28` 双属性位。补法：rne-read create_buffer 展开 + import_unowned 对照 + 游戏 SsboType 消费点。
2. **`GraphicsDevice::create`（0x140bf2560，155KB）分段算法纪要**：raw-core 域最大业务主体，当前仅 1 行。补法：分段 grep + 与 raw-init create_device 交叉。
3. **rebuild 对照表修正（STALE 3 项）**：`DescriptorSetState::allocate_descriptor_set`（✗→✓/半）、`bind_persistent_descriptor_set`（✗→半）、`bind_descriptor_set` 措辞——这是最容易被外部误用的失实信息。
4. **`bind_descriptor_set` 第 4 实例地址**（U#7）与 `get_vk_image_view` 注册表消费点。

### P1（机制层未闭合）

5. **全局退役哈希表 0x1436308f0**（U#8）：mark_stale/on_focus/wait_idle_and_recycle 三处共用，条目 0x40B 结构 + 与 disposal 桶关系。补法：构造入口 0x14290b400 族。
6. **`init_texture_data` jumptable**（U#9）：rne-read 跳表 0x142b174fc 逐目标。
7. **`Queue::reclaim_command_buffer_resources`（25KB）状态机展开**：回收完成判定与回池时序。
8. **`DynamicConstantsChunk` 双 region 语义**（U#11）+ chunk 0x48B 布局在 rebuild 对照登记。
9. **5 个设备微函数 rne-read 补证**（0x140bf84c0 族）——含 `refresh_memory_budget` GraphicsDevice 方法本体。

### P2（批量补证，成本低收益明确）

10. **~30 个 NotFound 叶子 getter 批量 rne-read `--hex` 复核**（TextureDesc/TextureViewDesc/RenderPassAttachmentDesc/ImageBarrier/BufferBarrier 等），把 O(消费端) 升级为 O。
11. **`queue_submit`/`wait_submit_done`/`wait_duration_or_until_submit_done`/`update_pending_submission_timestamp`** 提交收尾序列机器码核（当前 I/O(部分)）。
12. **`with_setup_cb`/`with_global_allocator`/`create_buffer`/`create_texture` 未定位实例** rne-read 确认 ICF 折叠或独立地址。

### P3（低优先）

13. `Surface::create`/`TempList` 形状核（I，接入时再做）。
14. `enumerate_surface_formats` 格式挑选、`create_swapchain` present_mode 展开。
15. ALG §1.1 文件→compiled 计数校正（buffer 21 vs 23、texture 26 vs 47、shader 13 vs 17、swapchain 19 vs 18、barrier 9 vs 10、binding 8 vs 7、render_pass 3 vs 1、sampler 2 vs 1——现列与实际骨架 compiled 数不符）。
16. ALG 引用路径卫生：`decomp-0x140bee030/090/150` 三个文件在本域不存在（在 n1-device），应改引 `n1-device/0x140bee0*.c`（同类还有 get_vk_buffer 0x140bef470 等，地址来自跨域 .c 而非本域 decomp）。

---

## 附：核对方法

- 脚本解析 `raw-core.json`（184 compiled / 95 地址）与 ALG 表行，按函数短名匹配；NotFound 按 82B 文件判定；decomp 95 份 VA 与骨架 95 地址完全对应（94 compiled + 1 inline `Queue::with_raw`）。
- 业务证据交叉：重读 n1/n2/a5/a6 findings、`allocate_descriptor_set`/`into_uniform_buffer_binding`/`acquire_next_image`/`wait_idle_and_recycle_resources`/`create` 等反编译原文。
- 未修改任何 reconstruction 源码与既有交付文档。
