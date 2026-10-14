//! `pass_kit/test_env.rs` — 所有 compute pass 单元测试共用的**唯一**测试环境（仅 `cfg(test)`；合并 c1 `pass_kit::test_env`、
//! c2 `c2_testenv`、c3 `pass_env::test_support`，复刻 A4 `compute_pass_builder` 测试里的 Env）。
//!
//! 提供：
//! * [`Reflection`]：着色器反射构造器（可按需声明绑定名哈希 / uniform 字段 / `_dims` 维度 uniform）。
//!   [`Reflection::for_names`] 是 c3 约定（绑定从 3 起编号、每个 uniform 占 16 B 槽、每个绑定另带 `hash+1` 维度 uniform），
//!   `new().bind().uniforms().dims()` 是 c2 约定（binding 号 = 0x10+序号、uniform 偏移 = 序号×16）。
//! * [`TestWorld`]：持有各库的 world（按 TypeId 128 位查表）；[`TestFrame`]：记录型帧（实现 `PassFrame`，RG 资源创建全部记录）。
//! * [`Env`]：`world + frame + 偏移表` 的打包，`Env::run(|env| …)` 构造 `PassEnv`；松散的 [`with_env`] 供只要 world/frame 的测试。
//! * 断言小工具：[`threads`]、[`accesses`]、[`rgtex`]、[`uniform_u32`]；SSBO 替身 `test_carrier!`。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::field_reassign_with_default)]
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::sync::Arc;

use crate::asset_libraries::color_library::ColorLibrarySsbo;
use crate::asset_libraries::shader_library::AssetShaderLibrary;
use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType as LibrarySsbo};
use crate::asset_libraries::texture_library::{
    AssetTextureLibrary, ImportedTexture, TextureRenderGraph,
};
use crate::render::load_texture::boundary::TextureDevice as PresetTextureDevice;
use crate::render::pass_builder::bind_resources::*;
use crate::render::pass_builder::compute_pass_builder::{
    ComputePassDevice, DispatchMode, RecordedComputePass, ResourceRegistry, RgBuffer, WorldAccess,
    with_suffix_dims,
};
use crate::render::pass_builder::pass_resource_access::{RgTexture, TextureDevice};
use crate::render::pass_builder::pass_resources::{ResourceLookup, TypeId128, type_ids};
use crate::render::pass_builder::rg_resources::{
    RgError, RgResources, TemporalBuffer, TemporalTexture, TemporalTexturePair,
};
use crate::render::pass_builder::shader_uri::ComputePipelineDesc;
use crate::render::pass_builder::uniform_binder::{
    UniformAllocator, UniformBlockInfo, UniformReflection,
};
use crate::render::rg_helpers::rg_boundary::{RenderFn, RenderGraph, StagingAlloc};
use crate::render::shader::ComputeShaderProgram;
use crate::render::shader::boundary::{
    CompiledShaderPipeline, ComputePipeline, RasterPipeline, RasterPipelineDesc, RasterPipelineKey,
    ShaderBackend,
};
use crate::render::ssbo::rhapsody_boundary::BufferDesc;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_contract::{GraphicsDevice, ManagedHandle};
use crate::rhapsody_types::{BackendError, Error, Texture, TextureDesc};
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, PassFrame, ResourceView};

/// 着色器反射构造器：声明哪些绑定名哈希存在（set 0 / 给定 binding）、哪些 uniform 字段（给定偏移）。
pub struct Reflection {
    pub registry: ResourceRegistry,
}

impl Default for Reflection {
    fn default() -> Self {
        Self::new()
    }
}

