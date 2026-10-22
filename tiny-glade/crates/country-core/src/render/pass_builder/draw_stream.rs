//! draw_stream.rs — DrawPacket / RasterPass / DrawPassRenderCheck / TextureBindingKey 与 DrawBuilder 所需的宿主边界。
//!
//! O = 机器码观测，I = 推断，U = 未知（不编造）。全部地址取自 B 版 EXE（见 B_a4_draw.md）。
//!
//! # 已核对的过程
//! - O `DrawPacket::new` 0x140867850（238 B）：`(out, shader: u64 @0xa0, head0 @0, head1 @8)`；
//!   `+0x10` 空 Vec（元素 8 B：`u32 binding + u8 slot`，对齐 4）；`+0x28` 空 `Vec<u8>`；`+0x40 = 0x1_0000_0003`
//!   （draw 种类 3，值 1）；`+0x70` 空 `HashMap<BindingIndex, GpuOwned<DescriptorSetBinding>>`（RawTable 0x20 B +
//!   `RandomState` @0x90）；`+0xa8` = `rhapsody::pipeline::impl$7::default()`（u32 打包的光栅状态）；
//!   `+0xac = 1.0f32`；`+0xb0 = 0u16`（`variant` 在 `DrawBuilder::new` 里被改写）。
//! - O `TextureBindingKey::with_view` 0x140867940（33 B）：`key ^ calculate_hash(view)`（view 为 None 时原样返回）。
//! - O `impl$8::to_texture_binding_key` 0x1408eefd0（61 B）：`unreachable!("Raw textures must not be used as rg resources")`
//!   （draw_stream.rs:204:9）。
//! - O `drop_in_place<DrawPassRenderCheck>` 0x140896d80：`!rendered && !thread::panicking()` →
//!   `panic!("RasterPass dropped without a call to render()")`（draw_stream.rs:262:13）。
//! - O `RasterPass::new_impl` 0x140867970（2560 B）、`RasterPass::render` 0x140868370（2423 B）：见各方法注释。
//!
//! # RasterPass 布局（O，0x3e8 B）
//! `+0x00` draws `Vec<DrawPacket(0xb8)>`；`+0x18` bindings `Vec<RenderPassBinding(0x44)>`（≤ 0x100）；
//! `+0x30` `HashMap<TextureBindingKey, u8>`（+RandomState @0x50）；`+0x60` `HashMap<&str, u8>`（+RandomState @0x80）；
//! `+0x90` PassBuilder（0x50 B，`+0xd0` = rg 资源信息 `rg::resource_info::from(device.+0xa0)`）；
//! `+0xe0` 深度视图（u32 tag，2 = 无深度）…`+0x12d`；`+0x130` 描述符集克隆；`+0x1a8` `PassResources`（7 个指针）；
//! `+0x1e0` 设备引用、`+0x1e8` 渲染 pass 缓存引用、`+0x1f0` world、`+0x1f8` 视口尺寸引用；
//! `+0x200` `ArrayVec<RasterPassColorAttachment(0x60), 5>`（len @0x3e0）；`+0x3e4` 已渲染标志。
//!
//! # 与旧版的差异（均为机器码证据）
//! 旧 `DrawPacket` 的 `texture_bindings: HashMap<TextureBindingKey, ()>`、`blend` 以外字段、
//! `TextureBindingKey{texture:u32, view_hash:u64}`、`&Texture` 的键转换实现都与机器码不符，已重写；
//! 旧 `RasterPass::get_render_pass` 的占位已落实为 `render` 内联的 entry 去重。

use alloc::sync::Arc;
use alloc::vec::Vec;
use hashbrown::HashMap;

use crate::render::pass_builder::framebuffer_binding::{
    ColorAttachment, ColorViewWords, DepthAttachment, DepthClear,
};
use crate::render::pass_builder::rg_resources::RgResources;
use crate::render::rg_helpers::rg_boundary::ResourceLease;
use crate::render::shader::RasterShaderProgram;
use crate::rhapsody_contract::{Handle, ManagedHandle, ReadOnlyHandle, RenderPassBinding};
use crate::rhapsody_types::{Texture, TextureViewDesc};

// ------------------------------------------------------------------------------------------
// 光栅状态 / 绑定
// ------------------------------------------------------------------------------------------

/// `DrawPacket +0xa8`：打包的光栅状态 u32（`shader::impl$7::default()` 的返回值）。
/// O：`raster_single_object` 对其做 `byte0 &= 0x7f`、`byte3 = (byte3 & 0x9f) | 0x40`；各位的名称 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RasterState(pub u32);

impl RasterState {
    /// O（`raster_single_object` 0x1409ce43d）：清除第 0 字节的 bit7。
    pub fn clear_byte0_bit7(self) -> RasterState {
        RasterState(self.0 & !0x80)
    }
    /// O（0x1409ce430/0x1409ce433）：第 3 字节的 bit5..6 设为 0b10。
    pub fn set_byte3_field_2(self) -> RasterState {
        RasterState((self.0 & !0x6000_0000) | 0x4000_0000)
    }
}

/// `rhapsody::command::DescriptorSetBinding` 中本模块观测到的变体（O）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescriptorSetBinding {
    /// 判别 0（`bind_texture_to_draw_builder` raw 路径）：`{0, image_view, 5}`，keepalive 句柄 @+0x28。
    SampledImage {
        image_view: u64,
        layout_word: u64,
        keepalive: ManagedHandle,
    },
    /// 判别 5（`bind_external_storage_buffer`）：`{vk_buffer, 0, u64::MAX}`，keepalive 句柄。
    StorageBuffer {
        buffer: u64,
        offset: u64,
        range: u64,
        keepalive: ManagedHandle,
    },
    /// `DynamicConstantsChunk::into_uniform_buffer_binding_and_data` 产出的 uniform 缓冲绑定
    /// （`DrawBuilder::new` 插入；判别值与载荷字段 U，只知它不是 0xa 这个 `None` niche）。
    UniformBuffer {
        size: u32,
        keepalive: ManagedHandle,
        lease: Option<ResourceLease>,
    },
}

/// `GpuOwned<DescriptorSetBinding>`（0x48 B 桶里的 `+0x40` 状态字节：0 = 持有，2 = 已 `discard` 解除）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuOwned {
    pub binding: DescriptorSetBinding,
    /// O：`DrawBuilder::discard` 把所有桶的该字节写成 2。
    pub state: u8,
}

