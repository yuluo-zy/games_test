# P1 详细设计:拆出后端中立的 `prism-gfx`

本文是 [METAL_DESIGN.md](METAL_DESIGN.md) 第 4 节 P1(依赖拆分)的详细设计,取代该文第 3.4 节的 S0–S7。依赖数字由 [`tools/rhapsody_core_split_deps.py`](../../tools/rhapsody_core_split_deps.py) 生成,每完成一步都要重跑。

## 1. 范围与验收

**范围**:把 `crates/rhapsody` 中不依赖 Vulkan 后端的代码移到新 crate `prism-gfx`。现有 `rhapsody` crate 成为"门面 + Vulkan 后端",对外导入路径不变。

**不在范围内**:Metal 后端(P5/P6);同步计划输出的中立化(P2);`resource` 的创建与描述拆分(第 6.2 节,另出设计);删除过渡别名(第 3.1 节)。

**每一步的验收**:
1. `cargo check` 全工作区通过;在本机(Linux)对 country-core 用 windows-gnu 目标做类型检查,tiny-glade 在 Windows 实机编译。
2. 现有测试结果不变:rhapsody 在本机 lavapipe 上运行;tiny-glade 在 Windows 实机运行。
3. **测试源码不改**,第 4 节逐步列出的例外除外。
4. 性能按 METAL_DESIGN.md 第 0.5 节的门槛判定(前提是 P0 已补齐基线)。
5. 新增 CI 检查(第 3.1 节)通过。

## 2. 量化现状

### 2.1 模块级依赖(非测试代码)

| 模块 | 行数 | `vk::` | `ash::` | 依赖的 crate 内模块 |
| --- | ---: | ---: | ---: | --- |
| `log_ring` | 205 | 0 | 0 | — |
| `profile` | 443 | 0 | 0 | — |
| `error` | 198 | 17 | 1 | — |
| `sync` | 635 | 21 | 13 | error |
| `accounting` | 1515 | 7 | 2 | sync |
| `collections` | 1310 | 0 | 0 | accounting |
| `retire` | 375 | 0 | 0 | accounting、sync |
| `tracking` | 2346 | 19 | 7 | sync、accounting、native(13)、collections、retire |
| `rg` | 27157 | 467 | 72 | tracking 122、resource 72、command 57、sync 55、native 54、device 44、…… |

两点结论:
- 第 3 版 S1 的说法不成立。完全不依赖 Vulkan 的只有 `log_ring` 和 `profile`。`collections` 依赖 `accounting`,`retire` 依赖 `accounting` 和 `sync`,而依赖链 `error → sync → accounting` 上每个模块都直接用了 `ash` 类型。
- `tracking` 对 `native` 的 13 处依赖全部是访问词汇(`AccessScope`、`ACCESS_TABLE`、`is_write`)和 `SubmitScratch`。

### 2.2 链上的 Vulkan 用法

| 模块 | 用到的 Vulkan 类型 | 位置 |
| --- | --- | --- |
| `error` | `Error::Vulkan(vk::Result)`;`From<vk::Result> for Error` | `error/mod.rs:53,171-191` |
| `sync` | `SyncSet` 的阶段字段 `vk::PipelineStageFlags` | `sync/mod.rs:105-162` |
| `sync/timeline` | `vk::Semaphore`;`impl TimelineApi for ash::Device` | `sync/timeline/mod.rs:10-60,116,168,323` |
| `accounting` | `vk::MemoryPropertyFlags`;`vk::MAX_MEMORY_HEAPS` | `accounting/stats/registry.rs`、`state.rs:24` |
| `tracking` | `AccessScope`(内嵌阶段与访问标志);`NativeState.layout: vk::ImageLayout` | `tracking/range_state/*`、`tracked/mod.rs:280` |

### 2.3 测试对这些类型的直接使用

| 类型 | 测试中的出现次数 | 文件数 | 这些文件中的测试数 |
| --- | ---: | ---: | ---: |
| `vk::PipelineStageFlags` | 107 | 18 | 104 |
| `vk::AccessFlags` | 61 | 15 | 89 |
| `vk::ImageLayout` | 113 | 17 | 104 |
| `AccessScope { … }` 字面量 | 23 | 11 | 66 |
| `vk::Result` | 69 | 6 | 59 |
| `Error::Vulkan` | 4 | 4 | 40 |
| `TimelineApi` 的假实现 | 4 | 4 | — |

