//! Test-only observers for the real enclosing world graph. This module never
//! creates a scene, a draw list, a ripple, a BVH, a history or a game shader.
//! Call `SurfaceReadback::declare` after real W44..46, inside the same main
//! prelude. Complete the enclosing frame and wait for its actual submits before
//! `read`. Include this file from the integration fixture; do not add it to the
//! production module tree.
use ash::vk::{self, Handle as _};
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::{
        Handle, mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary,
        ssbo_library::AssetSsboLibrary, vao_library::AssetVAOLibrary,
    },
    render::{
        frame_constants::FrameConstants, mesh::Mesh, vao::Vao,
        pass_builder::{
            draw_stream::PassHost, framebuffer_binding::AttachmentTexture,
            pass_resource_access::RgTexture,
        },
    },
    resources::{
        rt::{RtBvhSsbo, RtTriangleMetaSsbo, RtTrianglesSsbo},
        session::{ActiveSession, GladeType, SessionId}, water::RippleSourceSsbo,
    },
    systems::render_loop::draw_passes::{
        d_main_support::{DrawPassCtx, ShaderLookup}, draw_lists::DrawLists, final_post,
    },
};
use rhapsody::{
    resource::{GpuUse, UseVisitor},
    rg::{Descriptor, ResourceHandle},
};
use std::sync::{Arc, Mutex};
use tiny_glade::render_bridge::CoreGraphFrame;

fn invalid(what: impl Into<String>) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(what)
}

/// Execute W1..54 against an already-started real gameplay App, without
/// replacing its Update schedule or startup. The fixture must call its actual
/// `App::update` once before every invocation. `extent` and `controls` come from
/// the normal caller; neither a fabricated camera nor a synthetic history is
/// installed here. The shared world_frame descriptor helpers are the same ones
/// production_renderer uses. Completion here proves this GPU graph, not WSI
/// presentation or publication of the gameplay SubApp.
pub fn render_existing_world_frame(
    bridge: &mut tiny_glade::render_bridge::CoreRenderBridge,
    world: &mut World,
    extent: [u32; 2],
    controls: tiny_glade::world_graph::WorldGraphControls,
) -> rhapsody::Result<SurfaceFrameEvidence> {
    use country_core::{
        render::pass_builder::rg_resources::RgResources as _,
        systems::render_loop::{draw_passes::draw_lists::build_draw_lists, world_frame},
    };
    let decision = tiny_glade::world_graph::select_main_world(world)?;
    if !decision.render_world { return Err(invalid("Surface fixture is not on a rendered gameplay frame")); }
    let mut lists = DrawLists::new();
    build_draw_lists(world, bridge.shader_library().as_ref(), &mut lists);
    lists.sort();
    bridge.raster_resources().refresh_draw_vaos(world, &lists)?;
    let (pending, built) = bridge.build_frame(|frame| frame.with_main_prelude(world, |frame, world| {
        let (color, depth) = frame.with_compute(|compute| Ok((
            compute.create_raw_texture(world_frame::MAIN_COLOR_NAME, &world_frame::main_color_desc(extent)),
            compute.create_raw_texture(world_frame::MAIN_DEPTH_NAME, &world_frame::main_depth_desc(extent)),
        )))?;
        let out = tiny_glade::world_graph::build_world(frame, world, &lists, extent, depth, color, controls)?;
        // Keep the real downstream W47..54 consumer as well as the observed
        // W44..46 output; the test does not truncate world construction at water.
        frame.with_graph(|graph| {
            let final_color = graph.resource_handle(out.taa.resource)?;
            graph.export(final_color, 0x22)?;
            Ok(())
        })?;
        SurfaceReadback::declare(frame, world, &lists, color)
    }))?;
    bridge.shader_backend().prewarm(std::time::Duration::from_secs(10))?;
    let compiled = bridge.compile_built(built)?;
    let receipt = bridge.execute_compiled(compiled)?;
    if receipt.receipts.is_empty() { return Err(invalid("Surface graph produced no actual GPU submissions")); }
    bridge.wait_submits_done(10_000_000_000)?;
    pending.read(&receipt.snapshot)
}

