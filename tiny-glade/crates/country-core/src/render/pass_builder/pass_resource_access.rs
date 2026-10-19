//! pass_resource_access.rs — SSBO 访问登记与纹理访问 trait。
//!
//! 机器码锚点：
//! - `read_access_buffer<T>` 0x1402b00e0（53 个实例）/ `write_access_buffer<T>` 0x140973720（37）/
//!   `write_access_buffer_concurrent<T>` 0x140975c20（9）。ICF 后同形体，只差下表三处。
//! - 纹理访问 trait：`impl$0` 0x140a59800/0x140a59860（`TexturePreset`）、`impl$2` 0x1408ef010/0x1408ef170
//!   （受管 `Handle<Texture>`）、`impl$3`/`impl$6` 0x1409248b0/0x1409248f0（两者机器码相同，被 ICF 合并：
//!   RG 已导入纹理与 `WriteTextureUnchecked`）。
//!
//! `*_access_buffer<T>` 的机器码（O）：
//!   1. `AssetSsboLibrary::get_rg_buffer<T>(&PassResources.ssbo_library /* [rcx+0x10] */, rg = *(builder+0x40))`
//!      → 句柄，资源号 = 句柄 `+0x10`（u32）；
//!   2. `PassBuilder::add_resource_access(builder, resource, access, group)`；
//!   3. 绑定数 `>= 0x100` → `Result::unwrap` 失败（索引要装进 u8）；
//!   4. 追加一个 0x44 B `RenderPassBinding`，返回追加前的下标（`mov eax, edi`）。
//!
//! | 函数 | access（调用方给，u8） | group | `RenderPassBinding` 构造（O 地址） |
//! |---|---|---|---|
//! | read  | 调用方（`bind_color_library` 等给 0x1a） | **1** | 标签 3（0x140bd6c70） |
//! | write | 调用方（`write_to_cpu_mapped_*` 给 0x32） | **0** | 标签 4（0x140bd6c60） |
//! | write_concurrent | 调用方 | 调用方再给一个 u32 | 标签 4 |
//!
//! 纹理 trait 的 `as_texture`（O）返回一个枚举：标签 0 = RG 已导入纹理 `{extent, id:u32, desc(32B)}`；
//! 标签 1 = 受管纹理 `{extent, 图像视图, 受管句柄克隆, 0}`（视图经 `GraphicsDevice::get_vk_image_view` 取得，
//! `Result::unwrap`）。`dims` 恒为 `TextureDesc::extent_2d`。
//!
//! 偏离：`PassBuilder`/`GraphicsDevice` 属 rhapsody（复用库），以 [`RecordingPassBuilder`] / [`TextureDevice`] 接缝；
//! 旧版以 `hash → 注册表` 的 `append_resource_binding` 是臆造的（机器码里 SSBO 访问不经注册表），已移除。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::field_reassign_with_default)]
use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType as LibrarySsbo};
use crate::asset_libraries::texture_library::AssetTextureLibrary;
use crate::render::pass_builder::pass_resources::PassResources;
use crate::resources::texture_presets::TextureFromDisk;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_contract::{self, Extent2D, Handle, ManagedHandle, ReadOnlyHandle, Texture};
use crate::rhapsody_types::{BackendError, RenderGraph, TextureDesc, TextureViewDesc};

