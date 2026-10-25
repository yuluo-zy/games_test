//! 按原已取证的完整时序合同验证多帧构图与编译，而不是按文件非空验收。
use rhapsody::rg::{
    graph::{
        self, AccessDependency, BufferDesc, BuildError, Descriptor, EventBackend, RenderGraph,
        TextureDesc,
    },
    state::{PersistentCompileState, RenderGraphState, TakeFailure},
    temporal::{
        TEMPORAL_EXPORT_ACCESS, TemporalError, TemporalRecordState, TemporalRenderGraphState,
        TemporalResourceOwner,
    },
};
use std::panic::{AssertUnwindSafe, catch_unwind};

fn texture(width: u32) -> Descriptor {
    Descriptor::Image(TextureDesc {
        words_at_00_through_18: [0x40, 44, 1, width, 8, 1, 1],
        bytes_at_1c_through_1e: [0, 0, 2],
    })
}
fn buffer(size: u64) -> Descriptor {
    Descriptor::Buffer(BufferDesc {
        word_at_00: size,
        word_at_08: 0x40,
        bytes_at_0c_through_0e: [0; 3],
    })
}
struct Access;
impl AccessDependency for Access {
    fn access_mask(&mut self, _: u8) -> u32 {
        0x40
    }
    fn is_write(&mut self, code: u8) -> bool {
        code >= 40
    }
}

#[test]
fn multi_frame_pair_exports_compile_consumer_and_preserves_parity() {
    let mut state = RenderGraphState::default();
    let mut next = 100;
    for frame in 0..6 {
        let mut graph = state.begin_frame();
        let pair = state
            .temporal
            .get_or_create_temporal_pair(&mut graph, "history", texture(8), || {
                next += 1;
                Ok(next)
            })
            .unwrap();
        assert_eq!(pair.parity, frame % 2);
        assert_eq!(pair.both_reused, frame != 0);
        assert_eq!(
            (pair.first.resource.token, pair.second.resource.token),
            if frame % 2 == 0 {
                (101, 102)
            } else {
                (102, 101)
            }
        );
        let mut pass = graph.add_pass("accumulate");
        pass.add_resource_access(pair.first.handle, 4, 0).unwrap();
        pass.add_resource_access(pair.second.handle, 40, 0).unwrap();
        pass.render(|_| true);
        let compiled = state
            .compile_frame(graph, &mut EventBackend::default(), &mut Access)
            .unwrap();
        assert_eq!(compiled.compiled.snapshot.exports.len(), 2);
        assert!(
            compiled
                .compiled
                .declarations
                .exports
                .iter()
                .all(|e| e.access_code == TEMPORAL_EXPORT_ACCESS)
        );
        assert!(
            compiled
                .compiled
                .snapshot
                .physical_mapping
                .iter()
                .all(|m| m.initial_access == if frame == 0 { 0 } else { 0x37 })
        );
        assert_eq!(state.temporal.parity("history"), Some(frame % 2));
        assert_eq!(
            state.temporal.record_state("history:0"),
            Some(TemporalRecordState::Exported)
        );
    }
    assert_eq!(next, 102);
}

#[test]
fn same_frame_duplicate_is_error_without_another_import_or_create() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "texture", texture(8), || Ok(10))
        .unwrap();
    let duplicate =
        state.get_or_create_temporal(&mut graph, "texture", texture(8), || panic!("不应再创建"));
    assert!(matches!(duplicate, Err(TemporalError::AlreadyRequested(_))));
    assert_eq!(state.record_state("texture").unwrap().original_tag(), 0);
    assert_eq!(graph.build().declaration.resources.len(), 1);
}

#[test]
fn buffer_hit_ignores_requested_descriptor_and_keeps_actual_descriptor() {
    let mut state = TemporalRenderGraphState::default();
    let mut first = RenderGraph::new();
    state
        .get_or_create_temporal(&mut first, "buffer", buffer(64), || Ok(20))
        .unwrap();
    assert_eq!(state.record_state("buffer").unwrap().original_tag(), 1);
    state.export_temporal(&mut first).unwrap();
    let mut next = RenderGraph::new();
    let lookup = state
        .get_or_create_temporal(&mut next, "buffer", buffer(128), || {
            panic!("buffer 命中不得重建")
        })
        .unwrap();
    assert!(lookup.reused);
    assert_eq!(lookup.resource.descriptor, buffer(64));
    assert_eq!(next.build().declaration.resources[0].descriptor, buffer(64));
}