/// One immutable real draw identity, copied before graph declaration releases
/// World. Handles are checked against this App's Mesh -> VAO -> shader owners.
#[derive(Clone, Debug)]
pub struct SurfaceDraw {
    pub list: usize,
    pub entity: bevy_ecs::entity::Entity,
    pub vao: u64,
    pub shader: u64,
    pub shader_name: &'static str,
    pub indices: usize,
}

fn real_surface_draws(world: &World, lists: &DrawLists, glade: GladeType)
    -> rhapsody::Result<Vec<SurfaceDraw>>
{
    let meshes = world.resource::<AssetMeshLibrary>();
    let vaos = world.resource::<AssetVAOLibrary>();
    let water = meshes.get_handle_by_name("water_plane");
    let mesh = meshes.get(water).ok_or_else(|| invalid("real water mesh missing"))?;
    if mesh.indices.is_empty() || !mesh.source_file.as_ref()
        .is_some_and(|source| source.path.ends_with("water_plane.json"))
    {
        return Err(invalid("surface fixture requires the shipped nonempty water mesh"));
    }
    let mut result = Vec::with_capacity(lists.list(14).len() + lists.list(15).len());
    for (list, expected) in [(14, "water_plane_refraction_copy"),
        (15, if glade.is_winter() { "ice_plane" } else { "water_plane" })]
    {
        if lists.list(list).is_empty() {
            return Err(invalid(format!("real surface DrawLists[{list}] is empty")));
        }
        for (item, _) in lists.list(list) {
            let entity = world.get_entity(item.params.entity)
                .map_err(|_| invalid("surface draw references a dead entity"))?;
            let mesh = entity.get::<Handle<Mesh>>()
                .ok_or_else(|| invalid("surface entity lacks its actual Mesh handle"))?;
            let vao = entity.get::<Handle<Vao>>()
                .ok_or_else(|| invalid("surface entity lacks maintenance-produced VAO"))?;
            let shader = entity.get::<Handle<country_core::render::shader::RasterShaderProgram>>()
                .ok_or_else(|| invalid("surface entity lacks its actual shader handle"))?;
            let actual = vaos.get_by_handle(*vao)
                .ok_or_else(|| invalid("surface VAO does not belong to this App"))?;
            let by_mesh = vaos.get(*mesh)
                .ok_or_else(|| invalid("surface Mesh has no registered VAO"))?;
            if *mesh != water || !std::ptr::eq(actual, by_mesh)
                || item.vao != vao.id().raw() || item.shader != shader.id().raw()
                || item.params.shader_name != expected || actual.index_count == 0
            {
                return Err(invalid("surface DrawItem disagrees with actual entity/VAO/shader"));
            }
            result.push(SurfaceDraw { list, entity: item.params.entity,
                vao: item.vao, shader: item.shader, shader_name: expected,
                indices: actual.index_count });
        }
    }
    Ok(result)
}

/// A diagnostic compute copy is required for the *actual* canonical ripple
/// and RT allocations: their original 0x22 usage has STORAGE_BUFFER and
/// TRANSFER_DST, but no TRANSFER_SRC. Altering the producer's usage for a test
/// would test a different allocation. This shader only copies uint words into
/// a host-visible storage output; it does not implement any game behavior.
fn copy_spirv() -> Vec<u32> {
    use rspirv::{binary::Assemble, dr::{Builder, Operand}, spirv::*};
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let function = b.type_function(void, []);
    let uint = b.type_int(32, 0);
    let v3 = b.type_vector(uint, 3);
    let input_ptr = b.type_pointer(None, StorageClass::Input, v3);
    let invocation = b.variable(input_ptr, None, StorageClass::Input, None);
    b.decorate(invocation, Decoration::BuiltIn, [Operand::BuiltIn(BuiltIn::GlobalInvocationId)]);
    let words = b.type_runtime_array(uint);
    b.decorate(words, Decoration::ArrayStride, [Operand::LiteralInt32(4)]);
    let block = b.type_struct([words]);
    b.decorate(block, Decoration::Block, []);
    b.member_decorate(block, 0, Decoration::Offset, [Operand::LiteralInt32(0)]);
    let block_ptr = b.type_pointer(None, StorageClass::StorageBuffer, block);
    let word_ptr = b.type_pointer(None, StorageClass::StorageBuffer, uint);
    let source = b.variable(block_ptr, None, StorageClass::StorageBuffer, None);
    let output = b.variable(block_ptr, None, StorageClass::StorageBuffer, None);
    for (binding, variable) in [(0, source), (1, output)] {
        b.decorate(variable, Decoration::DescriptorSet, [Operand::LiteralInt32(0)]);
        b.decorate(variable, Decoration::Binding, [Operand::LiteralInt32(binding)]);
    }
    b.decorate(source, Decoration::NonWritable, []);
    b.decorate(output, Decoration::NonReadable, []);
    let zero = b.constant_bit32(uint, 0);
    let main = b.begin_function(void, None, FunctionControl::NONE, function).unwrap();
    b.begin_block(None).unwrap();
    let id = b.load(v3, None, invocation, None, []).unwrap();
    let x = b.composite_extract(uint, None, id, [0]).unwrap();
    let from = b.access_chain(word_ptr, None, source, [zero, x]).unwrap();
    let to = b.access_chain(word_ptr, None, output, [zero, x]).unwrap();
    let value = b.load(uint, None, from, None, []).unwrap();
    b.store(to, value, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::GLCompute, main, "main", [invocation]);
    b.execution_mode(main, ExecutionMode::LocalSize, [1, 1, 1]);
    b.module().assemble()
}

