use tiny_glade_graph_compiler_r2::dsl::parse;
use tiny_glade_graph_compiler_r2::{
    compile_graph, AccessDeclaration, AccessDependency, Descriptor, EventBackend, GraphDeclaration,
    OriginSource, PassDeclaration, Pool, ResourceDeclaration, TextureDesc,
};

struct Dependency;
impl AccessDependency for Dependency {
    fn access_mask(&mut self, code: u8) -> u32 {
        match code {
            1 => 0x20,
            2 => 0x40,
            3 => 0x80,
            4 => 0x800,
            _ => 0,
        }
    }
    fn is_write(&mut self, code: u8) -> bool {
        matches!(code, 2 | 3)
    }
}

fn image(id: u32) -> Descriptor {
    Descriptor::Image(TextureDesc {
        words_at_00_through_18: [id, 0, 0, 0, 0, 0, 0],
        bytes_at_1c_through_1e: [0, 0, 0],
    })
}

#[test]
fn dsl_snapshot_covers_order_ranges_mapping_and_barriers() {
    let fixture = parse("batch per-pass\nresource 0 7 image\nresource 1 9 buffer import-unowned 4\npass upload 0:0:1 1:0:2\npass draw 0:0:3\nexport 0 4").unwrap();
    let mut pool = fixture.pool;
    let mut backend = fixture.backend;
    let mut dependency = Dependency;
    let compiled = compile_graph(fixture.graph, &mut pool, &mut backend, &mut dependency).unwrap();
    assert_eq!(compiled.snapshot.pass_order, ["upload", "draw"]);
    assert_eq!(compiled.snapshot.pass_ranges, [0..2, 2..3]);
    assert_eq!(compiled.snapshot.accesses[2].previous_access_index, 0);
    assert_eq!(compiled.snapshot.accesses[0].next_access_index, 2);
    assert_eq!(compiled.snapshot.physical_mapping.len(), 2);
    assert!(compiled.snapshot.batches.last().unwrap().export_barriers);
    assert_eq!(compiled.snapshot.imports, [1]);
    assert_eq!(compiled.snapshot.exports, [0]);
}

#[test]
fn empty_pass_and_empty_graph_compile() {
    let graph = GraphDeclaration {
        resources: vec![ResourceDeclaration {
            name: "empty".into(),
            descriptor: image(1),
            source: OriginSource::Create,
        }],
        passes: vec![PassDeclaration {
            name: "empty-pass".into(),
            accesses: vec![],
        }],
        exports: vec![],
        batch_per_pass: false,
    };
    let mut pool = Pool::default();
    let mut backend = EventBackend::default();
    let mut dependency = Dependency;
    let compiled = compile_graph(graph, &mut pool, &mut backend, &mut dependency).unwrap();
    assert_eq!(compiled.snapshot.pass_ranges.len(), 1);
    assert_eq!(compiled.snapshot.pass_ranges[0], 0..0);
    let mut dependency = Dependency;
    let empty = compile_graph(
        GraphDeclaration::default(),
        &mut pool,
        &mut backend,
        &mut dependency,
    )
    .unwrap();
    assert!(empty.snapshot.batches.is_empty());
}

#[test]
fn invalid_access_is_rejected_before_backend_creation() {
    let graph = GraphDeclaration {
        resources: vec![ResourceDeclaration {
            name: "r".into(),
            descriptor: image(1),
            source: OriginSource::Create,
        }],
        passes: vec![PassDeclaration {
            name: "bad".into(),
            accesses: vec![AccessDeclaration {
                resource_index: 4,
                synchronization_group: 0,
                access_code: 1,
            }],
        }],
        exports: vec![],
        batch_per_pass: false,
    };
    let mut pool = Pool::default();
    let mut backend = EventBackend::default();
    let mut dependency = Dependency;
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| compile_graph(
            graph,
            &mut pool,
            &mut backend,
            &mut dependency
        )))
        .is_err()
    );
    assert!(backend.events.is_empty());
}

#[test]
fn imported_ownership_export_moves_and_initial_state() {
    let mut fixture = parse("resource 0 7 image import-owned 5\nresource 1 9 buffer import-unowned 4\npass empty\nexport 0 4").unwrap();
    let compiled = compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    )
    .unwrap();
    assert_eq!(compiled.snapshot.physical_mapping[0].payload_tag, 0);
    assert_eq!(compiled.snapshot.physical_mapping[1].payload_tag, 3);
    assert!(compiled.snapshot.initial.is_empty());
    assert_eq!(compiled.snapshot.export[0].from, 5);
    let exports = compiled.into_exports();
    assert_eq!(exports.exported.len(), 1);
    assert_eq!(exports.remaining.len(), 1);
}

