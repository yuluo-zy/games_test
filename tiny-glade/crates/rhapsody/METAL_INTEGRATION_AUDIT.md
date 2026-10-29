# Metal 集成调查与移动端铺垫

> 修订说明:本文为第一轮调查。以下结论已被 [METAL_DESIGN.md](METAL_DESIGN.md) 第 3 版修正:测试影响(第 1 节)以其第 3.3 节为准,"约 13 个 bindless 着色器"不成立(全部管线在运行时降级为 bindless,见其 F6),G-buffer 读者为 3 个 compute 加 4 个光照着色器(F14),移动端铺垫(第 4 节)以其 D13、D15 为准。冲突时以 METAL_DESIGN.md 为准。

本文是 [CROSS_BACKEND_API.md](CROSS_BACKEND_API.md) 第 7 节第 1 步(泄漏审计)的结果,外加测试影响分析和移动端铺垫清单。**本次没有修改任何测试和源码**,只新增了本文和 [METAL_AUDIT_leaks.tsv](METAL_AUDIT_leaks.tsv)。

数据来自对 `rhapsody`、`country-core`、`tiny-glade` 源码的脚本统计和读码。脚本用正则分类,结果是估计值,各节末尾标明了口径和未验证项。

## 1. 测试分析

### 1.1 数量与分类

按文件里的 `#[test]` 个数统计(行数不用:内联测试文件的行数包含非测试代码,会夸大)。分类规则:文件含设备创建关键字(`Device::new`、`RHAPSODY_REQUIRE_VULKAN`、`lavapipe`、`VulkanInstance`、`DeviceConfig` 等)记为"需真设备";否则文件含 `vk::`/`ash::` 记为"含 Vulkan 类型";否则为"纯逻辑"。

| crate | 纯逻辑 | 需真设备 | 含 Vulkan 类型 | 合计 |
| --- | ---: | ---: | ---: | ---: |
| rhapsody | 263(26%) | 337(33%) | 408(41%) | 约 1008 |
| country-core | 2890(91%) | 144(5%) | 136(4%) | 约 3170 |
| tiny-glade | 258(52%) | 225(46%) | 9(2%) | 约 492 |

rhapsody 测试分布:`tests/` 集成测试约 446 个(其中需真设备 262、含 Vulkan 类型 149、纯逻辑 35),`tests.rs`/`*_tests.rs` 文件约 506 个,示例与基准约 27 个。

### 1.2 对 Metal 工作的含义

| 类别 | 影响 | 说明 |
| --- | --- | --- |
| 纯逻辑(rhapsody 263 个,country-core 约 2890 个,tiny-glade 约 258 个) | 无影响 | 不碰 Vulkan,迁移期间应保持全绿,是回归基线 |
| 需真设备(rhapsody 337 个,tiny-glade 225 个,country-core 144 个) | Vulkan 路径继续用 lavapipe 跑;Metal 路径**没有等价环境** | 这批测试在 Linux 上靠 lavapipe 运行(`RHAPSODY_REQUIRE_VULKAN` 需要校验层,本机没有)。Metal 后端的同类测试只能在 Mac 上跑 |
| 含 Vulkan 类型(rhapsody 408 个,country-core 136 个) | **类型改中立后会编译失败** | 断言里直接构造 `vk::ImageUsageFlags`、`vk::Format`、`vk::AccessFlags` 等 |

最后一类集中在渲染图的屏障推导与校验(`sync_plan`、`plan_check`、`attachments`、`access_ledger`、`resource_contracts`)的 `tests.rs` 里,即最需要保持正确性的部分。

### 1.3 "先不改测试"的做法

要保证 1008 个 rhapsody 测试和 country-core 的 136 个含 Vulkan 类型测试**一行不动**还能编译,有一个前提:**现有 Vulkan 类型的 API 保持原样**。这决定了迁移方式:

