//! The only place in this crate that may name `ash`.
//!
//! Each neutral name below is either the Vulkan type (feature `native-types`) or a numeric newtype with the
//! same value space (feature off). The numeric values equal the Vulkan values, so a backend lowers them with
//! a cast. Only the operations this crate and its callers actually use are provided on the neutral types;
//! add one when a caller needs it, do not mirror the whole Vulkan API.

#[cfg(feature = "native-types")]
mod native {
    pub type StageMask = ash::vk::PipelineStageFlags;
    pub type MemoryProperties = ash::vk::MemoryPropertyFlags;
    pub type SemaphoreHandle = ash::vk::Semaphore;
    pub type BackendResult = ash::vk::Result;
    /// Number of memory heaps a device may report (`VK_MAX_MEMORY_HEAPS`).
    pub const MAX_MEMORY_HEAPS: usize = ash::vk::MAX_MEMORY_HEAPS;
}

#[cfg(not(feature = "native-types"))]
mod neutral {
    use std::ops::{BitAnd, BitOr, BitOrAssign, Not};

    /// Pipeline stages a dependency waits at. Bit values equal `VkPipelineStageFlagBits`.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct StageMask(u32);
    impl StageMask {
        pub const ALL_COMMANDS: Self = Self(0x0001_0000);
        pub const HOST: Self = Self(0x0000_4000);
        pub const fn empty() -> Self {
            Self(0)
        }
        pub const fn is_empty(self) -> bool {
            self.0 == 0
        }
        pub const fn contains(self, other: Self) -> bool {
            self.0 & other.0 == other.0
        }
        pub const fn from_raw(raw: u32) -> Self {
            Self(raw)
        }
        pub const fn as_raw(self) -> u32 {
            self.0
        }
    }
    impl BitAnd for StageMask {
        type Output = Self;
        fn bitand(self, rhs: Self) -> Self {
            Self(self.0 & rhs.0)
        }
    }
    impl BitOr for StageMask {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self {
            Self(self.0 | rhs.0)
        }
    }
    impl BitOrAssign for StageMask {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 |= rhs.0;
        }
    }
    impl Not for StageMask {
        type Output = Self;
        fn not(self) -> Self {
            Self(!self.0)
        }
    }

    /// Memory property bits. Values equal `VkMemoryPropertyFlagBits`.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct MemoryProperties(u32);
    impl MemoryProperties {
        pub const DEVICE_LOCAL: Self = Self(0x1);
        pub const fn empty() -> Self {
            Self(0)
        }
        pub const fn contains(self, other: Self) -> bool {
            self.0 & other.0 == other.0
        }
        pub const fn from_raw(raw: u32) -> Self {
            Self(raw)
        }
        pub const fn as_raw(self) -> u32 {
            self.0
        }
    }
    impl BitOr for MemoryProperties {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self {
            Self(self.0 | rhs.0)
        }
    }

    /// An opaque per-queue timeline object (a Vulkan semaphore or a Metal shared event slot).
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct SemaphoreHandle(u64);
    impl SemaphoreHandle {
        pub const fn from_raw(raw: u64) -> Self {
            Self(raw)
        }
        pub const fn as_raw(self) -> u64 {
            self.0
        }
    }

    /// A backend status code that is not mapped to a neutral error class.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct BackendResult(pub i32);

    pub const MAX_MEMORY_HEAPS: usize = 16;
}

#[cfg(feature = "native-types")]
pub use native::*;
#[cfg(not(feature = "native-types"))]
pub use neutral::*;

#[cfg(all(test, feature = "native-types"))]
mod tests {
    use super::*;

    // The neutral configuration promises Vulkan's numeric values. These assertions pin the Vulkan side, so a
    // neutral constant that drifts from the spec shows up when the two are compared in `numeric_values`.
    #[test]
    fn vulkan_values_the_neutral_types_mirror() {
        assert_eq!(StageMask::ALL_COMMANDS.as_raw(), 0x0001_0000);
        assert_eq!(StageMask::HOST.as_raw(), 0x0000_4000);
        assert_eq!(MemoryProperties::DEVICE_LOCAL.as_raw(), 0x1);
        assert_eq!(MAX_MEMORY_HEAPS, 16);
    }
}
