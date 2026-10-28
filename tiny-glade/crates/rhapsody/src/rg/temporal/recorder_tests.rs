use super::*;

fn image() -> Descriptor {
    use crate::resource::texture::{TextureDesc, TextureKind};
    super::super::vulkan::ResourceSpec::Image(
        TextureDesc::new(
            ash::vk::Extent3D {
                width: 8,
                height: 8,
                depth: 1,
            },
            TextureKind::D2,
            ash::vk::Format::R8G8B8A8_UNORM,
        )
        .usage(ash::vk::ImageUsageFlags::TRANSFER_DST),
    )
    .descriptor()
}

#[test]
fn failed_dynamic_names_and_unknown_resets_leave_no_owned_keys() {
    let mut state = TemporalRenderGraphState::default();
    for index in 0..1025 {
        let name = format!("failed dynamic history {index}");
        let mut graph = RenderGraph::new();
        let error = state
            .get_or_create_temporal_pair(&mut graph, &name, buffer(), || unreachable!())
            .unwrap_err();
        let TemporalError::WrongResourceKind(key) = error else {
            panic!("expected a typed name error");
        };
        let weak = Arc::downgrade(&key);
        assert_eq!(key.as_ref(), name);
        drop(key);
        assert!(
            weak.upgrade().is_none(),
            "failed names must have no global owner"
        );
        let error = state
            .get_or_create_temporal(&mut graph, &name, image(), || {
                Err(crate::Error::OutOfHostMemory.into())
            })
            .unwrap_err();
        assert!(matches!(error, TemporalError::Backend(_)));
        state.reset(&name).unwrap();
        assert!(graph.declaration.resources.is_empty());
    }
    assert!(state.resources.is_empty());
    assert!(state.parity.is_empty());
    assert!(state.pending_parity.is_empty());
    assert!(state.frame_requested.is_empty());
    assert!(state.frame_exports.is_empty());
    assert!(state.retired_tokens.is_empty());
    assert_eq!(state.resources.capacity(), 0);
    assert_eq!(state.parity.capacity(), 0);
    assert_eq!(state.pending_parity.capacity(), 0);
    assert_eq!(state.frame_requested.capacity(), 0);
    assert_eq!(state.frame_exports.capacity(), 0);
}

#[test]
fn names_are_shared_by_requests_and_released_with_session_and_errors() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "owned history", image(), || Ok(42))
        .unwrap();
    let key = state.resources.get_key_value("owned history").unwrap().0;
    let weak = Arc::downgrade(key);
    assert!(Arc::ptr_eq(
        key,
        state.frame_requested.get("owned history").unwrap()
    ));
    let error = state
        .get_or_create_temporal(&mut graph, "owned history", image(), || unreachable!())
        .unwrap_err();
    let TemporalError::AlreadyRequested(key) = &error else {
        panic!("expected a duplicate request error");
    };
    assert!(Arc::ptr_eq(
        key,
        state.resources.get_key_value("owned history").unwrap().0
    ));
    state.clear();
    drop(state);
    assert!(
        weak.upgrade().is_some(),
        "the returned error still owns its name"
    );
    drop(error);
    assert!(
        weak.upgrade().is_none(),
        "no process-global owner may retain the name"
    );
}

fn initialize(graph: &mut RenderGraph, lookup: TemporalLookup) {
    let mut pass = graph.add_pass("initialize history");
    pass.add_resource_access(lookup.handle, 0x34, 0).unwrap();
    pass.render(|_| true);
}

#[test]
fn explicit_whole_export_keeps_the_consumers_boundary_layout_on_reuse() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let first = state.get_or_create_temporal(&mut graph, "final color", image(), || Ok(91)).unwrap();
    initialize(&mut graph, first);
    graph.export(first.handle, 0x22).unwrap();
    state.export_temporal(&mut graph).unwrap();
    assert_eq!(graph.declaration.exports.len(), 1,
        "named publication must not append an incompatible second export");
    state.confirm_exports();
    let mut next = RenderGraph::new();
    let again = state.get_or_create_temporal(&mut next, "final color", image(), || unreachable!()).unwrap();
    assert_eq!(again.resource.access, 0x22);
    assert!(again.reused);
}