#[test]
fn first_transition_is_initial_unless_reused_and_drains_once() {
    let mut fixture = parse("resource 0 7 image import-owned 5\npass first 0:0:1").unwrap();
    let mut compiled = compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    )
    .unwrap();
    assert_eq!(compiled.snapshot.initial[0].from, 5);
    assert!(compiled.snapshot.pre[0].is_empty());
    let mut pending = vec![0];
    assert_eq!(
        tiny_glade_graph_compiler_r2::initial_transitions(
            &compiled.info,
            &compiled.registry,
            &mut pending
        )
        .len(),
        1
    );
    assert!(pending.is_empty());
    assert!(tiny_glade_graph_compiler_r2::initial_transitions(
        &compiled.info,
        &compiled.registry,
        &mut pending
    )
    .is_empty());
    compiled.registry.resources[0].reused = true;
    compiled.registry.resource_initial_access[0] = 0;
    let mut pending = vec![0];
    assert!(tiny_glade_graph_compiler_r2::initial_transitions(
        &compiled.info,
        &compiled.registry,
        &mut pending
    )
    .is_empty());
    assert_eq!(
        tiny_glade_graph_compiler_r2::pre_pass_transitions(
            &compiled.info,
            &compiled.registry,
            0..1,
            &mut Dependency
        )[0]
        .from,
        0
    );
    compiled.registry.resource_initial_access[0] = 5;
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tiny_glade_graph_compiler_r2::pre_pass_transitions(
            &compiled.info,
            &compiled.registry,
            0..1,
            &mut Dependency,
        )
    }))
    .is_err());
}

#[test]
fn pool_hit_is_lifo_and_still_uses_initial_transition() {
    let mut fixture = parse("resource 0 7 image\npass first 0:0:1").unwrap();
    let descriptor = fixture.graph.resources[0].descriptor.with_usage(7 | 4);
    let Descriptor::Image(desc) = descriptor else {
        unreachable!()
    };
    fixture
        .pool
        .insert_texture(tiny_glade_graph_compiler_r2::Texture { handle: 90, desc });
    fixture
        .pool
        .insert_texture(tiny_glade_graph_compiler_r2::Texture { handle: 91, desc });
    let compiled = compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    )
    .unwrap();
    assert_eq!(compiled.snapshot.pool_hits, [0]);
    assert_eq!(compiled.snapshot.physical_mapping[0].token, Some(91));
    assert!(!compiled.snapshot.physical_mapping[0].reused);
    assert_eq!(compiled.snapshot.initial.len(), 1);
    assert_eq!(fixture.backend.events.len(), 1);
}

#[test]
fn no_graph_local_alias_is_produced_for_equal_descriptors() {
    let mut fixture =
        parse("resource 0 7 image\nresource 1 7 image\npass first 0:0:1\npass second 1:0:1")
            .unwrap();
    let compiled = compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    )
    .unwrap();
    assert_ne!(
        compiled.snapshot.physical_mapping[0].physical,
        compiled.snapshot.physical_mapping[1].physical
    );
    assert_ne!(
        compiled.snapshot.physical_mapping[0].token,
        compiled.snapshot.physical_mapping[1].token
    );
}

#[test]
fn backend_failure_preserves_partial_consumption_and_does_not_rollback_pool() {
    let mut fixture =
        parse("resource 0 7 image\nresource 1 8 image\npass first 0:0:1 1:0:1").unwrap();
    fixture.backend.fail_at_creation = Some((1, 3));
    let failure = match compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    ) {
        Err(failure) => failure,
        Ok(_) => panic!("must fail"),
    };
    assert_eq!(failure.error.discriminant, 3);
    assert_eq!(failure.partial.registry.backing_resources.len(), 1);
    assert!(failure
        .partial
        .remaining_origins
        .iter()
        .all(Option::is_none));
    assert_eq!(fixture.backend.events.len(), 2);
}

#[test]
fn fatal_backend_tags_panic() {
    for tag in [9, 10] {
        let mut fixture = parse("resource 0 7 image\npass first 0:0:1").unwrap();
        fixture.backend.fail_at_creation = Some((0, tag));
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| compile_graph(
                fixture.graph,
                &mut fixture.pool,
                &mut fixture.backend,
                &mut Dependency
            )))
            .is_err()
        );
    }
}

#[test]
fn access_concurrency_uses_external_classifier() {
    for (group, code, expected) in [(0, 1, 0), (3, 2, 0), (0, 2, 1)] {
        let mut fixture = parse(&format!(
            "resource 0 7 image\npass first 0:{group}:{code}\npass second 0:{group}:{code}"
        ))
        .unwrap();
        let compiled = compile_graph(
            fixture.graph,
            &mut fixture.pool,
            &mut fixture.backend,
            &mut Dependency,
        )
        .unwrap();
        assert_eq!(compiled.snapshot.pre[1].len(), expected);
    }
}