impl GpuOwned {
    pub const STATE_OWNED: u8 = 0;
    pub const STATE_DISCARDED: u8 = 2;
    pub fn new(binding: DescriptorSetBinding) -> Self {
        GpuOwned {
            binding,
            state: Self::STATE_OWNED,
        }
    }
}

/// `+0x10` Vec 的元素（8 B）：着色器绑定序号 + RasterPass.bindings 里的槽下标。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceAccess {
    pub binding_index: u32,
    pub slot: u8,
}

// ------------------------------------------------------------------------------------------
// 绘制种类
// ------------------------------------------------------------------------------------------

/// `DrawPacket +0x40`：draw 种类（u32 判别 + 载荷 @+0x44..）。各判别值均来自对应 `DrawBuilder` 方法的写入。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DrawKind {
    /// 0（`draw_no_vao`）：`vertex_count`。
    NoVao { vertex_count: u32 },
    /// 1（`draw_no_vao_indexed`）：索引数 @0x44 + 索引缓冲句柄克隆 @0x48。
    NoVaoIndexed {
        index_count: u32,
        index_buffer: ManagedHandle,
    },
    /// 2（`draw_no_index_buffer`）。
    NoIndexBuffer { vertex_count: u32 },
    /// 3（`draw_instanced_w_gl_draw_flag`；`DrawPacket::new` 的缺省值，flag = 1）。
    InstancedGlDrawFlag { flag: u32 },
    /// 4（`draw_multi_instanced`）：`Vec<[u32; 2]>`（cap @0x48、ptr @0x50、len @0x58，元素 8 B/对齐 4）。
    MultiInstanced { ranges: Vec<[u32; 2]> },
    /// 5（`multi_draw_indirect`）：间接缓冲资源下标 @0x44、`0` @0x48、`arg_50` @0x50、`arg_58` @0x58。
    MultiDrawIndirect {
        buffer: u32,
        arg_50: u64,
        arg_58: u64,
    },
    /// 7（`multi_draw_indexed_indirect` / `draw_indexed_indirect_raw`）。
    /// `first_index_count`：`+0x60 == 1` 时 `+0x64` 取自静态表（见 `draw_builder::INDEXED_INDIRECT_INDEX_COUNTS`）。
    IndexedIndirect {
        buffer: u32,
        arg_48: u64,
        arg_50: u64,
        arg_58: u64,
        first_index_count: Option<u32>,
    },
    /// 8（`draw_indexed_indirect_count`）：draw 缓冲 @0x44、count 缓冲 @0x48、四个 u64 @0x50/0x58/0x60/0x68。
    IndexedIndirectCount {
        buffer: u32,
        count_buffer: u32,
        args: [u64; 4],
    },
}

impl DrawKind {
    /// 机器码里写入 `+0x40` 的判别值。
    pub fn discriminant(&self) -> u32 {
        match self {
            DrawKind::NoVao { .. } => 0,
            DrawKind::NoVaoIndexed { .. } => 1,
            DrawKind::NoIndexBuffer { .. } => 2,
            DrawKind::InstancedGlDrawFlag { .. } => 3,
            DrawKind::MultiInstanced { .. } => 4,
            DrawKind::MultiDrawIndirect { .. } => 5,
            DrawKind::IndexedIndirect { .. } => 7,
            DrawKind::IndexedIndirectCount { .. } => 8,
        }
    }
}

// ------------------------------------------------------------------------------------------
// DrawPacket
// ------------------------------------------------------------------------------------------

/// 绘制包（0xb8 B）。
#[derive(Debug)]
pub struct DrawPacket {
    /// `+0x00`/`+0x08`：`DrawPacket::new` 的第 3/4 个参数（`r8`/`r9`）。`+0` 是 `NoDropOnlyTake` 的 niche
    /// （值 2 = 已取走），所以它是个至少含 0/1 两个取值的枚举；含义 U。
    pub head: [u64; 2],
    /// `+0x10`：`Vec<ResourceAccess>`。
    pub accesses: Vec<ResourceAccess>,
    /// `+0x28`：`Vec<u8>`（含义 U）。
    pub unknown_bytes: Vec<u8>,
    /// `+0x40`。
    pub kind: DrawKind,
    /// `+0x70`：着色器 binding 序号 → 描述符绑定。
    pub bindings: HashMap<u32, GpuOwned>,
    /// `+0xa0`：`Handle<RasterShaderProgram>`。
    pub shader: Handle<RasterShaderProgram>,
    /// `+0xa8`。
    pub state: RasterState,
    /// `+0xac`（缺省 1.0；O：`0x3f800000`）。
    pub blend: f32,
    /// `+0xb0`：着色器变体下标（`DrawBuilder::new` 写入，`variants[variant << 6]`）。
    pub variant: u8,
    /// `+0xb1`。
    pub unknown_b1: u8,
    /// Current graph-host metadata, not additional native DrawPacket fields.
    /// Binding indices refer to the actual packet accesses below the original
    /// shader reflection; ordinary packets retain strict content checks.
    pub preserving_buffer_bindings: Vec<u32>,
    pub inactive_texture_bindings: Vec<u32>,
    /// 宿主逐 binding 内容许可：真实只读 fetch 可读取未定义内容，仍保留同步与已有 producer。
    pub undefined_texture_bindings: Vec<u32>,
}

impl DrawPacket {
    /// `new`（0x140867850）。`default_state` 对应 `shader::impl$7::default()`（由宿主给出，原为外部库调用）。
    pub fn new(
        shader: Handle<RasterShaderProgram>,
        head: [u64; 2],
        default_state: RasterState,
    ) -> DrawPacket {
        DrawPacket {
            head,
            accesses: Vec::new(),
            unknown_bytes: Vec::new(),
            kind: DrawKind::InstancedGlDrawFlag { flag: 1 },
            bindings: HashMap::new(),
            shader,
            state: default_state,
            blend: 1.0,
            variant: 0,
            unknown_b1: 0,
            preserving_buffer_bindings: Vec::new(),
            inactive_texture_bindings: Vec::new(),
            undefined_texture_bindings: Vec::new(),
        }
    }

    /// 方便调用（consumer 约定）：`variant` 读取。
    pub fn variant(&self) -> u8 {
        self.variant
    }
}

// ------------------------------------------------------------------------------------------
// TextureBindingKey
// ------------------------------------------------------------------------------------------

/// `RasterPass +0x30` 表的键（u64）。O：`bind_texture_to_draw_builder` 里
/// `key = 0x4000_0000_0000_0000 | DecoratorRank::new(texture_id)`，有 view 时再 `^ calculate_hash(view)`。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureBindingKey(pub u64);

