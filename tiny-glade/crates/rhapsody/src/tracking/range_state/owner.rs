//! Range ownership, transfers and ownership denials.
use ash::vk;
use crate::native::access_table::AccessScope;
use crate::sync::SyncPoint;
use crate::tracking::ResourceSharing;
use super::native::NativeRange;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RangeOwner {
    #[default]
    Unclaimed,
    Owned(u32),
    Released {
        transfer: RangeTransfer,
        point: SyncPoint,
    },
}

impl RangeOwner {
    /// The single ownership admission rule: whether `family` may use a range
    /// in this state under `sharing` without an acquire. An unclaimed
    /// exclusive range is admitted (the access claims it).
    pub(crate) fn admits(
        self,
        sharing: ResourceSharing,
        family: u32,
    ) -> std::result::Result<(), OwnershipDenial> {
        if !sharing.allows_family(family) {
            return Err(OwnershipDenial::NotShared);
        }
        if sharing.is_concurrent() {
            return Ok(());
        }
        match self {
            Self::Unclaimed => Ok(()),
            Self::Owned(owner) if owner == family => Ok(()),
            Self::Owned(_) => Err(OwnershipDenial::OtherFamily),
            Self::Released { .. } => Err(OwnershipDenial::Released),
        }
    }
    /// [`Self::admits`] for an access that does not preserve the previous
    /// contents (`discarded`, see [`NativeState::contents_discarded`]): an
    /// exclusive range last owned by another family is admitted, because
    /// there is nothing to hand over; the access simply claims it. A pending
    /// release still denies (its acquire completes a real transfer).
    pub(crate) fn admits_discarding(
        self,
        sharing: ResourceSharing,
        family: u32,
        discarded: bool,
    ) -> std::result::Result<(), OwnershipDenial> {
        match self.admits(sharing, family) {
            Err(OwnershipDenial::OtherFamily) if discarded => Ok(()),
            other => other,
        }
    }
}

/// Why a queue family may not use a range now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OwnershipDenial {
    /// A concurrent resource does not list the family.
    NotShared,
    /// A queue-family release is still waiting for its acquire.
    Released,
    /// Another family owns the exclusive range.
    OtherFamily,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeTransfer {
    pub pair: u64,
    pub source_family: u32,
    pub destination_family: u32,
    pub range: NativeRange,
    pub source: AccessScope,
    pub destination: AccessScope,
    pub old_layout: vk::ImageLayout,
    pub new_layout: vk::ImageLayout,
}