#[test]
fn invalid_pre_ranges_and_wrong_barrier_payload_panic() {
    let mut fixture = parse("resource 0 7 image\npass first 0:0:1").unwrap();
    let mut compiled = compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    )
    .unwrap();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tiny_glade_graph_compiler_r2::pre_pass_transitions(
            &compiled.info,
            &compiled.registry,
            0..9,
            &mut Dependency,
        )
    }))
    .is_err());
    compiled.registry.backing_resources[0] = tiny_glade_graph_compiler_r2::RegistryResource::Empty;
    let mut pending = vec![0];
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tiny_glade_graph_compiler_r2::initial_transitions(
            &compiled.info,
            &compiled.registry,
            &mut pending,
        )
    }))
    .is_err());
    assert!(pending.is_empty());
}

#[test]
fn event_order_controls_physical_mapping_and_unused_resource_is_unmapped() {
    let mut fixture = parse("resource 0 7 image\nresource 1 8 image\nresource 2 9 buffer\npass first 1:0:1\npass second 0:0:1").unwrap();
    let compiled = compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    )
    .unwrap();
    assert_eq!(compiled.snapshot.physical_mapping[1].physical, 0);
    assert_eq!(compiled.snapshot.physical_mapping[0].physical, 1);
    assert_eq!(compiled.snapshot.physical_mapping[2].physical, u16::MAX);
}

#[test]
fn unused_create_export_panics_at_final_mapping_boundary() {
    let mut fixture = parse("resource 0 7 image\npass empty\nexport 0 4").unwrap();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| compile_graph(
            fixture.graph,
            &mut fixture.pool,
            &mut fixture.backend,
            &mut Dependency
        )))
        .is_err()
    );
    assert!(fixture.backend.events.is_empty());
}

#[test]
fn zero_usage_population_emits_empty_without_backend() {
    let mut fixture = parse("resource 0 0 buffer\npass first 0:0:0").unwrap();
    let origins = [tg_resource_info_candidate::OriginKind::Tag0 {
        descriptor_code: 7,
        image_creation_flags: 0,
        buffer_creation_flags: 0,
    }];
    let passes = [fixture.graph.passes[0].accesses.as_slice()];
    let mut info = tiny_glade_graph_compiler_r2::ResourceInfo::default();
    info.calculate(&origins, &[], &passes, |_| 0);
    let population = tiny_glade_graph_compiler_r2::populate_resources(
        &info,
        &fixture.graph.resources,
        &mut fixture.pool,
        &mut fixture.backend,
    )
    .unwrap();
    assert!(matches!(
        population.registry.backing_resources[0],
        tiny_glade_graph_compiler_r2::RegistryResource::Empty
    ));
    assert!(fixture.backend.events.is_empty());
}

#[test]
fn external_dependency_table_is_required_and_parser_rejects_bad_input() {
    let mut fixture = parse("resource 0 7 image\npass first 0:0:1").unwrap();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| compile_graph(
            fixture.graph,
            &mut fixture.pool,
            &mut fixture.backend,
            &mut fixture.dependency
        )))
        .is_err()
    );
    assert!(parse("resource 2 7 image").is_err());
    assert!(parse("unknown foo").is_err());
    assert_eq!(
        tiny_glade_graph_compiler_r2::snapshot::json_string("a\nb\t"),
        "\"a\\nb\\t\""
    );
}

#[test]
fn one_whole_graph_batch_has_final_flag_and_zero_graph_has_no_export_event() {
    let mut fixture =
        parse("resource 0 7 image import-owned 5\npass one 0:0:1\npass empty\nexport 0 4").unwrap();
    let compiled = compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    )
    .unwrap();
    assert_eq!(compiled.snapshot.batches.len(), 1);
    assert_eq!(compiled.snapshot.batches[0].pass_range, 0..2);
    assert!(compiled.snapshot.batches[0].export_barriers);
    let mut fixture = parse("resource 0 7 image import-owned 5\nexport 0 4").unwrap();
    let compiled = compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    )
    .unwrap();
    assert!(compiled.snapshot.batches.is_empty());
    assert!(compiled.snapshot.export.is_empty());
}

#[test]
fn failed_population_does_not_restore_taken_pool_resource() {
    let mut fixture =
        parse("resource 0 7 image\nresource 1 8 image\npass first 0:0:1 1:0:1").unwrap();
    let Descriptor::Image(desc) = fixture.graph.resources[0].descriptor.with_usage(7 | 4) else {
        unreachable!()
    };
    fixture
        .pool
        .insert_texture(tiny_glade_graph_compiler_r2::Texture { handle: 99, desc });
    fixture.backend.fail_at_creation = Some((0, 0));
    let failure = match compile_graph(
        fixture.graph,
        &mut fixture.pool,
        &mut fixture.backend,
        &mut Dependency,
    ) {
        Err(failure) => failure,
        Ok(_) => panic!("must fail"),
    };
    assert_eq!(failure.partial.pool_hits, [0]);
    assert_eq!(failure.partial.registry.backing_resources.len(), 1);
    assert!(fixture.pool.take_texture(&desc).is_none());
}