impl Reflection {
    pub fn new() -> Self {
        let mut registry = ResourceRegistry::default();
        registry.uniform_block = Some(UniformBlockInfo {
            size: 256,
            set_index: 1,
        });
        registry.uniform = UniformReflection {
            valid: true,
            fields: HashMap::new(),
        };
        Reflection { registry }
    }
    /// 声明绑定名（binding 号 = 0x10 + 声明顺序）。
    pub fn bind(mut self, hashes: &[u64]) -> Self {
        for (i, h) in hashes.iter().enumerate() {
            self.registry.bindings.insert(*h, (0, 0x10 + i as u32));
        }
        self
    }
    /// 声明 uniform 字段（偏移 = 声明顺序 × 16）。
    pub fn uniforms(mut self, keys: &[u64]) -> Self {
        for (i, k) in keys.iter().enumerate() {
            self.registry.uniform.fields.insert(*k, (i as u32) * 16);
        }
        self
    }
    /// 声明 `with_suffix_dims` 的维度 uniform（键 = 名 + 1）。
    pub fn dims(mut self, names: &[u64], base: u32) -> Self {
        for (i, n) in names.iter().enumerate() {
            self.registry
                .uniform
                .fields
                .insert(with_suffix_dims(*n), base + (i as u32) * 8);
        }
        self
    }
    /// 没有 uniform 块的着色器（`UniformBinder::new` → `None`）。
    pub fn without_uniform_block(mut self) -> Self {
        self.registry.uniform_block = None;
        self
    }

    /// c3 约定：绑定从 3 起编号（0..2 留给颜色库等）；每个 uniform 占 16 B 槽；每个绑定另带 `hash+1` 维度 uniform。
    /// 返回反射与 `uniform 键 → 偏移` 表。
    pub fn for_names(bindings: &[u64], uniforms: &[u64]) -> (Reflection, HashMap<u64, usize>) {
        let mut r = ResourceRegistry::default();
        r.bindings.insert(COLOR_LIBRARY_BINDING_HASH, (0, 2));
        for (i, h) in bindings.iter().enumerate() {
            r.bindings.insert(*h, (0, 3 + i as u32));
        }
        let mut offsets = HashMap::new();
        let mut off = 0usize;
        r.uniform = UniformReflection {
            valid: true,
            fields: HashMap::new(),
        };
        for h in uniforms
            .iter()
            .copied()
            .chain(bindings.iter().map(|h| with_suffix_dims(*h)))
        {
            offsets.insert(h, off);
            r.uniform.fields.insert(h, off as u32);
            off += 16;
        }
        r.uniform_block = Some(UniformBlockInfo {
            size: off.max(16) as u32,
            set_index: 1,
        });
        (Reflection { registry: r }, offsets)
    }
}

pub struct FakeDevice {
    registry: Arc<ResourceRegistry>,
}
impl TextureDevice for FakeDevice {
    fn get_vk_image_view(&self, h: &ManagedHandle) -> Result<u64, BackendError> {
        Ok(h.0 + 1)
    }
    fn managed_texture_desc(&self, _: &ManagedHandle) -> TextureDesc {
        TextureDesc::new([32, 16, 1], 0, 0)
    }
}
impl ComputePassDevice for FakeDevice {
    fn get_vk_buffer(&self, h: &ManagedHandle) -> Option<u64> {
        Some(h.0)
    }
    fn pipeline_registry(&self, _p: &ComputeShaderProgram) -> Arc<ResourceRegistry> {
        Arc::clone(&self.registry)
    }
}

/// 计数的着色器后端：记录每次 `load_compiled_compute` 的 `path:entry`。
#[derive(Default)]
pub struct Backend {
    pub loads: Cell<u32>,
    pub paths: RefCell<Vec<String>>,
}
impl ShaderBackend for Backend {
    fn load_compiled_compute(
        &self,
        d: &ComputePipelineDesc,
    ) -> Result<CompiledShaderPipeline, Error> {
        self.loads.set(self.loads.get() + 1);
        self.paths
            .borrow_mut()
            .push(format!("{}:{}", d.path, d.entry));
        Ok(CompiledShaderPipeline(vec![]))
    }
    fn load_compiled_raster(
        &self,
        _: &RasterPipelineDesc,
    ) -> Result<CompiledShaderPipeline, Error> {
        unreachable!()
    }
    fn raster_shader_variants(&self, _: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
        unreachable!()
    }
    fn create_compute_pipeline(&self, _: &str, _: Arc<CompiledShaderPipeline>) -> ComputePipeline {
        ComputePipeline { id: 1 }
    }
    fn create_raster_pipeline(
        &self,
        _: Arc<CompiledShaderPipeline>,
        _: &RasterPipelineKey,
    ) -> Result<RasterPipeline, Error> {
        unreachable!()
    }
}

#[derive(Default)]
pub struct Pool(pub Vec<u32>);
impl UniformAllocator for Pool {
    fn allocate_uniform(&mut self, size: u32) -> Vec<u8> {
        self.0.push(size);
        vec![0; size as usize]
    }
}

