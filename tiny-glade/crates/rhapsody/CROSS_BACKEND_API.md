# 跨后端公共 API 定义(草案 1)

> 修订说明:本文是初稿,多处已被 [METAL_DESIGN.md](METAL_DESIGN.md) 第 3 版取代:§4.3 的 `Device` 分发(D2)、`StageMask`(D3,增加"全部"位)、§4.5 资源表分档(D8:全部管线已在运行时降级为 bindless,Metal 侧采用单一 argument buffer 模型)、§5 泄漏点(country-core 同样有泄漏,见 METAL_DESIGN.md F2)、§7 步骤(第 3.4 节与第 4 节)。E6 的格式事实已更正。冲突时以 METAL_DESIGN.md 为准。

范围:Windows(Vulkan)、macOS 与 iOS(Metal)。Android 暂缓,接口不为它预留专门类型,但不阻止以后加。

本文回答三件事:现有代码里**哪些能复用**、**哪些需要抽象**、**公共 API 应该是什么**。第 4 节的接口签名已在独立 crate 中通过 `cargo check`,只验证了类型自洽,没有任何实现。

## 1. 结论

1. 游戏侧(`country-core`)与渲染框架之间**已经存在一层边界**,且大部分是后端中立的。不需要凭空设计新边界,需要做的是把残留的 Vulkan 类型从这层边界上拿掉。
2. rhapsody 约 6.8 万行非测试代码里,**33% 的代码(22.4k 行)完全不含 `vk::`/`ash::` 引用**,可以原样复用;60%(40.7k 行)引用密集,需要抽象或按后端重写。
3. 渲染图的"访问声明"用的是 u8 访问码词汇表,不是裸 Vulkan 标志。这是最好的抽象点:码值保持不变,只把码到 Vulkan 屏障的映射表,换成每个后端各自的降级实现。

## 2. 现状证据

以下均来自读代码与脚本统计,位置给出文件。

| 编号 | 事实 | 位置 |
| --- | --- | --- |
| E1 | 游戏侧已定义 10 个以上边界 trait:`RenderHost`、`PassHost`、`ShaderBackend`、ssbo 的 `GraphicsDevice`、`TextureDevice`、`rg_helpers::RenderGraph`/`RenderPassApi` 等。资源用 `u32` 编号,访问用 `u8` 码,pass 提交用 `ComputePassDesc`/`RasterPassDesc` | `country-core/src/render/*.rs`、`systems/render_loop/render_frame.rs`、`painterly.rs` |
| E2 | 这些 trait 由 `tiny-glade` 的宿主层实现,约 1.5 万行(`render_host.rs` 6460、`raster_host.rs` 4614、`production_renderer.rs` 2959 等) | `crates/tiny-glade/src/` |
| E3 | 渲染图声明层用不透明的 `u64` token 标识资源,后端通过 `ResourceBackend`/`ExecutionBackend` 两个 trait 接入 | `rhapsody/src/rg/graph/backend.rs`、`rg/executor/backend_api.rs` |
| E4 | 访问词汇是 64 个 u8 码(顺序跟随 `vk-sync` 的 AccessType),经 `ACCESS_TABLE` 降级为 (stages, access, layout)。游戏直接写码值,如 `0x34` = TransferWrite | `rhapsody/src/native/access_table.rs` |
| E5 | 已有类型化的 `Command` 枚举和 `CommandBackend` trait,这是现成的录制缝,但枚举里用的是 `vk::Buffer`、`vk::RenderingInfo` 等 | `rhapsody/src/rg/pass_api/mod.rs` |
| E6 | 游戏里写的格式值就是 Vulkan 格式编号(`GBUFFER=0x65`=R32G32_UINT、`COLOR_HDR=0x7a`=B10G11R11_UFLOAT、`VELOCITY=0x61`=R16G16B16A16_SFLOAT)。游戏代码不写 `vk::Format` 类型名,但数值是 Vulkan 的。(初稿误写为"原版私有编号",审查时更正) | `country-core/.../render_world.rs:171` |
| E7 | 缓冲和纹理的 usage 数值与 Vulkan 位相同(缓冲 0x2/0x10/0x20/0x40/0x80,纹理 sampled|storage = 0xc),但只用到少数几位 | `country-core/src/render/ssbo.rs:31`、`render_world.rs:183` |
| E8 | 着色器侧已是中立的:`shader-pipeline`(812 行)与 `shader-compiler`(2090 行)只依赖 SPIR-V 工具库,各只有 2 处 `vk` 引用 | `rhapsody/crates/` |
| E9 | 已有间接绘制计数的适配器 `IndexedCountAdapter`,可作为不支持 `draw_indirect_count` 时的回退入口。**我没有读它的实现,不确定它的回退方式** | `rg/indirect_count.rs` |
| E10 | `country-core` 里直接出现 `vk::` 的只有 35 处/14 个文件,集中在 `rhapsody_contract.rs`(命令缓冲、缓冲句柄、设备地址、内存预算)和格式的调试输出 | `country-core/src/` |
| E11 | `tiny-glade` 宿主层的 `vk::` 引用集中在转换类:`BufferUsageFlags` 31 处、`ImageLayout` 19、`ImageUsageFlags` 11、`Format` 10、`CommandBuffer` 6、`BufferImageCopy` 5、`BufferCopy` 4。多数是机械转换 | `render_host.rs`、`raster_host.rs` |

