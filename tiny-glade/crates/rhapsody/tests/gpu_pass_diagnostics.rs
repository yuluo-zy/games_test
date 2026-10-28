//! Per-pass GPU diagnostics: serialised (isolated) spans and pipeline statistics.
//! A raster pass draws a known number of triangles and a compute pass dispatches
//! a known number of invocations, so the counters can be checked exactly.
mod draw_support;
use ash::vk::{self, Handle};
use draw_support::*;
use parking_lot::Mutex;
use rhapsody::pipeline::shader::{ComputePipelineDesc, PipelineCache, PipelineLayout, ShaderModule, VulkanPipeline};
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use rhapsody::rg::{
    attachments::{Attachment, AttachmentCache},
    executor::PassContext,
    pass_api::{AshCommandBackend, RenderPassApi},
    profiling::{GpuCommandTiming, GpuTimingConfig},
    vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
};
use rhapsody::resource::texture::TextureDesc;
use std::sync::Arc;

const VERTEX_READ: u8 = 0x04;
const COLOR_WRITE: u8 = 0x2e;
const HOST_WRITE: u8 = 0x35;
const TIMEOUT_NS: u64 = 10_000_000_000;
/// `draw_support::Rig` renders `SIZE`x`SIZE`; the passes below repeat these counts.
const DRAWS: u32 = 2;
const LOCAL: [u32; 3] = [4, 2, 1];
const GROUPS: u32 = 3;

/// Minimal empty compute entry point (`main`, local size 1x1x1), as in `async_pipeline.rs`.
const WORDS: &[u32] = &[
    0x07230203, 0x00010000, 0, 6, 0, 0x00020011, 1, 0x0003000e, 0, 1, 0x0005000f, 5, 4, 0x6e69616d,
    0, 0x00060010, 4, 17, 1, 1, 1, 0x00020013, 1, 0x00030021, 2, 1, 0x00050036, 1, 4, 0, 2,
    0x000200f8, 5, 0x000100fd, 0x00010038,
];
fn compute_spirv(local: [u32; 3]) -> Vec<u8> {
    use rspirv::{binary::Assemble, dr::Operand};
    let mut module = rspirv::dr::load_words(WORDS).unwrap();
    for (operand, value) in module.execution_modes[0].operands[2..].iter_mut().zip(local) {
        *operand = Operand::LiteralInt32(value);
    }
    module.assemble().into_iter().flat_map(u32::to_le_bytes).collect()
}

fn extent() -> vk::Extent2D {
    vk::Extent2D { width: SIZE, height: SIZE }
}

struct Harness {
    rig: Rig,
    session: VulkanGraphSession,
    attachments: Arc<Mutex<AttachmentCache>>,
    compute: Arc<VulkanPipeline>,
}

fn harness(config: GpuTimingConfig) -> Option<Harness> {
    let rig = Rig::new()?;
    let mut backend = VulkanGraphBackend::new(rig.device.clone(), AccessTable::original()).ok()?;
    backend.configure_gpu_timing(Some(config)).ok()?;
    let cache = PipelineCache::new(rig.device.clone(), &[]).unwrap();
    let layout = PipelineLayout::new(rig.device.clone(), vec![], vec![]).unwrap();
    let shader = ShaderModule::from_spirv(rig.device.clone(), &compute_spirv(LOCAL)).unwrap();
    let compute =
        VulkanPipeline::create_compute_pipeline(&cache, ComputePipelineDesc::new(shader, layout)).unwrap();
    let attachments = Arc::new(Mutex::new(AttachmentCache::new(rig.device.clone()).unwrap()));
    Some(Harness { session: VulkanGraphSession::new(backend), rig, attachments, compute })
}

