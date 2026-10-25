//! Replaying a plan into a `RenderGraph` (resources in plan order + adopted
//! passes) must declare exactly what normal declaration code declared.
use rhapsody::rg::{
    graph::{BufferDesc, Descriptor, OriginSource, RenderCallback, RenderGraph},
    plan::{GraphDefinition, PlanContext, compile_definition},
    resource::ResourceOrigin,
    vulkan::AccessTable,
};

const READ: u8 = 0x26;
const WRITE: u8 = 0x34;

fn descriptor() -> Descriptor {
    Descriptor::Buffer(BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    })
}
fn origin(source: OriginSource) -> ResourceOrigin {
    ResourceOrigin {
        descriptor: descriptor(),
        source,
    }
}
fn normal(token: u64) -> RenderGraph {
    let mut graph = RenderGraph::new();
    let imported = graph
        .create_raw_resource(
            "imported",
            origin(OriginSource::ImportOwned {
                token,
                initial_access: READ,
            }),
        )
        .unwrap();
    let created = graph
        .create_raw_resource("created", origin(OriginSource::Create))
        .unwrap();
    let mut a = graph.add_pass("A");
    a.add_resource_access(created, WRITE, 0).unwrap();
    a.add_resource_access(imported, READ, 0).unwrap();
    a.render(|_| true);
    let mut b = graph.add_pass("B").with_side_effect();
    b.add_resource_access(created, READ, 0).unwrap();
    b.render(|_| true);
    graph.export(imported, READ).unwrap();
    graph
}
fn definition(graph: &RenderGraph) -> GraphDefinition {
    GraphDefinition::from_declaration(
        graph.declaration(),
        &mut AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap()
}
fn callbacks(count: usize) -> Vec<RenderCallback> {
    (0..count)
        .map(|_| -> RenderCallback { Box::new(|_| true) })
        .collect()
}

#[test]
fn replayed_graph_declares_what_the_normal_path_declared() {
    let first = normal(7);
    let plan = compile_definition(definition(&first)).unwrap();

    // Next frame: a different import object, otherwise the same shape.
    let expected = normal(99);
    let mut replay = RenderGraph::new();
    let definition = &plan.definition;
    replay
        .create_raw_resource(
            definition.resources[0].name.clone(),
            origin(OriginSource::ImportOwned {
                token: 99,
                initial_access: READ,
            }),
        )
        .unwrap();
    let created = replay
        .declare_planned_create(&definition.resources[1])
        .unwrap();
    replay
        .adopt_planned_passes(definition, callbacks(definition.passes.len()))
        .unwrap();
    replay
        .export(replay.resource_handle(0).unwrap(), READ)
        .unwrap();
    assert_eq!(created.raw(), 1);

    let (got, got_callbacks, _) = replay.build().into_parts();
    let (want, want_callbacks, _) = expected.build().into_parts();
    assert_eq!(got.resources, want.resources);
    assert_eq!(got.exports, want.exports);
    assert_eq!(got.passes.len(), want.passes.len());
    assert_eq!(got_callbacks.len(), want_callbacks.len());
    // Pass names and resolved accesses are what the plan will recompile from.
    assert_eq!(
        compile_definition(
            GraphDefinition::from_declaration(
                &got,
                &mut AccessTable::original(),
                PlanContext::default()
            )
            .unwrap()
        )
        .unwrap()
        .definition,
        plan.definition
    );
}

#[test]
fn adopting_rejects_a_replay_that_differs_from_the_plan() {
    let plan = compile_definition(definition(&normal(1))).unwrap();
    let definition = &plan.definition;
    let owned = |replay: &mut RenderGraph| {
        replay
            .create_raw_resource(
                definition.resources[0].name.clone(),
                origin(OriginSource::ImportOwned {
                    token: 5,
                    initial_access: READ,
                }),
            )
            .unwrap();
    };
    // Missing resource.
    let mut replay = RenderGraph::new();
    owned(&mut replay);
    assert!(
        replay
            .adopt_planned_passes(definition, callbacks(2))
            .is_err()
    );
    // Renamed resource.
    let mut replay = RenderGraph::new();
    owned(&mut replay);
    replay
        .create_raw_resource("other", origin(OriginSource::Create))
        .unwrap();
    assert!(
        replay
            .adopt_planned_passes(definition, callbacks(2))
            .is_err()
    );
    // Wrong recorder count.
    let mut replay = RenderGraph::new();
    owned(&mut replay);
    replay
        .declare_planned_create(&definition.resources[1])
        .unwrap();
    assert!(
        replay
            .adopt_planned_passes(definition, callbacks(1))
            .is_err()
    );
    // A planned import cannot be declared as a created resource.
    assert!(
        RenderGraph::new()
            .declare_planned_create(&definition.resources[0])
            .is_err()
    );
}
