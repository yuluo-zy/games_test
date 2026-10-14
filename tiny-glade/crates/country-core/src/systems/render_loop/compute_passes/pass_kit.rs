//! `pass_kit.rs` — compute pass 声明层的**唯一**共用环境（B 阶段 i_infra 合并 c1 `pass_kit` / c2 `c2_support` /
//! c3 `indirect_params::pass_env` 三份等价实现；非原版模块，原版里这些调用都是内联的）。
//!
//! 原版每个 compute pass 函数的形状：`get_resource_id(<128 位 TypeId>)` → 读资源 → `ComputePass::new(name, source, world)` →
//! 一串 `bind_*` / `uniform` → `dispatch_*`（= `PassBuilder::render`）。重建里：
//! * world 由 `ComputePassInit` 的三个接缝（世界资源、着色器后端、uniform 池）代替，再加上渲染图帧接缝，打包成 [`PassEnv`]；
//! * `RenderGraph::create_raw_resource` / `get_or_create_temporal[_pair]`（rhapsody，外部库）= A4 的 [`RgResources`]
//!   （`render::pass_builder::rg_resources`，唯一一份），`PassBuilder::render` 的落点 = [`PassFrame::record`]；
//! * 纹理库 / SSBO 库 / 预设纹理描述一律经 world 里的 `PassResources`（`new_from`）读取，不再另开接缝。
//!
//! # 绑定名哈希（O）
//! `ComputePass::{bind_texture, uniform, *_storage_buffer}` 的 `r8` 立即数 = `BindingNameHash::new(name)`（O 0x140c87660）：
//! 名字以 `_dims` 结尾 → `fnv1a_64(去掉后缀) + 1`，否则 `fnv1a_64(名字)`；名字即 HLSL 里的资源 / `$Globals` 字段名
//! （`__alias_` 前缀已剥掉）。[`binding_hash`] 是 **`const fn`**（唯一一份）：各 pass 文件里的绑定名常量要写在 `const` 里
//! （`fnv` crate 的哈希器不能在常量表达式里用），测试里再用真实 `fnv` crate 交叉校验。纹理库键 / SSBO 键是 `NameHash`
//! （xxh3_64），与此不同。
//!
//! # 访问码（O）
//! 采样读 `0x19`、存储写 `0x32`、SPD mip 视图写 `0x37`（见 [`TEX_READ`] 等）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::redundant_closure)]
#![allow(dead_code)]

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType as LibrarySsbo};
use crate::asset_libraries::texture_library::AssetTextureLibrary;
use crate::render::pass_builder::compute_pass_builder::{
    ComputePass, ComputePassInit, RecordedComputePass, RgBuffer, WorldAccess,
};
use crate::render::pass_builder::pass_resource_access::{RgTexture, extent_2d};
use crate::render::pass_builder::pass_resources::{PassResources, ResourceLookup, TypeId128};
use crate::render::pass_builder::rg_resources::{RgResources, TemporalBuffer, TemporalTexturePair};
use crate::render::pass_builder::shader_uri::{
    AsComputeShaderSource, ComputeShaderSource, ShaderDefine,
};
use crate::render::pass_builder::uniform_binder::UniformAllocator;
use crate::render::rg_helpers::rg_boundary::RenderGraph;
use crate::render::shader::boundary::ShaderBackend;
use crate::render::ssbo::InstanceCounted;
use crate::render::ssbo::rhapsody_boundary::BufferDesc;
use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use crate::rhapsody_contract::Extent2D;
use crate::rhapsody_types::TextureDesc;

// ---------------------------------------------------------------------------------------------
// 绑定名哈希
// ---------------------------------------------------------------------------------------------

/// `shader_pipeline::reflection::BindingNameHash::new`（O 0x140c87660）。见模块文档。
pub const fn binding_hash(name: &str) -> u64 {
    let b = name.as_bytes();
    let n = b.len();
    let has_dims = n >= 5
        && b[n - 5] == b'_'
        && b[n - 4] == b'd'
        && b[n - 3] == b'i'
        && b[n - 2] == b'm'
        && b[n - 1] == b's';
    let len = if has_dims { n - 5 } else { n };
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut i = 0;
    while i < len {
        h ^= b[i] as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
        i += 1;
    }
    if has_dims { h.wrapping_add(1) } else { h }
}

