//! S07 drives the production bounded transfer manager while the normal frame
//! graph displays bytes from its real GPU destination buffer.
use super::fixture::{HEIGHT, WIDTH};
use ash::vk::{self, Handle};
use rhapsody::{
    resource::Buffer, resource::BufferDesc, resource::Texture, rg::AccessRange, rg::BuiltGraph,
    rg::FrameLoop, rg::GpuResource, rg::PassContext, rg::ResourceSpec,
    transfer::QueuedUploadTicket, transfer::ReadbackTicket, transfer::TransferConfig,
    transfer::TransferManager, transfer::UploadKey, transfer::UploadOptions,
    transfer::UploadPriority,
};
use serde::Serialize;
use std::{collections::VecDeque, sync::Arc};

const SLOTS: u32 = 2;
const CHUNK: usize = 1024 * 1024;
const PAYLOAD: u64 = 16 * 1024 * 1024;
const BUDGET: u64 = 8 * 1024 * 1024;
const HOLD_FRAMES: u64 = 4;
const IMAGE_BYTES: usize = (WIDTH * HEIGHT * 4) as usize;

#[derive(Clone, Default, Serialize)]
pub struct TransferReport {
    pub payload_pattern_version: u32,
    pub slots: u32,
    pub chunk_bytes: u64,
    pub payload_buffer_bytes: u64,
    pub staging_budget_bytes: u64,
    pub committed_bytes: u64,
    pub uploaded_bytes: u64,
    pub verified_bytes: u64,
    pub verified_chunks: u64,
    pub completed_payload_rounds: u64,
    pub capacity_stalls: u64,
    pub full_capacity_checks: u64,
    pub capacity_recoveries: u64,
    pub peak_leased_readbacks: u32,
    pub peak_pending_uploads: u32,
    pub peak_pending_readbacks: u32,
    pub try_read_calls: u64,
    pub try_read_pending: u64,
    pub try_read_completed: u64,
    pub minimum_hold_frames: u64,
    pub minimum_observed_readback_delay: Option<u64>,
    pub displayed_buffer_frames: u64,
    pub shutdown_complete: bool,
    pub queued_dirty_requests: u64,
    pub merged_upload_requests: u64,
    pub queued_upload_submissions: u64,
    pub deferred_upload_pumps: u64,
    pub cancelled_upload_jobs: u64,
    pub peak_upload_queue_cpu_bytes: usize,
}
struct PendingRead {
    ticket: ReadbackTicket,
    offset: u64,
    epoch: u64,
    issued_frame: u64,
}
pub struct TransferScene {
    manager: TransferManager,
    destination: Arc<Buffer>,
    token: u64,
    pending: VecDeque<PendingRead>,
    queued: Option<QueuedUploadTicket>,
    upload: Vec<u8>,
    readback: Vec<u8>,
    next_offset: u64,
    epoch: u64,
    display_epoch: u64,
    stalled: bool,
    ready_to_display: bool,
    submissions: rhapsody::sync::SyncSet,
    pub report: TransferReport,
}
impl TransferScene {
    pub fn new(frame_loop: &mut FrameLoop) -> rhapsody::Result<Self> {
        let device = frame_loop.swapchain().device().clone();
        let manager = TransferManager::new(
            device.clone(),
            TransferConfig {
                slots: SLOTS,
                chunk_bytes: CHUNK as u64,
                max_bytes: BUDGET,
            },
        )?;
        let destination = Arc::new(Buffer::new(
            device,
            &BufferDesc::new_gpu_only(
                PAYLOAD,
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )?);
        let token = frame_loop
            .graph
            .backend
            .bind_import(GpuResource::Buffer(destination.clone()))?;
        let mut pending = VecDeque::new();
        pending
            .try_reserve_exact(SLOTS as usize)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        let mut result = Self {
            manager,
            destination,
            token,
            pending,
            queued: None,
            upload: vec![0; CHUNK],
            readback: vec![0; CHUNK],
            next_offset: 0,
            epoch: 1,
            display_epoch: 1,
            stalled: false,
            ready_to_display: false,
            submissions: rhapsody::sync::SyncSet::EMPTY,
            report: TransferReport {
                payload_pattern_version: 1,
                slots: SLOTS,
                chunk_bytes: CHUNK as u64,
                payload_buffer_bytes: PAYLOAD,
                staging_budget_bytes: BUDGET,
                minimum_hold_frames: HOLD_FRAMES,
                ..Default::default()
            },
        };
        result.update_stats()?;
        Ok(result)
    }

    pub fn tick(&mut self, frame: u64) -> rhapsody::Result<()> {
        while self
            .pending
            .front()
            .is_some_and(|read| frame.saturating_sub(read.issued_frame) >= HOLD_FRAMES)
        {
            let read = self.pending.front().expect("pending readback");
            self.report.try_read_calls += 1;
            let Some(bytes) = read.ticket.try_read(&mut self.readback)? else {
                self.report.try_read_pending += 1;
                break;
            };
            if bytes != CHUNK {
                return Err(rhapsody::Error::invalid_usage(
                    "S07 readback returned an unexpected byte count",
                ));
            }
            verify_payload(read.offset, read.epoch, &self.readback)?;
            let delay = frame - read.issued_frame;
            self.report.minimum_observed_readback_delay = Some(
                self.report
                    .minimum_observed_readback_delay
                    .map_or(delay, |old| old.min(delay)),
            );
            self.report.try_read_completed += 1;
            self.report.verified_chunks += 1;
            self.report.verified_bytes += bytes as u64;
            self.pending.pop_front(); // Returning this lease makes actual capacity available again.
        }
        if self.next_offset == PAYLOAD && self.pending.is_empty() {
            self.report.completed_payload_rounds += 1;
            self.epoch += 1;
            self.next_offset = 0;
        }
        while self.pending.len() < SLOTS as usize && self.next_offset < PAYLOAD {
            if self.queued.is_none() {
                fill_payload(self.next_offset, self.epoch, &mut self.upload);
                for part in 0..4 {
                    let start = part * (CHUNK / 4);
                    let ticket = self.manager.enqueue_upload(
                        self.destination.clone(),
                        self.next_offset + start as u64,
                        &self.upload[start..start + CHUNK / 4],
                        UploadOptions {
                            key: UploadKey(self.next_offset / CHUNK as u64),
                            epoch: self.epoch,
                            priority: UploadPriority::Normal,
                        },
                    )?;
                    self.queued = Some(ticket);
                    self.report.queued_dirty_requests += 1;
                }
            }
            self.manager.flush_uploads(1)?;
            let Some(uploaded) = self
                .queued
                .as_ref()
                .expect("accepted dirty upload")
                .try_submitted()?
            else {
                self.stalled = true;
                break;
            };
            self.submissions.insert(uploaded.point);
            self.queued = None;
            let ticket =
                self.manager
                    .read_buffer(self.destination.clone(), self.next_offset, CHUNK)?;
            self.submissions.insert(ticket.completion());
            if self.stalled {
                self.report.capacity_recoveries += 1;
                self.stalled = false;
            }
            self.pending.push_back(PendingRead {
                ticket,
                offset: self.next_offset,
                epoch: self.epoch,
                issued_frame: frame,
            });
            if self.next_offset == 0 {
                self.display_epoch = self.epoch;
                self.ready_to_display = true;
            }
            self.next_offset += CHUNK as u64;
            self.report.uploaded_bytes += CHUNK as u64;
        }
        if self.pending.len() == SLOTS as usize {
            // Leases, including completed ones deliberately held by this
            // consumer, must prevent the mapped slots being recycled.
            match self.manager.read_buffer(self.destination.clone(), 0, CHUNK) {
                Err(rhapsody::Error::Timeout) => {
                    self.report.full_capacity_checks += 1;
                    self.stalled = true;
                }
                Ok(ticket) => {
                    self.submissions.insert(ticket.completion());
                    return Err(rhapsody::Error::invalid_usage(
                        "S07 reused a readback slot whose ticket is still held",
                    ));
                }
                Err(error) => return Err(error),
            }
        }
        self.update_stats()
    }
    fn update_stats(&mut self) -> rhapsody::Result<()> {
        let stats = self.manager.stats()?;
        if stats.committed_bytes > BUDGET || stats.leased_readbacks > SLOTS {
            return Err(rhapsody::Error::invalid_usage(
                "S07 transfer capacity exceeded its configured bound",
            ));
        }
        let queue = self.manager.upload_queue_stats();
        self.report.merged_upload_requests = queue.merged_requests;
        self.report.queued_upload_submissions = queue.submitted_jobs;
        self.report.deferred_upload_pumps = queue.deferred_pumps;
        self.report.cancelled_upload_jobs = queue.cancelled_jobs;
        self.report.peak_upload_queue_cpu_bytes = self
            .report
            .peak_upload_queue_cpu_bytes
            .max(queue.known_cpu_bytes);
        self.report.committed_bytes = stats.committed_bytes;
        self.report.capacity_stalls = stats.capacity_stalls;
        self.report.peak_leased_readbacks = self
            .report
            .peak_leased_readbacks
            .max(stats.leased_readbacks);
        self.report.peak_pending_uploads =
            self.report.peak_pending_uploads.max(stats.pending_uploads);
        self.report.peak_pending_readbacks = self
            .report
            .peak_pending_readbacks
            .max(stats.pending_readbacks);
        Ok(())
    }

    pub fn build_graph(
        &self,
        frame_loop: &mut FrameLoop,
        image: &Arc<Texture>,
        readback: &Arc<Buffer>,
        tokens: [u64; 3],
        first: bool,
    ) -> rhapsody::Result<BuiltGraph> {
        if !self.ready_to_display {
            return Err(rhapsody::Error::Timeout);
        }
        let mut graph = frame_loop.begin_frame()?;
        let source = graph.import_unowned(
            "S07 uploaded GPU payload",
            ResourceSpec::Buffer(self.destination.desc).descriptor(),
            self.token,
            0x37,
        )?;
        let source_id = graph.buffer_id(source)?;
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
        let region = vk::BufferImageCopy::default()
            .image_subresource(
                vk::ImageSubresourceLayers::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .layer_count(1),
            )
            .image_extent(vk::Extent3D {
                width: WIDTH,
                height: HEIGHT,
                depth: 1,
            });
        {
            let buffer = self.destination.clone();
            let image = image.clone();
            let device = buffer.device().clone();
            let mut pass = graph.add_pass("S07 display real transfer destination");
            pass.read_buffer(
                source_id,
                0x25,
                AccessRange::Buffer {
                    offset: 0,
                    size: IMAGE_BYTES as u64,
                },
            )?;
            pass.add_resource_access(output, 0x34, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                unsafe {
                    device.raw_device().cmd_copy_buffer_to_image(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        buffer.raw_handle(),
                        image.raw_handle(),
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &[region],
                    );
                }
                Ok(())
            });
        }
        {
            let image = image.clone();
            let readback = readback.clone();
            let device = image.device().clone();
            let mut pass = graph.add_pass("S07 verify displayed GPU transfer bytes");
            pass.add_resource_access(output, 0x25, 0)?;
            pass.add_resource_access(target, 0x34, 0)?;
            pass.render_with_result_context(move |ctx: PassContext<'_>| {
                unsafe {
                    device.raw_device().cmd_copy_image_to_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        image.raw_handle(),
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        readback.raw_handle(),
                        &[region],
                    );
                }
                Ok(())
            });
        }
        graph.export(output, 0x25)?;
        graph.export(target, 0x26)?;
        Ok(graph.build())
    }
    pub fn display_epoch(&self) -> u64 {
        self.display_epoch
    }
    pub fn verify_snapshot(
        epoch: u64,
        output: &[u8],
        reference: &mut [u8],
    ) -> rhapsody::Result<()> {
        fill_payload(0, epoch, reference);
        if output != reference {
            return Err(rhapsody::Error::invalid_usage(
                "benchmark upload snapshot mismatch",
            ));
        }
        Ok(())
    }
    pub fn verify_display(&mut self, output: &[u8], reference: &mut [u8]) -> rhapsody::Result<()> {
        fill_payload(0, self.display_epoch, reference);
        if output != reference {
            return Err(rhapsody::Error::invalid_usage(
                "S07 displayed pixels differ from the actual uploaded payload",
            ));
        }
        self.report.displayed_buffer_frames += 1;
        Ok(())
    }
    pub fn ensure_completed(&self) -> rhapsody::Result<()> {
        if self.report.completed_payload_rounds == 0
            || self.report.capacity_stalls == 0
            || self.report.full_capacity_checks == 0
            || self.report.capacity_recoveries == 0
            || self.report.try_read_completed < PAYLOAD / CHUNK as u64
            || self.report.peak_leased_readbacks != SLOTS
            || self.report.queued_upload_submissions == 0
            || self.report.merged_upload_requests < 3 * self.report.queued_upload_submissions
            || self.report.queued_dirty_requests < 4 * self.report.queued_upload_submissions
        {
            return Err(rhapsody::Error::invalid_usage(
                "S07 did not complete the large upload, leased-capacity stall, delayed readback and recovery checks",
            ));
        }
        Ok(())
    }
    pub fn shutdown(&mut self, timeout: u64) -> rhapsody::Result<()> {
        if let Some(ticket) = self.queued.take() {
            self.manager.cancel_upload(&ticket)?;
        }
        self.destination.device().wait(&self.submissions, timeout)?;
        self.report.shutdown_complete = true;
        self.update_stats()
    }
}

