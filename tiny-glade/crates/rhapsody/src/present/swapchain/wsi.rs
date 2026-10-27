//! WSI call wrappers and present-result classification.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::device::Device;

// All fallible WSI device calls share the submission runtime's sticky loss
// state. Inspecting a VkResult only at the FrameLoop boundary is too late:
// acquired-image Drop may otherwise attempt a rescue submit while unwinding.
pub(super) fn wsi_call<A: crate::sync::timeline::TimelineApi, T, E: Into<Error>>(
    timelines: &crate::sync::timeline::Timelines<A>,
    call: impl FnOnce() -> std::result::Result<T, E>,
) -> Result<T> {
    if timelines.is_device_lost() {
        return Err(Error::DeviceLost);
    }
    let result = call().map_err(Into::into);
    if let Err(error) = &result
        && error.is_device_lost()
    {
        timelines.mark_device_lost();
    }
    result
}

/// The fence boundary is shared by the live WSI state machine and CPU fault
/// injection. Failed waits must leave ownership intact and prevent any reset.
pub(super) trait WsiFenceOperations {
    fn wait(&self, fence: vk::Fence, timeout: u64) -> Result<()>;
    fn reset(&self, fence: vk::Fence) -> Result<()>;
    fn signaled(&self, fence: vk::Fence) -> Result<bool>;
}

pub(super) struct DeviceFenceOperations<'a>(pub(super) &'a Device);

impl WsiFenceOperations for DeviceFenceOperations<'_> {
    fn wait(&self, fence: vk::Fence, timeout: u64) -> Result<()> {
        wsi_call(&self.0.queues.timelines, || unsafe {
            self.0.core.device.wait_for_fences(&[fence], true, timeout)
        })
    }
    fn reset(&self, fence: vk::Fence) -> Result<()> {
        wsi_call(&self.0.queues.timelines, || unsafe {
            self.0.core.device.reset_fences(&[fence])
        })
    }
    fn signaled(&self, fence: vk::Fence) -> Result<bool> {
        wsi_call(&self.0.queues.timelines, || unsafe {
            self.0.core.device.get_fence_status(fence)
        })
    }
}

/// Whether the present's queue operation (and therefore its semaphore wait)
/// was enqueued. Per vkQueuePresentKHR, OUT_OF_DATE, SURFACE_LOST and
/// FULL_SCREEN_EXCLUSIVE_MODE_LOST still enqueue the wait.
pub(super) fn present_was_enqueued(result: std::result::Result<bool, vk::Result>) -> bool {
    matches!(
        result,
        Ok(_)
            | Err(vk::Result::ERROR_OUT_OF_DATE_KHR
                | vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
                | vk::Result::ERROR_SURFACE_LOST_KHR)
    )
}

/// Whether a present fence is guaranteed to signal for this result. The
/// maintenance1 specification does not guarantee a signal for presents that
/// the presentation engine rejected, and drivers have been observed to leave
/// those fences unsignaled; such fences are only polled, never waited on.
pub(super) fn present_fence_is_certain(result: std::result::Result<bool, vk::Result>) -> bool {
    result.is_ok()
}

/// Maps a present result to `(suboptimal, out_of_date)`. SUBOPTIMAL does not
/// by itself retire the generation (see [`Swapchain::note_suboptimal`]);
/// SURFACE_LOST is a failure.
pub(super) fn present_result(result: std::result::Result<bool, vk::Result>) -> Result<(bool, bool)> {
    match result {
        Ok(suboptimal) => Ok((suboptimal, false)),
        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok((false, true)),
        Err(vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT) => Ok((false, true)),
        Err(error) => Err(error.into()),
    }
}

/// Whether a SUBOPTIMAL generation must be rebuilt: only when the surface's
/// fixed current extent or its transform differs from the created chain.
pub(super) fn suboptimal_requires_recreate(
    caps: &vk::SurfaceCapabilitiesKHR,
    extent: vk::Extent2D,
    transform: vk::SurfaceTransformFlagsKHR,
) -> bool {
    let fixed_extent = caps.current_extent.width != u32::MAX;
    (fixed_extent && caps.current_extent != extent) || caps.current_transform != transform
}
