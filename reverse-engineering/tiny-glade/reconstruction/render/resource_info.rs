//! Game-private rhapsody resource graph rules recovered from the original machine code.
//!
//! Original source: rhapsody::rg::{resource_info,recorder} (PDB module mapping
//! is retained under evidence/render-skeleton). This file is a behavioral reconstruction,
//! not the original author's text. Vec/sort and the dependency's access classification
//! are reused through Rust APIs; their implementations are outside the recovery scope.

pub const NO_ACCESS: u32 = u32::MAX;

/// Fields written by ResourceInfo::calculate (0x140530430) and used by
/// link_up_next_access_chains (0x140bd47d0). These are the observed 20-byte
/// record fields, not a claim about the full original ResourceInfo Rust ABI.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceAccessRecord {
    pub resource_index: u32,
    pub previous_access_index: u32,
    pub next_access_index: u32,
    pub synchronization_group: u32,
    pub access_code: u8,
    /// The original routines do not read these bytes; retain them unchanged.
    pub unobserved_padding: [u8; 3],
}

/// Numeric projection of the original tagged first/last pass endpoints.
/// Lifecycle generation tests the full tag word for equality to 1.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassEndpoint {
    pub tag: u32,
    pub pass_index: u32,
}

/// Observed 24-byte per-resource record (ResourceInfo +0x38/+0x40 vector).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceLifetimeRecord {
    pub first_pass: PassEndpoint,
    pub last_pass: PassEndpoint,
    pub first_access_index: u32,
    pub last_access_index: u32,
}

/// The private synchronization rule delegates access classification to the
/// original vk_sync_fork::AccessType::is_write dependency. The dependency's
/// precise local source revision is not established by the lock file, so this
/// API takes its classifier explicitly rather than inventing a replacement enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassResourceAccessType<A> {
    pub synchronization_group: u32,
    pub access: A,
}

impl<A: PartialEq> PassResourceAccessType<A> {
    /// Original private function: 0x140bece70.
    /// Preserve the original classifier call order and short circuit.
    pub fn can_be_concurrent_with(
        &self,
        other: &Self,
        mut is_write: impl FnMut(&A) -> bool,
    ) -> bool {
        if !is_write(&self.access) && !is_write(&other.access) {
            self.access == other.access
        } else {
            self.access == other.access
                && self.synchronization_group != 0
                && self.synchronization_group == other.synchronization_group
        }
    }
}

/// Link each previous access to the current access. Original: 0x140bd47d0.
/// The method does not clear next links: records without a successor keep
/// their previous next value, and duplicate predecessors are overwritten in
/// traversal order. Bounds failures preserve writes from earlier records.
pub fn link_up_next_access_chains(accesses: &mut [ResourceAccessRecord]) {
    for current in 0..accesses.len() {
        let previous = accesses[current].previous_access_index;
        if previous != NO_ACCESS {
            let current = u32::try_from(current).expect("resource access index exceeds u32");
            accesses[previous as usize].next_access_index = current;
        }
    }
}

/// Append and sort the observed lifetime event words. Original: 0x140bd4860.
/// Each eligible resource contributes two events. The second event sets bit
/// 16; pass indices occupy bits shifted by 17, and resource indices are ORed
/// without an extra mask. This leaf neither clears the vector nor validates
/// the packed index domain. Keep those exact behaviors.
pub fn calculate_lifetime_events(resources: &[ResourceLifetimeRecord], events: &mut Vec<u32>) {
    for (resource_index, resource) in resources.iter().enumerate() {
        if resource.first_pass.tag == 1 && resource.last_pass.tag == 1 {
            let resource_index = resource_index as u32;
            events.push(resource.first_pass.pass_index.wrapping_shl(17) | resource_index);
            events.push(resource.last_pass.pass_index.wrapping_shl(17) | resource_index | 0x1_0000);
        }
    }
    // The emitted order is what the game consumes; Rust's sorting machinery
    // is a standard-library dependency and is deliberately not reconstructed.
    events.sort_unstable();
}