前四类合计分布在 21 个测试文件里。只要把字段直接换成新类型,这些测试都要改。

上表是第一次手工统计。X3 脚本(`tools/metal_experiments/x3_test_impact.py`)的结果更完整:P1/P2 相关的全部类型族(含 `ImageAspectFlags`、队列族、`NativeState`、`vk::Semaphore`)会触及 **100 个测试文件、最多 643 个 `#[test]`**(rhapsody 493 个、tiny-glade 150 个、country-core 0 个);这是上界,因为同一文件里的测试不一定都用到该类型。K1 的目标就是让这些测试一行不改,这一点要在 S1/S2 的真实代码上验证。

### 2.4 `rg` 的污染源

在第 4 节 S2 移走访问词汇等项之后,按 `rg` 子模块计算传递依赖闭包:只有约 1.4k 行是干净的,约 2.5 万行被污染。污染几乎都来自两处:

| 污染源 | 位置 | 被污染的(直接或间接) |
| --- | --- | --- |
| 渲染回调类型绑定后端录制上下文 | `graph/declaration.rs:162`:`RenderCallback = Box<dyn for<'a> FnMut(executor::PassContext<'a>) -> bool + Send>`;`pass_builder.rs:385,400` 的回调参数 | `graph`,并经它污染 `plan`、`sync_plan`、`plan_check`、`resource_info`、`access_ledger`、`temporal`、`result_cache`、`subgraph`、`queue_schedule`、`receipt_storage`、`transaction`、`resource_contracts` 等 |
| 资源模块里的绑定辅助 | `rg/resource/mod.rs:503-600`:`Ref<'borrow, 'graph, Kind>` 持有 `&mut PassBuilder`,生成 `pass_bindings::PassBinding` | `resource`、`binding`、`frame_imports` |

外部对这几个名字的使用:

| 名字 | rg 非测试代码 | country-core + tiny-glade | rhapsody 测试 |
| --- | ---: | ---: | ---: |
| `RenderGraph` | 66 | 159 | 255 |
| `PassBuilder` | 11 | 43 | 0 |
| `BuiltGraph` | 18 | 5 | — |
| `RenderCallback` | 11 | 0 | — |
| `PassContext` | 25 | 7 | 59 |

## 3. 关键设计决策

### K1. 过渡期的"原生类型别名"

问题:core 的类型(`AccessScope`、`SyncSet`、`NativeState`、`Error` 等)目前内嵌 Vulkan 类型,测试直接用 Vulkan 值构造和断言它们(第 2.3 节)。

| 方案 | 做法 | 测试 | 风险 |
| --- | --- | --- | --- |
| a. 直接换成中立新类型 | `StageMask(u32)` 等,数值兼容 | 21 个文件约 280 处要改 | 改动大,但类型干净 |
| b. **按特性切换的类型别名** | core 有特性 `native-types`(依赖可选的 `ash`)。开启时,中立名字是 Vulkan 类型的别名,例如 `pub type StageMask = ash::vk::PipelineStageFlags`;关闭时,是 core 自己的数值兼容新类型,提供与 core 实际用到的同名常量和方法 | **不改** | 特性不可叠加:开与关时类型不同 |
| c. 泛型化 | core 的类型对"阶段类型"等泛型 | 不改 | 泛型参数扩散到全部跟踪与同步代码 |

**选择 b,作为过渡方案。**理由:
- 它是唯一能让 P1 不改测试的方案,并且 Vulkan 构建里的类型与现在完全相同,运行行为和性能都不可能变化。
- D3 已经选定"数值兼容":新类型的数值就是 Vulkan 编码,所以两种配置下的语义一致。
- 特性不可叠加的风险可控:按 METAL_DESIGN.md D2,每个二进制只有一个后端。Windows 与 macOS 的 MoltenVK 构建开启,Metal 构建关闭。

别名清单:

| 中立名字 | 开启 `native-types` 时 | 关闭时 |
| --- | --- | --- |
| `StageMask` | `vk::PipelineStageFlags` | `#[repr(transparent)] struct StageMask(u32)` |
| `AccessMask` | `vk::AccessFlags` | `struct AccessMask(u32)` |
| `ImageState` | `vk::ImageLayout` | `struct ImageState(i32)` |
| `AspectMask` | `vk::ImageAspectFlags` | `struct AspectMask(u32)` |
| `TextureUsage`、`BufferUsage` | `vk::ImageUsageFlags`、`vk::BufferUsageFlags` | `struct …(u32)` |
| `MemoryProperties` | `vk::MemoryPropertyFlags` | `struct MemoryProperties(u32)` |
| `SemaphoreHandle` | `vk::Semaphore` | `struct SemaphoreHandle(u64)` |
| `BackendResult` | `vk::Result` | `struct BackendResult(i32)` |

约束与检查(新增 CI):
1. core 中所有 `ash` 引用只能出现在 `native_types.rs` 一个文件里,用 grep 检查。
2. core、tracking 与 rg 的中立部分必须在**开、关两种配置下都能编译**。CI 对 core 分别跑 `cargo check` 和 `cargo check --features native-types`。两个配置都编译通过,就证明 core 只用到了两边共有的常量和方法。
3. 中立新类型的常量从 Vulkan 头文件的数值生成,附一个测试:开启特性时逐个比对中立常量与 `ash` 常量的数值。
4. Metal 后端 crate 只依赖 core 的共有子集。如果 macOS 上工作区同时编译两个后端导致特性被合并打开,Metal crate 也必须能编译;CI 在 macOS 上对 Metal crate 分别以两种配置检查。

退出条件:P4 完成、各测试已按 METAL_DESIGN.md 第 3.3 节的规则迁移到中立常量之后,删除别名,只保留新类型。删除时间另行决定,不在 P1 内。

### K2. 错误类型

`?` 运算符在 Vulkan 后端的大量 ash 调用上依赖 `From<vk::Result> for Error`。受孤儿规则限制,这个 `impl` 只能写在定义 `Error` 的 crate,也就是 core 里。

| 方案 | 做法 | 后端调用点 | 测试 |
| --- | --- | --- | --- |
| a. 后端定义自己的错误类型再转换 | — | 所有 ash 调用处要改 | 要改 |
| b. 扩展 trait `.vk()?` | — | 同上 | 要改 |
| c. **core 中按特性提供** | `Error::Vulkan(vk::Result)` 变体和 `From<vk::Result>` 只在开启 `native-types` 时存在;关闭时改为 `Error::Backend(BackendResult)` | 不改 | 不改 |

**选择 c。**`Error` 已经是 `#[non_exhaustive]`;`ErrorKind` 的分类不变,Metal 的错误也映射到同一套分类。`error/mod.rs` 内的 `match` 按特性写两个分支。这与 K1 是同一个特性。

### K3. 时间线

`TimelineApi` 已经是 trait(测试有 4 个假实现),只是签名里写死了 `vk::Semaphore`。

**选择:签名改为使用 `SemaphoreHandle`(K1 的别名)。**开启特性时它就是 `vk::Semaphore`,4 个测试假实现不用改;`Timelines<A>` 整体移入 core。`impl TimelineApi for ash::Device` 受孤儿规则限制,必须放在 core 里,放进 `native_types.rs`。Metal 后端以后实现自己的 `TimelineApi`,`SemaphoreHandle` 是 `MTLSharedEvent` 表的索引。

### K4. 渲染回调与录制上下文(`rg` 最大的切口)

| 方案 | 做法 | 热路径 | 外部代码 | core 内部改动 |
| --- | --- | --- | --- | --- |
| a. **按后端族泛型 + 门面别名** | core 定义 `pub trait RecordFamily: 'static { type Context<'a>; }`;`RenderGraph<F>`、`PassBuilder<'g, F>`、`BuiltGraph<F>`、`RenderCallback<F>` 对 `F` 泛型;门面写 `pub type RenderGraph = core::RenderGraph<VulkanFamily>`、`pub type PassContext<'a> = …` | 单态化,零开销 | 名字不变,159 + 255 处引用不改 | rg 内约 130 处签名机械添加 `<F>` |
| b. 抹掉录制上下文的类型 | `PassContext` 内放 `&mut dyn Recorder` | 每条命令一次虚调用(估算每帧约 10–20µs,未测) | 不变 | 中 |
| c. core 只存不透明回调(`Box<dyn Any>`),后端下转型 | `add_pass` 等方法改成后端提供的扩展 trait | 每 pass 一次下转型 | **调用处要导入扩展 trait**,游戏代码与测试要改 | 中 |