#[test]
fn texture_resize_replaces_only_after_success_and_retires_previous_owner() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "resize", texture(8), || Ok(30))
        .unwrap();
    state.export_temporal(&mut graph).unwrap();
    let mut next = RenderGraph::new();
    assert!(
        state
            .get_or_create_temporal(&mut next, "resize", texture(16), || Err(
                BuildError::ResourceLimit
            ))
            .is_err()
    );
    assert_eq!(state.get("resize").unwrap().token, 30);
    assert_eq!(
        state.record_state("resize"),
        Some(TemporalRecordState::Exported)
    );
    assert!(state.retired_tokens().is_empty());
    let new = state
        .get_or_create_temporal(&mut next, "resize", texture(16), || Ok(31))
        .unwrap();
    assert!(!new.reused);
    assert_eq!(new.resource.token, 31);
    assert_eq!(state.retired_tokens(), &[30]);
}

#[test]
fn same_extent_fixed_mismatch_panics_and_wrong_kind_is_error() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "strict", texture(8), || Ok(40))
        .unwrap();
    state.export_temporal(&mut graph).unwrap();
    let Descriptor::Image(mut changed) = texture(8) else {
        unreachable!()
    };
    changed.words_at_00_through_18[1] += 1;
    let mut next = RenderGraph::new();
    assert!(
        catch_unwind(AssertUnwindSafe(|| state.get_or_create_temporal(
            &mut next,
            "strict",
            Descriptor::Image(changed),
            || Ok(41)
        )))
        .is_err()
    );
    assert!(matches!(
        state.get_or_create_temporal(&mut next, "strict", buffer(8), || Ok(42)),
        Err(TemporalError::WrongResourceKind(_))
    ));
    assert_eq!(
        state.record_state("strict"),
        Some(TemporalRecordState::Exported)
    );
}

#[test]
fn pair_second_failure_keeps_first_and_does_not_flip() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    let mut calls = 0;
    assert!(
        state
            .get_or_create_temporal_pair(&mut graph, "partial", texture(8), || {
                calls += 1;
                if calls == 2 {
                    Err(BuildError::ResourceLimit)
                } else {
                    Ok(50)
                }
            })
            .is_err()
    );
    assert_eq!(state.len(), 1);
    assert_eq!(state.parity("partial"), None);
    assert!(matches!(
        state.get_or_create_temporal_pair(&mut graph, "partial", texture(8), || Ok(51)),
        Err(TemporalError::AlreadyRequested(_))
    ));
    state.export_temporal(&mut graph).unwrap();
    let mut next = RenderGraph::new();
    let pair = state
        .get_or_create_temporal_pair(&mut next, "partial", texture(8), || Ok(51))
        .unwrap();
    assert!(!pair.both_reused);
    assert_eq!(pair.parity, 0);
    assert_eq!(
        (pair.first.resource.token, pair.second.resource.token),
        (50, 51)
    );
}

#[test]
fn export_is_idempotent_and_resize_does_not_reset_pair_parity() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal_pair(&mut graph, "pair", texture(8), || Ok(60))
        .unwrap();
    state.export_temporal(&mut graph).unwrap();
    state.export_temporal(&mut graph).unwrap();
    assert_eq!(graph.build().declaration.exports.len(), 2);
    let mut next = RenderGraph::new();
    let resized = state
        .get_or_create_temporal_pair(&mut next, "pair", texture(16), || Ok(61))
        .unwrap();
    assert_eq!(resized.parity, 1);
    assert!(!resized.both_reused);
}

#[test]
fn initial_creation_failure_does_not_insert() {
    let mut state = TemporalRenderGraphState::default();
    assert!(
        state
            .get_or_create_temporal(&mut RenderGraph::new(), "missing", texture(8), || Err(
                BuildError::ResourceLimit
            ))
            .is_err()
    );
    assert!(state.is_empty());
}

#[derive(Default)]
struct Owner {
    next: u64,
    released: Vec<u64>,
}
impl TemporalResourceOwner for Owner {
    fn create_temporal(&mut self, _: &str, _: Descriptor) -> Result<u64, BuildError> {
        self.next += 1;
        Ok(self.next)
    }
    fn release_temporal(&mut self, token: u64) {
        self.released.push(token);
    }
}
#[test]
fn owned_resize_and_clear_release_all_cache_references_once() {
    let mut state = TemporalRenderGraphState::default();
    let mut owner = Owner::default();
    let mut graph = RenderGraph::new();
    state
        .get_or_create_owned(&mut graph, "owned", texture(8), &mut owner)
        .unwrap();
    state.export_temporal(&mut graph).unwrap();
    state
        .get_or_create_owned(&mut RenderGraph::new(), "owned", texture(16), &mut owner)
        .unwrap();
    assert_eq!(owner.released, [1]);
    state.clear_owned(&mut owner);
    state.clear_owned(&mut owner);
    assert_eq!(owner.released, [1, 2]);
    assert!(state.is_empty());
    let mut previous = TemporalRenderGraphState::default();
    previous
        .get_or_create_owned(&mut RenderGraph::new(), "retained", texture(8), &mut owner)
        .unwrap();
    let mut next = TemporalRenderGraphState::default();
    // 外部物理适配中同一 token 的新状态仍持资源；旧状态释放不能使它消失。
    next.get_or_create_temporal(&mut RenderGraph::new(), "retained", texture(8), || Ok(3))
        .unwrap();
    let mut persistent = PersistentCompileState {
        graph: None,
        temporal: Some(previous),
    };
    persistent.restore_temporal_owned(next, &mut owner);
    assert_eq!(owner.released, [1, 2]);
    persistent
        .temporal
        .as_mut()
        .unwrap()
        .clear_owned(&mut owner);
    assert_eq!(owner.released, [1, 2, 3]);
}

