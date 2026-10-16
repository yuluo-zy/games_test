//! 实际托管句柄消费单个原子 tracker/refcount/resource_checksum，保留 raw 对象拥有权。
use super::resources::{ResourceKey, ResourceState, SharedResources, enqueue_retirement};
use super::tracking::{
    CommandKey, CommandState, ControlBlock, Failure, TrackOutcome, clone_reference, drop_reference,
    track_handle,
};
use std::{
    fmt,
    ops::Deref,
    sync::{Arc, atomic::Ordering},
};

pub(crate) struct ManagedInner<T: Send + Sync + 'static> {
    pub(crate) key: ResourceKey,
    pub(crate) resource: Arc<T>,
    pub(crate) control: Arc<ControlBlock>,
    pub(crate) state: Arc<ResourceState<T>>,
    pub(crate) shared: Arc<SharedResources>,
    /// 仅 `Managed<Texture>` 使用（原 `ManagedTexture` 自带的视图缓存）；其他类型恒为空。
    pub(crate) views: super::texture::ViewCache,
}
/// Arc 保障 Rust 内存安全；CPU 句柄数量由控制块的 references 独立维护。
pub struct Managed<T: Send + Sync + 'static>(pub(crate) Arc<ManagedInner<T>>);
impl<T: Send + Sync + 'static> Managed<T> {
    pub fn key(&self) -> ResourceKey {
        self.0.key
    }
    /// 原 inner 无 domain。跨域映射未恢复，不能补造多队列 tracker。
    pub fn track_on(&self, domain: u32, command: CommandState) -> Result<(), Failure> {
        if domain != 0 {
            return Err(Failure::CrossDomainMappingUnknown);
        }
        self.track(command)
    }
    /// 把命令登记到句柄：校验和不等（句柄已过期）静默跳过；命令不在录制状态返回 `InvalidCommandState`。
    pub fn track(&self, command: CommandState) -> Result<(), Failure> {
        self.track_outcome(command).map(|_| ())
    }
    /// 同 `track`，并返回是否因校验和不等被跳过（供需要结果的调用方）。
    pub fn track_outcome(&self, command: CommandState) -> Result<TrackOutcome, Failure> {
        track_handle(&self.0.control, self.0.key.resource_checksum, command)
    }
    pub fn tracked_sequence(&self) -> u64 {
        self.0.control.last_sequence.load(Ordering::SeqCst)
    }
    /// 仅供旧候选适配。domain=0 不是恢复出的原 ABI 字段。
    pub fn tracked_commands(&self) -> Vec<CommandKey> {
        let sequence = self.tracked_sequence();
        if sequence == 0 {
            Vec::new()
        } else {
            vec![CommandKey {
                domain: 0,
                sequence,
            }]
        }
    }
    pub fn reference_count(&self) -> u32 {
        self.0.control.references.load(Ordering::SeqCst) as u32
    }
    pub fn resource(&self) -> &T {
        &self.0.resource
    }
    /// 原 `RawManagedHandle` 24 B 布局的投影（指针以 `usize` 给出，仅作身份/测试用，不可解引用）。
    pub fn abi(&self) -> RawManagedHandleAbi {
        RawManagedHandleAbi {
            resources: Arc::as_ptr(&self.0.shared) as usize,
            control: Arc::as_ptr(&self.0.control) as usize,
            resource_checksum: self.0.key.resource_checksum,
            device_id: self.0.key.device_id,
            resource_type: self.0.key.resource_tag,
            _pad: 0,
        }
    }
    /// 共享的物理资源本体（图后端按 token 登记 `GpuResource` 时用）。
    pub fn shared(&self) -> Arc<T> {
        self.0.resource.clone()
    }
}
impl<T: Send + Sync + 'static> Clone for Managed<T> {
    fn clone(&self) -> Self {
        clone_reference(&self.0.control);
        Self(self.0.clone())
    }
}
impl<T: Send + Sync + 'static> Deref for Managed<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.resource()
    }
}
impl<T: Send + Sync + 'static> PartialEq for Managed<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}
impl<T: Send + Sync + 'static> Eq for Managed<T> {}
impl<T: Send + Sync + 'static> fmt::Debug for Managed<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Managed")
            .field("key", &self.key())
            .field("tracked_sequence", &self.tracked_sequence())
            .finish()
    }
}
impl<T: Send + Sync + 'static> Drop for Managed<T> {
    fn drop(&mut self) {
        let last = drop_reference(&self.0.control, false).expect("存活句柄的控制块必须有效");
        if last {
            enqueue_retirement(&self.0);
        }
    }
}
pub type RawManagedHandle<T> = Managed<T>;