**选择 a。**理由:外部名字不变,运行时零开销,改动集中在 core 内部,并且是机械的。

注意点:
- 别名带生命周期参数时写成 `pub type PassBuilder<'g> = core::PassBuilder<'g, VulkanFamily>`。调用方写 `RenderGraph::new()` 照常可用。
- 测试中有 59 处使用 `PassContext`,如果有测试直接用结构体字面量构造它,别名同样可用;个别依赖具体字段的测试在 S3 用编译结果确认。
- `pass_builder.rs:437,444` 对 `pass_bindings::PassBindingError` 的引用:把这个错误枚举(不含后端类型)移入 core。

推翻条件:泛型参数蔓延到 `rg` 之外的公共签名,导致门面别名无法覆盖,则改用 b,并测量虚调用开销。

### K5. 跨 crate 可见性

移入 core 的约 104 个文件中有约 250 个 `pub(crate)` 项。按标识符粗略匹配,其中约 170 个被留在 rhapsody 的代码引用(这个数字有噪声)。

| 方案 | 做法 | 评价 |
| --- | --- | --- |
| a. **改为 `pub` 并加 `#[doc(hidden)]`** | 只改被外部引用的项,以编译错误为准 | 机械;门面不再导出这些项,游戏代码看不到 |
| b. 重新设计成正式接口 | — | 工作量大,不属于 P1 |

**选择 a。**门面只导出原来就公开的项,所以游戏侧可见的接口不变。

### K6. 跨 crate 的固有 impl 与孤儿规则

规则:类型的固有 `impl` 只能写在定义它的 crate;外部 trait 给外部类型的 `impl` 不能写在第三个 crate。

处理原则:类型和它的全部固有 `impl` 一起移动。统计脚本报告 `resource` 中有 10 处疑似跨 crate 固有 impl,核对后是同名类型误判(`rg::resource_registry` 中也有 `Buffer`、`TextureDesc`)。`resource` 本来就推迟到第 6.2 节,P1 不受影响。实际违规以编译错误为准。

### K7. 门面的重新导出

```rust
// crates/rhapsody/src/lib.rs(示意)
pub use prism_gfx::{accounting, collections, profile, sync, tracking};
pub use prism_gfx::{Error, ErrorKind, Result, SharedError};
```

- 模块级 `pub use` 让 rhapsody 内部原有的 `crate::sync::…` 路径继续解析,后端代码基本不用改导入。
- `internal_mod!` 暴露给测试的内部模块(`log_ring`、`retire`):core 也提供同名的 `test-support` 特性,门面的 `test-support` 转发给它。
- `rg` 跨两个 crate:门面的 `rg` 模块写 `pub use prism_gfx::rg::*;`,再声明留在后端的子模块(`vulkan`、`executor` 等)。

## 4. 步骤

每一步结束时都满足第 1 节的验收,然后重跑统计脚本。

### S1 基础层

| 项 | 内容 |
| --- | --- |
| 新建 | `crates/prism-gfx`,特性 `native-types = ["dep:ash"]`、`test-support`;`native_types.rs`(K1 别名、K2 的 `From`、K3 的 `impl TimelineApi for ash::Device`) |
| 移入 | `log_ring`、`profile`、`error`(K2)、`sync`(含 `timeline`,K3)、`accounting`(`MemoryProperties` 别名;`MAX_HEAPS` 改为字面量 16 并加数值断言)、`collections`(另加从 `native/submit/mod.rs:12-63` 移来的 `SubmitScratch`)、`retire` |
| 门面 | `rhapsody` 依赖 `prism-gfx`(开启 `native-types`),按 K7 重新导出 |
| 测试改动 | `profile` 中引用 `crate::rg` 的两处测试(`record_trace_tests.rs:125`、`command_workload/tests.rs:26`)移到 rhapsody 侧;其余不改 |
| 规模 | 约 4.7k 行 |

### S2 访问词汇、跟踪与 CPU 辅助