struct CopyKernel {
    device: Arc<rhapsody::device::Device>,
    shader: vk::ShaderModule,
    sets: vk::DescriptorSetLayout,
    layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    pool: vk::DescriptorPool,
}
impl GpuUse for CopyKernel {
    fn visit_uses(&self, _: &mut UseVisitor<'_>) -> rhapsody::Result<()> { Ok(()) }
}
impl Drop for CopyKernel {
    fn drop(&mut self) {
        // The graph command owns an Arc through retain_object. Its declared
        // memory owners remain separately retained by ManagedPassResources.
        unsafe {
            let d = self.device.raw_device();
            if self.pool != vk::DescriptorPool::null() { d.destroy_descriptor_pool(self.pool, None); }
            if self.pipeline != vk::Pipeline::null() { d.destroy_pipeline(self.pipeline, None); }
            if self.layout != vk::PipelineLayout::null() { d.destroy_pipeline_layout(self.layout, None); }
            if self.sets != vk::DescriptorSetLayout::null() { d.destroy_descriptor_set_layout(self.sets, None); }
            if self.shader != vk::ShaderModule::null() { d.destroy_shader_module(self.shader, None); }
        }
    }
}
impl CopyKernel {
    fn new(device: Arc<rhapsody::device::Device>, copies: u32) -> rhapsody::Result<Arc<Self>> {
        let mut out = Self { device, shader: vk::ShaderModule::null(),
            sets: vk::DescriptorSetLayout::null(), layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(), pool: vk::DescriptorPool::null() };
        let map = |error| invalid(format!("surface diagnostic compute setup: {error:?}"));
        unsafe {
            let d = out.device.raw_device();
            out.shader = d.create_shader_module(&vk::ShaderModuleCreateInfo::default()
                .code(&copy_spirv()), None).map_err(map)?;
            let bindings = [0, 1].map(|binding| vk::DescriptorSetLayoutBinding::default()
                .binding(binding).descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1).stage_flags(vk::ShaderStageFlags::COMPUTE));
            out.sets = d.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default()
                .bindings(&bindings), None).map_err(map)?;
            out.layout = d.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default()
                .set_layouts(&[out.sets]), None).map_err(map)?;
            let stage = vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::COMPUTE).module(out.shader).name(c"main");
            out.pipeline = match d.create_compute_pipelines(vk::PipelineCache::null(),
                &[vk::ComputePipelineCreateInfo::default().stage(stage).layout(out.layout)], None)
            {
                Ok(mut pipelines) => pipelines.remove(0),
                Err((pipelines, error)) => {
                    for pipeline in pipelines { d.destroy_pipeline(pipeline, None); }
                    return Err(map(error));
                }
            };
            out.pool = d.create_descriptor_pool(&vk::DescriptorPoolCreateInfo::default()
                .max_sets(copies).pool_sizes(&[vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::STORAGE_BUFFER, descriptor_count: 2 * copies,
                }]), None).map_err(map)?;
        }
        Ok(Arc::new(out))
    }

    fn read_words(self: &Arc<Self>, frame: &mut CoreGraphFrame<'_>, source: ResourceHandle,
        words: u32, name: &'static str) -> rhapsody::Result<Arc<rhapsody::resource::Buffer>>
    {
        let size = u64::from(words) * 4;
        if words == 0 { return Err(invalid("zero-word surface probe is not a GPU readback")); }
        let output = Arc::new(rhapsody::resource::Buffer::new(self.device.clone(),
            &rhapsody::resource::BufferDesc::new_gpu_to_cpu(size, vk::BufferUsageFlags::STORAGE_BUFFER))?);
        let output_id = frame.import_native_buffer(output.clone(), 0)?;
        let set = unsafe { self.device.raw_device().allocate_descriptor_sets(
            &vk::DescriptorSetAllocateInfo::default().descriptor_pool(self.pool)
                .set_layouts(&[self.sets])) }
            .map_err(|error| invalid(format!("surface diagnostic descriptor allocation: {error:?}")))?[0];
        let kernel = self.clone();
        frame.with_graph(|graph| {
            let input = graph.buffer_id(source)?;
            let destination = graph.buffer_id(output_id)?;
            let mut pass = graph.add_pass(name);
            pass.read_buffer(input, 0x1a, rhapsody::rg::AccessRange::Buffer { offset: 0, size })?;
            pass.write_buffer(destination, 0x32, rhapsody::rg::AccessRange::Buffer { offset: 0, size })?;
            pass.render_with_result_context(move |ctx| {
                let resources = ctx.managed.ok_or_else(|| invalid("surface probe has no managed resources"))?;
                let from = resources.buffer_for(source.raw())
                    .map_err(|error| invalid(format!("surface probe source: {error:?}")))?;
                let to = resources.buffer_for(output_id.raw())
                    .map_err(|error| invalid(format!("surface probe destination: {error:?}")))?;
                let source_desc = [vk::DescriptorBufferInfo::default().buffer(from.raw_handle()).range(size)];
                let output_desc = [vk::DescriptorBufferInfo::default().buffer(to.raw_handle()).range(size)];
                ctx.retain_object(kernel.clone())?;
                unsafe {
                    let d = kernel.device.raw_device();
                    d.update_descriptor_sets(&[
                        vk::WriteDescriptorSet::default().dst_set(set).dst_binding(0)
                            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER).buffer_info(&source_desc),
                        vk::WriteDescriptorSet::default().dst_set(set).dst_binding(1)
                            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER).buffer_info(&output_desc),
                    ], &[]);
                    let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
                    d.cmd_bind_pipeline(cb, vk::PipelineBindPoint::COMPUTE, kernel.pipeline);
                    d.cmd_bind_descriptor_sets(cb, vk::PipelineBindPoint::COMPUTE, kernel.layout, 0, &[set], &[]);
                    d.cmd_dispatch(cb, words, 1, 1);
                }
                Ok(())
            });
            graph.export(output_id, 0x26)?;
            Ok(())
        })?;
        Ok(output)
    }
}

