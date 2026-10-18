//! draw_builder.rs — DrawBuilder（按值消费的 draw 声明器）。
//!
//! O = 机器码观测，I = 推断，U = 未知（不编造）。地址为 B 版 EXE。逐过程证据见 `B_a4_draw.md`。
//!
//! # DrawBuilder 布局（O，0xf8 B）
//! `+0x00` `NoDropOnlyTake<DrawPacket>`（`DrawPacket` 的第一个 u64 兼作 niche：值 2 = 已取走）；
//! `+0xb8..+0xe0` `Option<DynamicConstantsBinding>`（niche 字节 `+0xde == 8` = 无）；
//! `+0xe0` `&mut RasterPass`（旧版把它建模成 "GameContext"，机器码里它就是 `RasterPass`：
//! `+0x00` draws Vec、`+0x18` bindings Vec、`+0x60` 名字→槽表、`+0x90` PassBuilder、
//! `+0x1a8` PassResources…）；`+0xe8` `Arc<CompiledShaderPipeline>`；`+0xf0` `bind_common` 写入的布尔。
//!
//! # 已核对（地址 → Rust 项）
//! - 0x140905be0 `DrawBuilder::new` → [`DrawBuilder::new`]
//! - 0x14089a140 `impl Drop::drop`（经 0x1408935c0 drop_in_place）→ `impl Drop for DrawBuilder`
//! - 0x140906150 / 0x1409067e0 `read_rg_storage_buffer` / `write_rg_storage_buffer`（仅访问类型 0x24 / 0x37、
//!   同步组 1 / 0 不同）→ [`DrawBuilder::read_rg_storage_buffer`] / [`DrawBuilder::write_rg_storage_buffer`]
//! - 0x140906400 `bind_external_storage_buffer`；0x140907280 `draw_instanced_from_placement_data`
//! - 0x140906a90 `draw_no_vao`(种类 0)；0x140906cb0 `draw_no_vao_indexed`(1)；0x1409083f0 `draw_no_index_buffer`(2)；
//!   0x140907490 `draw_instanced_w_gl_draw_flag`(3)；0x140907050 `draw_multi_instanced`(4)；
//!   0x1409076b0 `multi_draw_indirect`(5)；0x140907990 `multi_draw_indexed_indirect`(7)；
//!   0x140907cc0 `draw_indexed_indirect_raw`(7)；0x140907f90 `draw_indexed_indirect_count`(8)；
//!   0x140906f40 `draw`；0x1409082c0 `discard`
//! - 0x141629a20/0x141629b90/0x141629d20 `optional_uniform<f32/Mat4/[u32;2]>`；0x14096b850 起 `uniform`
//! - 0x141629e90… `read_storage_buffer<T>`（43 个实例，同一泛型）
//! - 0x1403728a0/0x1408ee800/0x1408fd6c0/0x140908610/0x140908c50/0x14096f650 `bind_texture_to_draw_builder<T>`
//! - 0x14090b460 `bind_common`；0x14090bb70 `bind_wall_color_customization`；0x14090be10 `bind_brick_erase_highlight`；
//!   0x14090c300 `bind_stone_floor_textures`；0x14090c980 `bind_wooden_floor_textures`
//!
//! # 与旧版的差异
//! 旧版的 `DrawCommand{discriminant,payload[0xb0]}`、`GameContext`、`mode/draw_args`、`has_dynamic_constants` 与机器码
//! 不符，全部重写；`draw_*` 不再返回 `DrawBuilder`（机器码里它们是终结操作，无返回值）；`DrawBuilder` 增加借用
//! `RasterPass` 的生命周期参数。`BindGameResources for DrawBuilder` 不再由本文件实现（`bind_color_library`
//! 在 `bind_common` 里内联展开，见 [`DrawBuilder::bind_color_library`]）。

// Rust 1.88 enables newer style suggestions; retain original nested branches and modulo expressions.
#![allow(clippy::collapsible_if)]
// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation)]
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::render::pass_builder::bind_resources::{
    RT_BVH_BINDING_HASH, RT_BVH_UNIFORM_HASH, RT_TRIANGLE_META_BINDING_HASH,
    RT_TRIANGLES_BINDING_HASH, RT_TRIANGLES_UNIFORM_HASH,
};
use crate::render::pass_builder::draw_list::EntityDrawParams;
use crate::render::pass_builder::draw_stream::{
    DescriptorSetBinding, DrawKind, DrawPacket, DynamicConstantsBinding, GpuOwned, PassAccess,
    RasterPass, ResourceAccess, ShaderReflection, TextureBindingKey, access_is_write,
};
use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use crate::rhapsody_contract::{ManagedHandle, RenderPassBinding};
use crate::rhapsody_types::TextureViewDesc;

// ------------------------------------------------------------------------------------------
// 常量（均为机器码里的立即数）
// ------------------------------------------------------------------------------------------

/// `bind_texture_to_draw_builder` 在所有 `bind_*` 调用点使用的访问类型（O：`mov r9b, 0x1d`）。
pub const ACCESS_SAMPLED_IMAGE: u8 = 0x1d;
/// `read_storage_buffer<T>` 传给 `read_access_buffer` 的访问类型（O：`mov byte ptr [rsp+0x20], 0x1f`）。
pub const ACCESS_STORAGE_BUFFER_READ: u8 = 0x1f;
/// `read_rg_storage_buffer`（O：`0x24`，同步组 1）。
pub const ACCESS_RG_STORAGE_READ: u8 = 0x24;
/// `write_rg_storage_buffer`（O：`0x37`，同步组 0）。
pub const ACCESS_RG_STORAGE_WRITE: u8 = 0x37;
/// 间接缓冲读取（O：多个 `multi_draw_*` 里 `mov r8b, 2`，同步组 1）。
pub const ACCESS_INDIRECT_BUFFER_READ: u8 = 0x02;
/// 一个 pass 内的绑定上限（O：`cmp rdi, 0x100; jae unwrap_failed`）。
pub const MAX_PASS_BINDINGS: usize = 0x100;

/// `draw_instanced_from_placement_data` 用的 storage-buffer 名字 hash（O：`movabs r8, 0x30e6778252531a83`）。
pub const PLACEMENT_DATA_BINDING_HASH: u64 = 0x30e6_7782_5253_1a83;
/// 缺省空槽时 `expect` 的文案（O：draw_builder.rs:315:14，长度 0x45）。
pub const PLACEMENT_MISSING_MSG: &str =
    "This Handle for PlacementDataSSBO doesn't have a corresponding buffer";

/// `multi_draw_indexed_indirect` 读取的静态表（O：`DAT_142aaaa78 + idx * 0x14` 的首个 u32，
/// 表项 20 B = 一个 `DrawIndexedIndirectCommand`，这里只保留首字 `index_count`）。
/// 表长 24（O：`0x142aaaa78 + 24 * 0x14` 起是别的数据，`(1118480592, 1, 59, 0, 623)` 这类 `f32 / 字符串` 内容）；
/// 原函数无边界检查，调用点用到的最大下标是 0x17（`draw_bricks` LOD1 代理路径）。
/// 第 10、11 项在 EXE 里就是 0（O）。`draw_shaders` 的调用点用到下标 3/8/9/0xf/0x11–0x17。
pub const INDEXED_INDIRECT_INDEX_COUNTS: [u32; 24] = [
    0x1b, 0x12c, 0x60, 0x39, 0x24, 6, 0x3c, 6, 0x30, 0xae, 0, 0, 2817, 4716, 66, 180, 15, 900, 18,
    900, 18, 18, 18, 18,
];
/// 表项步长（O：`shl rax,2; lea rcx,[rax+4*rax]`）。
pub const INDEXED_INDIRECT_STRIDE: u64 = 0x14;

// bind_common 的名字 hash（O：movabs）。
const BIND_COMMON_FLAG_UNIFORM: u64 = 0x418a_9844_1f59_a54c;
const BIND_COMMON_PRESET_NAME: u64 = 0x1165_1f4d_c984_1aa4;
const BIND_COMMON_MODEL_MATRIX: u64 = 0x1465_8de1_04be_f99d;
const BIND_COMMON_FLAG_INT_UNIFORM: u64 = 0xc5ab673fb08651f5; // -0x3a5498c04f79ae0b
const BIND_COMMON_TIME: u64 = 0x1e54_85ef_2e9f_9384;
const BIND_COMMON_VIEWPORT: u64 = 0x531b_b931_8eea_4971;
/// `bind_common` 里 flag 为真/假时绑定的纹理预设（O：`0x12` / `0x11`）。
pub const BIND_COMMON_PRESET_ON: u8 = 0x12;
pub const BIND_COMMON_PRESET_OFF: u8 = 0x11;
/// `bind_color_library` 用的 binding 名 hash 与槽名（O：movabs / 字符串 "ColorLibrary"）。
pub const COLOR_LIBRARY_BINDING_HASH: u64 = 0x52ec_6a17_beb8_fef0;
pub const COLOR_LIBRARY_SLOT_NAME: &str = "ColorLibrary";
/// `bind_wall_color_customization` 末尾的 `read_storage_buffer<WallColorIdSsbo>`。
/// 槽名（O：`read_storage_buffer<WallColorIdSsbo>` 0x14162c290 的 `rustc_entry` 键，`mov r9d, 0xc`）= "WallColorIds"
/// （复数；旧版写成 "WallColorId"，`xxh3_64("WallColorIds") == SsboId 0xaf42315d263aa22a` 已核对）。
pub const WALL_COLOR_ID_SLOT_NAME: &str = "WallColorIds";
pub const WALL_COLOR_ID_BINDING_HASH: u64 = 0xe9701a257aa8828c; // -0x168fe5da85577d74

/// `bind_wall_color_customization`：四张 `TextureFromDisk`（文件键 hash, binding 名 hash），访问均 0x1d（O；顺序即调用顺序）。
pub const WALL_COLOR_TEXTURES: [(u64, u64); 4] = [
    (0x4b15227408664b67, 0x58d5652d3b800ae7),
    (0x3db5defdbd000787, 0x5a94ece060c4e795),
    (0x7984363975076e17, 0x549dae761b7907e7),
    (0x7f13c75f8edf1f67, 0xbbd21ec1fbd5f851),
];

/// `bind_brick_erase_highlight` 的三个 uniform 名字 hash（O：0x14090be10 的 `r8` 立即数）。
pub const BRICK_ERASE_ENABLED_UNIFORM: u64 = 0x203fd61c57064e54;
pub const BRICK_ERASE_CURSOR_UNIFORM: u64 = 0x06bdce4199980bf7;
/// `-0x4a7a31de4938d73c`。
pub const BRICK_ERASE_RADIUS_UNIFORM: u64 = 0xb585ce21b6c728c4;

/// `bind_brick_erase_highlight` 从 World 读出的四个资源值（见该函数文档；A/C/D 的资源类型 U）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BrickEraseHighlightInputs {
    /// 资源 A（TypeId `0x6efcda1c77e106b4/0x7c999c10e7fa637f`）的字节 `+0xc`。
    pub enabled: bool,
    /// 资源 B（`0x062dfe8f62b6fcc5/-0x09aff71ab79cd459` = `CursorTerrainRaycast`）：`Some(pos)` 当标签 bit0。
    pub cursor_position: Option<glam::Vec3>,
    /// 资源 C（`0x0ae77a86a2c6cefc/-0x0aaf65a94cd17e30`）的首个 f32。
    pub radius: f32,
    /// 资源 D（`0x48b321ef779dc301/-0x429a898e6b262238`）的字节 `+0xfe`。
    pub suppress: bool,
}

/// 旧版 `DrawCommand`（0xb8 B draw 记录）就是 `DrawPacket`（`RasterPass.draws` 的元素）；
/// 保留别名以免 `lib.rs::consume::pass_builder` 的 re-export 失效。
pub type DrawCommand = DrawPacket;

// ------------------------------------------------------------------------------------------
// NoDropOnlyTake
// ------------------------------------------------------------------------------------------

/// `NoDropOnlyTake<T>`：只能被取走、不能被 drop 的 `Option<T>`（原 `unwrap_failed` 位置 draw_builder.rs:770:23）。
#[derive(Debug)]
pub struct NoDropOnlyTake<T>(Option<T>);

impl<T> NoDropOnlyTake<T> {
    pub fn new(value: T) -> Self {
        NoDropOnlyTake(Some(value))
    }
    /// `take`：已取走再取 → `Option::unwrap` panic。
    pub fn take(&mut self) -> T {
        self.0
            .take()
            .expect("called `Option::unwrap()` on a `None` value")
    }
    /// `try_take`：已取走 → `None`。
    pub fn try_take(&mut self) -> Option<T> {
        self.0.take()
    }
    pub fn is_taken(&self) -> bool {
        self.0.is_none()
    }
    pub fn get(&self) -> &T {
        self.0
            .as_ref()
            .expect("called `Option::unwrap()` on a `None` value")
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.0
            .as_mut()
            .expect("called `Option::unwrap()` on a `None` value")
    }
}

// ------------------------------------------------------------------------------------------
// uniform 值
// ------------------------------------------------------------------------------------------

