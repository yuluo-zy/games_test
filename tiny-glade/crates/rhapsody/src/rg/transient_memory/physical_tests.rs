use super::*;
fn fact(resource: u32, uses: &[usize]) -> MemoryRequirement {
    MemoryRequirement {
        sharing: crate::tracking::ResourceSharing::Exclusive,
        resource,
        compatibility: 0,
        size: 4096,
        alignment: 256,
        memory_type_bits: 3,
        allocation_flags: 0,
        requires_dedicated: false,
        prefers_dedicated: false,
        uses: uses.to_vec(),
    }
}
#[test]
fn disjoint_recording_indices_without_dag_proof_never_alias() {
    assert_eq!(
        plan_memory_requirements(&[fact(0, &[0]), fact(1, &[2])], &[])
            .unwrap()
            .backings
            .len(),
        2
    );
    let plan = plan_memory_requirements(
        &[fact(0, &[0, 1]), fact(1, &[3, 4])],
        &[(0, 1), (1, 2), (2, 3), (3, 4)],
    )
    .unwrap();
    assert_eq!(plan.backings.len(), 1);
    assert_eq!(plan.stats.reuse_bytes, 4096);
}
#[test]
fn overlapping_uses_and_missing_branch_order_never_alias() {
    assert_eq!(
        plan_memory_requirements(
            &[fact(0, &[0, 2]), fact(1, &[1, 3])],
            &[(0, 1), (1, 2), (2, 3)]
        )
        .unwrap()
        .backings
        .len(),
        2
    );
    assert_eq!(
        plan_memory_requirements(&[fact(0, &[0, 1]), fact(1, &[2])], &[(0, 2)])
            .unwrap()
            .backings
            .len(),
        2
    );
}
#[test]
fn dedicated_types_and_allocation_flags_never_alias() {
    for variant in 0..4 {
        let a = fact(0, &[0]);
        let mut b = fact(1, &[1]);
        match variant {
            0 => b.requires_dedicated = true,
            1 => b.memory_type_bits = 4,
            2 => b.allocation_flags = 2,
            _ => b.compatibility = 1,
        }
        assert_eq!(
            plan_memory_requirements(&[a, b], &[(0, 1)])
                .unwrap()
                .backings
                .len(),
            2
        );
    }
}
#[test]
fn physical_plan_checks_overflow_duplicates_and_actual_alignment() {
    let mut b = fact(1, &[1]);
    b.alignment = 8192;
    b.size = 8193;
    let plan = plan_memory_requirements(&[fact(0, &[0]), b], &[(0, 1)]).unwrap();
    assert_eq!(plan.backings[0].size, 16384);
    assert_eq!(plan.backings[0].alignment, 8192);
    let mut bad = fact(0, &[0]);
    bad.size = u64::MAX;
    assert!(plan_memory_requirements(&[bad], &[]).is_err());
    assert!(plan_memory_requirements(&[fact(0, &[0]), fact(0, &[1])], &[]).is_err());
    assert!(plan_memory_requirements(&[fact(0, &[0])], &[(1, 0)]).is_err());
}
