# raw-core 域：描述符 / 着色器管线 / 渲染通道 / 绑定 复现报告

范围：`crates/rhapsody` 的 `raw/{descriptor_set,descriptor_binding,binding,shader,render_pass,framebuffer,sampler}.rs`、`rg/binding.rs`、`crates/shader-pipeline`。
证据来源：原版 patched EXE 机器码（`/tmp/dis.py` 逐字节反汇编），辅以 `country-core-deep/rhapsody/domains/raw-core` 反编译。图例：✅ 原版对齐[证据地址] / ⚠️ 候选或假设 / ❌ 读不出。

## 0. 事故说明（必读）
中途一次 Python 批量替换把 `raw/shader.rs` 误写成 `descriptor_set.rs` 的内容（我的脚本 bug）。该文件含上一会话**未提交**的 M11 改动，git HEAD 里是更早的旧版（`RasterPipelineOpts` 还是 `Option` 字段结构）。我用 HEAD + 本会话读过的原文 + 机器码重新拼回，`cargo test` 全绿，但**存在无法排除的细微偏差**：
- 我没读过的两段（Opts 的 getter/setter 与 `raster_opts_tests` 旧测试）是按位表重写的；旧的三个测试名保留，内容重写成逐位断言；
- 其余区段（`ShaderLibrary` / `from_reflection` / `stored_pipeline_entry` 等）与我事前打印的版本一致（偏移差恒为 +59 行，已核）。
被覆盖前的破坏现场副本在 `/tmp/shader_clobbered.rs`（内容即 descriptor_set.rs），重建基础 `/tmp/shader_head.rs`。建议把本文件当作“需复核”的重建，复核点：`RasterPipelineOpts` 的公开方法名/签名（已用 tiny-glade 的 `render_host` 调用方编译验证）。

## 1. create_render_pass 缓存键与创建本体（0x140bfc0d0）
- ✅ **键不是 `ManagedHandle@0 + u32@0x18`**。那是 `RasterPipelineKey{渲染通道句柄, 选项 u32}` 的形状，被旧文档张冠李戴。真实键 `RenderPassDesc` = 0x98B：`ArrayVec<RenderPassAttachmentDesc,8>`（`[T;8]@0` + `len:u32@0x80`，0x84B）+ `depth: Option<(RenderPassAttachmentDesc@0x84, tag:u8@0x94)>`，tag==3 为 `None`（niche）。证据：0x140bfc0d0 读 `[desc+0x80]`、`[desc+0x84..0x94)`、`[desc+0x94]`（跳表 0x142b174cc）；`RasterPass::render` 0x140868370 构造该键（`iStack_a8@0x80, pauStack_a4@0x84, uStack_94@0x94`）；缓存桶 stride 0xb0 = 键 0x98 + 值 `RenderPass`（ManagedHandle 0x18）。落码：`raw::render_pass::{AttachmentArray, RenderPassDesc, DepthAccess}`，`size_of` 断言 0x84/0x98。
- ✅ **深度 tag 跳表**（0x142b174cc 逐项解出）：0→`DEPTH_STENCIL_READ_ONLY_OPTIMAL(4)`、1→`DEPTH_STENCIL_ATTACHMENT_OPTIMAL(3)`、2→`1000117001`(`DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`)、3→无深度。变体名（ReadOnly/ReadWrite/DepthWriteStencilRead）是 ⚠️候选（按布局取名），数值已证。
- ✅ **AttachmentDescription 映射**（0x1418b8c50 的 Chain 展开，常量 0x142d032c0）：`flags=0, format=desc[0], samples=desc.sample_count 原字节, loadOp=desc[4](clear_input 原值), storeOp=desc[8] 原值, stencilLoad/Store=LOAD/STORE(0/0), initial=final=COLOR_ATTACHMENT_OPTIMAL`；深度附件追加在颜色之后，stencil 同上，initial=final=tag 布局。⇒ `RenderPassAttachmentDesc +0x08` 身份确定 = **storeOp**（旧文档“U”闭合），字段更名 `store_op`；`+0x0d..0x0f` 仍只被 `new` 清零，身份 ❌未证实。
- ✅ **子通道与依赖**（0x140bfc31c..0x140bfc335）：单个 GRAPHICS 子通道；颜色引用 `(i, COLOR_ATTACHMENT_OPTIMAL)`；深度引用 `(color_count, tag 布局)`；**`dependencyCount=0, pDependencies=null`**（无任何子通道依赖，旧“不猜测依赖掩码”候选可删）；`pResolveAttachments`/input/preserve 全空。
- ✅ **通道对象负载**（`ManagedResourceStore::allocate` 0x1418ae9f0，槽 0x28B，负载起于 +0x10）：`+0x10 vk::RenderPass`、`+0x18 颜色附件数(u64，不含深度)`、`+0x20 采样数`；采样数 = 有深度取深度附件的 `sample_count`，否则取第 0 个颜色附件的；二者皆无 → 原版 `attachments[0]` 越界 panic（0x140bfc47f），本实现返回 `Err`。管理句柄 kind=4。落码：`RenderPass::from_desc` + 字段 `color_count/samples`。
- ✅ 旧平铺路径 `RenderPassAttachmentDesc::to_attachment_key` 改为与 `from_desc` 同语义（load/store 原值、stencil 恒 LOAD/STORE、initial=final），否则 `device_api::create_render_pass`（别人拥有）会建出与原版不同的通道。`RenderPassDesc::from_flat` 提供平铺→原键转换。
- ⚠️ **需 device_api.rs 拥有者配合**：`GraphicsDevice::create_render_pass` 仍用 `RenderPassKey`（平铺）做缓存键；建议改为 `HashMap<RenderPassDesc, ManagedRenderPass>`，创建走 `RenderPass::from_desc`，并用 `RenderPassDesc::from_flat(attachments)`（或直接接受 `RenderPassDesc`）。
- ❌ `vkCreateRenderPass` 失败时的 panic 文案（0x142b16b08 起的 `Result::unwrap` 载荷）未逐字落码，返回 `Error::Vulkan`。

