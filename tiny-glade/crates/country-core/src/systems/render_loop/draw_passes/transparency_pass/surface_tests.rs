//! 水 / 冰表面 pass 的测试：真实实现（RasterPass + ComputePass + RG 创建）产生的调用序列 == 黄金表 `WATER_OPS` / `ICE_OPS`，
//! 并核对每一步的资源名 / 格式 / 尺寸公式 / 访问码 / dispatch 方式。
//!
//! 环境：栅格侧用 A4 的 `MockHost`（`draw_stream::test_host`），计算侧用本文件里复刻自 `compute_pass_builder` 测试的假设备 + 反射。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::field_reassign_with_default)]
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::cell::{Cell, RefCell};
use core::time::Duration;
use std::collections::HashMap;

use bevy_ecs::world::World;

use crate::asset_libraries::color_library::ColorLibrarySsbo;
use crate::asset_libraries::shader_library::AssetShaderLibrary;
use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::asset_libraries::texture_library::AssetTextureLibrary;
use crate::name_hash::NameHash;
use crate::render::pass_builder::bind_resources::*;
use crate::render::pass_builder::compute_pass_builder::{
    ComputePass, ComputePassDevice, ComputePassInit, DispatchMode, RecordedComputePass,
    ResourceRegistry, WorldAccess, with_suffix_dims,
};
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::pass_builder::draw_stream::test_host::{Log, MockHost};
use crate::render::pass_builder::draw_stream::{PassHost, ShaderReflection};
use crate::render::pass_builder::pass_resource_access::{RgTexture, SsboType as _, TextureDevice};
use crate::render::pass_builder::pass_resources::{ResourceLookup, TypeId128, type_ids};
use crate::render::pass_builder::rg_resources::{RecordingRg, RgKind};
use crate::render::pass_builder::shader_uri::ComputePipelineDesc;
use crate::render::pass_builder::uniform_binder::{
    UniformAllocator, UniformBlockInfo, UniformReflection,
};
use crate::render::rg_helpers::rg_boundary::{RenderFn, RenderGraph, StagingAlloc};
use crate::render::shader::ComputeShaderProgram;
use crate::render::shader::RasterShaderProgram;
use crate::render::shader::boundary::{
    CompiledShaderPipeline, ComputePipeline, RasterPipeline, RasterPipelineDesc, RasterPipelineKey,
    ShaderBackend,
};
use crate::rhapsody_contract::{GraphicsDevice, Handle, ManagedHandle};
use crate::rhapsody_types::{BackendError, Error, Texture, TextureDesc};
use crate::systems::render_loop::draw_passes::d_main_support::{DrawPassCtx, ShaderLookup};
use crate::systems::render_loop::draw_passes::draw_lists::{DrawItem, DrawLists, EntityDrawParams};

use super::surface_support::{
    BUFFER_USAGE, PRESET_SHADOW_MAP, SurfaceCompute, SurfaceEnv, SurfaceInputs, att, half_res,
    names, ray_units,
};
use super::{
    ICE_COMPUTE_PASSES, ICE_OPS, ICE_RESOURCE_NAMES, PlanKind, SURFACE_LIST, WATER_OPS,
    WATER_RAY_BUFFER_NAME, ice_surface_pass, water_ray_buffer_size, water_surface_pass,
};

// ------------------------------------------------------------------------------------------------ 计算环境

const HISTORY_VALID_OFFSET: usize = 16;