| 方案 | 做法 | 测试 | 代价 |
| --- | --- | --- | --- |
| A. 增量叠加 | 现有 `rhapsody` 原封不动,继续作为 Vulkan 实现。新增中立 API 层(新 crate)放在它上面,Vulkan 后端是对现有 `rhapsody` 的适配;Metal 后端另写 | 现有测试全部不改,继续验证 Vulkan 实现 | 渲染图的屏障推导仍是 Vulkan 类型的,**Metal 后端不能直接复用它**,要么在适配层把推导结果翻成中立形式,要么 Metal 自己再做一遍推导 |
| B. 原地替换 | 把 `vk::*` 改成中立类型,移动文件到 `rhapsody-core` | 约 544 个含 Vulkan 类型的测试要重写 | 一次改动大,但只有一份屏障推导 |

### 1.4 核实结果:`sync_plan` 的输出

读了 `rg/sync_plan/mod.rs` 的公开输出类型,结论是:**输出已经按 Vulkan 语义解析,不是只含访问码,但保留了足够的中立信息。**

| 输出类型 | 含 Vulkan 类型的字段 | 中立字段 |
| --- | --- | --- |
| `RangeUse` | `scope: AccessScope`(vk 阶段+访问标志)、`layout: vk::ImageLayout` | `resource`、`range`、**`access_code: u8`** |
| `BoundTransition` | `source`/`destination: AccessScope`、`old_layout`/`new_layout: vk::ImageLayout`、`source_family`/`destination_family: u32`(队列族) | `batch`、`phase`、`resource`、`range`、`kind` |
| `BoundPatchAction::Access` | `scope`、`layout` | `queue: QueueId`、`family` |
| `BoundWait` | `stages: vk::PipelineStageFlags` | `batch`、`dependency` |

`sync_plan` 内部只用到 4 种 Vulkan 类型:`ImageLayout`、`PipelineStageFlags`、`ImageAspectFlags`、`QUEUE_FAMILY_IGNORED`(共 55 处引用,分布在 `mod.rs`、`bind/access.rs`、`bind/full.rs`、`bind/shadow.rs`、`bind/templates.rs`)。

含义:

1. **方案 A(增量叠加)对 Metal 不够**:Metal 不需要布局,但需要依赖边(哪个 pass 的哪块资源范围要等哪个 pass)和阶段(fence 的更新与等待要按阶段)。这些信息在 `BoundTransition` 里有(`batch`、`phase`、`resource`、`range`、`scope.stages`),但被包在 Vulkan 类型里,**Metal 后端需要消费同一份计划,就必须能读这些类型**。
2. **中立化的面很小**:只需替换 4 种类型。`ImageLayout` 换成 `ImageState`(但计划里用到了完整的 Vulkan 布局集合,例如 `PREINITIALIZED`、深度/模板只读变体,`mod.rs:460-495` 有 `depth_stencil_intent` 一类的逻辑,映射表要覆盖这些);`PipelineStageFlags` 换成 `StageMask`。
3. **`CROSS_BACKEND_API.md` 里的 `StageMask` 只有 8 位,偏粗**:Vulkan 的访问码还含曲面细分、几何、光线追踪阶段(码 0x08–0x10、0x2a–0x2c、0x38–0x3f)。Metal 没有曲面细分和几何着色器,可折叠为顶点或"任意";本项目是否用到这些码没有核对。
4. 所以更合理的做法是 **B 的收窄版**:只对 `sync_plan`、`plan_check`、`attachments`、`access_ledger` 做这 4 种类型的中立化(这部分的测试会被触及,约属于 544 个含 Vulkan 类型测试的一部分),其余模块保持原样、测试不动。具体哪些测试被触及,需要在动手前按这 4 种类型再统计一遍。

### 1.5 口径与限制

- `#[test]` 个数只数了属性本身,没有统计 `proptest`/`rstest` 等其他形式。
- "需真设备"靠关键字判断,可能漏判或多判;"含 Vulkan 类型"文件也可能同时用设备,按优先级归入"需真设备"。
- 没有运行任何测试。

## 2. 集成 Metal 要重写什么

### 2.1 rhapsody 内的 Vulkan 概念分布

非测试文件中共 2206 处 `vk::` 引用,按概念分桶(同一引用只归一个桶,顺序:命令、同步、内存、描述符、管线、附件、WSI、查询、图像缓冲格式、设备):

