//! A plan can stand in for the declaration code: `declaration_for_frame` must
//! rebuild what a normal frame declared, so that compiling it again yields the
//! very same definition (the retained-graph contract).
use rhapsody::rg::{graph::{Descriptor, GraphDeclaration, OriginSource, PassDeclaration, ResourceDeclaration}, plan::{GraphDefinition, ImportBinding, PlanContext, compile_definition}, resource::AccessRange, vulkan::AccessTable};
use rhapsody::rg::resource_info::{AccessDeclaration, ExportDeclaration};
use rhapsody::rg::resource_registry::BufferDesc;

const READ: u8 = 0x26;
const WRITE: u8 = 0x34;

fn buffer(name: &str, source: OriginSource) -> ResourceDeclaration {
    ResourceDeclaration {
        sharing: rhapsody::tracking::ResourceSharing::Exclusive,
        name: name.into(),
        descriptor: Descriptor::Buffer(BufferDesc {
            word_at_00: 64,
            word_at_08: 0,
            bytes_at_0c_through_0e: [0; 3],
        }),
        source,
    }
}
fn access(resource: u32, code: u8) -> AccessDeclaration {
    AccessDeclaration {
        resource_index: resource,
        access_code: code,
        range: AccessRange::Whole,
        ..Default::default()
    }
}
fn pass(name: &str, accesses: Vec<AccessDeclaration>, side_effect: bool) -> PassDeclaration {
    PassDeclaration {
        queue_class: rhapsody::device::QueueClass::Graphics,
        graph_id: None,
        name: name.into(),
        accesses,
        side_effect,
    }
}
/// import(0) -> pass A writes created(1) -> pass B reads it, exports import(0)
/// untouched; a culled pass C writes created(2) that nothing reads.
fn frame(token: u64) -> GraphDeclaration {
    GraphDeclaration {
        resources: vec![
            buffer(
                "imported",
                OriginSource::ImportOwned {
                    token,
                    initial_access: READ,
                },
            ),
            buffer("created", OriginSource::Create),
            buffer("dead", OriginSource::Create),
        ],
        passes: vec![
            pass("A", vec![access(1, WRITE), access(0, READ)], false),
            pass("B", vec![access(1, READ)], true),
            pass("C", vec![access(2, WRITE)], false),
        ],
        exports: vec![ExportDeclaration {
            tag: 1,
            resource_index: 0,
            access_code: READ,
            range: AccessRange::Whole,
        }],
        ..Default::default()
    }
}
fn definition(graph: &GraphDeclaration) -> GraphDefinition {
    GraphDefinition::from_declaration(graph, &AccessTable::original(), PlanContext::default())
        .unwrap()
}

#[test]
fn rebuilt_declaration_compiles_to_the_identical_definition_and_plan() {
    let declared = frame(7);
    let first = compile_definition(definition(&declared)).unwrap();
    // The next frame imports another object under the same plan.
    let import = ImportBinding {
        logical: 0,
        source: OriginSource::ImportOwned {
            token: 99,
            initial_access: READ,
        },
    };
    let rebuilt = first
        .definition
        .declaration_for_frame(None, &[import])
        .unwrap();
    assert_eq!(rebuilt.resources.len(), declared.resources.len());
    assert_eq!(rebuilt.resources[0].source, import.source);
    assert_eq!(rebuilt.resources[1].source, OriginSource::Create);
    assert_eq!(rebuilt.exports, declared.exports);
    assert_eq!(rebuilt.passes.len(), declared.passes.len());
    let again = compile_definition(definition(&rebuilt)).unwrap();
    assert_eq!(again.definition, first.definition);
    assert_eq!(again.retained_passes, first.retained_passes);
    assert_eq!(again.batches.len(), first.batches.len());
    // Same imports as the declared frame reproduce it exactly.
    let same = first
        .definition
        .declaration_for_frame(
            None,
            &[ImportBinding {
                logical: 0,
                source: declared.resources[0].source,
            }],
        )
        .unwrap();
    assert_eq!(same.resources, declared.resources);
}

#[test]
fn import_bindings_must_match_the_planned_origins_exactly() {
    let plan = compile_definition(definition(&frame(1))).unwrap();
    let owned = |logical, token| ImportBinding {
        logical,
        source: OriginSource::ImportOwned {
            token,
            initial_access: READ,
        },
    };
    let d = &plan.definition;
    assert!(
        d.declaration_for_frame(None, &[]).is_err(),
        "missing import"
    );
    assert!(
        d.declaration_for_frame(None, &[owned(0, 1), owned(0, 2)])
            .is_err(),
        "duplicate"
    );
    assert!(
        d.declaration_for_frame(None, &[owned(0, 1), owned(1, 2)])
            .is_err(),
        "binding a created resource"
    );
    assert!(
        d.declaration_for_frame(None, &[owned(9, 1)]).is_err(),
        "out of range"
    );
    let unowned = ImportBinding {
        logical: 0,
        source: OriginSource::ImportUnowned {
            token: 1,
            initial_access: READ,
        },
    };
    assert!(
        d.declaration_for_frame(None, &[unowned]).is_err(),
        "wrong source kind"
    );
}