fn registry() -> ResourceRegistry {
    use names::*;
    let mut r = ResourceRegistry::default();
    let all = [
        HISTORY_VALID,
        HALF_DEPTH_TEX,
        TAA_HISTORY_TEX,
        REPROJECTION_TEX,
        REFRACTION_TEX,
        BLUE_NOISE_TEXTURE,
        TERRAIN_HEIGHTMAP,
        VISUAL_TERRAIN_HEIGHTMAP,
        TERRAIN_LIGHTING_TEXTURE,
        COLOR_TEX,
        WATER_RT_COUNT_BUFFER,
        WATER_RT_RAY_BUFFER,
        ICE_REFL_COUNT_BUFFER,
        ICE_REFL_PX_BUFFER,
        ICE_RT_COUNT_BUFFER,
        ICE_RT_RAY_BUFFER,
        REFL_HIT_TEX,
        REFL_HIT_RADIANCE_TEX,
        ICE_ROUGHNESS_TEX,
        REFR_HIT_TEX,
        REFR_HIT_RADIANCE_TEX,
        DEPTH_TEX,
        REFL_REFR_RADIANCE_OUT_TEX,
        BRDF_FG_LUT,
        REFL_REFR_RADIANCE_HISTORY_TEX,
        RAY_LEN_HISTORY_TEX,
        RAY_LEN_OUT_TEX,
    ];
    for (i, h) in all.iter().enumerate() {
        r.bindings.insert(*h, (0, 32 + i as u32));
    }
    for (h, b) in [
        (COLOR_LIBRARY_BINDING_HASH, 9),
        (RT_TRIANGLES_BINDING_HASH, 10),
        (RT_TRIANGLE_META_BINDING_HASH, 11),
        (RT_BVH_BINDING_HASH, 12),
        (WALL_COLOR_ID_BINDING_HASH, 13),
    ] {
        r.bindings.insert(h, (0, b));
    }
    r.uniform_block = Some(UniformBlockInfo {
        size: 64,
        set_index: 1,
    });
    r.uniform = UniformReflection {
        valid: true,
        fields: HashMap::new(),
    };
    r.uniform
        .fields
        .insert(HISTORY_VALID, HISTORY_VALID_OFFSET as u32);
    r.uniform.fields.insert(RT_TRIANGLES_UNIFORM_HASH, 24);
    r.uniform.fields.insert(RT_BVH_UNIFORM_HASH, 28);
    for h in all {
        r.uniform.fields.insert(with_suffix_dims(h), 32);
    }
    r
}