## 2. RasterPipelineOpts / create_raster_pipeline 回放核对（0x1416a3720）
逐字节重读 0x1416a3720..0x1416a4478（含 0x1416a3764 前缀、0x1416a4325 调用点）：
- ✅ 位域全部与机器码一致（拓扑跳表 0x142c88ff0：3/1/2；深度比较 0x142c89000：LESS/GREATER/EQUAL；剔除 `(b0>>2)&3`；深度写/测 b0 bit6/7；模板 fail `(b1>>1)&7`、pass `(b1>>4)&7`、depth-fail `(b1>>7)+(b2&3)*2`、compare `(b2>>2)&7`、mask `(b3<<3|b2>>5)&0xff`、write mask `0xffffffff`、reference 0；多边形 `b3>>7`；混合 `b3>>5&3`）。无修正，补了逐位测试。
- ✅ 固定状态：front_face=0(CCW)、`depthClamp/rasterizerDiscard/depthBias` 关、`lineWidth=1.0`、`depthBoundsTest` 关、bounds [0,1]；多重采样 `rasterizationSamples=[pass+0x20]`；混合附件数 `=[pass+0x18]`（颜色数），每个都是同一份 0x140be67f0 状态；**子通道 `subpass=0`**（`[rbp+0x330]=0`），`basePipelineHandle/Index=0`；管线缓存取 `device+0x808`。
- ✅ **动态状态**：表 0x142c88b20=`{VIEWPORT,SCISSOR,STENCIL_REFERENCE}`，0x142c88b2c=`{…,LINE_WIDTH}`，个数 `3+[device+0xad9]`。
- ✅ **`[device+0xad9]` 来源**：`GraphicsDevice::create`（0x140bf2560）里 `0x140bf3a6d: setne [rbp+0x327f]` → `0x140bf66c5: mov [rsi+0xad9]`；其源 `[rbp+0x22fc]` 位于要启用的 `PhysicalDeviceFeatures` 链第 15 槽（基址 0x22c0 + 15×4 = **wideLines**，同结构 0x22dc=dualSrcBlend 为必需特性检查）。即“设备实际启用 wideLines”，已有 `VulkanDevice.capabilities.wide_lines`。落码：`create_raster_pipeline` 用设备能力决定 LINE_WIDTH，**删除 `RasterPipelineDesc.line_width_dynamic` 字段**（全仓只有 shader.rs 用）。
- ✅ **着色器阶段与 `vertex_entry` 来源**：阶段闭包 0x141477010：每个 0x38B 阶段项 `+0x08/+0x10`=入口名 (ptr,len)→`CString::new`（失败 `Result::unwrap` panic）→`pName`；`+0x20/+0x28`=SPIR-V 字数→`vkCreateShaderModule`（sType 16）；`+0x30` 字节：1→VERTEX，2→FRAGMENT，其他 `unreachable`。即入口名来自**编译容器各 stage 的 entry 字符串**，没有任何常量 `vs_main`/`ps_main`；现有 `stored_pipeline_entry` 用 `stage.entry_point`，正确。`RasterPipelineDesc::new` 里的 `"vs_main"` 默认纯属候选便利。
- ✅ 顶点输入：每个 location 一个绑定（binding=location, stride=类型字节, offset 0, VERTEX），格式走 `map_vertex_attribute_format`；原版迭代 hashbrown 表序（任意），本实现按 location 升序（Vulkan 语义等价，⚠️顺序差异）。
- ✅ 管线对象返回 0x38B：`{DescriptorSetState Vec(24B)@0, 编译对象引用@0x18, VkPipelineLayout@0x20, VkPipeline@0x28, bindPoint:u32@0x30}`（绑定点 GRAPHICS=0）。
- ⚠️ 原版 `create_raster_pipeline` 没有按 desc 缓存（缓存只由 Vulkan 管线缓存承担）；本实现仍有 `raster_pipeline_key` 内存去重（优化，非原行为，保留）。
- ❌ 0x1416a4478 之后的错误尾段（0x1416a45a4..0x1416a45f4）只是 `Vec`/`Arc` 分配失败与 `Result::unwrap` panic 路径，未落码。