impl Harness {
    /// One frame: a raster pass drawing `DRAWS` triangles, then a compute pass
    /// dispatching `GROUPS` workgroups. Returns the completed samples.
    fn frame(&mut self) -> Vec<GpuCommandTiming> {
        let mut graph = self.session.begin_frame().unwrap();
        let triangle = rotated_triangle(0.0);
        let bytes: Vec<u8> = triangle
            .iter()
            .flat_map(|v| [v[0], -v[1], v[2], v[3]])
            .flat_map(f32::to_le_bytes)
            .collect();
        let vb_desc = BufferDesc::new_cpu_to_gpu(bytes.len() as u64, vk::BufferUsageFlags::VERTEX_BUFFER);
        let vertex_buffer = Arc::new(Buffer::new(self.rig.device.clone(), &vb_desc).unwrap());
        vertex_buffer.write_bytes_unsynchronized(0, &bytes).unwrap();
        let vb_token = self.session.backend.bind_import(GpuResource::Buffer(vertex_buffer)).unwrap();
        let vb = graph
            .import_unowned("triangle vb", ResourceSpec::Buffer(vb_desc).descriptor(), vb_token, HOST_WRITE)
            .unwrap();
        let color = self
            .session
            .backend
            .declare(
                &mut graph,
                "color",
                ResourceSpec::Image(TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM)),
            )
            .unwrap();
        {
            let device = self.rig.device.clone();
            let pipeline = self.rig.pipeline.clone();
            let cache = self.attachments.clone();
            let (color_index, vb_index) = (color.raw(), vb.raw());
            let mut pass = graph.add_pass("triangles").with_side_effect();
            pass.add_resource_access(vb, VERTEX_READ, 0).unwrap();
            pass.add_resource_access(color, COLOR_WRITE, 0).unwrap();
            pass.render_with_context(move |ctx: PassContext<'_>| {
                let Some(managed) = ctx.managed else { return false };
                let (Ok(texture), Ok(vertices)) =
                    (managed.texture_for(color_index), managed.buffer_for(vb_index))
                else {
                    return false;
                };
                let Ok(attachment) = Attachment::color(texture.shared(), 0, Some([0.0; 4])) else {
                    return false;
                };
                let Ok(prepared) = cache.lock().prepare_for_rendering(
                    &[attachment],
                    extent(),
                    1,
                    rhapsody::rg::attachments::RenderingPolicy::RequireDynamic,
                ) else {
                    return false;
                };
                if let Err(error) = prepared.retain_for_pass(&ctx) {
                    ctx.report_error(error);
                    return false;
                }
                let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
                // SAFETY: the command buffer is recording; the executor keeps the resources alive.
                let mut backend = unsafe { AshCommandBackend::new_for_device(&device) };
                let mut api = RenderPassApi::new(&mut backend, cb, 1, false);
                if api.begin_prepared(&prepared).is_err() {
                    return false;
                }
                api.set_default_view_and_scissor(extent());
                api.set_stencil_ref(0);
                unsafe {
                    device.raw_device().cmd_bind_pipeline(
                        cb,
                        vk::PipelineBindPoint::GRAPHICS,
                        pipeline.raw_handle(),
                    )
                };
                api.bind_vertex_buffer_by_binding_point(0, vertices.raw());
                for _ in 0..DRAWS {
                    api.draw(3, 1, 0, 0);
                }
                api.end_render_pass();
                true
            });
        }
        {
            let device = self.rig.device.clone();
            let pipeline = self.compute.clone();
            graph
                .add_pass("dispatch")
                .with_side_effect()
                .render_with_context(move |ctx: PassContext<'_>| {
                    let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
                    // SAFETY: recording command; the pipeline Arc outlives the test frame.
                    unsafe {
                        pipeline.bind_pipeline(cb);
                        device.raw_device().cmd_dispatch(cb, GROUPS, 1, 1);
                    }
                    true
                });
        }
        self.session.execute_frame(graph.build()).unwrap();
        self.session.wait_submits_done(TIMEOUT_NS).unwrap();
        self.session.backend.poll_gpu_timings().unwrap();
        self.session.backend.drain_gpu_timings().collect()
    }
}

fn pass<'a>(samples: &'a [GpuCommandTiming], name: &str) -> &'a GpuCommandTiming {
    let mut found = samples.iter().filter(|s| s.pass_name == Some(name));
    let sample = found.next().unwrap_or_else(|| panic!("no span for pass {name}: {samples:#?}"));
    assert!(found.next().is_none(), "duplicate span for pass {name}");
    sample
}