/// 资源校验和；候选以 resource_checksum 为校验语义，`invalidate` 使旧句柄失效。
/// 原字段来自 `.rdata` 的 `resource_checksum@+0x10`；这里只包装 u32 不伪造布局。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ManagedResourceChecksum(pub u32);
impl ManagedResourceChecksum {
    pub fn invalidate(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
    pub fn value(self) -> u32 {
        self.0
    }
}

/// 原 `ResourceType` 枚举（`RawManagedHandle` 的 `resource_type@+0x16` / `DisposedRawResourceHandle` 的 tag@+0xe）。
/// 名字取自 `RawManagedHandle as Debug`（0x140bd69a0）→ `ResourceType as Debug`（0x140bd6bd0）的 8 项名字表
/// （长度表 0x142b10168 + i32 偏移表 0x142b101a8）：`Texture, Buffer, Swapchain, TextureView, RenderPass, Sampler,
/// PersistentDescriptorSet, Framebuffer`。
///
/// `take_out_dropped_waste`（0x1418c0a70）的跳表 0x142d060f0 把 tag 映射到 `dispose_cb_resource<Managed*>` 单态：
/// 0→0x1418c65a0（Texture）、1→0x1418c6af0（Buffer）、2→0x1418c55b0（Swapchain）、3（TextureView，原版 `todo!()`；
/// 这里与其他种类一样走通用延迟销毁）、4→0x1418c6050
/// （RenderPass）、5→0x1418c5b00（Sampler）、6→0x1418c7040（PersistentDescriptorSet）、7→0x1418c7590（Framebuffer）。
/// tag 8 是 `Option<DisposedRawResourceHandle>::None` 的 niche，用作迭代哨兵。
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResourceTag {
    Texture = 0,
    Buffer = 1,
    Swapchain = 2,
    TextureView = 3,
    RenderPass = 4,
    Sampler = 5,
    DescriptorSet = 6,
    Framebuffer = 7,
}
impl ResourceTag {
    /// `Option<DisposedRawResourceHandle>::None` 的 niche（循环哨兵，`cmp tag, 8; je done`）。
    pub const NONE_NICHE: u8 = 8;
    pub fn from_u8(tag: u8) -> Option<Self> {
        Some(match tag {
            0 => Self::Texture,
            1 => Self::Buffer,
            2 => Self::Swapchain,
            3 => Self::TextureView,
            4 => Self::RenderPass,
            5 => Self::Sampler,
            6 => Self::DescriptorSet,
            7 => Self::Framebuffer,
            _ => return None,
        })
    }
    /// 原 `ResourceType as Debug` 打印的变体名。
    pub const fn name(self) -> &'static str {
        match self {
            Self::Texture => "Texture",
            Self::Buffer => "Buffer",
            Self::Swapchain => "Swapchain",
            Self::TextureView => "TextureView",
            Self::RenderPass => "RenderPass",
            Self::Sampler => "Sampler",
            Self::DescriptorSet => "PersistentDescriptorSet",
            Self::Framebuffer => "Framebuffer",
        }
    }
}