/// 写入动态常量块的值（按小端写出，大小 = 原函数 `movups`/`mov` 的字节数）。
pub trait UniformValue {
    fn write_bytes(&self, out: &mut Vec<u8>);
}

impl UniformValue for f32 {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}
impl UniformValue for u32 {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}
impl UniformValue for i32 {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}
/// O 0x14096b850：`bool` 以 `u32` 写入（`movzx eax, r9b; mov dword ptr [..], eax`）。
impl UniformValue for bool {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(*self as u32).to_le_bytes());
    }
}
impl UniformValue for [u32; 2] {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        for v in self {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
}
impl UniformValue for glam::Mat4 {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        for v in self.to_cols_array() {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
}

/// O 0x14090be10：`write_uniform<Vec3>` 写 12 字节（`{u64, u32}` 拷贝）。
impl UniformValue for glam::Vec3 {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        for v in self.to_array() {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
}

/// `bind_common` 读取的变换（O：`+0x20` 旋转四元数、`+0x30` 平移、`+0x3c` 缩放，与 bevy `Transform`
/// 的重排后内存布局一致；`compute_matrix` 内联为 `Mat4::from_scale_rotation_translation`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawTransform {
    pub rotation: glam::Quat,
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
}

impl DrawTransform {
    pub fn compute_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}

/// `CommonDrawParams +4` / `DrawBuilder +0xf0` 的小枚举（B 阶段 i_api，替代旧 `bool`）。
///
/// O（0..=4）：`bind_common` 以 `movzx r14d, byte ptr [params+4]` 读整个字节写进 `+0xf0`，`write_uniform<u32>` 写该字节的 u32；
/// `shadow_pass` 传 `cascade + 2`（cascade ∈ 0..3 → 2..=4）；`ui_pass`/`wireframe_pass` 传 1（`mov byte [..], 1`）；
/// 其它 pass 传 0/1；`indirect_draw_bricks` 读 `+0xf0` 做 `cmp byte [rcx+0xf0], 2`、`0x1c >> v`、`0x201000000 >> 8v`。
/// 变体名 I（数值 O）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CommonDrawFlag {
    /// 0（`bind_common` 的 bool uniform = true，预设 0x11）。
    #[default]
    Off = 0,
    /// 1。
    On = 1,
    /// 2 = 阴影级联 0（`shadow_pass`：`cascade + 2`）。
    ShadowCascade0 = 2,
    /// 3 = 阴影级联 1。
    ShadowCascade1 = 3,
    /// 4 = 阴影级联 2。
    ShadowCascade2 = 4,
}

impl CommonDrawFlag {
    /// 字节 → 枚举；机器码里只观测到 0..=4，其余 → `None`。
    pub const fn from_byte(b: u8) -> Option<CommonDrawFlag> {
        Some(match b {
            0 => CommonDrawFlag::Off,
            1 => CommonDrawFlag::On,
            2 => CommonDrawFlag::ShadowCascade0,
            3 => CommonDrawFlag::ShadowCascade1,
            4 => CommonDrawFlag::ShadowCascade2,
            _ => return None,
        })
    }
    /// 阴影级联 `cascade`（0..3）的标志字节 `cascade + 2`（O：`add r14b, 2`）。
    pub const fn shadow_cascade(cascade: u8) -> Option<CommonDrawFlag> {
        CommonDrawFlag::from_byte(cascade.wrapping_add(2))
    }
    /// 原版的 `test r14d, r14d`：非 0。
    pub const fn is_set(self) -> bool {
        self as u8 != 0
    }
}

impl From<bool> for CommonDrawFlag {
    fn from(b: bool) -> Self {
        if b {
            CommonDrawFlag::On
        } else {
            CommonDrawFlag::Off
        }
    }
}

/// `bind_common` 读取的 `CommonDrawParams` 字段：`+0` f32 时间、`+4` 标志字节（其余字段 U）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CommonDrawParams {
    pub time: f32,
    pub flag: CommonDrawFlag,
}

impl EntityDrawParams {
    /// 回调的 `&EntityDrawParams` 的 `+8` 就是实体（O：`draw_bricks`/`stairs_draw` 的 `[rdx+8]`）。
    /// 结构体定义在 `draw_list.rs`（A4，另有 agent 统一 `DrawItem`）；访问器放在这里以免与它的改写冲突。
    pub fn entity(&self) -> bevy_ecs::entity::Entity {
        self.entity
    }
}

// ------------------------------------------------------------------------------------------
// bind_texture_to_draw_builder 的输入
// ------------------------------------------------------------------------------------------

/// `bind_texture_to_draw_builder<T>` 的六个泛型实例按函数体归并成四类（O：见各实例地址）。
#[derive(Clone, Copy, Debug)]
pub enum BindTexture<'a> {
    /// `&Handle<Texture>`（0x1408ee800）与 `&ReadOnlyHandle<Texture>`（0x14096f650）：同一函数体；写访问 panic。
    RgHandle(&'a AttachmentTexture),
    /// `TexturePreset`（0x140908c50）：`AssetTextureLibrary::get_rg_texture`，允许写访问。
    Preset(TexturePreset),
    /// `TextureFromDisk`（0x140908610）：`get_by_name` + 原生描述符绑定。
    FromDisk(TextureFromDisk),
    /// `&Texture`（0x1403728a0 / 0x1408fd6c0）：原生纹理；断言 `view.is_none()`。
    Raw {
        texture: &'a ManagedHandle,
        desc: &'a AttachmentTexture,
    },
}

// ------------------------------------------------------------------------------------------
// DrawBuilder
// ------------------------------------------------------------------------------------------

/// DrawBuilder（0xf8 B）。
pub struct DrawBuilder<'p> {
    packet: NoDropOnlyTake<DrawPacket>,
    constants: Option<DynamicConstantsBinding>,
    pass: &'p mut RasterPass,
    shader: Arc<ShaderReflection>,
    /// `+0xf0`：`bind_common` 写入的标志字节（0..=4；`DrawBuilder::new` 置 0）。
    common_flag: CommonDrawFlag,
}

