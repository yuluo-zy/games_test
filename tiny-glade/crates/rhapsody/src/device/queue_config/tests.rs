use super::*;
use crate::error::ErrorKind;

const G: vk::QueueFlags = vk::QueueFlags::GRAPHICS;
const C: vk::QueueFlags = vk::QueueFlags::COMPUTE;
const T: vk::QueueFlags = vk::QueueFlags::TRANSFER;

fn family(flags: vk::QueueFlags, count: u32, present: bool) -> FamilyInfo {
    FamilyInfo {
        flags,
        count,
        present,
        min_image_transfer_granularity: vk::Extent3D {
            width: 1,
            height: 1,
            depth: 1,
        },
    }
}
fn coarse(mut f: FamilyInfo) -> FamilyInfo {
    f.min_image_transfer_granularity = vk::Extent3D {
        width: 8,
        height: 8,
        depth: 8,
    };
    f
}
fn pq(family: u32, index: u32) -> PhysicalQueue {
    PhysicalQueue { family, index }
}

#[test]
fn single_queue_device_aliases_everything_or_refuses_requirements() {
    // lavapipe / 多数 Intel 集显：一个族、一个队列。
    let families = [family(G | C | T, 1, true)];
    let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
    assert_eq!(plan.queues, [pq(0, 0)]);
    assert!(plan.is_aliased(QueueClass::Compute) && plan.is_aliased(QueueClass::Transfer));
    assert_eq!(plan.create_counts(), [(0, 1)]);

    let require = QueueConfig {
        async_compute: QueuePreference::Require,
        ..QueueConfig::default()
    };
    assert_eq!(
        plan_queues(&families, &require).unwrap_err().kind(),
        ErrorKind::Unsupported
    );
}

#[test]
fn off_never_creates_extra_queues() {
    let families = [
        family(G | C | T, 16, true),
        family(C | T, 8, false),
        family(T, 2, false),
    ];
    let plan = plan_queues(&families, &QueueConfig::default()).unwrap();
    assert_eq!(plan.queues, [pq(0, 0)]);
}

#[test]
fn discrete_layout_uses_dedicated_compute_and_dma_families() {
    // 常见独显：通用族 + 专用计算族 + 专用 DMA 族。
    let families = [
        family(G | C | T, 16, true),
        family(T, 2, false),
        family(C | T, 8, false),
    ];
    let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
    assert_eq!(plan.physical(QueueClass::Graphics), pq(0, 0));
    assert_eq!(plan.physical(QueueClass::Compute), pq(2, 0));
    assert_eq!(plan.physical(QueueClass::Transfer), pq(1, 0));
    assert_eq!(plan.create_counts(), [(0, 1), (1, 1), (2, 1)]);
    assert_eq!(plan.families(), [0, 1, 2]);
    assert!(!plan.is_aliased(QueueClass::Compute) && !plan.is_aliased(QueueClass::Transfer));
}

#[test]
fn coarse_granularity_dma_is_skipped_for_a_spare_compute_queue() {
    let families = [
        family(G | C | T, 1, true),
        family(C | T, 4, false),
        coarse(family(T, 2, false)),
    ];
    let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
    assert_eq!(plan.physical(QueueClass::Compute), pq(1, 0));
    assert_eq!(
        plan.physical(QueueClass::Transfer),
        pq(1, 1),
        "粗粒度 DMA 族不用于图像传输"
    );
    assert_eq!(plan.create_counts(), [(0, 1), (1, 2)]);
}

#[test]
fn second_queue_of_the_graphics_family_serves_async_compute() {
    let families = [family(G | C | T, 2, true)];
    let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
    assert_eq!(plan.physical(QueueClass::Compute), pq(0, 1));
    assert!(plan.is_aliased(QueueClass::Transfer));
    assert_eq!(plan.create_counts(), [(0, 2)]);
}

#[test]
fn graphics_family_must_present_when_required() {
    let families = [family(G | C | T, 1, false), family(G | C | T, 1, true)];
    let plan = plan_queues(&families, &QueueConfig::default()).unwrap();
    assert_eq!(plan.physical(QueueClass::Graphics), pq(1, 0));

    let headless = QueueConfig {
        require_present: false,
        ..QueueConfig::default()
    };
    assert_eq!(
        plan_queues(&families, &headless)
            .unwrap()
            .physical(QueueClass::Graphics),
        pq(0, 0)
    );

    let no_present = [family(G | C | T, 1, false)];
    assert_eq!(
        plan_queues(&no_present, &QueueConfig::default())
            .unwrap_err()
            .kind(),
        ErrorKind::Unsupported
    );
    assert_eq!(
        plan_queues(&[], &headless).unwrap_err().kind(),
        ErrorKind::Unsupported
    );
}

