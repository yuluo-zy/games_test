# wave2：内置采样器 / 描述符更新计数 / RenderPassBinding 0x44B / 创建失败文案

工具：`/tmp/dis.py`（oracle `tiny-glade.rne`）、`/tmp/scan*.py`、`/tmp/samp.py`（对 0x140bf5208..0x140bf5823 的只读模拟，不在仓库）。

## 1. 内置采样器 ✅ 原版对齐
- 为什么上轮找不到：名字池里是 **13** 个名字（9 个反射名 + 4 个 `_iso`），池地址 0x142b16b42..0x142b16bfd，顺序即创建顺序。13 个引用全部落在 `GraphicsDevice::create`（0x140bf2560）内的 0x140bf5209..0x140bf57ab。
- 构造：堆上 13 个 0x60B 元素 `(name:&str @+0/+8, VkSamplerCreateInfo @+0x10)`（sType=0x1f），经 `0x1418c2970`（HashMap::extend）→ `0x142135f90` 逐元素调用 `vkCreateSampler`：`call r13`，`r13=[table+0x1a8]`（ash 0.38 `DeviceFnV1_0::create_sampler` 偏移），调用点 **0x14213604d**，allocator=null。上轮按 `[reg+0x220]`（设备对象视角 +0x78）找 `call`，实际是 "先 mov 取指针再 call r13" 形态才漏掉。
- 失败路径：0x1421360a7 → `unwrap_failed`（0x1428d9760），消息 `"create_sampler"`（len 0xe，0x142fe8968），位置 `crates/rhapsody/src/raw/sampler.rs:191:66`（Location 0x142fe8998）——即 `.expect("create_sampler")`。缺失查找 panic 在 sampler.rs:201（上轮已证）。
- 状态（O，机器码常量；`flags=0、mipLodBias=0、unnormalized=0、pNext=null` 来自序言清零的栈模板，U=V=W）：

| # | 名字 | mag=min | mip | address | aniso(on/max) | cmp | maxLod | border |
|---|---|---|---|---|---|---|---|---|
|0|lnc|LINEAR|NEAREST|CLAMP_EDGE|1/4.0|0|1000|0|
|1|llr|LINEAR|LINEAR|REPEAT|1/4.0|0|1000|0|
|2|nnc|NEAREST|NEAREST|CLAMP_EDGE|**0/4.0**|0|1000|0|
|3|llc|LINEAR|LINEAR|CLAMP_EDGE|1/4.0|0|1000|0|
|4|llb|LINEAR|LINEAR|CLAMP_BORDER|1/4.0|0|1000|2(FLOAT_OPAQUE_BLACK)|
|5-8|lnc/llr/llc/llb `_iso`|同 0,1,3,4|同|同|0/1.0|0|1000|同|
|9|default|LINEAR|NEAREST|CLAMP_EDGE|0/1.0|0|**0**|0|
|10|mipmap|LINEAR|LINEAR|REPEAT|0/1.0|0|1000|0|
|11|shadow_nearest|NEAREST|NEAREST|CLAMP_BORDER|0/1.0|**0**|0|4(FLOAT_OPAQUE_WHITE)|
|12|shadow_linear|LINEAR|NEAREST|CLAMP_BORDER|0/1.0|1, LESS_OR_EQUAL(3)|0|4|

  注意（以机器码为准）：nnc 关闭各向异性但 maxAnisotropy 仍是 4.0；shadow_nearest 的 compareEnable 为 0。名字字母含义（l/n、c/r/b、iso）属推断（与数值吻合）。
- 落码：`sampler.rs` 新增 `BuiltinSamplerSpec`/`BUILTIN_SAMPLER_SPECS`（13 项，创建顺序）/`builtin_sampler_spec(name)`/`CREATE_SAMPLER_EXPECT_MSG`；`BUILTIN_SAMPLER_NAMES` 仍为 9 个反射名（既有测试断言 9）。`device.rs::create_samplers` 改为按规格创建 13 个；`get_built_in_sampler(index)` 保持兼容（下标=创建顺序），新增 `try_get_built_in_sampler_by_name` 与 `get_built_in_sampler_by_name`（缺失 `panic!("Sampler not found: {name}")`）。
- ⚠️ 未接线：`GraphicsDevice::finish_creation` 并不调用 `create_samplers`（structure agent 的文件）；原版是在 create 内创建。请其在创建尾段调用。另：原版启用各向异性不检查设备特性，Rust 版同样不检查（lavapipe 通过）。
- ⚠️ 原版把结果放进 `HashMap<name,Sampler>`（+0x680）；Rust 保留 `Vec`+静态名表，语义等价、迭代顺序不同。

