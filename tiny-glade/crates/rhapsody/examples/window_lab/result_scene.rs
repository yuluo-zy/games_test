//! S06 uses versioned building silhouettes, shadows and glyphs as deterministic
//! geometry fixtures. Persistent GPU buffers are the cached results; each miss
//! records real transfer draws, and each hit omits those producer passes.
use super::fixture::{HEIGHT, WIDTH};
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        device::Device,
        texture::Texture,
    },
    rg::{
        executor::PassContext,
        frame_loop::FrameLoop,
        graph::{BuiltGraph, RenderGraph},
        resource::{AccessRange, ResourceHandle},
        result_cache::{
            ContentDependency, DependencyId, ResultDependencies, ResultNodeId, ResultRequest,
        },
        vulkan::ResourceSpec,
    },
};
use serde::Serialize;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

const READ: u8 = 0x25;
const WRITE: u8 = 0x34;
const HOST: u8 = 0x26;
const PANEL_HEIGHT: u32 = HEIGHT / 3;
const PANEL_BYTES: u64 = (WIDTH * PANEL_HEIGHT * 4) as u64;
const GEOMETRY: u64 = 0x6001;
const MATERIAL: u64 = 0x6002;
const LIGHT: u64 = 0x6003;
const TEXT: u64 = 0x6004;
const VIEW: u64 = 0x6005;