/// `movabs r8, <signed imm>` → u64（EXE 里立即数以有符号十进制 / 十六进制出现）。
pub const fn imm(v: i64) -> u64 {
    v as u64
}

/// 纹理只读（O：`mov r9b, 0x19`，全部 compute pass 的采样绑定）。
pub const TEX_READ: u8 = 0x19;
/// 纹理写（O：`mov r9b, 0x32`）。
pub const TEX_WRITE: u8 = 0x32;
/// 存储图像写（O：`mov r9b, 0x37`，SPD 的 mip 视图）。
pub const TEX_STORAGE_WRITE: u8 = 0x37;

/// 机器码里出现的公共绑定名（O：立即数 = `binding_hash(名字)`；名字取自重建 HLSL 的反射成员名，已逐个验证）。
pub mod binding {
    use super::binding_hash as h;
    pub const BOOL_RENDER_SHADOWS: u64 = h("bool_render_shadows");
    pub const SHADOW_MAP: u64 = h("shadow_map");
    pub const BLUE_NOISE_TEXTURE: u64 = h("blue_noise_texture");
    pub const DEPTH_TEX: u64 = h("depth_tex");
    pub const COLOR_TEX: u64 = h("color_tex");
    pub const CURRENT_VIEW_INDEX: u64 = h("current_view_index");
    pub const GLOBAL_DENSITY_SCALE: u64 = h("global_density_scale");
    pub const GLOBAL_SIZE_SCALE: u64 = h("global_size_scale");
    pub const HISTORY_VALID: u64 = h("history_valid");
    pub const HALF_DEPTH_TEX: u64 = h("half_depth_tex");
    pub const HALF_VIEW_NORMAL_TEX: u64 = h("half_view_normal_tex");
    pub const OUTPUT_TEX: u64 = h("output_tex");
    pub const INPUT_TEX: u64 = h("input_tex");
    pub const HISTORY_TEX: u64 = h("history_tex");
    pub const REPROJECTION_TEX: u64 = h("reprojection_tex");
    pub const SSGI_TEX: u64 = h("ssgi_tex");
    pub const GBUFFER_TEX: u64 = h("gbuffer_tex");
    pub const TERRAIN_HEIGHTMAP: u64 = h("terrain_heightmap");
    pub const VISUAL_TERRAIN_HEIGHTMAP: u64 = h("visual_terrain_heightmap");
    pub const TERRAIN_LIGHTING_TEXTURE: u64 = h("terrain_lighting_texture");
    pub const TAA_HISTORY_TEX: u64 = h("taa_history_tex");
    pub const DRAW_COMMANDS: u64 = h("draw_commands");
    pub const DRAW_INDIRECT_BUFFER_OFFSET: u64 = h("draw_indirect_buffer_offset");
    pub const OUT_BUFFER: u64 = h("out_buffer");
    pub const IS_HORIZONTAL: u64 = h("is_horizontal");
    pub const SIGMA: u64 = h("sigma");
    pub const IMG_IN: u64 = h("img_in");
    pub const IMG_OUT: u64 = h("img_out");
    pub const STROKE_INDICES: u64 = h("stroke_indices");
    pub const RESTIR_RADIANCE_TEX: u64 = h("restir_radiance_tex");
    pub const RESTIR_RAY_DIR_TEX: u64 = h("restir_ray_dir_tex");
    pub const RESTIR_RESERVOIR_TEX: u64 = h("restir_reservoir_tex");
}

/// `blue_noise` 磁盘纹理（O：`NameHash::compute("blue_noise")` = 0x579f39ab91a01865，多个 GI / 体积 pass 共用）。
/// 经 A4 真实的 `ComputeTexture for TextureFromDisk`（外部描述符路径，不登记 RG 访问）。
pub const BLUE_NOISE: TextureFromDisk = TextureFromDisk(0x579f_39ab_91a0_1865);