#[test]
fn fixed_descriptor_change_returns_error_without_advancing_history() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let first = state
        .get_or_create_temporal(&mut graph, "fixed", image(), || Ok(91))
        .unwrap();
    initialize(&mut graph, first);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    let mut graph = RenderGraph::new();
    let Descriptor::Image(mut changed) = image() else {
        unreachable!()
    };
    changed.words_at_00_through_18[2] += 1;
    assert!(matches!(
        state.get_or_create_temporal(
            &mut graph,
            "fixed",
            Descriptor::Image(changed),
            || unreachable!()
        ),
        Err(TemporalError::DescriptorMismatch(_))
    ));
    assert!(graph.declaration.resources.is_empty());
    let again = state
        .get_or_create_temporal(&mut graph, "fixed", image(), || unreachable!())
        .unwrap();
    assert_eq!(again.resource.token, 91);
    assert!(again.reused);
}
#[test]
fn owned_temporal_propagates_real_sharing_and_recreates_when_policy_changes() {
    use crate::tracking::{QueueFamilies, ResourceSharing};
    struct Owner {
        next: u64,
        sharing: ResourceSharing,
        released: Vec<u64>,
        fail: bool,
    }
    impl TemporalResourceOwner for Owner {
        fn create_temporal(&mut self, _: &str, _: Descriptor) -> Result<u64, BuildError> {
            if self.fail {
                return Err(BuildError::from(crate::Error::OutOfDeviceMemory));
            }
            self.next += 1;
            Ok(self.next)
        }
        fn release_temporal(&mut self, token: u64) {
            self.released.push(token);
        }
        fn temporal_sharing(&self, _: &str) -> ResourceSharing {
            self.sharing
        }
    }
    let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1]).unwrap());
    let mut owner = Owner {
        next: 0,
        sharing,
        released: Vec::new(),
        fail: false,
    };
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let first = state
        .get_or_create_owned(&mut graph, "shared", image(), &mut owner)
        .unwrap();
    assert_eq!(graph.declaration.resources[0].sharing, sharing);
    initialize(&mut graph, first);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    let mut graph = RenderGraph::new();
    let reused = state
        .get_or_create_owned(&mut graph, "shared", image(), &mut owner)
        .unwrap();
    assert!(reused.reused);
    assert_eq!(graph.declaration.resources[0].sharing, sharing);
    state.abandon_frame();
    owner.sharing = ResourceSharing::Exclusive;
    owner.fail = true;
    let mut failed_graph = RenderGraph::new();
    let error = state
        .get_or_create_owned(&mut failed_graph, "shared", image(), &mut owner)
        .unwrap_err();
    assert!(matches!(
        crate::Error::from(error),
        crate::Error::OutOfDeviceMemory
    ));
    assert_eq!(state.get("shared").unwrap().token, first.resource.token);
    assert!(owner.released.is_empty());
    owner.fail = false;
    let mut graph = RenderGraph::new();
    let replacement = state
        .get_or_create_owned(&mut graph, "shared", image(), &mut owner)
        .unwrap();
    assert!(!replacement.reused);
    assert_ne!(replacement.resource.token, first.resource.token);
    assert_eq!(owner.released, vec![first.resource.token]);
    assert_eq!(
        graph.declaration.resources[0].sharing,
        ResourceSharing::Exclusive
    );
}