## 3. 复用 / 抽象 / 重写分类

统计口径:rhapsody `src/` 下 276 个非测试 `.rs` 文件,共 67,670 行。"零引用"指文件内没有 `vk::` 或 `ash::`。

| 模块 | 总行数 | 零引用行数 | 分类 | 说明 |
| --- | ---: | ---: | --- | --- |
| collections | 1336 | 1336 | **复用** | 全部后端无关 |
| retire、log_ring、profile | 1023 | 1023 | **复用** | 完成回收、日志环、命令工作量统计 |
| accounting | 1515 | 1065 | **复用** | 预算与统计;少量引用需清理 |
| error | 198 | 0 | **复用**(去掉 `vk::Result` 转换) | `Error`/`ErrorKind` 保持不变 |
| rg(渲染图) | 27222 | 11950 | **拆分** | 见下表三组 |
| tracking | 2346 | 755 | **抽象** | 队列族、所有权范围用到 Vulkan 队列族编号 |
| sync | 635 | 76 | **抽象** | `QueueId`、`SyncPoint` 已中立;`Timelines` 绑定 Vulkan 信号量 |
| format | 1770 | 0 | **抽象** | 格式属性表改为中立,原生格式列按后端提供 |
| resource | 4145 | 853 | **抽象** | `TextureDesc`/`BufferDesc` 改中立类型;创建与分配按后端 |
| bindless | 1960 | 707 | **抽象** | 表分三档,见 §4.5 |
| pipeline | 4259 | 1226 | **抽象 + 后端** | 反射、请求队列、缓存键中立;管线创建与磁盘缓存按后端 |
| transfer | 4077 | 1112 | **抽象 + 后端** | 上传队列与票据逻辑中立;拷贝录制按后端 |
| device | 5643 | 907 | **后端** | 设备选择、队列、内存分配;选择策略有一部分可复用 |
| command | 5570 | 434 | **后端** | 命令池与录制 |
| present | 3059 | 106 | **后端** | 交换链与呈现 |
| native | 2772 | 691 | **后端** | Vulkan 实例、表面、物理设备 |

`rg` 内部的拆分(按子目录统计,不含测试文件;分组依据是 `vk::` 密度加读到的代码,**第一步审计要逐文件确认**):

| 分组 | 子目录/文件 | 行数 | `vk::`/`ash::` 引用 | 分类 |
| --- | --- | ---: | ---: | --- |
| A | `graph`、`plan`、`compiled`、`graph_caches`、`result_cache`、`temporal`、`subgraph`、`frame*`、`parallel`、`resource_registry`、`resource_info`、`pass_bindings`、`pass_builder`、`access_ledger`、`queue_schedule`、`executor`、`transient_memory*`、`indirect_count*`、`recorder`、`owner_storage`、`receipt_storage`、`import_export`、`resource`、`resource_contracts` 等 | 13,843 | 81 | **复用**(少量引用清理) |
| B | `sync_plan`、`plan_check`、`attachments`、`pass_api`、`profiling`、`transient_resources` | 7,843 | 334 | **抽象**(屏障推导与校验用到 Vulkan 的阶段/布局,改为中立 `AccessInfo`,降级交给后端) |
| C | `vulkan`(`VulkanGraphBackend`、`VulkanGraphSession`、呈现、命令捕获等) | 5,134 | 84 | **后端** |

