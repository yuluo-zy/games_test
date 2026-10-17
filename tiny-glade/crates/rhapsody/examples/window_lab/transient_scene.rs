//! S05 carries actual pixels through an image-copy chain. Adjacent attachments
//! overlap in the copy pass; every other attachment may alias only because the
//! next copy already depends on the previous copy's destination.
use super::fixture::{HEIGHT, WIDTH};
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::Buffer,
        device::Device,
        texture::{Texture, TextureDesc},
        transient_backing::TransientBackingCounters,
    },
    rg::{executor::PassContext, frame_loop::FrameLoop, graph::BuiltGraph, vulkan::ResourceSpec},
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Default, Serialize)]
pub struct TransientReport {
    pub committed_scope: &'static str,
    pub aliasing_enabled: bool,
    pub attachments: u32,
    pub attachment_width: u32,
    pub attachment_height: u32,
    pub verified_frames: u64,
    pub current_committed_bytes: u64,
    pub peak_committed_bytes: u64,
    pub current_allocation_records: usize,
    pub peak_allocation_records: usize,
    pub dependency_edge_count: usize,
    pub dependency_edges: Vec<(usize, usize)>,
    pub added_dependency_edges: usize,
    pub planned_backings: usize,
    pub planned_bytes: u64,
    pub unaliased_requirement_bytes: u64,
    pub reuse_bytes: u64,
    pub shared_backing_allocations: u64,
    pub shared_backing_frees: u64,
    pub shared_backing_allocated_bytes: u64,
    pub shared_backing_freed_bytes: u64,
    pub physical_plan_dump: String,
    pub shutdown_complete: bool,
}
pub struct TransientScene {
    device: Arc<Device>,
    counters: Arc<TransientBackingCounters>,
    frame: u64,
    pub report: TransientReport,
}
impl TransientScene {
    pub fn new(
        frame_loop: &mut FrameLoop,
        aliasing: bool,
        attachments: u32,
        scale: u32,
    ) -> rhapsody::Result<Self> {
        if !(3..=64).contains(&attachments) || !(1..=8).contains(&scale) {
            return Err(rhapsody::Error::invalid_usage(
                "S05 requires 3..64 attachments and scale 1..8",
            ));
        }
        frame_loop.graph.backend.set_transient_aliasing(aliasing);
        Ok(Self {
            device: frame_loop.swapchain().device.clone(),
            counters: frame_loop.graph.backend.transient_backing_counters(),
            frame: 0,
            report: TransientReport {
                aliasing_enabled: aliasing,
                attachments,
                attachment_width: WIDTH * scale,
                attachment_height: HEIGHT * scale,
                ..Default::default()
            },
        })
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
        self.frame = frame;
        let mut graph = frame_loop.begin_frame()?;
        let desc = TextureDesc::new_2d(
            self.report.attachment_width,
            self.report.attachment_height,
            vk::Format::R8G8B8A8_UNORM,
        );
        let mut attachments = Vec::with_capacity(self.report.attachments as usize);
        for index in 0..self.report.attachments {
            attachments.push(frame_loop.graph.backend.declare(
                &mut graph,
                &format!("S05 transient attachment {index}"),
                ResourceSpec::Image(desc),
            )?);
        }
        let output = graph.import_unowned(
            "lab.image",
            ResourceSpec::Image(image.desc).descriptor(),
            tokens[1],
            if first { 0 } else { 0x25 },
        )?;
        let target = graph.import_unowned(
            "lab.readback",
            ResourceSpec::Buffer(readback.desc).descriptor(),
            tokens[2],
            0x26,
        )?;
        {
            let first = attachments[0];
            let index = first.raw();
            let device = self.device.clone();
            let color = reference_color(frame).map(|byte| f32::from(byte) / 255.);
            let mut pass = graph.add_pass("S05 initialize first attachment");
            pass.add_resource_access(first, 0x34, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                let managed = ctx.managed.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("S05 managed registry is missing")
                })?;
                let texture = managed.texture_for(index).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("S05 first attachment", error)
                })?;

                unsafe {
                    device.device.cmd_clear_color_image(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        texture.raw(),
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &vk::ClearColorValue { float32: color },
                        &[vk::ImageSubresourceRange::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .level_count(1)
                            .layer_count(1)],
                    );
                }
                Ok(())
            });
        }
        for index in 1..attachments.len() {
            let source = attachments[index - 1];
            let destination = attachments[index];
            let device = self.device.clone();
            let region = image_copy(self.report.attachment_width, self.report.attachment_height);
            let mut pass = graph.add_pass(format!(
                "S05 actual pixel dependency {} to {index}",
                index - 1
            ));
            pass.add_resource_access(source, 0x25, 0)?;
            pass.add_resource_access(destination, 0x34, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                let managed = ctx.managed.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("S05 managed registry is missing")
                })?;
                let source = managed.texture_for(source.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("S05 source attachment", error)
                })?;
                let destination = managed.texture_for(destination.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("S05 destination attachment", error)
                })?;

                unsafe {
                    device.device.cmd_copy_image(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        source.raw(),
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        destination.raw(),
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &[region],
                    );
                }
                Ok(())
            });
        }
        {
            let source = *attachments.last().expect("S05 attachment count validated");
            let image = image.clone();
            let device = self.device.clone();
            let mut pass = graph.add_pass("S05 display final chain attachment");
            pass.add_resource_access(source, 0x25, 0)?;
            pass.add_resource_access(output, 0x34, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                let managed = ctx.managed.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("S05 managed registry is missing")
                })?;
                let source = managed.texture_for(source.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("S05 display source", error)
                })?;

                unsafe {
                    device.device.cmd_copy_image(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        source.raw(),
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        image.handle,
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &[image_copy(WIDTH, HEIGHT)],
                    );
                }
                Ok(())
            });
        }
        {
            let image = image.clone();
            let readback = readback.clone();
            let device = self.device.clone();
            let mut pass = graph.add_pass("S05 verify actual displayed chain pixels");
            pass.add_resource_access(output, 0x25, 0)?;
            pass.add_resource_access(target, 0x34, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                unsafe {
                    device.device.cmd_copy_image_to_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        image.handle,
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        readback.handle,
                        &[vk::BufferImageCopy::default()
                            .image_subresource(color_layers())
                            .image_extent(vk::Extent3D {
                                width: WIDTH,
                                height: HEIGHT,
                                depth: 1,
                            })],
                    );
                }
                Ok(())
            });
        }
        graph.export(output, 0x25)?;
        graph.export(target, 0x26)?;
        Ok(graph.build())
    }
    pub fn verify_display(
        &mut self,
        output: &[u8],
        reference: &mut [u8],
        frame_loop: &FrameLoop,
    ) -> rhapsody::Result<()> {
        if output.len() != (WIDTH * HEIGHT * 4) as usize || reference.len() != output.len() {
            return Err(rhapsody::Error::invalid_usage(
                "S05 output/reference extent differs",
            ));
        }
        let color = reference_color(self.frame);
        for pixel in reference.as_chunks_mut::<4>().0 {
            pixel.copy_from_slice(&color);
        }
        if output != reference {
            return Err(rhapsody::Error::invalid_usage(
                "S05 chain output differs from the independent CPU reference",
            ));
        }
        let plan = frame_loop.graph.backend.transient_memory_plan();
        self.report.dependency_edge_count = plan.dependency_edges.len();
        self.report.dependency_edges = plan.dependency_edges.clone();
        self.report.planned_backings = plan.backings.len();
        self.report.planned_bytes = plan.stats.allocated_bytes;
        self.report.unaliased_requirement_bytes = plan.stats.unaliased_bytes;
        self.report.reuse_bytes = plan.stats.reuse_bytes;
        self.report.physical_plan_dump = plan.dump();
        self.sample();
        self.report.verified_frames += 1;
        Ok(())
    }
    fn sample(&mut self) {
        let memory = self.device.allocator_stats.report();
        self.report.committed_scope = "Resource ranges and dedicated backings only; excludes retained suballocator block slack. See application_gpu_memory for actual native reservations.";
        self.report.current_committed_bytes = memory.totals.total_allocated_bytes;
        self.report.peak_committed_bytes = self
            .report
            .peak_committed_bytes
            .max(memory.totals.total_allocated_bytes);
        self.report.current_allocation_records = memory.allocations.len();
        self.report.peak_allocation_records = self
            .report
            .peak_allocation_records
            .max(memory.allocations.len());
        let backing = self.counters.snapshot();
        self.report.shared_backing_allocations = backing.allocations;
        self.report.shared_backing_frees = backing.frees;
        self.report.shared_backing_allocated_bytes = backing.allocated_bytes;
        self.report.shared_backing_freed_bytes = backing.freed_bytes;
    }
    pub fn ensure_completed(&self) -> rhapsody::Result<()> {
        if self.report.verified_frames == 0
            || (self.report.aliasing_enabled
                && (self.report.reuse_bytes == 0 || self.report.shared_backing_allocations == 0))
        {
            return Err(rhapsody::Error::invalid_usage(
                "S05 did not verify actual pixels and enabled physical memory reuse",
            ));
        }
        Ok(())
    }
    pub fn shutdown(
        &mut self,
        frame_loop: &mut FrameLoop,
        tokens: [u64; 3],
        timeout: u64,
    ) -> rhapsody::Result<()> {
        frame_loop.graph.wait_submits_done(timeout)?;
        frame_loop.graph.collect_after_completion(&tokens)?;
        self.device.try_collect_all_completed()?;
        self.sample();
        if self.report.shared_backing_allocations != self.report.shared_backing_frees
            || self.report.shared_backing_allocated_bytes != self.report.shared_backing_freed_bytes
        {
            return Err(rhapsody::Error::invalid_usage(
                "S05 shared backing owners remain after completed-frame cleanup",
            ));
        }
        self.report.shutdown_complete = true;
        Ok(())
    }
}
fn color_layers() -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .layer_count(1)
}
fn image_copy(width: u32, height: u32) -> vk::ImageCopy {
    vk::ImageCopy::default()
        .src_subresource(color_layers())
        .dst_subresource(color_layers())
        .extent(vk::Extent3D {
            width,
            height,
            depth: 1,
        })
}
fn reference_color(frame: u64) -> [u8; 4] {
    [
        (frame.wrapping_mul(37).wrapping_add(17) % 256) as u8,
        (frame.wrapping_mul(67).wrapping_add(91) % 256) as u8,
        (frame.wrapping_mul(13).wrapping_add(43) % 256) as u8,
        255,
    ]
}