/// `RawManagedHandle` 的真实 24 字节布局（`Clone` 0x140bd6b00 / `PartialEq` 0x140bd6b30 / `track` 0x140bd6a70 /
/// `Drop` 0x140be24b0 / `Debug` 0x140bd69a0 逐字节核对）：
///
/// | 偏移 | 内容 |
/// |---|---|
/// | `+0x00` | `*mut Mutex<Vec<DisposedRawResourceHandle>>`（退役队列；Clone 复制、Eq 不比较） |
/// | `+0x08` | `*mut ManagedResource<T>`（控制块 `{seq,refcount,checksum,resource}`；Eq 比较） |
/// | `+0x10` | `u32 resource_checksum`（期望值；`track` 比较 `[ctl+0xc]`，不等则静默跳过） |
/// | `+0x14` | `u16 device_id` |
/// | `+0x16` | `u8 resource_type`（[`ResourceTag`]） |
/// | `+0x17` | 填充 |
///
/// Rust 侧的 `Managed<T>` 用 `Arc` 承载拥有权；本类型是它在原 ABI 下的**投影**（`Managed::abi`），指针以 `usize`
/// 表示，不能被解引用。`PartialEq` 与原版一致：只比较控制块指针 + 校验和 + device + tag（不比较队列指针）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawManagedHandleAbi {
    pub resources: usize,
    pub control: usize,
    pub resource_checksum: u32,
    pub device_id: u16,
    pub resource_type: u8,
    pub _pad: u8,
}
const _: () = assert!(std::mem::size_of::<RawManagedHandleAbi>() == 0x18);
const _: () = assert!(std::mem::offset_of!(RawManagedHandleAbi, control) == 0x08);
const _: () = assert!(std::mem::offset_of!(RawManagedHandleAbi, resource_checksum) == 0x10);
const _: () = assert!(std::mem::offset_of!(RawManagedHandleAbi, device_id) == 0x14);
const _: () = assert!(std::mem::offset_of!(RawManagedHandleAbi, resource_type) == 0x16);
impl RawManagedHandleAbi {
    /// 原 `PartialEq`（0x140bd6b30）：`[+8..+0x10) == && [+0xf..+0x17) ==`（控制块指针 + 校验和/device/tag，
    /// 第二段 8 字节与第一段的最高字节重叠 1 字节）。`resources`（+0）与填充（+0x17）不参与。
    pub fn original_eq(&self, other: &Self) -> bool {
        self.control == other.control
            && self.resource_checksum == other.resource_checksum
            && self.device_id == other.device_id
            && self.resource_type == other.resource_type
    }
    pub fn tag(&self) -> Option<ResourceTag> {
        ResourceTag::from_u8(self.resource_type)
    }
}

/// `DisposedRawResourceHandle` 的真实 16 字节布局（`Drop` 0x140be24b0 入队写序列 + `take_out_dropped_waste`
/// 0x1418c0a70 读序列 + `dispose<T>` 0x1418af090..0x1418afa20 的 `[rdx]/[rdx+8]/[rdx+0xc]/[rdx+0xe]` 互证）：
///
/// | 偏移 | 内容 |
/// |---|---|
/// | `+0x00` | `*mut ManagedResource<T>` 控制块（`*元素` = `inner.seq`，作 `CommandBufferSeq` 键） |
/// | `+0x08` | `u32 resource_checksum`（`dispose` 比较 `[ctl+0xc]`） |
/// | `+0x0c` | `u16 device_id`（`dispose` 比较 `ManagedResources+0xa50`） |
/// | `+0x0e` | `u8 resource_type`（跳表键；8 = `Option::None` niche） |
/// | `+0x0f` | **从不写入的填充字节**（`Drop` 只写 15 字节；循环读它作 `edx` 传给 `dispose_cb_resource`，是栈残值，语义无意义） |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisposedRawResourceHandleAbi {
    pub control: usize,
    pub resource_checksum: u32,
    pub device_id: u16,
    pub resource_type: u8,
    pub _pad: u8,
}
const _: () = assert!(std::mem::size_of::<DisposedRawResourceHandleAbi>() == 0x10);
const _: () = assert!(std::mem::offset_of!(DisposedRawResourceHandleAbi, resource_checksum) == 0x08);
const _: () = assert!(std::mem::offset_of!(DisposedRawResourceHandleAbi, device_id) == 0x0c);
const _: () = assert!(std::mem::offset_of!(DisposedRawResourceHandleAbi, resource_type) == 0x0e);
impl DisposedRawResourceHandleAbi {
    /// 原 `Drop` 入队：`[+0]=inner [+8]=handle.checksum [+0xc]=handle.device [+0xe]=handle.tag`，`+0xf` 不写（此处置 0）。
    pub fn from_handle(handle: &RawManagedHandleAbi) -> Self {
        Self {
            control: handle.control,
            resource_checksum: handle.resource_checksum,
            device_id: handle.device_id,
            resource_type: handle.resource_type,
            _pad: 0,
        }
    }
    pub fn tag(&self) -> Option<ResourceTag> {
        ResourceTag::from_u8(self.resource_type)
    }
}