| 项 | 内容 |
| --- | --- |
| 移入 | `native/access_table.rs`(码表、`AccessScope`、`AccessSpec`、`is_write`;字段类型改为 `StageMask`、`AccessMask`、`ImageState` 别名);`native/resource_usage`(`image_usage_for_access`,用 `TextureUsage` 别名);从 `device/queue_config/mod.rs:61` 抽出 `QueueClass`;`command/cpu_arena`;`tracking` 整个模块(约 2.3k 行) |
| 门面 | `rhapsody::native::{AccessScope, AccessSpec, access_table, is_write}`、`rhapsody::device::QueueClass`、`rhapsody::command::{CpuArenaPool, CpuArenaLease}` 在原路径重新导出 |
| 测试改动 | 预期无:`AccessScope` 字面量和 `vk::` 常量在别名下照常编译 |
| 规模 | 约 3.5k 行 |

### S3 切开 `rg` 的两个污染源

| 项 | 内容 |
| --- | --- |
| K4 | 引入 `RecordFamily`;`RenderGraph`、`PassBuilder`、`BuiltGraph`、`RenderCallback` 泛型化;rhapsody 中定义 `VulkanFamily`(`Context<'a> = executor::PassContext<'a>`),门面导出别名 |
| 绑定辅助 | `rg/resource/mod.rs:503-600` 的 `Ref`、`BufferRef`、`TextureRef` 及其 `impl` 移到 `pass_builder` 旁边,和 `PassBuilder` 一起泛型化;`PassBindingError` 移入 core |
| `pass_bindings` | 只有 `pass_bindings/mod.rs:10` 引用 `device::Device`。把不含 `Device` 的类型(`PassBinding` 等)移入 core,含 `Device` 的部分(`ManagedPassResources` 的创建)留在后端。具体切分在动手时按编译结果确定 |
| 这一步不移动文件 | 只切依赖,结束后重跑脚本 |
| 测试改动 | 预期无;若有测试依赖 `PassContext` 的具体字段,逐个列出评审 |

### S4 移入 `rg` 的中立部分

S3 之后重跑脚本,预期闭包干净、可以移入 core 的子模块:`compiled`、`frame`、`frame_storage`、`frame_throttle`、`graph_caches`、`import_export`、`indirect_count_spirv`、`resource_registry`、`transient_memory`、`transient_memory_check`(这些现在已经干净),以及切开后应变干净的 `graph`、`plan`、`sync_plan`、`plan_check`、`access_ledger`、`resource`、`resource_info`、`resource_contracts`、`result_cache`、`subgraph`、`temporal`、`queue_schedule`、`receipt_storage`、`transaction`、`binding`、`frame_imports`、`pass_builder`、`parallel`(`parallel` 对 `executor` 的 1 处引用需要在 S3 一并确认)。

**以 S3 后的脚本输出为准**,不以这里的预期为准。

留在 rhapsody(Vulkan 后端)的 `rg` 子模块:`vulkan`(5.3k 行)、`executor`、`frame_loop`(依赖 `present`)、`transient_resources`(依赖 `device`)、`attachments`、`pass_api`、`pass_bindings` 的后端部分、`profiling`、`bindless_binding`、`indirect_count`、`owner_storage`、`recorder`、`transfer_import`,合计约 12.6k 行。

预期移入 core 的 `rg` 代码约 14–15k 行(27.2k 减去上一段)。

### S5 收尾

- 运行 CI 新增检查(K1 第 1–3 条)。
- 门面导出清单与拆分前的公开 API 对比:用 `cargo public-api` 或等价方法,在拆分前后各生成一次公开项清单,差异必须为零(`#[doc(hidden)]` 项除外)。
- 重跑全部测试与性能门槛。

## 5. P1 结束时的预期

| crate | 内容 | 预期行数 |
| --- | --- | ---: |
| `prism-gfx` | 基础层、访问词汇、跟踪、`rg` 中立部分 | 约 22–23k |
| `rhapsody`(门面 + Vulkan) | 其余全部 | 约 45k |

这些行数由第 2 节的统计推算,不是测量结果。

## 6. 风险与后续

### 6.1 风险

| 风险 | 影响 | 缓解 |
| --- | --- | --- |
| K1 的特性不可叠加,在 macOS 同时编译两个后端时被合并打开 | Metal crate 看到的是 Vulkan 类型 | Metal crate 只用共有子集;CI 双配置检查 |
| K4 泛型的编译时间与错误信息变复杂 | 开发体验 | 每个二进制只有一个实例化;门面别名隐藏泛型 |
| `pub(crate)` 改 `pub` 让内部项可见 | 意外被游戏代码依赖 | `#[doc(hidden)]`;门面不导出;公开 API 清单对比 |
| 拆 crate 改变内联与代码生成 | CPU 性能 | 第 0.5 节门槛;release 已是 thin LTO + `codegen-units=1`,预期影响小,以实测为准 |
| 统计脚本是文本匹配 | 漏判或误判 | 每一步以编译结果为准;脚本只用于规划 |