struct FakeDevice {
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

#[derive(Default)]
struct Backend {
    paths: RefCell<Vec<String>>,
    loads: Cell<u32>,
}
impl ShaderBackend for Backend {
    fn load_compiled_compute(
        &self,
        d: &ComputePipelineDesc,
    ) -> Result<CompiledShaderPipeline, Error> {
        self.loads.set(self.loads.get() + 1);
        self.paths.borrow_mut().push(d.path.clone());
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

struct Pool;
impl UniformAllocator for Pool {
    fn allocate_uniform(&mut self, size: u32) -> Vec<u8> {
        vec![0; size as usize]
    }
}

struct CEnv {
    _shaders: Box<AssetShaderLibrary>,
    _textures: Box<AssetTextureLibrary>,
    _ssbos: Box<AssetSsboLibrary>,
    map: HashMap<TypeId128, *const ()>,
    device: Arc<FakeDevice>,
    backend: Backend,
}
impl ResourceLookup for CEnv {
    fn get_resource_ptr(&self, id: TypeId128) -> Option<*const ()> {
        self.map.get(&id).copied()
    }
}
impl WorldAccess for CEnv {
    fn graphics_device(&self) -> Option<Arc<dyn ComputePassDevice>> {
        Some(self.device.clone() as Arc<dyn ComputePassDevice>)
    }
}

fn cenv() -> CEnv {
    let dev = GraphicsDevice(1);
    let shaders = Box::new(AssetShaderLibrary::new());
    let mut textures = Box::new(AssetTextureLibrary::new(&dev));
    let ssbos = Box::new(AssetSsboLibrary::new());
    ssbos.add_new::<ColorLibrarySsbo>(&dev);
    ssbos.add_new::<RtTrianglesSsbo>(&dev);
    ssbos.add_new::<RtTriangleMetaSsbo>(&dev);
    ssbos.add_new::<RtBvhSsbo>(&dev);
    ssbos.add_new::<WallColorIdSsbo>(&dev);
    textures.create_preset_textures_with(&dev, |_| TextureDesc::new([8, 4, 1], 0x25, 0));
    textures.import_texture_presets_to_rg(&mut crate::rhapsody_types::RenderGraph::default());
    // blue noise：`0x579f39ab91a01865 = xxh3_64("blue_noise")`（B_c3.md）。
    textures.add(
        Some("blue_noise".to_owned()),
        Texture {
            id: 900,
            width: 128,
            height: 128,
            format: 0x25,
        },
    );
    assert_eq!(NameHash::compute("blue_noise").0, names::BLUE_NOISE_FILE);
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
    CEnv {
        _shaders: shaders,
        _textures: textures,
        _ssbos: ssbos,
        map,
        device: Arc::new(FakeDevice {
            registry: Arc::new(registry()),
        }),
        backend: Backend::default(),
    }
}

struct Seam<'a> {
    env: &'a CEnv,
    recorded: Vec<RecordedComputePass>,
}
impl SurfaceCompute for Seam<'_> {
    fn new_pass(&mut self, name: &'static str, uri: &'static str) -> ComputePass {
        ComputePass::new(
            name,
            &uri,
            ComputePassInit {
                world: self.env,
                backend: &self.env.backend,
                uniforms: &mut Pool,
            },
        )
    }
    fn record(&mut self, pass: RecordedComputePass) {
        self.recorded.push(pass);
    }
}

#[derive(Default)]
struct Graph {
    passes: Vec<(&'static str, u32, u8)>,
}
impl RenderGraph for Graph {
    fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, _render: RenderFn) {
        self.passes.push((name, resource, access));
    }
    fn allocate_dynamic(&mut self, size: usize) -> StagingAlloc {
        StagingAlloc {
            buffer: 0,
            offset: 0,
            bytes: vec![0; size],
        }
    }
}

// ------------------------------------------------------------------------------------------------ 栅格环境

fn mock_host(log: &Rc<RefCell<Log>>) -> Box<dyn PassHost> {
    let (mut host, _) = MockHost::new();
    host.log = log.clone();
    let mut refl = ShaderReflection::default();
    refl.program_name = "surface_test";
    let hashes = [
        names::HALF_DEPTH_TEX,
        names::TAA_HISTORY_TEX,
        names::REPROJECTION_TEX,
        names::REFRACTION_TEX,
        names::SHADOWS_HISTORY_TEX,
        names::SHADOW_MAP,
        names::WATER_RT_COUNT_BUFFER,
        names::WATER_RT_RAY_BUFFER,
        names::ICE_REFL_COUNT_BUFFER,
        names::ICE_REFL_PX_BUFFER,
    ];
    for (i, h) in hashes.iter().enumerate() {
        refl.bindings.insert(*h, (0, 20 + i as u32));
    }
    refl.has_uniform_block = true;
    refl.uniform_offsets.insert(names::HISTORY_VALID, 16);
    refl.uniform_offsets.insert(names::BOOL_RENDER_SHADOWS, 20);
    refl.uniform_block_size = 64;
    host.shaders.insert(1, Arc::new(refl));
    host.rg_buffers
        .insert(ColorLibrarySsbo::rg_buffer_hash(), 1);
    let t = |id| crate::render::pass_builder::framebuffer_binding::AttachmentTexture {
        id,
        extent: (4, 4),
        format: 44,
        sample_count: 1,
    };
    host.presets.insert(0x11, t(900));
    host.presets.insert(0x12, t(901));
    host.presets.insert(PRESET_SHADOW_MAP, t(900));
    host.boxed()
}

struct NoShaders;
impl ShaderLookup for NoShaders {
    fn raster_handle_by_name(&self, _: &str) -> Option<Handle<RasterShaderProgram>> {
        None
    }
}

fn cb(b: DrawBuilder<'_>, _i: &EntityDrawParams, _w: &mut World) {
    b.draw_no_vao(3);
}

fn rgtex(id: u32, w: u32, h: u32, format: u32) -> RgTexture {
    RgTexture {
        resource: id,
        desc: TextureDesc::new_2d([w, h], format),
    }
}

struct Scene {
    color: RgTexture,
    depth: RgTexture,
    a: RgTexture,
    b: RgTexture,
    c: RgTexture,
    d: RgTexture,
    lists: DrawLists,
}

fn scene(items: usize) -> Scene {
    let mut lists = DrawLists::new();
    for _ in 0..items {
        let mut it = DrawItem::new(1, 0);
        it.vao = 0x77;
        lists.list_mut(SURFACE_LIST).push((it, cb));
    }
    Scene {
        color: rgtex(1001, 641, 361, 44),
        depth: rgtex(1002, 641, 361, 45),
        a: rgtex(1003, 320, 181, 46),
        b: rgtex(1004, 641, 361, 47),
        c: rgtex(1005, 641, 361, 48),
        d: rgtex(1006, 641, 361, 49),
        lists,
    }
}

struct Run {
    trace: Vec<(PlanKind, Option<u64>)>,
    extra: Vec<&'static str>,
    log: Rc<RefCell<Log>>,
    rg: RecordingRg,
    graph: Graph,
    compute: Vec<RecordedComputePass>,
    paths: Vec<String>,
}

fn run(water: bool, s: &Scene, history_valid: u8) -> Run {
    let log = Rc::new(RefCell::new(Log::default()));
    let mut world = World::new();
    let mut factory = || mock_host(&log);
    let shaders = NoShaders;
    let mut ctx = DrawPassCtx::new(
        &mut world,
        Duration::new(2, 500_000_000),
        &mut factory,
        &shaders,
    );
    let cenv = cenv();
    let mut seam = Seam {
        env: &cenv,
        recorded: Vec::new(),
    };
    let mut rg = RecordingRg {
        next_id: 500,
        ..RecordingRg::default()
    };
    let mut graph = Graph::default();
    let (trace, extra) = {
        let mut env = SurfaceEnv::new(&mut rg, &mut graph, &mut seam);
        let inputs = SurfaceInputs {
            color: &s.color,
            depth: &s.depth,
            half_depth: &s.a,
            taa_history: &s.b,
            history_valid,
            refraction: &s.c,
            reprojection: &s.d,
            lists: &s.lists,
        };
        if water {
            water_surface_pass(&mut ctx, &mut env, &inputs);
        } else {
            ice_surface_pass(&mut ctx, &mut env, &inputs);
        }
        let t = env
            .trace
            .borrow()
            .iter()
            .map(|o| (o.kind, o.key))
            .collect::<Vec<_>>();
        let x = env.extra.borrow().clone();
        (t, x)
    };
    let paths = cenv.backend.paths.borrow().clone();
    Run {
        trace,
        extra,
        log,
        rg,
        graph,
        compute: seam.recorded,
        paths,
    }
}

fn golden(ops: &[super::PlanOp]) -> Vec<(PlanKind, Option<u64>)> {
    ops.iter().map(|o| (o.kind, o.key)).collect()
}

fn uniform(p: &RecordedComputePass, off: usize) -> u32 {
    let d = p.uniform_binder.as_ref().unwrap().data();
    u32::from_le_bytes(d[off..off + 4].try_into().unwrap())
}

// ------------------------------------------------------------------------------------------------ 名字 / 公式

fn fnv(s: &str) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for b in s.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h
}