## 2. `device+0xab0` ✅（写端）/ ❌（读端）
- 写：4 处 `lock inc qword [dev+0xab0]`：0x1416a26aa（update_persistent_descriptor_set）、0x1416a56e5 / 0x1416a5ce8 / 0x1416a63f2（三个 bind_descriptor_set 单态，紧跟 `vkUpdateDescriptorSets` 调用之后）；初始化 0：0x140bf631d。
- 读：全 .text 扫描（`mov r,[r+0xab0]` / `xchg` / `lock xadd`）在 rhapsody 地址范围内没有读取者（其余命中是无关结构的同偏移字段，已抽查 0x140d93330/0x140d93450）。语义按纯统计累计计数处理，**不清零**；不称其为 RenderStats 字段。
- 落码：`VulkanDevice.descriptor_update_count: AtomicU64`（device.rs 三处 Edit：字段、构造初值，Drop 不动；`raw/binding.rs::DescriptorSetBinding::write` 每次 vkUpdateDescriptorSets 后 +1）。⚠️ 原版是批量写入一次 +1；Rust 的 `VulkanDescriptorSet::update` 仍是单写，计数按调用数累加，批量 `update_batch` 未做。

## 3. RenderPassBinding 0x44B ✅（布局）/ ⚠️（语义名）
`rg_binding_to_descriptor_binding`（0x14205eff0）对槽 `r9` 的全部访问读完。槽 = `RenderPassBinding`，大小 **0x44**（"0x44B"即 0x44 字节）：
- `[0..0x3c)` = `TextureViewDesc`，首字段 `u32 @0` 同时是**槽判别式**；`@0x20` u8 bit0=显式 mip 数 Option 标志，`@0x24` u32 显式 mip 数，`@0x30` u32 base mip，`@0x38` u8 bit0=带子范围，`@0x39` u16 / `@0x3b` u8 原样拷贝。
- `@0x3c` u32 资源号（`ResourceRegistry` 间接表索引）；`@0x40` u8 写标志（0→输出 tag 0/2，1→tag 1/3）；0x41..0x44 填充。
- 判别式（跳表 0x142fcf75c）：0/1/2 视图槽（disc==2 或 `+0x38` bit0==0 走整视图路径 0x14205f2de；否则子范围路径 0x14205f198，输出 tag `(write|2)`）；**3**→整缓冲 tag 6（0x14205f18d，资源号在 `+4`，offset=0 range=~0）；**4**→整缓冲 tag 5（0x14205f0e9）；**5**→`todo!()`（0x14205f4c0，"not yet implemented"，`rg/binding.rs:134:57`）。tag 5/6 的 uniform/storage 归属是推断（tag 6 = `into_uniform_buffer_binding` 同 tag）。
- 资源表项（stride 0x40）：`u64 kind@0`（缓冲 ∈{2,3}，否则 `unreachable!()` `rg/resource_registry.rs:100:59`；图像 <2）、`u16 device@0x1c`==`dev+0xa50`、`u8 @0x1e`（缓冲=1/图像=0）、代数 `@0x18`==`[ptr+0xc]`；缓冲 vk 句柄 `[ptr+0x30]`、图像 mip 数 u8 `[ptr+0x2c]`。失败文案 `managed/resources.rs:318/319`（不归我）。
- 子范围 mip 校验（0x14205f263..0x14205f2b0）：`avail = image_mips(u8) - base(低8位)`，借位 panic（0x14205f275）；显式 n：`n >= avail`（`jae`）panic `"The requested mip count ({n}) is larger than the image mip count ({image}) minus base level ({base})"`（片段 0x142fcf520，位置 `rg/binding.rs:54:25`）；`end=n+base`，无显式则 `end=image_mips`。⚠️ 机器码是 `>=`（与文案 "larger than" 不一致），按机器码实现。
- 落码（`rg/binding.rs`）：`TextureViewDescRaw`(0x3c)、`RenderPassBindingRaw`(0x44)+偏移测试、`pass_slot_disc`、`TODO_SLOT_MESSAGE`、`MipRangeError`+`resolve_view_mip_range`+测试。⚠️ 现有 4 变体 `PassResourceSlot`（带 offset/size）未改——它的缓冲 offset/size 在原版不存在（整缓冲），属不同抽象，改动会波及 `pass_bindings.rs`（非我的文件范围），留给集成者。