#[test]
fn partial_submission_invalidates_only_requested_history_until_a_successful_export() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    for (name, token) in [("written", 5), ("untouched", 6)] {
        let lookup = state
            .get_or_create_temporal(&mut graph, name, image(), || Ok(token))
            .unwrap();
        initialize(&mut graph, lookup);
    }
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();

    state.begin_frame();
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "written", image(), || panic!("retain token"))
        .unwrap();
    state.export_temporal(&mut graph).unwrap();
    state.invalidate_submitted_frame();
    assert!(state.retired_tokens().is_empty());

    // The next frame must initialize the damaged contents, while the actual
    // allocation and backend recovery state stay associated with token 5.
    state.begin_frame();
    let mut graph = RenderGraph::new();
    let damaged = state
        .get_or_create_temporal(&mut graph, "written", image(), || panic!("retain token"))
        .unwrap();
    assert!(!damaged.reused);
    assert_eq!(damaged.resource.token, 5);
    assert_eq!(damaged.resource.access, TEMPORAL_EXPORT_ACCESS);
    let untouched = state
        .get_or_create_temporal(&mut graph, "untouched", image(), || panic!("retain token"))
        .unwrap();
    assert!(untouched.reused);
    state.abandon_frame();

    // A further unsubmitted failure cannot accidentally validate contents.
    let mut graph = RenderGraph::new();
    let damaged = state
        .get_or_create_temporal(&mut graph, "written", image(), || panic!("retain token"))
        .unwrap();
    assert!(!damaged.reused);
    initialize(&mut graph, damaged);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    let mut graph = RenderGraph::new();
    assert!(
        state
            .get_or_create_temporal(&mut graph, "written", image(), || panic!("retain token"))
            .unwrap()
            .reused
    );
}

#[test]
fn unsubmitted_history_keeps_its_allocation_but_needs_initialization() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let created = state
        .get_or_create_temporal(&mut graph, "history", image(), || Ok(5))
        .unwrap();
    assert!(!created.reused);
    state.export_temporal(&mut graph).unwrap();
    // Starting another frame also cancels an exported graph that was never
    // submitted; its speculative export layout must not become an import.
    state.begin_frame();
    let mut graph = RenderGraph::new();
    let retry = state
        .get_or_create_temporal(&mut graph, "history", image(), || {
            panic!("retain allocation")
        })
        .unwrap();
    assert_eq!(retry.resource.token, created.resource.token);
    assert_eq!(retry.resource.access, 0);
    assert!(!retry.reused);
    initialize(&mut graph, retry);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    state.begin_frame();
    let mut graph = RenderGraph::new();
    let confirmed = state
        .get_or_create_temporal(&mut graph, "history", image(), || {
            panic!("retain allocation")
        })
        .unwrap();
    assert!(confirmed.reused);
    assert_eq!(confirmed.resource.token, created.resource.token);
    assert_eq!(confirmed.resource.access, TEMPORAL_EXPORT_ACCESS);
}

#[test]
fn pair_parity_commits_only_with_exports_and_rolls_back_failed_frames() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let mut token = 0;
    let first = state
        .get_or_create_temporal_pair(&mut graph, "history", image(), || {
            token += 1;
            Ok(token)
        })
        .unwrap();
    assert_eq!(first.parity, 0);
    state.export_temporal(&mut graph).unwrap();
    state.abandon_frame();
    assert_eq!(state.parity("history"), None);

    let mut graph = RenderGraph::new();
    let committed = state
        .get_or_create_temporal_pair(&mut graph, "history", image(), || panic!("retain pair"))
        .unwrap();
    assert_eq!(committed.parity, 0);
    initialize(&mut graph, committed.first);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    assert_eq!(state.parity("history"), Some(0));

    let mut graph = RenderGraph::new();
    let failed = state
        .get_or_create_temporal_pair(&mut graph, "history", image(), || panic!("retain pair"))
        .unwrap();
    assert_eq!(failed.parity, 1);
    state.export_temporal(&mut graph).unwrap();
    state.invalidate_submitted_frame();
    assert_eq!(state.parity("history"), Some(0));

    let mut graph = RenderGraph::new();
    let recovered = state
        .get_or_create_temporal_pair(&mut graph, "history", image(), || panic!("retain pair"))
        .unwrap();
    assert_eq!(recovered.parity, 1);
    assert!(!(recovered.first.reused && recovered.second.reused));
    initialize(&mut graph, recovered.first);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    assert_eq!(state.parity("history"), Some(1));
}