/// 已进入销毁路径的裸资源句柄；`resource` 返回被移交的物理对象引用。
#[derive(Debug)]
pub struct DisposedRawResourceHandle<T> {
    pub(crate) resource: Arc<T>,
}
impl<T> DisposedRawResourceHandle<T> {
    pub fn new(resource: Arc<T>) -> Self {
        Self { resource }
    }
    pub fn resource(&self) -> &T {
        &self.resource
    }
}

/// 原 1.x 句柄包装：持有 store 引用与逻辑索引。候选以 `Managed<T>` 承载同样语义。
#[derive(Clone, Debug)]
pub struct ManagedHandle<T: Send + Sync + 'static> {
    pub(crate) inner: Managed<T>,
}
impl<T: Send + Sync + 'static> ManagedHandle<T> {
    pub fn from_managed(inner: Managed<T>) -> Self {
        Self { inner }
    }
    /// 原 `into_raw_gpu_owned`：单态 `GpuOwned<(), RawManagedHandle>`。
    pub fn into_raw_gpu_owned(self) -> GpuOwned<(), RawManagedHandle<T>> {
        GpuOwned::new((), self.inner)
    }
    pub fn as_managed_handle(&self) -> &Managed<T> {
        &self.inner
    }
}

/// 任何能把“被某条命令使用”记到 GPU 生命周期上的 tracker（原 trait `TrackGpuLifetime`）。
pub trait TrackGpuLifetime {
    /// 命令状态非法返回 `InvalidCommandState`。
    fn track(&self, command: CommandState) -> Result<(), Failure>;
}
impl<T: Send + Sync + 'static> TrackGpuLifetime for Managed<T> {
    fn track(&self, command: CommandState) -> Result<(), Failure> {
        Managed::track(self, command)
    }
}
impl<T: Send + Sync + 'static> TrackGpuLifetime for ManagedHandle<T> {
    fn track(&self, command: CommandState) -> Result<(), Failure> {
        self.inner.track(command)
    }
}

/// `GpuOwned` 的状态字节（`Cell<GpuOwnedState>`，紧随 tracker 之后）。
///
/// 三个判别值均由机器码读出：`new`/克隆 = 0（`mov byte [dst+0x58],0`，0x1403abd10）；`track_unwrap` 的
/// `Cell::set(Tracked)` = **1**（内联在 `0x1415fb605: mov byte [rbp+8],1`，紧跟 `RawManagedHandle::track`
/// 0x1415fb600 与 tracker 析构 0x1415fb638，元素 0x20 B = tracker 24 B + state@0x18）；`mark_unused` 的
/// `Cell::set(Unused)` = **2**（`ComputePass::bind_external_raw` 0x14096e658: `mov byte [rbp-0x18],2`，随后
/// `RawManagedHandle::drop` 0x14096e65c；资源 0x20 B @-0x50、tracker @-0x30、state @-0x18）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GpuOwnedState {
    Owned = 0,
    Tracked = 1,
    Unused = 2,
}