#[test]
fn compile_take_failure_does_not_roll_back_taken_graph() {
    let mut owner = PersistentCompileState {
        graph: None,
        temporal: Some(TemporalRenderGraphState::default()),
    };
    assert!(matches!(
        owner.take_for_compile(),
        Err(TakeFailure::GraphMissing)
    ));
    assert!(owner.temporal.is_some());
    owner.graph = Some(RenderGraph::new());
    owner.temporal = None;
    assert!(matches!(
        owner.take_for_compile(),
        Err(TakeFailure::TemporalMissing { .. })
    ));
    assert!(owner.graph.is_none());
}

#[test]
fn population_failure_retains_exported_state_installed_before_population() {
    let mut owner = PersistentCompileState {
        graph: Some(RenderGraph::new()),
        temporal: Some(TemporalRenderGraphState::default()),
    };
    let mut detached = owner.take_for_compile().ok().unwrap();
    let lookup = detached
        .temporal
        .get_or_create_temporal(&mut detached.graph, "history", texture(8), || Ok(70))
        .unwrap();
    let mut pass = detached.graph.add_pass("history");
    pass.add_resource_access(lookup.handle, 40, 0).unwrap();
    pass.render(|_| true);
    let fail = detached
        .graph
        .create_raw_resource(
            "will-fail",
            rhapsody::rg::resource::ResourceOrigin {
                descriptor: buffer(16),
                source: graph::OriginSource::Create,
            },
        )
        .unwrap();
    let mut pass = detached.graph.add_pass("create");
    pass.add_resource_access(fail, 40, 0).unwrap();
    pass.render(|_| true);
    let built = detached.export_and_restore(&mut owner).unwrap();
    assert!(owner.graph.is_none());
    assert_eq!(
        owner.temporal.as_ref().unwrap().record_state("history"),
        Some(TemporalRecordState::Exported)
    );
    let mut backend = EventBackend::default();
    backend.fail_at_creation = Some((0, 1));
    assert!(
        graph::compile_graph(
            built.declaration,
            &mut graph::Pool::default(),
            &mut backend,
            &mut Access
        )
        .is_err()
    );
    assert_eq!(
        owner.temporal.as_ref().unwrap().record_state("history"),
        Some(TemporalRecordState::Exported)
    );
}

#[test]
fn ownership_option_projection_matches_saved_original_machine_code_oracle() {
    // 重用原指令已捕获的 oracle；本项只对照 Option 所有权，不冒充全编译 ABI 差分。
    let oracle: serde_json::Value = serde_json::from_str(include_str!(
        "../../../evidence/render-recovery/parallel-batch-06/temporal-lifecycle/native-oracle.json"
    ))
    .unwrap();
    let qword = |hex: &str| -> u64 {
        let bytes =
            std::array::from_fn(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap());
        u64::from_le_bytes(bytes)
    };
    let mut checked = 0;
    for case in oracle.as_array().unwrap() {
        let input = case["input"].as_array().unwrap();
        let mode = input[0].as_u64().unwrap();
        if mode > 1 {
            continue;
        }
        let gp = input[1].as_u64().unwrap() != 0;
        let tp = input[2].as_u64().unwrap() != 0;
        let native = case["native"]
            .as_str()
            .unwrap()
            .split('|')
            .collect::<Vec<_>>();
        let mut candidate = PersistentCompileState {
            graph: gp.then(RenderGraph::new),
            temporal: tp.then(TemporalRenderGraphState::default),
        };
        if mode == 0 {
            let outcome = match candidate.take_for_compile() {
                Ok(_) => "taken",
                Err(TakeFailure::GraphMissing) => "graph-missing",
                Err(TakeFailure::TemporalMissing { .. }) => "temporal-missing",
            };
            assert_eq!(outcome, native[0]);
        } else {
            candidate.restore_temporal(TemporalRenderGraphState::default());
        }
        assert_eq!(candidate.graph.is_some(), qword(native[1]) != 1u64 << 63);
        assert_eq!(candidate.temporal.is_some(), qword(native[2]) != 0);
        checked += 1;
    }
    assert_eq!(checked, 96);
}