impl TextureBindingKey {
    /// 句柄类 RG 纹理键的高位标记（O：`movabs r14, 0x4000000000000000`）。
    pub const RG_TAG: u64 = 0x4000_0000_0000_0000;

    /// `0x4000_0000_0000_0000 | id`（O；`DecoratorRank::new` 是 ICF 别名，I：对 u32 恒等）。
    pub fn from_rg_texture_id(id: u32) -> TextureBindingKey {
        TextureBindingKey(Self::RG_TAG | id as u64)
    }

    /// `with_view`（0x140867940）：`key ^ calculate_hash(view)`；`None` 原样返回。
    pub fn with_view(self, view: Option<&TextureViewDesc>) -> TextureBindingKey {
        match view {
            Some(v) => TextureBindingKey(self.0 ^ crate::render::utils::calculate_hash(v)),
            None => self,
        }
    }
}

/// 纹理 → 绘制流绑定键。
pub trait ToTextureDrawStreamBindingKey {
    fn to_texture_binding_key(&self) -> TextureBindingKey;
}

impl ToTextureDrawStreamBindingKey for &Texture {
    /// O 0x1408eefd0：恒 panic。
    fn to_texture_binding_key(&self) -> TextureBindingKey {
        unreachable!("Raw textures must not be used as rg resources")
    }
}

impl ToTextureDrawStreamBindingKey for &Handle<Texture> {
    fn to_texture_binding_key(&self) -> TextureBindingKey {
        TextureBindingKey::from_rg_texture_id(self.handle())
    }
}

impl ToTextureDrawStreamBindingKey for &ReadOnlyHandle<Texture> {
    fn to_texture_binding_key(&self) -> TextureBindingKey {
        TextureBindingKey::from_rg_texture_id(self.handle())
    }
}

// ------------------------------------------------------------------------------------------
// DrawPassRenderCheck
// ------------------------------------------------------------------------------------------

/// `RasterPass +0x3e4`：drop 时断言 pass 已 `render`。
#[derive(Debug)]
pub struct DrawPassRenderCheck(pub bool);

impl Drop for DrawPassRenderCheck {
    /// O 0x140896d80：未渲染且当前线程不在 panic 中 → panic。
    fn drop(&mut self) {
        if !self.0 && !std::thread::panicking() {
            panic!("RasterPass dropped without a call to render()");
        }
    }
}

// ------------------------------------------------------------------------------------------
// 着色器反射 / 动态常量 / 渲染 pass 描述（外部库的最小替身）
// ------------------------------------------------------------------------------------------

/// `shader_pipeline::compiled::CompiledShaderPipeline` 中本模块读取的反射字段（最小替身，
/// `rhapsody_types::CompiledShaderPipeline` 只是不透明字节，无法承载这些字段）。
/// 偏移为 O：名字表 @+0x98（items @+0xb0，桶 0x10 B：`hash u64, set u32, binding u32`）、
/// uniform 块 @+0xe8（非 0 才有）、uniform 偏移表 @+0x118（items @+0x130，桶 0x10 B：`hash u64, offset u32`）、
/// 块大小 @+0x138、块 binding @+0x140。
#[derive(Clone, Debug, Default)]
pub struct ShaderReflection {
    /// `RasterShaderProgram` 名（`+0x48`，`DrawBuilder` drop 的 panic 文案用）。
    pub program_name: &'static str,
    /// 名字 hash → `(set, binding)`。
    pub bindings: HashMap<u64, (u32, u32)>,
    /// `+0xe8 != 0`。
    pub has_uniform_block: bool,
    /// 名字 hash（含后缀）→ 块内偏移。
    pub uniform_offsets: HashMap<u64, u32>,
    pub uniform_block_size: u32,
    pub uniform_block_binding: u32,
}

/// 帧内动态常量块（`DynamicConstants::allocate` + `into_uniform_buffer_binding_and_data`）。
/// 原版持有可写映射切片 + GPU 绑定；这里用拥有的字节缓冲代表该切片。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicConstantsBinding {
    pub data: Vec<u8>,
    /// `HashMap::insert(BindingIndex, GpuOwned<_>)` 的键（`reflection.+0x140`）。
    pub binding_index: u32,
    pub keepalive: ManagedHandle,
    pub lease: Option<ResourceLease>,
}

/// `rhapsody::pipeline::RenderPassAttachmentDesc`（0x10 B）的最小替身。
/// O：`new(format)`、`.sample_count(n)`、`.clear_input()`；`+4` 被覆盖为附件 tag；`+0xc` 的字节在
/// `take_while` 循环里决定是否继续收集。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct AttachmentDesc {
    pub format: u32,
    pub sample_count: u8,
    pub kind_tag: u32,
    pub clear_input: bool,
    /// `+0xc != 0`（I：`format != 0`）。
    pub valid: bool,
}

impl AttachmentDesc {
    /// `RenderPassAttachmentDesc::new(format)`。I：`valid = format != 0`。
    pub fn new(format: u32) -> Self {
        AttachmentDesc {
            format,
            valid: format != 0,
            ..Default::default()
        }
    }
    pub fn with_sample_count(mut self, n: u8) -> Self {
        self.sample_count = n;
        self
    }
    pub fn with_clear_input(mut self) -> Self {
        self.clear_input = true;
        self
    }
}

/// 深度字节为 3 表示“无深度”（O：`mov al, 3`）。
pub const DEPTH_STATE_NONE: u8 = 3;

/// `RenderPassDesc`（缓存键）：`[AttachmentDesc; 8]`(0x80) + count(u32) + depth(0x10) + depth_state(u8)（O 布局）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RasterRenderPassDesc {
    pub colors: [AttachmentDesc; 8],
    pub color_count: u32,
    pub depth: AttachmentDesc,
    /// `+0x12c`（0 = 只读 / 1 = 可写，I）或 `DEPTH_STATE_NONE`。
    pub depth_state: u8,
}

/// `vk_sync_fork::AccessType::is_write`（O 0x140c10e10，10 B）：`(access - 0x28) < 0x10`。
pub fn access_is_write(access: u8) -> bool {
    access.wrapping_sub(0x28) < 0x10
}

