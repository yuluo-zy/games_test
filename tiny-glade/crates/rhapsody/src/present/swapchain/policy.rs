//! Swapchain and present policies.
use crate::Error;
use crate::Result;
use super::MAX_RETIRED_GENERATIONS;

/// Completion guarantees required by a swapchain owner.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwapchainPolicy {
    /// Production presentation. With `VK_EXT_swapchain_maintenance1` final
    /// teardown is proven by present fences; without it (e.g. Intel UHD 630)
    /// teardown and generation retirement fall back to a queue-idle proof
    /// (`PresentProof::QueueIdle`).
    #[default]
    Strict,
    /// Acquisition tests and offscreen consumers; presentation is rejected.
    AcquireOnly,
}

/// How final presentation completion is proven for a generation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PresentProof {
    /// Per-present fences (maintenance1), plus reacquisition of the same
    /// image and the newer generation's first completed present.
    Fences,
    /// No present fences. A bounded wait on an empty submit to the
    /// presentation queue proves completion (see
    /// `Swapchain::prove_presents_by_queue_idle`). Used at teardown and when
    /// the retired-generation budget is full.
    QueueIdle,
}

impl SwapchainPolicy {
    pub(super) fn present_proof(self, maintenance1: bool) -> PresentProof {
        if self == Self::Strict && !maintenance1 {
            PresentProof::QueueIdle
        } else {
            PresentProof::Fences
        }
    }
    pub(super) fn allow_present(self) -> Result<()> {
        if self == Self::AcquireOnly {
            Err(Error::unsupported("AcquireOnly 交换链不允许呈现"))
        } else {
            Ok(())
        }
    }
}

pub(super) fn check_retired_generation_budget(pending: usize) -> Result<()> {
    if pending >= MAX_RETIRED_GENERATIONS {
        Err(Error::exhausted(
            "retired swapchain generations await completion proofs",
        ))
    } else {
        Ok(())
    }
}

/// What happens to an acquired image whose frame failed before complete
/// content was rendered into it (the presentation graph errored, was dropped
/// or cancelled). The default never shows undefined memory.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PresentPolicy {
    /// Return the image without presenting it. With
    /// `VK_EXT_swapchain_maintenance1` release-images that is a true release
    /// (nothing reaches the screen). Where the release call is not usable
    /// (see `Swapchain::can_release_images`) the acquire wait is still
    /// consumed by a real submit and a cleared opaque black frame is
    /// presented instead of undefined content.
    #[default]
    SkipPresent,
    /// Present the image cleared to this RGBA colour (requires the swapchain
    /// usage to include `TRANSFER_DST`; otherwise the policy degrades to
    /// [`Self::SkipPresent`]).
    ClearTo([f32; 4]),
    /// Re-present the previous frame's content. The swapchain keeps no copy of
    /// presented images, so this is unsupported: configuring it returns
    /// [`Error::Unsupported`].
    PreviousFrame,
}

impl PresentPolicy {
    /// Opaque black, the colour of the fallback frame.
    pub const FALLBACK_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    /// Reject policies this library cannot honour.
    pub fn validate(self) -> Result<()> {
        match self {
            Self::PreviousFrame => Err(Error::unsupported(
                "PresentPolicy::PreviousFrame: the swapchain keeps no copy of presented frames",
            )),
            Self::ClearTo(color) if color.iter().any(|c| !c.is_finite()) => Err(
                Error::invalid_usage("PresentPolicy::ClearTo needs finite colour components"),
            ),
            _ => Ok(()),
        }
    }
}