### 6.2 后续设计(不在 P1)

- `resource` 拆分:描述类型和托管句柄进 core,创建、读写留后端。`Buffer`、`Texture` 的固有 `impl` 分布在 `create.rs`、`read.rs`、`write.rs`、`host.rs` 等多个文件,需要单独设计。
- P2 同步计划中立化:有了 K1 的别名,P2 的重点从"换类型"变为"让 Metal 降级能直接读 `StageMask`/`ImageState` 的数值语义"。
- 删除过渡别名(K1 的退出条件)。
- METAL_DESIGN.md 实验 X3 的定位相应调整:P1 不需要它,删除别名之前才需要。

## 7. 已验证与未验证

**已验证**:第 2 节的数字由脚本统计(可复现);第 2.2 节的位置都读过源码;`SubmitScratch` 不含 Vulkan 类型;`QueueClass` 是不含 Vulkan 的枚举;污染源与外部引用次数由 grep 统计。

**原型验证(在 /tmp 下的独立工作区,未入库)**:用三个小 crate(与 `ash` 同形的替身类型、core、门面)验证了:
- K1:core 在开、关 `native-types` 两种配置下都能编译,共享代码同时适用两种配置;开启时中立名字就是替身的 Vulkan 类型本身,测试可直接用 Vulkan 常量断言。
- K2:按特性存在的 `Error::Vulkan` 变体与 `From<vk::Result>`,`?` 可用。
- K4:`RenderGraph<F>`、`PassBuilder<'g, F>`、`RenderCallback<F>` 对带 GAT 的 `RecordFamily` 泛型;门面别名 `RenderGraph`、`PassBuilder<'g>`、`PassContext<'a>` 下,`RenderGraph::new()`、`add_pass(..).render_with_context(|ctx| ..)` 的闭包推断、显式标注 `PassContext<'_>` 都能编译并运行通过。
原型用的是替身类型,不是真实的 `ash`;真实代码中的边角(例如 `ash` 标志类型的全部常量、带 `Send` 约束的回调与 rhapsody 现有并行录制的组合)以 S1、S3 的编译结果为准。

**未验证**:
- 没有在真实代码上实现任何一步。
- `pub(crate)` 中实际需要提升的项数。
- S4 的预期移动清单,以 S3 后重跑脚本的结果为准。
- 第 5 节的行数。

## 8. 实施记录:WP2(S1)

分支 `codex/gfx-core-split`,提交 `262cd9c4`(骨架)到 `91445d44`(步骤 6)。已移入 `prism-gfx`:`trace`、`log_ring`、`error`、`sync`、`accounting`、`collections`、`retire`,非测试代码约 4.6k 行、测试约 2.0k 行。`rhapsody/src` 净减约 6.4k 行。**`profile` 没有移动**(见偏差 D2)。

每一步的验收都是:`prism-gfx` 在开、关 `native-types` 两种配置下 `cargo check` 通过且无警告;`rhapsody` 与 `country-core`(windows-gnu 类型检查)通过;`tools/rhapsody_test_baseline.py compare` 对 `baseline/test_results.tsv` 无回归、无消失。最终:958 个测试,944 ok、2 FAILED(基线里就有,校验层缺失)、12 ignored;比基线多的 1 个是 `native_types` 的数值测试。

### 与设计和计划不一致的地方(都已在代码里处理,逐条列出)

