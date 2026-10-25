# 从底层逐层恢复 Tiny Glade 渲染器：可行性分析

2026-10-12。用户要求分析分层逆向计划的可行性；本文是基于当前构建证据的研究与实施建议，不是新一批实现完成记录。

## 判断

可行，而且适合作为恢复私有渲染框架的主线。底层接口和生命周期经过验证后，可以显著减少上层反编译中的未知指针、临时对象、同步状态和资源类型；上层pass的GPU算法仍有独立难度，不会因底层完成自动恢复。

最底层需要复用Vulkan/ash、公开分配器和同步库的匹配源码。逆向的起点是rhapsody在其上建立的私有封装。Cargo中的local/path条目先确认来源，不能全部当成作者从零实现。尤其vk-sync-fork 0.4.0和gpu-allocator 0.26.0在原锁文件没有source；可确认有公开库家族，但本构建具体来源与补丁范围尚未确立。

实际工作应按“底层实现与验收为主，上层调用证据辅助辨认接口”推进。查上层调用者的输入、返回值使用和失败路径，并不要求提前实现上层。原编译器还会把游戏构图闭包内联到RenderGraphState::compile的单态化实例，不能按一个大机器码函数归属就把整段内容都当成底层原源码。

## 原作分层依据

当前限定PDB源文件索引记录rhapsody 55个源路径：raw直接模块29、raw/managed 9、rg 15、lib/leek 2。另有country-core/src/render 38个路径。这个93文件索引没有覆盖所有游戏pass/shader，也不覆盖优化掉或未编译的文件；不是完整源码清单或完成率分母。

原作真实目录包括：

- raw：format、buffer、texture、sampler、binding、barrier、instance、init、physical_device、device、allocator、shader、descriptor_set、render_pass、framebuffer、command_buffer、semaphore、submit、disposal、dynamic_constants、swapchain等。
- raw/managed：handle、resources、buffer、texture、sampler、descriptor_set、render_pass、framebuffer、swapchain。
- rg：resource、recorder、pass_builder、pass_api、binding、graph、resource_info、resource_registry、resource_pool、compiled、executor、import_export、state、temporal、util。
- 游戏侧：pass_builder、GpuWorld、MeshAtlas、typed/shadowed/sparse缓冲等；具体systems/render_loop及shader算法需要单独扩充范围。

这些是源码组织证据。下述七个工作层是恢复时采用的划分，不声称原作者实现了严格单向、无环的七层API。

## 建议恢复与验收顺序

| 层 | 工作范围 | 完成这一层可提供什么 | 主要关口 |
|---|---|---|---|
| L0 原依赖 | 原锁定ash/ash-window；allocator/vk-sync与shader辅助包来源核实 | 可直接使用的API与明确的依赖边界 | 版本/source/checksum或git/path来源；必要fork差异清单，未知仍标记 |
| L1 基础类型与纯规则 | BufferDesc、TextureDesc、TextureViewDesc、format、binding、barrier描述 | 稳定的资源描述、用途、范围、格式及同步参数 | 字段读写/默认值/边界有原码依据；Rust编译和机器码差分 |
| L2 设备上下文与资源创建 | instance/init/physical_device必要链、device、allocator私有封装、buffer/image/view/sampler创建 | 能在底层创建和绑定实际资源 | 创建参数/扩展/features/error清理符合原路径；API参数追踪，必要时最小GPU资源测试 |
| L3 所有权、提交与回收 | managed句柄/资源表、command_buffer、semaphore、submit、disposal、dynamic_constants | 明确何时资源可读、可复用、可销毁 | 句柄有效性、提交序列、等待/信号、在途资源、完成后回收；状态轨迹与最小复制回读测试 |
| L4 Shader与Pipeline | shader/descriptor_set/render_pass/framebuffer、shader-pipeline、compiled SPIR-V加载 | 创建并绑定compute/raster pipeline | 变体/入口/stage/反射数据与实际绑定核实；标准SPIR-V与公开API复用；输出与读取布局一致 |
| L5 完整渲染图 | recorder/resource_info、registry/pool、compiled/executor、import/export、temporal | 上层声明pass与资源即可正确录制/执行 | 小图CPU计划差分、同步屏障与资源复用、图执行、跨帧history，异常/清理路径明确 |
| L6 游戏GPU场景与pass | country_core实例、mesh atlas、draw/compute builder、场景上传和具体pass | 真实游戏数据到GPU工作贯通 | CPU/GPU数据布局、实例更新、间接参数、绑定和pass依赖，功能链对照 |
| L7 专属算法与最终集成 | 剔除/HZB、阴影、GI、SSGI、透明、水、体积、DoF、TAA/TGSR、UI和present接线 | 原作渲染行为与画面逐项验收 | Shader算法恢复、固定输入/相机画面对照、性能和多帧稳定性 |