/// 预设纹理创建设备：保留 desc 的格式（契约设备恒写 `format: 0`），并按纹理 id 记下完整 desc，供 [`PresetRg`] 回填
/// `ImportedTexture.desc`（真实 `Texture+0x18` 是整块 `TextureDesc`，占位 `Texture` 装不下）。
struct PresetDevice {
    descs: RefCell<HashMap<u64, TextureDesc>>,
    next: Cell<u64>,
}
impl PresetTextureDevice for PresetDevice {
    fn create_texture(
        &self,
        _name: &str,
        desc: &TextureDesc,
        _data: &[u8],
    ) -> Result<Texture, BackendError> {
        let id = 1000 + self.next.get();
        self.next.set(self.next.get() + 1);
        self.descs.borrow_mut().insert(id, *desc);
        Ok(Texture {
            id,
            width: desc.extent[0],
            height: desc.extent[1],
            format: desc.format,
        })
    }
}
/// 预设导入 RG：沿用占位 `RenderGraph` 的导入记录，把 `desc` 换成创建时的整块描述。
struct PresetRg<'a> {
    descs: &'a HashMap<u64, TextureDesc>,
    inner: crate::rhapsody_types::RenderGraph,
}
impl TextureRenderGraph for PresetRg<'_> {
    fn import_unowned_texture(&mut self, texture: &Texture, access: u8) -> ImportedTexture {
        let mut t = self.inner.import_unowned_texture(texture, access);
        t.desc = self.descs[&texture.id];
        t
    }
    fn export_texture(&mut self, handle: ImportedTexture, access: u8) {
        self.inner.export_texture(handle, access);
    }
}

/// 预设纹理缺省 desc（`create_preset_textures` 的闭包返回值；`preset_desc` 读回）。
pub const DEFAULT_PRESET_DESC: TextureDesc = TextureDesc {
    extent: [8, 4, 1],
    arg3: 0x25,
    arg4: 0,
    sampled: false,
    mip_levels: 4,
    array_elements: 0,
    storage: false,
    color_attachment: false,
    depth_stencil_attachment: false,
    format: 0x7a,
};

/// 持有各库与自定义资源的 world。
pub struct TestWorld {
    _shaders: Box<AssetShaderLibrary>,
    pub textures: Box<AssetTextureLibrary>,
    pub ssbos: Box<AssetSsboLibrary>,
    map: HashMap<TypeId128, *const ()>,
    device: Arc<FakeDevice>,
    pub backend: Backend,
    pub pool: RefCell<Pool>,
}
impl ResourceLookup for TestWorld {
    fn get_resource_ptr(&self, id: TypeId128) -> Option<*const ()> {
        self.map.get(&id).copied()
    }
}
impl WorldAccess for TestWorld {
    fn graphics_device(&self) -> Option<Arc<dyn ComputePassDevice>> {
        Some(self.device.clone() as Arc<dyn ComputePassDevice>)
    }
}

impl TestWorld {
    pub fn new(reflection: Reflection) -> Self {
        Self::with_presets(reflection, |_| DEFAULT_PRESET_DESC)
    }