// ---- 访问类型（`vk_sync_fork::AccessType` 的数值，O 立即数；枚举名 U）----
/// 间接参数缓冲读（`dispatch_indirect_*` 里 `mov r8b, 0x2`）。
pub const ACCESS_INDIRECT_READ: u8 = 0x02;
/// compute 着色器读 storage buffer（`read_rg_storage_buffer` / `bind_color_library` 等 `0x1a`）。
pub const ACCESS_READ: u8 = 0x1a;
/// compute 着色器写 storage buffer（`write_rg_storage_buffer*` 的 `0x37`）。
pub const ACCESS_WRITE: u8 = 0x37;
/// 并发写（O：与 `ACCESS_WRITE` 同值，区别只在同步组由调用方给）。
pub const ACCESS_WRITE_CONCURRENT: u8 = ACCESS_WRITE;
/// `write_to_cpu_mapped_storage_buffer` 传给 `write_access_buffer` 的访问码（O：`mov byte ptr [rsp+0x20], 0x32`）。
pub const ACCESS_CPU_MAPPED_WRITE: u8 = 0x32;
/// DrawBuilder 版 `bind_color_library` 的读访问码（O：`mov byte ptr [rsp+0x20], 0x1f`）。
pub const ACCESS_DRAW_READ: u8 = 0x1f;
/// 传输写（`clear_rg_*` / `rg_buffer_copy_from_slice` 的 `0x34`）。
pub const ACCESS_TRANSFER_WRITE: u8 = 0x34;
/// 同步组：写 / 默认 = 0。
pub const SYNC_GROUP_DEFAULT: u32 = 0;
/// 同步组：读 = 1（`read_access_buffer`、`read_rg_storage_buffer`、`dispatch_indirect_*` 的 `r9d = 1`）。
pub const SYNC_GROUP_READ: u32 = 1;

/// `vk_sync_fork::AccessType::is_write`（O 0x140c10e10）：`(access - 0x28) < 0x10`，即 0x28..=0x37 为写。
pub const fn access_is_write(access: u8) -> bool {
    access.wrapping_sub(0x28) < 0x10
}

/// `rhapsody::rg::RenderPassBinding` 的变体（0x44 B 枚举；这里只保留已观测的变体与载荷）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PassBinding {
    /// 标签 3：`read_access_buffer` 用（0x140bd6c70），载荷 = 资源号 u32。
    BufferRead { resource: u32 },
    /// 标签 4：storage buffer（0x140bd6c60），载荷 = 资源号 u32。
    Buffer { resource: u32 },
    /// 图像视图（`Ref::bind_view` 0x140bd6c00 读 / 0x140bd6c30 写：`+0x00..0x3c` 整块 `TextureViewDesc`，`+0x3c` 资源号，
    /// `+0x40` 写标志；B 阶段 i_api 补上视图描述——无视图时是全 `None` 的缺省描述）。
    ImageView {
        resource: u32,
        write: bool,
        view: TextureViewDesc,
    },
}

impl PassBinding {
    /// 观测到的标签值（视图变体的标签在视图描述内，U，这里给 0xff）。
    pub fn tag(&self) -> u8 {
        match self {
            PassBinding::BufferRead { .. } => 3,
            PassBinding::Buffer { .. } => 4,
            PassBinding::ImageView { .. } => 0xff,
        }
    }

    pub fn resource(&self) -> u32 {
        match *self {
            PassBinding::BufferRead { resource }
            | PassBinding::Buffer { resource }
            | PassBinding::ImageView { resource, .. } => resource,
        }
    }

    /// 与共享契约 `RenderPassBinding` 的互转（access_type 位放标签）。
    pub fn to_contract(&self) -> rhapsody_contract::RenderPassBinding {
        rhapsody_contract::bind(self.resource(), self.tag())
    }
}

/// 一次 `PassBuilder::add_resource_access` 调用的记录。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceAccess {
    pub resource: u32,
    pub access: u8,
    pub group: u32,
}

/// `PassBuilder<PassResources>` 的接缝：只复刻调用点用到的面（`add_resource_access`、`+0x40` 处的渲染图）。
#[derive(Debug, Default)]
pub struct RecordingPassBuilder {
    /// pass 名（O：`PassBuilder` 内 `name: &str`，`ComputePass::new` 的 `name` 实参）。
    pub name: &'static str,
    /// `builder+0x40`：渲染图（`get_rg_buffer` 的第二参数）。
    pub render_graph: RenderGraph,
    /// 已登记的访问（原版写进 `PassBuilder` 内部表，顺序保持）。
    pub accesses: Vec<ResourceAccess>,
}

impl RecordingPassBuilder {
    /// `PassBuilder::add_resource_access`（O 0x1409dca90 / 0x140533b10）。
    pub fn add_resource_access(&mut self, resource: u32, access: u8, group: u32) {
        self.accesses.push(ResourceAccess {
            resource,
            access,
            group,
        });
    }
}