#[test]
fn binding_hashes_are_fnv1a_of_the_hlsl_reflection_names() {
    use names::*;
    for (h, n) in [
        (HISTORY_VALID, "history_valid"),
        (HALF_DEPTH_TEX, "half_depth_tex"),
        (TAA_HISTORY_TEX, "taa_history_tex"),
        (REPROJECTION_TEX, "reprojection_tex"),
        (REFRACTION_TEX, "refraction_tex"),
        (BLUE_NOISE_TEXTURE, "blue_noise_texture"),
        (TERRAIN_HEIGHTMAP, "terrain_heightmap"),
        (VISUAL_TERRAIN_HEIGHTMAP, "visual_terrain_heightmap"),
        (TERRAIN_LIGHTING_TEXTURE, "terrain_lighting_texture"),
        (COLOR_TEX, "color_tex"),
        (WATER_RT_COUNT_BUFFER, "water_rt_count_buffer"),
        (WATER_RT_RAY_BUFFER, "water_rt_ray_buffer"),
        (SHADOW_MAP, "shadow_map"),
        (BOOL_RENDER_SHADOWS, "bool_render_shadows"),
        (SHADOWS_HISTORY_TEX, "shadows_history_tex"),
        (ICE_REFL_COUNT_BUFFER, "ice_refl_count_buffer"),
        (ICE_REFL_PX_BUFFER, "ice_refl_px_buffer"),
        (ICE_RT_COUNT_BUFFER, "ice_rt_count_buffer"),
        (ICE_RT_RAY_BUFFER, "ice_rt_ray_buffer"),
        (REFL_HIT_TEX, "refl_hit_tex"),
        (REFL_HIT_RADIANCE_TEX, "refl_hit_radiance_tex"),
        (ICE_ROUGHNESS_TEX, "ice_roughness_tex"),
        (REFR_HIT_TEX, "refr_hit_tex"),
        (REFR_HIT_RADIANCE_TEX, "refr_hit_radiance_tex"),
        (DEPTH_TEX, "depth_tex"),
        (REFL_REFR_RADIANCE_OUT_TEX, "refl_refr_radiance_out_tex"),
        (BRDF_FG_LUT, "brdf_fg_lut"),
        (
            REFL_REFR_RADIANCE_HISTORY_TEX,
            "refl_refr_radiance_history_tex",
        ),
        (RAY_LEN_HISTORY_TEX, "ray_len_history_tex"),
        (RAY_LEN_OUT_TEX, "ray_len_out_tex"),
    ] {
        assert_eq!(h, fnv(n), "{n}");
    }
}