#[test]
fn pass_statistics_count_the_work_each_pass_recorded() {
    for isolate_passes in [false, true] {
        let Some(mut h) = harness(GpuTimingConfig {
            pass_spans: true,
            pass_statistics: true,
            isolate_passes,
            pending_commands: 64,
            ..Default::default()
        }) else {
            return;
        };
        let samples = h.frame();
        let triangles = pass(&samples, "triangles").statistics.expect("raster statistics");
        assert_eq!(triangles.input_vertices, u64::from(3 * DRAWS), "isolate={isolate_passes}");
        assert_eq!(triangles.input_primitives, u64::from(DRAWS));
        assert!(triangles.fragment_invocations > 0, "{triangles:?}");
        assert_eq!(triangles.compute_invocations, 0);
        let dispatch = pass(&samples, "dispatch").statistics.expect("compute statistics");
        let threads = u64::from(GROUPS * LOCAL[0] * LOCAL[1] * LOCAL[2]);
        assert_eq!(dispatch.compute_invocations, threads, "isolate={isolate_passes}");
        assert_eq!(dispatch.input_vertices, 0);
        assert_eq!(dispatch.fragment_invocations, 0);
        // Whole-command spans never carry statistics, and nothing stays pending.
        assert!(samples.iter().filter(|s| s.pass_index.is_none()).all(|s| s.statistics.is_none()));
        let stats = h.session.backend.poll_gpu_timings().unwrap().unwrap();
        assert_eq!((stats.pending, stats.capacity_skips), (0, 0));
        h.session.backend.configure_gpu_timing(None).unwrap();
    }
}

#[test]
fn statistics_are_absent_unless_requested() {
    let Some(mut h) = harness(GpuTimingConfig { pass_spans: true, pending_commands: 64, ..Default::default() }) else {
        return;
    };
    let samples = h.frame();
    assert!(samples.iter().any(|s| s.pass_index.is_some()));
    assert!(samples.iter().all(|s| s.statistics.is_none()));
}

#[test]
fn isolation_and_statistics_require_pass_spans() {
    let Some(rig) = Rig::new() else { return };
    let mut backend = VulkanGraphBackend::new(rig.device.clone(), AccessTable::original()).unwrap();
    for config in [
        GpuTimingConfig { isolate_passes: true, ..Default::default() },
        GpuTimingConfig { pass_statistics: true, ..Default::default() },
    ] {
        assert!(backend.configure_gpu_timing(Some(config)).is_err(), "{config:?}");
    }
}

/// Offscreen demonstration: `cargo test -p rhapsody --test gpu_pass_diagnostics
/// -- --ignored --nocapture`. Prints each pass's mean span with and without
/// isolation, plus its statistics. On a software rasterizer the absolute times
/// only show the mechanism; real conclusions need the target GPU.
#[test]
#[ignore = "prints a report; run manually with --nocapture"]
fn print_pass_report() {
    const FRAMES: usize = 20;
    let run = |isolate_passes: bool| -> Option<Vec<(String, f64, Option<rhapsody::rg::GpuPassStatistics>)>> {
        let mut h = harness(GpuTimingConfig {
            pass_spans: true,
            pass_statistics: true,
            isolate_passes,
            pending_commands: 256,
            ..Default::default()
        })?;
        let mut totals: std::collections::BTreeMap<String, (u64, Option<rhapsody::rg::GpuPassStatistics>)> =
            Default::default();
        for frame in 0..FRAMES + 2 {
            let samples = h.frame();
            if frame < 2 {
                continue; // warm-up: pipeline and attachment creation
            }
            for s in samples.iter().filter(|s| s.pass_name.is_some()) {
                let entry = totals.entry(s.pass_name.unwrap().to_owned()).or_default();
                entry.0 += s.duration_ns;
                entry.1 = s.statistics;
            }
        }
        Some(totals.into_iter().map(|(k, (ns, st))| (k, ns as f64 / FRAMES as f64 / 1e6, st)).collect())
    };
    let (Some(plain), Some(isolated)) = (run(false), run(true)) else { return };
    println!("{:<12} {:>12} {:>14} {:>10} {:>10} {:>10} {:>10}", "pass", "span ms", "isolated ms", "in verts", "in prims", "frag", "compute");
    for ((name, ms, stats), (_, iso_ms, _)) in plain.iter().zip(&isolated) {
        let st = stats.unwrap_or_default();
        println!(
            "{:<12} {:>12.4} {:>14.4} {:>10} {:>10} {:>10} {:>10}",
            name, ms, iso_ms, st.input_vertices, st.input_primitives, st.fragment_invocations, st.compute_invocations
        );
    }
}