/// 绑定表上限：下标必须装进 u8（O：`cmp rdi, 0x100; jae → Result::unwrap 失败`）。
const MAX_BINDINGS: usize = 0x100;

/// 追加前的下标；`>= 0x100` 时 `Result::unwrap()` 失败（原版 `u8::try_from(len).unwrap()`）。
pub(crate) fn next_binding_index(bindings: &[PassBinding]) -> u8 {
    let len = bindings.len();
    if len >= MAX_BINDINGS {
        // `core::result::unwrap_failed`：`called `Result::unwrap()` on an `Err` value: TryFromIntError(())`。
        panic!("called `Result::unwrap()` on an `Err` value: TryFromIntError(())");
    }
    len as u8
}

/// 取 `T` 对应的 RG 缓冲资源号（`get_rg_buffer<T>` 返回句柄的 `+0x10`）。
fn rg_resource<T: LibrarySsbo>(resources: &PassResources, rg: &mut RenderGraph) -> u32 {
    let library: &AssetSsboLibrary = resources.ssbo_library();
    library.get_rg_buffer::<T>(rg).resource as u32
}

/// `read_access_buffer<T>`（0x1402b00e0）：只读访问。返回新绑定的下标。
pub fn read_access_buffer<T: LibrarySsbo>(
    resources: &PassResources,
    builder: &mut RecordingPassBuilder,
    bindings: &mut Vec<PassBinding>,
    access: u8,
) -> u8 {
    let resource = rg_resource::<T>(resources, &mut builder.render_graph);
    builder.add_resource_access(resource, access, SYNC_GROUP_READ);
    let index = next_binding_index(bindings);
    bindings.push(PassBinding::BufferRead { resource });
    index
}

/// `write_access_buffer<T>`（0x140973720）：写访问（同步组 0）。
pub fn write_access_buffer<T: LibrarySsbo>(
    resources: &PassResources,
    builder: &mut RecordingPassBuilder,
    bindings: &mut Vec<PassBinding>,
    access: u8,
) -> u8 {
    let resource = rg_resource::<T>(resources, &mut builder.render_graph);
    builder.add_resource_access(resource, access, SYNC_GROUP_DEFAULT);
    let index = next_binding_index(bindings);
    bindings.push(PassBinding::Buffer { resource });
    index
}

/// `write_access_buffer_concurrent<T>`（0x140975c20）：并发写，同步组由调用方给。
pub fn write_access_buffer_concurrent<T: LibrarySsbo>(
    resources: &PassResources,
    builder: &mut RecordingPassBuilder,
    bindings: &mut Vec<PassBinding>,
    access: u8,
    group: u32,
) -> u8 {
    let resource = rg_resource::<T>(resources, &mut builder.render_graph);
    builder.add_resource_access(resource, access, group);
    let index = next_binding_index(bindings);
    bindings.push(PassBinding::Buffer { resource });
    index
}

/// 兼容旧调用点的 SSBO 标记 trait（`rg_buffer_hash` 由各 `*Ssbo` 载体按名字哈希给出；
/// 机器码里的 `*_access_buffer<T>` 用的是 [`LibrarySsbo`]，不使用本 trait）。
pub trait SsboType {
    /// 该 SSBO 的 RG 缓冲名字哈希。
    fn rg_buffer_hash() -> u64;
}

/// `ssbo_to_rg_handle`（内联）：`&T` → RG 句柄 hash（兼容旧 trait）。
#[inline]
pub fn ssbo_to_rg_handle<T: SsboType>() -> u64 {
    T::rg_buffer_hash()
}

// ---- 纹理访问 trait 族（as_texture / dims）----

/// `GraphicsDevice` 里纹理相关的外部调用点。
pub trait TextureDevice {
    /// `GraphicsDevice::get_vk_image_view(handle, None)`（O 0x140bfeea0）；`Err` 时调用点 `Result::unwrap`。
    fn get_vk_image_view(&self, handle: &ManagedHandle) -> Result<u64, BackendError>;
    /// 受管纹理的 desc（原版从句柄指向的纹理对象 `+0x18` 处读）。
    fn managed_texture_desc(&self, handle: &ManagedHandle) -> TextureDesc;
}

