# M3 描述符布局协商与页式宿主合同（待 root/reviewer 审查）

本轮只读分析，未修改生产、未编译、未运行 GPU。保留 refactored BindlessHeap 单物理 set，不重新构建原版 rhapsody。前一轮按类别统计候选已由 root 整合，3 个 Slots 测试通过的结果属于 root；本轮没有自行复验。

## 当前门槛与来源

实际日志 `/private/tmp/tg-main-chain-validation-20261021-v5-gpu.log` 与 v6：原生 M3 Device 创建成功，ShaderHost 申请 24896 项布局遭支持查询拒绝；不能与沙箱内 ERROR_INCOMPATIBLE_DRIVER 混为一类。

当前 ShaderHost 定容 `render_host.rs:2329–2381` 为 [8192 采样图、256 存储图、16384 存储缓冲、64 sampler]，合计24896。算法只收敛 descriptor-indexing limits，不询问组合布局支持，因此在本机1e6/500k/1e9大限值下不收敛。注释所谓“Mac1212收敛”与实际算法不符。1212本身是 guaranteed 支持数，不能按 Vulkan 通则当物理硬上限。

root 已取得 `/private/tmp/tg-m3-vulkaninfo-20261021.txt`：maxBoundDescriptorSets=8；传统每 stage Buffer31/Texture256/Sampler16；update-after-bind图与buffer均1e6、sampler500k、全池1e9；maxPerSetDescriptors=1212。

本机 ICD `/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json` 指向 Homebrew MoltenVK1.4.2；dylib SHA256 `05df2d2145b9ef5b86b08ed93a7a6e916815c2bbac244298a50b2d58061600d1`。已读安装头 private API43，argument buffer配置为Bool32，env可被instance layer settings覆盖；现工程没有设置覆盖参数。正常二进制入口已在启动线程前置 env=1，integration-test入口没有调用该二进制main；root另设env=1的v6仍同错。

支持查询在当前 rhapsody `raw/descriptor_set.rs:169–210` 带实际 layout flags、binding flags；实际 supported=false 才拒绝，未发现当前 ash 链使用错误。没有删除/忽略该检查的理由。

## MoltenVK 查询规则的核实

