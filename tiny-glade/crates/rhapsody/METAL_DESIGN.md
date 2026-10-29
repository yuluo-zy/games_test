# Metal 集成与移动/桌面双端渲染设计(第 3 版)

本文给出 Metal 集成的完整方案。每个模块列出候选方案,按统一标准比较,给出选择、理由和推翻条件(出现什么证据时应改选)。

版本历史:
- 第 1 版:初稿。
- 第 2 版:自审,修订 18 项。
- 第 3 版:两位独立审查者(A:正确性与可行性;B:性能与工程可行性)审查后修订。审查意见、分歧和裁定见第 10 节。

前置文档:
- [CROSS_BACKEND_API.md](CROSS_BACKEND_API.md):复用/抽象/重写分类与接口草案。本文 D2、D3、D8、D24 与第 4 节覆盖其中对应内容。
- [METAL_INTEGRATION_AUDIT.md](METAL_INTEGRATION_AUDIT.md):测试影响、重写清单、泄漏审计。

## 0. 目标、约束与评判标准

### 0.1 目标

| 平台 | 图形 API | 设备 |
| --- | --- | --- |
| Windows | Vulkan(现有) | 桌面独显与集显 |
| macOS | Metal(开发构建可选 Vulkan + MoltenVK 作对照) | Apple Silicon |
| iOS / iPadOS | Metal | Metal3 级别设备(暂定 A14 及以上,见 D14) |

Android 暂缓。

### 0.2 硬约束

1. **Windows 路径不退化**:画面与现有像素对比一致;性能按第 0.5 节的门槛判定。
2. **现有测试尽量不改**:每一步会触及哪些测试,事先列清单并评审。第 3 版承认"测试完全不改"做不到,见第 3.3 节。
3. **桌面画质保真**:桌面配置档以原版为准。移动配置档允许偏离,但偏离必须可开关、可测量。
4. **游戏逻辑层不感知后端**:`country-core` 不出现 Vulkan 或 Metal 类型。**现状不满足**(F2),由 P3 负责收口。

### 0.3 评判标准与权重

| 标准 | 权重 |
| --- | ---: |
| 正确性风险(出错概率和发现难度) | 25% |
| 移动端适配 | 20% |
| 桌面保真与性能 | 20% |
| 工作量 | 15% |
| 测试与验证 | 10% |
| 长期维护 | 10% |

### 0.4 已确认的事实

第 3 版的事实表经两位审查者逐条核对,并由我抽查了其中影响最大的几条。标"(修)"的行是第 3 版改动过的。

| 编号 | 事实 | 来源 |
| --- | --- | --- |
| F1 | rhapsody 约 6.77 万行非测试代码;276 个非测试文件中约 130 个不含 `vk::`/`ash::`,这些文件合计约 2.24 万行(33%) | 脚本统计 |
| F2(修) | 游戏与框架间有用 `u32` 资源编号和 `u8` 访问码的边界 trait,**但不是后端中立的**:`country-core/src/rhapsody_contract.rs:53,65,77` 暴露 `ash::vk::CommandBuffer`,`:157` `vk::Buffer`,`:187` `vk::DeviceAddress`,`:118` 显存预算扩展结构;`country-core/Cargo.toml` 依赖 `ash`;`compute_pass_builder.rs:225-227` 有手工全屏障接口 | 读码 |
| F3(修) | 游戏只用 14 个访问码:0x02、0x17、0x19、0x1a、0x1d、0x1f、0x22、0x24、0x2e、0x2f、0x32、0x34、0x36、0x37(0x10 只出现在测试中)。主写码 0x37 以及 0x22、0x24 的阶段是"全部命令" | grep;`native/access_table.rs:284-307,399-404` |
| F4(修) | `sync_plan` 非测试代码用 4 种 Vulkan 符号共 34 行;此外 `AccessScope` 内嵌 Vulkan 阶段与访问标志,`NativeState` 内嵌 Vulkan 布局,实际触及面大于"4 种类型" | 读码 |
| F5(修) | 间接绘制有三种:① 带计数(`IndexedIndirectCount`),设备不支持时由 `IndexedCountAdapter` 降级为多 draw 间接;② 不带计数的多 draw 间接(Nani 的 raw 路径 `draw_count = mesh_part_count`,以及 grass、stairs、water_planes 的 `MultiDrawIndirect`/`IndexedIndirect`);③ lavapipe 上的计数形式改写 | `raster_host.rs:2026-2081,2790-2810,2877-2909,2998-3034` |
| F6(修) | 着色器:143 个 compute、117 个 raster。**全部管线在运行时被降级成 bindless**:`render_host.rs:3581`(compute)、`:3638`(raster)都调用 `BindlessShaderPackage::new(..., 64)`,即 `shader_compiler::bindless::lower_pipeline_with_limit`:同类描述符数组别名到同一 binding,1 个推送字选择参数块,资源经堆索引访问 | 读码 |
| F7 | 着色器构建链:include 展开 → DXC → SPIR-V 反射 → 发行容器;发行 SPIR-V 与原版逐项比对过。**GPU 上实际运行的是 F6 降级后的 SPIR-V** | `crates/rhapsody/crates/shader-compiler/src/build.rs` |
| F8 | 帧内 pass 组合入口是 `FramePasses` trait | `render_world.rs:219` |
| F9 | 附件存储操作写死为 STORE;没有瞬态/无内存附件;没有 pass 合并 | `raster_host.rs:1086` |
| F10 | 已有从 AppKit 视图创建 `CAMetalLayer` 的代码 | `production_renderer.rs:2691-2696` |
| F11 | 已有显存不足时降分辨率的状态机 | `adaptive_render_state.rs` |
| F12(修) | Vulkan 引用共 260 处,其中 244 处在 `tiny-glade` 宿主层(73% 在 `render_host.rs` 与 `raster_host.rs`),16 处在 `country-core` | `METAL_AUDIT_leaks.tsv` |
| F13(修) | 游戏的格式值就是 Vulkan 格式编号:9 R8、0x25 RGBA8、0x26 RGBA8_SNORM、0x2b RGBA8_SRGB、50 BGRA8_SRGB(交换链)、0x4c R16F、0x53 RG16F、0x5b RGBA16_UNORM、0x5c RGBA16_SNORM、0x61 RGBA16F、0x62 R32_UINT、0x64 R32F、0x65 R32G32_UINT(G-buffer、GI reservoir)、0x6d RGBA32F、0x7a B10G11R11F、0x7b E5B9G9R9(调色 LUT 3D 纹理;并作为 R32_UINT 纹理的视图格式,`surface_ice.rs:50-60`)、0x7e D32F、0x82 D32F_S8。没有 D24S8 | 字面量 + 审查 A |
| F14(修) | G-buffer(R32G32_UINT,8 字节/像素)的读者:4 个光照着色器(solid、canopy、grass、plant),3 个 compute(`apply_screen_shadows`、`ssgi/upsample`、`extract_half_res_depth_normals`) | grep |
| F15(修) | 光照 pass 在当前像素读 G-buffer,但在其他像素位置用线性采样器读深度(D32F);同时把同一张深度作为只读深度附件绑定 | `solid_light_sun_sky.raster.hlsl:171,245`;`deferred_raster.rs:166,254-258` |
| F16(修) | 默认配置下只有 Canopy、TreeTrunk 两种 kind 走带计数路径,上限 8192,实际计数取 `min(表上限, main_cull_command_bound)`;表中 262144 等大值属于 raw 路径的 kind。静态帧实测:带计数间接 10 次/帧,raw 多 draw 间接 149 次/帧 | `draw_ext.rs:105-111,214-236`;`nani/mod.rs:60-62`;`static/session.json` |
| F17 | 并行录制按连续 pass 块切分,每块一个主命令缓冲,块按上一帧 CPU 耗时划分;已有 `split_allowed` 机制但调用方传空 | `rg/parallel/mod.rs:53,103`;`executor/batch_recording.rs:174-181` |
| F18 | 传输模块:每槽一块持久映射内存,主机写不覆盖仍在使用的 GPU 区域;动态常量页带 CPU 租约和精确完成跟踪;非一致内存由 `host_flush` 刷新 | `transfer/mod.rs:1-22` |
| F19(修) | 依赖必须同时统计 `crate::` 引用和外部 `ash`:`sync/timeline` 基于 `vk::Semaphore`;`error` 有 `Error::Vulkan(vk::Result)` 并被大量匹配;`accounting/stats` 用 `vk::MemoryPropertyFlags`;`format/registry.rs` 有 311 处 `vk::`;`rg` 依赖 `resource`(86 处)和 `tracking`(164 处),`resource` 依赖 `device`(49 处) | 审查 A、B |
| F20 | 着色器公共采样器 13 个,其中内置 9 个;单个着色器最多 13 个 | grep |
| F21(修) | `PassCoverage` 按 pass 声明的访问校验 bindless 租约,**只覆盖图中声明的资源**,看不到着色器经缓冲设备地址读出的地址 | `pass_context.rs:50-61` |
| F22 | 缓冲设备地址(`vk::RawBufferLoad`)用于 7 个着色器:5 个 Nani 剔除变体(`nani/cull_and_bucket*.cs.hlsl`,含 include)、`terrain_editing/rasterize_terrain_stroke.cs.hlsl`、光栅着色器 `nani_plaster.raster.hlsl`、`nani_stairs_floor.raster.hlsl`。`shaders/inc/frustum.hlsl:8` 记载过 SPIRV-Cross 处理该类模块失败 | grep |
| F23 | Wave/Quad 操作出现在 18 个文件中,其中 9 个是光栅着色器(4 个光照着色器、final_post、mei_shadow、ice_plane、water_plane、waterfall) | 审查 A |
| F24 | 帧末合成:最后的 blit 和 4 个 UI/Mei 2D pass 画在同一张 4×MSAA RGBA8_SRGB 目标上;由自定义光栅着色器读 `Texture2DMS` 完成 resolve;`blur_downsample.cs.hlsl` 也读 MSAA 纹理 | `final_post.rs:61-64,197-203`;`shaders/blit/resolve_msaa_to_swapchain.raster.hlsl` |
| F25 | 静态帧按 pass 名去重的 109 个 pass 中,14 个的统计全为 0(无顶点、无片元、无 compute),包括 G-buffer emissive、Velocity、ForwardVelocity、Forward OC/2 等;同一附件集上的光栅 pass 相邻出现(如 G-buffer、emissive、velocity、Forward 连续出现) | 排名 JSON;审查 B |
| F26 | 几何类 pass(输入顶点 > 10 万)在静态帧合计 66.6ms / 154.7ms(43%),many_draws 约 77%;阴影 3 级各约 1.1M 顶点 | 排名 JSON |
| F27 | GI 链(Trace/Reproject/Filter GI、SSGI、Update GI radiance)已是半分辨率(235,520 次调用 = 942,080 的 1/4);全分辨率的是屏幕阴影、SSGI upsample/temporal、体积光、DOF | 排名 JSON |
| F28 | 着色器没有实际使用半精度类型(`min16float`、`float16_t`) | 审查 B |
| F29 | 有顶点属性的光栅着色器 106 个;使用推送常量的 33 个(降级后推送字节进入参数块,推送只剩 1 个选择字) | 审查 A;F6 |
| F30 | Nani 有多块按固定命令总量定容的缓冲(约 15MB/块),大小与场景无关 | `gpu_generate_draw_lists.rs:917,1147` |
| F31 | 降级后的 SPIR-V 实测(404 个管线、638 个阶段,`tools/metal_experiments/results/x1_static.md`):**每个**阶段都带运行时描述符数组能力;**486/638(76%)的阶段至少有一个 binding 别名了多种资源类型**(binding 2 是不同布局的存储缓冲,所有受影响阶段都有,最宽 7 种;binding 0 是分量类型或深度标志不同的图像,39 个阶段;binding 1,16 个阶段) | X1a 实测 |
| F32 | 不使用 K1 别名时,P1/P2 相关类型会触及 100 个测试文件、最多 643 个测试(rhapsody 493、tiny-glade 150、country-core 0);全部类型族合计 139 个文件、841 个测试。上界,不是实际修改数 | X3 实测(`results/x3_test_impact.md`) |
| F33 | SPIRV-Cross(`vulkan-sdk-1.4.350.1`)把全部 638 个降级后的阶段转成了 MSL(`--msl-argument-buffers --msl-argument-buffer-tier 1 --msl-device-argument-buffer 0`,MSL 3.0)。别名 binding 被翻译成「同一个基地址按不同类型重新解释」,不是被拒绝;所有声明都用 `[[id(0)]]`,只用到描述符集 0,即**整个堆是一块平铺的描述符表,所有种类共用同一个槽位索引**。没有阶段超出 Metal 的绑定数上限(最多 2 个缓冲槽、0 个纹理、0 个采样器)。**Metal 编译器是否接受这些 MSL 未验证**(需要 Mac) | X1b 实测(`tools/metal_experiments/results/x1b_*.md`) |