// ---------------------------------------------------------------------------------------------
// world 资源按原 EXE 的 128 位 TypeId 取值
// ---------------------------------------------------------------------------------------------

/// 可由 `Components::get_resource_id(lo, hi)` 取得的 world 资源（O：TypeId 为 `movabs rdx/r8` 立即数，rdx=lo、r8=hi）。
///
/// # Safety
/// 实现者保证 `TYPE_ID` 对应的资源在 world 里的内存布局就是 `Self`（原版同为裸指针强转）。
pub unsafe trait ResourceView: Sized {
    const TYPE_ID: TypeId128;
}

/// 取资源；缺失返回 `None`（调用点按原版 `Option::unwrap` 展开，见 [`unwrap_resource`]）。
pub fn get_resource<W: ResourceLookup + ?Sized, T: ResourceView>(world: &W) -> Option<&T> {
    let ptr = world.get_resource_ptr(T::TYPE_ID)?;
    // SAFETY: `ResourceView` 约定；指针来自存活的 world。
    Some(unsafe { &*(ptr as *const T) })
}

/// `get_resource_id` 之后的 `core::option::unwrap_failed`（O：每个 pass 函数都有）。
pub fn unwrap_resource<W: ResourceLookup + ?Sized, T: ResourceView>(world: &W) -> &T {
    get_resource::<W, T>(world).expect("called `Option::unwrap()` on a `None` value")
}

// ---------------------------------------------------------------------------------------------
// 帧接缝 + PassEnv
// ---------------------------------------------------------------------------------------------

/// 渲染图帧上下文（原版的 `&mut RenderContext` / `&mut RenderGraph`）：RG 资源创建（A4 [`RgResources`]）、
/// `clear_rg_buffer` 等 rg_helpers 用的 [`RenderGraph`]、以及 `PassBuilder::render` 的登记落点。
pub trait PassFrame: RgResources + RenderGraph {
    /// `PassBuilder::render`：登记一个 compute pass。
    fn record(&mut self, pass: RecordedComputePass);
}

/// 一次 pass 函数调用的全部外部输入。
pub struct PassEnv<'a> {
    pub world: &'a dyn WorldAccess,
    pub backend: &'a dyn ShaderBackend,
    pub uniforms: &'a mut dyn UniformAllocator,
    pub frame: &'a mut dyn PassFrame,
}

impl<'a> PassEnv<'a> {
    pub fn new(
        world: &'a dyn WorldAccess,
        backend: &'a dyn ShaderBackend,
        uniforms: &'a mut dyn UniformAllocator,
        frame: &'a mut dyn PassFrame,
    ) -> Self {
        PassEnv {
            world,
            backend,
            uniforms,
            frame,
        }
    }

    /// `ComputePass::new::<&str>(name, uri, world)`（O 0x1416032a0；每个 pass 函数的起点）。
    pub fn pass(&mut self, name: &'static str, uri: &str) -> ComputePass {
        self.new_pass(name, &uri)
    }

    /// `ComputePass::new::<T: AsComputeShaderSource>`（`TgsrMain` / `BinLights` / DoF 等具名来源；O 各 impl 实例）。
    pub fn new_pass<T: AsComputeShaderSource>(
        &mut self,
        name: &'static str,
        source: &T,
    ) -> ComputePass {
        ComputePass::new(
            name,
            source,
            ComputePassInit {
                world: self.world,
                backend: self.backend,
                uniforms: &mut *self.uniforms,
            },
        )
    }