/// 原 `GpuOwned<T, Tracker>`：`repr(C)` 字段序 `{resource@0, tracker@size(T), state@size(T)+size(Tracker)}`。
/// 例：`<[DescriptorSetBinding; 2], RawManagedHandle>` = resource 0x40@0、tracker 0x18@0x40、state@0x58、共 0x60 B
/// （0x1403abd10 / 0x140161b50）；`<(), RawManagedHandle>` = tracker@0、state@0x18、共 0x20 B（0x1415fb5c0 循环步长 0x20）。
/// 原版实际单态：`<DescriptorSetBinding, RawManagedHandle>`、`<[DescriptorSetBinding; 2], RawManagedHandle>`、
/// `<(), ManagedHandle<ManagedBuffer>>`。
#[repr(C)]
pub struct GpuOwned<R, Tracker> {
    resource: R,
    tracker: Tracker,
    state: std::cell::Cell<GpuOwnedState>,
}
impl<R: Clone, Tracker: Clone> Clone for GpuOwned<R, Tracker> {
    fn clone(&self) -> Self {
        Self {
            resource: self.resource.clone(),
            tracker: self.tracker.clone(),
            state: std::cell::Cell::new(self.state.get()),
        }
    }
}
impl<R, Tracker> GpuOwned<R, Tracker> {
    pub fn new(resource: R, tracker: Tracker) -> Self {
        Self {
            resource,
            tracker,
            state: std::cell::Cell::new(GpuOwnedState::Owned),
        }
    }
    pub fn state(&self) -> GpuOwnedState {
        self.state.get()
    }
    pub fn resource(&self) -> &R {
        &self.resource
    }
    /// 原 `map_tracker`：换一个 tracker，资源与状态原样保留。
    pub fn map_tracker<U>(self, f: impl FnOnce(Tracker) -> U) -> GpuOwned<R, U> {
        GpuOwned {
            resource: self.resource,
            tracker: f(self.tracker),
            state: self.state,
        }
    }
    /// 原 `mark_unused`：只置状态（`Cell::set(Unused=2)`），不动引用计数。
    pub fn mark_unused(&self) {
        self.state.set(GpuOwnedState::Unused);
    }
    /// 原 `drop_unused`（`ComputePass::bind_external_raw` 内联，0x14096e658）：置 `Unused` 后析构资源与 tracker，
    /// 不登记任何命令。
    pub fn drop_unused(self) {
        self.mark_unused();
        drop(self);
    }
}
impl<R, Tracker: TrackGpuLifetime> GpuOwned<R, Tracker> {
    /// 先登记到命令，再置“已跟踪”，然后交出资源并丢弃 tracker。命令状态非法返回
    /// `InvalidCommandState`，状态仍为 Owned。
    pub fn track_unwrap(self, command: CommandState) -> Result<R, Failure> {
        self.tracker.track(command)?;
        self.state.set(GpuOwnedState::Tracked);
        Ok(self.resource)
    }
}
impl<T: Send + Sync + 'static> ManagedHandle<T> {
    /// 原 `into_gpu_owned`：单态 `GpuOwned<(), ManagedHandle<T>>`。
    pub fn into_gpu_owned(self) -> GpuOwned<(), ManagedHandle<T>> {
        GpuOwned::new((), self)
    }
}

/// 借用视图：可绑定但未持有所有权的句柄投影；`get_tracked` 先跟踪再取资源。
#[derive(Clone, Copy, Debug)]
pub struct ManagedRef<'a, T: Send + Sync + 'static> {
    inner: &'a Managed<T>,
}
impl<'a, T: Send + Sync + 'static> ManagedRef<'a, T> {
    pub fn new(inner: &'a Managed<T>) -> Self {
        Self { inner }
    }
    pub fn as_owner(self) -> Managed<T> {
        self.inner.clone()
    }
    pub fn into_owner(self) -> Managed<T> {
        self.inner.clone()
    }
    pub fn as_managed_handle(&self) -> &Managed<T> {
        self.inner
    }
    pub fn get_untracked(&self) -> &T {
        self.inner.resource()
    }
    /// “获取并跟踪”：先把命令记到句柄上，再交出资源引用；命令状态非法返回 `InvalidCommandState`。
    pub fn get_tracked(&self, command: CommandState) -> Result<&T, Failure> {
        self.inner.track(command)?;
        Ok(self.inner.resource())
    }
}

#[cfg(test)]
mod gpu_owned_tests {
    use super::*;

    #[test]
    fn state_discriminants_are_read_from_machine_code() {
        assert_eq!(GpuOwnedState::Owned as u8, 0); // 0x1403abd10 新建/克隆
        assert_eq!(GpuOwnedState::Tracked as u8, 1); // 0x1415fb605 track_unwrap
        assert_eq!(GpuOwnedState::Unused as u8, 2); // 0x14096e658 mark_unused
    }

    #[test]
    fn gpu_owned_layout_matches_original_instances() {
        use std::mem::{offset_of, size_of};
        // <[DescriptorSetBinding; 2], RawManagedHandle>：resource 0x40 @0、tracker 0x18 @0x40、state @0x58、共 0x60。
        type Dsb2 = GpuOwned<[[u64; 4]; 2], RawManagedHandleAbi>;
        assert_eq!(offset_of!(Dsb2, resource), 0);
        assert_eq!(offset_of!(Dsb2, tracker), 0x40);
        assert_eq!(offset_of!(Dsb2, state), 0x58);
        assert_eq!(size_of::<Dsb2>(), 0x60);
        // <DescriptorSetBinding, RawManagedHandle>：state @0x38（bind_external_raw：-0x50/-0x30/-0x18）。
        type Dsb1 = GpuOwned<[u64; 4], RawManagedHandleAbi>;
        assert_eq!(offset_of!(Dsb1, tracker), 0x20);
        assert_eq!(offset_of!(Dsb1, state), 0x38);
        assert_eq!(size_of::<Dsb1>(), 0x40);
        // <(), RawManagedHandle>：state @0x18、共 0x20（0x1415fb5c0 循环 shl 5）。
        type Unit = GpuOwned<(), RawManagedHandleAbi>;
        assert_eq!(offset_of!(Unit, tracker), 0);
        assert_eq!(offset_of!(Unit, state), 0x18);
        assert_eq!(size_of::<Unit>(), 0x20);
    }