`test_image_copy`(约 300 行,仅测试支持)未计入三组。

整体数字的含义:约三分之一的文件完全不含 Vulkan 引用;`rg` 里有近一半行数属于几乎无引用的分组 A。需要类型替换的部分集中在 `rg` 分组 B、`resource`、`format`、`bindless`、`pipeline`、`transfer` 的中立部分;按后端重写的是 `device`、`command`、`present`、`native` 与 `rg/vulkan`。这是按引用密度做的分类,不是工作量承诺。

## 4. 公共 API 定义

### 4.1 设计规则

1. **码值不变**:`AccessCode`、`Format`、usage 位的数值与现有游戏代码里的字面量一致(这些数值本身就是 Vulkan 编码),游戏侧不改数字。
2. **接口上不出现原生类型**:没有 `vk::*`、没有 Metal 类型。原生访问只允许在后端 crate 内部。
3. **能力是数据**:设备能力用 `DeviceCaps` 描述,调用方按能力选路径,不按平台名判断。
4. **热路径不逐调用虚分发**:录制通过 `CommandSink::encode_all` 批量提交。该设计对 CPU 渲染阶段(基线均值 9.57ms)的影响**未测量**,必须在 §7 的验收里量。
5. **pass 合并是一等概念**:`RenderingDesc::merge_group` 让同组 pass 可作为 tile 内连续序列编码;不支持合并的后端自动拆开。

### 4.2 crate 划分

| crate | 内容 |
| --- | --- |
| `prism-gfx` | 词汇类型、渲染图声明/计划/编译/缓存、访问词汇、能力、错误、统计、回收(§3 中"复用"与抽象后的中立部分) |
| `rhapsody-rhi` | 后端 trait:`DeviceBackend`、`SwapchainBackend`、`PipelineBackend`、`ResourceTable`、`CommandSink`、`AccessLowering` |
| `rhapsody-vulkan` | Vulkan 实现与访问码到屏障的降级表(现有代码迁入) |
| `rhapsody-metal` | Metal 实现与降级(新写) |
| `rhapsody`(门面) | 重新导出,保持现有 `rhapsody::rg::...`、`rhapsody::resource::...` 路径,使 `country-core`、`tiny-glade` 少改动 |

### 4.3 签名(已通过独立 crate `cargo check`)