### 0.5 性能门槛(新增)

第 2 版只写了"不退化",没有可执行的标准。修订为:

| 项目 | 要求 |
| --- | --- |
| 基线 | P1 开始前,在 Windows 实机上补齐 4 个场景 × 3 轮:CPU 渲染阶段(均值、p95)、GPU 非隔离整帧时间、GPU 每 pass 隔离耗时 |
| CPU 退化判定 | p95 上升超过 2% **且**超出三轮离散带 |
| GPU 退化判定 | 非隔离整帧时间按同样规则判定(P3 会改变屏障位置,必须看整帧,不能只看 CPU) |
| 门槛的确定 | 2% 是初值,看到三轮离散度后再定 |

### 0.6 Metal 知识的来源

Metal 能力判断有三种来源:我的已有了解;审查者 A 的网络检索(Apple 官方文档被网络拦截,只读到搜索摘要、Apple 论坛和开源项目的 PR,属二手来源);以及待 X5 原型实测。第 6.2 节逐项标注来源和状态。

---

## 1. 总体架构

### 1.1 选择

| 标准 | A. 只用 MoltenVK | B. 迁到 wgpu | C. 中立层 + 原生 Vulkan/Metal | D. 独立 Metal 渲染器 |
| --- | --- | --- | --- | --- |
| 正确性风险 | 良 | 中 | 中 | 差 |
| 移动端适配 | 差 | 差 | 优 | 优 |
| 桌面保真与性能 | 优 | 差 | 良 | 优 |
| 工作量 | 优 | 差 | 中 | 差 |
| 测试与验证 | 优 | 差 | 良 | 中 |
| 长期维护 | 良 | 中 | 良 | 差 |

**选择 C。**两位审查者都认可这个方向。推翻条件:X1 转译失败率很高且人手不足,重新评估 B;X2 与移动预算显示 Metal 原生优化收益很小,只做 A。

### 1.2 三级验证基准

| 基准 | 与 Metal 后端的共同部分 | 能发现的错误 |
| --- | --- | --- |
| B1. Windows 原生 Vulkan 或 lavapipe 参考图 | 只共用 HLSL 源和降级后的 SPIR-V | 转译、精度、后端错误 |
| B2. 同一台 Mac 上的 MoltenVK | 共用转译器(MoltenVK 内部也用 SPIRV-Cross)和 GPU | 后端自身的同步、资源、绑定错误 |
| B3. Metal 后端全跟踪调试模式 | 全部代码 | 同步降级错误 |

B2 一致而 B1 不一致,是转译或精度问题;B2 不一致而 B3 一致,是同步问题。B2 要求 Mac 上能构建 Vulkan 后端,D2 据此修订。

---

## 2. 各模块设计

格式:候选 → 选择 → 理由 → 推翻条件。标"(修)"为第 3 版改动,"(新)"为新增。

### D1. crate 划分

**选择:多 crate + 门面**:`rhapsody-core`(中立)、`rhapsody-vulkan`、`rhapsody-metal`,门面 `rhapsody` 保持现有导入路径。单 crate 加特性开关会让代码交织;动态库插件在 iOS 不可用。拆分方式见第 3.4 节。

### D2. 后端选择与分发(修)

| 方案 | 热路径开销 | 对游戏代码 | Mac 上能否同时构建两个后端 |
| --- | --- | --- | --- |
| a. trait 对象 | 每条命令一次虚调用 | 无 | 能 |
| b. 泛型 | 零 | 到处加类型参数 | 能 |
| c. 枚举分发 | 小 | 无 | 能,但每个二进制都带两个后端 |
| d. 按目标做类型别名 | 零 | 无 | **不能**,与 B2 冲突 |
| e. **按目标 + 特性选择,编译期类型别名** | 零 | 无 | 能:macOS 默认 Metal,`--features backend-vulkan` 时用 MoltenVK |