    /// `Components::get_resource_id` + 取值，缺失 `Option::unwrap` panic（O）。
    pub fn resource<T: ResourceView>(&self) -> &'a T {
        let world: &'a dyn WorldAccess = self.world;
        unwrap_resource::<dyn WorldAccess, T>(world)
    }

    /// 取资源，缺失返回 `None`。
    pub fn try_resource<T: ResourceView>(&self) -> Option<&'a T> {
        let world: &'a dyn WorldAccess = self.world;
        get_resource::<dyn WorldAccess, T>(world)
    }

    /// world 里的 `PassResources`（`PassResources::new_from`，O 0x140909310；必选资源缺失 → `unwrap` panic）。
    pub fn pass_resources(&self) -> PassResources {
        PassResources::new_from(self.world)
    }

    /// 预设纹理的 `TextureDesc`（O 0x14093cf30 `get_rg_texture(preset)` 返回 `{id, desc}`；缺失 → `expect`，未导入 → `unwrap`）。
    pub fn preset_desc(&self, preset: u8) -> TextureDesc {
        self.pass_resources()
            .texture_library()
            .get_rg_texture(TexturePreset(preset))
            .desc
    }

    /// 实例缓冲 SSBO 的实例数（O：`AssetSsboLibrary::get::<T>()` → `[rax]`；未注册 `None`）。
    pub fn ssbo_instance_count<T: LibrarySsbo>(&self) -> Option<usize>
    where
        T::Storage: InstanceCounted,
    {
        let res = self.pass_resources();
        let lib: &AssetSsboLibrary = res.ssbo_library();
        lib.get::<T>().map(|s| InstanceCounted::instance_count(s))
    }

    /// 纹理库（调用方需要按名字取磁盘纹理时用）。
    pub fn with_texture_library<R>(&self, f: impl FnOnce(&AssetTextureLibrary) -> R) -> R {
        let res = self.pass_resources();
        f(res.texture_library())
    }

    // ---- RG 资源创建（`RenderGraph::create_raw_resource` / temporal，经 A4 `RgResources`）

    /// `create_raw_resource(name, TextureDesc)`（O 0x140be94d0）。
    pub fn create_texture(&mut self, name: &str, desc: TextureDesc) -> RgTexture {
        self.frame.create_raw_texture(name, &desc)
    }

    /// `create_raw_resource(name, BufferDesc::new_gpu_only(size, usage))`（c2 全部 `usage = 0x20`，c1/c3 为 0）。
    pub fn create_buffer(&mut self, name: &str, size: u32, usage: u32) -> RgBuffer {
        self.frame
            .create_raw_buffer(name, &BufferDesc::new_gpu_only(size as usize, usage))
    }

    /// `get_or_create_temporal_pair(name, desc)` + `Result::unwrap`（O：调用点一律 unwrap）。
    pub fn temporal_pair(&mut self, name: &str, desc: TextureDesc) -> TemporalTexturePair {
        self.frame
            .get_or_create_temporal_texture_pair(name, &desc)
            .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"))
    }

    /// `get_or_create_temporal(name, BufferDesc::new_gpu_only(size, usage))` + `Result::unwrap`。
    pub fn temporal_buffer(&mut self, name: &str, size: u32, usage: u32) -> TemporalBuffer {
        self.frame
            .get_or_create_temporal_buffer(name, &BufferDesc::new_gpu_only(size as usize, usage))
            .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"))
    }

    /// `PassBuilder::render`：登记 pass（c2 风格；c1 / c3 的函数改为把 `RecordedComputePass` 返回给调用方）。
    pub fn record(&mut self, pass: RecordedComputePass) {
        self.frame.record(pass);
    }
}

/// `&dyn AsComputeShaderSource` 作源（O 0x141600570：`ComputePass::new<&dyn AsComputeShaderSource>`，
/// `calc_dof` 用它在 3 个全分辨率 DoF 变体里选一个）。
pub struct DynSource<'a>(pub &'a dyn AsComputeShaderSource);

impl AsComputeShaderSource for DynSource<'_> {
    fn source_hash(&self) -> u64 {
        self.0.source_hash()
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        self.0.as_compute_pipeline()
    }
    fn pipeline_defines(&self) -> Vec<ShaderDefine> {
        self.0.pipeline_defines()
    }
}

// ---------------------------------------------------------------------------------------------
// TextureDesc 小算术（rhapsody，O）
// ---------------------------------------------------------------------------------------------

/// `TextureDesc::half_res`（O 0x140bd5730）：三个 extent 分量都取 `max(ceil(v / 2), 1)`。
pub fn half_res(mut desc: TextureDesc) -> TextureDesc {
    for v in desc.extent.iter_mut() {
        *v = (*v - *v / 2).max(1);
    }
    desc
}