impl<'p> DrawBuilder<'p> {
    /// `new`（0x140905be0）。
    ///
    /// O 顺序：`packet.variant = variant`（`+0xb0`）；`FrozenMap::get(shader)`（缺失 → `Option::unwrap` panic，
    /// draw_builder.rs:85:14）取 `variants[variant].compiled`（Arc 克隆）；资源/锁取得失败 → draw_builder.rs:91:67 的
    /// `Option::unwrap` panic（由宿主承担）；仅当反射有 uniform 块（`+0xe8 != 0`）时才
    /// `DynamicConstants::allocate`，并把产出的 uniform 绑定 `insert` 进 `packet.bindings`（被替换的旧值照常 drop）；
    /// 否则常量槽为 `None`（`+0xde = 8`）。
    pub fn new(pass: &'p mut RasterPass, mut packet: DrawPacket, variant: u8) -> DrawBuilder<'p> {
        packet.variant = variant;
        let shader = pass
            .host
            .raster_shader(&packet.shader, variant)
            .expect("called `Option::unwrap()` on a `None` value");
        let mut constants = None;
        if shader.has_uniform_block {
            if let Some(c) = pass.host.allocate_dynamic_constants(&shader) {
                packet.bindings.insert(
                    c.binding_index,
                    GpuOwned::new(DescriptorSetBinding::UniformBuffer {
                        size: shader.uniform_block_size,
                        keepalive: c.keepalive.clone(),
                        lease: c.lease.clone(),
                    }),
                );
                constants = Some(c);
            }
        }
        DrawBuilder {
            packet: NoDropOnlyTake::new(packet),
            constants,
            pass,
            shader,
            common_flag: CommonDrawFlag::Off,
        }
    }

    /// 着色器变体下标（O：`nani_draw` 读 `packet.+0xb0`）。
    pub fn variant(&self) -> u8 {
        self.packet.get().variant
    }

    /// 只读访问包（测试/调用方观察；原版无此函数）。
    pub fn packet(&self) -> &DrawPacket {
        self.packet.get()
    }
    /// 可变访问包（closure 里改光栅状态，如 `raster_single_object` 的 mei_pass 实例）。
    pub fn packet_mut(&mut self) -> &mut DrawPacket {
        self.packet.get_mut()
    }
    /// `+0xf0`（0..=4 的小枚举；`as u8` 即原字节）。
    pub fn common_flag(&self) -> CommonDrawFlag {
        self.common_flag
    }
    /// 动态常量块当前内容（观察用）。
    pub fn constants_data(&self) -> Option<&[u8]> {
        self.constants.as_ref().map(|c| c.data.as_slice())
    }

    // ---------------------------------------------------------------- 内部辅助

    /// 在 `shader.bindings`（名字表）里查名字 hash；表空时跳过（O：`cmp [refl+0xb0], 0; je`）。
    /// 命中且 `set != 0` → `assert_failed<SetIndex>`（左 0 右 set）。
    fn lookup_binding(&self, name_hash: u64) -> Option<u32> {
        if self.shader.bindings.is_empty() {
            return None;
        }
        let &(set, binding) = self.shader.bindings.get(&name_hash)?;
        assert_eq!(0u32, set, "assertion `left == right` failed");
        Some(binding)
    }

    /// 向 `RasterPass.bindings` 追加一条记录并返回其下标（O：`cmp r13, 0x100; jae unwrap_failed`）。
    fn push_pass_binding(&mut self, binding: RenderPassBinding) -> u8 {
        let idx = self.pass.bindings.len();
        let idx = u8::try_from(idx)
            .expect("called `Result::unwrap()` on an `Err` value: TryFromIntError(())");
        assert!(self.pass.bindings.len() < MAX_PASS_BINDINGS);
        self.pass.bindings.push(binding);
        idx
    }

    /// `packet.accesses.push(ResourceAccess)`（`+0x10` Vec）。
    fn push_packet_access(&mut self, binding_index: u32, slot: u8) {
        self.packet.get_mut().accesses.push(ResourceAccess {
            binding_index,
            slot,
        });
    }

    /// 动态常量写入公共路径（`optional_uniform` / `uniform` / `UniformBinder::write_uniform` 的共同行为，O）：
    /// 常量块为 `None`（`+0xde == 8`）、无 uniform 块（`refl+0xe8 == 0`）、偏移表空（`refl+0x130 == 0`）、
    /// 名字未命中 → 静默忽略；命中 → `data[offset..]`（越界 → `slice_start_index_len_fail`）写入。
    fn write_uniform_bytes(&mut self, name_hash: u64, bytes: &[u8]) {
        let Some(c) = self.constants.as_mut() else {
            return;
        };
        if !self.shader.has_uniform_block || self.shader.uniform_offsets.is_empty() {
            return;
        }
        let Some(&offset) = self.shader.uniform_offsets.get(&name_hash) else {
            return;
        };
        let tail = &mut c.data[offset as usize..];
        tail[..bytes.len()].copy_from_slice(bytes);
    }

    /// `[w, h]` 维度 uniform（`write_optional_uniform<_, [u32; 2]>`，名字 = `with_suffix_dims(name)`）。
    fn write_dims(&mut self, name_hash: u64, extent: (u32, u32)) {
        let hash = self.pass.host.with_suffix_dims(name_hash);
        let mut b = Vec::new();
        [extent.0, extent.1].write_bytes(&mut b);
        self.write_uniform_bytes(hash, &b);
    }

    // ---------------------------------------------------------------- uniform

    /// `optional_uniform<T>`（0x141629a20 / 0x141629b90 / 0x141629d20）。
    pub fn optional_uniform<T: UniformValue>(mut self, name_hash: u64, value: T) -> Self {
        let mut b = Vec::new();
        value.write_bytes(&mut b);
        self.write_uniform_bytes(name_hash, &b);
        self
    }

    /// `uniform<T>`：与 `optional_uniform` 行为相同（O 0x14096b850：`UniformBinder::write_uniform` 同样静默忽略未命中）。
    pub fn uniform<T: UniformValue>(self, name_hash: u64, value: T) -> Self {
        self.optional_uniform(name_hash, value)
    }

    // ---------------------------------------------------------------- 缓冲绑定

    fn rg_storage_access(
        mut self,
        name_hash: u64,
        buffer_resource: u32,
        access: u8,
        group: u32,
    ) -> Self {
        // 名字表空 / 未命中：原样返回（`je` 到尾部的 0xf8 B 搬运）。
        if let Some(binding) = self.lookup_binding(name_hash) {
            self.pass.accesses.push(PassAccess {
                resource: buffer_resource,
                access,
                group,
            });
            let slot = self.push_pass_binding(RenderPassBinding {
                resource_index: buffer_resource,
                access_type: access,
                view: None,
            });
            self.push_packet_access(binding, slot);
        }
        self
    }

    /// `read_rg_storage_buffer`（0x140906150）：访问 0x24、同步组 1。
    pub fn read_rg_storage_buffer(self, name_hash: u64, buffer_resource: u32) -> Self {
        self.rg_storage_access(name_hash, buffer_resource, ACCESS_RG_STORAGE_READ, 1)
    }

    /// `write_rg_storage_buffer`（0x1409067e0）：访问 0x37、同步组 0。
    pub fn write_rg_storage_buffer(self, name_hash: u64, buffer_resource: u32) -> Self {
        self.rg_storage_access(name_hash, buffer_resource, ACCESS_RG_STORAGE_WRITE, 0)
    }

    /// Current-host content intent for an original fragment atomic/RMW.
    /// It keeps the native access and real clear/upload dependency intact.
    pub fn preserve_storage_buffer_contents(mut self, hash: u64) -> Self {
        let Some(binding) = self.lookup_binding(hash) else { return self; };
        let access = self.packet.get().accesses.iter().rev()
            .find(|access| access.binding_index == binding)
            .expect("preserving raster buffer requires its actual graph binding");
        let actual = &self.pass.bindings[access.slot as usize];
        assert!(!self.pass.texture_slots.values().any(|slot| *slot == access.slot) && access_is_write(actual.access_type),
            "preserving raster buffer requires a buffer write");
        let packet = self.packet.get_mut();
        if !packet.preserving_buffer_bindings.contains(&binding) {
            packet.preserving_buffer_bindings.push(binding);
        }
        self
    }

    /// A proven uniform configuration prevents all texel fetches from this
    /// binding. Preserve the actual descriptor, owner and native sync access.
    pub fn inactive_texture_contents(mut self, hash: u64) -> Self {
        let Some(binding) = self.lookup_binding(hash) else { return self; };
        let access = self.packet.get().accesses.iter().rev()
            .find(|access| access.binding_index == binding)
            .expect("inactive raster texture requires its actual graph binding");
        let actual = &self.pass.bindings[access.slot as usize];
        assert!(self.pass.texture_slots.values().any(|slot| *slot == access.slot) && !access_is_write(actual.access_type),
            "inactive raster texture requires a non-writing image binding");
        let packet = self.packet.get_mut();
        if !packet.inactive_texture_bindings.contains(&binding) {
            packet.inactive_texture_bindings.push(binding);
        }
        self
    }

    /// `bind_external_storage_buffer`（0x140906400）：名字命中后取 `get_vk_buffer`（`None` → `Option::unwrap`
    /// panic，draw_builder.rs:189:89），把 `{buffer, 0, u64::MAX}` + 句柄克隆 `insert` 进 `packet.bindings`。
    pub fn bind_external_storage_buffer(mut self, name_hash: u64, buffer: &ManagedHandle) -> Self {
        if let Some(binding) = self.lookup_binding(name_hash) {
            let vk = self
                .pass
                .host
                .vk_buffer(buffer)
                .expect("called `Option::unwrap()` on a `None` value");
            self.packet.get_mut().bindings.insert(
                binding,
                GpuOwned::new(DescriptorSetBinding::StorageBuffer {
                    buffer: vk,
                    offset: 0,
                    range: u64::MAX,
                    keepalive: buffer.clone(),
                }),
            );
        }
        self
    }

    /// `read_storage_buffer<T>`（0x141629e90 等 43 个实例）。槽名在原版来自 `T`（"ShapeStream" 等），这里由调用方给出。
    ///
    /// O：`RasterPass +0x60` 的 `HashMap<&str, u8>` entry：空位 → `read_access_buffer::<T>`（访问 0x1f、同步组 1）
    /// 并插入返回的槽；然后按 binding 名 hash 查名字表（表空/未命中 → 结束；`set != 0` 断言），
    /// 命中则 `packet.accesses.push((binding, slot))`。
    pub fn read_storage_buffer<T: SsboType>(
        self,
        slot_name: &'static str,
        binding_name_hash: u64,
    ) -> Self {
        self.read_storage_buffer_hash(
            slot_name,
            T::rg_buffer_hash(),
            core::any::type_name::<T>(),
            binding_name_hash,
        )
    }

    fn read_storage_buffer_hash(
        mut self,
        slot_name: &'static str,
        rg_hash: u64,
        type_name: &'static str,
        binding_name_hash: u64,
    ) -> Self {
        let slot = match self.pass.buffer_slots.get(slot_name).copied() {
            Some(s) => s,
            None => {
                let resource = self
                    .pass
                    .host
                    .rg_buffer_resource_typed(rg_hash, type_name)
                    .expect("no entry found for key");
                self.pass.accesses.push(PassAccess {
                    resource,
                    access: ACCESS_STORAGE_BUFFER_READ,
                    group: 1,
                });
                let s = self.push_pass_binding(RenderPassBinding {
                    resource_index: resource,
                    access_type: ACCESS_STORAGE_BUFFER_READ,
                    view: None,
                });
                self.pass.buffer_slots.insert(slot_name, s);
                s
            }
        };
        if let Some(binding) = self.lookup_binding(binding_name_hash) {
            self.push_packet_access(binding, slot);
        }
        self
    }

    // ---------------------------------------------------------------- 纹理绑定

    /// `bind_texture_to_draw_builder<T>`（六个实例）。`access` 为 vk_sync 访问类型数值，`view` 为可选视图。
    pub fn bind_texture_to_draw_builder(
        mut self,
        tex: BindTexture<'_>,
        name_hash: u64,
        access: u8,
        view: Option<&TextureViewDesc>,
    ) -> Self {
        match tex {
            BindTexture::RgHandle(t) => {
                // O 0x1408ee800：写访问 → panic（draw_builder.rs:609:13）。
                if access_is_write(access) {
                    panic!("Trying to bind {:?} for writing, but it's not writable", t);
                }
                self.bind_rg_texture(t, name_hash, access, view);
            }
            BindTexture::Preset(p) => {
                // O 0x140908c50：`get_rg_texture(preset)`；写访问允许（同步组 0），读为 1。
                let t = self.pass.host.rg_texture(p.0);
                self.bind_rg_texture(&t, name_hash, access, view);
            }
            BindTexture::FromDisk(d) => {
                // O 0x140908610：`assert!(!access.is_write())`；原生描述符。
                assert!(
                    !access_is_write(access),
                    "assertion failed: !access.is_write()"
                );
                let (handle, desc) = self.pass.host.texture_by_name(d.0);
                self.bind_raw_texture(&handle, &desc, name_hash);
            }
            BindTexture::Raw { texture, desc } => {
                // O 0x1408fd6c0：`assert!(!access.is_write())`、`assert!(view.is_none())`。
                assert!(
                    !access_is_write(access),
                    "assertion failed: !access.is_write()"
                );
                assert!(view.is_none(), "assertion failed: view.is_none()");
                self.bind_raw_texture(texture, desc, name_hash);
            }
        }
        self
    }

    /// RG 纹理路径（0x1408ee800 / 0x14096f650 / 0x140908c50 共有）。
    fn bind_rg_texture(
        &mut self,
        t: &AttachmentTexture,
        name_hash: u64,
        access: u8,
        view: Option<&TextureViewDesc>,
    ) {
        match self.lookup_binding(name_hash) {
            None => {
                // 名字未命中：仅写维度 uniform（0x1408ee8fe → 0x1408eee55）。
            }
            Some(binding) => {
                let key = TextureBindingKey::from_rg_texture_id(t.id).with_view(view);
                let slot = match self.pass.texture_slots.get(&key).copied() {
                    Some(s) => s,
                    None => {
                        // 空位：登记资源访问（写 → 同步组 0，读 → 1）并追加 bind_view 绑定。
                        let group = if access_is_write(access) { 0 } else { 1 };
                        self.pass.accesses.push(PassAccess {
                            resource: t.id,
                            access,
                            group,
                        });
                        let s = self.push_pass_binding(RenderPassBinding {
                            resource_index: t.id,
                            access_type: access,
                            view: view.copied(),
                        });
                        self.pass.texture_slots.insert(key, s);
                        s
                    }
                };
                self.push_packet_access(binding, slot);
            }
        }
        self.write_dims(name_hash, t.extent);
    }

    /// 原生纹理路径（0x1408fd6c0 / 0x140908610）：`get_vk_image_view`（`Err` → `Result::unwrap` panic）后
    /// 把 `{sampler 0, view, 5}` + 句柄克隆 `insert` 进 `packet.bindings`，再写维度 uniform。
    fn bind_raw_texture(
        &mut self,
        handle: &ManagedHandle,
        desc: &AttachmentTexture,
        name_hash: u64,
    ) {
        if let Some(binding) = self.lookup_binding(name_hash) {
            let view = self
                .pass
                .host
                .vk_image_view(handle)
                .expect("called `Result::unwrap()` on an `Err` value");
            self.packet.get_mut().bindings.insert(
                binding,
                GpuOwned::new(DescriptorSetBinding::SampledImage {
                    image_view: view,
                    layout_word: 5,
                    keepalive: handle.clone(),
                }),
            );
        }
        self.write_dims(name_hash, desc.extent);
    }

    // ---------------------------------------------------------------- 终结操作

    /// 公共尾部：覆盖 draw 种类（旧载荷自动 drop，对应机器码里按 tag 4/1 释放 Vec/句柄），
    /// 取走 packet 压入 `RasterPass.draws`（`self` 随后 drop，`Drop` 检查看到已取走）。
    fn finish(mut self, kind: DrawKind) {
        self.packet.get_mut().kind = kind;
        self.flush_constants();
        let packet = self.packet.take();
        self.pass.draws.push(packet);
    }

    /// 把动态常量字节交回宿主（见 [`PassHost::flush_dynamic_constants`]：原版是映射切片，写入即生效）。
    fn flush_constants(&mut self) {
        if let Some(constants) = self.constants.as_ref() {
            self.pass.host.flush_dynamic_constants(constants);
        }
    }

    /// `draw`（0x140906f40）：不改 draw 种类，直接提交。
    pub fn draw(mut self) {
        self.flush_constants();
        let packet = self.packet.take();
        self.pass.draws.push(packet);
    }

    /// `draw_no_vao`（0x140906a90）：种类 0。
    pub fn draw_no_vao(self, vertex_count: u32) {
        self.finish(DrawKind::NoVao { vertex_count });
    }

    /// `draw_no_vao_indexed`（0x140906cb0）：种类 1，索引缓冲句柄克隆。
    pub fn draw_no_vao_indexed(self, index_buffer: &ManagedHandle, index_count: u32) {
        self.finish(DrawKind::NoVaoIndexed {
            index_count,
            index_buffer: index_buffer.clone(),
        });
    }

    /// `draw_no_index_buffer`（0x1409083f0）：种类 2。
    pub fn draw_no_index_buffer(self, vertex_count: u32) {
        self.finish(DrawKind::NoIndexBuffer { vertex_count });
    }

    /// `draw_instanced_w_gl_draw_flag`（0x140907490）：种类 3。
    pub fn draw_instanced_w_gl_draw_flag(self, flag: u32) {
        self.finish(DrawKind::InstancedGlDrawFlag { flag });
    }

    /// `draw_multi_instanced`（0x140907050）：种类 4，接管 `Vec<[u32; 2]>`。
    pub fn draw_multi_instanced(self, ranges: Vec<[u32; 2]>) {
        self.finish(DrawKind::MultiInstanced { ranges });
    }

    /// 取 SSBO 间接缓冲的 rg 资源并登记访问（`get_rg_buffer<DrawIndirectBuffer/DrawIndexedIndirectBuffer>`
    /// + `add_resource_access(res, 2, 1)`）。
    fn indirect_resource<D: SsboType>(&mut self) -> u32 {
        let resource = self
            .pass
            .host
            .rg_buffer_resource_typed(D::rg_buffer_hash(), core::any::type_name::<D>())
            .expect("no entry found for key");
        self.pass.accesses.push(PassAccess {
            resource,
            access: ACCESS_INDIRECT_BUFFER_READ,
            group: 1,
        });
        resource
    }

    /// `multi_draw_indirect`（0x1409076b0）：种类 5。`D` = `DrawIndirectBuffer` 的 SSBO 类型。
    pub fn multi_draw_indirect<D: SsboType>(mut self, arg_50: u64, arg_58: u64) {
        let buffer = self.indirect_resource::<D>();
        self.finish(DrawKind::MultiDrawIndirect {
            buffer,
            arg_50,
            arg_58,
        });
    }

    /// `multi_draw_indexed_indirect`（0x140907990）：种类 7；`+0x48 = idx * 0x14`；`+0x60 = Some`、
    /// `+0x64 = INDEXED_INDIRECT_INDEX_COUNTS[idx]`（越界 → panic，原版无检查）。
    pub fn multi_draw_indexed_indirect<D: SsboType>(mut self, idx: u32, arg_50: u64, arg_58: u64) {
        let buffer = self.indirect_resource::<D>();
        let first_index_count = INDEXED_INDIRECT_INDEX_COUNTS[idx as usize];
        self.finish(DrawKind::IndexedIndirect {
            buffer,
            arg_48: idx as u64 * INDEXED_INDIRECT_STRIDE,
            arg_50,
            arg_58,
            first_index_count: Some(first_index_count),
        });
    }

    /// `draw_indexed_indirect_raw`（0x140907cc0）：种类 7，`+0x60 = None`；缓冲资源取自传入句柄（`+0x10`）。
    /// 参数顺序（O）：`+0x48 = arg_48`（r8）、`+0x50 = arg_50`（第 5 个参数）、`+0x58 = arg_58`（r9）。
    pub fn draw_indexed_indirect_raw(
        self,
        buffer_resource: u32,
        arg_48: u64,
        arg_58: u64,
        arg_50: u64,
    ) {
        self.pass.accesses.push(PassAccess {
            resource: buffer_resource,
            access: ACCESS_INDIRECT_BUFFER_READ,
            group: 1,
        });
        self.finish(DrawKind::IndexedIndirect {
            buffer: buffer_resource,
            arg_48,
            arg_50,
            arg_58,
            first_index_count: None,
        });
    }

    /// `draw_indexed_indirect_count`（0x140907f90）：种类 8；两个缓冲各登记一次访问（先 draw 后 count）。
    pub fn draw_indexed_indirect_count(
        self,
        buffer_resource: u32,
        count_resource: u32,
        args: [u64; 4],
    ) {
        self.pass.accesses.push(PassAccess {
            resource: buffer_resource,
            access: ACCESS_INDIRECT_BUFFER_READ,
            group: 1,
        });
        self.pass.accesses.push(PassAccess {
            resource: count_resource,
            access: ACCESS_INDIRECT_BUFFER_READ,
            group: 1,
        });
        self.finish(DrawKind::IndexedIndirectCount {
            buffer: buffer_resource,
            count_buffer: count_resource,
            args,
        });
    }

    /// `draw_instanced_from_placement_data`（0x140907280）：按 `GlobalId` 查 placement 表（缺失 →
    /// `expect(PLACEMENT_MISSING_MSG)`），克隆句柄后 `bind_external_storage_buffer(0x30e6…, &handle)`，
    /// 再 `draw_instanced_w_gl_draw_flag(flag)`。
    pub fn draw_instanced_from_placement_data(self, id: u64) {
        let (handle, flag) = self
            .pass
            .host
            .placement_buffer(id)
            .expect(PLACEMENT_MISSING_MSG);
        let b = self.bind_external_storage_buffer(PLACEMENT_DATA_BINDING_HASH, &handle);
        b.draw_instanced_w_gl_draw_flag(flag);
    }

    /// `discard`（0x1409082c0）：取走 packet（已取走 → `Option::unwrap` panic），把 `bindings` 里每个桶的状态字节
    /// 置 2（解除 GPU 持有），丢弃 packet 与 builder。
    pub fn discard(mut self) {
        let mut packet = self.packet.take();
        for b in packet.bindings.values_mut() {
            b.state = GpuOwned::STATE_DISCARDED;
        }
        drop(packet);
    }

    // ---------------------------------------------------------------- bind_* 组（0x14090b460 起）

    /// `bind_color_library`（`bind_resources::impl$0::bind_color_library<DrawBuilder>` 0x1416286f0，内联进 `bind_common`）：
    /// 槽名 "ColorLibrary"，binding 名 hash 0x52ec6a17beb8fef0。`C` = `ColorLibrarySsbo`。
    pub fn bind_color_library<C: SsboType>(self) -> Self {
        self.read_storage_buffer_hash(
            COLOR_LIBRARY_SLOT_NAME,
            C::rg_buffer_hash(),
            core::any::type_name::<C>(),
            COLOR_LIBRARY_BINDING_HASH,
        )
    }

    /// `bind_common`（0x14090b460）。O 顺序：
    /// 1. `flag = movzx byte [params+4]` 写入 `self.+0xf0`；
    /// 2. 常量块存在 → `write_uniform<bool>(0x418a98441f59a54c, flag == 0)`
    ///    （O 0x14090b4ea：`test r14d, r14d; je` → 非 0 路径 `xor r9d, r9d`（false），0 路径 `mov r9b, 1`（true）；
    ///    旧版把它写成 `flag`，与机器码相反）；
    /// 3. 绑定纹理预设（flag != 0 ? 0x12 : 0x11）到名字 0x11651f4dc9841aa4，访问 0x1d，无 view；
    /// 4. `optional_uniform<Mat4>(0x14658de104bef99d, transform.compute_matrix())`；
    /// 5. 常量块存在 → `write_uniform<u32>(0xc5ab673fb08651f5, flag 字节 as u32)`；
    /// 6. `optional_uniform<f32>(0x1e5485ef2e9f9384, params.time)`、
    ///    `optional_uniform<[u32;2]>(0x531bb9318eea4971, viewport)`（视口取自 `RasterPass +0x1f8`）；
    /// 7. `bind_color_library`。
    pub fn bind_common<C: SsboType>(
        mut self,
        params: &CommonDrawParams,
        transform: &DrawTransform,
    ) -> Self {
        self.common_flag = params.flag;
        if self.constants.is_some() {
            self = self.uniform(BIND_COMMON_FLAG_UNIFORM, !params.flag.is_set());
        }
        let preset = if params.flag.is_set() {
            BIND_COMMON_PRESET_ON
        } else {
            BIND_COMMON_PRESET_OFF
        };
        self = self.bind_texture_to_draw_builder(
            BindTexture::Preset(TexturePreset(preset)),
            BIND_COMMON_PRESET_NAME,
            ACCESS_SAMPLED_IMAGE,
            None,
        );
        self = self.optional_uniform(BIND_COMMON_MODEL_MATRIX, transform.compute_matrix());
        if self.constants.is_some() {
            self = self.uniform(BIND_COMMON_FLAG_INT_UNIFORM, params.flag as u8 as u32);
        }
        let viewport = self.pass.viewport;
        self = self.optional_uniform(BIND_COMMON_TIME, params.time);
        self = self.optional_uniform(BIND_COMMON_VIEWPORT, viewport);
        self.bind_color_library::<C>()
    }

    /// 依次绑定 `TextureFromDisk`（访问 0x1d、无 view）：`bind_wall_color_customization` /
    /// `bind_stone_floor_textures` / `bind_wooden_floor_textures` 的共同形态。
    fn bind_disk_textures(mut self, table: &[(u64, u64)]) -> Self {
        for &(file_hash, binding_hash) in table {
            self = self.bind_texture_to_draw_builder(
                BindTexture::FromDisk(TextureFromDisk(file_hash)),
                binding_hash,
                ACCESS_SAMPLED_IMAGE,
                None,
            );
        }
        self
    }

    /// `bind_brick_erase_highlight`（0x14090be10，1253 B）。
    ///
    /// 原版（`rdx` = 按值 builder、`r8` = `&World`）取四个 ECS 资源（`Components::get_resource_id` +
    /// `[world+0x210/0x218]` 稀疏表 + `[world+0x1e0]` 资源槽，缺失 → `Option::unwrap` panic，
    /// draw_builder.rs 行 `0x142aaafd8..0x142aab020` 四处），然后依次 `write_uniform`：
    /// 1. `bool`（`0x203fd61c57064e54`）= `!suppress && enabled`（`xor bl,1; and bl,r14b`）；
    /// 2. `Vec3`（`0x06bdce4199980bf7`）= `cursor.position`（`CursorTerrainRaycast` 的 `Option<Vec3>`：
    ///    标签字节 bit0 为 Some，`Vec3` 在 `+4`；None → 全 0）；
    /// 3. `f32`（`0xb585ce21b6c728c4` = `-0x4a7a31de4938d73c`）= `radius`（资源 C 的首个 f32）。
    /// 三次写入各自先看常量块是否存在（`+0xde == 8` → 跳过），行为即 [`DrawBuilder::uniform`]。
    ///
    /// A4 缺口：`PassHost` 没有 World 资源访问器，四个资源的**值**由调用方读出后经
    /// [`BrickEraseHighlightInputs`] 传入。四个资源里只有 B（`CursorTerrainRaycast`，布局吻合）能确定类型，
    /// 其余三个（A：字节 `+0xc`；C：f32 `+0`；D：字节 `+0xfe`）的类型 U。
    pub fn bind_brick_erase_highlight(self, inputs: &BrickEraseHighlightInputs) -> Self {
        let enabled_and_not_suppressed = !inputs.suppress && inputs.enabled;
        let cursor = inputs.cursor_position.unwrap_or(glam::Vec3::ZERO);
        self.uniform(BRICK_ERASE_ENABLED_UNIFORM, enabled_and_not_suppressed)
            .uniform(BRICK_ERASE_CURSOR_UNIFORM, cursor)
            .uniform(BRICK_ERASE_RADIUS_UNIFORM, inputs.radius)
    }

    /// `bind_wall_color_customization`（0x14090bb70）：4 张磁盘纹理 + `read_storage_buffer<WallColorIdSsbo>`。
    pub fn bind_wall_color_customization<W: SsboType>(self) -> Self {
        self.bind_disk_textures(&WALL_COLOR_TEXTURES)
            .read_storage_buffer::<W>(WALL_COLOR_ID_SLOT_NAME, WALL_COLOR_ID_BINDING_HASH)
    }

    /// `bind_stone_floor_textures`（0x14090c300）：12 张磁盘纹理。
    pub fn bind_stone_floor_textures(self) -> Self {
        self.bind_disk_textures(&STONE_FLOOR_TEXTURES)
    }

    /// `bind_wooden_floor_textures`（0x14090c980）：12 张磁盘纹理。
    pub fn bind_wooden_floor_textures(self) -> Self {
        self.bind_disk_textures(&WOODEN_FLOOR_TEXTURES)
    }

    /// `bind_ray_tracing<DrawBuilder>`（0x1416289d0，4167 B；`bind_resources::BindGameResources::bind_ray_tracing` 的 DrawBuilder 实例）。
    ///
    /// O 顺序（与 ComputePass 版 0x1415fc230 的差别：走槽表 + 多一次墙色读，访问码 0x1f）：
    ///  1. 从 world 取 `AssetSsboLibrary`；对 `RtTrianglesSsbo` 与 `RtBvhSsbo`：`FrozenMap::get` → 缺失 `Option::unwrap`；
    ///     `assert_eq!(state.name, type_name::<T>())`（`assert_failed<&str,&str>`）→ 取 `Buffer`，InstanceBuffer 的首个计数字段低 32 位留作 uniform 值；
    ///  2. 三个槽 `"RtTriangles"`(len 0xb) / `"RtTriangleMeta"`(0xe) / `"RtBvh"`(5)：槽表 `rustc_entry`，空位 →
    ///     `read_access_buffer<T>`（访问 0x1f、同步组 1）并插槽；再按 binding 名 `0x168ddb2c3b5f38a9` / `0x47ddebfb264383d0` /
    ///     `0x339e76846042a271` 查名字表，命中 `packet.accesses.push`——即 [`DrawBuilder::read_storage_buffer`] 的同一路径；
    ///  3. `read_storage_buffer<WallColorIdSsbo>`（槽 "WallColorIds"，binding `0xe9701a257aa8828c`）；
    ///  4. `write_uniform<u32>`：键 `0x1c9f2eef0ac739ac` ← RtTriangles 缓冲字，键 `0xc2b7f2fdf5e21521` ← RtBvh 缓冲字
    ///     （两处 uniform 值与 ComputePass 版一致；已核为 instance_count，Buffer 从 +8 开始）。
    ///
    /// 缓冲字经 [`PassHost::ssbo_instance_count`](crate::render::pass_builder::draw_stream::PassHost::ssbo_instance_count)
    /// 取得（库查询在宿主侧；宿主未提供 → `Option::unwrap` panic，与原版“库里没有”同一路径）。
    pub fn bind_ray_tracing<Tri: SsboType, Meta: SsboType, Bvh: SsboType, Wall: SsboType>(
        self,
    ) -> Self {
        let triangles_word = self
            .pass
            .host
            .ssbo_instance_count_typed(Tri::rg_buffer_hash(), core::any::type_name::<Tri>())
            .expect("called `Option::unwrap()` on a `None` value");
        let bvh_word = self
            .pass
            .host
            .ssbo_instance_count_typed(Bvh::rg_buffer_hash(), core::any::type_name::<Bvh>())
            .expect("called `Option::unwrap()` on a `None` value");
        self.read_storage_buffer::<Tri>(RT_TRIANGLES_SLOT_NAME, RT_TRIANGLES_BINDING_HASH)
            .read_storage_buffer::<Meta>(RT_TRIANGLE_META_SLOT_NAME, RT_TRIANGLE_META_BINDING_HASH)
            .read_storage_buffer::<Bvh>(RT_BVH_SLOT_NAME, RT_BVH_BINDING_HASH)
            .read_storage_buffer::<Wall>(WALL_COLOR_ID_SLOT_NAME, WALL_COLOR_ID_BINDING_HASH)
            .uniform(RT_TRIANGLES_UNIFORM_HASH, triangles_word)
            .uniform(RT_BVH_UNIFORM_HASH, bvh_word)
    }
}

/// `bind_ray_tracing<DrawBuilder>` 的槽表键（O：rdata 里的 `&str`，长度 0xb / 0xe / 5）。
pub const RT_TRIANGLES_SLOT_NAME: &str = "RtTriangles";
pub const RT_TRIANGLE_META_SLOT_NAME: &str = "RtTriangleMeta";
pub const RT_BVH_SLOT_NAME: &str = "RtBvh";

impl Drop for DrawBuilder<'_> {
    /// O 0x14089a140：panic 计数为 0 且 packet 仍在 → 取走（置 tag 2）并
    /// `panic!("The DrawBuilder for {name} was not drawn or discarded.")`（name = 着色器程序名，`+0x48`）。
    /// 线程已在 panic 中则什么都不做（packet 随后正常 drop）。
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        if let Some(_p) = self.packet.try_take() {
            panic!(
                "The DrawBuilder for {} was not drawn or discarded.",
                self.shader.program_name
            );
        }
    }
}