交换链的raw对象和managed租约需要在L2/L3理解；最终画面的present接线在L6/L7集成。Shader/Pipeline(L4)可以与L3中不依赖shader的部分交错推进。L5已有5个叶规则实现，是跨层已有进展，不说明L0-L4已完整完成。

## 为什么不按raw文件顺序机械推进

Managed句柄、资源表、提交队列、命令缓冲和延迟销毁会互相影响：从CPU drop到GPU真正完成之间必须保留资源。应把这组共同状态当成一个联合恢复单元，沿create → record → submit → complete → reclaim追踪，不能凭某个drop函数单独宣称回收语义恢复。

raw::device是上下文汇聚点，包含大量方法和回调/泛型实例；从整个device大函数起步会同时遇到很多尚未知的类型。先恢复描述类型与资源参数，再返回初始化/创建与队列状态链，信息收益更高。

为完成整个底层库，最终仍需补齐观察到的初始化兼容分支、错误恢复、交换链resize/out-of-date、诊断/统计等范围。可先打通主资源通路，但未覆盖分支必须留在底层待办；主通路通过不等于全库完成。

## 第一个具体恢复单元

建议下一批先恢复 **BufferDesc → TextureDesc → TextureViewDesc → Vulkan创建信息/Barrier描述**。这些接口被很多上层函数共同使用，原PDB已给出较短且精确的函数边界，适合建立可复用的类型与规则。

| 原作例子 | VA | PDB字节数 | 研究重点 |
|---|---|---:|---|
| BufferDesc::new | 0x140bd4de0 | 59 | 默认字段与大小/用途输入 |
| BufferDesc::alignment | 0x140bd4f20 | 96 | 对齐语义及边界 |
| TextureDesc::new_2d | 0x140bd52a0 | 49 | 维度/格式和默认字段 |
| TextureDesc::all_mip_levels | 0x140bd54a0 | 160 | mip规则及零尺寸边界 |
| TextureDesc::half_res | 0x140bd5730 | 86 | 原尺寸缩减规则，不从名称猜舍入 |
| TextureViewDesc::new | 0x140bd57a0 | 57 | view范围/默认值 |
| ImageBarrier::new | 0x140bd65e0 | 34 | 图像屏障描述，分类/屏障库复用 |
| BufferBarrier::new | 0x140bd6610 | 23 | 缓冲屏障描述 |

随后完成同组构造器、修饰方法和消费者；上述长度只是选取入口的证据，不据此预测总工时。类型的布局以多处读写和调用关系交叉核实，Rust所有权设计采用能表达观察行为的形式，原ABI未证实时不得宣称完全一致。

## 每层的完成定义

1. 清单：原模块、已发射函数实例、同地址别名、仅内联方法和依赖胶水分开，测试范围与未覆盖范围明确。
2. 接口：上层使用的参数/返回值、字段、错误、所有权与状态不变量均有证据，unknown不能被默认值掩盖。
3. 实现：游戏私有规则有真实Rust函数体；标准库/Vulkan/公开依赖不重新实现；锁定版本编译。
4. 局部校验：纯规则做原指令差分；驱动调用封装比较创建/绑定/提交参数及状态轨迹，不能把仿真驱动返回值当成真实GPU验证。
5. 联合校验：多个模块协同完成资源创建、执行与回收；实际GPU测试可用Vulkan验证层检查API及同步问题，但通过验证层也不证明复现了原作算法。
6. 完整范围：需要的兼容/异常/清理分支逐项关闭；尚未覆盖时记录局部通过，不能记全库完成。

公开验证层用途参考：[Khronos Validation Overview](https://docs.vulkan.org/guide/latest/validation_overview.html)、[Khronos Synchronization](https://docs.vulkan.org/guide/latest/synchronization.html)。这是后续验收建议，本轮没有运行GPU或修改系统配置。

## 当前最重要的不确定性

- PDB native签名普遍void()占位；仍须从机器码和调用者恢复参数/字段/枚举语义。
- local vk-sync/allocator/shader辅助包的来源与补丁范围尚未全部核实；这是L0的具体任务，不从最新版直接替换。
- 底层完整调用图、泛型/闭包归属和仅内联函数覆盖尚待补齐；93文件清单不能充当完整分母。
- GPU提交/同步/延迟回收的正确性需要联合状态和设备证据；CPU叶函数差分不能覆盖它。
- 上层shader与场景资源格式有独立研究成本。底层完成降低接口成本，不消除GI或重建算法的数学复杂度。

可行性结论：自底向上能减少重复猜测、稳定可复用接口、隔离错误并使后续pass恢复更可控。实施中结合上层调用证据和联合生命周期校验，收益较高；当前证据不足以给整套渲染器确定工期或完成百分比。

证据入口：render-skeleton/files.json、native-procedures.json；render-architecture/dependencies.json、resolved-calls.json；render-recovery/verification.json及已有Rust模块。本文没有新增实现验收或将某层状态改为完成。