[官方v1.4.2 MVKDevice.mm](https://raw.githubusercontent.com/KhronosGroup/MoltenVK/v1.4.2/MoltenVK/MoltenVK/GPUObjects/MVKDevice.mm) L3249–3254、L3626–3770：fixed路径把所有binding descriptorCount求和，与4倍传统Metal三类上限（至少1024）作严格小于比较。variable helper在后续输出pNext请求时只会填maxVariableDescriptorCount或进一步拒绝，不能把false改true；单独storage variable仍依据Metal buffer31。故argument buffer开关与VARIABLE都不能让当前24896布局通过该实现。

已用只读otool核已装dylib一致：`_vkGetDescriptorSetLayoutSupport` @0x194f8调用内部 @0x78d80；内部 @0x78f30..0x78f60取三类值、求和左移2、max1024、`cset lo`写 supported；@0x78f64..0x78fa4仅在输出pNext类型匹配时调用variable helper。此处核的是安装驱动，非 Tiny Glade 原作机器码。

当前 Pool::allocate (`raw/descriptor_set.rs:420–441`) 已按VARIABLE flag接 AllocateInfo，计数使用layout上界；尚无按调用方较小actual count分配入口。Layout::new当前未接variable-support输出pNext。即使补齐，也不能挽救本机该路径，先不把通用variable扩展当M3修复。

## 最窄可行闭包

保持 shader-compiler lowering 固定set0/binding0 sampled、1 storage image、2 storage buffer（含参数块）、3 sampler；保持ABI_VERSION1、一个push字、全部 shader 原索引与数组边界、原dispatch/draw数量及算法。大总量拆成多个相同布局的物理heap页；每个draw/dispatch完整绑定组只选择一个页，并在该页重建全部参数中的描述符索引。

每页仍是现 BindlessHeap，一个Slots/Pool/Set，不把它改成多set框架。宿主按实际支持查询选择单页配置，所有页共享同一个真实Arc<VulkanDescriptorSetLayout>：当前 `validate_binding` 要求handle精确相同，仅结构相同但不同handle不能用。建议最小rhapsody新增闭包是“从已验证heap创建共享layout的新页”，不放松现验证。各页独立描述符槽和完成回收；旧heap在大布局被真实支持的设备上继续单页。

单纯将总容量改成1211会在一帧约2700参数块处再次失败。分页必须与完整绑定组选择、资源重写、完成后复用一起交付。单纯按类型拆4个set也不够：storage buffer单表16384仍过该驱动查询；把大表平铺为十余静态set会超过本机8个set并改变lowering，暂不选。

## 消费者映射与必要修改范围

| 当前消费者 | 当前约束 | 页式必须保持的合同 |
|---|---|---|
| ShaderHost::new，render_host.rs:1005 | heap先于pipeline创建；适配配置可能被原生拒绝 | 只在真实Supported布局上建模板页；为后续页保存共享layout；不能吞OOM/deviceLost后盲目降容 |
| BindlessShaderPackage::new，bindless_parameters.rs:69–132 | pipeline layout使用heap.layout；SPIRV数组是runtime数组 | 模板页layout用于全部pipeline；后续页共享它；不改SPIRV set/binding/资源字节布局 |
| ShaderHost::bindings，render_host.rs:1124–1172 | sampler_leases按名字缓存到唯一heap | 保留actual Arc<Sampler>与逻辑location/element，租约在所选页外置记录缓存；禁止把强lease缓存放heap内部形成heap→lease→heap环 |
| ShaderBindings::parameters，render_host.rs:884 | 立即创建单heap参数并写sampler/global | 拆出完整逻辑输入快照；选定页之后才写所有lease/index；禁止混入模板页旧lease |
| FrameGlobals::write_to，frame_setup.rs:56–146 | OnceLock对单heap缓存FrameConstants和sampled | 同一immutable FrameGlobals在每个真实页准备对应leases；b0/b1共用declared buffer角色；保留原global texture逻辑索引，参数表中的物理index可按页变化 |
| raster_host::bindless_parameters，:2002–2111；draw loop :1601 | 已聚齐local/external/pass资源，随后finish和draw | 聚齐后算完整组需求并选页，连sampler/global/parameter块一起容纳；准备失败时中止当前frame，不能跳draw |
| compute pass闭包，render_host.rs:3022–3087 | resources/image_arrays明确；首次prepare，以后with_push复用原lease | 同页with_push仍快路径；满页时用闭包中的同份逻辑resources/image_arrays+global快照/samplers重建完整组，再执行原次dispatch |
| PreparedBindlessParameters::with_push，bindless_parameters.rs:414 | 直接复制旧数值index，依赖self.heap不变 | 严禁用它迁页；迁页必须重写全部index和新参数descriptor；前次参数chunk不可变 |
| bind_pass/bind_command，bindless.rs:663–815 | heap、pipeline layout、lease同一性与范围/角色验证 | 原验证保留；按所选物理页绑定一个set，push该页的parameter index；全部真实leases由command持有 |
| ProductionRenderer / session completion | GPU receipt完成后才release命令owner | 不变；分页不能CPU帧末reset/clear，也不能新增每帧wait_idle |
| TextureHost::create_persistent_descriptor_set，render_host.rs:499–540 | asset texture library本来有独立小heap+实际支持降容 | 不与shaderheap页混用；FrameGlobals继续从真实view转换到当前shader页 |
| egui、present、world passes | 最终都走同raster/compute消费者 | 同组路径自然覆盖；不能单独替换其着色器或降低绘制数 |

## 一页“完整绑定组”的容量需求

计数针对不同实际资源key，按descriptor kind分四个数：sampler实际对象；image view+layout；storage image view；buffer owner+offset+size+declared/private-upload角色。FrameConstants b0/b1只算一次，逻辑同资源数组可共享同lease，参数块额外StorageBuffer+1。写入全部index前必须确认fresh页可容纳该组；不能用ABI parameter_bytes/4替代真实descriptor数，runtime未绑定条目原先保留0xffffffff语义。

当前生产包 lowering runtime_array_limit=64（render_host load_compute :1229 / load_raster :1287）。用已保留404原shader反射逐包按set/binding合并、runtime=64、不对实际资源做去重的保守上界：最大完整组72（nani_solid_vertex_color_window_glass/text），分别全体逐类最大sampled66、storage image14、storage buffer13（已含参数块）、sampler6；逐类最大并集99。这个界只涵盖该retained catalog，不包含尚未比对的新生成shader或实际individual mip替换，绝不把72当全项目已证硬上限。生产准入必须从当前实际ABI及资源快照计算，超过fresh页应返回带shader名/ABI/逐类需求的明确Unsupported，不能丢资源或裁剪数组。

页配置要先满足该实际最大组的逐类需求，再通过原生支持query。矩阵中的[128,64,960,32]=1184仅是查询试验输入，不是未经审查的默认容量建议。总存活容量由多页及既有内存/descriptor全池预算管理，不靠增大单页或减少原绘制数。

## 分配与失败原子性

1. 先冻结实际group输入：ABI、push、sampler元数据、FrameGlobals Arc、local resources、individual mips、buffer上传角色和访问范围。声明的graph访问不变，先完成缺失/重复/越界与角色校验。
2. 只对Exhausted/确证当前页空间不足尝试换页。OOM、DeviceLost、InvalidUsage、Unsupported等原错误立即传播；不能把它们吞成下一个页。页候选重试有界；fresh页仍装不下即明确失败。
3. reserve必须覆盖该次完整组所有四类缺失lease和参数block descriptor，不能先只留parameter槽。未提交的失败builder Drop归还新unused leases；不得覆盖已有live/retiring槽。新全局/采样cache应成功准备后才发布，不以半成品污染下一draw。
4. DynamicConstants写入可能已发生，失败允许留未使用chunk空洞（后续正常复用），不声称可回滚host写入；chunk始终不可变、失败时不发draw/dispatch，不新增虚假graph访问或timeline。
5. 所有准备/校验完成后才bind实际页、push参数index并发原指令。前面已经提交的batch按既有PartialSubmission保护；失败只放弃未提交命令，不能回收已提交页槽。
6. 并行录制：利用当前回调局部cursor快速持有候选页；同页完整组准入要独占reservation/borrow，防止多个worker看同free随后部分成功。不可拿全局page pool锁跨shader编译、device.poll或DynamicConstants锁；锁顺序须明确，避免lease Drop取Slots锁时反向等待。每draw正常路径O(1)，仅满页时有界选页，不每draw扫描所有页。
7. 页池和FrameGlobals页缓存必须外置，heap不反向拥有它们。页指针作身份时，cache必须持有对应实际页owner避免地址复用误命中；不能只存裸usize猜generation。
8. page高水位由在途命令真实lease、group规模及已有CPU/device/update-after-bind budgets限定。完成后仅归还槽，不每帧清页；内存压力下可移除不用的page记录，但实际heap仍须被残存lease保护到GPU完成。禁止每帧wait_idle/周期clear。

## 本轮矩阵与下一验收

`LAYOUT_MATRIX_INPUTS.json`只描述真实查询输入，root为唯一执行owner。先核fixed below/at/above1212、生产24896、小页1184、storage单表16384、variable31/32/16384；必须同时记录supported和maxVariableDescriptorCount。这个矩阵不执行游戏或减少其资源需求。

root/reviewer审合同后才产页式源码候选。需要的后续验证至少包括：同page原ABI/像素不变；强制小页跨多页且draw/dispatch数量不变；compute多次push在中间换页保持原先参数不被改写；并行group争用；失败注入全组回滚与已提交PartialSubmission寿命；真实M3完整玩法帧、窗口present、长稳页数/live/retiring收敛。所有原生false继续拒绝。