    #[test]
    fn drop_unused_marks_unused_and_never_tracks() {
        struct Probe<'a>(&'a std::cell::Cell<u32>);
        impl TrackGpuLifetime for Probe<'_> {
            fn track(&self, _: CommandState) -> Result<(), Failure> {
                self.0.set(self.0.get() + 100);
                Ok(())
            }
        }
        impl Drop for Probe<'_> {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        let hits = std::cell::Cell::new(0);
        GpuOwned::new((), Probe(&hits)).drop_unused();
        assert_eq!(hits.get(), 1, "只析构 tracker，不 track");
    }

    #[test]
    fn resource_tag_matches_original_resource_type_table() {
        let names: Vec<_> = (0..8)
            .map(|t| ResourceTag::from_u8(t).unwrap().name())
            .collect();
        assert_eq!(
            names,
            [
                "Texture",
                "Buffer",
                "Swapchain",
                "TextureView",
                "RenderPass",
                "Sampler",
                "PersistentDescriptorSet",
                "Framebuffer"
            ]
        );
        assert_eq!(ResourceTag::from_u8(ResourceTag::NONE_NICHE), None);
    }

    #[test]
    fn abi_projection_layout_and_original_equality() {
        use std::mem::offset_of;
        assert_eq!(offset_of!(DisposedRawResourceHandleAbi, control), 0);
        let a = RawManagedHandleAbi {
            resources: 1,
            control: 0x1000,
            resource_checksum: 7,
            device_id: 2,
            resource_type: 1,
            _pad: 0,
        };
        // 原 PartialEq 不比较队列指针（+0）。
        let b = RawManagedHandleAbi { resources: 99, ..a };
        assert!(a.original_eq(&b));
        assert!(!a.original_eq(&RawManagedHandleAbi { resource_checksum: 8, ..a }));
        assert!(!a.original_eq(&RawManagedHandleAbi { control: 0x2000, ..a }));
        let d = DisposedRawResourceHandleAbi::from_handle(&a);
        assert_eq!((d.control, d.resource_checksum, d.device_id, d.tag()), (0x1000, 7, 2, Some(ResourceTag::Buffer)));
        assert_eq!(d._pad, 0);
    }

    struct Counter(std::cell::Cell<u32>);
    impl TrackGpuLifetime for &Counter {
        fn track(&self, command: CommandState) -> Result<(), Failure> {
            if command.tag != 1 {
                return Err(Failure::InvalidCommandState);
            }
            self.0.set(self.0.get() + 1);
            Ok(())
        }
    }

    #[test]
    fn track_unwrap_tracks_then_yields_the_resource() {
        let c = Counter(Default::default());
        let owned = GpuOwned::new(7u32, &c);
        assert_eq!(owned.state(), GpuOwnedState::Owned);
        let ok = CommandState { tag: 1, sequence: 3 };
        assert_eq!(owned.track_unwrap(ok), Ok(7));
        assert_eq!(c.0.get(), 1);
        let bad = CommandState { tag: 0, sequence: 3 };
        assert_eq!(GpuOwned::new(1u8, &c).track_unwrap(bad), Err(Failure::InvalidCommandState));
        assert_eq!(c.0.get(), 1);
    }

    #[test]
    fn mark_unused_only_sets_state_and_map_tracker_keeps_it() {
        let o = GpuOwned::new("r", 1u8);
        o.mark_unused();
        let o = o.map_tracker(|t| t as u32 + 1);
        assert_eq!(o.state(), GpuOwnedState::Unused);
        assert_eq!(*o.resource(), "r");
    }
}