/// `PassBuilder::render<PassResources, closure>` 的捕获（O：`render` 把这些整体搬进闭包环境）。
#[derive(Debug)]
pub struct RasterPassSubmission {
    pub name: &'static str,
    pub draws: Vec<DrawPacket>,
    pub bindings: Vec<RenderPassBinding>,
    pub texture_slots: HashMap<TextureBindingKey, u8>,
    pub buffer_slots: HashMap<&'static str, u8>,
    pub render_pass: ManagedHandle,
    pub accesses: Vec<PassAccess>,
    pub color_count: u32,
    /// 宿主接缝（非原字段）：颜色附件（RG 资源号/格式/tag）。原版 `render` 的闭包环境同样带着 `RasterPass +0x200`
    /// 的附件数组（I：未逐字节核对闭包环境布局），回调里据此取附件纹理建帧缓冲。
    pub colors: Vec<RasterPassColorAttachment>,
    /// 深度附件（`RasterPass +0xe0..`，I，同上）。
    pub depth: Option<RasterPassDepth>,
    /// `RasterPass +0x1f8` 的视口尺寸（I，同上）。
    pub viewport: [u32; 2],
}

/// `PassBuilder::add_resource_access(resource, access, group)` 的记录。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassAccess {
    pub resource: u32,
    pub access: u8,
    pub group: u32,
}

/// `GraphicsDeviceResource` / `PassResources(7 个指针)` / 游戏资源库在本模块被读取的全部外部调用面
/// （O 调用点；实现者是 rhapsody 与游戏资源库）。
pub trait PassHost {
    /// `FrozenMap<Handle<RasterShaderProgram>, Box<RasterShaderProgram>>::get` + `variants[variant].compiled`
    /// （`PassResources +0x1a8`）。缺失返回 `None`。
    fn raster_shader(
        &self,
        shader: &Handle<RasterShaderProgram>,
        variant: u8,
    ) -> Option<Arc<ShaderReflection>>;
    /// `Components::get_resource_id` → `Mutex<DynamicConstants>::lock` → `allocate` →
    /// `into_uniform_buffer_binding_and_data`。`reflection` 无 uniform 块时 `DrawBuilder::new` 不会调用。
    fn allocate_dynamic_constants(
        &mut self,
        reflection: &ShaderReflection,
    ) -> Option<DynamicConstantsBinding>;
    /// `AssetSsboLibrary::get_rg_buffer<T>`：SSBO 类型的 rg 缓冲资源下标（`resource_index`）。
    fn rg_buffer_resource(&self, rg_buffer_hash: u64) -> Option<u32>;
    /// Preserve the original get_rg_buffer<T> full-type assertion in source
    /// hosts whose dynamic dispatch otherwise only receives the NAME hash.
    fn rg_buffer_resource_typed(
        &self,
        rg_buffer_hash: u64,
        _type_name: &'static str,
    ) -> Option<u32> {
        self.rg_buffer_resource(rg_buffer_hash)
    }
    /// `PlacementDataSsboLibrary` 表（`PassResources +0x1c8`）：`GlobalId` → `(缓冲句柄, flag)`。
    fn placement_buffer(&self, id: u64) -> Option<(ManagedHandle, u32)>;
    /// `GraphicsDevice::get_vk_buffer`。
    fn vk_buffer(&self, buffer: &ManagedHandle) -> Option<u64>;
    /// `GraphicsDevice::get_vk_image_view`（`Err` → 调用点 `Result::unwrap` panic）。
    fn vk_image_view(&self, texture: &ManagedHandle) -> Result<u64, String>;
    /// Native TextureFromDisk can select an image-array layer. Hosts must
    /// preserve the descriptor instead of silently using a whole-image view.
    fn vk_image_view_with_desc(
        &self,
        texture: &ManagedHandle,
        view: Option<&TextureViewDesc>,
    ) -> Result<u64, String> {
        if view.is_some() {
            return Err("explicit disk texture view is unsupported by this host".into());
        }
        self.vk_image_view(texture)
    }

    /// `AssetTextureLibrary::get_by_name`（`TextureFromDisk`）：返回 `(纹理句柄, 描述)`。
    fn texture_by_name(
        &self,
        name_hash: u64,
    ) -> (
        ManagedHandle,
        crate::render::pass_builder::framebuffer_binding::AttachmentTexture,
    );
    /// `AssetTextureLibrary::get_rg_texture`（`TexturePreset`）。
    fn rg_texture(
        &self,
        preset: u8,
    ) -> crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
    /// `shader_pipeline::reflection::BindingNameHash::with_suffix_dims`（外部库；算法 U）。
    fn with_suffix_dims(&self, name_hash: u64) -> u64;
    /// `rhapsody::pipeline::impl$7::default()`。
    fn default_raster_state(&self) -> RasterState;
    /// `GraphicsDevice::create_render_pass`。
    fn create_render_pass(&mut self, desc: &RasterRenderPassDesc) -> ManagedHandle;
    /// `GraphicsDeviceResource +0x188`：`HashMap<RenderPassDesc, RenderPass>` 缓存（`render` 的 entry 去重）。
    fn render_pass_cache(&mut self) -> &mut HashMap<RasterRenderPassDesc, ManagedHandle>;
    /// `PassBuilder::render<PassResources, closure>`：注册 pass。
    fn submit_raster_pass(&mut self, submission: RasterPassSubmission);

    // ---- B 阶段追加（i_api）：均为带缺省实现的“可选能力”，已有实现者（`RecHost` 等）无需改动 ----

    /// `InstanceBuffer<T>::instance_count` 的低 32 位（`bind_ray_tracing<DrawBuilder>` 的两个 `write_uniform<u32>` 的值）。
    /// `rg_buffer_hash` = `SsboId`（`xxh3_64(NAME)`）；宿主负责 `assert_eq!(state.name, type_name::<T>())`。缺省 `None`
    /// （调用点随后 `Option::unwrap` panic，与原版“库里没有该缓冲”同一路径）。
    fn ssbo_instance_count(&self, _rg_buffer_hash: u64) -> Option<u32> {
        None
    }
    fn ssbo_instance_count_typed(&self, hash: u64, _type_name: &'static str) -> Option<u32> {
        self.ssbo_instance_count(hash)
    }

    /// Compatibility for consumers outside the independently recovered RT
    /// count contract. RT binding exclusively uses ssbo_instance_count.
    fn ssbo_buffer_word(&self, _rg_buffer_hash: u64) -> Option<u32> {
        None
    }

    /// RG 资源创建面（`RenderGraph::create_raw_resource` / `get_or_create_temporal*`，见 `rg_resources.rs`）。
    /// 缺省 `None`：宿主不接渲染图（离线测试）。
    fn rg_resources(&mut self) -> Option<&mut dyn RgResources> {
        None
    }