| 概念 | 引用数 | 性质 |
| --- | ---: | --- |
| 图像/缓冲/格式 | 717 | 数据:格式表(327,集中在 `format`)、视图、范围、宽高;可替换为中立类型 |
| 同步/屏障/布局 | 544 | 数据加少量逻辑:布局(169)、阶段(148)、访问(93)、信号量与围栏(各 40) |
| 设备/实例/特性 | 211 | 后端内部 |
| 描述符/绑定 | 155 | 后端内部 |
| 内存/分配 | 140 | 后端内部 |
| 命令录制 | 87 | 后端内部(命令用类型化枚举,数量少) |
| 交换链/WSI/表面 | 55 | 后端内部 |
| 管线/着色器 | 44 | 后端内部 |
| 查询/调试/性能 | 39 | 后端内部 |
| 渲染通道/附件 | 31 | 数据 |
| 其他 | 183 | 未归类 |

读法:一半以上(717 + 544 + 31)是**数据**,可以靠中立类型加降级表处理;控制流层面真正依赖 Vulkan 的部分(命令、描述符、内存、管线、WSI)引用数不多,但对应的代码量大(见下表)。

### 2.2 使用的 Vulkan 扩展

| 扩展 | 用途 | Metal 侧对应概念(我的经验判断,**没有对照 Metal 文档逐项核实**) |
| --- | --- | --- |
| `VK_KHR_dynamic_rendering` | 渲染通道 | `MTLRenderPassDescriptor` |
| `VK_KHR_synchronization2` | 屏障 | 无直接对应;用 fence/event/hazard 跟踪 |
| 时间线信号量(1.2 核心) | 帧完成与跨队列 | `MTLSharedEvent`(带值事件) |
| `VK_KHR_swapchain`、`surface`、`get_surface_capabilities2`、`swapchain_maintenance1`、`present_id/wait` | 呈现 | `CAMetalLayer` 与 drawable,呈现节奏由 `presentDrawable` 与 `presentedHandler` 控制 |
| `VK_EXT_full_screen_exclusive` | Windows 独占全屏 | 无需对应 |
| `VK_EXT_memory_budget` | 显存预算 | `recommendedMaxWorkingSetSize`、`currentAllocatedSize` |
| `VK_EXT_debug_utils`、校验层 | 调试 | Metal 调试层、GPU 捕获标签 |
| `VK_KHR_portability_enumeration/subset` | MoltenVK | 不适用 |
| 可选:`descriptor_buffer`、`graphics_pipeline_library`、`shader_object`、`mesh_shader` | 探测但不要求 | 不需要对应 |
| `shader_demote_to_helper_invocation` | 必需 | MSL 的 `discard_fragment` 语义不同,**需核对着色器是否依赖该行为** |

### 2.3 逐项重写清单

行数来自前文统计(非测试)。"可复用部分"指该模块里零引用的行数。

| 编号 | 重写项 | 现状 | Metal 侧做法 | 可复用部分 | 难度判断 |
| --- | --- | --- | --- | --- | --- |
| R1 | 设备、实例、物理设备选择 | `native` 2772 行,`device` 5643 行 | `MTLDevice` 枚举,`MTLCommandQueue`;没有队列族概念 | 选择策略、驱动问题表(907 行零引用) | 中 |
| R2 | 内存分配 | `device/allocator`、`suballoc`;依赖仅启用 vulkan 特性的 `gpu-allocator 0.27` | `MTLHeap`、`MTLBuffer` 存储模式;统一内存下无显存/主存之分 | 统计与预算(`accounting`) | 中 |
| R3 | 资源创建与视图 | `resource` 4145 行 | `MTLTexture`、`MTLBuffer`、纹理视图 | 描述校验、命名、托管句柄(853 行) | 中 |
| R4 | 命令录制 | `command` 5570 行;类型化 `Command` 枚举 | `MTLCommandBuffer` 加 render/compute/blit 编码器;并行录制用 parallel render encoder | 枚举形状、并行分块策略 | 中 |
| R5 | **同步** | `sync_plan` 屏障推导;`tracking`、`sync`;约 544 处引用 | fence、`MTLSharedEvent`、hazard 跟踪;屏障语义不同 | 访问账本、队列调度(`access_ledger`、`queue_schedule`,接近零引用) | **高**(正确性风险最大:写错只会偶发花屏) |
| R6 | 描述符与 bindless | `bindless` 1960 行,`command/binding` | argument buffer;上限与层级因设备而异 | 槽位管理、租约、分页(707 行) | 中高 |
| R7 | 管线与着色器 | `pipeline` 4259 行;依赖 SPIR-V | `MTLRenderPipelineState`、`MTLComputePipelineState`;SPIR-V 转 MSL;binary archive 缓存 | 反射、请求队列、缓存键(1226 行);`shader-pipeline`、`shader-compiler` 全部 | 中高(取决于 415 个着色器的转译成功率,**未测**) |
| R8 | 呈现 | `present` 3059 行 | drawable 获取与呈现 | 呈现票据等中立类型 | 中 |
| R9 | 传输与读回 | `transfer` 4077 行 | blit 编码器;统一内存下读回可直接读 | 上传队列、票据(1112 行) | 低中 |
| R10 | 诊断 | `rg/profiling` | 时间戳用 counter sample buffer;**没有与 pipeline statistics 等价的查询**(我的判断,未核对) | `GpuTimingConfig` 形状 | 中:我们新加的"每 pass 片元/图元计数"在 Metal 上可能做不到 |
| R11 | 宿主层 | `tiny-glade` 的 `render_host.rs`(6460 行)、`raster_host.rs`(4614 行)等,共约 15k 行 | 按新接口重接线 | 大部分逻辑 | 中(见第 3 节) |
| R12 | 渲染图的访问降级 | `native/access_table`(64 个访问码到 Vulkan 阶段/访问/布局) | 新增 Metal 降级表 | 访问码本身 | 高(与 R5 同一件事) |

