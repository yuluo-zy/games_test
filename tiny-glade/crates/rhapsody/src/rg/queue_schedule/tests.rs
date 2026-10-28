use super::*;
fn pass(class: QueueClass) -> PassDeclaration {
    PassDeclaration {
        graph_id: None,
        name: "".into(),
        accesses: vec![],
        side_effect: true,
        queue_class: class,
    }
}
#[test]
fn physical_aliases_preserve_single_queue_batches_and_have_no_extra_waits() {
    let passes = [
        pass(QueueClass::Graphics),
        pass(QueueClass::Compute),
        pass(QueueClass::Transfer),
    ];
    let (batches, schedule) = plan_queue_batches(
        &passes,
        &[0, 1, 2],
        &[Batch {
            pass_range: 0..3,
            export_barriers: true,
        }],
        &[(0, 1), (1, 2)],
        &[],
        &PlanContext::default(),
    )
    .unwrap();
    assert_eq!(batches.len(), 1);
    assert_eq!(schedule.dependencies, vec![Vec::<usize>::new()]);
}
#[test]
fn independent_queue_branches_have_no_cross_queue_wait_and_only_dag_join_waits() {
    let passes = [
        pass(QueueClass::Graphics),
        pass(QueueClass::Compute),
        pass(QueueClass::Graphics),
    ];
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
    let (batches, schedule) = plan_queue_batches(
        &passes,
        &[0, 1, 2],
        &[Batch {
            pass_range: 0..3,
            export_barriers: true,
        }],
        &[(0, 2), (1, 2)],
        &[],
        &context,
    )
    .unwrap();
    assert_eq!(batches.len(), 3);
    assert!(schedule.dependencies[0].is_empty());
    assert!(schedule.dependencies[1].is_empty());
    assert_eq!(schedule.dependencies[2], vec![1]);
}