#[test]
fn ray_buffer_size_formulas_follow_the_32bit_arithmetic() {
    // 水：(n + n>>4) << 4；冰 px：4 * (n + n>>4)；32 位乘 / 加回绕。
    assert_eq!(ray_units(320, 181), 57_920 + 3_620);
    assert_eq!(
        u64::from(ray_units(320, 181)) << 4,
        water_ray_buffer_size(320, 181)
    );
    assert_eq!(ray_units(65536, 65536), 0);
    assert_eq!(ray_units(0x8000_0000, 2), 0);
    // n + (n >> 4) 在 u32 内回绕：n = 0xffff_fff0 → + 0x0fff_ffff
    assert_eq!(
        ray_units(0xffff_fff0, 1),
        0xffff_fff0u32.wrapping_add(0x0fff_ffff)
    );
}

#[test]
fn half_res_rounds_up_and_never_reaches_zero() {
    let d = half_res(TextureDesc::new([641, 361, 1], 0, 0));
    assert_eq!(d.extent, [321, 181, 1]);
    assert_eq!(
        half_res(TextureDesc::new([1, 2, 1], 0, 0)).extent,
        [1, 1, 1]
    );
}

// ------------------------------------------------------------------------------------------------ 水

#[test]
fn water_pass_call_sequence_equals_the_golden_table() {
    let s = scene(1);
    let r = run(true, &s, 1);
    assert_eq!(r.trace, golden(WATER_OPS));
}

#[test]
fn water_pass_creates_buffers_clears_count_and_sizes_the_ray_buffer_from_a() {
    let s = scene(1);
    let r = run(true, &s, 1);
    // RG 创建：count(0x10, 0x20) → ray((n+n>>4)<<4)，名字与用途字。
    assert_eq!(r.rg.created.len(), 2);
    assert_eq!(r.rg.created[0].0, "water_rt_count_buffer");
    assert_eq!(r.rg.created[1].0, WATER_RAY_BUFFER_NAME);
    let RgKind::Buffer(c) = r.rg.created[0].1 else {
        panic!()
    };
    let RgKind::Buffer(ray) = r.rg.created[1].1 else {
        panic!()
    };
    assert_eq!((c.size, c.usage, c.gpu_only), (0x10, BUFFER_USAGE, true));
    assert_eq!(ray.size as u64, water_ray_buffer_size(320, 181));
    assert_eq!(ray.usage, 0x20);
    // clear_rg_buffer("Clear buffer", 访问 0x34) 作用在 count 缓冲（资源号 500）。
    assert_eq!(r.graph.passes, [("Clear buffer", 500, 0x34)]);
}

#[test]
fn water_raster_pass_shape_and_bindings() {
    let s = scene(1);
    let r = run(true, &s, 1);
    let l = r.log.borrow();
    assert_eq!(l.submissions.len(), 1);
    let p = &l.submissions[0];
    assert_eq!(p.name, "Water plane");
    assert_eq!(p.color_count, 1);
    // 颜色 tag0 → 0x36；深度 tag0 可写 → 0x2f。
    assert_eq!(p.accesses[0].access, 0x36);
    assert_eq!(p.accesses[1].access, 0x2f);
    assert_eq!(p.draws.len(), 1);
    // 状态：默认 0x8000_0080 → byte3 = (b3 & 0x9f) | 0x40（没有 byte0 &= 0xbf）。
    assert_eq!(
        p.draws[0].state.0,
        (0x8000_0080 & !0x6000_0000) | 0x4000_0000
    );
    // RG 资源访问：A、B、D（0x1d）、count、ray（0x37）、C（0x1d），按此顺序。
    let rg: Vec<(u32, u8)> = p
        .accesses
        .iter()
        .filter(|a| matches!(a.resource, 500 | 501 | 1003..=1006))
        .map(|a| (a.resource, a.access))
        .collect();
    assert_eq!(
        rg,
        [
            (1003, 0x1d),
            (1004, 0x1d),
            (1006, 0x1d),
            (500, 0x37),
            (501, 0x37),
            (1005, 0x1d)
        ]
    );
}