#[derive(Clone, Debug)]
pub struct HistoryObservation {
    pub name: String,
    pub token: u64,
    pub valid: bool,
    /// Derived from this frame's real native writes, not frame_index parity.
    pub current: bool,
}

pub struct SurfaceReadback {
    pub session: SessionId,
    pub glade: GladeType,
    pub frame_index: u32,
    pub draws: Vec<SurfaceDraw>,
    pub duck_rows: usize,
    pub rt_counts: [usize; 3],
    buffers: Vec<(&'static str, Arc<rhapsody::resource::Buffer>)>,
    pixels: Arc<rhapsody::resource::Buffer>,
    history: Arc<Mutex<Vec<HistoryObservation>>>,
}

fn named_buffer(frame: &mut CoreGraphFrame<'_>, name: &str) -> rhapsody::Result<ResourceHandle> {
    frame.with_graph(|graph| {
        let index = graph.declaration().resources.iter().position(|r| r.name == name
            && matches!(r.descriptor, Descriptor::Buffer(_)))
            .ok_or_else(|| invalid(format!("actual Surface output {name:?} absent")))?;
        Ok(graph.resource_handle(index as u32)?)
    })
}

impl SurfaceReadback {
    /// `color` must be the exact caller-owned color passed to build_world /
    /// build_world_surface. The caller has already executed real startup and
    /// PostUpdate maintenance, then real build_draw_lists on that World.
    pub fn declare(frame: &mut CoreGraphFrame<'_>, world: &mut World,
        lists: &DrawLists, color: RgTexture) -> rhapsody::Result<Self>
    {
        let session = world.resource::<ActiveSession>();
        let (session_id, glade) = (session.id, session.meta.glade);
        let draws = real_surface_draws(world, lists, glade)?;
        let frame_index = world.resource::<FrameConstants>().frame_counter;
        let duck_rows = world.iter_entities().filter(|entity|
            entity.contains::<system_critters::components::DuckState>()
                && entity.contains::<country_core::utils::components::transform::Transform>()).count();
        let library = world.resource::<AssetSsboLibrary>();
        let rt_counts = [
            library.get::<RtTrianglesSsbo>().expect("real RT triangles missing").instance_count,
            library.get::<RtTriangleMetaSsbo>().expect("real RT meta missing").instance_count,
            library.get::<RtBvhSsbo>().expect("real RT BVH missing").instance_count,
        ];
        if rt_counts.contains(&0) || rt_counts[0] != rt_counts[1] {
            return Err(invalid("surface fixture needs a completed nonempty real RT upload"));
        }
        let ripple = library.get_buffer::<RippleSourceSsbo>()
            .ok_or_else(|| invalid("canonical 528-byte ripple owner absent"))?;
        if ripple.byte_size != 528 { return Err(invalid("canonical ripple allocation is not 528 bytes")); }
        // Use the *existing* library graph cache, as the actual water callback
        // does. No second import/library/carrier is constructed.
        let mut rg = frame.resource_graph();
        let ripple = library.get_rg_buffer::<RippleSourceSsbo>(&mut rg).resource as u32;
        let triangles = library.get_rg_buffer::<RtTrianglesSsbo>(&mut rg).resource as u32;
        let metadata = library.get_rg_buffer::<RtTriangleMetaSsbo>(&mut rg).resource as u32;
        let bvh = library.get_rg_buffer::<RtBvhSsbo>(&mut rg).resource as u32;
        let mut inputs = vec![("ripple528", ripple, 132),
            ("rt_triangles", triangles, (rt_counts[0].min(32) * 6) as u32),
            ("rt_meta", metadata, rt_counts[1].min(32) as u32),
            ("rt_bvh", bvh, (rt_counts[2].min(32) * 20) as u32)];
        for name in if glade.is_winter() {
            vec!["ice_refl_count_buffer", "ice_rt_count_buffer"]
        } else { vec!["water_rt_count_buffer"] } {
            inputs.push((name, named_buffer(frame, name)?.raw(), 4));
        }
        let device = frame.hosts().buffers.graphics_device().clone();
        let kernel = CopyKernel::new(device.clone(), inputs.len() as u32)?;
        let mut buffers = Vec::with_capacity(inputs.len());
        for (name, resource, words) in inputs {
            let handle = frame.with_graph(|graph| Ok(graph.resource_handle(resource)?))?;
            buffers.push((name, kernel.read_words(frame, handle, words, "fixture read actual surface words")?));
        }
        let history = Arc::new(Mutex::new(Vec::new()));
        let observed = history.clone();
        frame.with_graph(|graph| {
            let mut inputs = Vec::new();
            for (index, resource) in graph.declaration().resources.iter().enumerate() {
                if !resource.name.starts_with("pair/") { continue; }
                if ![":taa_history:", ":tgsr_history:", ":Ice shadows:", ":water refr temporal:", ":ice refl ray len:"]
                    .iter().any(|name| resource.name.contains(name)) { continue; }
                let handle = graph.resource_handle(index as u32)?;
                let current = graph.declaration().passes.iter().any(|pass| pass.accesses.iter().any(|access|
                    access.resource_index == index as u32
                        && rhapsody::raw::access_table::is_write(access.access_code)));
                inputs.push((resource.name.clone(), graph.texture_id(handle)?, current));
            }
            // Recording this CPU observation is the explicit test side effect.
            // It never reads or initializes any history image contents.
            let mut pass = graph.add_pass("fixture observe actual surface temporal members");
            pass.side_effect();
            pass.render_with_result_context(move |ctx| {
                let mut result = observed.lock().unwrap();
                for (name, texture, current) in &inputs {
                    result.push(HistoryObservation { name: name.clone(),
                        token: ctx.compiled.registry.image(texture.raw().raw()).handle,
                        valid: ctx.history_valid(*texture)?, current: *current });
                }
                Ok(())
            });
            Ok(())
        })?;
        let pixels = resolve_readback(frame, world, color, device)?;
        Ok(Self { session: session_id, glade, frame_index, draws, duck_rows,
            rt_counts, buffers, pixels, history })
    }