可原样保留的:`rg` 分组 A(约 1.38 万行)、`collections`、`retire`、`log_ring`、`profile`、`error`、`accounting` 大部分,以及游戏侧 `country-core` 的 pass 声明层。

### 2.4 口径与限制

- Metal 侧对应概念是我基于经验的判断,没有对着 Metal 文档逐项核对;动手前每一行都要核。
- 难度判断是定性的,不是工作量估算。

## 3. 宿主层与游戏侧泄漏审计

`country-core` 和 `tiny-glade` 的 `src/` 里共 **260 处** `vk::` 引用(不含注释行)。逐条清单见 [METAL_AUDIT_leaks.tsv](METAL_AUDIT_leaks.tsv)(文件、行号、类别、符号)。

### 3.1 按类别

| 类别 | 数量 | 替换方案 | 性质 |
| --- | ---: | --- | --- |
| 用法标志(usage) | 64 | `BufferUsage`、`TextureUsage`(数值与 Vulkan 位相同) | 机械 |
| 格式/范围/视图 | 48 | `Format`、中立范围类型 | 机械 |
| 同步/布局/队列 | 47 | 见 3.3 | **需审计** |
| 句柄/地址/原生对象 | 30 | `BufferId`、`device_address()` | 半机械 |
| 命令缓冲/原生录制 | 26 | `CommandSink` 与扩充 `Command` | **需设计** |
| 设备/驱动/属性 | 15 | `DeviceCaps` | 半机械 |
| 描述符 | 13 | `ResourceTable` | 需设计 |
| 表面/WSI | 4 | `SurfaceSource` | 机械 |
| 附件/渲染 | 1 | `RenderingDesc` | 机械 |
| 其他 | 12 | 逐条看 | — |

### 3.2 按文件

| 文件 | 引用数 | 主要类别 |
| --- | ---: | --- |
| `tiny-glade/src/render_host.rs` | 158 | usage 43、同步/布局 33、格式 23 |
| `tiny-glade/src/raster_host.rs` | 32 | 格式 9、句柄 8、usage 6 |
| `tiny-glade/src/screenshot_capture.rs` | 14 | 同步/布局 9 |
| `tiny-glade/src/production_renderer.rs` | 12 | 表面 4、句柄 4 |
| `tiny-glade/src/mei_host.rs` | 7 | — |
| `country-core/src/rhapsody_contract.rs` | 6 | 命令缓冲 3、句柄 2、属性 1 |
| `tiny-glade/src/surface_comparison.rs` | 6 | 格式 4 |
| 其余 | 25 | — |

集中度很高:**两个宿主文件占 73%**(190/260),`country-core` 只有 11 处。