/// `bind_stone_floor_textures` 的 12 个 (文件键, binding 名) 对（O：movabs，访问均 0x1d；顺序即调用顺序）。
pub const STONE_FLOOR_TEXTURES: [(u64, u64); 12] = [
    (0x7984363975076e17, 0x549dae761b7907e7),
    (0xbf18dbf7f353e814, 0x9f6388754d418697),
    (0x7f13c75f8edf1f67, 0xbbd21ec1fbd5f851),
    (0x4b14ae2c4d374912, 0xaac7f99e39b86bd6),
    (0x07556847ebc45f99, 0x0c1b2438c7aa4cae),
    (0x3f5b034a6b0ceeaa, 0x89e73acd87920c9e),
    (0x6983c7050feb1db1, 0xcc70a90c00557268),
    (0x4aaec8546504dd00, 0x5d0e7ef69cf6021b),
    (0x11e323391bf15b31, 0x0c1b2538c7aa4e61),
    (0x52da2cc9e7804cb7, 0x89e73bcd87920e51),
    (0x6e8417ac556e6be9, 0xcc70aa0c0055741b),
    (0xd0eacd1fd2aa71b9, 0x5d0e7df69cf60068),
];

/// `bind_wooden_floor_textures` 的 12 个 (文件键, binding 名) 对（O：movabs，访问均 0x1d；顺序即调用顺序）。
pub const WOODEN_FLOOR_TEXTURES: [(u64, u64); 12] = [
    (0xa167abe5c09ef232, 0x1090f9f98a8552b8),
    (0xac956f410bf28b6d, 0x9f4e44be9acb1102),
    (0xde4e62a2bfc6f041, 0x05d9c6ac8e038e70),
    (0x8fbac5267f2e4f04, 0xe3067790c8f7cdd3),
    (0xddb5758ccd1e3754, 0x40909fd60dab70f5),
    (0xe9c85d807ea39f26, 0x90fd775efda58d0f),
    (0x0479840e40f91b39, 0xad98bb970203911d),
    (0x29cad535fbefcfc0, 0x2bb9d453860af7a2),
    (0x08569dfad37d8b79, 0x40909ed60dab6f42),
    (0x5220274ab38287b2, 0x90fd765efda58b5c),
    (0xcee4cafe2a420814, 0xad98ba9702038f6a),
    (0x682b5c5201d52d5c, 0x2bb9d553860af955),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_stream::DEPTH_STATE_NONE;
    use crate::render::pass_builder::draw_stream::test_host::{Log, MockHost};
    use crate::render::pass_builder::framebuffer_binding::ColorAttachment;
    use crate::render::shader::RasterShaderProgram;
    use crate::rhapsody_contract::Handle;
    use alloc::rc::Rc;
    use core::cell::RefCell;
    use hashbrown::HashMap;

    struct Ssbo;
    impl SsboType for Ssbo {
        fn rg_buffer_hash() -> u64 {
            0xAAAA
        }
    }
    struct Ssbo2;
    impl SsboType for Ssbo2 {
        fn rg_buffer_hash() -> u64 {
            0xBBBB
        }
    }

    const SHADER: u64 = 1;
    const BLOCK: u32 = 0x40;

    fn refl() -> ShaderReflection {
        let mut bindings = HashMap::new();
        bindings.insert(0x10u64, (0u32, 3u32)); // name 0x10 -> set0 binding3
        bindings.insert(0x11u64, (0u32, 4u32));
        bindings.insert(0x12u64, (1u32, 9u32)); // set != 0 -> assert
        let mut uniform_offsets = HashMap::new();
        uniform_offsets.insert(0x20u64, 0u32);
        uniform_offsets.insert(0x21u64, 8u32);
        uniform_offsets.insert(0x22u64, 16u32);
        uniform_offsets.insert(0x23u64, 0x3cu32); // 4 bytes to the end of a 0x40 block
        ShaderReflection {
            program_name: "test_prog",
            bindings,
            has_uniform_block: true,
            uniform_offsets,
            uniform_block_size: BLOCK,
            uniform_block_binding: 7,
        }
    }

    fn handle() -> Handle<RasterShaderProgram> {
        Handle(ManagedHandle(SHADER), core::marker::PhantomData)
    }

    fn make_pass_with(r: ShaderReflection) -> (RasterPass, Rc<RefCell<Log>>) {
        let (mut host, log) = MockHost::new();
        host.shaders.insert(SHADER, Arc::new(r));
        host.rg_buffers.insert(0xAAAA, 5);
        host.rg_buffers.insert(0xBBBB, 6);
        host.vk_buffers.insert(42, 0xDEAD);
        let pass = RasterPass::new("t", &[], None, host.boxed());
        (pass, log)
    }

    fn make_pass() -> (RasterPass, Rc<RefCell<Log>>) {
        make_pass_with(refl())
    }

    fn builder(pass: &mut RasterPass) -> DrawBuilder<'_> {
        let st = pass.host.default_raster_state();
        DrawBuilder::new(pass, DrawPacket::new(handle(), [0, 1], st), 2)
    }

    /// 取走 pass 里已提交的 draw，并让 pass 通过 render 检查。
    fn finish_pass(mut pass: RasterPass) -> Vec<DrawPacket> {
        let d = core::mem::take(&mut pass.draws);
        pass.rendered.0 = true;
        d
    }

    #[test]
    fn new_sets_variant_allocates_constants_and_inserts_uniform_binding() {
        let (mut pass, log) = make_pass();
        let b = builder(&mut pass);
        assert_eq!(b.packet().variant, 2); // 0x140905c12: byte [packet+0xb0] = variant
        assert_eq!(b.constants_data().unwrap().len(), BLOCK as usize);
        assert_eq!(b.common_flag(), CommonDrawFlag::Off); // +0xf0 = 0
        assert!(matches!(
            b.packet().bindings.get(&7).unwrap().binding,
            DescriptorSetBinding::UniformBuffer { size: BLOCK, .. }
        ));
        assert_eq!(log.borrow().constants_requests, 1);
        b.draw();
        finish_pass(pass);
    }

    #[test]
    fn new_without_uniform_block_skips_allocation() {
        let mut r = refl();
        r.has_uniform_block = false; // [refl+0xe8] == 0
        let (mut pass, log) = make_pass_with(r);
        let b = builder(&mut pass);
        assert!(b.constants_data().is_none()); // +0xde == 8
        assert!(b.packet().bindings.is_empty());
        assert_eq!(log.borrow().constants_requests, 0);
        b.discard();
        finish_pass(pass);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn new_with_unknown_shader_panics() {
        let (mut pass, _l) = make_pass();
        let st = pass.host.default_raster_state();
        let bad = Handle::<RasterShaderProgram>(ManagedHandle(99), core::marker::PhantomData);
        let _ = DrawBuilder::new(&mut pass, DrawPacket::new(bad, [0, 0], st), 0);
    }

    #[test]
    #[should_panic(expected = "The DrawBuilder for test_prog was not drawn or discarded.")]
    fn dropping_an_undrawn_builder_panics_with_program_name() {
        let (mut pass, _l) = make_pass();
        {
            let _b = builder(&mut pass);
        }
        pass.rendered.0 = true;
    }

    #[test]
    fn drawn_or_discarded_builders_drop_silently() {
        let (mut pass, _l) = make_pass();
        builder(&mut pass).draw();
        builder(&mut pass).discard();
        assert_eq!(pass.draw_count(), 1);
        finish_pass(pass);
    }

    #[test]
    fn no_drop_only_take_semantics() {
        let mut t = NoDropOnlyTake::new(5);
        assert!(!t.is_taken());
        assert_eq!(t.try_take(), Some(5));
        assert_eq!(t.try_take(), None);
        assert!(t.is_taken());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn no_drop_only_take_take_twice_panics() {
        let mut t = NoDropOnlyTake::new(1u8);
        t.take();
        t.take();
    }

    #[test]
    fn draw_kinds_and_payloads() {
        let (mut pass, _l) = make_pass();
        builder(&mut pass).draw_no_vao(3);
        builder(&mut pass).draw_no_vao_indexed(&ManagedHandle(9), 12);
        builder(&mut pass).draw_no_index_buffer(6);
        builder(&mut pass).draw_instanced_w_gl_draw_flag(77);
        builder(&mut pass).draw_multi_instanced(alloc::vec![[1, 2], [3, 4]]);
        builder(&mut pass).draw();
        let d = finish_pass(pass);
        assert_eq!(d[0].kind, DrawKind::NoVao { vertex_count: 3 });
        assert_eq!(
            d[1].kind,
            DrawKind::NoVaoIndexed {
                index_count: 12,
                index_buffer: ManagedHandle(9)
            }
        );
        assert_eq!(d[2].kind, DrawKind::NoIndexBuffer { vertex_count: 6 });
        assert_eq!(d[3].kind, DrawKind::InstancedGlDrawFlag { flag: 77 });
        assert_eq!(
            d[4].kind,
            DrawKind::MultiInstanced {
                ranges: alloc::vec![[1, 2], [3, 4]]
            }
        );
        // `draw`（0x140906f40）不改 kind：保持 DrawPacket::new 的缺省 (3, 1)。
        assert_eq!(d[5].kind, DrawKind::InstancedGlDrawFlag { flag: 1 });
        assert_eq!(
            d.iter().map(|p| p.variant).collect::<Vec<_>>(),
            alloc::vec![2; 6]
        );
    }

    #[test]
    fn later_draw_overwrites_kind_payload() {
        // 机器码：先释放旧 kind（tag 4 释放 Vec / tag 1 释放句柄）再写新 kind —— 这里等价于直接覆盖。
        let (mut pass, _l) = make_pass();
        let mut b = builder(&mut pass);
        b.packet_mut().kind = DrawKind::MultiInstanced {
            ranges: alloc::vec![[1, 1]],
        };
        b.draw_no_vao(4);
        let d = finish_pass(pass);
        assert_eq!(d[0].kind, DrawKind::NoVao { vertex_count: 4 });
    }

    #[test]
    fn indirect_draws_register_accesses_and_fields() {
        let (mut pass, _l) = make_pass();
        builder(&mut pass).multi_draw_indirect::<Ssbo>(100, 200);
        builder(&mut pass).multi_draw_indexed_indirect::<Ssbo2>(1, 7, 8);
        builder(&mut pass).draw_indexed_indirect_raw(21, 0x30, 0x40, 0x50);
        builder(&mut pass).draw_indexed_indirect_count(31, 32, [1, 2, 3, 4]);
        assert_eq!(
            pass.accesses,
            alloc::vec![
                PassAccess {
                    resource: 5,
                    access: 2,
                    group: 1
                },
                PassAccess {
                    resource: 6,
                    access: 2,
                    group: 1
                },
                PassAccess {
                    resource: 21,
                    access: 2,
                    group: 1
                },
                PassAccess {
                    resource: 31,
                    access: 2,
                    group: 1
                },
                PassAccess {
                    resource: 32,
                    access: 2,
                    group: 1
                },
            ]
        );
        let d = finish_pass(pass);
        assert_eq!(
            d[0].kind,
            DrawKind::MultiDrawIndirect {
                buffer: 5,
                arg_50: 100,
                arg_58: 200
            }
        );
        // idx 1 → 偏移 0x14、静态表首字 0x12c（0x142aaaa78 + 0x14）。
        assert_eq!(
            d[1].kind,
            DrawKind::IndexedIndirect {
                buffer: 6,
                arg_48: 0x14,
                arg_50: 7,
                arg_58: 8,
                first_index_count: Some(0x12c)
            }
        );
        assert_eq!(
            d[2].kind,
            DrawKind::IndexedIndirect {
                buffer: 21,
                arg_48: 0x30,
                arg_50: 0x50,
                arg_58: 0x40,
                first_index_count: None
            }
        );
        assert_eq!(
            d[3].kind,
            DrawKind::IndexedIndirectCount {
                buffer: 31,
                count_buffer: 32,
                args: [1, 2, 3, 4]
            }
        );
    }

    #[test]
    fn indexed_indirect_table_matches_binary() {
        // 0x142aaaa78 起每 0x14 字节的首个 u32。
        assert_eq!(INDEXED_INDIRECT_INDEX_COUNTS[0], 0x1b);
        assert_eq!(INDEXED_INDIRECT_INDEX_COUNTS[9], 0xae);
        assert_eq!(INDEXED_INDIRECT_STRIDE, 0x14);
    }

    #[test]
    fn optional_uniform_writes_at_offset_and_ignores_misses() {
        let (mut pass, _l) = make_pass();
        let b = builder(&mut pass)
            .optional_uniform(0x20, 1.5f32)
            .optional_uniform(0x21, [7u32, 9u32])
            .optional_uniform(0x22, true) // bool → u32
            .optional_uniform(0x999, 3.0f32); // 未命中：静默
        let data = b.constants_data().unwrap().to_vec();
        assert_eq!(&data[0..4], &1.5f32.to_le_bytes());
        assert_eq!(&data[8..12], &7u32.to_le_bytes());
        assert_eq!(&data[12..16], &9u32.to_le_bytes());
        assert_eq!(&data[16..20], &1u32.to_le_bytes());
        assert!(data[20..].iter().all(|&x| x == 0));
        b.discard();
        finish_pass(pass);
    }

    /// 接线缺口回归：`constants.data` 是原版“可写映射切片”的拥有型替身，`draw*` 消费 builder 时必须把字节交回宿主，
    /// 否则 uniform 值随 builder 一起丢弃、永远到不了 GPU（宿主端到端的像素测试发现）。
    #[test]
    fn finishing_a_draw_flushes_the_uniform_bytes_to_the_host() {
        let (mut pass, log) = make_pass();
        builder(&mut pass)
            .optional_uniform(0x20, 1.5f32)
            .optional_uniform(0x21, [7u32, 9u32])
            .draw_no_vao(3);
        {
            let log = log.borrow();
            assert_eq!(log.flushed_constants.len(), 1, "一次 draw 恰好交回一次");
            let (keepalive, data) = &log.flushed_constants[0];
            assert_eq!(*keepalive, 7, "按分配时的 keepalive 句柄交回");
            assert_eq!(data.len(), BLOCK as usize);
            assert_eq!(&data[0..4], &1.5f32.to_le_bytes());
            assert_eq!(&data[8..12], &7u32.to_le_bytes());
        }
        // `discard` 不画，不交回。
        builder(&mut pass).optional_uniform(0x20, 2.0f32).discard();
        assert_eq!(log.borrow().flushed_constants.len(), 1);
        finish_pass(pass);
    }

    /// 光栅 pass 的 RG 纹理绑定必须带着视图描述到宿主（原 `RenderPassBinding` 的 0x3c B 视图块）。
    #[test]
    fn rg_texture_binding_keeps_the_requested_view_desc() {
        let (mut pass, _l) = make_pass();
        let tex = AttachmentTexture {
            id: 9,
            extent: (4, 4),
            format: 37,
            sample_count: 1,
        };
        let view = TextureViewDesc::new()
            .base_mip_level(2)
            .level_count(Some(1));
        builder(&mut pass)
            .bind_texture_to_draw_builder(
                BindTexture::RgHandle(&tex),
                0x10,
                ACCESS_SAMPLED_IMAGE,
                Some(&view),
            )
            .draw_no_vao(3);
        assert_eq!(pass.bindings.len(), 1);
        assert_eq!(pass.bindings[0].resource_index, 9);
        assert_eq!(pass.bindings[0].view, Some(view));
        // 不带视图：None（默认视图）。
        let tex2 = AttachmentTexture { id: 10, ..tex };
        builder(&mut pass)
            .bind_texture_to_draw_builder(
                BindTexture::RgHandle(&tex2),
                0x10,
                ACCESS_SAMPLED_IMAGE,
                None,
            )
            .draw_no_vao(3);
        assert_eq!(pass.bindings[1].view, None);
        finish_pass(pass);
    }

    #[test]
    fn optional_uniform_mat4_is_64_bytes_column_major() {
        let mut r = refl();
        r.uniform_offsets.insert(0x30, 0);
        let (mut pass, _l) = make_pass_with(r);
        let m = glam::Mat4::from_translation(glam::Vec3::new(1.0, 2.0, 3.0));
        let b = builder(&mut pass).uniform(0x30, m);
        let data = b.constants_data().unwrap();
        let cols = m.to_cols_array();
        for (i, v) in cols.iter().enumerate() {
            assert_eq!(&data[i * 4..i * 4 + 4], &v.to_le_bytes());
        }
        b.discard();
        finish_pass(pass);
    }

    #[test]
    fn uniform_noop_without_block_or_offsets_or_constants() {
        let mut r = refl();
        r.uniform_offsets.clear(); // [refl+0x130] == 0
        let (mut pass, _l) = make_pass_with(r);
        let b = builder(&mut pass).optional_uniform(0x20, 1.0f32);
        assert!(b.constants_data().unwrap().iter().all(|&x| x == 0));
        b.discard();
        finish_pass(pass);
        let mut r = refl();
        r.has_uniform_block = false;
        let (mut pass, _l) = make_pass_with(r);
        builder(&mut pass).optional_uniform(0x20, 1.0f32).discard();
        finish_pass(pass);
    }

    #[test]
    #[should_panic]
    fn uniform_write_past_block_end_panics() {
        // offset 0x3c 处写 8 字节越过 0x40 块 → 越界（slice 索引失败）。
        let (mut pass, _l) = make_pass();
        let _ = builder(&mut pass).optional_uniform(0x23, [1u32, 2u32]);
    }

    #[test]
    fn raster_content_intents_only_tag_actual_reflected_bindings() {
        let (mut pass, _) = make_pass();
        let texture = AttachmentTexture { id: 50, extent: (4,4), format: 37, sample_count: 1 };
        let b = builder(&mut pass)
            .bind_texture_to_draw_builder(BindTexture::RgHandle(&texture), 0x10, ACCESS_SAMPLED_IMAGE, None)
            .inactive_texture_contents(0x10).inactive_texture_contents(0x10)
            .write_rg_storage_buffer(0x11, 60)
            .preserve_storage_buffer_contents(0x11).preserve_storage_buffer_contents(0x11)
            .inactive_texture_contents(0xdead).preserve_storage_buffer_contents(0xdead);
        assert_eq!(b.packet().inactive_texture_bindings, [3]);
        assert_eq!(b.packet().preserving_buffer_bindings, [4]);
        assert_eq!(b.packet().accesses.len(), 2);
        b.discard();
        assert_eq!(pass.accesses[0].access, ACCESS_SAMPLED_IMAGE);
        assert_eq!(pass.accesses[1].access, ACCESS_RG_STORAGE_WRITE);
        finish_pass(pass);
    }

    #[test]
    fn rg_storage_buffer_access_types_and_groups() {
        let (mut pass, _l) = make_pass();
        let b = builder(&mut pass)
            .read_rg_storage_buffer(0x10, 50)
            .write_rg_storage_buffer(0x11, 60);
        let acc = b.packet().accesses.clone();
        assert_eq!(
            acc,
            alloc::vec![
                ResourceAccess {
                    binding_index: 3,
                    slot: 0
                },
                ResourceAccess {
                    binding_index: 4,
                    slot: 1
                }
            ]
        );
        b.discard();
        assert_eq!(
            pass.accesses,
            alloc::vec![
                PassAccess {
                    resource: 50,
                    access: 0x24,
                    group: 1
                },
                PassAccess {
                    resource: 60,
                    access: 0x37,
                    group: 0
                },
            ]
        );
        assert_eq!(pass.bindings.len(), 2);
        finish_pass(pass);
    }

    #[test]
    fn rg_storage_buffer_missing_name_is_noop() {
        let (mut pass, _l) = make_pass();
        builder(&mut pass)
            .read_rg_storage_buffer(0xdead, 50)
            .discard();
        assert!(pass.accesses.is_empty() && pass.bindings.is_empty());
        finish_pass(pass);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn rg_storage_buffer_nonzero_set_asserts() {
        let (mut pass, _l) = make_pass();
        let _ = builder(&mut pass).read_rg_storage_buffer(0x12, 1);
    }

    #[test]
    fn read_storage_buffer_reuses_slot_by_name() {
        let (mut pass, _l) = make_pass();
        let b = builder(&mut pass)
            .read_storage_buffer::<Ssbo>("Foo", 0x10)
            .read_storage_buffer::<Ssbo>("Foo", 0x11);
        let acc = b.packet().accesses.clone();
        // 两次访问共享同一槽 0，只登记一次 pass 绑定/访问（0x1f，同步组 1）。
        assert_eq!(
            acc,
            alloc::vec![
                ResourceAccess {
                    binding_index: 3,
                    slot: 0
                },
                ResourceAccess {
                    binding_index: 4,
                    slot: 0
                }
            ]
        );
        b.discard();
        assert_eq!(
            pass.accesses,
            alloc::vec![PassAccess {
                resource: 5,
                access: 0x1f,
                group: 1
            }]
        );
        assert_eq!(pass.buffer_slots.get("Foo"), Some(&0));
        finish_pass(pass);
    }

    #[test]
    fn read_storage_buffer_unknown_binding_still_registers_slot() {
        let (mut pass, _l) = make_pass();
        let b = builder(&mut pass).read_storage_buffer::<Ssbo>("Foo", 0xbad);
        assert!(b.packet().accesses.is_empty());
        b.discard();
        assert_eq!(pass.bindings.len(), 1);
        finish_pass(pass);
    }

    #[test]
    fn bind_external_storage_buffer_inserts_whole_buffer_binding() {
        let (mut pass, _l) = make_pass();
        let b = builder(&mut pass).bind_external_storage_buffer(0x10, &ManagedHandle(42));
        assert_eq!(
            b.packet().bindings.get(&3).unwrap().binding,
            DescriptorSetBinding::StorageBuffer {
                buffer: 0xDEAD,
                offset: 0,
                range: u64::MAX,
                keepalive: ManagedHandle(42)
            }
        );
        let b = b.bind_external_storage_buffer(0xdead, &ManagedHandle(42)); // 未命中：不变
        assert_eq!(b.packet().bindings.len(), 2); // 常量块 + 3
        b.discard();
        finish_pass(pass);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn bind_external_storage_buffer_without_vk_buffer_panics() {
        let (mut pass, _l) = make_pass();
        let _ = builder(&mut pass).bind_external_storage_buffer(0x10, &ManagedHandle(1));
    }

    #[test]
    fn discard_disarms_every_binding() {
        // 0x1409082c0：所有桶的状态字节写 2。
        let (mut pass, _l) = make_pass();
        let mut b = builder(&mut pass).bind_external_storage_buffer(0x10, &ManagedHandle(42));
        assert!(
            b.packet()
                .bindings
                .values()
                .all(|g| g.state == GpuOwned::STATE_OWNED)
        );
        let p: *const DrawPacket = b.packet();
        let _ = p;
        // 取出 packet 观察需要 discard 内部；这里改为：discard 后不 panic，且 builder 不再检查。
        b.packet_mut()
            .bindings
            .values_mut()
            .for_each(|g| g.state = 0);
        b.discard();
        finish_pass(pass);
        assert_eq!(GpuOwned::STATE_DISCARDED, 2);
    }

    #[test]
    fn placement_data_binds_buffer_then_draws_with_stored_flag() {
        // 注入 placement 表项
        let (mut host, _l2) = MockHost::new();
        host.shaders.insert(
            SHADER,
            Arc::new({
                let mut r = refl();
                r.bindings.insert(PLACEMENT_DATA_BINDING_HASH, (0, 8));
                r
            }),
        );
        host.placement.insert(77, (ManagedHandle(42), 5));
        host.vk_buffers.insert(42, 0xBEEF);
        let mut pass = RasterPass::new("t", &[], None, host.boxed());
        builder(&mut pass).draw_instanced_from_placement_data(77);
        let d = finish_pass(pass);
        assert_eq!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag: 5 });
        assert!(matches!(
            d[0].bindings.get(&8).unwrap().binding,
            DescriptorSetBinding::StorageBuffer { buffer: 0xBEEF, .. }
        ));
    }

    #[test]
    #[should_panic(
        expected = "This Handle for PlacementDataSSBO doesn't have a corresponding buffer"
    )]
    fn placement_data_missing_entry_panics_with_original_text() {
        let (mut pass, _l) = make_pass();
        builder(&mut pass).draw_instanced_from_placement_data(1);
    }

    #[test]
    fn placement_message_length_matches_expect_failed_arg() {
        assert_eq!(PLACEMENT_MISSING_MSG.len(), 0x45); // mov edx, 0x45
    }

    fn tex(id: u32) -> AttachmentTexture {
        AttachmentTexture {
            id,
            extent: (16, 8),
            format: 1,
            sample_count: 1,
        }
    }

    #[test]
    fn rg_texture_binds_once_per_key_and_writes_dims() {
        let mut r = refl();
        r.uniform_offsets.insert(0x10 ^ 0xD1D1, 0x20); // 维度 uniform（MockHost::with_suffix_dims = ^0xD1D1）
        let (mut pass, _l) = make_pass_with(r);
        let t = tex(11);
        let b = builder(&mut pass)
            .bind_texture_to_draw_builder(
                BindTexture::RgHandle(&t),
                0x10,
                ACCESS_SAMPLED_IMAGE,
                None,
            )
            .bind_texture_to_draw_builder(
                BindTexture::RgHandle(&t),
                0x11,
                ACCESS_SAMPLED_IMAGE,
                None,
            );
        assert_eq!(
            b.packet().accesses,
            alloc::vec![
                ResourceAccess {
                    binding_index: 3,
                    slot: 0
                },
                ResourceAccess {
                    binding_index: 4,
                    slot: 0
                }
            ]
        );
        let data = b.constants_data().unwrap();
        assert_eq!(&data[0x20..0x24], &16u32.to_le_bytes());
        assert_eq!(&data[0x24..0x28], &8u32.to_le_bytes());
        b.discard();
        // 同一 (id, 无 view) 键只登记一次访问/绑定。
        assert_eq!(
            pass.accesses,
            alloc::vec![PassAccess {
                resource: 11,
                access: 0x1d,
                group: 1
            }]
        );
        assert_eq!(pass.bindings.len(), 1);
        assert!(
            pass.texture_slots
                .contains_key(&TextureBindingKey(0x4000_0000_0000_000b))
        );
        finish_pass(pass);
    }

    #[test]
    fn rg_texture_with_view_gets_a_distinct_key() {
        let (mut pass, _l) = make_pass();
        let t = tex(11);
        let v = TextureViewDesc::default();
        let b = builder(&mut pass)
            .bind_texture_to_draw_builder(
                BindTexture::RgHandle(&t),
                0x10,
                ACCESS_SAMPLED_IMAGE,
                None,
            )
            .bind_texture_to_draw_builder(
                BindTexture::RgHandle(&t),
                0x11,
                ACCESS_SAMPLED_IMAGE,
                Some(&v),
            );
        assert_eq!(b.packet().accesses[1].slot, 1);
        b.discard();
        assert_eq!(pass.bindings.len(), 2);
        finish_pass(pass);
    }

    #[test]
    #[should_panic(expected = "for writing, but it's not writable")]
    fn rg_handle_write_access_panics() {
        let (mut pass, _l) = make_pass();
        let t = tex(1);
        let _ = builder(&mut pass).bind_texture_to_draw_builder(
            BindTexture::RgHandle(&t),
            0x10,
            0x2f,
            None,
        );
    }

    #[test]
    fn rg_texture_unknown_name_only_writes_dims() {
        let mut r = refl();
        r.uniform_offsets.insert(0x77 ^ 0xD1D1, 0);
        let (mut pass, _l) = make_pass_with(r);
        let t = tex(2);
        let b = builder(&mut pass).bind_texture_to_draw_builder(
            BindTexture::RgHandle(&t),
            0x77,
            ACCESS_SAMPLED_IMAGE,
            None,
        );
        assert!(b.packet().accesses.is_empty());
        assert_eq!(&b.constants_data().unwrap()[0..4], &16u32.to_le_bytes());
        b.discard();
        assert!(pass.accesses.is_empty());
        finish_pass(pass);
    }

    #[test]
    fn preset_texture_allows_write_with_group_zero() {
        let (mut host, _log) = MockHost::new();
        host.shaders.insert(SHADER, Arc::new(refl()));
        host.presets.insert(9, tex(30));
        let mut pass = RasterPass::new("t", &[], None, host.boxed());
        builder(&mut pass)
            .bind_texture_to_draw_builder(BindTexture::Preset(TexturePreset(9)), 0x10, 0x2f, None)
            .discard();
        assert_eq!(
            pass.accesses,
            alloc::vec![PassAccess {
                resource: 30,
                access: 0x2f,
                group: 0
            }]
        );
        finish_pass(pass);
    }

    #[test]
    fn raw_and_disk_textures_insert_sampled_image_bindings() {
        let (mut host, _log) = MockHost::new();
        host.shaders.insert(SHADER, Arc::new(refl()));
        host.textures_by_name
            .insert(0xF11E, (ManagedHandle(5), tex(40)));
        let mut pass = RasterPass::new("t", &[], None, host.boxed());
        let b = builder(&mut pass).bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(0xF11E)),
            0x10,
            ACCESS_SAMPLED_IMAGE,
            None,
        );
        assert_eq!(
            b.packet().bindings.get(&3).unwrap().binding,
            DescriptorSetBinding::SampledImage {
                image_view: 0x1005,
                layout_word: 5,
                keepalive: ManagedHandle(5)
            }
        );
        let h = ManagedHandle(6);
        let d = tex(41);
        let b = b.bind_texture_to_draw_builder(
            BindTexture::Raw {
                texture: &h,
                desc: &d,
            },
            0x11,
            ACCESS_SAMPLED_IMAGE,
            None,
        );
        assert!(matches!(
            b.packet().bindings.get(&4).unwrap().binding,
            DescriptorSetBinding::SampledImage {
                image_view: 0x1006,
                ..
            }
        ));
        b.discard();
        assert!(pass.accesses.is_empty(), "原生纹理不走 RG 访问登记");
        finish_pass(pass);
    }

    #[test]
    #[should_panic(expected = "assertion failed: view.is_none()")]
    fn raw_texture_rejects_views() {
        let (mut pass, _l) = make_pass();
        let h = ManagedHandle(6);
        let d = tex(41);
        let v = TextureViewDesc::default();
        let _ = builder(&mut pass).bind_texture_to_draw_builder(
            BindTexture::Raw {
                texture: &h,
                desc: &d,
            },
            0x10,
            ACCESS_SAMPLED_IMAGE,
            Some(&v),
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: !access.is_write()")]
    fn raw_texture_rejects_write_access() {
        let (mut pass, _l) = make_pass();
        let h = ManagedHandle(6);
        let d = tex(41);
        let _ = builder(&mut pass).bind_texture_to_draw_builder(
            BindTexture::Raw {
                texture: &h,
                desc: &d,
            },
            0x10,
            0x30,
            None,
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: !access.is_write()")]
    fn disk_texture_rejects_write_access() {
        let (mut pass, _l) = make_pass();
        let _ = builder(&mut pass).bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(1)),
            0x10,
            0x30,
            None,
        );
    }

    fn common_pass(preset_tex: &[(u8, u32)]) -> RasterPass {
        let mut r = refl();
        r.bindings.insert(BIND_COMMON_PRESET_NAME, (0, 10));
        r.bindings.insert(COLOR_LIBRARY_BINDING_HASH, (0, 11));
        for (i, h) in [
            BIND_COMMON_FLAG_UNIFORM,
            BIND_COMMON_MODEL_MATRIX,
            BIND_COMMON_FLAG_INT_UNIFORM,
            BIND_COMMON_TIME,
            BIND_COMMON_VIEWPORT,
        ]
        .iter()
        .enumerate()
        {
            r.uniform_offsets
                .insert(*h, [0u32, 0x10, 0x8, 0xc, 0x50][i]);
        }
        let (mut host, _log) = MockHost::new();
        host.shaders.insert(
            SHADER,
            Arc::new(ShaderReflection {
                uniform_block_size: 0x60,
                ..r
            }),
        );
        host.rg_buffers.insert(0xAAAA, 5);
        for &(p, id) in preset_tex {
            host.presets.insert(p, tex(id));
        }
        RasterPass::new("t", &[], None, host.boxed()).with_viewport([1920, 1080])
    }

    #[test]
    fn bind_common_flag_selects_preset_and_writes_uniforms() {
        let tr = DrawTransform {
            rotation: glam::Quat::IDENTITY,
            translation: glam::Vec3::new(1.0, 2.0, 3.0),
            scale: glam::Vec3::ONE,
        };
        // 0..=4 全部取值（`shadow_pass` 传 cascade+2）。
        for flag in [
            CommonDrawFlag::Off,
            CommonDrawFlag::On,
            CommonDrawFlag::ShadowCascade0,
            CommonDrawFlag::ShadowCascade1,
            CommonDrawFlag::ShadowCascade2,
        ] {
            let on = flag.is_set();
            let mut pass = common_pass(&[(0x11, 100), (0x12, 200)]);
            let b =
                builder(&mut pass).bind_common::<Ssbo>(&CommonDrawParams { time: 2.5, flag }, &tr);
            assert_eq!(b.common_flag(), flag); // +0xf0 保存完整字节
            assert_eq!(b.common_flag() as u8, flag as u8);
            let d = b.constants_data().unwrap().to_vec();
            // O 0x14090b4ea：bool uniform = (flag == 0)（`test r14d, r14d; je` → 0 路径 `mov r9b, 1`）。
            assert_eq!(
                u32::from_le_bytes(d[0..4].try_into().unwrap()),
                (!on) as u32
            ); // bool uniform
            assert_eq!(
                u32::from_le_bytes(d[8..12].try_into().unwrap()),
                flag as u8 as u32,
                "u32 uniform 写整个字节（2..=4 不被截成 1）"
            ); // u32 uniform
            assert_eq!(&d[0xc..0x10], &2.5f32.to_le_bytes()); // 时间
            assert_eq!(&d[0x50..0x54], &1920u32.to_le_bytes()); // 视口
            assert_eq!(&d[0x54..0x58], &1080u32.to_le_bytes());
            let m = tr.compute_matrix().to_cols_array();
            assert_eq!(&d[0x10..0x14], &m[0].to_le_bytes()); // Mat4 起点
            // 预设纹理：flag ? 0x12 : 0x11
            let want = if on { 200 } else { 100 };
            assert_eq!(
                b.packet().accesses[0],
                ResourceAccess {
                    binding_index: 10,
                    slot: 0
                }
            );
            b.discard();
            assert!(pass.accesses.contains(&PassAccess {
                resource: want,
                access: 0x1d,
                group: 1
            }));
            // 颜色库：槽 "ColorLibrary"，访问 0x1f
            assert!(pass.buffer_slots.contains_key(COLOR_LIBRARY_SLOT_NAME));
            assert!(pass.accesses.contains(&PassAccess {
                resource: 5,
                access: 0x1f,
                group: 1
            }));
            finish_pass(pass);
        }
    }

    #[test]
    fn bind_common_skips_flag_uniforms_without_constants() {
        let mut r = refl();
        r.has_uniform_block = false;
        r.bindings.insert(BIND_COMMON_PRESET_NAME, (0, 10));
        let (mut host, _l) = MockHost::new();
        host.shaders.insert(SHADER, Arc::new(r));
        host.presets.insert(0x11, tex(100));
        host.rg_buffers.insert(0xAAAA, 5);
        let mut pass = RasterPass::new("t", &[], None, host.boxed());
        let tr = DrawTransform {
            rotation: glam::Quat::IDENTITY,
            translation: glam::Vec3::ZERO,
            scale: glam::Vec3::ONE,
        };
        let b = builder(&mut pass).bind_common::<Ssbo>(
            &CommonDrawParams {
                time: 0.0,
                flag: CommonDrawFlag::Off,
            },
            &tr,
        );
        assert!(b.constants_data().is_none());
        b.discard();
        finish_pass(pass);
    }

    #[test]
    fn bind_tables_have_observed_shape() {
        assert_eq!(WALL_COLOR_TEXTURES.len(), 4);
        assert_eq!(STONE_FLOOR_TEXTURES.len(), 12);
        assert_eq!(WOODEN_FLOOR_TEXTURES.len(), 12);
        assert_eq!(
            WALL_COLOR_TEXTURES[0],
            (0x4b15227408664b67, 0x58d5652d3b800ae7)
        ); // 0x14090bbbd/0x14090bbdb
        assert_eq!(WALL_COLOR_TEXTURES[3].1, (-0x442de13e042a07afi64) as u64);
        assert_eq!(
            STONE_FLOOR_TEXTURES[0],
            (0x7984363975076e17, 0x549dae761b7907e7)
        );
        assert_eq!(STONE_FLOOR_TEXTURES[11].0, (-0x2f1532e02d558e47i64) as u64);
        assert_eq!(WOODEN_FLOOR_TEXTURES[0].0, (-0x5e98541a3f610dcei64) as u64);
        assert_eq!(
            WOODEN_FLOOR_TEXTURES[11],
            (0x682b5c5201d52d5c, 0x2bb9d553860af955)
        );
        // 墙色与石地面共享两项（同一磁盘纹理）。
        assert_eq!(WALL_COLOR_TEXTURES[2], STONE_FLOOR_TEXTURES[0]);
        assert_eq!(WALL_COLOR_TEXTURES[3], STONE_FLOOR_TEXTURES[2]);
    }

    #[test]
    fn bind_disk_texture_groups_bind_every_table_entry() {
        let (mut host, _l) = MockHost::new();
        let mut r = refl();
        let mut names: Vec<u64> = Vec::new();
        for (_, n) in STONE_FLOOR_TEXTURES
            .iter()
            .chain(WOODEN_FLOOR_TEXTURES.iter())
            .chain(WALL_COLOR_TEXTURES.iter())
        {
            if !names.contains(n) {
                names.push(*n);
            }
            r.bindings.insert(
                *n,
                (0, 100 + names.iter().position(|x| x == n).unwrap() as u32),
            );
        }
        let distinct_wall_only = WALL_COLOR_TEXTURES
            .iter()
            .filter(|(_, n)| !STONE_FLOOR_TEXTURES.iter().any(|(_, m)| m == n))
            .count();
        assert_eq!(distinct_wall_only, 2);
        r.bindings.insert(WALL_COLOR_ID_BINDING_HASH, (0, 999));
        host.shaders.insert(SHADER, Arc::new(r));
        host.rg_buffers.insert(0xAAAA, 5);
        for (f, _) in STONE_FLOOR_TEXTURES
            .iter()
            .chain(WOODEN_FLOOR_TEXTURES.iter())
            .chain(WALL_COLOR_TEXTURES.iter())
        {
            host.textures_by_name
                .insert(*f, (ManagedHandle(*f & 0xffff), tex(1)));
        }
        let mut pass = RasterPass::new("t", &[], None, host.boxed());
        let b = builder(&mut pass).bind_stone_floor_textures();
        assert_eq!(b.packet().bindings.len(), 1 + 12);
        let b = b.bind_wooden_floor_textures();
        assert_eq!(b.packet().bindings.len(), 1 + 12 + 12);
        let b = b.bind_wall_color_customization::<Ssbo>();
        // 墙色 4 张：其中 2 张与石地面同 binding 名 → 覆盖不新增，另 2 张新增。
        assert_eq!(b.packet().bindings.len(), 1 + 12 + 12 + distinct_wall_only);
        // WallColorIds 读 SSBO（槽 "WallColorIds"）。
        assert!(pass_has_slot(b.pass_slots(), WALL_COLOR_ID_SLOT_NAME));
        assert_eq!(b.packet().accesses.last().unwrap().binding_index, 999);
        b.discard();
        finish_pass(pass);
    }

    fn pass_has_slot(slots: &HashMap<&'static str, u8>, name: &str) -> bool {
        slots.contains_key(name)
    }

    impl DrawBuilder<'_> {
        fn pass_slots(&self) -> &HashMap<&'static str, u8> {
            &self.pass.buffer_slots
        }
    }

    #[test]
    fn constants_match_binary_immediates() {
        assert_eq!(ACCESS_SAMPLED_IMAGE, 0x1d);
        assert_eq!(ACCESS_STORAGE_BUFFER_READ, 0x1f);
        assert_eq!(
            (ACCESS_RG_STORAGE_READ, ACCESS_RG_STORAGE_WRITE),
            (0x24, 0x37)
        );
        assert_eq!(MAX_PASS_BINDINGS, 0x100);
        assert_eq!(
            BIND_COMMON_FLAG_INT_UNIFORM,
            (-0x3a5498c04f79ae0bi64) as u64
        );
        assert_eq!(WALL_COLOR_ID_BINDING_HASH, (-0x168fe5da85577d74i64) as u64);
        assert_eq!(
            (BIND_COMMON_PRESET_ON, BIND_COMMON_PRESET_OFF),
            (0x12, 0x11)
        );
        assert_eq!(DEPTH_STATE_NONE, 3);
    }

    #[test]
    #[should_panic(expected = "TryFromIntError")]
    fn more_than_256_pass_bindings_panic() {
        let (mut pass, _l) = make_pass();
        for _ in 0..0x100 {
            pass.bindings.push(RenderPassBinding::default());
        }
        let _ = builder(&mut pass).read_rg_storage_buffer(0x10, 1);
    }

    #[test]
    fn color_attachment_pass_is_unaffected_by_builder() {
        // 颜色附件访问在 RasterPass::new 登记，与 DrawBuilder 无关。
        let (mut host, _l) = MockHost::new();
        host.shaders.insert(SHADER, Arc::new(refl()));
        let c = [ColorAttachment::Texture(tex(3))];
        let mut pass = RasterPass::new("t", &c, None, host.boxed());
        builder(&mut pass).draw();
        assert_eq!(pass.accesses.len(), 1);
        finish_pass(pass);
    }

    #[test]
    fn bind_brick_erase_highlight_writes_three_uniforms() {
        let mut r = refl();
        r.uniform_offsets.insert(BRICK_ERASE_ENABLED_UNIFORM, 0x00);
        r.uniform_offsets.insert(BRICK_ERASE_CURSOR_UNIFORM, 0x10);
        r.uniform_offsets.insert(BRICK_ERASE_RADIUS_UNIFORM, 0x20);
        assert_eq!(BRICK_ERASE_RADIUS_UNIFORM, (-0x4a7a31de4938d73ci64) as u64);
        let cases = [
            // (enabled, suppress, cursor, radius) -> bool uniform
            (
                true,
                false,
                Some(glam::Vec3::new(1.0, 2.0, 3.0)),
                4.5f32,
                1u32,
            ),
            (true, true, Some(glam::Vec3::new(1.0, 2.0, 3.0)), 4.5, 0), // D 抑制
            (false, false, None, 0.25, 0),                              // A 关；光标 None → 全 0
        ];
        for (enabled, suppress, cursor, radius, want_bool) in cases {
            let (mut pass, _l) = make_pass_with(r.clone());
            let b = builder(&mut pass).bind_brick_erase_highlight(&BrickEraseHighlightInputs {
                enabled,
                cursor_position: cursor,
                radius,
                suppress,
            });
            let d = b.constants_data().unwrap().to_vec();
            assert_eq!(u32::from_le_bytes(d[0..4].try_into().unwrap()), want_bool);
            let c = cursor.unwrap_or(glam::Vec3::ZERO).to_array();
            for (i, v) in c.iter().enumerate() {
                assert_eq!(&d[0x10 + 4 * i..0x14 + 4 * i], &v.to_le_bytes());
            }
            assert_eq!(&d[0x20..0x24], &radius.to_le_bytes());
            b.discard();
            finish_pass(pass);
        }
    }

    #[test]
    fn bind_brick_erase_highlight_is_silent_without_constants() {
        let mut r = refl();
        r.has_uniform_block = false; // 常量块为 None（+0xde == 8）→ 三次写入全部跳过
        let (mut pass, _l) = make_pass_with(r);
        let b = builder(&mut pass).bind_brick_erase_highlight(&BrickEraseHighlightInputs {
            enabled: true,
            cursor_position: Some(glam::Vec3::ONE),
            radius: 1.0,
            suppress: false,
        });
        assert!(b.constants_data().is_none());
        b.discard();
        finish_pass(pass);
    }

    // ---------------------------------------------------------------- B 阶段（i_api）

    struct RtTri;
    impl SsboType for RtTri {
        fn rg_buffer_hash() -> u64 {
            0xC001
        }
    }
    struct RtMeta;
    impl SsboType for RtMeta {
        fn rg_buffer_hash() -> u64 {
            0xC002
        }
    }
    struct RtBvhT;
    impl SsboType for RtBvhT {
        fn rg_buffer_hash() -> u64 {
            0xC003
        }
    }
    struct WallT;
    impl SsboType for WallT {
        fn rg_buffer_hash() -> u64 {
            0xC004
        }
    }

    fn rt_pass(words: &[(u64, u32)]) -> RasterPass {
        let mut r = refl();
        for (i, h) in [
            RT_TRIANGLES_BINDING_HASH,
            RT_TRIANGLE_META_BINDING_HASH,
            RT_BVH_BINDING_HASH,
            WALL_COLOR_ID_BINDING_HASH,
        ]
        .into_iter()
        .enumerate()
        {
            r.bindings.insert(h, (0, 20 + i as u32));
        }
        r.uniform_offsets.insert(RT_TRIANGLES_UNIFORM_HASH, 0x30);
        r.uniform_offsets.insert(RT_BVH_UNIFORM_HASH, 0x34);
        let (mut host, _l) = MockHost::new();
        host.shaders.insert(SHADER, Arc::new(r));
        for (h, id) in [(0xC001u64, 11u32), (0xC002, 12), (0xC003, 13), (0xC004, 14)] {
            host.rg_buffers.insert(h, id);
        }
        for &(h, w) in words {
            host.buffer_words.insert(h, w);
        }
        RasterPass::new("rt", &[], None, host.boxed())
    }

    #[test]
    fn bind_ray_tracing_draw_reads_four_slots_then_writes_the_two_buffer_words() {
        // O 0x1416289d0：三个 Rt 槽 + 墙色槽，访问 0x1f 同步组 1；末尾两个 `write_uniform<u32>`。
        let mut pass = rt_pass(&[(0xC001, 0x1234), (0xC003, 0x5678)]);
        let b = builder(&mut pass).bind_ray_tracing::<RtTri, RtMeta, RtBvhT, WallT>();
        let acc: Vec<_> = b
            .packet()
            .accesses
            .iter()
            .map(|a| a.binding_index)
            .collect();
        assert_eq!(
            acc,
            [20, 21, 22, 23],
            "Tri / Meta / Bvh / WallColorId 的 binding 序"
        );
        let d = b.constants_data().unwrap().to_vec();
        assert_eq!(&d[0x30..0x34], &0x1234u32.to_le_bytes());
        assert_eq!(&d[0x34..0x38], &0x5678u32.to_le_bytes());
        let mut slots: Vec<_> = b.pass_slots().keys().copied().collect();
        slots.sort();
        assert_eq!(
            slots,
            ["RtBvh", "RtTriangleMeta", "RtTriangles", "WallColorIds"]
        );
        b.discard();
        let resources: Vec<_> = pass
            .accesses
            .iter()
            .map(|a| (a.resource, a.access, a.group))
            .collect();
        assert_eq!(
            resources,
            [(11, 0x1f, 1), (12, 0x1f, 1), (13, 0x1f, 1), (14, 0x1f, 1)]
        );
        finish_pass(pass);
    }

    #[test]
    fn bind_ray_tracing_draw_reuses_existing_slots_and_unwraps_missing_buffers() {
        // 槽已存在（比如先读过 WallColorIds）→ 不再登记访问，只追加 packet.accesses。
        let mut pass = rt_pass(&[(0xC001, 1), (0xC003, 2)]);
        let b = builder(&mut pass)
            .read_storage_buffer::<WallT>(WALL_COLOR_ID_SLOT_NAME, WALL_COLOR_ID_BINDING_HASH)
            .bind_ray_tracing::<RtTri, RtMeta, RtBvhT, WallT>();
        assert_eq!(b.packet().accesses.len(), 5);
        b.discard();
        assert_eq!(pass.accesses.len(), 4, "墙色槽复用：4 次访问而不是 5 次");
        finish_pass(pass);
        // 宿主没有 Bvh 缓冲字 → `Option::unwrap` panic（在任何绑定之前）。
        let mut pass = rt_pass(&[(0xC001, 1)]);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let b = builder(&mut pass);
            let b = b.bind_ray_tracing::<RtTri, RtMeta, RtBvhT, WallT>();
            b.discard();
        }));
        assert!(r.is_err());
        assert!(
            pass.accesses.is_empty(),
            "panic 发生在第一次 read_access_buffer 之前"
        );
        pass.rendered.0 = true;
    }

    #[test]
    fn common_draw_flag_bytes_are_0_to_4_and_shadow_is_cascade_plus_two() {
        for b in 0u8..=4 {
            assert_eq!(CommonDrawFlag::from_byte(b).unwrap() as u8, b);
        }
        assert_eq!(CommonDrawFlag::from_byte(5), None);
        assert_eq!(
            CommonDrawFlag::shadow_cascade(0),
            Some(CommonDrawFlag::ShadowCascade0)
        );
        assert_eq!(
            CommonDrawFlag::shadow_cascade(2),
            Some(CommonDrawFlag::ShadowCascade2)
        );
        assert_eq!(
            CommonDrawFlag::shadow_cascade(3),
            None,
            "shadow_pass 只有 3 个级联（cmp r14, 3）"
        );
        assert_eq!(CommonDrawFlag::from(true), CommonDrawFlag::On);
        assert_eq!(CommonDrawFlag::from(false), CommonDrawFlag::Off);
        assert!(!CommonDrawFlag::Off.is_set() && CommonDrawFlag::ShadowCascade1.is_set());
        assert_eq!(CommonDrawFlag::default(), CommonDrawFlag::Off);
    }

    #[test]
    fn indexed_indirect_table_has_the_24_exe_entries() {
        // 0x142aaaa78 + i * 0x14 的首个 u32（逐项 rdva 读出；第 24 项起是别的数据）。
        let exe: [u32; 24] = [
            0x1b, 0x12c, 0x60, 0x39, 0x24, 0x6, 0x3c, 0x6, 0x30, 0xae, 0x0, 0x0, 0xb01, 0x126c,
            0x42, 0xb4, 0xf, 0x384, 0x12, 0x384, 0x12, 0x12, 0x12, 0x12,
        ];
        assert_eq!(INDEXED_INDIRECT_INDEX_COUNTS, exe);
        assert_eq!(INDEXED_INDIRECT_STRIDE, 0x14);
    }

    #[test]
    fn entity_draw_params_exposes_the_entity() {
        let it = crate::render::pass_builder::draw_list::DrawListItem::new(1, 0);
        assert_eq!(it.params.entity(), bevy_ecs::entity::Entity::PLACEHOLDER);
    }
}