**选择 e,加三项强制措施**:
1. 门面类型(`Device`、`Swapchain` 等)是包装后端类型的结构体,只转发后端 trait 中声明的方法;后端类型对 `country-core`、`tiny-glade` 不可见。
2. 后端 trait 是唯一契约,缺方法时在编译该后端的目标上报错。
3. CI 对所有目标做类型检查,包括 `aarch64-apple-darwin`、`aarch64-apple-ios`。没有 Apple SDK 时能否通过 check 待 X6 验证,不能的话用 macOS CI 机器。

门面转发的性能:工作区 release 配置是 thin LTO 加 `codegen-units=1`,转发大概率会被内联(审查 B);拆 crate 会改变内联决策,用第 0.5 节的门槛实测。

### D3. 词汇类型(修)

**选择:数值兼容的新类型**(`Format(u16)`、`TextureUsage(u32)`、`AccessCode(u8)`,数值等于现有值,这些数值本身就是 Vulkan 编码)。游戏字面量不改;Vulkan 降级零成本。

修订:
- `AccessCode`:中立层保证 F3 的 14 个码,加上框架内部的呈现、传输读、主机读。
- `StageMask`:间接参数、顶点输入、顶点着色、片元着色、颜色附件、深度附件、计算、传输、主机、呈现,**加上"全部"位**。游戏主写码 0x37 的阶段就是"全部"。降级到 Metal 时,"全部"表示等待全部阶段(D5)。
- `ImageState`:覆盖 `sync_plan` 用到的布局集合,包括深度只读、模板只读组合。

### D4. 同步计划中立化(修)

**选择:中立核心 + Vulkan 降级层保持旧输出类型。**修订:
- 触及面比"4 种类型"大:`AccessScope` 内嵌 Vulkan 阶段与访问,`NativeState` 内嵌布局(F4)。X3 的统计范围扩大到这两个类型;审查 B 给出的上界是 413 个测试(提到布局或阶段的测试文件中的测试总数)。
- 等价快照除单元测试场景外,还要录制 static 与 many_draws 两个生产帧(各 164 个 pass)的 `BoundTransition`、`BoundWait`。中立化后逐字段对比。

推翻条件:中立化后 Vulkan 输出无法逐字段相等,改为直接替换输出类型并修改相应测试。

### D5. Metal 同步实现(修)

第 2 版的"混合跟踪"在全 bindless 下不成立:通过 argument buffer 访问的资源不参与驱动的 hazard 跟踪,驻留集也不提供跟踪(审查 A、B 一致,来源为搜索摘要与已有了解)。

| 方案 | 做法 | 评价 |
| --- | --- | --- |
| a. 全部自动跟踪 | — | 全 bindless 下实际得不到保护 |
| b. **全部不跟踪,由计划驱动** | 依赖全部来自中立计划;上传完成由完成点保证 | 与 rhapsody 设计一致 |
| c. 混合 | — | 跟踪类资源实际不受保护,还多驱动开销 |

**选择 b。**降级规则:

| 计划中的依赖 | Metal 实现 |
| --- | --- |
| 同一计算编码器内的 dispatch 之间 | **默认串行 dispatch 模式**(保证顺序执行);并发模式加编码器内内存屏障作为优化,需 X5 测出收益才启用 |
| 同一渲染编码器内(合并后的相邻光栅 pass) | 只有附件依赖,由渲染编码器语义保证(D13 e) |
| 跨编码器、同一命令缓冲 | `MTLFence`:前一编码器结束时更新,后一编码器开始时按阶段等待;阶段为"全部"时等待全部阶段 |
| 跨命令缓冲(并行录制的块之间)、跨帧、与 CPU 同步 | `MTLEvent`/`MTLSharedEvent` 带值事件。**不用 `MTLFence` 跨命令缓冲**:现行文档与论坛说法冲突,语义未定 |
| 上传完成后的首次使用 | 等待传输模块的完成点(`SyncPoint`,即共享事件的值) |
| 瞬态别名的交接 | 别名规划依赖已有 DAG 的传递执行顺序(`PreserveParallelism`,不加边)。Metal 降级时,要确认这条传递顺序都已经由上面的 fence 或事件实现;做不到的地方由计划显式补一条别名交接依赖 |
| 图像布局转换、队列族所有权转移 | 丢弃 |

调试构建提供"全部资源改为跟踪"的开关(B3),并检查每一次不跟踪资源的使用都能在计划里找到依赖。

### D6. 内存分配(修)

| 方案 | 做法 | 评价 |
| --- | --- | --- |
| a. 每资源独立分配 | — | 不能别名;与 D22 的 heap 驻留后备冲突 |
| b. **全部从大 heap 子分配** | 持久资源从大 heap 子分配,瞬态资源用 placement heap 按别名计划摆放,上传与动态常量用共享存储,tile 内专用附件用无内存存储 | 与现有默认开启的 `gpu-allocator` 子分配和记账模型一致;单次分配成本低;D22 的按 heap 驻留后备方案可用 |

**选择 b**(修订第 2 版的"持久资源单独分配")。别名规划的输入(大小与对齐)由后端提供,Metal 侧从 heap 的纹理尺寸与对齐查询获得(这项查询在 macOS 10.15 / iOS 13 起提供,据审查 A 的检索)。

### D7. 命令录制与编码器划分(修)

编码器分组本身仍然选:相邻、同类、无需跨编码器同步的 pass 归成一组。修订的是分组与并行分块的关系:

| 方案 | 做法 | 评价 |
| --- | --- | --- |
| a. **先分块,块内分组** | 分块不受分组约束,块边界处切开组 | 保住负载均衡;每个块边界最多多出一个编码器和一个事件 |
| b. 先分组,以组为不可分单位分块(第 2 版) | 用 `split_allowed` 实现 | 静态帧有长串连续 compute pass(30、21、18 个),禁止切开会破坏 8 块的负载均衡 |
| c. Metal 上关闭并行录制 | — | 丢掉 CPU 并行收益 |

**选择 a。**审查 A 和 B 在这里意见不同:A 认为复用已有的 `split_allowed` 就能"先分组后分块";B 用实测数据指出这会伤害负载均衡。我采纳 B,理由是代价对比:在块边界切开组,最多多出 7 个编码器和 7 个事件,开销在微秒级;而负载失衡会直接拉长 CPU 录制时间,后者是已测量过的瓶颈(基线 9.57ms)。

第 2 版说"先分组则结果确定"不成立:分块本身就按上一帧耗时变化。

Vulkan 后端不分组,行为不变。诊断模式下关闭分组与合并,保证每个 pass 能单独计时(D18)。

### D8. 资源绑定(修:改为单一模型)

现状(F6):所有管线在运行时降级为"堆表 + 参数块 + 1 个推送字"。第 2 版的"普通着色器直接绑槽位"是一条 Windows 上不存在的新路径。

| 方案 | 做法 | 评价 |
| --- | --- | --- |
| a. 第 2 版方案:普通着色器用槽位直接绑定,bindless 用 argument buffer | — | 两个后端从绑定层开始分叉;与 ICB 冲突(ICB 命令不能绑纹理,审查 A) |
| b. **单一模型:离线降级 → argument buffer** | 构建期对每个管线跑同一个 `lower_pipeline_with_limit`(limit 固定为 64,结果确定),再用 SPIRV-Cross 输出 argument buffer 形式的 MSL | 与 Windows 同构;`PassCoverage` 校验和驻留集直接复用;ICB 能用(纹理经 argument buffer) |

**选择 b。**对应关系:

| Vulkan(现有) | Metal |
| --- | --- |
| 共享描述符堆(一个 set,数组按类型别名到同一 binding;**每种资源各有独立的索引空间**,`bindless/slots.rs` 的 `entries: [Vec<Slot>; 4]`) | 一块**平铺的 8 字节描述符表**(device 地址空间的 argument buffer),所有种类共用 `[[id(0)]]`(F33)。堆的槽位索引必须跨种类唯一:`flat_slot = kind_base + index`,`kind_base` 取自各种类的容量配置;参数块里写入的是 `flat_slot` |
| 参数块(存储缓冲,内含原推送字节和堆索引) | 同样是缓冲,经缓冲偏移或 `setBytes` 传入 |
| 1 个推送字 | `setBytes` 4 字节;ICB 中改用继承的缓冲(ICB 命令不能 `setBytes`,审查 B) |
| 采样器在堆表里,索引运行时从参数块读取 | 同样在 argument buffer 里 |

