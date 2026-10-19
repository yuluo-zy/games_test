//! S08 reuses the production raster + compute scene and schedules its real
//! graphics draw, compute dispatch and transfer readback on configured queues.
//! Sharing policy changes real intermediate VkImage creation, never just labels.
use super::{
    fixture,
    scene::{RasterScene, RenderingChoice, SceneKind, SceneReport},
};
use ash::vk::Handle;
use rhapsody::{
    device::QueueClass, resource::Buffer, resource::QueueFamilies, resource::ResourceSharing,
    resource::Texture, rg::BuiltGraph, rg::FrameLoop, rg::OriginSource,
};
use serde::Serialize;
use std::{collections::BTreeSet, path::Path, sync::Arc};
#[derive(Clone, Copy, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SharingChoice {
    #[default]
    Exclusive,
    Concurrent,
}
impl SharingChoice {
    pub fn parse(value: &str) -> fixture::Result<Self> {
        match value {
            "exclusive" => Ok(Self::Exclusive),
            "concurrent" => Ok(Self::Concurrent),
            _ => Err("sharing must be exclusive or concurrent".into()),
        }
    }
}
#[derive(Clone, Default, Serialize)]
pub struct MultiQueueReport {
    pub sharing: SharingChoice,
    pub class_queue_indices: Vec<u32>,
    pub class_queue_families: Vec<u32>,
    pub configured_physical_queues: usize,
    pub same_queue_fallback: bool,
    pub concurrent_intermediate_images: usize,
    pub verified_frames: u64,
    pub last_graph_submissions: usize,
    pub peak_graph_submissions: usize,
    pub last_wait_count: usize,
    pub last_physical_queues_used: usize,
    pub submitted_values: Vec<u64>,
    pub rasterized_frames: u64,
    pub postprocessed_frames: u64,
    pub reference_verified_frames: u64,
    pub worst_reference_mismatches: usize,
    pub worst_boundary_pixels: usize,
    pub pipeline_cache_saved: bool,
    pub shutdown_complete: bool,
}
pub struct MultiQueueScene {
    inner: RasterScene,
    inner_report: SceneReport,
    frame: u64,
    pub report: MultiQueueReport,
}
impl MultiQueueScene {
    pub fn new(
        frame_loop: &mut FrameLoop,
        rendering: RenderingChoice,
        cache_path: &Path,
        sharing: SharingChoice,
    ) -> rhapsody::Result<Self> {
        let inner = RasterScene::new(frame_loop, SceneKind::Raster, rendering, cache_path)?;
        let device = &frame_loop.graph.backend.device();
        let queues = QueueClass::ALL.map(|class| device.queue_plan().queue(class).index() as u32);
        let families = queues.map(|index| device.queue_plan().queues[index as usize].family);
        Ok(Self {
            inner,
            inner_report: SceneReport::default(),
            frame: 0,
            report: MultiQueueReport {
                sharing,
                class_queue_indices: queues.to_vec(),
                class_queue_families: families.to_vec(),
                configured_physical_queues: device.queue_plan().queues.len(),
                same_queue_fallback: queues.iter().all(|queue| *queue == queues[0]),
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
        let mut graph = self
            .inner
            .build_graph(frame_loop, image, readback, tokens, frame, first)?;
        let mut found = [false; 3];
        for pass in &mut graph.declaration.passes {
            let (slot, class) = match pass.name.as_ref() {
                "S03 GPU raster triangle" => (0, QueueClass::Graphics),
                "S03 GPU color postprocess" => (1, QueueClass::Compute),
                "S03 readback postprocessed image" => (2, QueueClass::Transfer),
                _ => {
                    return Err(rhapsody::Error::invalid_usage(
                        "S08 encountered an unexpected production scene pass",
                    ));
                }
            };
            pass.queue_class = class;
            found[slot] = true;
        }
        if found != [true; 3] {
            return Err(rhapsody::Error::invalid_usage(
                "S08 did not contain raster, compute and transfer work",
            ));
        }
        if matches!(self.report.sharing, SharingChoice::Concurrent) {
            let families = self
                .report
                .class_queue_families
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            // Vulkan concurrent sharing requires at least two distinct families. Queue
            // aliases degrade to the same exclusive object with no ownership operations.
            if families.len() > 1 {
                let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&families)?);
                let names = graph
                    .declaration
                    .resources
                    .iter()
                    .filter(|resource| matches!(resource.source, OriginSource::Create))
                    .map(|resource| resource.name.clone())
                    .collect::<Vec<_>>();
                self.report.concurrent_intermediate_images = names.len();
                for name in names {
                    frame_loop
                        .graph
                        .backend
                        .set_declared_sharing(&mut graph, &name, sharing)?;
                }
            }
        }
        Ok(graph)
    }
    pub fn verify_display(
        &mut self,
        output: &[u8],
        reference: &mut [u8],
        frame_loop: &FrameLoop,
    ) -> fixture::Result<()> {
        self.inner
            .verify_pixels(self.frame, output, reference, &mut self.inner_report)?;
        self.inner
            .verify_rendering(self.frame, output, &mut self.inner_report)?;
        self.report.verified_frames += 1;
        let captures = &frame_loop.graph.backend.submissions;
        self.report.last_graph_submissions = captures.len();
        self.report.peak_graph_submissions = self.report.peak_graph_submissions.max(captures.len());
        self.report.last_wait_count = captures
            .iter()
            .flat_map(|capture| &capture.batches)
            .map(|batch| batch.wait_semaphores.len())
            .sum();
        self.report.last_physical_queues_used = captures
            .iter()
            .map(|capture| capture.queue.as_raw())
            .collect::<BTreeSet<_>>()
            .len();
        self.report.submitted_values = frame_loop
            .graph
            .backend
            .device()
            .timelines()
            .submitted_set()
            .points()
            .map(|point| point.value)
            .collect();
        self.report.rasterized_frames = self.report.verified_frames;
        self.report.postprocessed_frames = self.report.verified_frames;
        self.report.reference_verified_frames = self.inner_report.reference_verified_frames;
        self.report.worst_reference_mismatches = self.inner_report.worst_reference_mismatches;
        self.report.worst_boundary_pixels = self.inner_report.worst_boundary_pixels;
        if !self.report.same_queue_fallback && self.report.last_physical_queues_used < 2 {
            return Err(
                "S08 configured independent queues but recorded only one actual queue".into(),
            );
        }
        Ok(())
    }
    pub fn ensure_completed(&self) -> rhapsody::Result<()> {
        if self.report.verified_frames == 0 {
            return Err(rhapsody::Error::invalid_usage(
                "S08 did not verify raster/compute/transfer pixels",
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
        self.inner.save_pipeline_cache(&mut self.inner_report)?;
        self.report.pipeline_cache_saved = self.inner_report.pipeline_cache_saved;
        self.report.shutdown_complete = true;
        Ok(())
    }
}
