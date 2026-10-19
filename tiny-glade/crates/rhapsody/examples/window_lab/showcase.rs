//! Sustained GPU workload: four independently verified producers, one GPU mosaic.
use super::{
    fixture::{self, HEIGHT, WIDTH},
    multi_queue_scene::{MultiQueueScene, SharingChoice},
    result_scene::ResultScene,
    scene::{RasterScene, RenderingChoice, SceneKind, SceneReport},
    transfer_scene::TransferScene,
};
use ash::vk::{self, Handle};
use rhapsody::{
    resource::Buffer, resource::BufferDesc, resource::Texture, resource::TextureDesc,
    rg::BuiltGraph, rg::FrameLoop, rg::GpuResource, rg::ResourceSpec,
};
use serde::Serialize;
use std::{path::Path, sync::Arc};
const BYTES: usize = (WIDTH * HEIGHT * 4) as usize;
const TIMEOUT: u64 = 10_000_000_000;
struct Panel {
    image: Arc<Texture>,
    readback: Arc<Buffer>,
    tokens: [u64; 3],
    pixels: Vec<u8>,
}
struct Sample {
    buffer: Arc<Buffer>,
    completion: rhapsody::sync::SyncSet,
    frame: u64,
    upload_epoch: u64,
}
#[derive(Clone, Default, Serialize)]
pub struct ShowcaseReport {
    pub verified_frames: u64,
    pub sampled_frames: u64,
    pub sampling_backpressure: u64,
    pub producer_build_ns: [u64; 4],
    pub producer_submit_ns: [u64; 4],
    pub producer_wait_readback_ns: [u64; 4],
    pub producer_verify_ns: [u64; 4],
    pub producer_frames: [u64; 4],
    pub history_frames: u64,
    pub history_resets: u64,
    pub effect_publications: u64,
    pub uploaded_payload_rounds: u64,
    pub physical_queues: usize,
    pub changed_pixels: u64,
    pub shutdown_complete: bool,
}
pub struct Showcase {
    performance: bool,
    pending_samples: std::collections::VecDeque<Sample>,
    panels: Vec<Panel>,
    temporal: RasterScene,
    queues: MultiQueueScene,
    buildings: ResultScene,
    uploads: TransferScene,
    scene_report: SceneReport,
    reference: Vec<u8>,
    expected: Vec<u8>,
    previous: Vec<u8>,
    pub report: ShowcaseReport,
}
impl Showcase {
    pub fn new(loop_: &mut FrameLoop, cache: &Path) -> rhapsody::Result<Self> {
        let device = loop_.graph.backend.device().clone();
        let mut panels = Vec::new();
        for _ in 0..4 {
            let mut desc = TextureDesc::new_2d(WIDTH, HEIGHT, vk::Format::R8G8B8A8_UNORM);
            desc.usage = vk::ImageUsageFlags::STORAGE
                | vk::ImageUsageFlags::TRANSFER_SRC
                | vk::ImageUsageFlags::TRANSFER_DST;
            let image = Arc::new(Texture::new(device.clone(), &desc)?);
            let readback = Arc::new(Buffer::new(
                device.clone(),
                &BufferDesc::new_gpu_to_cpu(
                    BYTES as u64,
                    vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
                ),
            )?);
            let tokens = [
                0,
                loop_
                    .graph
                    .backend
                    .bind_import(GpuResource::Image(image.clone()))?,
                loop_
                    .graph
                    .backend
                    .bind_import(GpuResource::Buffer(readback.clone()))?,
            ];
            panels.push(Panel {
                image,
                readback,
                tokens,
                pixels: vec![0; BYTES],
            });
        }
        let mut buildings = ResultScene::new(loop_)?;
        buildings.set_continuous(true);
        let mut temporal =
            RasterScene::new(loop_, SceneKind::Temporal, RenderingChoice::Dynamic, cache)?;
        temporal.enable_mesh();
        Ok(Self {
            performance: false,
            pending_samples: std::collections::VecDeque::with_capacity(2),
            temporal,
            queues: MultiQueueScene::new(
                loop_,
                RenderingChoice::Dynamic,
                &cache.with_extension("queues.bin"),
                SharingChoice::Exclusive,
            )?,
            buildings,
            uploads: TransferScene::new(loop_)?,
            panels,
            scene_report: SceneReport::default(),
            reference: vec![0; BYTES],
            expected: vec![0; BYTES],
            previous: vec![0; BYTES],
            report: ShowcaseReport {
                physical_queues: device.queue_plan().queues.len(),
                ..Default::default()
            },
        })
    }
    pub fn set_performance(&mut self, enabled: bool) {
        self.performance = enabled;
        self.temporal.set_performance(enabled);
    }
    pub fn reset_history(&mut self, loop_: &mut FrameLoop) -> rhapsody::Result<()> {
        self.temporal.reset_history(
            loop_,
            &mut self.scene_report,
            "showcase resize/effect reset",
        )?;
        self.report.history_resets += 1;
        Ok(())
    }
    pub fn build_graph(
        &mut self,
        loop_: &mut FrameLoop,
        image: &Arc<Texture>,
        readback: &Arc<Buffer>,
        tokens: [u64; 3],
        first: bool,
        frame: u64,
    ) -> fixture::Result<BuiltGraph> {
        if !self.performance {
            self.temporal
                .advance_pipelines(frame, true, true, None, &mut self.scene_report)?;
        }
        if !self.performance && frame > 0 && frame.is_multiple_of(240) {
            self.reset_history(loop_)?;
        }
        for index in 0..4 {
            let phase = std::time::Instant::now();
            let p = &self.panels[index];
            let built = match index {
                0 => self.temporal.build_graph(
                    loop_,
                    &p.image,
                    &p.readback,
                    p.tokens,
                    frame,
                    first,
                )?,
                1 => {
                    self.queues
                        .build_graph(loop_, &p.image, &p.readback, p.tokens, first, frame)?
                }
                2 => self.buildings.build_graph(
                    loop_,
                    &p.image,
                    &p.readback,
                    p.tokens,
                    first,
                    frame,
                )?,
                _ => {
                    self.uploads.tick(frame)?;
                    self.uploads
                        .build_graph(loop_, &p.image, &p.readback, p.tokens, first)?
                }
            };
            // These are normal production graph submissions. Verification deliberately
            // waits; this correctness player does not claim a throughput benchmark.
            self.report.producer_build_ns[index] += phase.elapsed().as_nanos() as u64;
            let phase = std::time::Instant::now();
            let receipt = loop_.graph.execute_frame(built)?;
            self.report.producer_submit_ns[index] += phase.elapsed().as_nanos() as u64;
            if self.performance {
                self.report.producer_frames[index] += 1;
                continue;
            }
            let phase = std::time::Instant::now();
            loop_.graph.wait_frame(receipt.frame, TIMEOUT)?;
            let p = &mut self.panels[index];
            unsafe {
                p.readback.read_bytes_after_completion(0, &mut p.pixels)?;
            }
            self.report.producer_wait_readback_ns[index] += phase.elapsed().as_nanos() as u64;
            let phase = std::time::Instant::now();
            match index {
                0 => {
                    self.temporal.verify_pixels(
                        frame,
                        &p.pixels,
                        &mut self.reference,
                        &mut self.scene_report,
                    )?;
                    self.temporal
                        .verify_rendering(frame, &p.pixels, &mut self.scene_report)?;
                    self.temporal
                        .verify_history(&p.pixels, &mut self.scene_report)?;
                }
                1 => self
                    .queues
                    .verify_display(&p.pixels, &mut self.reference, loop_)?,
                2 => self
                    .buildings
                    .verify_display(&p.pixels, &mut self.reference)?,
                _ => self
                    .uploads
                    .verify_display(&p.pixels, &mut self.reference)?,
            }
            self.report.producer_verify_ns[index] += phase.elapsed().as_nanos() as u64;
            self.report.producer_frames[index] += 1;
        }
        // Equal-size center crops avoid implementation-dependent nearest ties.
        if !self.performance {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let panel = (y / (HEIGHT / 2) * 2 + x / (WIDTH / 2)) as usize;
                    let sx = x % (WIDTH / 2) + WIDTH / 4;
                    let local_y = y % (HEIGHT / 2);
                    let sy = if panel == 2 {
                        (local_y / 30) * 60 + 15 + local_y % 30
                    } else {
                        local_y + HEIGHT / 4
                    };
                    let src = ((sy * WIDTH + sx) * 4) as usize;
                    let dst = ((y * WIDTH + x) * 4) as usize;
                    self.expected[dst..dst + 4]
                        .copy_from_slice(&self.panels[panel].pixels[src..src + 4]);
                }
            }
        }
        let mut graph = loop_.begin_frame()?;
        let output = graph.import_unowned(
            "showcase composite",
            ResourceSpec::Image(image.desc).descriptor(),
            tokens[1],
            if first { 0 } else { 0x25 },
        )?;
        let target = graph.import_unowned(
            "showcase verified mosaic",
            ResourceSpec::Buffer(readback.desc).descriptor(),
            tokens[2],
            0x26,
        )?;
        let sources = self
            .panels
            .iter()
            .map(|p| {
                graph.import_unowned(
                    "showcase producer",
                    ResourceSpec::Image(p.image.desc).descriptor(),
                    p.tokens[1],
                    0x25,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let device = loop_.graph.backend.device().clone();
        let mut pass = graph.add_pass("showcase GPU four-panel composite");
        for source in &sources {
            pass.add_resource_access(*source, 0x25, 0)?;
        }
        pass.add_resource_access(output, 0x34, 0)?;
        pass.render_with_result_context(move |ctx| {
            let m = ctx.managed.ok_or_else(|| {
                rhapsody::Error::invalid_usage("showcase missing managed resources")
            })?;
            let dst = m.texture_for(output.raw())?;
            let layers = vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1);
            for (index, source) in sources.iter().enumerate() {
                let src = m.texture_for(source.raw())?;
                let x = (index % 2) as i32 * (WIDTH / 2) as i32;
                let y = (index / 2) as i32 * (HEIGHT / 2) as i32;
                for stripe in 0..if index == 2 { 3 } else { 1 } {
                    let source_y = if index == 2 {
                        stripe * 60 + 15
                    } else {
                        (HEIGHT / 4) as i32
                    };
                    let height = if index == 2 { 30 } else { (HEIGHT / 2) as i32 };
                    let target_y = y + if index == 2 { stripe * 30 } else { 0 };
                    let region = vk::ImageBlit::default()
                        .src_subresource(layers)
                        .dst_subresource(layers)
                        .src_offsets([
                            vk::Offset3D {
                                x: (WIDTH / 4) as i32,
                                y: source_y,
                                z: 0,
                            },
                            vk::Offset3D {
                                x: (WIDTH * 3 / 4) as i32,
                                y: source_y + height,
                                z: 1,
                            },
                        ])
                        .dst_offsets([
                            vk::Offset3D {
                                x,
                                y: target_y,
                                z: 0,
                            },
                            vk::Offset3D {
                                x: x + (WIDTH / 2) as i32,
                                y: target_y + height,
                                z: 1,
                            },
                        ]);
                    unsafe {
                        device.raw_device().cmd_blit_image(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            src.raw(),
                            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                            dst.raw(),
                            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                            &[region],
                            vk::Filter::NEAREST,
                        );
                    }
                }
            }
            Ok(())
        });
        let history_buffer = if self.performance {
            self.temporal
                .history_buffer()
                .ok_or("showcase has no history buffer")?
        } else {
            let (history_buffer, history_pixels) = self
                .temporal
                .history_display()
                .ok_or("showcase has no verified history")?;
            for y in 0..HEIGHT / 4 {
                for x in 0..WIDTH / 4 {
                    let src = (((y + HEIGHT / 4) * WIDTH + x + WIDTH / 4) * 4) as usize;
                    let dst = ((y * WIDTH + x) * 4) as usize;
                    self.expected[dst..dst + 4].copy_from_slice(&history_pixels[src..src + 4]);
                }
            }
            history_buffer
        };
        let history = loop_
            .graph
            .import_frame_buffer(&mut graph, history_buffer, 0x26)?;
        let device = loop_.graph.backend.device().clone();
        let mut history_pass = graph.add_pass("showcase visible previous-frame inset");
        history_pass.add_resource_access(history, 0x25, 0)?;
        // Read-modify-write preserves the mosaic outside this inset and keeps
        // its producer alive; GENERAL is valid for this transfer operation.
        history_pass.add_resource_access(output, 0x37, 0)?;
        history_pass.render_with_result_context(move |ctx| {
            let m = ctx.managed.ok_or_else(|| {
                rhapsody::Error::invalid_usage("showcase history resources absent")
            })?;
            let region = vk::BufferImageCopy::default()
                .buffer_offset(((HEIGHT / 4 * WIDTH + WIDTH / 4) * 4) as u64)
                .buffer_row_length(WIDTH)
                .buffer_image_height(HEIGHT)
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(vk::Extent3D {
                    width: WIDTH / 4,
                    height: HEIGHT / 4,
                    depth: 1,
                });
            unsafe {
                device.raw_device().cmd_copy_buffer_to_image(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    m.buffer_for(history.raw())?.raw(),
                    m.texture_for(output.raw())?.raw(),
                    vk::ImageLayout::GENERAL,
                    &[region],
                );
            }
            Ok(())
        });
        let device = loop_.graph.backend.device().clone();
        let mut pass = graph.add_pass("showcase mosaic readback");
        pass.add_resource_access(output, 0x25, 0)?;
        pass.add_resource_access(target, 0x34, 0)?;
        pass.render_with_result_context(move |ctx| {
            let m = ctx.managed.ok_or_else(|| {
                rhapsody::Error::invalid_usage("showcase missing readback resources")
            })?;
            unsafe {
                device.raw_device().cmd_copy_image_to_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    m.texture_for(output.raw())?.raw(),
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    m.buffer_for(target.raw())?.raw(),
                    &[vk::BufferImageCopy::default()
                        .image_subresource(
                            vk::ImageSubresourceLayers::default()
                                .aspect_mask(vk::ImageAspectFlags::COLOR)
                                .layer_count(1),
                        )
                        .image_extent(vk::Extent3D {
                            width: WIDTH,
                            height: HEIGHT,
                            depth: 1,
                        })],
                );
            }
            Ok(())
        });
        graph.export(output, 0x25)?;
        graph.export(target, 0x26)?;
        Ok(graph.build())
    }
    pub fn verify(&mut self, pixels: &[u8]) -> fixture::Result<()> {
        if pixels != self.expected {
            let at = pixels
                .iter()
                .zip(&self.expected)
                .position(|(a, b)| a != b)
                .unwrap();
            return Err(format!(
                "showcase GPU mosaic mismatch byte {at}: got {} expected {}",
                pixels[at], self.expected[at]
            )
            .into());
        }
        self.report.changed_pixels += pixels
            .as_chunks::<4>()
            .0
            .iter()
            .zip(self.previous.as_chunks::<4>().0.iter())
            .filter(|(a, b)| a != b)
            .count() as u64;
        self.previous.copy_from_slice(pixels);
        self.report.verified_frames += 1;
        self.report.history_frames = self.scene_report.history_verified_frames;
        self.report.effect_publications = self.scene_report.shader_frame_boundary_publications;
        self.report.uploaded_payload_rounds = self.uploads.report.completed_payload_rounds;
        Ok(())
    }
    pub fn sample(
        &mut self,
        loop_: &mut FrameLoop,
        readback: &Arc<Buffer>,
        frame: u64,
    ) -> fixture::Result<()> {
        self.collect_samples(loop_, false)?;
        if self.pending_samples.len() == 2 {
            self.report.sampling_backpressure += 1;
            return Ok(());
        }
        let device = loop_.graph.backend.device().clone();
        let buffer = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu((BYTES * 6) as u64, vk::BufferUsageFlags::TRANSFER_DST),
        )?);
        let mut graph = loop_.begin_frame()?;
        let sources = self
            .panels
            .iter()
            .map(|p| p.readback.clone())
            .chain([
                self.temporal
                    .history_buffer()
                    .ok_or("benchmark history absent")?,
                readback.clone(),
            ])
            .map(|b| loop_.graph.import_frame_buffer(&mut graph, b, 0x26))
            .collect::<Result<Vec<_>, _>>()?;
        let target = loop_
            .graph
            .import_frame_buffer(&mut graph, buffer.clone(), 0x26)?;
        let mut pass = graph.add_pass("benchmark sparse asynchronous snapshot");
        for source in &sources {
            pass.add_resource_access(*source, 0x25, 0)?;
        }
        pass.add_resource_access(target, 0x34, 0)?;
        pass.render_with_result_context(move |ctx| {
            let m = ctx.managed.ok_or_else(|| {
                rhapsody::Error::invalid_usage("benchmark snapshot resources absent")
            })?;
            for (index, source) in sources.iter().enumerate() {
                unsafe {
                    device.raw_device().cmd_copy_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        m.buffer_for(source.raw())?.raw(),
                        m.buffer_for(target.raw())?.raw(),
                        &[vk::BufferCopy {
                            src_offset: 0,
                            dst_offset: (index * BYTES) as u64,
                            size: BYTES as u64,
                        }],
                    );
                }
            }
            Ok(())
        });
        graph.export(target, 0x26)?;
        let receipt = loop_.graph.execute_frame(graph.build())?;
        let mut completion = rhapsody::sync::SyncSet::EMPTY;
        for point in receipt.receipts.iter() {
            completion.insert(*point);
        }
        self.pending_samples.push_back(Sample {
            buffer,
            completion,
            frame,
            upload_epoch: self.uploads.display_epoch(),
        });
        Ok(())
    }
    pub fn collect_samples(&mut self, loop_: &mut FrameLoop, drain: bool) -> fixture::Result<()> {
        while let Some(sample) = self.pending_samples.front() {
            let device = &loop_.graph.backend.device();
            if drain {
                device.wait(&sample.completion, TIMEOUT)?;
            } else if !sample.completion.is_reached(&device.poll()?) {
                break;
            }
            let sample = self.pending_samples.pop_front().expect("pending sample");
            let mut bytes = vec![0; BYTES * 6];
            unsafe {
                sample.buffer.read_bytes_after_completion(0, &mut bytes)?;
            }
            let mut reference = vec![0; BYTES];
            for index in 0..2 {
                self.temporal.verify_pixels(
                    sample.frame,
                    &bytes[index * BYTES..(index + 1) * BYTES],
                    &mut reference,
                    &mut self.scene_report,
                )?;
            }
            ResultScene::verify_snapshot(
                sample.frame,
                &bytes[2 * BYTES..3 * BYTES],
                &mut reference,
            )?;
            TransferScene::verify_snapshot(
                sample.upload_epoch,
                &bytes[3 * BYTES..4 * BYTES],
                &mut reference,
            )?;
            if sample.frame > 0 {
                self.temporal.verify_pixels(
                    sample.frame - 1,
                    &bytes[4 * BYTES..5 * BYTES],
                    &mut reference,
                    &mut self.scene_report,
                )?;
            } else if bytes[4 * BYTES..5 * BYTES].iter().any(|v| *v != 0) {
                return Err("benchmark initial history not clear".into());
            }
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let panel = (y / (HEIGHT / 2) * 2 + x / (WIDTH / 2)) as usize;
                    let sx = x % (WIDTH / 2) + WIDTH / 4;
                    let ly = y % (HEIGHT / 2);
                    let sy = if panel == 2 {
                        (ly / 30) * 60 + 15 + ly % 30
                    } else {
                        ly + HEIGHT / 4
                    };
                    let mut src = panel * BYTES + ((sy * WIDTH + sx) * 4) as usize;
                    if x < WIDTH / 4 && y < HEIGHT / 4 {
                        src = 4 * BYTES + (((y + HEIGHT / 4) * WIDTH + x + WIDTH / 4) * 4) as usize;
                    }
                    let dst = 5 * BYTES + ((y * WIDTH + x) * 4) as usize;
                    if bytes[src..src + 4] != bytes[dst..dst + 4] {
                        return Err("benchmark composite snapshot mismatch".into());
                    }
                }
            }
            self.report.sampled_frames += 1;
        }
        Ok(())
    }
    pub fn shutdown(&mut self, loop_: &mut FrameLoop) -> fixture::Result<()> {
        self.collect_samples(loop_, true)?;
        self.uploads.shutdown(TIMEOUT)?;
        self.buildings.shutdown(TIMEOUT)?;
        self.temporal.save_pipeline_cache(&mut self.scene_report)?;
        self.queues.shutdown(
            loop_,
            [0, self.panels[1].tokens[1], self.panels[1].tokens[2]],
            TIMEOUT,
        )?;
        self.report.shutdown_complete = true;
        Ok(())
    }
}