修订第 2 版关于采样器的说法:降级后采样器在堆表里,不能在转译时变成常量采样器;而且常量采样器同样计入每阶段 16 个的上限(审查 A,据功能表脚注的搜索摘要)。在 argument buffer 中的采样器数量上限待 X5 确认。

**最大风险**:降级后的 SPIR-V 中,同一个 binding 上别名了维度或类型不同的资源数组。这是 SPIRV-Cross 的已知薄弱点(审查 B 的已有了解)。X1a 的实测把它量化了(F31):它不是个别着色器的边角情况,而是 **76% 的阶段**都有,其中 binding 2 上不同布局的存储缓冲别名在所有受影响的阶段里都存在。所以 D8 的单一模型是否成立,取决于 X1b(SPIRV-Cross 能否处理这些别名)的结果,而且需要在 P0 里最先出结果。

**X1b 已部分回答**(F33):SPIRV-Cross 能转译全部别名,代价是堆必须是平铺的、跨种类索引唯一的 8 字节描述符表。要点:(1) Metal 的堆实现要用 `flat_slot = kind_base + index`,这发生在写参数块的 CPU 一侧;(2) 所有种类的描述符必须同为 8 字节,所以**不能**启用 `--msl-runtime-array-rich-descriptor`(它把缓冲描述符变成 16 字节),实测没有任何阶段需要它;(3) 仍未验证的是 Metal 编译器接受这些 MSL、device 地址空间的 argument buffer 性能可接受、生成代码正确——需要 Mac。

推翻条件:Metal 编译器拒绝这种按类型重新解释的写法,则改为构建期把别名数组拆成按类型、按维度分开的数组再转译(只在 Metal 构建中做,Windows 不变)。

### D9. 着色器转译链(修)

| 方案 | 做法 | 评价 |
| --- | --- | --- |
| a. 运行时转译 | — | 首次卡顿、发热 |
| b. **离线:降级后的 SPIR-V → SPIRV-Cross(argument buffer 模式)→ metallib** | 构建期完成 | 输入与 Windows 上运行的完全相同 |
| c. Apple Metal Shader Converter(输入 DXIL) | — | **在本项目中作后备也有问题**:bindless 降级是 SPIR-V 变换,DXIL 路径没有对应的降级,绑定 ABI 对不上 |
| d. naga | — | 支持面较窄(印象,未核实) |

**选择 b。**单个着色器失败时的后备从"改用 Metal Shader Converter"改为:先修 SPIRV-Cross 选项或做 SPIR-V 预处理;仍不行,则在 HLSL 源加 Metal 专用变体,且该变体同样经过 bindless 降级。

需要逐个确认的特性(X1、X5):
- Wave/Quad 操作在 compute 和**片元**阶段的映射(F23,9 个光栅着色器),包括 helper lane 语义;`tgsr_reproject` 用 `lane^2`、`lane^16` 取相邻像素,依赖线程到 lane 的映射。
- 缓冲设备地址(D23)。
- 别名数组(D8)。
- 图像原子、64 位类型、片元丢弃语义。

快速数学见 D25。

### D10. 管线创建与缓存

**选择:metallib + 二进制归档,未命中时走现有异步编译器。**`AsyncPipelineCompiler`、`PipelineTicket`、`HotPipeline` 中立,直接复用。二进制归档的兼容规则待核实。

### D11. 队列

**选择单队列。**现有 `queue_schedule` 已支持多个队列类别映射到同一队列。推翻条件:上传与渲染串行导致加载卡顿。

### D12. 呈现(修)

**选择直接用 `CAMetalLayer`**,iOS 从 UIView 的 layer 获取。补充:
- 交换链格式:现有路径优先 BGRA8_SRGB(`present/swapchain/recreate.rs:189`),Metal 映射到 sRGB 像素格式。
- 生产路径用光栅版 resolve 写交换链,`CAMetalLayer` 保持 `framebufferOnly`;compute 版 `resolve_to_swapchain.cs.hlsl` 在 Metal 上要求关闭 `framebufferOnly` 并支持 sRGB 存储写,只在用到它时启用。
- 帧率上限用"最短时长后呈现",macOS 上的可用性待核实。

### D13. TBDR 优化(修:新增零画质代价的主收益)

| 方案 | 做法 | 画质 | 优先级 |
| --- | --- | --- | --- |
| e1(新). **跳过空 pass** | 统计为 0 的 pass(F25)不建编码器;需要 clear 语义的保留为 clear | 逐位一致 | 1 |
| e2(新). **合并相邻同附件光栅 pass** | 附件集相同、中间没有非附件依赖的相邻光栅 pass 合成一个渲染编码器 | 逐位一致 | 1 |
| e3(新). **MSAA 在 tile 内 resolve** | MSAA 附件用无内存存储,store 动作用多重采样 resolve | **会变**:现有 resolve 是自定义着色器读 `Texture2DMS`,`blur_downsample` 也读 MSAA 纹理(F24) | 移动配置档 |
| b. 存储操作与无内存附件 | 附件描述增加"用完即丢"和"瞬态"标志,默认不变 | 逐位一致 | 2 |
| c2. G-buffer 留在 tile,替换读 G-buffer 的效果 | — | 改变 | 推迟 |
| d. 降低内部分辨率、砍全分辨率 compute | — | 改变 | 移动配置档 |

**选择:e1、e2 立即做(最优先),b 跟进,e3 与 d 放进移动配置档,c2 推迟。**

理由:
- 在 TBDR 上,每多一个渲染编码器,附件就要多 load 一次、store 一次。审查 B 估算,多余的约 9 个编码器加 4 个 UI 编码器,每帧约 250–400MB 带宽,在手机带宽下约 5–10ms。这个数用到了附件格式与 load 动作的假设,误差可能达 2 倍,需 X2 实测。
- e1、e2 不改变画面。e1 在 Windows 上同样有效,诊断方案第 6 节已把"空输入不声明 pass"列为逐位一致的优化,放在共享路径上做。e2 在 Vulkan 上可以用 dynamic rendering 的挂起/恢复实现,也可以不合并;Windows 上默认先不启用,验证像素一致后再开。
- e3:移动配置档有两种做法,一是关闭帧末 MSAA,二是把自定义 resolve 换成内置 resolve 并让 `blur_downsample` 改读 resolve 结果。两者都会改变画面,由 X2 或 X4 的画质与耗时数据决定。
- c2 推迟的理由更充分了:G-buffer 有 3 个 compute 读者和 4 个光照着色器读者(F14),而且只有 8 字节/像素。

### D14. 设备能力与分档(修)

**选择:能力 → 档位 → 配置档**,设备问题表可覆盖。修订:**设定最低设备**。

| 要求 | 原因 |
| --- | --- |
| Metal3 级别(暂定 iOS 上 A14 及以上) | 缓冲 GPU 地址需要 Metal3 且 macOS 13 / iOS 16 起(D23);A13 是否属于 Metal3,各来源说法冲突;A12/A13 在 iOS 18 上有 ICB 的 `inheritBuffers` 静默挂起问题(审查 A,据开源项目的 PR) |
| 驻留集可用(macOS 15 / iOS 18 起)为首选;更低系统走 heap 驻留后备 | D22 |
| ICB 可由 compute 填充,并能按 GPU 缓冲中的范围执行 | D19 |

档位:T1 主流手机、T2 高端手机/平板/Apple Silicon Mac、T3 桌面独显。第 2 版的 T0(低端设备)取消,低于最低设备的不支持。

### D15. 移动配置档(修:补几何与 CPU)

**选择:按效果替换实现**,保留 `render_world` 骨架。前提:先跑预设对照(诊断方案第 10 节)和 X2,确认分辨率缩放对 G-buffer 耗时的影响,从而知道几何受限和片元受限各占多少。

| 效果 | 桌面(原版) | 移动配置档初版 |
| --- | --- | --- |
| 几何:LOD 与距离裁剪 | 原版 | **新增**:LOD 切换距离与裁剪距离按档位缩短 |
| 几何:植被、石子密度 | 原版 | **新增**:按档位降低 |
| 阴影 | 3 级,无逐级联裁剪 | 级联数与级联分辨率可配,阴影绘制距离缩短;**逐级联剔除放在共享路径上做**(剔除着色器与 draw list 的改动,Vulkan 用现有带计数间接绘制),保守剔除逐位一致,Windows 的阴影 pass 同样受益 |
| G-buffer 与光照 | 分开 | 保持分开(c2 推迟);e1、e2 生效 |
| 屏幕空间阴影、SSGI upsample/temporal | 全分辨率 | 降分辨率或关闭 |
| GI 链 | **已是半分辨率**(F27) | 优先降低更新频率;不再降分辨率(再降就是 1/16 像素,画质风险大) |
| DOF | 全分辨率 | 关闭或半分辨率 |
| 体积光 | 全分辨率 | 四分之一分辨率 |
| 帧末 MSAA | 4× 自定义 resolve | e3 |
| 上采样 | TAA/TGSR | TGSR 低内部分辨率 |
| 半精度 | 无(F28) | **新增可选项**:对 ALU 受限的着色器引入半精度变体,画面会变,只用于移动配置档 |
| 纹理 | BC | ASTC |

