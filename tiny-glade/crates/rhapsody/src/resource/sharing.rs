//! Canonical, bounded family lists belong to resource identity and cache keys.
use crate::sync::MAX_QUEUES;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct QueueFamilies {
    count: u8,
    values: [u32; MAX_QUEUES],
}
impl QueueFamilies {
    pub fn new(families: &[u32]) -> crate::Result<Self> {
        if families.len() < 2 || families.len() > MAX_QUEUES {
            return Err(crate::Error::invalid_usage(
                "concurrent sharing requires 2..MAX_QUEUES distinct families",
            ));
        }
        let mut values = [0; MAX_QUEUES];
        values[..families.len()].copy_from_slice(families);
        values[..families.len()].sort_unstable();
        if values[..families.len()]
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err(crate::Error::invalid_usage(
                "concurrent queue families must be distinct",
            ));
        }
        Ok(Self {
            count: families.len() as u8,
            values,
        })
    }
    pub fn as_slice(&self) -> &[u32] {
        &self.values[..usize::from(self.count)]
    }
    pub(crate) fn validate_device(&self, device: &crate::device::Device) -> crate::Result<()> {
        if self.as_slice().iter().any(|family| {
            *family != device.queue_family
                && !device
                    .additional_queues
                    .iter()
                    .any(|queue| queue.family == *family)
        }) {
            return Err(crate::Error::invalid_usage(
                "concurrent resource names a family with no configured device queue",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ResourceSharing {
    #[default]
    Exclusive,
    Concurrent(QueueFamilies),
}
impl ResourceSharing {
    pub fn is_concurrent(self) -> bool {
        matches!(self, Self::Concurrent(_))
    }
    pub fn allows_family(self, family: u32) -> bool {
        match self {
            Self::Exclusive => true,
            Self::Concurrent(families) => families.as_slice().contains(&family),
        }
    }
    pub(crate) fn validate_device(self, device: &crate::device::Device) -> crate::Result<()> {
        match self {
            Self::Exclusive => Ok(()),
            Self::Concurrent(families) => families.validate_device(device),
        }
    }
    pub(crate) fn families(&self) -> &[u32] {
        match self {
            Self::Exclusive => &[],
            Self::Concurrent(families) => families.as_slice(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn family_keys_are_canonical_and_invalid_lists_are_rejected() {
        assert_eq!(
            QueueFamilies::new(&[5, 1, 3]).unwrap(),
            QueueFamilies::new(&[1, 3, 5]).unwrap()
        );
        assert!(QueueFamilies::new(&[1]).is_err());
        assert!(QueueFamilies::new(&[1, 1]).is_err());
        assert!(QueueFamilies::new(&[0; MAX_QUEUES + 1]).is_err());
        let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&[1, 3]).unwrap());
        assert!(sharing.allows_family(3));
        assert!(!sharing.allows_family(2));
    }
}