#[test]
fn starting_a_new_frame_cancels_uncommitted_parity() {
    let mut state = TemporalRenderGraphState::default();
    assert_eq!(state.get_and_flip_parity("pair"), 0);
    state.begin_frame();
    assert_eq!(state.parity("pair"), None);
    assert_eq!(state.get_and_flip_parity("pair"), 0);
    state.confirm_exports();
    assert_eq!(
        state.parity("pair"),
        None,
        "unpublished members cannot commit parity"
    );
    assert_eq!(state.get_and_flip_parity("pair"), 0);
    state.begin_frame();
    assert_eq!(state.parity("pair"), None);
    state.get_and_flip_parity("pair");
    state.clear();
    state.abandon_frame();
    assert_eq!(state.parity("pair"), None);
}

#[test]
fn fresh_pair_publishes_only_initialized_members_and_validates_their_actual_mapping() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let mut token = 10;
    let pair = state
        .get_or_create_temporal_pair(&mut graph, "pair", image(), || {
            token += 1;
            Ok(token)
        })
        .unwrap();
    initialize(&mut graph, pair.first);
    state.export_temporal(&mut graph).unwrap();
    assert_eq!(graph.declaration.exports.len(), 1);
    assert_eq!(
        graph.declaration.exports[0].resource_index,
        pair.first.handle.raw()
    );
    assert_eq!(state.get("pair:1").unwrap().access, 0);
    assert!(matches!(
        state.get_or_create_temporal(&mut graph, "pair:0", image(), || unreachable!()),
        Err(TemporalError::AlreadyRequested(_))
    ));
    let mut compiled = super::super::graph::compile_graph(
        graph.build().declaration,
        &mut Default::default(),
        &mut super::super::graph::EventBackend::default(),
        &super::super::vulkan::AccessTable::original(),
    )
    .unwrap();
    state.validate_compiled_exports(&compiled).unwrap();
    let index = pair.first.handle.raw() as usize;
    compiled.tables.snapshot.physical_mapping[index].token = Some(1000);
    assert!(matches!(
        state.validate_compiled_exports(&compiled),
        Err(TemporalError::ExportTokenMismatch(_))
    ));
    compiled.tables.snapshot.physical_mapping[index].token = Some(pair.first.resource.token);
    compiled.tables.snapshot.exports.clear();
    assert!(matches!(
        state.validate_compiled_exports(&compiled),
        Err(TemporalError::ExportMissing(_))
    ));
    state.confirm_exports();

    let mut graph = RenderGraph::new();
    let pair = state
        .get_or_create_temporal_pair(&mut graph, "pair", image(), || unreachable!())
        .unwrap();
    assert_eq!([pair.first.reused, pair.second.reused], [false, true]);
    initialize(&mut graph, pair.first);
    state.export_temporal(&mut graph).unwrap();
    assert_eq!(graph.declaration.exports.len(), 2);
    state.confirm_exports();
    let mut graph = RenderGraph::new();
    let reused = state
        .get_or_create_temporal_pair(&mut graph, "pair", image(), || unreachable!())
        .unwrap();
    assert!(reused.first.reused && reused.second.reused);
}

fn buffer() -> Descriptor {
    Descriptor::Buffer(crate::rg::resource_registry::BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    })
}
fn write_half(graph: &mut RenderGraph, lookup: TemporalLookup, offset: u64) {
    let mut pass = graph.add_pass("write half");
    pass.add_resource_access_range(
        lookup.handle,
        0x34,
        0,
        super::super::resource::AccessRange::Buffer { offset, size: 32 },
    )
    .unwrap();
    pass.render(|_| true);
}

