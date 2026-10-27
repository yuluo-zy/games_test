//! Hot pipelines held by the caller.
use crate::Error;
use std::sync::Arc;
use super::request::PipelinePoll;
use super::request::PipelineTicket;
use super::shader::VulkanPipeline;

/// Existing draws retain Arc owners and their SyncCell dependencies after a
/// successful publication. Failed/pending replacements keep the old pipeline.
pub struct HotPipeline {
    current: Arc<VulkanPipeline>,
    pending: Option<PipelineTicket>,
    last_error: Option<Arc<Error>>,
    revision: u64,
}

impl HotPipeline {
    pub fn new(current: Arc<VulkanPipeline>) -> Self {
        Self {
            current,
            pending: None,
            last_error: None,
            revision: 0,
        }
    }
    pub fn stage(&mut self, ticket: PipelineTicket) {
        if self
            .pending
            .as_ref()
            .is_some_and(|old| Arc::ptr_eq(&old.completion, &ticket.completion))
        {
            return;
        }
        if let Some(previous) = self.pending.replace(ticket) {
            previous.cancel_unshared_queued();
        }
    }
    pub fn current(&self) -> Arc<VulkanPipeline> {
        self.current.clone()
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn last_error(&self) -> Option<&Arc<Error>> {
        self.last_error.as_ref()
    }
    pub fn publish_at_frame_boundary(&mut self) -> bool {
        if self.current.device.is_device_lost() {
            self.pending = None;
            self.last_error = Some(Arc::new(Error::DeviceLost));
            return false;
        }
        let Some(ticket) = &self.pending else {
            return false;
        };
        match ticket.poll() {
            PipelinePoll::Pending => false,
            PipelinePoll::Failed(error) => {
                self.last_error = Some(error);
                self.pending = None;
                false
            }
            PipelinePoll::Ready(next) => {
                let compatible = Arc::ptr_eq(&next.device, &self.current.device)
                    && next.layout.bindless_abi() == self.current.layout.bindless_abi()
                    && next.point == self.current.point
                    && next.layout.sets.len() == self.current.layout.sets.len()
                    && next
                        .layout
                        .sets
                        .iter()
                        .zip(&self.current.layout.sets)
                        .all(|(a, b)| a.plan == b.plan)
                    && next.layout.push_constants.len() == self.current.layout.push_constants.len()
                    && next
                        .layout
                        .push_constants
                        .iter()
                        .zip(&self.current.layout.push_constants)
                        .all(|(a, b)| {
                            a.offset == b.offset
                                && a.size == b.size
                                && a.stage_flags == b.stage_flags
                        })
                    && next.raster_target_signature() == self.current.raster_target_signature()
                    && next.has_same_shader_interface(&self.current);
                self.pending = None;
                if !compatible {
                    self.last_error = Some(Arc::new(Error::invalid_usage(
                        "hot-reload pipeline interface/target changed",
                    )));
                    return false;
                }
                self.current = next;
                self.last_error = None;
                self.revision += 1;
                true
            }
        }
    }
}