## 4. RasterPipelineDesc@0x30 字符串 / create 失败文案
- ✅ **vkCreateRenderPass**：0x140bfc435 → `Result::unwrap()` panic，`render_pass.rs:124:18`（Location 0x142b16b08）。落为 `render_pass::CREATE_RENDER_PASS_FAILURE_LOCATION` 常量（Rust 仍以 Result 返回）。
- ✅ **vkCreateFramebuffer**（0x140bfbef0，调用 0x140bfbf98 `[dev+0x270]`，sType 0x25）：失败**不 panic**，返回 Err 变体（tag 9，+0x20 存 vk::Result），成功 tag 0xb；与 Rust `Result` 一致。另两个 panic：`"Must have at least one framebuffer attachment"`（`framebuffer.rs:45:25`，0x1416f776f..）和 `assert_eq!(.., "Framebuffer attachments must all have the same size")`（`framebuffer.rs:35:21`，0x1418c38aa..）。落码 `framebuffer::{NO_ATTACHMENT_MESSAGE,SIZE_MISMATCH_MESSAGE,extent_from_attachment_sizes}` + 测试（⚠️ 未接入 `from_desc`：Rust 的 extent 由调用方给出）。
- ❌ **raster/compute 管线创建 Err 分支文案**：`vkCreateComputePipelines`/`vkCreateGraphicsPipelines` 同样走"取指针再 call"形态（`[table+0x188/0x180]`），该偏移的候选命中太多，未能在预算内定位；shader.rs 的 Location 表（0x142b136c8.. / 0x142d06298..0x142d06380 / 0x142b13998..）已列出但未逐一对应到"创建"操作。原版证据未读出。
- ❌ **RasterPipelineDesc@0x30 字符串语义**：未读（时间）。

## 5. 其余未闭合项
- ❌ `RenderPassAttachmentDesc +0x0d..0x0f`、多宏 defines 折叠样本：本轮未触及。
- ALGORITHMS §2 "sampler ... 3 采样器（O 部分）"、get_built_in_sampler "按索引 I" 现应改为：13 具名、按名 HashMap（O）——文档未改（非本任务范围）。

## 改动文件
`raw/sampler.rs`、`raw/binding.rs`（write 内计数）、`raw/framebuffer.rs`、`raw/render_pass.rs`（常量）、`rg/binding.rs`、`raw/device.rs`（仅 builtin_samplers/create_samplers/get_built_in_sampler*、`descriptor_update_count` 字段+初值；注：一次编辑用脚本读改写整文件，随后他人改动未丢——事后 diff 核对仍是只多了这几处）、新增 `tests/wave2_samplers_binding.rs`（6 项）。内联新增单测：`rg/binding.rs::raw_layout_tests`(3)、`framebuffer.rs::original_panic_tests`(4)。

## 验证
`cargo test --workspace --all-features --no-fail-fast`：rhapsody、country-core、shader-pipeline 全绿（含 raw_core_descriptors_shader、original_oracle_descriptors）。失败项均为缺少发行文件（asset、asset-pipe、shader-compiler `original_contract`、tg-text 的 NotFound/shipped 数据），与本改动无关。

## 读过的函数地址
0x140bf2560（含 0x140bf5208..0x140bf5964）、0x1418c2970、0x142135f90、0x1421360a7、0x140bfc5b0（上轮）、0x1416a2320/0x1416a26aa、0x1416a5380、0x1416a5910、0x1416a6010（+0xab0 点）、0x140bf631d、0x14205eff0（全）、0x140bfbef0、0x140bfc0d0、0x1416f6e90（局部）、0x1418c36e0（局部）、0x140c08f10（误命中：实例创建）、0x140d93330/0x140d93450（无关）。