## 3. 描述符
- ✅ **`create_descriptor_set_layouts`（0x140be60a0）不存在“全局布局缓存”**：`0x140d4d740` 是 std `RandomState` 的线程局部 KEYS（`k0+=1` 取 seed），在**空 set** 分支里为该 set 的 `HashMap` 记录取 seed。原版每个 set、每次调用都 `vkCreateDescriptorSetLayout`(+0x230)，空 set 建空布局（sType 32/bindingCount 0）；`set 0` 阶段掩码取参数，其余 `0x7fffffff`；`assert!(set_count <= MAX_DESCRIPTOR_SETS)`，常量 4（字符串 0x142b13840）。落码：`MAX_DESCRIPTOR_SETS=4`，超过返回 `ERROR_TOO_MANY_OBJECTS`（原版 panic）。⚠️ `ShaderLibrary::prepare_raster_program` 里自带的 `layouts` 内存去重是候选优化，保留。
- ✅ **池参数**（`create_descriptor_set_states` 0x140be66c0 → 0x1418b6140 → 闭包 0x1418b3a00）：每个 `DescriptorSetState`(0x78B) 的池 `maxSets = 100`（0x140be5a40 的 CreateInfo `+0x14=0x64`），池尺寸 = 每种描述符类型 `Σ binding.descriptorCount × 100`，flags 恒 0，`vkAllocateDescriptorSets` 恒 1 个集合。落码：`DESCRIPTOR_POOL_MAX_SETS=100`、`DescriptorSetState::for_layout`、`create_descriptor_set_states(layouts)`（去掉 `sets_per_pool` 参数）。⚠️ tiny-glade `render_host.rs:557/632` 仍传 64（别人的文件），应改 `for_layout`。⚠️ 仍保留 `UPDATE_AFTER_BIND` 池标志（布局带该标志时 Vulkan 要求），原版瞬时池永远 flags=0（它只对不含该标志的布局用瞬时路径）。
- ✅ **`allocate_descriptor_set`（0x140be5a40）回放核对**：自旋锁 +0x58；空闲池 Vec +0x60（弹末尾）；池空 → 取共享来源 +0x50（`+0x10` 自旋锁、`+0x20/+0x28` 返还 Vec）整批并入后**对并入的每个池 vkResetDescriptorPool**，来源也空才新建池；`OUT_OF_POOL_MEMORY(-1000069000)`/`FRAGMENTED_POOL(-12)` 时把末尾池 + `Arc<来源>`（原子 +1）推入调用方回收 Vec 并重试，其他错误 `unwrap` panic。现有 `DescriptorPoolQueue::allocate` 与此逐步一致（已有 4 个单测），无需改动。Vulkan 分派表基址：第二参是 `device+0x78` 内嵌 ash 表，所以 `+0x1c8/+0x1d8/+0x1e0` = vkCreateDescriptorPool/vkResetDescriptorPool/vkAllocateDescriptorSets（旧文档写成“分配/reset/释放”有误，已核 +0x5c8=VkDevice 与 sType 33/34）。
- ✅ **`bind_persistent_descriptor_set`（0x140bee150）**：`get_tracked` → 命令缓冲状态必须 ==1 → `vkCmdBindDescriptorSets(cmd=[rec+0x48], bindPoint=[pipeline+0x30], layout=[pipeline+0x20], firstSet=参数, setCount=1, &set, dynOffsetCount=0)`。落码：`binding::bind_persistent_descriptor_set_single`（原版形态：1 个集合、无动态偏移）；原通用函数是其超集。
- ✅ **U#7 `bind_descriptor_set` 单态**：全 .text 扫描 `call [reg+0x328]`（vkCmdBindDescriptorSets）只有 4 处：0x140bee1c1（持久集）、0x1416a5768 / 0x1416a5d79 / 0x1416a6477（=三个单态 0x1416a5380/0x1416a5910/0x1416a6010）。**不存在第 4 个 `bind_descriptor_set` 单态**（旧文档“4 实例”多算了一个）；`vkUpdateDescriptorSets`（+0x268）全 .text 也只有 4 处：三个单态内联 + `update_persistent_descriptor_set`（0x1416a26a3），所以**没有独立的 `update_descriptor_set` 函数**。单态行为（0x1416a5380 完整读过）：取 `pipeline.states[set_index]`（stride 0x78）→ `allocate_descriptor_set` → 构造 `VkWriteDescriptorSet`(0x40) 数组 → **一次** `vkUpdateDescriptorSets(count, writes, 0, null)` → `device+0xab0` 原子 +1 → `vkCmdBindDescriptorSets(cmd, bindPoint=[pipeline+0x30], layout=[pipeline+0x20], firstSet=set_index, count=1, &set, dynCount, dynOffsets)`，并回退临时 bump 分配器。
- ⚠️ `update_persistent_descriptor_set`（0x1416a2320）：`get_vk_descriptor_set`（0x140bff400）→ 批量构造写入 → 单次 `vkUpdateDescriptorSets` → `device+0xab0` +1；现有 `VulkanDescriptorSet::update` 每次单写一条（API 形状不同）。**未落码**：`+0xab0` 计数器字段属 `device.rs`（别人的文件），需 `VulkanDevice` 增加 `descriptor_update_count: AtomicU64`，之后我可补 `update_batch`。
- ✅ 持久集 `create_persistent_descriptor_set` / `get_vk_descriptor_set` 参数（maxSets=1、flags=2、managed tag=6、校验 `+0x16==6` 与 `+0x14==device 代号`）沿用 a5 findings，本轮没有重新反汇编（仍是 O 级旧证）。