    /// 同 [`TestWorld::new`]，但每个预设纹理的 desc 由 `preset_desc` 给出（`upload_world_raster` 等按格式分支）。
    pub fn with_presets(
        reflection: Reflection,
        preset_desc: impl Fn(TexturePreset) -> TextureDesc,
    ) -> Self {
        let dev = GraphicsDevice(1);
        let shaders = Box::new(AssetShaderLibrary::new());
        let mut textures = Box::new(AssetTextureLibrary::new(&dev));
        let ssbos = Box::new(AssetSsboLibrary::new());
        ssbos.add_new::<ColorLibrarySsbo>(&dev);
        ssbos.add_new::<RtTrianglesSsbo>(&dev);
        ssbos.add_new::<RtTriangleMetaSsbo>(&dev);
        ssbos.add_new::<RtBvhSsbo>(&dev);
        ssbos.add_new::<WallColorIdSsbo>(&dev);
        let preset_dev = PresetDevice {
            descs: RefCell::new(HashMap::new()),
            next: Cell::new(0),
        };
        textures.create_preset_textures_with(&preset_dev, preset_desc);
        let descs = preset_dev.descs.into_inner();
        textures.import_texture_presets_to_rg(&mut PresetRg {
            descs: &descs,
            inner: crate::rhapsody_types::RenderGraph::default(),
        });
        // 磁盘纹理 `blue_noise`（`BLUE_NOISE` 的库键）。
        textures.add(
            Some("blue_noise".to_string()),
            Texture {
                id: 500,
                width: 64,
                height: 64,
                format: 0,
            },
        );
        let mut map: HashMap<TypeId128, *const ()> = HashMap::new();
        let dummy: &'static u8 = Box::leak(Box::new(0u8));
        for id in [
            type_ids::RESOURCE_E,
            type_ids::RESOURCE_G,
            type_ids::DYNAMIC_CONSTANTS,
        ] {
            map.insert(id, dummy as *const u8 as *const ());
        }
        map.insert(type_ids::SHADER_LIBRARY, &*shaders as *const _ as *const ());
        map.insert(
            type_ids::TEXTURE_LIBRARY,
            &*textures as *const _ as *const (),
        );
        map.insert(type_ids::SSBO_LIBRARY, &*ssbos as *const _ as *const ());
        TestWorld {
            _shaders: shaders,
            textures,
            ssbos,
            map,
            device: Arc::new(FakeDevice {
                registry: Arc::new(reflection.registry),
            }),
            backend: Backend::default(),
            pool: RefCell::new(Pool::default()),
        }
    }

    /// 注册一个 SSBO 到库里（`AssetSsboLibrary::add_new<T>`）。
    pub fn with_ssbo<T: LibrarySsbo>(self) -> Self {
        self.ssbos.add_new::<T>(&GraphicsDevice(1));
        self
    }

    /// 放入一个按 128 位 TypeId 查找的资源（泄漏以保证指针存活）。
    pub fn with_resource<T: ResourceView + 'static>(mut self, value: T) -> Self {
        let leaked: &'static T = Box::leak(Box::new(value));
        self.map.insert(T::TYPE_ID, leaked as *const T as *const ());
        self
    }
}

/// 记录型帧：实现 `PassFrame`（`RenderGraph::add_pass` 记录名字，compute pass 原样保存，RG 资源创建全部记录）。
/// 资源号 = `100 + 创建序号`（纹理 / 缓冲 / temporal 共用一个计数器）。
#[derive(Default)]
pub struct TestFrame {
    /// `PassFrame::record` 登记的 compute pass（按登记顺序）。
    pub passes: Vec<RecordedComputePass>,
    /// 全部登记事件的名字（`record` 与 `add_pass` 混排，按发生顺序）。
    pub order: Vec<String>,
    /// `RenderGraph::add_pass`：`(名字, 资源号, 访问码)`。
    pub rg_passes: Vec<(&'static str, u32, u8)>,
    /// `create_raw_texture`：`(名字, desc, 返回句柄)`。
    pub textures: Vec<(String, TextureDesc, RgTexture)>,
    /// `create_raw_buffer`：`(名字, desc, 返回句柄)`。
    pub buffers: Vec<(String, BufferDesc, RgBuffer)>,
    /// `get_or_create_temporal(name, TextureDesc)` 请求。
    pub temporal_textures: Vec<(String, TextureDesc)>,
    /// `get_or_create_temporal_pair` 请求。
    pub temporal_pairs: Vec<(String, TextureDesc)>,
    /// `get_or_create_temporal(name, BufferDesc)` 请求：`(名字, desc, 返回句柄)`。
    pub temporal_buffers: Vec<(String, BufferDesc, RgBuffer)>,
    /// 置位后所有 temporal 调用返回 `Err`（调用点 `Result::unwrap` panic）。
    pub fail_temporal: bool,
    /// `get_or_create_temporal_pair` / 单张 temporal 结果里的标志字节（A4 命名 `fresh`）。
    pub pair_fresh: bool,
    pub next: u32,
}

impl TestFrame {
    pub fn fresh(&mut self) -> u32 {
        self.next += 1;
        100 + self.next
    }
    /// `PassEnv::record` 的便捷读取：按名字找已登记的 pass。
    pub fn pass(&self, name: &str) -> Option<&RecordedComputePass> {
        self.passes.iter().find(|p| p.name == name)
    }
}

impl RenderGraph for TestFrame {
    fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, _render: RenderFn) {
        self.order.push(name.to_string());
        self.rg_passes.push((name, resource, access));
    }
    fn allocate_dynamic(&mut self, size: usize) -> StagingAlloc {
        StagingAlloc {
            buffer: 0,
            offset: 0,
            bytes: vec![0; size],
        }
    }
}