#[test]
fn water_compute_passes_names_uris_and_dispatches() {
    let s = scene(1);
    let r = run(true, &s, 1);
    assert_eq!(r.compute.len(), 2);
    let names_: Vec<&str> = r.compute.iter().map(|p| p.name).collect();
    assert_eq!(names_, ["Water deferred rt args", "Water deferred rt"]);
    assert!(
        r.paths[0].starts_with("/water/calc_deferred_rt_args.cs.hlsl"),
        "{:?}",
        r.paths
    );
    assert!(
        r.paths[1].starts_with("/water/deferred_rt.cs.hlsl"),
        "{:?}",
        r.paths
    );
    // args：单线程（disc 1，(1,1,1)）；rt：间接 dispatch（count 缓冲 500，偏移 0）。
    assert!(matches!(
        r.compute[0].dispatch,
        DispatchMode::Threads { x: 1, y: 1, z: 1 }
    ));
    assert!(matches!(
        r.compute[1].dispatch,
        DispatchMode::IndirectRaw {
            buffer: 500,
            offset: 0
        }
    ));
    // args：写 count（0x37、组 0）；rt：读 count / ray（0x1a、组 1）、color 写（0x32）、间接读（0x02）。
    let acc = |p: &RecordedComputePass| {
        p.builder
            .accesses
            .iter()
            .map(|a| (a.resource, a.access, a.group))
            .collect::<Vec<_>>()
    };
    assert!(acc(&r.compute[0]).contains(&(500, 0x37, 0)));
    let a1 = acc(&r.compute[1]);
    assert!(a1.contains(&(500, 0x1a, 1)) && a1.contains(&(501, 0x1a, 1)));
    assert!(a1.contains(&(1001, 0x32, 0)), "color_tex 以写访问绑定");
    assert!(
        a1.contains(&(500, 0x02, 1)),
        "dispatch_indirect_raw 登记间接读"
    );
    // history_valid（u8 → u32）写进计算 pass 的 uniform；args 没有该 uniform。
    assert_eq!(uniform(&r.compute[1], HISTORY_VALID_OFFSET), 1);
    let r0 = run(true, &s, 0);
    assert_eq!(uniform(&r0.compute[1], HISTORY_VALID_OFFSET), 0);
}

#[test]
fn water_with_an_empty_list_still_creates_the_raster_pass_but_binds_nothing_per_item() {
    let s = scene(0);
    let r = run(true, &s, 1);
    let mut expect = golden(WATER_OPS);
    expect.drain(6..13);
    assert_eq!(r.trace, expect);
    assert_eq!(r.log.borrow().submissions[0].draws.len(), 0);
}

// ------------------------------------------------------------------------------------------------ 冰

#[test]
fn ice_pass_call_sequence_equals_the_golden_table() {
    let s = scene(1);
    let r = run(false, &s, 1);
    assert_eq!(r.trace, golden(ICE_OPS));
    // PlanKind 词汇表之外的步骤（按调用顺序）。
    assert_eq!(
        r.extra,
        [
            "rg_buffer_copy_from_slice",
            "rg_buffer_copy_from_slice",
            "temporal_pair",
            "clear_rg_color_image",
            "temporal_pair",
            "temporal_pair",
        ]
    );
}