```rust
//! Cross-backend API signatures (type-check only; no implementation).
#![allow(dead_code)]
use std::sync::Arc;

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug)]
pub struct Error;

// ===================== vocabulary (prism-gfx) =====================
/// Vulkan format number as written by game code (e.g. 0x65 R32G32_UINT, 0x7a B10G11R11_UFLOAT). Backends own the native mapping.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct Format(pub u16);

macro_rules! flags { ($n:ident, $($c:ident = $v:expr),+) => {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
    #[repr(transparent)]
    pub struct $n(pub u32);
    impl $n { $(pub const $c: Self = Self($v);)+
        pub const fn contains(self, o: Self) -> bool { self.0 & o.0 == o.0 }
        pub const fn union(self, o: Self) -> Self { Self(self.0 | o.0) } }
}}
// Numeric values deliberately equal the Vulkan bits already used by game code (0x2, 0x10, 0x20, 0xc ...).
flags!(TextureUsage, TRANSFER_SRC = 0x1, TRANSFER_DST = 0x2, SAMPLED = 0x4, STORAGE = 0x8,
       COLOR_ATTACHMENT = 0x10, DEPTH_STENCIL = 0x20, TRANSIENT = 0x40, INPUT_ATTACHMENT = 0x80);
flags!(BufferUsage, TRANSFER_SRC = 0x1, TRANSFER_DST = 0x2, UNIFORM = 0x10, STORAGE = 0x20,
       INDEX = 0x40, VERTEX = 0x80, INDIRECT = 0x100, DEVICE_ADDRESS = 0x2_0000);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MemoryLocation { GpuOnly, CpuToGpu, GpuToCpu }
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TextureKind { D1, D2, D3, Cube }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureDesc {
    pub usage: TextureUsage, pub format: Format,
    pub width: u32, pub height: u32, pub depth: u32, pub array_elements: u32,
    pub mip_levels: u8, pub sample_count: u8, pub kind: TextureKind,
    pub mutable_format: bool,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BufferDesc { pub size: u64, pub usage: BufferUsage, pub alignment: u16, pub memory: MemoryLocation }

/// u8 access code. Numeric values are the existing 64-code vocabulary (game passes e.g. 0x34 TransferWrite).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct AccessCode(pub u8);
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ImageState { Undefined, General, ShaderRead, ColorAttachment, DepthStencilAttachment,
    DepthReadOnly, TransferSrc, TransferDst, Present }
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StageMask(pub u16);
impl StageMask { pub const VERTEX: Self = Self(1); pub const FRAGMENT: Self = Self(2);
    pub const COMPUTE: Self = Self(4); pub const TRANSFER: Self = Self(8); pub const INDIRECT: Self = Self(16);
    pub const HOST: Self = Self(32); pub const ATTACHMENT_OUTPUT: Self = Self(64); pub const PRESENT: Self = Self(128); }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AccessInfo { pub stages: StageMask, pub write: bool, pub image_state: ImageState }
/// Backend-neutral meaning of each access code; replaces `native::access_table` (vk stage/access/layout).
pub fn access_info(_code: AccessCode) -> Option<AccessInfo> { None }

// ===================== capabilities =====================
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemoryModel { Discrete, Unified }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResourceTableTier { Large, Paged, PerPass }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IndirectCountMode { Native, Emulated, Unsupported }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceClass { Software, Integrated, Discrete, Mobile }
#[derive(Clone, Debug)]
pub struct DeviceCaps {
    pub class: DeviceClass, pub memory_model: MemoryModel,
    pub resource_table: ResourceTableTier, pub resource_table_slots: u32,
    pub indirect_count: IndirectCountMode, pub max_draw_indirect_count: u32,
    pub subgroup_size: Option<(u32, u32)>, pub tile_memory: bool,
    pub block_compression: BlockCompression, pub timeline_sync: bool,
    pub memory_budget_bytes: Option<u64>,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BlockCompression { pub bc: bool, pub astc: bool }

// ===================== handles =====================
macro_rules! id { ($($n:ident),+) => {$(
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)] pub struct $n(pub u64);)+ } }
id!(BufferId, TextureId, ViewId, SamplerId, PipelineId, LayoutId);

// ===================== commands (hot path) =====================
#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum LoadOp { Load, Clear, DontCare }
#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum StoreOp { Store, DontCare }
#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum IndexType { U16, U32 }
#[derive(Clone, Copy, Debug)]
pub struct AttachmentDesc { pub view: ViewId, pub load: LoadOp, pub store: StoreOp, pub clear: [f32; 4] }
#[derive(Clone, Copy, Debug)]
pub struct Rect { pub x: i32, pub y: i32, pub w: u32, pub h: u32 }
#[derive(Clone, Copy, Debug)]
pub struct Viewport { pub x: f32, pub y: f32, pub w: f32, pub h: f32, pub min_depth: f32, pub max_depth: f32 }
/// `merge_group`: passes with the same group may be encoded as one tile-resident sequence
/// (Metal tile memory / Vulkan subpass); a backend that cannot merge splits them.
pub struct RenderingDesc<'a> {
    pub area: Rect, pub colors: &'a [AttachmentDesc], pub depth: Option<AttachmentDesc>,
    pub merge_group: Option<u32>,
}
pub struct BufferCopy { pub src: u64, pub dst: u64, pub size: u64 }
pub enum Command<'a> {
    BeginRendering(&'a RenderingDesc<'a>), EndRendering,
    SetViewport(Viewport), SetScissor(Rect), SetStencilRef(u8),
    BindPipeline(PipelineId),
    BindVertexBuffer { binding: u32, buffer: BufferId },
    BindIndexBuffer { buffer: BufferId, ty: IndexType },
    PushConstants { layout: LayoutId, bytes: &'a [u8] },
    CopyBuffer { src: BufferId, dst: BufferId, regions: &'a [BufferCopy] },
    FillBuffer { buffer: BufferId, offset: u64, size: u64, value: u32 },
    ClearColor { texture: TextureId },
    Draw { vertices: u32, instances: u32, first_vertex: u32, first_instance: u32 },
    DrawIndexed { indices: u32, instances: u32, first_index: u32, vertex_offset: i32, first_instance: u32 },
    DrawIndirect { buffer: BufferId, offset: u64, count: u32, stride: u32 },
    DrawIndirectCount { buffer: BufferId, offset: u64, count_buffer: BufferId, count_offset: u64, max: u32, stride: u32 },
    DrawIndexedIndirect { buffer: BufferId, offset: u64, count: u32, stride: u32 },
    DrawIndexedIndirectCount { buffer: BufferId, offset: u64, count_buffer: BufferId, count_offset: u64, max: u32, stride: u32 },
    Dispatch { x: u32, y: u32, z: u32 }, DispatchIndirect { buffer: BufferId, offset: u64 },
}
/// One per recording thread; replaces `CommandBackend::dispatch(vk::CommandBuffer, Command)`.
pub trait CommandSink {
    fn encode(&mut self, command: Command<'_>);
    /// Batch entry point: lets a backend avoid one dynamic call per draw.
    fn encode_all(&mut self, commands: &[Command<'_>]) { let _ = commands; }
}

// ===================== device =====================
pub trait DeviceBackend: Send + Sync {
    fn caps(&self) -> &DeviceCaps;
    fn create_buffer(&self, desc: &BufferDesc, name: &str, data: Option<&[u8]>) -> Result<BufferId>;
    fn create_texture(&self, desc: &TextureDesc, name: &str, data: Option<&[u8]>) -> Result<TextureId>;
    fn release_buffer(&self, id: BufferId);
    fn release_texture(&self, id: TextureId);
    fn write_buffer(&self, id: BufferId, offset: u64, data: &[u8]) -> Result<()>;
    fn device_address(&self, id: BufferId) -> Option<u64>;
    fn wait_idle(&self) -> Result<()>;
    fn is_lost(&self) -> bool;
}
/// Concrete handle the game keeps (`Arc<Device>` today); dispatches to the selected backend.
#[derive(Clone)]
pub struct Device { inner: Arc<dyn DeviceBackend> }
impl Device { pub fn caps(&self) -> &DeviceCaps { self.inner.caps() } }

// ===================== present =====================
pub enum SurfaceSource {
    Win32 { hwnd: *mut (), hinstance: *mut () },
    MetalLayer { layer: *mut () },
    Xlib { display: *mut (), window: u64 }, Xcb { connection: *mut (), window: u32 },
    Wayland { display: *mut (), surface: *mut () },
}
pub struct SwapchainInfo { pub format: Format, pub extent: [u32; 2], pub image_count: u32 }
pub trait SwapchainBackend: Send {
    fn info(&self) -> SwapchainInfo;
    fn acquire(&mut self) -> Result<Option<(u32, TextureId)>>;
    fn present(&mut self, image: u32) -> Result<()>;
    fn resize(&mut self, extent: [u32; 2]) -> Result<()>;
}

// ===================== pipelines =====================
pub struct ShaderArtifact { pub spirv: Vec<u32>, pub entry: String }
pub trait PipelineBackend: Send + Sync {
    fn create_compute(&self, name: &str, shader: &ShaderArtifact) -> Result<PipelineId>;
    fn create_raster(&self, name: &str, vs: &ShaderArtifact, fs: &ShaderArtifact, key: &[u8]) -> Result<PipelineId>;
}

// ===================== resource table (bindless) =====================
pub trait ResourceTable: Send + Sync {
    fn tier(&self) -> ResourceTableTier;
    fn capacity(&self) -> u32;
    fn register_texture(&self, view: ViewId) -> Result<u32>;
    fn register_buffer(&self, buffer: BufferId) -> Result<u32>;
    fn release(&self, slot: u32);
}

// ===================== lowering =====================
/// What each backend supplies to the shared graph compiler (replaces vk barrier building in sync_plan).
pub trait AccessLowering {
    type Dependency;
    fn lower(&self, from: AccessInfo, to: AccessInfo, texture: bool) -> Self::Dependency;
}
```