### 3.3 需要审计的三处

1. **同步/布局(47 处,主要在 `render_host.rs` 33、`screenshot_capture.rs` 9)**:宿主层自己在做图像布局转换和屏障,说明部分上传和读回**绕过了渲染图**。要么把它们纳入图声明/传输 API,要么在后端内部实现。我没有读这些调用,**不知道有多少能被图声明取代**。
2. **命令缓冲/原生录制(26 处)**:`render_host` 里有 `vk::CommandBuffer`(6)、`BufferImageCopy`(5)、`BufferCopy`(4)。现有 `Command` 枚举没有图像拷贝,需要补。`rhapsody_contract.rs` 里 `issue_full_pipeline_barrier` 等 3 个方法要找出全部调用点。
3. **Vulkan 味道的 rhapsody 接口**(宿主层使用的次数):`native::is_write`(7)、`device::Feature`(6)、`rg::VulkanGraphSession`(4)、`native::VulkanInstance`(4)、`native::surface::Surface`(4)、`command::SynchronizationScope`(3)、`raw::workarounds::GraphicsWorkarounds`(3)、`rg::CommandBackend`(2)、`device::semaphore::TimelineSemaphore`(2),其余各 1 次:`AshCommandBackend`、`VulkanCommandBuffer`、`VulkanCommandPool`、`VulkanGraphBackend`、`InstanceBuilder`、`GraphicsInstanceLoader`、`GraphicsInstanceLayerAllowlist`、`DescriptorSetBinding`、`TextureUpdateRegion`、`PresentTicket`、`AcquiredImageLease`、`enumerate_physical_devices`。

## 4. 移动端铺垫

> 修订说明:审查发现 G-buffer 还被屏幕阴影与 SSGI 上采样两个 compute 读取、深度在光照 pass 中按其他像素采样,"G-buffer 留在 tile 内"的前提不成立。移动端以降低内部分辨率和砍全分辨率 compute 为主手段,G-buffer 合并推迟,详见 [METAL_DESIGN.md](METAL_DESIGN.md) D13、D15。第 5 节的顺序以该文第 4 节为准。

### 4.1 已有的基础

| 基础 | 位置 | 说明 |
| --- | --- | --- |
| 显存预算与内存不足自动降分辨率 | `country-core/src/render/adaptive_render_state.rs`、`tiny-glade/src/vram.rs`、`frame_resolution.rs` | 内存不足时缩小世界分辨率并按 250ms 起退避重试 |
| 上采样与内部分辨率 | TGSR(内部分辨率 0.33–0.77)、`world_resolution_scale` | 已按质量预设切换 |
| 质量预设 | `tiny-glade/src/gui/settings.rs` | Performance/Balanced/Quality/UltraQuality |
| 附件 DONT_CARE 载入 | `raster_host.rs:2560` | 载入操作支持 LOAD/CLEAR/DONT_CARE |
| 着色器管线中立 | `shader-pipeline`、`shader-compiler` | 只依赖 SPIR-V 工具库 |
| 能力与特性集 | `device::caps` | 已有 `FeatureSet` |
| 帧 pass 组合的缝 | `country-core/.../render_world.rs:219` 的 `FramePasses` trait | 备选的"渲染配置档"切入点,**我没有读它的内容** |
| 压缩纹理格式表 | `rhapsody/src/format/registry.rs` | 含 ETC2、ASTC 的 Vulkan 格式条目 |

### 4.2 缺口(均经 grep 或读码确认)

| 缺口 | 证据 |
| --- | --- |
| **附件存储操作恒为 STORE** | `raster_host.rs:1086` 写死 `store_op: 0`;`attachments/contract.rs:70-71` 的模板化键里 stencil 恒为 LOAD/STORE。没有"本 pass 后丢弃"的表达,深度和中间 G-buffer 都会写回主存 |
| 没有瞬态/无内存/惰性分配附件 | 全仓库搜索 `TRANSIENT_ATTACHMENT`、`LAZILY`、`memoryless` 无命中 |
| 没有 pass 合并 | 渲染图以单个 pass 为单位,没有 subpass 或 tile 序列概念(`ARCHITECTURE.md` 也这么说明) |
| 没有设备档位 | 没有 `DeviceProfile`;只有 `adaptive_bindless_config` 缩小表,没有"每 pass 小表"回退 |
| 没有 ASTC 资产路径 | `asset-pipe` 里只有 BC 路径(`dds.rs`),搜索 ASTC 无命中 |
| 内存压力输入只有"分配失败" | `AdaptiveRenderState` 在 OOM 时触发,没有系统内存警告或热状态输入 |
| 表面只有 Win32、AppKit 和 Linux | iOS 的 UIView 没有 |
| 渲染图的 Vulkan 依赖布局 | `attachments` 里 `RenderPassAttachmentDesc` 的格式字段是 `vk::Format`,深度判断用 `(124..=130).contains(format.as_raw())` 这类 Vulkan 数值 |