#[derive(Clone, Default, Serialize)]
pub struct ResultSceneReport {
    pub fixture: &'static str,
    pub verified_frames: u64,
    pub requested_hits: [u64; 3],
    pub requested_misses: [u64; 3],
    pub producer_recordings: [u64; 3],
    pub cached_completion_checks: u64,
    pub pixel_mismatches: u64,
    pub geometry_changed_frames: u64,
    pub light_changed_frames: u64,
    pub text_changed_frames: u64,
    pub implementation_changed_frames: u64,
    pub view_changed_frames: u64,
    pub material_changed_frames: u64,
    pub cache_entries: usize,
    pub cache_bytes: usize,
    pub invalidated_entries: u64,
    pub publications: u64,
    pub whole_plan_hits: u64,
    pub whole_plan_compilations: u64,
    pub subgraph_plan_hits: u64,
    pub subgraph_plan_compilations: u64,
    pub shutdown_complete: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Versions {
    geometry: u64,
    material: u64,
    light: u64,
    text: u64,
    implementation: u64,
    view: u64,
}
impl Versions {
    fn at(frame: u64) -> Self {
        Self {
            geometry: u64::from(frame >= 60),
            light: u64::from(frame >= 120),
            text: u64::from(frame >= 180),
            implementation: u64::from(frame >= 240),
            view: u64::from(frame >= 300),
            material: u64::from(frame >= 320),
        }
    }
    fn external(self) -> [(u64, u64); 5] {
        [
            (GEOMETRY, self.geometry),
            (MATERIAL, self.material),
            (LIGHT, self.light),
            (TEXT, self.text),
            (VIEW, self.view),
        ]
    }
}
#[derive(Clone, Copy)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: [u8; 4],
}
impl Rect {
    #[cfg(test)]
    fn contains(self, x: u32, y: u32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}
pub struct ResultScene {
    cache_policy: [bool; 3],
    device: Arc<Device>,
    calls: Arc<[AtomicU64; 3]>,
    versions: Option<Versions>,
    frame: u64,
    continuous: bool,
    reference_version: Option<Versions>,
    reference: Vec<u8>,
    hits: [bool; 3],
    before: [u64; 3],
    pub report: ResultSceneReport,
}
impl ResultScene {
    pub fn new(frame_loop: &mut FrameLoop) -> rhapsody::Result<Self> {
        frame_loop
            .graph
            .set_result_cache_limits(16, 8 * 1024 * 1024);
        Ok(Self {
            cache_policy: [true; 3],
            device: frame_loop.swapchain().device().clone(),
            calls: Arc::new(std::array::from_fn(|_| AtomicU64::new(0))),
            versions: None,
            frame: 0,
            continuous: false,
            reference_version: None,
            reference: vec![0; (WIDTH * HEIGHT * 4) as usize],
            hits: [false; 3],
            before: [0; 3],
            report: ResultSceneReport {
                fixture: "versioned GPU building silhouettes / geometry-dependent shadows / glyphs",
                ..Default::default()
            },
        })
    }
    pub fn verify_snapshot(
        frame: u64,
        output: &[u8],
        reference: &mut [u8],
    ) -> rhapsody::Result<()> {
        reference_pixels(continuous_versions(frame), reference);
        if output != reference {
            return Err(rhapsody::Error::invalid_usage(
                "benchmark building snapshot mismatch",
            ));
        }
        Ok(())
    }
    pub fn set_continuous(&mut self, enabled: bool) {
        self.continuous = enabled;
    }
    pub fn build_graph(
        &mut self,
        frame_loop: &mut FrameLoop,
        image: &Arc<Texture>,
        readback: &Arc<Buffer>,
        tokens: [u64; 3],
        first: bool,
        frame: u64,
    ) -> rhapsody::Result<BuiltGraph> {
        let versions = if self.continuous {
            continuous_versions(frame)
        } else {
            Versions::at(frame)
        };
        if let Some(previous) = self.versions {
            for ((id, old), (_, new)) in previous.external().into_iter().zip(versions.external()) {
                if old != new {
                    frame_loop
                        .graph
                        .invalidate_result_dependency(DependencyId::External(id), new);
                }
            }
        }
        self.frame = frame;
        self.versions = Some(versions);
        self.before = std::array::from_fn(|index| self.calls[index].load(Ordering::SeqCst));
        let mut graph = frame_loop.begin_frame()?;
        let spec = ResourceSpec::Buffer(BufferDesc::new_gpu_only(
            PANEL_BYTES,
            vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
        ));
        let mesh = frame_loop.graph.cached_result(
            &mut graph,
            request(
                1,
                vec![
                    ContentDependency::external(GEOMETRY, versions.geometry),
                    ContentDependency::external(MATERIAL, versions.material),
                    ContentDependency::external(VIEW, versions.view),
                ],
                versions.view.to_le_bytes().to_vec(),
                1,
            ),
            spec,
            READ,
            {
                let device = self.device.clone();
                let calls = self.calls.clone();
                move |_session, graph, output| {
                    paint(graph, device, calls, 0, output, None, versions)
                }
            },
        )?;
        let shadow = frame_loop.graph.cached_result(
            &mut graph,
            request(
                2,
                vec![
                    mesh.dependency(),
                    ContentDependency::external(LIGHT, versions.light),
                ],
                Vec::new(),
                1,
            ),
            spec,
            READ,
            {
                let device = self.device.clone();
                let calls = self.calls.clone();
                let source = mesh.handle;
                move |_session, graph, output| {
                    paint(graph, device, calls, 1, output, Some(source), versions)
                }
            },
        )?;
        let text = frame_loop.graph.cached_result(
            &mut graph,
            request(
                3,
                vec![ContentDependency::external(TEXT, versions.text)],
                Vec::new(),
                1 + versions.implementation,
            ),
            spec,
            READ,
            {
                let device = self.device.clone();
                let calls = self.calls.clone();
                move |_session, graph, output| {
                    paint(graph, device, calls, 2, output, None, versions)
                }
            },
        )?;
        let results = [mesh, shadow, text];
        self.hits = std::array::from_fn(|index| results[index].hit);
        for result in &results {
            if result.hit {
                if result.produced.is_empty() {
                    return Err(rhapsody::Error::invalid_usage(
                        "S06 cached output lost its real producer completion",
                    ));
                }
                self.report.cached_completion_checks += 1;
            }
        }
        let output = graph.import_unowned(
            "S06 displayed image",
            ResourceSpec::Image(image.desc).descriptor(),
            tokens[1],
            if first { 0 } else { READ },
        )?;
        let target = graph.import_unowned(
            "S06 pixel readback",
            ResourceSpec::Buffer(readback.desc).descriptor(),
            tokens[2],
            HOST,
        )?;
        graph.subgraph("S06 compose persistent outputs and read pixels", |scope| {
            for result in &results {
                scope.input(result.handle, AccessRange::Whole, READ)?;
            }
            {
                let image = image.clone();
                let device = self.device.clone();
                let handles = results.map(|result| result.handle);
                let mut pass = scope.graph().add_pass("S06 compose cached GPU panels");
                for handle in handles {
                    pass.add_resource_access(handle, READ, 0)?;
                }
                pass.add_resource_access(output, WRITE, 0)?;
                pass.render_with_result_context(move |ctx| {
                    let managed = ctx.managed.ok_or_else(|| {
                        rhapsody::Error::invalid_usage("S06 requires managed resources")
                    })?;

                    for (index, handle) in handles.into_iter().enumerate() {
                        let source = managed
                            .buffer_for(handle.raw())
                            .map_err(|error| {
                                rhapsody::Error::invalid_usage_with(
                                    "S06 managed buffer binding",
                                    error,
                                )
                            })?
                            .clone();

                        unsafe {
                            device.raw_device().cmd_copy_buffer_to_image(
                                vk::CommandBuffer::from_raw(ctx.command.handle()),
                                source.raw(),
                                image.raw_handle(),
                                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                                &[panel_region(index as u32)],
                            );
                        }
                    }
                    Ok(())
                });
            }
            {
                let image = image.clone();
                let readback = readback.clone();
                let device = self.device.clone();
                let mut pass = scope.graph().add_pass("S06 read actual composite pixels");
                pass.add_resource_access(output, READ, 0)?;
                pass.add_resource_access(target, WRITE, 0)?;
                pass.render_with_result_context(move |ctx| {
                    unsafe {
                        device.raw_device().cmd_copy_image_to_buffer(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            image.raw_handle(),
                            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                            readback.raw_handle(),
                            &[full_region()],
                        );
                    }
                    Ok(())
                });
            }
            scope.output(output, AccessRange::Whole, READ)?;
            scope.output(target, AccessRange::Whole, HOST)?;
            Ok(())
        })?;
        graph.export(output, READ)?;
        graph.export(target, HOST)?;
        // Stats are sampled again on the next build, after this graph compiled.
        let cache = frame_loop.graph.result_cache_stats();
        self.report.cache_entries = cache.entries;
        self.report.cache_bytes = cache.bytes;
        self.report.invalidated_entries = cache.invalidations;
        self.report.publications = cache.publications;
        let plans = frame_loop.graph.chain.plan_cache_stats();
        self.report.whole_plan_hits = plans.hits;
        self.report.whole_plan_compilations = plans.insertions;
        let local = frame_loop.graph.chain.subgraph_cache_stats();
        self.report.subgraph_plan_hits = local.hits;
        self.report.subgraph_plan_compilations = local.compilations;
        Ok(graph.build())
    }
    pub fn verify_display(&mut self, output: &[u8], reference: &mut [u8]) -> rhapsody::Result<()> {
        let versions = self.versions.expect("S06 graph built before readback");
        if self.reference_version != Some(versions) {
            reference_pixels(versions, &mut self.reference);
            self.reference_version = Some(versions);
        }
        reference.copy_from_slice(&self.reference);
        let mismatches = output
            .iter()
            .zip(reference.iter())
            .filter(|(a, b)| a != b)
            .count();
        self.report.pixel_mismatches += mismatches as u64;
        if mismatches != 0 || output.len() != reference.len() {
            return Err(rhapsody::Error::invalid_usage(format!(
                "S06 cached pixels differ at {mismatches} bytes on frame {}",
                self.frame
            )));
        }
        for index in 0..3 {
            let recorded = self.calls[index].load(Ordering::SeqCst);
            if recorded - self.before[index] != u64::from(!self.hits[index]) {
                return Err(rhapsody::Error::invalid_usage(
                    "S06 producer callback did not match actual hit/miss result",
                ));
            }
            self.report.producer_recordings[index] = recorded;
            self.report.requested_hits[index] += u64::from(self.hits[index]);
            self.report.requested_misses[index] += u64::from(!self.hits[index]);
        }
        self.report.verified_frames += 1;
        self.report.geometry_changed_frames += versions.geometry;
        self.report.light_changed_frames += versions.light;
        self.report.text_changed_frames += versions.text;
        self.report.implementation_changed_frames += versions.implementation;
        self.report.view_changed_frames += versions.view;
        self.report.material_changed_frames += versions.material;
        Ok(())
    }
    pub fn set_cache_policy(&mut self, plan: bool, subgraph: bool, result: bool) {
        self.cache_policy = [plan, subgraph, result];
    }
    pub fn ensure_completed(&self) -> rhapsody::Result<()> {
        let [plan_cache, subgraph_cache, result_cache] = self.cache_policy;
        let expected_misses = if result_cache {
            [4, 5, 3]
        } else {
            [self.report.verified_frames; 3]
        };
        if self.report.verified_frames < 360
            || self.report.material_changed_frames == 0
            || self.report.pixel_mismatches != 0
            || (result_cache && self.report.cached_completion_checks == 0)
            || (result_cache && self.report.requested_hits.contains(&0))
            || self.report.requested_misses != expected_misses
            || self.report.producer_recordings != self.report.requested_misses
            || (result_cache && self.report.invalidated_entries == 0)
            || (plan_cache && self.report.whole_plan_hits == 0)
            || (subgraph_cache && self.report.subgraph_plan_hits == 0)
        {
            return Err(rhapsody::Error::invalid_usage(
                "S06 did not verify all local changes, producer skips, plan reuse and persistent-result pixels",
            ));
        }
        Ok(())
    }
    pub fn shutdown(&mut self, _timeout_ns: u64) -> rhapsody::Result<()> {
        self.report.shutdown_complete = true;
        Ok(())
    }
}
fn request(
    node: u64,
    dependencies: Vec<ContentDependency>,
    parameters: Vec<u8>,
    implementation: u64,
) -> ResultRequest {
    ResultRequest {
        node: ResultNodeId(0x6000 + node),
        dependencies: ResultDependencies::Complete(dependencies),
        parameters,
        shader_implementation: implementation,
        shader_interface: 1,
    }
}
fn paint(
    graph: &mut RenderGraph,
    device: Arc<Device>,
    calls: Arc<[AtomicU64; 3]>,
    index: usize,
    output: ResourceHandle,
    source: Option<ResourceHandle>,
    versions: Versions,
) -> rhapsody::Result<()> {
    graph.subgraph(format!("S06 persistent panel {index}"), |scope| {
        if let Some(source) = source {
            scope.input(source, AccessRange::Whole, READ)?;
        }
        scope.interface_mut().implementation_signature = if index == 2 {
            1 + versions.implementation
        } else {
            1
        };
        {
            let mut pass = scope
                .graph()
                .add_pass("S06 generate versioned geometry pixels");
            if let Some(source) = source {
                pass.add_resource_access(source, READ, 0)?;
            }
            pass.add_resource_access(output, WRITE, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                calls[index].fetch_add(1, Ordering::SeqCst);
                let managed = ctx.managed.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("S06 requires managed resources")
                })?;
                let target = managed
                    .buffer_for(output.raw())
                    .map_err(|error| {
                        rhapsody::Error::invalid_usage_with("S06 managed buffer binding", error)
                    })?
                    .clone();

                let command = vk::CommandBuffer::from_raw(ctx.command.handle());
                unsafe {
                    if let Some(source) = source {
                        let source = managed
                            .buffer_for(source.raw())
                            .map_err(|error| {
                                rhapsody::Error::invalid_usage_with(
                                    "S06 managed buffer binding",
                                    error,
                                )
                            })?
                            .clone();

                        device.raw_device().cmd_copy_buffer(
                            command,
                            source.raw(),
                            target.raw(),
                            &[vk::BufferCopy {
                                src_offset: 0,
                                dst_offset: 0,
                                size: PANEL_BYTES,
                            }],
                        );
                    } else {
                        device.raw_device().cmd_fill_buffer(
                            command,
                            target.raw(),
                            0,
                            PANEL_BYTES,
                            u32::from_ne_bytes(background(index)),
                        );
                    }
                    // Consecutive paint layers overlap intentionally; Vulkan
                    // transfer WAW ordering is explicit inside this one pass.
                    for layer in layers(index, versions) {
                        device.raw_device().cmd_pipeline_barrier(
                            command,
                            vk::PipelineStageFlags::TRANSFER,
                            vk::PipelineStageFlags::TRANSFER,
                            vk::DependencyFlags::empty(),
                            &[vk::MemoryBarrier::default()
                                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                                .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)],
                            &[],
                            &[],
                        );
                        for rect in layer {
                            for y in rect.y..rect.y + rect.height {
                                device.raw_device().cmd_fill_buffer(
                                    command,
                                    target.raw(),
                                    u64::from((y * WIDTH + rect.x) * 4),
                                    u64::from(rect.width * 4),
                                    u32::from_ne_bytes(rect.color),
                                );
                            }
                        }
                    }
                }
                Ok(())
            });
        }
        scope.output(output, AccessRange::Whole, READ)?;
        Ok(())
    })
}
fn background(index: usize) -> [u8; 4] {
    if index == 2 {
        [24, 31, 49, 255]
    } else {
        [126, 181, 203, 255]
    }
}
fn buildings(v: Versions) -> Vec<Rect> {
    (0..6)
        .map(|index| {
            let height = 17 + (index * 7 + v.geometry as u32 * 11) % 26;
            Rect {
                x: 8 + index * 50 + v.view as u32 * 3,
                y: 52 - height,
                width: 34,
                height,
                color: if v.material == 0 {
                    [201, 148, 91, 255]
                } else {
                    [157, 183, 125, 255]
                },
            }
        })
        .collect()
}
fn layers(index: usize, v: Versions) -> Vec<Vec<Rect>> {
    if index == 1 {
        return vec![
            buildings(v)
                .into_iter()
                .map(|building| Rect {
                    x: building.x + building.width - 3,
                    y: building.y + 5,
                    width: 6 + v.light as u32 * 5,
                    height: building.height - 5,
                    color: [55, 69, 82, 255],
                })
                .collect(),
        ];
    }
    if index == 0 {
        let buildings = buildings(v);
        let windows = buildings
            .iter()
            .flat_map(|building| {
                (building.y + 5..49).step_by(7).flat_map(move |y| {
                    (building.x + 5..building.x + 28)
                        .step_by(9)
                        .map(move |x| Rect {
                            x,
                            y,
                            width: 4,
                            height: 4,
                            color: [250, 230, 157, 255],
                        })
                })
            })
            .collect();
        return vec![
            vec![Rect {
                x: 0,
                y: 52,
                width: WIDTH,
                height: PANEL_HEIGHT - 52,
                color: [82, 115, 82, 255],
            }],
            buildings,
            windows,
        ];
    }
    let mut rects = Vec::new();
    let glyphs: [[u8; 7]; 4] = [
        [30, 17, 17, 30, 20, 18, 17], // R
        [17, 17, 17, 31, 17, 17, 17], // H
        [14, 17, 17, 31, 17, 17, 17], // A
        if v.text == 0 {
            [4, 12, 4, 4, 4, 4, 14]
        } else {
            [14, 17, 1, 2, 4, 8, 31]
        },
    ];
    for (glyph, rows) in glyphs.into_iter().enumerate() {
        for (row, bits) in rows.into_iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    rects.push(Rect {
                        x: 80 + glyph as u32 * 40 + col * 5,
                        y: 12 + row as u32 * 5,
                        width: 5,
                        height: 5,
                        color: if v.implementation == 0 {
                            [232, 214, 153, 255]
                        } else {
                            [148, 226, 220, 255]
                        },
                    });
                }
            }
        }
    }
    vec![rects]
}
// Independent CPU rectangle painter: clipped rows in painter order, without
// GPU command offsets or readback data. Work scales with covered spans rather
// than testing every rectangle against every framebuffer pixel.
fn reference_pixels(v: Versions, output: &mut [u8]) {
    let mesh = layers(0, v);
    let shadows = layers(1, v);
    let text = layers(2, v);
    for panel in 0..3 {
        let start = panel * PANEL_BYTES as usize;
        let pixels = &mut output[start..start + PANEL_BYTES as usize];
        for pixel in pixels.as_chunks_mut::<4>().0 {
            *pixel = background(panel);
        }
        let geometry = if panel == 2 { &text } else { &mesh };
        let rectangles = geometry
            .iter()
            .flatten()
            .chain(shadows.iter().flatten().filter(|_| panel == 1));
        for rect in rectangles {
            for y in rect.y.min(PANEL_HEIGHT)..rect.y.saturating_add(rect.height).min(PANEL_HEIGHT)
            {
                let left = rect.x.min(WIDTH);
                let right = rect.x.saturating_add(rect.width).min(WIDTH);
                let begin = ((y * WIDTH + left) * 4) as usize;
                let end = ((y * WIDTH + right) * 4) as usize;
                for pixel in pixels[begin..end].as_chunks_mut::<4>().0 {
                    *pixel = rect.color;
                }
            }
        }
    }
}
#[cfg(test)]
fn reference_pixels_slow(v: Versions, output: &mut [u8]) {
    // Evaluate geometric coverage independently for every pixel; do not reuse
    // GPU command offsets, packed fill words, or read-back producer buffers.
    let mesh = layers(0, v);
    let shadow = layers(1, v);
    let text = layers(2, v);
    for (pixel, rgba) in output.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        let x = pixel as u32 % WIDTH;
        let global_y = pixel as u32 / WIDTH;
        let panel = (global_y / PANEL_HEIGHT) as usize;
        let y = global_y % PANEL_HEIGHT;
        let mut color = background(panel);
        let geometry = if panel == 2 { &text } else { &mesh };
        for rect in geometry.iter().flatten() {
            if rect.contains(x, y) {
                color = rect.color;
            }
        }
        if panel == 1 {
            for rect in shadow.iter().flatten() {
                if rect.contains(x, y) {
                    color = rect.color;
                }
            }
        }
        rgba.copy_from_slice(&color);
    }
}
fn panel_region(panel: u32) -> vk::BufferImageCopy {
    vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_offset(vk::Offset3D {
            x: 0,
            y: (panel * PANEL_HEIGHT) as i32,
            z: 0,
        })
        .image_extent(vk::Extent3D {
            width: WIDTH,
            height: PANEL_HEIGHT,
            depth: 1,
        })
}
fn full_region() -> vk::BufferImageCopy {
    vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: WIDTH,
            height: HEIGHT,
            depth: 1,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn span_reference_matches_independent_pixel_coverage_for_all_version_combinations() {
        for mask in 0..64 {
            let v = Versions {
                geometry: mask & 1,
                light: (mask >> 1) & 1,
                text: (mask >> 2) & 1,
                implementation: (mask >> 3) & 1,
                view: (mask >> 4) & 1,
                material: (mask >> 5) & 1,
            };
            let mut slow = vec![0; (WIDTH * HEIGHT * 4) as usize];
            let mut fast = slow.clone();
            reference_pixels_slow(v, &mut slow);
            reference_pixels(v, &mut fast);
            assert_eq!(slow, fast, "version mask {mask}");
        }
    }
    #[test]
    fn local_changes_touch_only_their_geometric_dependencies() {
        let mut previous = vec![0; (WIDTH * HEIGHT * 4) as usize];
        reference_pixels(Versions::at(0), &mut previous);
        for (frame, changed) in [
            (60, [true, true, false]),
            (120, [false, true, false]),
            (180, [false, false, true]),
            (240, [false, false, true]),
            (300, [true, true, false]),
            (320, [true, true, false]),
        ] {
            let mut next = previous.clone();
            reference_pixels(Versions::at(frame), &mut next);
            for (index, expected_change) in changed.into_iter().enumerate() {
                let range = index * PANEL_BYTES as usize..(index + 1) * PANEL_BYTES as usize;
                assert_eq!(
                    previous[range.clone()] != next[range],
                    expected_change,
                    "frame={frame} panel={index}"
                );
            }
            assert!(next.as_chunks::<4>().0.iter().all(|pixel| pixel[3] == 255));
            previous = next;
        }
    }
}

fn continuous_versions(frame: u64) -> Versions {
    Versions {
        geometry: (frame / 30) % 2,
        light: (frame / 45) % 2,
        text: (frame / 60) % 2,
        implementation: (frame / 150) % 2,
        view: (frame / 90) % 2,
        material: (frame / 120) % 2,
    }
}