## 4. 着色器侧
- ✅ `map_vertex_attribute_format`（0x140be6850）：重读，`(bytes-4)>>2 ∈ [0,4)` 且 `bytes%4==0` 才合法（`ROR` 技巧），`result = idx*3 + base`，base：tag2→99(R32_SINT)/tag3→98(R32_UINT)/tag4→100(R32_SFLOAT)，其他 tag 与越界 panic。与现有代码一致，补逐格式测试。
- ✅ **采样器名→内置采样器**：`get_built_in_sampler`（0x140bfc5b0）= `device+0x680` 处 `HashMap<name, vk::Sampler>`（桶 0x18B：key ptr/len，句柄 @+0x10）按名字查找；**缺失 `panic!("Sampler not found: {name}")`**（源 `crates/rhapsody/src/raw/sampler.rs:201`，字符串 0x142b16bfd）。落码：`InterfaceError::MissingSampler` 的 Display 与之逐字一致；`sampler::BUILTIN_SAMPLER_NAMES` 九个名字（`sampler_lnc/llr/nnc/llc/llb/default/mipmap/shadow_nearest/shadow_linear`，来自 404 个编译 shader 的反射，O）。
- ❌ 各内置采样器的 `VkSamplerCreateInfo`（过滤/环绕/比较）读不出：`create_samplers` 内联在 `GraphicsDevice::create` 内，全 .text 扫描 `call [reg+0x220]`(vkCreateSampler 槽) 无命中，调用点形态未找到；名字缩写 L/N/C/R/B 的含义属 I。`device.rs::create_samplers` 现在只造 3 个匿名采样器（候选，应换成上面 9 个具名采样器；等调用点找到再填状态）——文件不是我的。
- ✅ **shader defines 哈希（crates 域 U 闭合）**：机器码 0x141795d10（compute）/0x141796080（raster）：defines 元素 stride 0x30（一对 `String`），`acc ^= hash_pair(elem)`，初值 0（0x141795d60..0x141795d73 / 0x1417960d0..0x1417960e3）；逐对哈希 = std `DefaultHasher`(SipHash-1-3, key 0,0) 的 `(String,String)` 元组。**数值复现**：`("SHADER_VARIANT_DEPTH_ONLY","0")`→`0xb903e43ffb3da915`、`("…","1")`→`0xf0adff76827e73ff`，恰好是发行 404 个 .bin 里仅有的两个 raster 哈希（宏名来自 `raster_shader_variants` 0x1417388a0，旧“暴力未复现”是因为宏名不对）。现有 `ShaderDefines::artifact_hash` 本就是该算法，补测试与文档。⚠️ 多宏组合的折叠只有“XOR”这一机器码证据，发行物里没有 ≥2 个宏的样本做数值验证。
- ✅ `ComputePipelineDesc::with_entry`（0x140539370）：入口为空串时默认 `"cs_main"`（已落码）；原 compute 描述布局 `{source_path@0, entry@0x18, defines@0x30}`。**`RasterPipelineDesc`**（0x1409e3980）= 0x48B `{source_path@0, defines@0x18, String@0x30(=path 克隆)}`，文件键只用前两项（0x141796080）；第三个字符串语义 ❌未证实，`vertex_entry/fragment_entry` 是候选便利字段，注明不对应原布局，未改结构（tiny-glade `load_compiled_raster` 在用）。
- ✅ `raster_shader_variants`（0x1417388a0）核对已做（tiny-glade 测试存在）。该函数在 tiny-glade，非我的文件。