#[test]
fn unused_or_partially_initialized_history_is_never_confirmed_and_first_reads_are_rejected() {
    for descriptor in [image(), buffer()] {
        let mut state = TemporalRenderGraphState::default();
        let mut graph = RenderGraph::new();
        state
            .get_or_create_temporal(&mut graph, "unused", descriptor, || Ok(5))
            .unwrap();
        state.export_temporal(&mut graph).unwrap();
        assert!(graph.declaration.exports.is_empty());
        state.confirm_exports();
        let mut graph = RenderGraph::new();
        let history = state
            .get_or_create_temporal(&mut graph, "unused", descriptor, || unreachable!())
            .unwrap();
        assert!(!history.reused);
        assert_eq!(history.resource.access, 0);
        let mut reader = graph.add_pass("illegal first read");
        reader.add_resource_access(history.handle, 0x25, 0).unwrap();
        reader.render(|_| true);
        assert!(matches!(
            state.export_temporal(&mut graph),
            Err(TemporalError::Backend(BuildError::Contract(
                super::super::resource::ResourceContractError::UninitializedRead { .. }
            )))
        ));
    }

    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let history = state
        .get_or_create_temporal(&mut graph, "partial", buffer(), || Ok(6))
        .unwrap();
    write_half(&mut graph, history, 0);
    state.export_temporal(&mut graph).unwrap();
    assert!(graph.declaration.exports.is_empty());
    state.confirm_exports();
    let mut graph = RenderGraph::new();
    let history = state
        .get_or_create_temporal(&mut graph, "partial", buffer(), || unreachable!())
        .unwrap();
    assert!(!history.reused);
    write_half(&mut graph, history, 0);
    write_half(&mut graph, history, 32);
    state.export_temporal(&mut graph).unwrap();
    assert_eq!(graph.declaration.exports.len(), 1);
    state.confirm_exports();
    let mut graph = RenderGraph::new();
    let history = state
        .get_or_create_temporal(&mut graph, "partial", buffer(), || unreachable!())
        .unwrap();
    assert!(history.reused);
    state.export_temporal(&mut graph).unwrap();
    state.invalidate_submitted_frame();
    let mut graph = RenderGraph::new();
    let history = state
        .get_or_create_temporal(&mut graph, "partial", buffer(), || unreachable!())
        .unwrap();
    assert!(!history.reused);
    write_half(&mut graph, history, 0);
    state.export_temporal(&mut graph).unwrap();
    assert!(
        graph.declaration.exports.is_empty(),
        "half a failed history cannot publish a whole valid history"
    );
    state.confirm_exports();
    let mut graph = RenderGraph::new();
    assert!(
        !state
            .get_or_create_temporal(&mut graph, "partial", buffer(), || unreachable!())
            .unwrap()
            .reused
    );
}
#[test]
fn uploaded_initial_contents_are_readable_without_publishing_history_and_survive_cancel() {
    struct UploadedOwner {
        next: u64,
    }
    impl TemporalResourceOwner for UploadedOwner {
        fn create_temporal(&mut self, _: &str, _: Descriptor) -> Result<u64, BuildError> {
            self.next += 1;
            Ok(self.next)
        }
        fn release_temporal(&mut self, _: u64) {}
        fn initial_contents_defined(&self, _: u64) -> bool {
            true
        }
    }
    fn reader(graph: &mut RenderGraph, lookup: TemporalLookup) {
        let mut pass = graph.add_pass("read actually uploaded bytes");
        pass.add_resource_access(lookup.handle, 0x25, 0).unwrap();
        pass.render(|_| true);
    }
    let mut owner = UploadedOwner { next: 0 };
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let first = state
        .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
        .unwrap();
    assert!(!first.reused, "an upload is not preceding frame history");
    reader(&mut graph, first);
    state.export_temporal(&mut graph).unwrap();
    assert_eq!(graph.declaration.exports.len(), 1);
    state.abandon_frame();
    let mut graph = RenderGraph::new();
    let cancelled = state
        .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
        .unwrap();
    assert!(!cancelled.reused);
    assert_eq!(cancelled.resource.token, first.resource.token);
    reader(&mut graph, cancelled);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    let mut graph = RenderGraph::new();
    let confirmed = state
        .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
        .unwrap();
    assert!(confirmed.reused);
    state.invalidate_submitted_frame();
    let mut graph = RenderGraph::new();
    let failed = state
        .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
        .unwrap();
    assert!(!failed.reused);
    reader(&mut graph, failed);
    assert!(matches!(
        state.export_temporal(&mut graph),
        Err(TemporalError::Backend(BuildError::Contract(
            super::super::resource::ResourceContractError::UninitializedRead { .. }
        )))
    ));
    state.abandon_frame();
    state.reset("uploaded").unwrap();
    let mut graph = RenderGraph::new();
    let reset = state
        .get_or_create_owned(&mut graph, "uploaded", buffer(), &mut owner)
        .unwrap();
    reader(&mut graph, reset);
    assert!(state.export_temporal(&mut graph).is_err());
    assert_eq!(
        owner.next, 1,
        "reset/cancel do not secretly allocate or reupload"
    );
}