fn payload_byte(offset: u64, epoch: u64) -> u8 {
    if offset < IMAGE_BYTES as u64 && offset % 4 == 3 {
        return 255;
    }
    let value = offset
        .wrapping_mul(1_664_525)
        .wrapping_add(epoch.wrapping_mul(1_013_904_223));
    (value ^ (value >> 11) ^ (offset >> 19)) as u8
}
fn fill_payload(offset: u64, epoch: u64, output: &mut [u8]) {
    for (index, value) in output.iter_mut().enumerate() {
        *value = payload_byte(offset + index as u64, epoch);
    }
}
fn verify_payload(offset: u64, epoch: u64, bytes: &[u8]) -> rhapsody::Result<()> {
    if let Some((index, _)) = bytes
        .iter()
        .enumerate()
        .find(|&(index, &value)| value != payload_byte(offset + index as u64, epoch))
    {
        return Err(rhapsody::Error::invalid_usage(format!(
            "S07 GPU readback differs at byte {} in payload round {epoch}",
            offset + index as u64
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chunk_reference_preserves_offsets_rounds_and_display_alpha() {
        let mut chunk = vec![0; CHUNK];
        fill_payload(0, 1, &mut chunk);
        verify_payload(0, 1, &chunk).unwrap();
        assert!((3..IMAGE_BYTES).step_by(4).all(|index| chunk[index] == 255));
        assert!(verify_payload(CHUNK as u64, 1, &chunk).is_err());
        assert!(verify_payload(0, 2, &chunk).is_err());
        chunk[CHUNK - 1] ^= 1;
        assert!(verify_payload(0, 1, &chunk).is_err());
    }
}