预算目标(初值,X2 和预设对照之后修订):

| 项目 | 目标 |
| --- | --- |
| 帧率 | 30 FPS。审查 B 估算 60 FPS 在 CPU 和 GPU 上都不成立 |
| CPU | 渲染线程 p95 不超过 33ms 的一定比例(待定);手机按 2 个性能核、持续降频估算(审查 B 估约为桌面 1.0–1.3 倍,即 p95 约 16–20ms) |
| GPU | 持续负载下不超过 30ms |
| 发热 | 30 分钟持续运行的帧率与温度曲线;测法在 P8 前定 |
| 内存 | 见 D17 |

### D16. 资产与纹理压缩

**选择:离线按平台产出**(Windows BC,Apple ASTC)。资产授权见 D20,已提前到 P0。

### D17. 内存预算与生命周期(修)

**选择:主动预算**(系统可用内存加内存警告,作为 `AdaptiveRenderState` 的输入),资源分级释放后续跟进;iOS 进入后台必须停止提交 GPU 工作(审查 A 核实:后台提交会被拒绝,必须在进入后台的回调返回前停止)。

补充:
- **现在就开始记录内存数字**:在 Windows 上用别名规划的统计(`PhysicalMemoryPlan` 的 planned bytes)记录每帧瞬态内存。瞬态别名集按在途帧各借一份,在途帧越多占用越高。
- Nani 的固定容量缓冲(F30,约 15MB/块)按档位缩减容量。
- 移动配置档的内存目标在拿到上述数字后确定。

### D18. 诊断(修)

**选择:编码器边界的计数器采样;隔离模式下每个 pass 单独一个编码器,并关闭分组与合并。**
- Apple GPU 只支持阶段边界采样,不支持 draw、dispatch 粒度(审查 A,据检索);有报告称 macOS 26 / Metal 4 上旧接口返回全 0,要改用新的计数器堆接口,X5 一并确认。
- Metal 没有 pipeline statistics,`pass_statistics` 返回不支持。基于"顶点、片元、compute 调用数"的归因方法在 Metal 上没有对应物,移动端分析改用 Xcode 工具。

### D19. 间接绘制(修:分路径)

按 F5 的三种路径分别设计:

| 路径 | 频率(静态帧) | 上限 | Metal 做法 |
| --- | --- | --- | --- |
| ① 不带计数的多 draw 间接(Nani raw、grass、stairs、water_planes) | 149 次调用/帧 | draw 数在 CPU 端已知(`mesh_part_count` 等) | **CPU 逐条编码**:按已知 draw 数循环调用单 draw 间接绘制,各条读参数缓冲的不同偏移 |
| ② 带计数(Canopy、TreeTrunk) | 10 次/帧 | ≤ 8192 | **ICB**:图层声明一个转换 compute pass(与 Vulkan 上 `IndexedCountAdapter` 的地位相同),读间接命令与计数,写入 ICB;再按 GPU 缓冲中的范围执行 ICB |
| ③ lavapipe 计数形式改写 | — | — | 与 Metal 无关 |

理由:
- ① 的 draw 数是 CPU 已知的小数目,逐条编码最简单,不需要 ICB。总编码次数是 Σdraw 数,X2 统计它,以确认 CPU 成本可接受。
- ② 如果逐条编码,要按上限编,即每帧最多 8192 × 10 次,CPU 不可接受,所以用 ICB。ICB 容量按 8192 × 10 条定。转换 compute 每帧 10 次,每次不超过 8192 线程,GPU 成本与现有的 "Reset indirect args"(0.019ms)同量级(审查 B 估算)。
- 转换 pass 在**图层**声明,而不是藏在后端内部(审查 A):这样它的资源访问进入同步计划,`PassCoverage` 和 B3 基准都能覆盖到。由后端能力决定是否插入。
- ICB 的约束:纹理必须经 argument buffer 访问(D8 已满足);命令不能 `setBytes`,推送字改用继承的缓冲;A12/A13 + iOS 18 上 `inheritBuffers` 有挂起问题(D14 的最低设备避开了它)。

推翻条件:
- X5 中 ICB 的 GPU 耗时劣于"按实际计数逐条编码"。有开源项目实测在 M4 Pro 上 ICB 更慢(审查 A,据检索)。如果出现这种情况,② 改为回读上一帧计数、按它逐条编码,并评估延迟一帧带来的画面影响。
- 设备不支持按 GPU 范围执行 ICB,则不支持该设备(D14)。

### D20. 平台层(修)

| 项目 | 选择 |
| --- | --- |
| 窗口与事件循环 | winit;项目用的 PounceLight 定制 fork 的 iOS 支持需验证 |
| 触摸输入 | 独立触摸输入,UI 层做手势 |
| 音频 | FMOD(iOS SDK);`native-audio` 在非 Windows 上是可选特性,早期 iOS 实验可以关闭音频 |
| 平台服务 | Steam 只在桌面 |
| 文件路径 | `paths` 增加 iOS 沙盒路径 |
| **资产与 FMOD 分发授权** | **提前到 P0**:它同时阻塞 X4(iOS 实验)和 P8。资产管线按随包分发设计,打包以授权为前提 |

### D21. 上传、读回与动态常量

**选择:按资源类型分**:动态常量页、`register_cpu_authoritative_buffer` 登记的缓冲直接放共享存储;纹理和大块缓冲走共享暂存加 blit 到私有存储;读回用共享缓冲。槽位、租约、完成跟踪原样复用;`host_flush` 在 Metal 上为空操作;单队列,不产生所有权转移。

宿主层 47 处手工同步按用途归类迁入:上传拷贝迁入 `TransferManager`;截图读回(`screenshot_capture.rs` 14 处)迁入读回票据;图像初始化迁入渲染图的首次使用声明。

### D22. 资源驻留(修)

全 bindless 之后,驻留的范围是**整个堆表**,不只是少数着色器。

| 方案 | 做法 | CPU 开销 |
| --- | --- | --- |
| a. 每编码器逐资源声明 | — | 审查 B 估算:约 40 个编码器 × 上千资源,每帧 4 万以上条目,约 1–4ms(推测) |
| b. 每编码器按 heap 声明 | 依赖 D6 的"全部资源从大 heap 子分配" | 常数 |
| c. **驻留集** | 堆表内容就是驻留集 | 最低 |

**选择 c,后备 b,a 只用于调试校验。**
- 驻留集需要 macOS 15 / iOS 18 起(审查 A,据头文件标注)。更低系统用 b。
- 缓冲设备地址引用的缓冲(D23)由着色器在 GPU 上读出地址,`PassCoverage` 看不到(F21)。这类缓冲单独登记,常驻在驻留集里。
- 调试校验:图内声明的资源用 `PassCoverage` 比对;经地址访问的缓冲比对登记表。

### D23. 缓冲设备地址(修)

影响面是 7 个着色器,包括 2 个光栅着色器(F22)。

| 方案 | 做法 | 评价 |
| --- | --- | --- |
| a. **Metal 缓冲 GPU 地址** | 转译器把物理存储缓冲指针转成 MSL 设备指针 | 着色器不改;需要 Metal3 且 macOS 13 / iOS 16 起;`frustum.hlsl:8` 记载过 SPIRV-Cross 处理失败,X1 要优先测这 7 个 |
| b. 改为堆索引访问 | 7 个着色器改写 | 偏离原版;作后备 |

**选择 a。**设备门槛并入 D14 的最低设备。X1 失败时对这 7 个着色器走 b,做成 Metal 专用变体并经过同一套 bindless 降级。

### D24. 格式映射(修)

**选择:Metal 后端静态表**(Vulkan 编号 → Metal 像素格式,加上能力需求),覆盖 F13 的完整格式清单。设备创建时按"格式 × 用法"检查一遍,不满足就在启动时报错。

需要 X5 确认的组合:

