use super::*;
use crate::rg::transient_memory::{MemoryRequirement, plan_memory_requirements};
fn plan() -> PhysicalMemoryPlan {
    plan_memory_requirements(
        &(0..2)
            .map(|resource| MemoryRequirement {
                sharing: crate::tracking::ResourceSharing::Exclusive,
                resource,
                compatibility: 0,
                size: 4096,
                alignment: 256,
                memory_type_bits: 3,
                allocation_flags: 0,
                requires_dedicated: false,
                prefers_dedicated: false,
                uses: vec![resource as usize],
            })
            .collect::<Vec<_>>(),
        &[(0, 1)],
    )
    .unwrap()
}
#[test]
fn independently_rejects_bad_offsets_missing_order_types_and_duplicate_ownership() {
    let original = plan();
    assert!(check_memory_plan(&original, &[(0, 1)]).is_ok());
    assert!(check_memory_plan(&original, &[]).is_err());
    let mut broken = original.clone();
    broken.placements[0].offset = 1;
    assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
    let mut broken = original.clone();
    broken.requirements[0].requires_dedicated = true;
    assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
    let mut broken = original.clone();
    broken.backings[0].memory_type_bits = 4;
    assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
    let mut broken = original.clone();
    broken.backings.push(broken.backings[0].clone());
    assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
    let mut broken = original.clone();
    broken.requirements[0].compatibility = 1;
    assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
    let mut broken = original;
    broken.backings[0].memory_type = Some(32);
    assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
}