    /// 接线补口（非原版函数，**宿主接缝**）：`DrawBuilder` 终结（`draw*`）时把 `constants.data` 交回宿主。
    ///
    /// 原版里 `DynamicConstantsBinding.data` 是动态常量页的**可写映射切片**（`write_uniform` 直接写 GPU 可见内存），
    /// 重建版为了不碰裸指针把它建模成拥有的 `Vec<u8>`——于是 `DrawBuilder` 被 `finish` 消费时这份字节随
    /// builder 一起丢弃，uniform 值永远到不了 GPU。此回调就是“映射切片”的等价物：宿主在这里把字节写进
    /// `keepalive` 对应的常量页。缺省空实现（离线测试宿主不需要）。
    fn flush_dynamic_constants(&mut self, _constants: &DynamicConstantsBinding) {}
}

/// `PassHost::rg_resources` 的必选版本：调用点原版没有“可选”概念，宿主缺失 = 装配错误。
pub fn require_rg_resources(host: &mut dyn PassHost) -> &mut dyn RgResources {
    host.rg_resources()
        .expect("PassHost does not provide RG resources (no render graph attached)")
}

// ------------------------------------------------------------------------------------------
// RasterPass
// ------------------------------------------------------------------------------------------

/// `RasterPassColorAttachment`（0x60 B）里本模块读取的字段：`+0x50` id、`+0x54` format、`+0x58` tag、`+0x5c` samples。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RasterPassColorAttachment {
    pub id: u32,
    pub format: u32,
    pub tag: u32,
    pub sample_count: u8,
    /// Exact VkClearColorValue bits; historical field name retained for source compatibility.
    pub view: ColorViewWords,
}

impl RasterPassColorAttachment {
    pub fn clear_color_bits(&self) -> [u32; 4] {
        self.view
    }
}

/// 深度状态（`RasterPass +0xe0..+0x12e`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RasterPassDepth {
    pub texture_id: u32,
    pub format: u32,
    pub sample_count: u8,
    pub clear: DepthClear,
    pub writable: bool,
    pub layer: u32,
}

/// `ArrayVec<_, 5>` 容量（O：`from_iter<RasterPassColorAttachment, 5, _>`）。
pub const MAX_COLOR_ATTACHMENTS: usize = 5;
/// `render` 里 `ArrayVec<AttachmentDesc, 8>` 的容量（O：`cmp rsi, 0x80`）。
pub const MAX_RENDER_PASS_COLORS: usize = 8;

/// RasterPass（0x3e8 B）。
pub struct RasterPass {
    pub(crate) name: &'static str,
    pub(crate) draws: Vec<DrawPacket>,
    pub(crate) bindings: Vec<RenderPassBinding>,
    pub(crate) texture_slots: HashMap<TextureBindingKey, u8>,
    pub(crate) buffer_slots: HashMap<&'static str, u8>,
    pub(crate) accesses: Vec<PassAccess>,
    pub(crate) depth: Option<RasterPassDepth>,
    pub(crate) colors: Vec<RasterPassColorAttachment>,
    pub(crate) host: alloc::boxed::Box<dyn PassHost>,
    /// `+0x1f8`：视口尺寸引用（O：`bind_common` 以它作 `optional_uniform<[u32; 2]>` 的值；`raster_single_object`
    /// 传入第一个附件的 `extent_2d`）。
    pub(crate) viewport: [u32; 2],
    pub(crate) rendered: DrawPassRenderCheck,
}

impl RasterPass {
    /// `new`（inline 42）→ `new_impl`（0x140867970）。
    ///
    /// O 顺序：先把颜色附件逐个映射并 `add_resource_access(id, tag0 ? 0x36 : 0x2e, 1)`（`ArrayVec` 上限 5，
    /// 超出 panic）；再看深度：tag0 → 访问 `0x2f`、tag1 → `0x17`（均同步组 1），其它 → 无深度；
    /// 深度视图 = `TextureViewDesc::default().level_count(1).layer_count(1).base_layer(layer)`；随后构造
    /// `PassResources`、克隆描述符集、创建两张空表（`RandomState`）。
    pub fn new(
        name: &'static str,
        attachments: &[ColorAttachment],
        depth: Option<&DepthAttachment>,
        host: alloc::boxed::Box<dyn PassHost>,
    ) -> RasterPass {
        assert!(
            attachments.len() <= MAX_COLOR_ATTACHMENTS,
            "ArrayVec: capacity exceeded in extend/from_iter"
        );
        let mut accesses = Vec::new();
        let mut colors = Vec::new();
        for a in attachments {
            let t = a.desc();
            accesses.push(PassAccess {
                resource: t.id,
                access: a.access_type(),
                group: 1,
            });
            colors.push(RasterPassColorAttachment {
                id: t.id,
                format: t.format,
                tag: a.tag(),
                sample_count: t.sample_count,
                view: a.clear_color_bits(),
            });
        }
        let depth = depth.map(|d| {
            accesses.push(PassAccess {
                resource: d.texture.id,
                access: d.access_type(),
                group: 1,
            });
            RasterPassDepth {
                texture_id: d.texture.id,
                format: d.texture.format,
                sample_count: d.texture.sample_count,
                clear: d.clear,
                writable: d.writable,
                layer: d.layer(),
            }
        });
        RasterPass {
            name,
            draws: Vec::new(),
            bindings: Vec::new(),
            texture_slots: HashMap::new(),
            buffer_slots: HashMap::new(),
            accesses,
            depth,
            colors,
            host,
            viewport: [0, 0],
            rendered: DrawPassRenderCheck(false),
        }
    }

    /// 宿主（`PassResources`/设备/渲染图的集合；`rg_resources()` 等从这里取）。
    pub fn host_mut(&mut self) -> &mut dyn PassHost {
        &mut *self.host
    }

    /// 设置视口尺寸（`+0x1f8`）。
    pub fn with_viewport(mut self, viewport: [u32; 2]) -> RasterPass {
        self.viewport = viewport;
        self
    }

    /// 已登记的 draw 数。
    pub fn draw_count(&self) -> usize {
        self.draws.len()
    }