## 5. §4 U 卡点与 §3 “半/✗”项落点
| 项 | 结论 |
|---|---|
| U#6 create_render_pass 键 | ✅ 见 §1（键=0x98B `RenderPassDesc`，含 depth Option；附件 0x10B 字段 load/store/clear 全部落到 desc 里） |
| U#7 bind_descriptor_set 第 4 实例 | ✅ 不存在（全 .text 只有 3 个单态）；count=1、firstSet=set_index 已证 |
| §3 create_descriptor_set_layouts（半） | ✅ 无缓存；0x140d4d740=RandomState KEYS |
| §3 allocate_descriptor_set（半） | ✅ 回放核对一致 |
| §3 bind_persistent_descriptor_set（半） | ✅ 参数（count=1/无偏移）已落 `_single` |
| §3 bind_descriptor_set ×4（半） | ✅ 3 单态 + 行为已读；设备方法身份在 device_api.rs（已有） |
| §3 update_descriptor_set / update_persistent（半） | ⚠️ 前者不存在独立函数；后者批量+计数器见 §3 |
| §3 create_render_pass（半） | ✅ 见 §1，另需 device_api.rs 切换键 |
| §3 map_vertex_attribute_format（✓） | ✅ 复核 |
| rg/binding.rs `rg_binding_to_descriptor_binding`（0x14205eff0） | ⚠️ 本轮读出判别跳表（0x142fcf75c，4 项）：`disc-3<3 → disc-2`，disc 3→DescriptorSetBinding **tag 6**（整缓冲：offset=0, range=~0）、disc 4→**tag 5**（同形）、disc 0/1/2→另一路（带 +0x38 标志与 +0x39 起的视图描述，tag 0/1）、disc 5 项目标 0x14205f4c0 未展开；两个缓冲分支先校验资源表项种类 ∈{2,3}、设备代号 `+0x1c`、kind 字节 `+0x1e==1`、代数 `+0xc`。现有 Rust `PassResourceSlot` 4 个变体（Uniform/Storage/Sampled/StorageImage 且带 offset/size）与原 ≥6 个判别式不同，**未改**（缺 `RenderPassBinding` 0x44B 布局证据）。 |
| framebuffer.rs / managed/*.rs | 未改（`vkCreateFramebuffer`（+0x270）本轮没有重读机器码，仍沿用 M10：创建参数按 desc） |

## 6. 改动文件
- `crates/rhapsody/src/raw/render_pass.rs`：新增 `DepthAccess/AttachmentArray/RenderPassDesc/RenderPass::from_desc/color_count/samples`；字段 `reserved_08→store_op`；`to_attachment_key` 对齐原语义；`from_flat`；`RenderPassAttachmentDesc` 派生 `Hash`。
- `crates/rhapsody/src/raw/shader.rs`（**见 §0 重建**）：删 `line_width_dynamic`，LINE_WIDTH 动态态取设备能力；`create_descriptor_set_states` 去参数；层数上限；`MissingSampler` Display；新增 `raster_opts_tests`（5 个）。
- `crates/rhapsody/src/raw/descriptor_set.rs`：`DESCRIPTOR_POOL_MAX_SETS/MAX_DESCRIPTOR_SETS/DescriptorSetState::for_layout`。
- `crates/rhapsody/src/raw/binding.rs`：`bind_persistent_descriptor_set_single`。
- `crates/rhapsody/src/raw/sampler.rs`：`BUILTIN_SAMPLER_NAMES`。
- `crates/rhapsody/crates/shader-pipeline/src/{shader_defines,pipeline_desc}.rs`：文档 + `with_entry` 空串→`cs_main` + 4 个测试。

## 7. 新增测试
- `crates/rhapsody/tests/raw_core_descriptors_shader.rs`（8 个：键形状 0x84/0x98、ArrayVec 满 panic、深度跳表、采样数选择、平铺转换语义、lavapipe 上 `from_desc` 建通道含颜色数/采样数、池/层数常量、采样器名与缺失文案）。
- `raw/shader.rs::raster_opts_tests`（5 个：逐位位置、跨字节字段往返、跳表枚举次序、`apply_to` 复制混合状态、顶点格式表）。
- `shader-pipeline`：defines 哈希数值复现 + 文件名（含发行 .bin 名）+ `cs_main` 默认。
验证：`cargo test --offline -p rhapsody -p tiny-glade -p country-core -p shader-pipeline --all-features`（lavapipe）全部通过。

## 8. 遗留与对他人的需求
1. `device_api.rs::create_render_pass` 改键为 `RenderPassDesc` 并走 `RenderPass::from_desc`（+0x188 缓存）。
2. `device.rs`：加 `descriptor_update_count`（+0xab0）；`create_samplers` 改为 9 个具名采样器 + 名字表（+0x680），并提供 `get_built_in_sampler(name)`，缺失 panic `Sampler not found: {name}`。
3. tiny-glade `render_host.rs` 的 `DescriptorSetState::new(layout, 64)` 改 `for_layout`（100）。
4. 未读到：内置采样器 CreateInfo 状态；`vkCreateRenderPass`/`vkCreateFramebuffer` 错误 panic 文案；`PassResourceSlot` 其余判别式与 `RenderPassBinding` 0x44B 布局；`RenderPassAttachmentDesc +0x0d..0x0f`；`RasterPipelineDesc` 的 `@0x30` 字符串语义；多宏 defines 折叠的数值样本。