| # | 设计/计划的说法 | 实际情况 | 处理 |
| --- | --- | --- | --- |
| D1 | 步骤顺序 `collections`、`retire` 在前 | 它们依赖 `accounting`、`sync`,后者依赖 `error`:顺序必须是 `error → sync → accounting → collections/retire` | 按依赖顺序移动;`trace` 因被 `sync/timeline` 使用而一起移 |
| D2 | `profile` 在 S1 内移动,其两处 `crate::rg` 测试移到 rhapsody | 这两处测试用 `super::*` 访问 `profile` 的私有内部,不能搬出 crate;而本批次移动的模块没有一个使用 `profile` | `profile` 留到 S4 随 `rg` 一起移动,届时测试自然同处一个 crate;S1 因此没有改任何测试逻辑 |
| D3 | `SubmitScratch` 随 `collections` 移动 | 它只被 `tracking` 使用 | 留给 S2 |
| D4 | K2:`Error` 已是 `#[non_exhaustive]`,没有影响 | 跨 crate 后 `non_exhaustive` 会强制 `_ =>` 通配符;rhapsody 有两处故意写成穷举(`command_buffer/retention.rs`、`transfer/queue/mod.rs`),新增变体时靠编译错误提醒 | **去掉了 `Error` 上的 `#[non_exhaustive]`**,保住这道保险。代价:新增变体会让下游匹配也报错;下游都在本工作区。**这是个 API 约定层面的决定,可以否决** |
| D5 | 未提及特性 | `accounting` 里有三处受 `suballocation` 特性控制的代码;核心不声明该特性就会**永远被编译掉** | 核心声明 `suballocation`,rhapsody 的同名特性转发给它。教训:被移动的代码引用的每个特性,核心都必须声明并转发 |
| D6 | K6 预告了孤儿规则 | 实际遇到 3 处:`impl Evictable for parking_lot::Mutex<Staging>`;`impl CpuPoolValue for Arc<ParameterCpuBlock>`;`impl CpuPoolValue for Arc<ReceiptSlot>` | 第一处用本地新类型 `StagingLock`(`Deref` 到 mutex,`.lock()` 调用点和 `Arc` 所有权不变)。后两处在核心加 `ArcPoolValue` trait 和对 `Arc<T>` 的泛型 `CpuPoolValue` 实现,rhapsody 为自己的本地类型实现 `ArcPoolValue`,方法体原样迁移,池的类型不变 |
| D7 | K5:约 250 个 `pub(crate)` 项,约 170 个被引用 | 实际放开 73 个(`accounting` 30、`collections`/`retire` 43),全部 `pub` + `#[doc(hidden)]` | 同时让 `rhapsody::sync::timeline`、`collections::cache` 等从 crate 私有变成对文档隐藏的公开 |
| D8 | 宏 `atomic_snapshot!` 没有提到 | 跨 crate 调用需要 `#[macro_export]` | 导出为 `__atomic_snapshot` 并在原路径重新导出;宏内部通过 `$crate` 引用自己 |
| D9 | 未提及 | 测试里直接写 `vk::` 的模块在关闭 `native-types` 时编译不过 | 这些测试模块只在 `native-types` 下编译;`error/tests.rs`、`sync/timeline/tests.rs` 各加了一行 `use ash::vk;`(原来靠 `use super::*` 间接得到) |

### 改动的测试

只有两类,且断言一律没变:`error/tests.rs` 和 `sync/timeline/tests.rs` 各加一行 `use ash::vk;`;若干测试模块的声明加上 `cfg(feature = "native-types")`。没有测试被删除或改写断言。

### 工作中暴露的工具问题

- `cargo test` 不带 `--no-fail-fast` 会在第一个失败的二进制处停下,漏掉后面的测试;`cargo` 的标准输出与标准错误不能分开读,否则测试会归到错误的二进制名下(见 `baseline/BASELINE.md`)。
- 每次改动,rhapsody 的 95 个集成测试都会重新链接,旧可执行文件留在 `target/debug/deps`;加上增量缓存,这台机器的磁盘被填满两次。处理:构建时设 `CARGO_INCREMENTAL=0`,定期运行 `tools/clean_superseded_test_binaries.py`(每个测试名只保留最新的一份,默认只报告)。

### 仍然没有验证的

- `tiny-glade` 在 Windows 上编译、`country-core` 与 `tiny-glade` 的测试(本机只能做 `country-core` 的类型检查)。
- 性能:拆 crate 会改变内联与代码生成,release 是 thin LTO + `codegen-units=1`,预期影响小,但**没有测量**。合入主线前要按 METAL_DESIGN.md 第 0.5 节的门槛在 Windows 实机上比较。
- K1 的关闭配置只验证了编译与 63 个不依赖 `ash` 的测试;错误映射、同步集合等需要 Vulkan 类型的行为只在开启配置下测试。