### 4.3 铺垫项(按"不改变现有画面和行为"排序)

全部满足:**默认值与现状相同**,Windows 路径像素和性能不变,现有测试不改。

| 编号 | 铺垫 | 内容 | 对现有行为的影响 |
| --- | --- | --- | --- |
| G1 | 存储操作可表达 | 在中立附件描述里加 `StoreOp::DontCare`,默认仍为 STORE | 无 |
| G2 | pass 合并与瞬态声明 | 附件描述加 `transient` 提示,pass 声明加 `merge_group`;Vulkan 后端先忽略 | 无 |
| G3 | 能力字段 | `DeviceCaps` 增加资源表档位、统一内存、tile 内存、间接绘制模式,由 Vulkan 后端按现有探测填充 | 无 |
| G4 | 宿主层手工同步收口 | 把 `render_host`、`screenshot_capture` 里的手工布局转换与拷贝,迁入图声明或传输 API(第 3.3 节第 1、2 点) | 需像素对比验证 |
| G5 | 渲染配置档接口 | 研究 `FramePasses` 是否能承载"桌面/移动"两套 pass 组合 | 无(只读调研) |
| G6 | ASTC 资产路径 | 资产管线增加 ASTC 输出,格式表已有条目 | 无(新增路径) |
| G7 | 内存压力输入 | `AdaptiveRenderState` 增加系统内存警告输入(iOS 内存警告)的抽象 | 无(新增输入) |
| G8 | 诊断中立化 | `GpuTimingConfig` 的 pass 统计按能力可选,不支持的后端返回 `None` | 无 |

G1–G3 与 G8 是类型和默认值的改动,不需要 Metal 就能做,也是中立 API 的组成部分。G4 风险最大,需要现有像素对比和性能夹具来验证。

## 5. 建议顺序

1. ~~核实 `sync_plan` 的输出类型~~ **已完成**(第 1.4 节):输出含 Vulkan 类型,走"收窄版 B":先对 `sync_plan`、`plan_check`、`attachments`、`access_ledger` 中立化 `ImageLayout`、`PipelineStageFlags`、`ImageAspectFlags`、队列族这 4 种类型,并先统计这 4 种类型触及的测试。
2. **宿主层审计的三处需设计项**(第 3.3 节):读 `render_host.rs`、`screenshot_capture.rs` 的同步与录制调用,确认哪些能纳入图声明。
3. 在中立 API 里先落 G1–G3、G8(默认值不变)。
4. G4 单独做,配合像素和性能验证。
5. Metal 后端从 R1、R2、R3、R4 起步(清屏、三角形、compute、读回),R5(同步)单独设计评审。

## 6. 已验证与未验证

**已验证**
- 测试分类、`vk::` 概念分桶、扩展清单、宿主层泄漏清单由脚本生成,数字可复现。
- `store_op` 恒为 STORE、没有瞬态附件、没有 ASTC 资产路径:由 grep 与读码确认。
- 本次没有修改测试和源码。

**未验证**
- 没有运行任何测试。
- 4 种类型的中立化会触及哪些具体测试(只按文件类型估过,没有按类型统计)。
- 本项目实际用到哪些访问码,尤其曲面细分、几何和光线追踪相关的码。
- Metal 侧每个对应概念;pipeline statistics 在 Metal 上是否有等价物。
- 宿主层 47 处同步引用中有多少能被图声明取代。
- `FramePasses` 能否承载多套 pass 组合。
- 着色器 SPIR-V 到 MSL 的转译成功率。
