//! Linear owner of one acquired image that a render graph renders and the
//! presentation engine then consumes.
//!
//! The graph backend injects the binary acquire wait into the first submitted
//! batch that touches the image and the binary `rendered` signal into the last
//! one, reporting each successful submit here. Every exit path returns the
//! image exactly once: present after `rendered` was signalled, a discard
//! submit + present when only the acquire wait was consumed, or the ordinary
//! unsubmitted cancellation (release/rescue) when nothing reached the GPU.
use crate::present::swapchain::{ACQUIRE_TIMEOUT_NS, AcquiredImageLease, PresentTicket};
use super::lease::consumed;
use super::wsi::wsi_call;
use crate::{
    error::{Error, Result},
    sync::{SyncPoint, SyncSet},
};
use ash::vk;

/// Binary semaphores and fence a graph submission must use for this image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PresentSync {
    /// Waited exactly once, by the first submitted batch touching the image.
    pub acquire: vk::Semaphore,
    /// Signalled exactly once, by the last batch touching the image.
    pub rendered: vk::Semaphore,
    /// Passed to the submit that waits `acquire`; guards the slot's reuse.
    pub acquire_fence: vk::Fence,
}

pub(crate) struct GraphPresentation {
    image: Option<AcquiredImageLease>,
    acquire_point: Option<SyncPoint>,
    rendered_point: Option<SyncPoint>,
    /// Every successful submit that touched the image.
    gpu_use: SyncSet,
}

impl GraphPresentation {
    /// Wait for this image's previous present (so `rendered` may be signalled
    /// again) and reset the slot fence. A failure drops `image`, which runs the
    /// unsubmitted cancellation protocol.
    pub(crate) fn arm(mut image: AcquiredImageLease) -> Result<Self> {
        image.swapchain.policy.allow_present()?;
        if image.lease.is_none() {
            return Err(consumed());
        }
        if let Err(error) = image.presentation.prepare(ACQUIRE_TIMEOUT_NS) {
            image.recover_on_drop &= !error.is_device_lost();
            return Err(error);
        }
        let device = image.swapchain.device.clone();
        let fence = image.sync.fence;
        if let Err(error) = wsi_call(&device.queues.timelines, || unsafe {
            device.core.device.reset_fences(&[fence])
        }) {
            image.recover_on_drop &= !error.is_device_lost();
            return Err(error);
        }
        Ok(Self {
            image: Some(image),
            acquire_point: None,
            rendered_point: None,
            gpu_use: SyncSet::EMPTY,
        })
    }
    fn image(&self) -> Result<&AcquiredImageLease> {
        self.image.as_ref().ok_or_else(consumed)
    }
    pub(crate) fn sync(&self) -> Result<PresentSync> {
        let image = self.image()?;
        Ok(PresentSync {
            acquire: image.sync.acquired,
            rendered: image.presentation.rendered,
            acquire_fence: image.sync.fence,
        })
    }
    pub(crate) fn acquire_consumed(&self) -> bool {
        self.acquire_point.is_some()
    }
    pub(crate) fn rendered_point(&self) -> Option<SyncPoint> {
        self.rendered_point
    }
    /// Publish one successful queue submission of the presentation graph.
    pub(crate) fn submitted(
        &mut self,
        point: SyncPoint,
        waited_acquire: bool,
        signalled_rendered: bool,
    ) -> Result<()> {
        let image = self.image.as_mut().ok_or_else(consumed)?;
        if waited_acquire {
            // The binary acquire wait is consumed: Drop must never wait on it
            // again, and the slot fence now guards its reuse.
            self.acquire_point = Some(point);
            image.recover_on_drop = false;
            image.sync.state.lock().submitted = true;
        }
        // Retirement of the generation (and reuse of `rendered`) waits for
        // every GPU use of the image, on whichever queue it ran.
        self.gpu_use.insert(point);
        image
            .presentation
            .progress
            .lock()
            .record_gpu_submission(point);
        if signalled_rendered {
            self.rendered_point = Some(point);
        }
        Ok(())
    }
    /// Queue the rendered image for presentation.
    pub(crate) fn present(&mut self) -> Result<PresentTicket> {
        let point = self
            .rendered_point
            .ok_or_else(|| Error::invalid_usage("presentation graph has not rendered the image"))?;
        let mut image = self.image.take().ok_or_else(consumed)?;
        image.present_submitted(point, None)
    }
    /// Return the image after a failed, cancelled or incomplete graph, using
    /// the cheapest protocol that keeps every binary semaphore balanced.
    pub(crate) fn abandon(&mut self) -> Result<()> {
        if self.rendered_point.is_some() {
            // The engine owns the content now; presenting returns the image.
            return self.present().map(drop);
        }
        let Some(mut image) = self.image.take() else {
            return Ok(());
        };
        if image.swapchain.device.is_device_lost() {
            image.abandon_device_lost();
            return Err(Error::DeviceLost);
        }
        // Policy-driven return (release, or a cleared frame); a consumed
        // acquire is never waited again.
        image.return_without_content(self.gpu_use, self.acquire_point.is_some(), ACQUIRE_TIMEOUT_NS)
    }
}

impl Drop for GraphPresentation {
    fn drop(&mut self) {
        if self.image.is_some()
            && let Err(error) = self.abandon()
        {
            log::warn!(target: "rhapsody::raw::swapchain", "graph presentation cleanup failed: {error}");
        }
    }
}