    /// Call only after the enclosing CoreRenderBridge has waited for its
    /// actual submission receipts. Never read these buffers while recording.
    pub fn read(&self, snapshot: &rhapsody::rg::Snapshot) -> rhapsody::Result<SurfaceFrameEvidence> {
        let mut buffers = Vec::with_capacity(self.buffers.len());
        for (name, buffer) in &self.buffers {
            let mut bytes = vec![0; buffer.desc.size as usize];
            buffer.read_bytes(0, &mut bytes)?;
            buffers.push((*name, bytes.chunks_exact(4).map(|word|
                u32::from_le_bytes(word.try_into().unwrap())).collect()));
        }
        let mut pixels = vec![0; self.pixels.desc.size as usize];
        self.pixels.read_bytes(0, &mut pixels)?;
        let evidence = SurfaceFrameEvidence { session: self.session, glade: self.glade,
            frame_index: self.frame_index, draws: self.draws.clone(), duck_rows: self.duck_rows,
            rt_counts: self.rt_counts, buffers, pixels, history: self.history.lock().unwrap().clone(),
            passes: snapshot.pass_order.clone() };
        evidence.assert_nonempty();
        Ok(evidence)
    }
}

pub struct SurfaceFrameEvidence {
    pub session: SessionId,
    pub glade: GladeType,
    pub frame_index: u32,
    pub draws: Vec<SurfaceDraw>,
    pub duck_rows: usize,
    pub rt_counts: [usize; 3],
    pub buffers: Vec<(&'static str, Vec<u32>)>,
    pub pixels: Vec<u8>,
    pub history: Vec<HistoryObservation>,
    pub passes: Vec<String>,
}
impl SurfaceFrameEvidence {
    pub fn words(&self, name: &str) -> &[u32] {
        &self.buffers.iter().find(|row| row.0 == name).expect("surface GPU readback absent").1
    }
    pub fn assert_nonempty(&self) {
        assert!(self.pixels.chunks_exact(4).any(|p| p[..3] != [0, 0, 0]),
            "real world surface result did not reach the original resolve");
        let ripple = self.words("ripple528");
        assert_eq!(ripple.len(), 132);
        assert!(ripple[128] <= 32, "native count is the last16B, after32 records");
        assert_eq!(&ripple[129..], &[0, 0, 0]);
        if self.glade == GladeType::Summer {
            assert!(self.duck_rows > 0 && ripple[128] > 0,
                "Summer must execute the actual nonempty duck/ripple producer");
        }
        let expected: &[&str] = if self.glade.is_winter() { &[
            "Copy refraction target", "Ice plane", "Ice refl march", "Ice refl trace",
            "Ice refr resolve", "Ice refl resolve", "Pre-TAA transparency",
        ] } else { &[
            "Copy refraction target", "Water plane", "Water deferred rt args",
            "Water deferred rt", "Pre-TAA transparency",
        ] };
        let mut previous = None;
        for name in expected {
            let index = self.passes.iter().position(|pass| pass == name)
                .unwrap_or_else(|| panic!("actual submitted Surface pass {name:?} missing"));
            assert!(previous.is_none_or(|old| index > old));
            previous = Some(index);
        }
        for name in if self.glade.is_winter() {
            vec!["ice_refl_count_buffer", "ice_rt_count_buffer"]
        } else { vec!["water_rt_count_buffer"] } {
            let count = self.words(name);
            assert_eq!(count.len(), 4);
            assert_eq!(&count[1..3], &[1, 1]);
            assert!(count[3] > 0, "fixture must actually execute the GPU indirect consumer: {name}");
            assert_eq!(count[0], count[3].div_ceil(64));
        }
    }
}

/// Preserve chronological evidence. This verifies actual committed member
/// swapping; it does not infer parity from CPU frame_index or clear histories.
pub fn assert_two_consecutive_frames(first: &SurfaceFrameEvidence, second: &SurfaceFrameEvidence) {
    assert_eq!(first.session, second.session);
    assert_eq!(first.glade, second.glade);
    assert_eq!(second.frame_index, first.frame_index.wrapping_add(1));
    assert_eq!(first.rt_counts, second.rt_counts);
    for current in first.history.iter().filter(|row| row.current) {
        let previous = second.history.iter().find(|row| !row.current && row.token == current.token)
            .unwrap_or_else(|| panic!("committed temporal member did not become next previous: {}", current.name));
        assert!(previous.valid, "second frame history is not the completed first frame");
    }
    assert!(!first.history.is_empty(), "real color history pair must have been requested");
    if first.glade == GladeType::Summer {
        assert_ne!(first.words("ripple528"), second.words("ripple528"),
            "actual Local smoothing must survive and advance between Summer updates");
    }
}

struct Shaders<'a>(&'a AssetShaderLibrary);
impl ShaderLookup for Shaders<'_> {
    fn raster_handle_by_name(&self, name: &str) -> Option<country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>> {
        self.0.get_raster_handle_by_name(name).map(|h| country_core::rhapsody_contract::Handle(
            country_core::rhapsody_contract::ManagedHandle(h.id().raw()), std::marker::PhantomData))
    }
}