#[test]
fn ice_pass_creates_the_thirteen_named_resources_with_the_observed_formats_and_sizes() {
    let s = scene(1);
    let r = run(false, &s, 1);
    let created: Vec<&str> = r.rg.created.iter().map(|(n, _)| n.as_str()).collect();
    // create_raw_resource 的顺序：3 纹理、4 缓冲、（栅格）、2 个半分辨率纹理。
    assert_eq!(
        created,
        [
            "ice roughness tex",
            "ice refr hits",
            "ice refr hit radiance",
            "ice_refl_count_buffer",
            "ice_rt_count_buffer",
            "ice_refl_px_buffer",
            "ice_rt_ray_buffer",
            "water refl hit radiance",
            "water refl hits",
        ]
    );
    for n in &created {
        assert!(ICE_RESOURCE_NAMES.contains(n));
    }
    // 十三个名字：九个 create_raw_resource + 两个 temporal pair("water refr temporal"/"ice refl ray len") + "Ice shadows" + "Ice plane"。
    assert_eq!(ICE_RESOURCE_NAMES.len(), 13);
    let tex = |i: usize| match r.rg.created[i].1 {
        RgKind::Texture(t) => t,
        _ => panic!(),
    };
    let buf = |i: usize| match r.rg.created[i].1 {
        RgKind::Buffer(b) => b,
        _ => panic!(),
    };
    // 纹理：color 的 extent（641x361）与格式码。
    assert_eq!((tex(0).extent, tex(0).format), ([641, 361, 1], 9));
    assert_eq!((tex(1).extent, tex(1).format), ([641, 361, 1], 0x62));
    assert_eq!(
        (tex(2).extent, tex(2).format, tex(2).sampled),
        ([641, 361, 1], 0x62, true)
    );
    // 缓冲：计数 0x10；n = A.w*A.h = 320*181；px = 4*(n + n>>4)、ray = (n + n>>4) << 4；用途字全是 0x20。
    let units = ray_units(320, 181) as usize;
    assert_eq!(buf(3).size, 0x10);
    assert_eq!(buf(4).size, 0x10);
    assert_eq!(buf(5).size, 4 * units);
    assert_eq!(buf(6).size, units << 4);
    for i in 3..=6 {
        assert_eq!(buf(i).usage, 0x20);
    }
    // 半分辨率：ceil(641/2) x ceil(361/2) = 321x181。
    assert_eq!(
        (tex(7).extent, tex(7).format, tex(7).sampled),
        ([321, 181, 1], 0x62, true)
    );
    assert_eq!((tex(8).extent, tex(8).format), ([321, 181, 1], 0x53));
    // temporal pair 的描述（名字 / 格式）：Ice shadows(9, sampled)、water refr temporal(0x7a, sampled+storage)、ice refl ray len(0x53)。
    assert_eq!(r.rg.temporal.len(), 3);
    for n in ["Ice shadows", "water refr temporal", "ice refl ray len"] {
        assert!(r.rg.temporal.contains_key(n), "{n}");
    }
    // 图：两个计数缓冲的 memcpy（"Buffer memcpy"，访问 0x34）→ clear_rg_color_image("Clear image")。
    assert_eq!(
        r.graph.passes,
        [
            ("Buffer memcpy", 503, 0x34),
            ("Buffer memcpy", 504, 0x34),
            ("Clear image", 510, 0x34)
        ]
    );
}

#[test]
fn ice_raster_pass_has_five_colors_with_the_observed_tags_and_binds_in_order() {
    let s = scene(1);
    let r = run(false, &s, 1);
    let l = r.log.borrow();
    assert_eq!(l.submissions.len(), 1);
    let p = &l.submissions[0];
    assert_eq!(p.name, "Ice plane");
    assert_eq!(p.color_count, 5);
    // tag0 → 0x36；tag1 / tag2 → 0x2e；深度可写 0x2f。
    let acc: Vec<u8> = p.accesses.iter().take(6).map(|a| a.access).collect();
    assert_eq!(acc, [0x36, 0x2e, 0x2e, 0x2e, 0x2e, 0x2f]);
    assert_eq!(p.draws.len(), 1);
    assert_eq!(
        p.draws[0].state.0,
        (0x8000_0080 & !0x6000_0000) | 0x4000_0000
    );
}