| 组合 | 来源 | 不满足时的处理 |
| --- | --- | --- |
| R32G32_UINT 作颜色附件 | G-buffer | 无可替代格式,不满足则不支持该设备 |
| B10G11R11F 存储写 | 多处 compute 输出 | 改用 RGBA16F(内存翻倍),仅限 Metal |
| R32_UINT 纹理以 E5B9G9R9 视图采样 | `surface_ice.rs:50-60` | 改为着色器内手工解码,仅限 Metal |
| E5B9G9R9 作 3D 纹理 | 调色 LUT | 改用 RGBA16F |
| D32F_S8 的模板在 compute 中读取(需要仅模板视图) | `extract_half_res_depth_normals.cs.hlsl:19` | 拆出单独的模板拷贝 |
| D32F 线性过滤 | 光照 pass 用线性采样器读深度(F15) | 拷贝到 R32F 再采样;不能改成最近采样,那样会改变画面 |
| 同一深度纹理同时作只读深度附件和采样纹理 | 光照 pass(F15) | 据检索,Dawn 在 Metal 上要求关闭深度写并带上读与附件两种用法;不满足则拷贝 |

### D25. 浮点一致性与像素验收(修)

| 构建 | 快速数学 | 用途 |
| --- | --- | --- |
| 验证构建 | 全局关闭 | B1/B2 对比 |
| 发行构建 | **默认开启**,按着色器白名单关闭 | 只关闭在 B1 容差测试中超差的着色器 |

理由:全局关闭在 ALU 受限的着色器上可能有 5–15% 代价(审查 B 推测)。Metal 编译器默认开启快速数学(审查 A 核实);`-fno-fast-math` 与安全数学模式的效果不同,X5 确认用哪个开关。

容差规则:

| 输出类型 | 规则 |
| --- | --- |
| 整数格式(G-buffer、reservoir 等打包数据) | 必须相等 |
| UNORM/SRGB 8 位 | 每通道差不超过若干 LSB(初值 2) |
| 浮点格式 | 相对误差阈值(初值 1e-3),非有限值必须一致 |
| 时间性效果 | 固定抖动、重置历史后比较第 N 帧 |

扩展现有的 `surface_comparison.rs` 来承载。

### D26. 顶点输入与推送常量(新)

| 项目 | 现状 | Metal |
| --- | --- | --- |
| 顶点输入 | 106 个光栅着色器有顶点属性 | 从反射生成顶点描述符 |
| 缓冲槽位 | — | 顶点缓冲、参数块、argument buffer 共用每阶段 31 个缓冲槽;构建期为每个管线分配并检查不超限 |
| 推送常量 | 33 个着色器原本用;降级后只剩 1 个选择字(F29) | `setBytes`;ICB 中用继承缓冲(D19) |

### D27. UI、截图与交换链(新)

| 项目 | 现状 | Metal |
| --- | --- | --- |
| egui | `egui_gpu.rs` 有 5 处 `vk::` | 走同一套光栅 pass 与 bindless 模型;并入 P3 收口 |
| 截图 | `screenshot_capture.rs` 有 14 处 `vk::`,多为布局与屏障 | 迁入读回票据(D21) |
| 交换链颜色 | 优先 BGRA8_SRGB | 见 D12 |

---

## 3. 重构方案

### 3.1 迁移策略

**选择:门面 + 分阶段替换。**

### 3.2 兼容层规则

1. 门面继续导出现有公开路径;移入 core 的项在原路径用 `pub use` 重新导出。
2. `rhapsody::native::*`、`rhapsody::raw::*` 保留在 `vulkan-raw` 特性后面,直到对应测试迁移完毕。
3. CI 检查 `rhapsody-core` 中不出现 `ash::`/`vk::`。

### 3.3 测试影响(修)

第 2 版"原则上不改"被高估。审查结果:

| 测试组 | 规模 | 何时受影响 |
| --- | --- | --- |
| rhapsody 集成测试中,所在文件引用 `ash`/`vk` 的 | 390 / 488 个(78 个文件) | P4 门面隐藏后端类型时;保留 `vulkan-raw` 特性可推迟 |
| tiny-glade 中,所在文件引用 `vk` 的 | 237 个 | P3 宿主层收口时 |
| 提到布局或阶段的测试文件中的测试(X3 上界) | 413 个 | P2 同步计划中立化时 |
| `profile` 模块中引用 `crate::rg` 的测试 | 2 处(`record_trace_tests.rs:125`、`command_workload/tests.rs:26`) | S1 移动 `profile` 时,这些测试要移到门面 crate |
| 纯逻辑测试 | 3400+ | 不改 |

策略:
- 测试修改**只允许**两类:改导入路径;把对 Vulkan 类型的断言改为对中立类型的断言。断言的含义不能变。
- 每个阶段开始前列出受影响的测试清单,并评审。
- 新增测试:同步计划快照(含 2 个生产帧)、记录后端、Metal 后端(macOS CI)、三级基准像素对比。

### 3.4 依赖拆分(修:同时统计外部 `ash`)

> 本节的 S0–S7 已由详细设计 [CORE_SPLIT_DESIGN.md](CORE_SPLIT_DESIGN.md) 取代:实测依赖链 `error → sync → accounting → collections/retire` 全部直接使用 `ash`,`rg` 的污染源只有两处(渲染回调类型、资源模块里的绑定辅助)。以该文为准。

第 2 版的 S1 把 `sync`、`error`、`accounting` 列为"无后端依赖",但它们本身直接用 `ash` 类型(F19)。修订后的步骤:

| 步 | 内容 | 前置条件 | 受影响测试 |
| --- | --- | --- | --- |
| S0 | 重新生成依赖图:同时统计 `crate::`、`ash::`、`vk::`、`gpu_allocator` | — | — |
| S1 | 移入 core:`log_ring`、`retire`、`collections`、`profile` | S0 | `profile` 的 2 处测试移到门面 |
| S2a | 拆 `sync`:`SyncPoint`、`QueueId`、`SyncSet` 的中立部分移入 core;`SyncSet` 的阶段字段改为中立 `StageMask`;`timeline`(Vulkan 信号量)留在后端,core 只看到句柄 | S1 | 断言 `SyncSet` 阶段的测试 |
| S2b | 拆 `error`:`Error::Vulkan(vk::Result)` 改为不透明的后端错误(保留可下转型),错误分类(`ErrorKind`)保持不变 | S1 | 匹配 `Error::Vulkan` 的代码(如 `present/swapchain/recreate.rs:170`)与测试 |
| S2c | 拆 `accounting`:内存属性改为中立的内存类别 | S1 | 少量 |
| S2d | 拆 `format`:中立的格式信息(按数值)进 core,Vulkan 格式表留在后端 | S1 | 少量 |
| S3 | 把挂在后端模块下的纯中立项(`CpuArenaPool`/`CpuArenaLease`、`QueueClass`、`RetireTicket` 等)移入 core | S2 | 否(重新导出) |
| S4 | 访问码词汇中立化:码表、读写属性、`AccessInfo` 进 core;`AccessScope`(内嵌 Vulkan 阶段与访问)的 Vulkan 版本留在后端 | S2a | 断言 `AccessScope` 字段的测试(X3 统计) |
| S5 | `tracking` 移入 core(其对 `native` 的依赖主要是 S4 的词汇) | S4 | X3 统计 |
| S6 | `resource` 拆分:描述类型与托管句柄进 core,创建与分配留后端(`resource` 依赖 `device` 49 处) | S5 | X3 统计 |
| S7 | `rg` 分组 A 移入 core;`rg` 对 `command::recording`、`VulkanCommandBuffer`、`Device` 的依赖改为依赖后端 trait | S6、P2 | X3 统计 |

第 2 版的 S4("rg 分组 A 移入 core")必须等 `resource` 和 `tracking` 中立化之后才能做,所以排到 S7。审查 B 估计 P1 的工作量至少是第 2 版隐含估计的两倍。

### 3.5 后端 trait 的实际录制面(新)

后端 trait 要覆盖的不只是 `Command` 枚举(审查 B):

| 录制面 | 现状 | 处理 |
| --- | --- | --- |
| `pass_api::Command` | 携带 `vk::Buffer`、`vk::RenderingInfo`、`vk::PipelineLayout`;`CommandBackend::dispatch` 接收 `vk::CommandBuffer` | 改为中立句柄,避免 Metal 每条命令查表 |
| 管线与描述符绑定 | `rg/bindless_binding.rs` 直接调用绑定描述符集 | 纳入 trait |
| 屏障 | `command/barrier` | 由同步降级(D5)产生,纳入 trait |
| 计时 | `rg/profiling` 9 处、`render_host.rs` 11 处直接录制 | 纳入 trait |
| 执行后端 | `rg/executor/backend_api.rs` 的 `ExecutionBackend` 含 Vulkan 类型 | 中立化 |