    /// 构造 `RenderPassDesc`（O：`render` 前半段）。
    ///
    /// 颜色：`new(format).sample_count(n)`，`+4` 覆盖为附件 tag；遇到 `+0xc == 0` 的描述即停止收集，
    /// 超过 8 个 → `arrayvec::extend_panic`。深度：`new(format).sample_count(n)`，有清除值 → `clear_input`；
    /// 深度字节 = 可写标志，无深度 → 3。
    pub fn build_render_pass_desc(&self) -> RasterRenderPassDesc {
        let mut colors = [AttachmentDesc::default(); MAX_RENDER_PASS_COLORS];
        let mut n = 0usize;
        for c in &self.colors {
            let mut d = AttachmentDesc::new(c.format).with_sample_count(c.sample_count);
            d.kind_tag = c.tag;
            if !d.valid {
                break;
            }
            assert!(
                n < MAX_RENDER_PASS_COLORS,
                "ArrayVec: capacity exceeded in extend"
            );
            colors[n] = d;
            n += 1;
        }
        let (depth, depth_state) = match &self.depth {
            None => (AttachmentDesc::default(), DEPTH_STATE_NONE),
            Some(d) => {
                let mut desc = AttachmentDesc::new(d.format).with_sample_count(d.sample_count);
                if let DepthClear::Clear(_) = d.clear {
                    desc = desc.with_clear_input();
                }
                (desc, d.writable as u8)
            }
        };
        RasterRenderPassDesc {
            colors,
            color_count: n as u32,
            depth,
            depth_state,
        }
    }

    /// `render`（0x140868370）：置已渲染；构造 `RenderPassDesc`；用设备缓存的 `rustc_entry` 去重
    /// （命中 → 克隆句柄；未命中 → `create_render_pass` 后插入再克隆）；把全部 pass 状态交给
    /// `PassBuilder::render`。
    pub fn render(mut self) {
        let probe_started = crate::render::declare_probe::start();
        self.rendered.0 = true;
        let desc = self.build_render_pass_desc();
        let render_pass = match self.host.render_pass_cache().get(&desc) {
            Some(h) => h.clone(),
            None => {
                let created = self.host.create_render_pass(&desc);
                self.host.render_pass_cache().insert(desc, created.clone());
                created
            }
        };
        let submission = RasterPassSubmission {
            name: self.name,
            draws: core::mem::take(&mut self.draws),
            bindings: core::mem::take(&mut self.bindings),
            texture_slots: core::mem::take(&mut self.texture_slots),
            buffer_slots: core::mem::take(&mut self.buffer_slots),
            render_pass,
            accesses: core::mem::take(&mut self.accesses),
            color_count: desc.color_count,
            colors: core::mem::take(&mut self.colors),
            depth: self.depth.take(),
            viewport: self.viewport,
        };
        self.host.submit_raster_pass(submission);
        crate::render::declare_probe::render(probe_started);
    }
}

/// DrawStream（旧占位聚合体，无 PDB 过程；保留给既有 re-export）。
#[derive(Debug, Default)]
pub struct DrawStream {
    pub packets: Vec<DrawPacket>,
}

#[cfg(test)]
pub(crate) mod test_host {
    use super::*;
    use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::string::String;
    use core::cell::RefCell;

    /// 记录型宿主：用于断言机器码里观测到的外部调用。
    #[derive(Default)]
    pub struct Log {
        pub created_passes: Vec<RasterRenderPassDesc>,
        pub submissions: Vec<RasterPassSubmission>,
        pub constants_requests: u32,
        pub image_views: Vec<(u64, Option<TextureViewDesc>)>,
        /// `flush_dynamic_constants` 收到的（`keepalive`, 字节）。
        pub flushed_constants: Vec<(u64, Vec<u8>)>,
    }

    pub struct MockHost {
        pub log: Rc<RefCell<Log>>,
        pub shaders: HashMap<u64, Arc<ShaderReflection>>,
        pub rg_buffers: HashMap<u64, u32>,
        pub placement: HashMap<u64, (ManagedHandle, u32)>,
        pub vk_buffers: HashMap<u64, u64>,
        pub textures_by_name: HashMap<u64, (ManagedHandle, AttachmentTexture)>,
        pub presets: HashMap<u8, AttachmentTexture>,
        pub cache: HashMap<RasterRenderPassDesc, ManagedHandle>,
        pub next_handle: u64,
        pub constants: Option<usize>,
        /// `ssbo_instance_count`（`SsboId` → 实际实例数量）。
        pub buffer_words: HashMap<u64, u32>,
        /// `rg_resources`：记录型渲染图（`None` = 不接渲染图）。
        pub rg: Option<crate::render::pass_builder::rg_resources::RecordingRg>,
    }

    impl MockHost {
        pub fn new() -> (MockHost, Rc<RefCell<Log>>) {
            let log = Rc::new(RefCell::new(Log::default()));
            (
                MockHost {
                    log: log.clone(),
                    shaders: HashMap::new(),
                    rg_buffers: HashMap::new(),
                    placement: HashMap::new(),
                    vk_buffers: HashMap::new(),
                    textures_by_name: HashMap::new(),
                    presets: HashMap::new(),
                    cache: HashMap::new(),
                    next_handle: 100,
                    constants: None,
                    buffer_words: HashMap::new(),
                    rg: None,
                },
                log,
            )
        }
        pub fn boxed(self) -> Box<dyn PassHost> {
            Box::new(self)
        }
    }