#[test]
fn ice_compute_passes_names_uris_dispatches_and_resources() {
    let s = scene(1);
    let r = run(false, &s, 1);
    let got: Vec<&str> = r.compute.iter().map(|p| p.name).collect();
    let want: Vec<&str> = ICE_COMPUTE_PASSES.iter().map(|(n, _)| *n).collect();
    assert_eq!(got, want);
    for (i, (_, uri)) in ICE_COMPUTE_PASSES.iter().enumerate() {
        assert!(r.paths[i].starts_with(uri), "{} vs {}", r.paths[i], uri);
    }
    // 资源号：refl_count 503、rt_count 504、px 505、ray 506；"Ice shadows" pair 占 507/508，refl_hit_radiance 509、refl_hits 510。
    assert!(matches!(
        r.compute[0].dispatch,
        DispatchMode::IndirectRaw {
            buffer: 503,
            offset: 0
        }
    ));
    assert!(matches!(
        r.compute[1].dispatch,
        DispatchMode::IndirectRaw {
            buffer: 504,
            offset: 0
        }
    ));
    // resolve 两个：dispatch_2d(color extent 641x361)。
    assert!(matches!(
        r.compute[2].dispatch,
        DispatchMode::Threads {
            x: 641,
            y: 361,
            z: 1
        }
    ));
    assert!(matches!(
        r.compute[3].dispatch,
        DispatchMode::Threads {
            x: 641,
            y: 361,
            z: 1
        }
    ));
    let acc = |p: &RecordedComputePass| {
        p.builder
            .accesses
            .iter()
            .map(|a| (a.resource, a.access, a.group))
            .collect::<Vec<_>>()
    };
    let m = acc(&r.compute[0]);
    assert!(
        m.contains(&(503, 0x1a, 1)) && m.contains(&(505, 0x1a, 1)),
        "march 读 refl_count / refl_px"
    );
    assert!(
        m.contains(&(504, 0x37, 0)) && m.contains(&(506, 0x37, 0)),
        "march 写 rt_count / rt_ray"
    );
    assert!(
        m.contains(&(510, 0x32, 0)) && m.contains(&(509, 0x32, 0)),
        "march 写 refl_hit / refl_hit_radiance"
    );
    let t = acc(&r.compute[1]);
    assert!(
        t.contains(&(504, 0x1a, 1)) && t.contains(&(506, 0x1a, 1)),
        "trace 读 rt_count / rt_ray"
    );
    let rr = acc(&r.compute[3]);
    assert!(rr.contains(&(1001, 0x32, 0)), "refl resolve 写 color_tex");
    // refr resolve 写 water refr temporal 当前帧（RecordingRg 的 pair 占连续两个资源号）。
    let refr = acc(&r.compute[2]);
    assert!(refr.iter().any(|a| a.1 == 0x32), "{refr:?}");
    // history_valid：march / trace = 输入 u8；refl resolve = (flag1 & flag2) & 1（RecordingRg 首次创建标志 = 1）。
    assert_eq!(uniform(&r.compute[0], HISTORY_VALID_OFFSET), 1);
    assert_eq!(uniform(&r.compute[1], HISTORY_VALID_OFFSET), 1);
    assert_eq!(uniform(&r.compute[3], HISTORY_VALID_OFFSET), 1);
    // refr resolve 没有 history_valid。
    assert_eq!(uniform(&r.compute[2], HISTORY_VALID_OFFSET), 0);
    // 输入 u8 = 0 → march / trace 为 0，resolve 仍由 temporal 标志决定。
    let z = run(false, &s, 0);
    assert_eq!(uniform(&z.compute[0], HISTORY_VALID_OFFSET), 0);
    assert_eq!(uniform(&z.compute[1], HISTORY_VALID_OFFSET), 0);
    assert_eq!(uniform(&z.compute[3], HISTORY_VALID_OFFSET), 1);
}

#[test]
fn ice_refr_resolve_and_refl_resolve_bind_the_radiance_textures_with_a_view() {
    let s = scene(1);
    let r = run(false, &s, 1);
    use crate::render::pass_builder::pass_resource_access::PassBinding;
    for idx in [2usize, 3] {
        let views: Vec<_> = r.compute[idx]
            .bindings
            .iter()
            .filter_map(|b| match b {
                PassBinding::ImageView { view, .. } if view.format == Some(0x7b) => Some(*view),
                _ => None,
            })
            .collect();
        assert_eq!(views.len(), 1, "pass {idx}");
        assert_eq!(views[0].usage, Some(4));
    }
}

#[test]
fn att_maps_rg_textures_to_attachments() {
    let a = att(&rgtex(7, 64, 32, 44));
    assert_eq!(
        (a.id, a.extent, a.format, a.sample_count),
        (7, (64, 32), 44, 1)
    );
}