---

## 4. 分阶段计划(修)

主要调整:P0 增加基线、预设对照、资产授权和 Metal 原型分支;X4 改为最小 iOS harness;country-core 收口并入 P3;共享路径上的逐位一致优化单列为 P7。

| 阶段 | 内容 | 依赖 | 验收 |
| --- | --- | --- | --- |
| **P0 准备与决策实验** | ① Windows 基线:4 场景 × 3 轮(第 0.5 节);② 预设对照(诊断方案第 10 节);③ X1、X2、X3、X5、X6;④ 资产与 FMOD 分发授权确认;⑤ X4(最小 iOS harness,见第 5 节) | 无 | 门槛定稿;各决策的推翻条件逐项判定 |
| **P0′ Metal 原型分支**(与 P1–P3 并行) | 在分支上用现有内部接口直接写 Metal 路径,目标跑到 G-buffer 加光照 | X1、X5 | 跑通;产出对后端 trait 形状的反馈,供 P4 定稿 |
| P1 依赖拆分 | 第 3.4 节 S0–S6 | P0 | 构建通过;测试按第 3.3 节规则;门槛不退化 |
| P2 同步计划中立化 | D4(先录快照,含 2 个生产帧) | P1 | 快照逐字段相等;像素一致 |
| P3 宿主层与 country-core 收口 | 宿主层 244 处 + country-core 16 处 Vulkan 引用;`rhapsody_contract.rs` 的泄漏接口;47 处手工同步(D21);egui、截图(D27) | P1 | 宿主层与 country-core 无 `vk::`(`vulkan-raw` 内除外);像素一致;**GPU 非隔离整帧门槛** |
| P4 后端 trait 定稿,Vulkan 迁入 | S7;第 3.5 节录制面;D2 强制措施与 CI | P2、P3、P0′ | 测试按规则;门槛不退化;Apple 目标类型检查通过 |
| P5 Metal 基础后端 | 设备、内存(D6)、资源、格式(D24)、命令与编码器(D7)、传输(D21)、呈现(D12)、顶点输入(D26) | P4 | Mac 上清屏、三角形、compute、读回 |
| P6 Metal 完整后端 | 同步(D5)、绑定(D8)、驻留(D22)、缓冲设备地址(D23)、间接绘制(D19)、管线缓存(D10)、诊断(D18) | P5 | 桌面配置档在 Mac 上通过三级基准(D25) |
| **P7 共享路径的逐位一致优化** | D13 e1(跳过空 pass)、b(存储操作);D15 的逐级联保守剔除;D13 e2(Metal 默认开启,Vulkan 验证后再开) | P2 | Windows 像素逐位一致,性能按门槛判定(预期改善) |
| P8 移动配置档 | D14、D15、D16、D17、D13 e3/d | P6、P7、预设对照、X2、X4 | 达到 D15 的预算目标;30 分钟持续发热测试 |
| P9 iOS 平台层 | D20(授权已在 P0 确认) | P5 | 真机启动、前后台切换、内存警告 |

## 5. 决策实验(修)

| 编号 | 实验 | 回答的问题 | 设备 |
| --- | --- | --- | --- |
| X1 | 把**降级后**的全部 SPIR-V 用 SPIRV-Cross(argument buffer 模式)转成 MSL 并编译;重点测别名数组(D8)、缓冲设备地址的 7 个着色器(D23)、片元阶段 Wave/Quad(F23) | 转译成功率与失败原因 | 转译:Linux;编译:Mac |
| X2 | Mac 上用 MoltenVK 跑现有游戏(仓库记录中已有 M3 + MoltenVK 环境,F10 已有 `CAMetalLayer` 代码);统计每 pass 耗时、编码器数、Σraw 间接 draw 数(D19)、带宽(D13) | Apple GPU 上的热点;e1/e2 的实际收益;MoltenVK 缺口 | Mac |
| X3 | 按 `ImageLayout`、`PipelineStageFlags`、`ImageAspectFlags`、队列族、`AccessScope`、`NativeState` 统计受影响的测试函数 | P2、S4–S7 的测试修改清单 | Linux |
| X4 | **最小 iOS harness**:把 headless 夹具编译到 iOS,关闭音频,侧载内部开发资产,跑静态场景;记录帧时间、温度、内存 | 移动端瓶颈与预算 | iPhone;依赖 P0 ④ 的授权结论 |
| X5 | Metal 能力原型:第 6.2 节全部"待 X5"项 | 各决策依赖的能力是否成立;后端 trait 形状 | Mac + iPhone |
| X6 | 在 Linux 上对 Apple 目标做 `cargo check`(审查 B 未能完成:本机没装 Apple target) | D2 的 CI 是否需要 Mac | Linux |

**实验代码已准备好**,位置、运行状态和已得结果见 [`tools/metal_experiments/README.md`](../../tools/metal_experiments/README.md)。截至本版:X1a、X3 已在 Linux 上实测并出结果(F31、F32);X6 在本机为 n.a.(Apple 目标的标准库下载被网络拦截);X1b 的转译部分已实测(F33),Metal 编译一步未做;X2、X4、X5 的代码已写好但尚未运行,X4 的 harness 本身未写(阻塞项见其 README)。

## 6. 风险与 Metal 能力状态

### 6.1 风险

| 风险 | 等级 | 缓解 |
| --- | --- | --- |
| Metal 同步错误(偶发花屏) | 高 | D5 全不跟踪 + 三级基准 + 调试全跟踪模式 |
| 降级后 SPIR-V 中的别名数组无法转译 | 高 | X1 优先测;构建期拆数组的后备 |
| 移动端几何量过大 | 高 | D15 几何手段;预设对照先行 |
| ICB 性能劣于逐条编码,或部分设备不可用 | 中 | X5;推翻条件;最低设备 |
| 驻留遗漏 | 高 | 驻留集;地址缓冲单独登记;调试校验 |
| 迁移期间 Windows 退化 | 中 | 第 0.5 节门槛;每步像素对比 |
| P1–P4 工作量超出预期 | 中 | 按 S0 的依赖图重新估算;P0′ 并行 |
| 资产与 FMOD 授权 | 中(阻塞) | 提前到 P0 |
| PounceLight winit fork 不支持 iOS | 中 | X4 前验证 |

### 6.2 Metal 能力状态

"检索"表示审查 A 的二手来源(搜索摘要、Apple 论坛、开源项目),没有读到 Apple 原文。

| 能力 | 状态 | 用于 |
| --- | --- | --- |
| 多 draw 间接绘制 | 不存在(检索) | D19 |
| ICB 由 compute 填充;按 GPU 缓冲中的范围执行 | 存在(检索;按范围执行的 macOS 版本未核实);待 X5 | D19 |
| ICB 绑定纹理 | 不能,须经 argument buffer(检索) | D8、D19 |
| ICB 的 `inheritBuffers` 在 A12/A13 + iOS 18 上挂起 | 有报告(检索) | D14 |
| `MTLFence` 跨命令缓冲 | 语义有冲突(检索);不使用 | D5 |
| 串行 dispatch 保证顺序 | 社区来源(检索);待 X5 | D5 |
| 计算编码器内内存屏障 | 存在(检索);并发模式的收益待 X5 | D5 |
| 驻留集 | macOS 15 / iOS 18 起(检索);不提供 hazard 跟踪 | D22、D5 |
| 缓冲 GPU 地址 | macOS 13 / iOS 16 起,需要 Metal3(检索);A13 归属有冲突 | D23、D14 |
| placement heap 与纹理尺寸对齐查询 | macOS 10.15 / iOS 13 起(检索) | D6 |
| 无内存存储 | 存在;能否放进 heap 未核实;待 X5 | D6、D13 |
| 计数器采样 | Apple GPU 只支持阶段边界(检索);Metal 4 接口变化待 X5 | D18 |
| pipeline statistics | 不存在(审查 A、B 一致) | D18 |
| 快速数学默认开启 | 是(检索);开关选择待 X5 | D25 |
| 每阶段采样器上限 16,常量采样器也计入 | 是(检索) | D8 |
| argument buffer 中的采样器与资源上限 | 待 X5 | D8 |
| R32G32_UINT 颜色附件、B10G11R11F 存储写、R32_UINT→E5B9G9R9 视图、E5B9G9R9 3D 纹理、D32F_S8 仅模板视图、D32F 线性过滤、只读深度附件同时采样 | 部分检索、部分未核实;待 X5 | D24 |
| framebuffer fetch | 仅 Apple GPU;uint 格式未核实 | D13 c2(推迟) |
| 片元阶段 simd/quad 函数与 helper lane | 未核实;待 X1、X5 | D9 |
| 子组宽度与 lane 映射 | 未核实;待 X5 | D9 |
| 二进制归档兼容规则 | 未核实 | D10 |
| "最短时长后呈现"在 macOS 上的可用性 | 未核实 | D12 |
| iOS 后台 GPU 限制 | 属实(检索) | D17 |
| 并行编码多个命令缓冲的约束 | 未核实;待 X5 | D7 |