impl RgResources for TestFrame {
    fn create_raw_texture(&mut self, name: &str, desc: &TextureDesc) -> RgTexture {
        let t = RgTexture {
            resource: self.fresh(),
            desc: *desc,
        };
        self.textures.push((name.to_string(), *desc, t));
        t
    }
    fn create_raw_buffer(&mut self, name: &str, desc: &BufferDesc) -> RgBuffer {
        let b = RgBuffer {
            resource: self.fresh(),
        };
        self.buffers.push((name.to_string(), *desc, b));
        b
    }
    fn get_or_create_temporal_texture(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexture, RgError> {
        self.temporal_textures.push((name.to_string(), *desc));
        if self.fail_temporal {
            return Err(RgError("temporal failed".to_string()));
        }
        Ok(TemporalTexture {
            texture: RgTexture {
                resource: self.fresh(),
                desc: *desc,
            },
            flag: self.pair_fresh as u8,
        })
    }
    fn get_or_create_temporal_texture_pair(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexturePair, RgError> {
        self.temporal_pairs.push((name.to_string(), *desc));
        if self.fail_temporal {
            return Err(RgError("temporal failed".to_string()));
        }
        let current = RgTexture {
            resource: self.fresh(),
            desc: *desc,
        };
        let previous = RgTexture {
            resource: self.fresh(),
            desc: *desc,
        };
        Ok(TemporalTexturePair {
            current,
            previous,
            fresh: self.pair_fresh,
        })
    }
    fn get_or_create_temporal_buffer(
        &mut self,
        name: &str,
        desc: &BufferDesc,
    ) -> Result<TemporalBuffer, RgError> {
        if self.fail_temporal {
            self.temporal_buffers
                .push((name.to_string(), *desc, RgBuffer { resource: 0 }));
            return Err(RgError("temporal failed".to_string()));
        }
        let b = RgBuffer {
            resource: self.fresh(),
        };
        self.temporal_buffers.push((name.to_string(), *desc, b));
        Ok(TemporalBuffer {
            buffer: b,
            flag: self.pair_fresh as u8,
        })
    }
}

impl PassFrame for TestFrame {
    fn record(&mut self, pass: RecordedComputePass) {
        self.order.push(pass.name.to_string());
        self.passes.push(pass);
    }
}

/// 读 uniform 块里 `offset` 处的 u32/i32/f32 位（c2 约定：偏移 = 声明序号 × 16）。
pub fn uniform_u32(pass: &RecordedComputePass, offset: usize) -> u32 {
    let d = pass
        .uniform_binder
        .as_ref()
        .expect("no uniform block")
        .data();
    u32::from_le_bytes(d[offset..offset + 4].try_into().unwrap())
}

/// 用 `world` 与 `frame` 跑一个 pass 函数。
pub fn with_env<R>(
    world: &TestWorld,
    frame: &mut TestFrame,
    f: impl FnOnce(&mut PassEnv<'_>) -> R,
) -> R {
    let mut pool = world.pool.borrow_mut();
    let mut env = PassEnv {
        world,
        backend: &world.backend,
        uniforms: &mut *pool,
        frame,
    };
    f(&mut env)
}

/// `world + frame + uniform 偏移表` 的打包（c3 风格的测试入口）。
pub struct Env {
    pub world: TestWorld,
    pub frame: TestFrame,
    pub uniform_offsets: HashMap<u64, usize>,
}

/// `bindings`/`uniforms` 为该 pass 反射里存在的名字哈希。
pub fn env(bindings: &[u64], uniforms: &[u64]) -> Env {
    env_with(bindings, uniforms, |_, _| {})
}

/// 同 [`env`]；`reg` 在 world 构造后注册额外 SSBO。
pub fn env_with(
    bindings: &[u64],
    uniforms: &[u64],
    reg: impl FnOnce(&AssetSsboLibrary, &GraphicsDevice),
) -> Env {
    let (reflection, uniform_offsets) = Reflection::for_names(bindings, uniforms);
    let world = TestWorld::new(reflection);
    reg(&world.ssbos, &GraphicsDevice(1));
    Env {
        world,
        frame: TestFrame::default(),
        uniform_offsets,
    }
}

impl Env {
    /// 由已有反射构造（c2 约定；偏移表为空，读 uniform 用 [`uniform_u32`]）。
    pub fn from_reflection(reflection: Reflection) -> Env {
        Env {
            world: TestWorld::new(reflection),
            frame: TestFrame::default(),
            uniform_offsets: HashMap::new(),
        }
    }

    /// 打包成 `PassEnv` 并运行 `f`。
    pub fn run<R>(&mut self, f: impl FnOnce(&mut PassEnv<'_>) -> R) -> R {
        let Env { world, frame, .. } = self;
        with_env(world, frame, f)
    }

    /// 后端记录的 `path:entry`（按加载顺序）。
    pub fn shader_paths(&self) -> Vec<String> {
        self.world.backend.paths.borrow().clone()
    }

    /// 读回某个 uniform 槽的前 `n` 字节（c3 约定）。
    pub fn uniform(&self, pass: &RecordedComputePass, key: u64, n: usize) -> Vec<u8> {
        let off = self.uniform_offsets[&key];
        pass.uniform_binder.as_ref().expect("uniform block").data()[off..off + n].to_vec()
    }
    pub fn uniform_u32(&self, pass: &RecordedComputePass, key: u64) -> u32 {
        u32::from_le_bytes(self.uniform(pass, key, 4).try_into().unwrap())
    }
    pub fn uniform_f32(&self, pass: &RecordedComputePass, key: u64) -> f32 {
        f32::from_bits(self.uniform_u32(pass, key))
    }
}

/// 外部 RG 句柄（调用方给的 `Handle<Texture>`）。
pub fn rgtex(resource: u32, w: u32, h: u32) -> RgTexture {
    RgTexture {
        resource,
        desc: TextureDesc::new([w, h, 1], 0, 0),
    }
}

/// `dispatch_1d/2d`（模式 1）的线程总数；其它模式 → panic。
pub fn threads(p: &RecordedComputePass) -> (u32, u32, u32) {
    match p.dispatch {
        DispatchMode::Threads { x, y, z } => (x, y, z),
        _ => panic!("expected a thread-count dispatch"),
    }
}

/// 记录里每条资源访问 `(资源号, 访问码, 同步组)`，跳过 `ComputePass::new` 里自动加的颜色库访问。
pub fn accesses(p: &RecordedComputePass) -> Vec<(u32, u8, u32)> {
    p.builder
        .accesses
        .iter()
        .skip(1)
        .map(|a| (a.resource, a.access, a.group))
        .collect()
}

/// 测试用 SSBO 载体：给“别的模块里的类型”（泛型参数）一个可注册的替身。
macro_rules! test_carrier {
    ($name:ident, $short:literal) => {
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name;
        impl crate::render::ssbo::StorageElement for $name {
            const NAME: &'static str = $short;
            const BUFFER_LEN: usize = 1;
            fn default_value() -> Option<Self> {
                Some($name)
            }
        }
        impl crate::asset_libraries::ssbo_library::SsboType for $name {
            type Storage = crate::render::ssbo::TypedStorageBuffer<$name>;
            fn new_storage_buffer(
                device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
                initial: Option<&[u8]>,
            ) -> Self::Storage {
                match initial {
                    None => crate::render::ssbo::TypedStorageBuffer::<$name>::new(device, None),
                    Some(b) => {
                        crate::render::ssbo::TypedStorageBuffer::<$name>::new_storage_buffer(
                            device,
                            Some(b),
                        )
                    }
                }
            }
        }
    };
}
pub(crate) use test_carrier;

test_carrier!(SsboA, "SsboA");
test_carrier!(SsboB, "SsboB");
test_carrier!(SsboC, "SsboC");

/// 断言辅助：`create_raw_texture` 记录 → `(名字, [w,h], format)`。
pub fn tex_key(t: &(String, TextureDesc, RgTexture)) -> (&str, [u32; 2], u32) {
    (t.0.as_str(), t.1.extent_2d(), t.1.format)
}

/// 断言辅助：temporal 请求记录 → `(名字, [w,h], format)`。
pub fn temporal_key(t: &(String, TextureDesc)) -> (&str, [u32; 2], u32) {
    (t.0.as_str(), t.1.extent_2d(), t.1.format)
}