### 4.4 与现有名字的对应

| 新 | 旧 | 备注 |
| --- | --- | --- |
| `Format(u16)` | `vk::Format` | 数值就是 Vulkan 格式编号;Metal 后端用静态表映射到像素格式(见 METAL_DESIGN.md D24) |
| `TextureUsage`、`BufferUsage` | `vk::ImageUsageFlags`、`vk::BufferUsageFlags` | 数值相同,`from_raw` 调用点改为构造 |
| `AccessCode`、`access_info` | `u8` 访问码、`native::access_table::ACCESS_TABLE` | `ImageState` 取代 `vk::ImageLayout`;`StageMask` 取代 `vk::PipelineStageFlags` |
| `Command`、`CommandSink` | `pass_api::Command`、`CommandBackend` | `BindPipeline` 是新增:现有枚举没有它,管线绑定走别处,**需要核对现有绑定路径** |
| `RenderingDesc` | `vk::RenderingInfo` | 新增 `merge_group` |
| `DeviceBackend`、`Device` | `device::Device` | 门面保留具体类型 `Device`,内部 `Arc<dyn DeviceBackend>` |
| `SurfaceSource`、`SwapchainBackend` | `native::surface::Surface::from_win32*`、`present::Swapchain` | 现有已支持 Win32/AppKit/Xlib/Xcb/Wayland,iOS 的 UIView 沿用 `MetalLayer` |
| `ResourceTable` | `bindless::BindlessHeap` 及分页 | 见 §4.5 |
| `DeviceCaps` | `device::Feature`、`FeatureSet`、`caps::DeviceCaps` | 特性名里的 Vulkan 名称换为能力字段 |
| `AccessLowering` | `sync_plan` 内的 Vulkan 屏障构造 | 每个后端各一份 |
| `ShaderArtifact`、`PipelineBackend` | `pipeline::shader::*`、`ShaderBackend` | SPIR-V 作为交换格式;Metal 后端在内部做转译 |