// ---------------------------------------------------------------- 5. temporal 记录放弃 / 确认

#[test]
fn an_abandoned_temporal_request_is_released_at_the_next_frame_begin() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let history = state
        .get_or_create_temporal(&mut graph, "hist", image(), || Ok(7))
        .unwrap();
    let mut initialize = graph.add_pass("initialize history");
    initialize
        .add_resource_access(history.handle, 0x34, 0)
        .unwrap();
    initialize.render(|_| true);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    // 下一帧请求后放弃构图（不导出、不编译）。
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
        .unwrap();
    drop(graph);
    // 同一帧内重复请求仍报错。
    let mut same = RenderGraph::new();
    assert!(matches!(
        state.get_or_create_temporal(&mut same, "hist", image(), || Ok(8)),
        Err(TemporalError::AlreadyRequested(_))
    ));
    state.begin_frame();
    let mut graph = RenderGraph::new();
    let again = state
        .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
        .unwrap();
    assert!(again.reused);
    assert_eq!(again.resource.token, 7);
}

#[test]
fn an_unexecuted_export_does_not_promote_a_new_resource_beyond_undefined() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let history = state
        .get_or_create_temporal(&mut graph, "hist", image(), || Ok(7))
        .unwrap();
    let mut initialize = graph.add_pass("initialize history");
    initialize
        .add_resource_access(history.handle, 0x34, 0)
        .unwrap();
    initialize.render(|_| true);
    state.export_temporal(&mut graph).unwrap();
    // export_temporal 写入时态导出访问码。
    assert_eq!(state.get("hist").unwrap().access, TEMPORAL_EXPORT_ACCESS);
    // 本帧没有真正执行：放弃后回到 GPU 已确认的访问（新资源 = undefined）。
    state.abandon_frame();
    assert_eq!(state.get("hist").unwrap().access, 0);
    assert_eq!(
        state.record_state("hist"),
        Some(TemporalRecordState::Exported)
    );
    // 执行确认后再放弃不回退。
    let mut graph = RenderGraph::new();
    let history = state
        .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
        .unwrap();
    let mut initialize = graph.add_pass("initialize history");
    initialize
        .add_resource_access(history.handle, 0x34, 0)
        .unwrap();
    initialize.render(|_| true);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    state.abandon_frame();
    assert_eq!(state.get("hist").unwrap().access, TEMPORAL_EXPORT_ACCESS);
}