/// `TextureDesc::extent_2d`（O 0x140bd5790）：取 desc 的 (width, height)。
pub fn extent_2d(desc: &TextureDesc) -> Extent2D {
    Extent2D {
        width: desc.extent[0],
        height: desc.extent[1],
    }
}

/// `as_texture` 的返回枚举（O：标签 0 / 1）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextureBinding {
    /// 标签 0：RG 已导入纹理 `{extent, resource:u32, desc}`。
    Rg {
        extent: Extent2D,
        resource: u32,
        desc: TextureDesc,
    },
    /// 标签 1：受管纹理 `{extent, view, handle 克隆}`（末尾的 0 字节 U）。
    Managed {
        extent: Extent2D,
        view: u64,
        handle: ManagedHandle,
    },
    /// 标签 2（B 阶段补，i_api；O：`bind_texture_to_compute_pass` 里 `as_texture` 结果 `cmp eax, 2; je` → `AssetTextureLibrary::get_by_name`
    /// （名字在结果 `+8`），随后取库纹理的 `extent_2d(desc @+0x18)`、`get_vk_image_view(.., 0)`、`handle.clone()`）：磁盘纹理按名字解析。
    ByName { name_hash: u64 },
}

impl TextureBinding {
    pub fn extent(&self) -> Extent2D {
        match self {
            TextureBinding::Rg { extent, .. } | TextureBinding::Managed { extent, .. } => *extent,
            // 名字解析前没有 extent（由 `ComputePass::bind_texture` 经库取得）。
            TextureBinding::ByName { .. } => Extent2D::default(),
        }
    }
    /// 枚举标签（O：0 / 1）。
    pub fn tag(&self) -> u32 {
        match self {
            TextureBinding::Rg { .. } => 0,
            TextureBinding::Managed { .. } => 1,
            TextureBinding::ByName { .. } => 2,
        }
    }
}

/// 帧上下文：`as_texture`/`dims` 需要的 `PassResources`（纹理库）与设备。
pub struct TextureCtx<'a> {
    pub resources: &'a PassResources,
    pub device: &'a dyn TextureDevice,
}