### 4.5 资源表分档

现有 `BindlessHeap` 要求 update-after-bind、运行时描述符数组、partially-bound,并按设备上限自适应(`render_host.rs:4696`,注释提到 Mac 上限为 1212)。新接口分三档:

| 档 | 含义 | 适用 |
| --- | --- | --- |
| `Large` | 大表,按编号访问 | 桌面 Vulkan |
| `Paged` | 受限大小的分页表 | 上限偏低的设备(现有 `BindlessPagePool` 已做分页) |
| `PerPass` | 每个 pass 一张小表 | 无法提供大表的设备 |

着色器侧通过一层包装访问资源,同一份 HLSL 能编译成多档。这是**设计意图,未验证**:`SPV_EXT_descriptor_indexing` 在 14 个着色器里用到,包装是否能覆盖全部写法没有核对。

### 4.6 游戏侧边界 trait 的处理

E1 里的边界 trait **保持名字和方法不变**,只替换其中的 Vulkan 类型。这样 `country-core` 的 pass 声明代码不用重写(`rhapsody_contract.rs` 加 `render/pass_builder/` 合计约 9.9k 行,这一层目前几乎没有 Vulkan 引用)。

## 5. 必须替换的泄漏点

| 泄漏 | 位置 | 替换 |
| --- | --- | --- |
| `issue_full_pipeline_barrier(vk::CommandBuffer)` 等 3 个方法(共 7 处原生命令缓冲签名) | `country-core/src/rhapsody_contract.rs:53,65,77` | 用图内声明的访问替代手工屏障,或收进后端内部;**需逐个调用点审计**,我没有读调用方,不确定能否都消除 |
| 返回 `vk::Buffer`、`vk::DeviceAddress` | `rhapsody_contract.rs:157,187` | `BufferId`、`DeviceBackend::device_address` |
| `PhysicalDeviceMemoryBudgetPropertiesEXT` | `rhapsody_contract.rs:118` | `DeviceCaps::memory_budget_bytes` |
| `BufferUsageFlags::from_raw(desc.usage)` | `country-core/src/render/ssbo.rs:195` | `BufferUsage(desc.usage)` |
| `vk::Format::from_raw` 后用 `{:?}` 打印格式名(为保持原版 panic 文案) | `render_frame.rs:264`、`:1016` | 为 `Format` 提供与原文案一致的调试名;**有保真度测试要求,改后必须对照** |
| 宿主层 `vk::*` 转换(约 170 处) | `tiny-glade/src/render_host.rs`、`raster_host.rs` | 改用中立类型,多数机械替换;`vk::CommandBuffer`(6 处)改为 `CommandSink`;`BufferImageCopy`/`BufferCopy` 需要在 `Command` 里补充图像拷贝区域类型(**签名里尚未包含**) |
| 表面创建 | `production_renderer.rs` | `SurfaceSource` |
| `VulkanGraphSession`、`VulkanGraphBackend`、`PresentTarget` | `rg::vulkan` | 改名为不带后端名的 `GraphSession`、`GraphBackend`(泛型或 trait 对象) |
| `native::InstanceBuilder`、`GraphicsInstanceLayerAllowlist` | `tiny-glade` 启动代码 | 后端专属配置,放进 `BackendConfig` 枚举,不进通用接口 |
| `device::semaphore::TimelineSemaphore` | 宿主层 2 处 | `SyncPoint`(已中立)加后端内部实现 |

