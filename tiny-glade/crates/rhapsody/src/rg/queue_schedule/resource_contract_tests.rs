use super::*;
use crate::rg::{graph::{Descriptor, RenderGraph}, plan::{GraphDefinition, compile_definition}, resource::AccessRange, vulkan::AccessTable};
use crate::tracking::{QueueFamilies, ResourceSharing};
use crate::rg::resource_registry::BufferDesc;
fn fixture(sharing: ResourceSharing, write: bool, same_family: bool) -> GraphDefinition {
    let mut graph = RenderGraph::new();
    let resource = graph
        .import_unowned_with_sharing(
            "shared object",
            Descriptor::Buffer(BufferDesc {
                word_at_00: 64,
                word_at_08: 0,
                bytes_at_0c_through_0e: [0; 3],
            }),
            1,
            0x25,
            sharing,
        )
        .unwrap();
    let mut buffer = graph.buffer_id(resource).unwrap();
    for (index, class) in [QueueClass::Graphics, QueueClass::Compute]
        .into_iter()
        .enumerate()
    {
        let mut pass = graph
            .add_pass(format!("branch {index}"))
            .with_side_effect()
            .with_queue(class);
        if write {
            buffer = pass
                .write_buffer(
                    buffer,
                    0x34,
                    AccessRange::Buffer {
                        offset: index as u64 * 32,
                        size: 32,
                    },
                )
                .unwrap();
        } else {
            pass.read_buffer(buffer, 0x25, AccessRange::Whole).unwrap();
        }
        pass.render(|_| true);
    }
    let mut context = PlanContext {
        queue_domains: Some([
            QueueDomain {
                queue_index: 0,
                family: 0,
            },
            QueueDomain {
                queue_index: 1,
                family: 1,
            },
            QueueDomain {
                queue_index: 2,
                family: 2,
            },
        ]),
        ..Default::default()
    };
    if same_family {
        context.queue_domains.as_mut().unwrap()[1].family = 0;
    }
    GraphDefinition::from_declaration(
        &graph.build().declaration,
        &AccessTable::original(),
        context,
    )
    .unwrap()
}
#[test]
fn unordered_exclusive_cross_family_reads_and_disjoint_writes_are_rejected_without_added_edges()
{
    for write in [false, true] {
        let cross_family =
            compile_definition(fixture(ResourceSharing::Exclusive, write, false));
        if write {
            assert!(
                cross_family.is_ok(),
                "disjoint ranges may have different owners"
            );
        } else {
            assert_eq!(
                cross_family.unwrap_err().kind(),
                crate::ErrorKind::Unsupported
            );
        }
        let plan =
            compile_definition(fixture(ResourceSharing::Exclusive, write, true)).unwrap();
        assert!(plan.dependency_edges.is_empty());
        assert!(plan.queue_schedule.dependencies.iter().all(Vec::is_empty));
    }
}
#[test]
fn concurrent_full_family_lists_allow_unordered_ranges_and_missing_families_are_rejected() {
    for write in [false, true] {
        let plan = compile_definition(fixture(
            ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1]).unwrap()),
            write,
            false,
        ))
        .unwrap();
        assert!(plan.dependency_edges.is_empty());
        assert!(plan.queue_schedule.dependencies.iter().all(Vec::is_empty));
        assert!(
            compile_definition(fixture(
                ResourceSharing::Concurrent(QueueFamilies::new(&[0, 2]).unwrap()),
                write,
                false
            ))
            .is_err()
        );
    }
}
#[test]
fn final_export_batch_requires_the_actual_foreign_producer_wait_in_the_independent_checker() {
    let mut graph = RenderGraph::new();
    let resource = graph
        .import_unowned(
            "compute output",
            Descriptor::Buffer(BufferDesc {
                word_at_00: 64,
                word_at_08: 0,
                bytes_at_0c_through_0e: [0; 3],
            }),
            1,
            0x25,
        )
        .unwrap();
    let mut pass = graph
        .add_pass("foreign producer")
        .with_queue(QueueClass::Compute);
    pass.add_resource_access(resource, 0x34, 0).unwrap();
    pass.render(|_| true);
    graph.export(resource, 0x26).unwrap();
    let context = PlanContext {
        queue_domains: Some([
            QueueDomain {
                queue_index: 0,
                family: 0,
            },
            QueueDomain {
                queue_index: 1,
                family: 1,
            },
            QueueDomain {
                queue_index: 2,
                family: 2,
            },
        ]),
        ..Default::default()
    };
    let definition = GraphDefinition::from_declaration(
        &graph.build().declaration,
        &AccessTable::original(),
        context,
    )
    .unwrap();
    let mut plan = compile_definition(definition.clone()).unwrap();
    assert_eq!(plan.batches.len(), 2);
    assert_eq!(plan.batches[1].pass_range, 1..1);
    assert_eq!(plan.queue_schedule.dependencies[1], vec![0]);
    plan.queue_schedule.dependencies[1].clear();
    assert!(crate::rg::plan_check::check_plan(&definition, &plan).is_err());
}