/// 可作纹理访问的资源（原版的 `impl$0..$6`）。
pub trait TextureAccess {
    fn as_texture(&self, ctx: &TextureCtx<'_>) -> TextureBinding;
    fn dims(&self, ctx: &TextureCtx<'_>) -> Extent2D;
}

/// RG 已导入纹理（`impl$3`/`impl$6` 的被引用类型：`{id:u32, desc}`，O 36 B）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RgTexture {
    pub resource: u32,
    pub desc: TextureDesc,
}

/// 写纹理（不检查）：`generate_mips<WriteTextureUnchecked>` 等的参数（O 名；载荷 = RG 已导入纹理，I）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WriteTextureUnchecked(pub RgTexture);

fn rg_binding(t: &RgTexture) -> TextureBinding {
    TextureBinding::Rg {
        extent: extent_2d(&t.desc),
        resource: t.resource,
        desc: t.desc,
    }
}

/// `impl$3`/`impl$6`（0x1409248b0）：标签 0，`extent_2d(desc)`。
impl TextureAccess for &RgTexture {
    fn as_texture(&self, _ctx: &TextureCtx<'_>) -> TextureBinding {
        rg_binding(self)
    }
    fn dims(&self, _ctx: &TextureCtx<'_>) -> Extent2D {
        extent_2d(&self.desc)
    }
}

impl TextureAccess for WriteTextureUnchecked {
    fn as_texture(&self, _ctx: &TextureCtx<'_>) -> TextureBinding {
        rg_binding(&self.0)
    }
    fn dims(&self, _ctx: &TextureCtx<'_>) -> Extent2D {
        extent_2d(&self.0.desc)
    }
}

/// `TexturePreset` 经纹理库解析出 RG 纹理（`get_rg_texture`）与 desc。
///
/// O（0x140a59800）：`get_rg_texture` 的返回值本身就是 36 B `{id: u32, desc: TextureDesc}`，`as_texture` 对 `+4` 处的 desc 调
/// `extent_2d`——desc 随 `ImportedTexture` 一起来（B 阶段 i_api：`ImportedTexture.desc` 已补，不再回头查 `PresetTextureState.texture`）。
fn preset_texture(library: &AssetTextureLibrary, preset: TexturePreset) -> RgTexture {
    let imported = library.get_rg_texture(preset);
    RgTexture {
        resource: imported.resource as u32,
        desc: imported.desc,
    }
}

/// `impl$0`（0x140a59800/0x140a59860）：经 `AssetTextureLibrary::get_rg_texture(preset)`，标签 0。
impl TextureAccess for TexturePreset {
    fn as_texture(&self, ctx: &TextureCtx<'_>) -> TextureBinding {
        rg_binding(&preset_texture(ctx.resources.texture_library(), *self))
    }
    fn dims(&self, ctx: &TextureCtx<'_>) -> Extent2D {
        extent_2d(&preset_texture(ctx.resources.texture_library(), *self).desc)
    }
}

/// `impl$2`（0x1408ef010/0x1408ef170）：受管纹理句柄。`get_vk_image_view` 失败 → `Result::unwrap` panic
/// （"called `Result::unwrap()` on an `Err` value"，O 0x1408ef0ce 处 0x2b 字节文案）；标签 1，克隆句柄。
impl TextureAccess for &Handle<Texture> {
    fn as_texture(&self, ctx: &TextureCtx<'_>) -> TextureBinding {
        let view = ctx
            .device
            .get_vk_image_view(&self.0)
            .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"));
        let desc = ctx.device.managed_texture_desc(&self.0);
        TextureBinding::Managed {
            extent: extent_2d(&desc),
            view,
            handle: self.0.clone(),
        }
    }
    fn dims(&self, ctx: &TextureCtx<'_>) -> Extent2D {
        extent_2d(&ctx.device.managed_texture_desc(&self.0))
    }
}

/// 磁盘纹理（`bind_texture_to_compute_pass<TextureFromDisk>` 0x1409056d0）：`as_texture` 恒返回标签 2（按名字解析）；
/// `dims` = 库纹理的 `extent_2d`（`get_by_name` 在名字缺失时 panic）。
impl TextureAccess for TextureFromDisk {
    fn as_texture(&self, _ctx: &TextureCtx<'_>) -> TextureBinding {
        TextureBinding::ByName { name_hash: self.0 }
    }
    fn dims(&self, ctx: &TextureCtx<'_>) -> Extent2D {
        let t = ctx
            .resources
            .texture_library()
            .get_by_name(crate::name_hash::NameHash(self.0));
        extent_2d(&TextureDesc::new_2d([t.width, t.height], t.format))
    }
}

/// 原生纹理（`impl$2::as_texture` 0x1408ef010，`bind_texture_to_compute_pass<&Texture>` 0x1408fcd40 调用）：标签 1，
/// `get_vk_image_view(device, tex, None)`（`Err` → `Result::unwrap` panic）+ 句柄克隆。
/// 占位 `Texture` 只有 `id/width/height`：句柄取 `ManagedHandle(id)`、extent 取 `width/height`（I）。
impl TextureAccess for &Texture {
    fn as_texture(&self, ctx: &TextureCtx<'_>) -> TextureBinding {
        let handle = ManagedHandle(self.id);
        let view = ctx
            .device
            .get_vk_image_view(&handle)
            .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"));
        TextureBinding::Managed {
            extent: Extent2D {
                width: self.width,
                height: self.height,
            },
            view,
            handle,
        }
    }
    fn dims(&self, _ctx: &TextureCtx<'_>) -> Extent2D {
        Extent2D {
            width: self.width,
            height: self.height,
        }
    }
}

/// 时间对的只读句柄：解引用到内部 `Handle`（I：原版无独立实例，内联为同一路径）。
impl TextureAccess for &ReadOnlyHandle<Texture> {
    fn as_texture(&self, ctx: &TextureCtx<'_>) -> TextureBinding {
        (&self.0).as_texture(ctx)
    }
    fn dims(&self, ctx: &TextureCtx<'_>) -> Extent2D {
        (&self.0).dims(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::SsboType as LibSsbo;
    use crate::render::ssbo::{StorageElement, TypedStorageBuffer};
    use std::cell::RefCell;

    #[test]
    fn is_write_is_the_0x28_to_0x37_window() {
        assert!(!access_is_write(0x27));
        assert!(access_is_write(0x28));
        assert!(access_is_write(0x32));
        assert!(access_is_write(0x34));
        assert!(access_is_write(0x37));
        assert!(!access_is_write(0x38));
        assert!(!access_is_write(ACCESS_READ));
        assert!(!access_is_write(ACCESS_INDIRECT_READ));
        assert!(access_is_write(ACCESS_WRITE));
    }

    #[test]
    fn observed_access_constants() {
        assert_eq!(ACCESS_INDIRECT_READ, 0x02);
        assert_eq!(ACCESS_READ, 0x1a);
        assert_eq!(ACCESS_WRITE, 0x37);
        assert_eq!(ACCESS_CPU_MAPPED_WRITE, 0x32);
        assert_eq!(ACCESS_DRAW_READ, 0x1f);
        assert_eq!(ACCESS_TRANSFER_WRITE, 0x34);
        assert_eq!((SYNC_GROUP_DEFAULT, SYNC_GROUP_READ), (0, 1));
    }

    #[test]
    fn binding_tags_match_the_two_bind_constructors() {
        assert_eq!(PassBinding::BufferRead { resource: 1 }.tag(), 3);
        assert_eq!(PassBinding::Buffer { resource: 1 }.tag(), 4);
        let c = PassBinding::Buffer { resource: 9 }.to_contract();
        assert_eq!((c.resource_index, c.access_type), (9, 4));
    }

    #[test]
    fn binding_index_limit_is_0x100() {
        let full = vec![PassBinding::Buffer { resource: 0 }; 0xff];
        assert_eq!(next_binding_index(&full), 0xff);
        let over = vec![PassBinding::Buffer { resource: 0 }; 0x100];
        let r = std::panic::catch_unwind(|| next_binding_index(&over));
        assert!(r.is_err());
    }

    #[test]
    fn rg_texture_dims_and_tag() {
        let t = RgTexture {
            resource: 7,
            desc: TextureDesc::new([640, 480, 1], 0, 0),
        };
        let ctx = NoCtx::new();
        let b = (&t).as_texture(&ctx.ctx());
        assert_eq!(b.tag(), 0);
        assert_eq!(
            b.extent(),
            Extent2D {
                width: 640,
                height: 480
            }
        );
        assert_eq!(
            (&t).dims(&ctx.ctx()),
            Extent2D {
                width: 640,
                height: 480
            }
        );
        let w = WriteTextureUnchecked(t);
        assert_eq!(w.as_texture(&ctx.ctx()), b);
    }

    struct FakeDevice {
        fail: bool,
        asked: RefCell<Vec<u64>>,
    }
    impl TextureDevice for FakeDevice {
        fn get_vk_image_view(&self, h: &ManagedHandle) -> Result<u64, BackendError> {
            self.asked.borrow_mut().push(h.0);
            if self.fail {
                Err(BackendError::DeviceLost)
            } else {
                Ok(0xABC0 + h.0)
            }
        }
        fn managed_texture_desc(&self, _: &ManagedHandle) -> TextureDesc {
            TextureDesc::new([256, 128, 1], 0, 0)
        }
    }

    struct NoCtx {
        res: PassResources,
        dev: FakeDevice,
    }
    impl NoCtx {
        fn new() -> Self {
            NoCtx {
                res: PassResources::default(),
                dev: FakeDevice {
                    fail: false,
                    asked: RefCell::new(vec![]),
                },
            }
        }
        fn ctx(&self) -> TextureCtx<'_> {
            TextureCtx {
                resources: &self.res,
                device: &self.dev,
            }
        }
    }

    #[test]
    fn managed_handle_tag_1_clones_handle_and_keeps_view() {
        let ctx = NoCtx::new();
        let h: Handle<Texture> = Handle(ManagedHandle(5), core::marker::PhantomData);
        let b = (&h).as_texture(&ctx.ctx());
        assert_eq!(b.tag(), 1);
        assert_eq!(
            b,
            TextureBinding::Managed {
                extent: Extent2D {
                    width: 256,
                    height: 128
                },
                view: 0xABC5,
                handle: ManagedHandle(5)
            }
        );
        assert_eq!(
            (&h).dims(&ctx.ctx()),
            Extent2D {
                width: 256,
                height: 128
            }
        );
        // ReadOnlyHandle 走同一路径。
        let ro = ReadOnlyHandle(h.clone());
        assert_eq!((&ro).as_texture(&ctx.ctx()), b);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn managed_handle_view_failure_unwraps() {
        let mut ctx = NoCtx::new();
        ctx.dev.fail = true;
        let h: Handle<Texture> = Handle(ManagedHandle(1), core::marker::PhantomData);
        let _ = (&h).as_texture(&ctx.ctx());
    }

    // ---- *_access_buffer<T>：用库里真实的 AssetSsboLibrary ----
    #[derive(Clone, Copy)]
    struct Probe;
    impl StorageElement for Probe {
        const NAME: &'static str = "ProbeSsbo";
        const BUFFER_LEN: usize = 1;
        fn default_value() -> Option<Self> {
            Some(Probe)
        }
    }
    impl LibSsbo for Probe {
        type Storage = TypedStorageBuffer<Probe>;
        fn new_storage_buffer(
            device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
            initial: Option<&[u8]>,
        ) -> Self::Storage {
            match initial {
                None => TypedStorageBuffer::<Probe>::new(device, None),
                Some(b) => TypedStorageBuffer::<Probe>::new_storage_buffer(device, Some(b)),
            }
        }
    }

    #[test]
    fn read_write_concurrent_register_access_and_binding() {
        let lib = AssetSsboLibrary::new();
        lib.add_new::<Probe>(&crate::render::test_device::TestDevice(1));
        let mut res = PassResources::default();
        res.ssbo_library = &lib;
        let mut b = RecordingPassBuilder::default();
        let mut v = Vec::new();

        let i0 = read_access_buffer::<Probe>(&res, &mut b, &mut v, ACCESS_READ);
        let i1 = write_access_buffer::<Probe>(&res, &mut b, &mut v, ACCESS_CPU_MAPPED_WRITE);
        let i2 = write_access_buffer_concurrent::<Probe>(&res, &mut b, &mut v, ACCESS_WRITE, 5);
        assert_eq!((i0, i1, i2), (0, 1, 2));
        let r = b.accesses[0].resource;
        assert_eq!(
            b.accesses,
            [
                ResourceAccess {
                    resource: r,
                    access: 0x1a,
                    group: 1
                },
                ResourceAccess {
                    resource: r,
                    access: 0x32,
                    group: 0
                },
                ResourceAccess {
                    resource: r,
                    access: 0x37,
                    group: 5
                },
            ]
        );
        assert_eq!(
            v,
            [
                PassBinding::BufferRead { resource: r },
                PassBinding::Buffer { resource: r },
                PassBinding::Buffer { resource: r }
            ]
        );
        // 同一缓冲只向渲染图导入一次（库缓存了句柄）。
        assert_eq!(b.render_graph.imports.len(), 1);
    }

    #[test]
    fn access_is_registered_before_the_binding_limit_check() {
        let lib = AssetSsboLibrary::new();
        lib.add_new::<Probe>(&crate::render::test_device::TestDevice(1));
        let mut res = PassResources::default();
        res.ssbo_library = &lib;
        let mut b = RecordingPassBuilder::default();
        let mut v = vec![PassBinding::Buffer { resource: 0 }; 0x100];
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            read_access_buffer::<Probe>(&res, &mut b, &mut v, ACCESS_READ)
        }));
        assert!(r.is_err());
        assert_eq!(
            b.accesses.len(),
            1,
            "add_resource_access runs before the cmp 0x100 check"
        );
        assert_eq!(v.len(), 0x100);
    }
}