#[test]
fn graphics_only_family_cannot_back_a_compute_alias() {
    let families = [family(G | T, 4, true)];
    for config in [QueueConfig::default(), QueueConfig::prefer_all()] {
        assert_eq!(
            plan_queues(&families, &config).unwrap_err().kind(),
            ErrorKind::Unsupported
        );
    }
}

#[test]
fn transfer_uses_a_spare_graphics_queue_before_other_general_families() {
    let families = [family(G | C | T, 3, true), family(G | C | T, 1, false)];
    let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
    assert_eq!(plan.physical(QueueClass::Compute), pq(0, 1));
    assert_eq!(plan.physical(QueueClass::Transfer), pq(0, 2));
    assert_eq!(plan.create_counts(), [(0, 3)]);
    let plan = plan_queues(
        &families,
        &QueueConfig {
            transfer: QueuePreference::Require,
            ..QueueConfig::default()
        },
    )
    .unwrap();
    assert_eq!(plan.physical(QueueClass::Transfer), pq(0, 1));
}

#[test]
fn separate_general_families_supply_independent_compute_and_transfer_queues() {
    // MoltenVK can expose multiple one-queue universal families with no
    // dedicated COMPUTE or DMA flags. They are still distinct VkQueues.
    let families = [family(G | C | T, 1, true); 4];
    for preference in [QueuePreference::Prefer, QueuePreference::Require] {
        let plan = plan_queues(
            &families,
            &QueueConfig {
                async_compute: preference,
                transfer: preference,
                require_present: true,
            },
        )
        .unwrap();
        assert_eq!(plan.queues, [pq(0, 0), pq(1, 0), pq(2, 0)]);
        assert_eq!(plan.create_counts(), [(0, 1), (1, 1), (2, 1)]);
        assert!(!plan.is_aliased(QueueClass::Compute));
        assert!(!plan.is_aliased(QueueClass::Transfer));
        assert_ne!(
            plan.queue(QueueClass::Compute),
            plan.queue(QueueClass::Transfer)
        );
    }
    let off = plan_queues(&families, &QueueConfig::default()).unwrap();
    assert_eq!(off.queues, [pq(0, 0)]);
}

#[test]
fn compute_prefers_the_graphics_familys_second_queue_before_another_general_family() {
    let families = [family(G | C | T, 2, true), family(G | C | T, 1, false)];
    let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
    assert_eq!(plan.physical(QueueClass::Compute), pq(0, 1));
    assert_eq!(plan.physical(QueueClass::Transfer), pq(1, 0));
}

#[test]
fn exhausted_general_families_alias_prefer_and_reject_only_unavailable_requirements() {
    let families = [family(G | C | T, 1, true), family(G | C | T, 1, false)];
    let plan = plan_queues(&families, &QueueConfig::prefer_all()).unwrap();
    assert_eq!(plan.physical(QueueClass::Compute), pq(1, 0));
    assert!(plan.is_aliased(QueueClass::Transfer));
    let require_both = QueueConfig {
        async_compute: QueuePreference::Require,
        transfer: QueuePreference::Require,
        ..QueueConfig::default()
    };
    assert_eq!(
        plan_queues(&families, &require_both).unwrap_err().kind(),
        ErrorKind::Unsupported
    );
    let transfer_only = plan_queues(
        &families,
        &QueueConfig {
            transfer: QueuePreference::Require,
            ..QueueConfig::default()
        },
    )
    .unwrap();
    assert_eq!(transfer_only.physical(QueueClass::Transfer), pq(1, 0));
}

#[test]
fn transfer_requirement_without_any_candidate_fails() {
    let families = [family(G | C | T, 1, true)];
    let config = QueueConfig {
        transfer: QueuePreference::Require,
        ..QueueConfig::default()
    };
    assert_eq!(
        plan_queues(&families, &config).unwrap_err().kind(),
        ErrorKind::Unsupported
    );
}