/// `TextureDesc::div_up_extent([dx, dy, dz])`（O 0x140bd55e0）：逐轴 `max(ceil(v / d), 1)`；
/// 任一除数为 0 → `panic_const_div_by_zero`（"attempt to divide by zero"）。
pub fn div_up_extent(mut desc: TextureDesc, div: [u32; 3]) -> TextureDesc {
    for (v, d) in desc.extent.iter_mut().zip(div) {
        assert!(d != 0, "attempt to divide by zero");
        let q = *v / d;
        let r = u32::from(*v % d != 0);
        *v = (q + r).max(1);
    }
    desc
}

/// `(width, height)`（`dispatch_2d(extent_2d(desc))` 的实参）。
pub fn extent_xy(desc: &TextureDesc) -> (u32, u32) {
    let [w, h] = desc.extent_2d();
    (w, h)
}

/// `TextureDesc::extent_2d` 的便捷封装（调用点大量使用：dispatch 范围 = 某句柄 desc 的 extent）。
pub fn extent_of(t: &RgTexture) -> Extent2D {
    extent_2d(&t.desc)
}

/// 常用 dispatch：以 desc 的 (w,h) 做 `dispatch_2d`（O：`{mode 1, w, h, 1}` 闭包状态）。
pub fn dispatch_extent(pass: ComputePass, t: &RgTexture) -> RecordedComputePass {
    let e = extent_of(t);
    pass.dispatch_2d(e.width, e.height)
}

// ---------------------------------------------------------------------------------------------
// SSBO 载体（`AssetSsboLibrary` 的 `SsboType` 实现）
// ---------------------------------------------------------------------------------------------

/// 为 typed-gap SSBO 标记类型补 `StorageElement` + 库的 `SsboType`（`NAME` 经 xxh3_64 比对 `get_rg_buffer<T>` 的
/// `SsboId` 立即数核实，见各调用点注释）。存储形态 I：`TypedStorageBuffer<T>`（原版三选一，调用点看不出）。
macro_rules! ssbo_carrier {
    ($ty:ty, $name:literal) => {
        impl $crate::render::ssbo::StorageElement for $ty {
            const NAME: &'static str = $name;
            /// I：单个结构体缓冲（载荷布局 U；`BUFFER_LEN == 0` 的缓冲必须带初始数据，无法无数据创建）。
            const BUFFER_LEN: usize = 1;
            fn default_value() -> Option<Self> {
                Some(<$ty as Default>::default())
            }
        }
        impl $crate::asset_libraries::ssbo_library::SsboType for $ty {
            type Storage = $crate::render::ssbo::TypedStorageBuffer<$ty>;
            fn new_storage_buffer(
                device: &dyn $crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
                initial: Option<&[u8]>,
            ) -> Self::Storage {
                match initial {
                    None => $crate::render::ssbo::TypedStorageBuffer::<$ty>::new(device, None),
                    Some(bytes) => {
                        $crate::render::ssbo::TypedStorageBuffer::<$ty>::new_storage_buffer(
                            device,
                            Some(bytes),
                        )
                    }
                }
            }
        }
    };
}
pub(crate) use ssbo_carrier;

