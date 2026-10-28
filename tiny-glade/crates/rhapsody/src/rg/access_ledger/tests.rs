use super::*;
use crate::rg::resource_info::{OriginKind, ResourceInfo};

#[test]
fn duplicate_and_in_place_merge_keep_first_resource_order() {
    let mut entries = Vec::new();
    collect_resource_access(&mut entries, 5, 0x17, 9).unwrap();
    collect_resource_access(&mut entries, 3, 0x1a, 0).unwrap();
    assert_eq!(
        collect_resource_access(&mut entries, 5, 0x17, 9),
        Ok(LedgerUpdate::Deduplicated { index: 0 })
    );
    collect_resource_access(&mut entries, 5, 0x1d, 9).unwrap();
    collect_resource_access(&mut entries, 3, 0x02, 0).unwrap();
    assert_eq!(
        entries
            .iter()
            .map(|e| (e.resource_index, e.synchronization_group, e.access_code))
            .collect::<Vec<_>>(),
        [(5, 9, 0x1e), (3, 0, 0x02)]
    );
    let mut reverse = Vec::new();
    collect_resource_access(&mut reverse, 0, 0x1d, 2).unwrap();
    collect_resource_access(&mut reverse, 0, 0x17, 2).unwrap();
    assert_eq!(reverse[0].access_code, 0x1e);
}

#[test]
fn upgrade_appends_and_calculate_consumes_group_code_without_swapping() {
    use crate::rg::graph::{BuildError, Descriptor, OriginSource, RenderGraph};
    use crate::rg::resource_registry::BufferDesc;
    use crate::rg::resource::{ResourceHandle, ResourceOrigin};
    let mut graph = RenderGraph::new();
    let handles: Vec<_> = (0..2)
        .map(|index| {
            graph
                .create_raw_resource(
                    format!("buffer-{index}"),
                    ResourceOrigin {
                        descriptor: Descriptor::Buffer(BufferDesc {
                            word_at_00: 16,
                            word_at_08: 0,
                            bytes_at_0c_through_0e: [0; 3],
                        }),
                        source: OriginSource::ImportUnowned {
                            token: index + 1,
                            initial_access: 0,
                        },
                    },
                )
                .unwrap()
        })
        .collect();
    let mut pass = graph.add_pass("访问升级");
    assert!(matches!(
        pass.add_resource_access(ResourceHandle::from_raw(99), 0x19, 3),
        Err(BuildError::InvalidHandle(99))
    ));
    pass.add_resource_access(handles[0], 0x19, 3).unwrap();
    pass.add_resource_access(handles[1], 0x24, 0).unwrap();
    pass.add_resource_access(handles[0], 0x37, 17).unwrap();
    pass.add_resource_access(handles[0], 0x37, 0).unwrap();
    pass.render(|_| true);
    let built = graph.build();
    let entries = &built.declaration.passes[0].accesses;
    assert_eq!(entries.len(), 3);
    assert_eq!(
        (entries[0].synchronization_group, entries[0].access_code),
        (0, 0x37)
    );
    assert_eq!(
        (entries[2].synchronization_group, entries[2].access_code),
        (0, 0x37)
    );
    let origins = [OriginKind::Tag1 { type_code: 1 }; 2];
    let mut info = ResourceInfo::default();
    info.calculate(&origins, &[], &[entries], |_| ash::vk::AccessFlags::empty())
        .unwrap();
    assert_eq!(info.accesses.len(), 3);
    assert_eq!(info.accesses[2].previous_access_index, 0);
    assert_eq!(info.accesses[0].next_access_index, 2);
    assert_eq!(info.accesses[2].access_code, 0x37);
    assert_eq!(info.accesses[2].synchronization_group, 0);
    assert_eq!(info.accesses[1].resource_index, 1);
}

#[test]
fn conflict_preserves_prior_entries_and_group_equality_is_exact() {
    let mut entries = Vec::new();
    collect_resource_access(&mut entries, 0, 0x37, 3).unwrap();
    let before = entries.clone();
    let error = collect_resource_access(&mut entries, 0, 0x37, 4).unwrap_err();
    assert_eq!(error.previous_group, 3);
    assert_eq!(error.new_group, 4);
    assert_eq!(entries, before);
    assert!(ResourceSyncGroup::NONE.is_equivalent(ResourceSyncGroup::NONE));
    assert!(!ResourceSyncGroup::NONE.is_equivalent(ResourceSyncGroup::from_raw(2)));
    let mut asymmetric = Vec::new();
    collect_resource_access(&mut asymmetric, 0, 0x02, 0).unwrap();
    assert!(collect_resource_access(&mut asymmetric, 0, 0x1a, 0).is_err());
}

#[test]
fn group_counter_and_twelve_byte_ledger_projection() {
    let counter = AtomicU32::new(2);
    assert_eq!(allocate_group(&counter), 2);
    assert_eq!(allocate_group(&counter), 3);
    counter.store(u32::MAX, Ordering::Relaxed);
    assert!(std::panic::catch_unwind(|| allocate_group(&counter)).is_err());
    assert_eq!(counter.load(Ordering::Relaxed), u32::MAX);
    assert_eq!(std::mem::size_of::<AccessLedgerEntry>(), 12);
    assert_eq!(
        std::mem::offset_of!(AccessLedgerEntry, synchronization_group),
        4
    );
    assert_eq!(std::mem::offset_of!(AccessLedgerEntry, access_code), 8);
}

#[test]
fn upgrade_saves_real_read_role_and_duplicate_output_preserving_intent() {
    use super::super::resource::{AccessRange, AccessVersion};
    for permitted in [false, true] {
        let mut entries = Vec::new();
        collect_resource_access_range_with_policy(
            &mut entries,
            0,
            0x19,
            1,
            AccessRange::Whole,
            AccessVersion::Automatic,
            false,
            false,
            permitted,
        )
        .unwrap();
        assert!(!entries[0].reads_previous);
        assert!(entries[0].reads_contents(false));
        collect_resource_access(&mut entries, 0, 0x37, 0).unwrap();
        collect_resource_access(&mut entries, 0, 0x37, 0).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries[0].previous_content_read,
            if permitted {
                PreviousContentRead::UndefinedAllowed
            } else {
                PreviousContentRead::DefinedRequired
            }
        );
        assert_eq!(
            entries[1].previous_content_read,
            PreviousContentRead::NotRead
        );
        collect_resource_access_range(
            &mut entries,
            0,
            0x37,
            0,
            AccessRange::Whole,
            AccessVersion::Automatic,
            true,
        )
        .unwrap();
        assert_eq!(
            entries[1].previous_content_read,
            PreviousContentRead::DefinedRequired
        );
        assert!(entries.iter().all(|access| access.access_code == 0x37
            && access.synchronization_group == 0
            && access.reads_previous));
    }
}

#[test]
fn malformed_upgrade_does_not_erase_invalid_caller_content_mode() {
    use super::super::resource::{AccessRange, AccessVersion};
    let mut entries = Vec::new();
    collect_resource_access(&mut entries, 0, 0x19, 1).unwrap();
    let before = entries.clone();
    assert!(
        collect_resource_access_range_with_policy(
            &mut entries,
            0,
            0x37,
            0,
            AccessRange::Whole,
            AccessVersion::Automatic,
            true,
            false,
            true,
        )
        .is_err()
    );
    assert_eq!(entries, before);
}