    impl PassHost for MockHost {
        fn raster_shader(
            &self,
            shader: &Handle<RasterShaderProgram>,
            _variant: u8,
        ) -> Option<Arc<ShaderReflection>> {
            self.shaders.get(&(shader.0.0)).cloned()
        }
        fn allocate_dynamic_constants(
            &mut self,
            r: &ShaderReflection,
        ) -> Option<DynamicConstantsBinding> {
            self.log.borrow_mut().constants_requests += 1;
            let size = self.constants.unwrap_or(r.uniform_block_size as usize);
            Some(DynamicConstantsBinding {
                data: alloc::vec![0u8; size],
                binding_index: r.uniform_block_binding,
                keepalive: ManagedHandle(7),
                lease: None,
            })
        }
        fn rg_buffer_resource(&self, h: u64) -> Option<u32> {
            self.rg_buffers.get(&h).copied()
        }
        fn placement_buffer(&self, id: u64) -> Option<(ManagedHandle, u32)> {
            self.placement.get(&id).cloned()
        }
        fn vk_buffer(&self, b: &ManagedHandle) -> Option<u64> {
            self.vk_buffers.get(&b.0).copied()
        }
        fn vk_image_view(&self, t: &ManagedHandle) -> Result<u64, String> {
            Ok(0x1000 + t.0)
        }
        fn vk_image_view_with_desc(
            &self,
            t: &ManagedHandle,
            view: Option<&TextureViewDesc>,
        ) -> Result<u64, String> {
            self.log.borrow_mut().image_views.push((t.0, view.copied()));
            self.vk_image_view(t)
        }
        fn texture_by_name(&self, h: u64) -> (ManagedHandle, AttachmentTexture) {
            self.textures_by_name
                .get(&h)
                .cloned()
                .expect("No such texture")
        }
        fn rg_texture(&self, p: u8) -> AttachmentTexture {
            self.presets
                .get(&p)
                .copied()
                .expect("no entry found for key")
        }
        fn with_suffix_dims(&self, h: u64) -> u64 {
            h ^ 0xD1D1
        }
        fn default_raster_state(&self) -> RasterState {
            RasterState(0x8000_0080)
        }
        fn create_render_pass(&mut self, desc: &RasterRenderPassDesc) -> ManagedHandle {
            self.log.borrow_mut().created_passes.push(*desc);
            self.next_handle += 1;
            ManagedHandle(self.next_handle)
        }
        fn render_pass_cache(&mut self) -> &mut HashMap<RasterRenderPassDesc, ManagedHandle> {
            &mut self.cache
        }
        fn submit_raster_pass(&mut self, s: RasterPassSubmission) {
            self.log.borrow_mut().submissions.push(s);
        }
        fn flush_dynamic_constants(&mut self, c: &DynamicConstantsBinding) {
            self.log
                .borrow_mut()
                .flushed_constants
                .push((c.keepalive.0, c.data.clone()));
        }
        fn ssbo_instance_count(&self, h: u64) -> Option<u32> {
            self.buffer_words.get(&h).copied()
        }
        fn ssbo_buffer_word(&self, h: u64) -> Option<u32> {
            self.buffer_words.get(&h).copied()
        }
        fn rg_resources(&mut self) -> Option<&mut dyn RgResources> {
            self.rg.as_mut().map(|r| r as &mut dyn RgResources)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_host::MockHost;
    use super::*;
    use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;

    fn tex(id: u32, format: u32) -> AttachmentTexture {
        AttachmentTexture {
            id,
            extent: (8, 8),
            format,
            sample_count: 4,
        }
    }

    #[test]
    fn draw_packet_new_defaults_match_0x140867850() {
        let p = DrawPacket::new(Handle::gap_new(), [0, 0], RasterState(5));
        assert_eq!(p.kind, DrawKind::InstancedGlDrawFlag { flag: 1 }); // +0x40 = 0x1_0000_0003
        assert_eq!(p.kind.discriminant(), 3);
        assert_eq!(p.blend, 1.0); // +0xac = 0x3f800000
        assert_eq!(p.variant, 0); // +0xb0 = 0
        assert!(p.accesses.is_empty() && p.bindings.is_empty() && p.unknown_bytes.is_empty());
        assert_eq!(p.state, RasterState(5));
    }

    #[test]
    fn kind_discriminants_are_the_observed_values() {
        use DrawKind::*;
        let h = ManagedHandle(1);
        let ds = [
            (NoVao { vertex_count: 1 }, 0),
            (
                NoVaoIndexed {
                    index_count: 1,
                    index_buffer: h.clone(),
                },
                1,
            ),
            (NoIndexBuffer { vertex_count: 1 }, 2),
            (InstancedGlDrawFlag { flag: 1 }, 3),
            (MultiInstanced { ranges: Vec::new() }, 4),
            (
                MultiDrawIndirect {
                    buffer: 0,
                    arg_50: 0,
                    arg_58: 0,
                },
                5,
            ),
            (
                IndexedIndirect {
                    buffer: 0,
                    arg_48: 0,
                    arg_50: 0,
                    arg_58: 0,
                    first_index_count: None,
                },
                7,
            ),
            (
                IndexedIndirectCount {
                    buffer: 0,
                    count_buffer: 0,
                    args: [0; 4],
                },
                8,
            ),
        ];
        for (k, d) in ds {
            assert_eq!(k.discriminant(), d);
        }
    }

    #[test]
    fn texture_binding_key_tag_and_view_xor() {
        // 0x1408ee800：key = 0x4000.. | id；有 view 时再 xor hash。
        let k = TextureBindingKey::from_rg_texture_id(7);
        assert_eq!(k.0, 0x4000_0000_0000_0007);
        assert_eq!(k.with_view(None), k); // 0x140867940：view 为空原样返回
        let v = TextureViewDesc::default();
        let expected = k.0 ^ crate::render::utils::calculate_hash(&v);
        assert_eq!(k.with_view(Some(&v)).0, expected);
    }

    #[test]
    #[should_panic(expected = "Raw textures must not be used as rg resources")]
    fn raw_texture_key_conversion_panics() {
        let t = Texture::default();
        let _ = (&t).to_texture_binding_key();
    }

    #[test]
    fn rg_handle_keys_use_the_tagged_id() {
        let h: Handle<Texture> = Handle(ManagedHandle(9), core::marker::PhantomData);
        assert_eq!((&h).to_texture_binding_key().0, 0x4000_0000_0000_0009);
    }

    #[test]
    #[should_panic(expected = "RasterPass dropped without a call to render()")]
    fn render_check_panics_when_not_rendered() {
        let _c = DrawPassRenderCheck(false);
    }

    #[test]
    fn render_check_silent_when_rendered() {
        let _c = DrawPassRenderCheck(true);
    }

    #[test]
    fn access_is_write_range_matches_0x140c10e10() {
        assert!(
            !access_is_write(0x27)
                && access_is_write(0x28)
                && access_is_write(0x37)
                && !access_is_write(0x38)
        );
        assert!(
            access_is_write(0x2f)
                && access_is_write(0x36)
                && !access_is_write(0x17)
                && !access_is_write(0x1d)
        );
    }

    #[test]
    fn raster_state_bit_edits_match_raster_single() {
        // 0x1409ce42e: and al, 0x7f ; 0x1409ce430/33: dl = (dl & 0x9f) | 0x40
        let s = RasterState(0x1f80_00ff);
        assert_eq!(s.clear_byte0_bit7().0 & 0xff, 0x7f);
        assert_eq!(s.set_byte3_field_2().0 >> 24, (0x1f & 0x9f) | 0x40);
    }

    #[test]
    fn new_registers_color_and_depth_accesses() {
        let (host, _log) = MockHost::new();
        let colors = [
            ColorAttachment::Texture(tex(1, 10)),
            ColorAttachment::View {
                view: [0; 4],
                texture: tex(2, 11),
            },
        ];
        let depth = DepthAttachment {
            writable: true,
            layer: 2,
            texture: tex(3, 12),
            clear: DepthClear::Clear(0.0),
        };
        let p = RasterPass::new("p", &colors, Some(&depth), host.boxed());
        assert_eq!(
            p.accesses,
            alloc::vec![
                PassAccess {
                    resource: 1,
                    access: 0x36,
                    group: 1
                },
                PassAccess {
                    resource: 2,
                    access: 0x2e,
                    group: 1
                },
                PassAccess {
                    resource: 3,
                    access: 0x2f,
                    group: 1
                },
            ]
        );
        let ro = DepthAttachment {
            writable: false,
            ..depth
        };
        let (host, _l) = MockHost::new();
        let p2 = RasterPass::new("p", &[], Some(&ro), host.boxed());
        assert_eq!(p2.accesses[0].access, 0x17);
        let mut p = p;
        p.rendered.0 = true; // 避免 drop 检查
        let mut p2 = p2;
        p2.rendered.0 = true;
    }

    #[test]
    #[should_panic(expected = "capacity exceeded")]
    fn more_than_five_color_attachments_panic() {
        let (host, _l) = MockHost::new();
        let c = ColorAttachment::Texture(tex(1, 1));
        let six = [c; 6];
        let _ = RasterPass::new("p", &six, None, host.boxed());
    }

    #[test]
    fn render_dedupes_render_passes_via_the_device_cache() {
        let (host, log) = MockHost::new();
        let mut h = host;
        h.cache.clear();
        // 同一描述渲染两次：create_render_pass 只会被调用一次（rustc_entry 命中分支克隆句柄）。
        let colors = [ColorAttachment::Texture(tex(1, 10))];
        let mut pass1 = RasterPass::new("a", &colors, None, h.boxed());
        let desc = pass1.build_render_pass_desc();
        assert_eq!(desc.color_count, 1);
        assert_eq!(desc.depth_state, DEPTH_STATE_NONE);
        assert_eq!(desc.colors[0].format, 10);
        assert_eq!(desc.colors[0].sample_count, 4);
        assert_eq!(desc.colors[0].kind_tag, 0);
        // 预置缓存，模拟第二次 render 命中。
        let pre = ManagedHandle(77);
        pass1.host.render_pass_cache().insert(desc, pre.clone());
        pass1.render();
        let l = log.borrow();
        assert!(l.created_passes.is_empty());
        assert_eq!(l.submissions.len(), 1);
        assert_eq!(l.submissions[0].render_pass, pre);
    }

    #[test]
    fn render_creates_and_caches_on_miss_and_flags_rendered() {
        let (host, log) = MockHost::new();
        let depth = DepthAttachment {
            writable: false,
            layer: 0,
            texture: tex(5, 20),
            clear: DepthClear::Load,
        };
        let pass = RasterPass::new("d", &[], Some(&depth), host.boxed());
        let d = pass.build_render_pass_desc();
        assert_eq!(d.depth_state, 0); // 只读
        assert!(!d.depth.clear_input);
        pass.render();
        assert_eq!(log.borrow().created_passes.len(), 1);
        // 清除值 → clear_input
        let (host, _l) = MockHost::new();
        let dc = DepthAttachment {
            writable: true,
            clear: DepthClear::Clear(1.0),
            ..depth
        };
        let pass = RasterPass::new("d", &[], Some(&dc), host.boxed());
        let d = pass.build_render_pass_desc();
        assert!(d.depth.clear_input);
        assert_eq!(d.depth_state, 1);
        pass.render();
    }

    #[test]
    #[should_panic(expected = "RasterPass dropped without a call to render()")]
    fn dropping_an_unrendered_pass_panics() {
        let (host, _l) = MockHost::new();
        let _p = RasterPass::new("x", &[], None, host.boxed());
    }

    #[test]
    fn invalid_color_format_stops_collection() {
        // O：take_while（`+0xc == 0` 即停止）。
        let (host, _l) = MockHost::new();
        let colors = [
            ColorAttachment::Texture(tex(1, 10)),
            ColorAttachment::Texture(tex(2, 0)),
            ColorAttachment::Texture(tex(3, 11)),
        ];
        let pass = RasterPass::new("x", &colors, None, host.boxed());
        let d = pass.build_render_pass_desc();
        assert_eq!(d.color_count, 1);
        pass.render();
    }

    // ---------------------------------------------------------------- B 阶段（i_api）

    #[test]
    fn tag2_color_registers_view_access_and_overrides_the_description_tag() {
        use crate::render::pass_builder::framebuffer_binding::{
            AttachmentTexture, ColorAttachment,
        };
        let (host, _l) = MockHost::new();
        let t = AttachmentTexture {
            id: 7,
            extent: (4, 4),
            format: 37,
            sample_count: 1,
        };
        let pass = RasterPass::new(
            "g",
            &[ColorAttachment::Tag2(t), ColorAttachment::Texture(t)],
            None,
            host.boxed(),
        );
        assert_eq!(
            pass.accesses[0],
            PassAccess {
                resource: 7,
                access: 0x2e,
                group: 1
            }
        );
        assert_eq!(pass.accesses[1].access, 0x36);
        assert_eq!(pass.colors[0].tag, 2);
        let d = pass.build_render_pass_desc();
        assert_eq!(
            (d.colors[0].kind_tag, d.colors[1].kind_tag, d.color_count),
            (2, 0, 2)
        );
        let mut pass = pass;
        pass.rendered.0 = true;
    }

    #[test]
    fn host_provides_rg_resources_before_a_pass_exists() {
        use crate::render::pass_builder::rg_resources::{RecordingRg, RgKind};
        use crate::rhapsody_types::TextureDesc;
        let (mut host, _l) = MockHost::new();
        host.rg = Some(RecordingRg::default());
        let desc = TextureDesc::new_2d([960, 540], 0x4c).sampled().storage();
        let tex = require_rg_resources(&mut host).create_raw_texture("mei temp", &desc);
        assert_eq!((tex.resource, tex.desc), (0, desc));
        let mut pass = RasterPass::new("p", &[], None, host.boxed());
        // 建 pass 后仍可经 `host_mut` 创建（`msaa color out` 一类）。
        let buf = require_rg_resources(pass.host_mut()).create_raw_buffer(
            "b",
            &crate::render::ssbo::rhapsody_boundary::BufferDesc::new_gpu_only(16, 0x20),
        );
        assert_eq!(buf.resource, 1);
        pass.rendered.0 = true;
        let _ = RgKind::Texture(desc);
    }

    #[test]
    fn host_without_a_render_graph_reports_it() {
        let (mut host, _l) = MockHost::new();
        assert!(host.rg_resources().is_none());
        assert_eq!(host.ssbo_instance_count(1), None);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = require_rg_resources(&mut host);
        }));
        assert!(r.is_err());
    }
}