macro_rules! foreign_carrier {
    ($(#[$m:meta])* $ty:ident, $name:literal) => {
        $(#[$m])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $ty;
        ssbo_carrier!($ty, $name);
    };
}

/// `render::ssbo::impl$0::new_storage_buffer<T>` 在 `BUFFER_LEN > 0` 时的形态（O：`BushSsbo` 0x141610b60、`FlowerSsbo` 0x141613b90）：
/// 固定大小 `BUFFER_LEN × size_of::<T>()`、usage 0x22、memory 1、对齐 4；有初始数据时先断言 `len % size_of::<T>() == 0`、
/// 指针按 4 对齐，再 `len >= size + size_of::<T>()` → `panic!("Excessive data size supplied for buffer {type}")`；
/// 无初始数据（`r8 == 0`）时不上传数据。A3 的 `TypedStorageBuffer::new_storage_buffer` 只覆盖 `BUFFER_LEN == 0` 的
/// （`SheepAnimationSsbo`，memory 2、大小随数据）形态，这里经 `TypedStorageBuffer::new`（固定大小、memory 1）落地；
/// 差别：`BUFFER_LEN > 1` 且无初始数据时 `new` 上传全零，原版不上传（U：未初始化缓冲的内容由设备决定）。
pub fn fixed_len_typed_storage<T: crate::render::ssbo::StorageElement>(
    device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
    initial: Option<&[u8]>,
) -> crate::render::ssbo::TypedStorageBuffer<T> {
    use crate::render::ssbo::TypedStorageBuffer;
    let es = core::mem::size_of::<T>();
    match initial {
        None => TypedStorageBuffer::<T>::new(device, None),
        Some(bytes) => {
            assert_eq!(bytes.len() % es, 0);
            assert_eq!((bytes.as_ptr() as usize) % 4, 0);
            // SAFETY: 整除与 4 字节对齐已断言；`T: StorageElement` 是 `Copy` 的 `repr(C)` 元素（本函数只给这类类型用），
            // 字节内容按元素重解释（与原版把字节切片当元素切片同义）。
            let typed: &[T] = unsafe {
                core::slice::from_raw_parts(bytes.as_ptr() as *const T, bytes.len() / es)
            };
            TypedStorageBuffer::<T>::new(device, Some(typed))
        }
    }
}

foreign_carrier!(
    /// `resources::water::RippleSourceSsbo`（O：SsboId = xxh3("RippleSourceSsbo") = 0x28351c5dbc9c80fb；原类型非 `Copy`，
    /// 真实类型是 typed gap 且没有库侧 `SsboType`，故仍用载体）。
    RippleSourceBuf,
    "RippleSourceSsbo"
);
foreign_carrier!(
    /// `resources::rocky_terrain::RockyTerrainVertexSSBO`（O：0x1c5c4204bf6436bb = xxh3("Rocky Terrain Vertices")）。
    RockyTerrainVertexBuf,
    "Rocky Terrain Vertices"
);
foreign_carrier!(
    /// `resources::rocky_terrain::RockyTerrainTriangleSSBO`（O：0x2a397285cdc5c9db = xxh3("Rocky Terrain Triangles")）。
    RockyTerrainTriangleBuf,
    "Rocky Terrain Triangles"
);
foreign_carrier!(
    /// `systems::audio::terrain_stairs_audio::StairsCounterSsbo`（O：0x0e13781d72713b06 = xxh3("Stairs Counter")）。
    StairsCounterBuf,
    "Stairs Counter"
);

#[cfg(test)]
#[path = "pass_kit/test_env.rs"]
pub mod test_env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_hash_matches_exe_immediates() {
        // O：`movabs r8` 立即数（calculate_dof_autofocus / dof_prepare / calculate_taa / 各 c2、c3 pass）。
        assert_eq!(binding_hash("depth_tex"), 0x4861579da049fbfc);
        assert_eq!(binding_hash("color_tex"), imm(-0x266871abe31656a2));
        assert_eq!(binding_hash("point_uv"), imm(-0x192809e5814464df));
        assert_eq!(binding_hash("history_valid"), imm(-0x565ef6489f1126fe));
        assert_eq!(binding_hash("sharpen_amount"), imm(-0x56fd2af498ade2b7));
        assert_eq!(binding_hash("coc_out_tex"), 0x4ed61548046f4345);
        assert_eq!(binding_hash("terrain_heightmap"), 0x61b08b5e0470c5f2);
        assert_eq!(binding_hash("visual_terrain_heightmap"), 0x297d1e4cdae8f657);
        assert_eq!(binding_hash("grass_object_push"), 0x035280b96dab13e6);
        assert_eq!(binding_hash("water_noise_texture"), 0xfb32aff2a6359615);
        assert_eq!(binding_hash("ripple_source_buffer"), 0x68197c4966ba52c0);
    }

    /// c3 的 7 个名字与 EXE 立即数逐位吻合（apply_volumetrics / ssgi / half_res 等反编译常量）。
    #[test]
    fn binding_module_matches_exe_immediates() {
        assert_eq!(binding::BOOL_RENDER_SHADOWS, 0x418a_9844_1f59_a54c);
        assert_eq!(binding::SHADOW_MAP, 0x1165_1f4d_c984_1aa4);
        assert_eq!(binding::BLUE_NOISE_TEXTURE, 0x815b_f503_6966_978c);
        assert_eq!(binding::DEPTH_TEX, 0x4861_579d_a049_fbfc);
        assert_eq!(binding::COLOR_TEX, 0xd997_8e54_1ce9_a95e);
        assert_eq!(binding::HISTORY_VALID, 0xa9a1_09b7_60ee_d902);
        assert_eq!(binding::HALF_DEPTH_TEX, 0x2a33_3c3a_2802_8230);
        assert_eq!(binding::OUTPUT_TEX, 0x8821_9a3a_e698_92b2);
        assert_eq!(binding_hash(""), 0xcbf2_9ce4_8422_2325);
    }

    #[test]
    fn const_hash_agrees_with_fnv_crate() {
        use std::hash::Hasher;
        for name in [
            "shadow_map",
            "depth_tex",
            "history_valid",
            "$Globals",
            "ripple_source_buffer",
        ] {
            let mut h = fnv::FnvHasher::default();
            h.write(name.as_bytes());
            assert_eq!(binding_hash(name), h.finish(), "{name}");
        }
    }

    #[test]
    fn dims_suffix_is_stripped_and_adds_one() {
        // O 0x140c87660：`_dims` 后缀 → fnv(前缀) + 1（与 `with_suffix_dims` 一致）。
        assert_eq!(binding_hash("grid_dims"), 0xfb539f7243dbb832);
        assert_eq!(
            binding_hash("grid_dims"),
            binding_hash("grid").wrapping_add(1)
        );
        // 恰为 `_dims` 的边界：`_dims` → fnv("") + 1；不足 5 字节不剥。
        assert_eq!(
            binding_hash("_dims"),
            0xcbf2_9ce4_8422_2325u64.wrapping_add(1)
        );
        assert_ne!(binding_hash("dims"), binding_hash("_dims"));
    }

    #[test]
    fn blue_noise_constant_is_namehash_of_blue_noise() {
        // O：xxh3_64("blue_noise")；harness 的 xxh3 是真实 crate，这里只断言常量本身。
        assert_eq!(BLUE_NOISE.0, 0x579f_39ab_91a0_1865);
    }

    #[test]
    fn half_res_rounds_up_and_clamps_to_one() {
        let d = half_res(TextureDesc::new([5, 1, 1], 0, 0));
        assert_eq!(d.extent, [3, 1, 1]);
        let d = half_res(TextureDesc::new([0, 2, 0], 0, 0));
        assert_eq!(d.extent, [1, 1, 1]);
    }

    #[test]
    fn div_up_extent_rounds_up_and_clamps() {
        let d = div_up_extent(TextureDesc::new([17, 16, 1], 0, 0), [8, 8, 1]);
        assert_eq!(d.extent, [3, 2, 1]);
        let d = div_up_extent(TextureDesc::new([0, 0, 0], 0, 0), [8, 8, 1]);
        assert_eq!(d.extent, [1, 1, 1]);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn div_up_extent_zero_divisor_panics() {
        div_up_extent(TextureDesc::new([4, 4, 1], 0, 0), [0, 8, 1]);
    }

    #[test]
    fn new_2d_has_depth_one_and_format() {
        let d = TextureDesc::new_2d([640, 360], 0x4c);
        assert_eq!((d.extent, d.format), ([640, 360, 1], 0x4c));
        assert_eq!(d.with_format(0x53).format, 0x53);
        assert_eq!(extent_xy(&d), (640, 360));
    }

    #[test]
    fn ssbo_carriers_use_verified_names() {
        use crate::render::ssbo::StorageElement;
        assert_eq!(RippleSourceBuf::NAME, "RippleSourceSsbo");
        assert_eq!(RockyTerrainVertexBuf::NAME, "Rocky Terrain Vertices");
        assert_eq!(StairsCounterBuf::NAME, "Stairs Counter");
    }
}