## 7. 移动端预算估算(审查 B,未测量)

方法:以 UHD 630 的隔离 pass 耗时为单位,按类别套用 D15 的削减,再按 GPU 算力比折算到 Apple GPU;持续负载按峰值 70% 计。算力与带宽数字来自审查 B 的已有了解,未核实。

| 类别 | UHD 630 原值 | 削减后(TGSR 0.5 倍内部分辨率) |
| --- | ---: | ---: |
| DOF | 22.0ms | 0 |
| 全分辨率类(光照、体积光、屏幕阴影等) | 29.2ms | 约 7.3ms(不含 TGSR 自身) |
| GI 类 | 22.4ms | 约 9ms |
| 几何 | 67.9ms | 约 42ms(假设顶点与片元各半;顶点占 30–70% 时为 34–57ms) |
| 其他与输出分辨率 | 13ms | 13ms |
| 合计 | — | 约 60–80ms UHD 630 当量 |

- 静态场景:折算到 A16 及以上,峰值约 16–27ms,持续负载约 22–38ms。30 FPS 勉强可行,A15 在边缘,60 FPS 不可行。D13 e1/e2 可再省 5–10ms(估算)。
- many_draws:几何约 303ms,约 70% 不随分辨率缩放,手机上约 60–90ms,不可接受。几何量需要再降 4–5 倍,这是 D15 几何手段要解决的问题。
- 这些数字的可信度很低,P0 的预设对照、X2、X4 之后要重算。

## 8. 与现有文档的关系

- [CROSS_BACKEND_API.md](CROSS_BACKEND_API.md):§4.3 的 `Device` 分发由 D2 取代;`StageMask` 由 D3 取代;§4.5 的资源表分档由 D8 取代(单一 argument buffer 模型);§7 的步骤由本文第 4 节取代。
- [METAL_INTEGRATION_AUDIT.md](METAL_INTEGRATION_AUDIT.md):第 1.3 节的测试影响由本文第 3.3 节取代;第 4 节的移动端铺垫由 D13、D15 取代。

## 9. 已验证与未验证

**已验证**:第 0.4 节的事实,来自读码、脚本统计和两位审查者的核对。其中影响最大的几条由我抽查过:F6(全管线 bindless 降级)、F16(只有 2 种 kind 走带计数路径)、F22(7 个着色器使用缓冲设备地址)、F25(空 pass)、F26(几何占比)、F27(GI 半分辨率)。

**未验证**:
- 本文没有实现任何代码,没有运行任何测试。
- 第 6.2 节标"检索"或"未核实"的全部 Metal 能力。
- 第 7 节全部预算数字;D13 e1/e2 的带宽收益。
- 第 3.3 节的测试数量来自审查者的统计,X3 之前不作为最终清单。

## 10. 审查记录

### 10.1 第 3 版采纳的审查意见

| # | 意见 | 来源 | 我的核实 | 处理 |
| --- | --- | --- | --- | --- |
| 1 | 全部管线运行时 bindless 降级,绑定模型认错 | B | 已核实 | D8 改为单一 argument buffer 模型;D9 输入改为降级后 SPIR-V;D22 驻留范围改为整个堆表 |
| 2 | 间接绘制有三种路径,D19 只覆盖带计数;带计数上限 8192 | A、B | 已核实 | D19 分路径 |
| 3 | ICB 不能绑纹理,与 D8 矛盾 | A | — | 由 D8 单一模型解决 |
| 4 | 缓冲设备地址影响 7 个着色器,含 2 个光栅 | A | 已核实 | D23、D14 最低设备 |
| 5 | S1 的模块直接用 `ash` 类型;漏统计 `rg`→`resource`/`tracking` | A、B | — | 第 3.4 节重排 S0–S7 |
| 6 | 漏掉跳过空 pass、合并同附件 pass、MSAA tile resolve | B(A 指出 MSAA 未覆盖) | 空 pass 已核实 | D13 e1–e3;P7 |
| 7 | 移动配置档无几何手段;GI 已是半分辨率 | B | 已核实 | D15 |
| 8 | 混合跟踪在全 bindless 下名存实亡 | A、B | — | D5 改为全部不跟踪 |
| 9 | 串行 dispatch 更安全 | A | — | D5 |
| 10 | `MTLFence` 跨命令缓冲语义未定 | A | — | D5 跨命令缓冲改用事件 |
| 11 | D6 与 D22 冲突 | B | — | D6 改为全部从大 heap 子分配 |
| 12 | StageMask 缺"全部" | A | — | D3 |
| 13 | 快速数学应只在验证构建全局关闭;缺半精度手段 | B | — | D25;D15 |
| 14 | 常量采样器也计入上限 | A | — | D8 |
| 15 | 片元阶段也用 Wave/Quad | A | — | F23;D9;X1 |
| 16 | 格式清单不全;0x7b 在用;深度线性过滤、仅模板视图、同读同绑 | A | — | F13;D24 |
| 17 | 缺顶点输入、推送常量、egui、截图、交换链 sRGB | A | — | D26、D27、D12 |
| 18 | country-core 有泄漏但无阶段负责 | A、B | — | F2;P3 |
| 19 | 转换 pass 应在图层声明 | A | — | D19 |
| 20 | ICB 可能比逐条编码更慢 | A | — | D19 推翻条件 |
| 21 | 性能门槛不可执行 | B | — | 第 0.5 节 |
| 22 | X4 在 P0 不可行;授权要提前 | A、B | — | X4 改为最小 iOS harness;P0 ④ |
| 23 | D2 与 B2 冲突 | A | — | D2 改为目标 + 特性选择 |
| 24 | 测试影响被高估 | B | — | 第 3.3 节 |
| 25 | 后端 trait 的录制面未盘点 | B | — | 第 3.5 节 |
| 26 | 内存预算无数字;Nani 固定容量缓冲 | B | — | D17 |
| 27 | 诊断方法在 Metal 上不能照搬 | B | — | D18 |
| 28 | 缺 CPU 移动预算 | B | — | D15 预算目标 |
| 29 | 快照要包含生产帧 | B | — | D4 |
| 30 | Metal 原型应与 P1–P3 并行 | B | — | P0′ |
| 31 | 事实表多处不准(F1–F21) | A | — | 第 0.4 节逐条修订 |

### 10.2 分歧与裁定

| 议题 | A 的意见 | B 的意见 | 裁定 |
| --- | --- | --- | --- |
| D7 分组与分块的顺序 | 已有 `split_allowed`,可直接实现"先分组后分块" | 长串 compute pass 会被禁止切开,伤害 8 块负载均衡;应先分块、块内分组 | 采纳 B:块边界切开组只多出微秒级开销,负载失衡会直接拉长已测量的 CPU 瓶颈 |
| D5 跟踪方式 | 跟踪类资源的正确性只依赖传输完成点 | 全部改为不跟踪 | 两者结论一致,采纳 B 的表述(全部不跟踪) |
| D19 ICB 的定位 | ICB 只用于带计数路径;注意性能可能劣于逐条编码 | ICB 只覆盖带计数路径,raw 路径先用 CPU 逐条编码 | 一致,合并采纳 |

### 10.3 未采纳或部分采纳

| 意见 | 来源 | 理由 |
| --- | --- | --- |
| 移动端以 X4 的 iPhone MoltenVK 测试为准 | 第 2 版自身 | X4 改为最小 iOS harness;MoltenVK 只在 Mac 上作 B2 基准 |
| 移动配置档进一步降低 GI 分辨率 | 第 2 版 D15 | GI 已是半分辨率,改为优先降低更新频率 |
