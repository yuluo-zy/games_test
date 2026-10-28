//! 私有包装的峰值/错误/快照规则；公开分配器的池算法不在这里复制。
use ash::vk;
use rhapsody::device::allocator::{AllocationBackend, AllocationRequest, GpuAllocator, MemoryLocation};
use rhapsody::accounting::{AllocationTotals, AllocatorReport, AllocatorStats};
use rhapsody::{Error, Result};
use std::collections::VecDeque;

#[test]
fn primitive_statistics_release_budget_and_failure_are_atomic() {
    let stats = AllocatorStats::default();
    stats.set_heap_budget(0, 100).unwrap();
    let a = stats
        .register(40, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, "a".into())
        .unwrap();
    let b = stats
        .register(50, 1, 0, vk::MemoryPropertyFlags::HOST_VISIBLE, "b".into())
        .unwrap();
    let report = stats.report();
    let peak = stats.peak_allocator_report().unwrap();
    assert!(
        stats
            .register(
                11,
                1,
                0,
                vk::MemoryPropertyFlags::HOST_VISIBLE,
                "fail".into()
            )
            .is_err()
    );
    assert!(stats.set_heap_budget(0, 80).is_err());
    assert_eq!(stats.report(), report);
    stats.release(a).unwrap();
    assert_eq!(stats.report().totals.total_allocated_bytes, 50);
    assert_eq!(stats.peak_totals().total_allocated_bytes, 90);
    assert_eq!(stats.peak_allocator_report().unwrap(), peak);
    assert!(stats.release(a).is_err());
    assert_eq!(stats.report().totals.total_allocated_bytes, 50);
    stats.release(b).unwrap();
    assert_eq!(stats.report().totals, AllocationTotals::default());
    assert_eq!(peak.allocations[0].name, "a");
    assert_eq!(peak.allocations[1].name, "b");
}
struct ExternalAllocator {
    reports: VecDeque<(AllocatorReport, bool)>,
    current: AllocatorReport,
    free_calls: usize,
}
impl AllocationBackend for ExternalAllocator {
    type Allocation = ();
    fn allocate(&mut self, _: &AllocationRequest) -> Result<()> {
        let (report, success) = self.reports.pop_front().unwrap();
        self.current = report;
        if success {
            Ok(())
        } else {
            Err(Error::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY))
        }
    }
    fn free(&mut self, _: ()) -> Result<()> {
        self.free_calls += 1;
        self.current.totals.total_allocated_bytes = 200;
        Ok(())
    }
    fn report(&self) -> AllocatorReport {
        self.current.clone()
    }
}
fn request() -> AllocationRequest {
    AllocationRequest {
        name: "probe".into(),
        requirements: vk::MemoryRequirements {
            size: 4,
            alignment: 4,
            memory_type_bits: 1,
        },
        location: MemoryLocation::GpuOnly,
        flags: vk::MemoryAllocateFlags::empty(),
    }
}
fn report(total: u64, device: u64, system: u64) -> AllocatorReport {
    AllocatorReport {
        totals: AllocationTotals {
            total_allocated_bytes: total,
            device_local_bytes: device,
            system_bytes: system,
        },
        allocations: vec![],
    }
}
#[test]
fn wrapper_samples_after_error_and_retains_strict_peak_snapshot() {
    let backend = ExternalAllocator {
        reports: VecDeque::from([
            (report(10, 6, 4), true),
            (report(10, 4, 6), true),
            (report(12, 7, 5), false),
        ]),
        current: AllocatorReport::default(),
        free_calls: 0,
    };
    let mut allocator = GpuAllocator::new(backend, true);
    allocator.allocate(&request()).unwrap();
    let snapshot = allocator.peak_allocator_report().unwrap().clone();
    allocator.allocate(&request()).unwrap();
    assert_eq!(allocator.peak_allocator_report().unwrap(), &snapshot);
    assert_eq!(allocator.peak_system_bytes(), 6);
    assert!(allocator.allocate(&request()).is_err());
    assert_eq!(allocator.peak_total_allocated_bytes(), 12);
    assert_eq!(allocator.peak_device_local_bytes(), 7);
    allocator.free(()).unwrap();
    assert_eq!(allocator.inner().free_calls, 1);
    assert_eq!(allocator.peak_total_allocated_bytes(), 12);
    assert_eq!(snapshot.totals.total_allocated_bytes, 10);
}
#[test]
fn wrapper_disabled_peak_and_counter_overflow_are_explicit() {
    let backend = ExternalAllocator {
        reports: VecDeque::from([(report(10, 6, 4), true)]),
        current: AllocatorReport::default(),
        free_calls: 0,
    };
    let mut allocator = GpuAllocator::new(backend, false);
    allocator.allocate(&request()).unwrap();
    assert_eq!(allocator.peak_total_allocated_bytes(), 0);
    assert!(allocator.peak_allocator_report().is_none());
    let stats = AllocatorStats::default();
    stats
        .register(
            u64::MAX,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            "large".into(),
        )
        .unwrap();
    let before = stats.report();
    assert!(
        stats
            .register(
                1,
                0,
                0,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                "overflow".into()
            )
            .is_err()
    );
    assert_eq!(stats.report(), before);
}

#[test]
fn lazy_peak_snapshot_matches_state_at_the_moment_of_peak() {
    use ash::vk;
    use rhapsody::accounting::AllocatorStats;
    let stats = AllocatorStats::default();
    let reg = |size| {
        stats
            .register(
                size,
                0,
                0,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                String::new(),
            )
            .unwrap()
    };
    let (a, b, c) = (reg(100), reg(200), reg(300));
    // 到达峰值(600)后尚未物化即释放：快照必须仍是三条记录。
    stats.release(b).unwrap();
    let peak = stats.peak_allocator_report().unwrap();
    assert_eq!(peak.totals.total_allocated_bytes, 600);
    assert_eq!(
        peak.allocations.iter().map(|r| r.id).collect::<Vec<_>>(),
        vec![a, b, c]
    );
    // 未超过峰值的登记不改写快照。
    let d = reg(50);
    assert_eq!(stats.peak_allocator_report().unwrap(), peak);
    // 超过峰值：快照更新为当刻状态（a, c, d, e），且不被后续释放污染。
    let e = reg(400);
    stats.release(a).unwrap();
    let new_peak = stats.peak_allocator_report().unwrap();
    assert_eq!(new_peak.totals.total_allocated_bytes, 850);
    assert_eq!(
        new_peak
            .allocations
            .iter()
            .map(|r| r.id)
            .collect::<Vec<_>>(),
        vec![a, c, d, e]
    );
    assert_eq!(stats.peak_totals().total_allocated_bytes, 850);
}