fn resolve_readback(frame: &mut CoreGraphFrame<'_>, world: &mut World, source: RgTexture,
    device: Arc<rhapsody::device::Device>) -> rhapsody::Result<Arc<rhapsody::resource::Buffer>>
{
    let [w, h] = source.desc.extent_2d();
    let target = Arc::new(rhapsody::resource::Texture::new(device.clone(),
        &rhapsody::resource::TextureDesc::new_2d(w, h, vk::Format::R8G8B8A8_UNORM)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC))?);
    let output = Arc::new(rhapsody::resource::Buffer::new(device.clone(),
        &rhapsody::resource::BufferDesc::new_gpu_to_cpu(u64::from(w) * u64::from(h) * 4,
            vk::BufferUsageFlags::TRANSFER_DST))?);
    let target_id = frame.import_native_texture(target.clone(), 0)?;
    let output_id = frame.import_native_buffer(output.clone(), 0)?;
    let shaders = frame.shader_library();
    let elapsed = world.resource::<country_core::resources::game_clock::GameClock>().elapsed;
    frame.with_raster(|host| {
        let mut make = || -> Box<dyn PassHost> { Box::new(host.sibling()) };
        let lookup = Shaders(&shaders);
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut make, &lookup);
        final_post::resolve_to_swapchain(&mut ctx,
            &AttachmentTexture { id: source.resource, extent: (w, h), format: source.desc.format, sample_count: 1 },
            &AttachmentTexture { id: target_id.raw(), extent: (w, h), format: 37, sample_count: 1 }, 0);
        Ok(())
    })?;
    frame.with_graph(|graph| {
        let output = output.clone();
        let mut pass = graph.add_pass("fixture read actual Surface resolve pixels");
        pass.add_resource_access(target_id, 0x25, 0)?;
        pass.add_resource_access(output_id, 0x34, 0)?;
        pass.render_with_result_context(move |ctx| {
            unsafe { device.raw_device().cmd_copy_image_to_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()), target.raw_handle(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL, output.raw_handle(),
                &[vk::BufferImageCopy::default().image_subresource(vk::ImageSubresourceLayers::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR).layer_count(1))
                    .image_extent(vk::Extent3D { width: w, height: h, depth: 1 })]); }
            Ok(())
        });
        graph.export(output_id, 0x26)?;
        Ok(())
    })?;
    Ok(output)
}