## 6. 兼容策略

- 门面 crate 保持现有导入路径,迁移分步进行,每步 Windows 路径都保持可编译。
- `rhapsody::native::*`、`rhapsody::raw::*` 在迁移完成前保留,放在 `vulkan-raw` 特性后面;游戏层不再使用。
- 访问码、格式编号、usage 位的数值不变。

## 7. 实施步骤与验收

每步通过验收才进行下一步。

| 步 | 内容 | 验收 |
| --- | --- | --- |
| 1 | **泄漏审计**(只读):列出 §5 每个泄漏点的全部调用点,逐个给出替换方案;逐文件确认 §3 `rg` 子目录分类 | 清单完整;每项有替换方案或明确标为"无法消除" |
| 2 | 把零引用文件(约 22.4k 行)移入 `prism-gfx`,门面重新导出 | Windows 构建通过;现有 `cargo test -p rhapsody`(lavapipe)结果与迁移前一致 |
| 3 | 引入中立词汇类型,Vulkan 后端用适配层对接 | 表面像素对比(`tools/compare_surface_pixels.py`)结果不变;CPU 渲染阶段 p95 不退化(用现有性能夹具) |
| 4 | 定稿 `rhapsody-rhi` trait,Vulkan 实现迁到 `rhapsody-vulkan` | 同上,加上 `CommandSink` 分发开销的专项测量 |
| 5 | Metal 后端最小可用:清屏、三角形、compute、读回 | 在 Mac 上跑通;lavapipe 之外增加 Metal 的功能测试 |
| 6 | 着色器转译链(SPIR-V 到 MSL),像素对比 | 逐着色器对照 Vulkan 输出 |

## 8. 已验证与未验证

**已验证**
- §3 的行数和零引用统计:由脚本对 `rhapsody/src` 计算(排除测试文件)。
- §4.3 的签名在独立 crate 中 `cargo check` 通过(只含 `std`)。
- E1–E11 中引用的类型、trait、文件位置都读过。

**未验证**
- 这套中立类型能否表达游戏侧全部约 270 个调用点(`country-core` 73 个符号、`tiny-glade` 197 个符号),没有逐个映射。
- `CommandSink` 的批量编码对 CPU 渲染阶段的影响。
- `Command` 枚举是否覆盖宿主层的全部实际录制调用:现有 `Command` 里没有屏障、图像拷贝、描述符绑定;`BindPipeline` 与图像拷贝区域是我补充的,签名里图像拷贝区域**还没有**。
- `access_info` 的 64 个码能否全部无损降级到 Metal 的 fence/hazard 模型;光线追踪相关码(0x38–0x3f)在本项目中是否使用,没有核对。
- `merge_group` 在 Metal 上的 tile 内存实现,以及在 Vulkan 上落到哪种机制。
- `IndexedCountAdapter` 的回退行为(E9)。
- 整个方案在 Apple 硬件上的任何实测。

## 9. 下一步

执行 §7 的第 1 步(泄漏审计),产出逐调用点的清单。这一步只读代码,不修改任何文件。