#[test]
fn owned_descriptor_replacement_is_transactional_on_allocation_failure() {
    struct Owner {
        next: u64,
        fail: bool,
        released: Vec<u64>,
    }
    impl TemporalResourceOwner for Owner {
        fn create_temporal(&mut self, _: &str, _: Descriptor) -> Result<u64, BuildError> {
            if self.fail {
                return Err(crate::Error::OutOfDeviceMemory.into());
            }
            self.next += 1;
            Ok(self.next)
        }
        fn release_temporal(&mut self, token: u64) {
            self.released.push(token);
        }
    }
    let mut owner = Owner {
        next: 20,
        fail: false,
        released: Vec::new(),
    };
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let original = state
        .get_or_create_owned(&mut graph, "slot", image(), &mut owner)
        .unwrap();
    initialize(&mut graph, original);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    let mut changed = match image() {
        Descriptor::Image(desc) => desc,
        _ => unreachable!(),
    };
    changed.words_at_00_through_18[3] += 1;
    owner.fail = true;
    let mut graph = RenderGraph::new();
    let failure = state
        .get_or_create_owned(&mut graph, "slot", Descriptor::Image(changed), &mut owner)
        .unwrap_err();
    assert_eq!(
        crate::Error::from(failure).kind(),
        crate::ErrorKind::OutOfMemory
    );
    assert!(graph.declaration.resources.is_empty());
    assert_eq!(state.get("slot").unwrap().token, original.resource.token);
    assert_eq!(
        state.get("slot").unwrap().descriptor,
        original.resource.descriptor
    );
    assert_eq!(state.get("slot").unwrap().access, TEMPORAL_EXPORT_ACCESS);
    assert!(owner.released.is_empty());
    owner.fail = false;
    let retained = state
        .get_or_create_owned(&mut graph, "slot", image(), &mut owner)
        .unwrap();
    assert!(retained.reused);
    assert_eq!(retained.resource.token, original.resource.token);
    state.abandon_frame();
    state.reset("slot").unwrap();
    let mut graph = RenderGraph::new();
    let reset = state
        .get_or_create_owned(&mut graph, "slot", image(), &mut owner)
        .unwrap();
    assert!(!reset.reused);
    assert_eq!(reset.resource.token, original.resource.token);
    assert!(state.reset("slot").is_err());
}

#[test]
fn pair_second_allocation_failure_retains_both_confirmed_owners_and_parity() {
    struct Owner {
        next: u64,
        fail_at: u64,
        released: Vec<u64>,
    }
    impl TemporalResourceOwner for Owner {
        fn create_temporal(&mut self, _: &str, _: Descriptor) -> Result<u64, BuildError> {
            self.next += 1;
            if self.next == self.fail_at {
                return Err(crate::Error::OutOfDeviceMemory.into());
            }
            Ok(self.next)
        }
        fn release_temporal(&mut self, token: u64) {
            self.released.push(token);
        }
    }
    let mut owner = Owner {
        next: 0,
        fail_at: 4,
        released: Vec::new(),
    };
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let original = state
        .get_or_create_owned_pair(&mut graph, "pair", image(), &mut owner)
        .unwrap();
    initialize(&mut graph, original.first);
    initialize(&mut graph, original.second);
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    let mut changed = match image() {
        Descriptor::Image(desc) => desc,
        _ => unreachable!(),
    };
    changed.words_at_00_through_18[3] += 1;
    let mut graph = RenderGraph::new();
    let failure = state
        .get_or_create_owned_pair(&mut graph, "pair", Descriptor::Image(changed), &mut owner)
        .unwrap_err();
    assert_eq!(
        crate::Error::from(failure).kind(),
        crate::ErrorKind::OutOfMemory
    );
    assert!(graph.declaration.resources.is_empty());
    assert_eq!(state.parity("pair"), Some(0));
    assert_eq!(owner.released, vec![3]);
    let retained = state
        .get_or_create_owned_pair(&mut graph, "pair", image(), &mut owner)
        .unwrap();
    assert!(retained.first.reused && retained.second.reused);
    assert_eq!(
        (
            retained.first.resource.token,
            retained.second.resource.token
        ),
        (
            original.second.resource.token,
            original.first.resource.token
        )
    );
}
