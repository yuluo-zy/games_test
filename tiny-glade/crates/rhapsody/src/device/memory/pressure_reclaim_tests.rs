use super::*;
#[test]
fn one_allocation_shares_its_quota_and_snapshot_even_when_eviction_reports_zero() {
    let budget = crate::device::ReclamationBudget { max_scans: 3, max_destroys: 2 };
    let device = match Device::new_headless(&crate::device::DeviceConfig {
        reclamation: crate::device::ReclamationConfig { normal: budget, pressure: budget },
        ..crate::device::DeviceConfig::headless()
    }) {
        Ok(device) => device,
        Err(error) => {
            assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
            return;
        }
    };
    for _ in 0..9 {
        let owners = crate::collections::temp_list::TempList::new(1, device.allocator_stats().clone()).unwrap();
        assert!(device.lifetime.retirement.try_enqueue(crate::sync::SyncSet::EMPTY,
            crate::device::graveyard::Doomed::Owners(owners)).is_ok());
    }
    let mut reclaim = crate::device::allocator::AllocationReclaim::new(&device);
    for _ in 0..4 {
        assert_eq!(device.evict_idle_caches_reclaim(u64::MAX, &mut reclaim), 0);
    }
    assert!(reclaim.completed.as_ref().unwrap().is_some());
    assert_eq!(reclaim.remaining.max_destroys, 0);
    assert_eq!(device.retirement_stats().pending, 7, "one attempt never gets a fresh quota");
    while device.retirement_stats().pending > 0 {
        device.evict_idle_caches(u64::MAX);
    }
    assert_eq!(device.retirement_stats().idle_waits, 0);
}

#[test]
fn proactive_eviction_and_one_retry_share_owner_quota_then_new_attempt_recovers_credits() {
    // These are queued CPU owner payloads with real backing and reservation
    // credits. Allocation outcomes below are injected through the existing
    // retry closure; this does not count native vkAllocateMemory calls.
    for retry_succeeds in [false, true] {
        let budget = crate::device::ReclamationBudget { max_scans: 3, max_destroys: 2 };
        let device = match Device::new_headless(&crate::device::DeviceConfig {
            reclamation: crate::device::ReclamationConfig { normal: budget, pressure: budget },
            ..crate::device::DeviceConfig::headless()
        }) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                return;
            }
        };
        struct Owner;
        impl crate::tracking::GpuUse for Owner {
            fn visit_uses(&self, _: &mut crate::tracking::UseVisitor<'_>) -> crate::Result<()> { Ok(()) }
        }
        let initial = device.allocator_stats().cpu_memory_usage().total();
        let mut weak_owners = Vec::new();
        for _ in 0..6 {
            let owner = Arc::new(Owner);
            weak_owners.push(Arc::downgrade(&owner));
            let mut owners = crate::collections::temp_list::TempList::new(1, device.allocator_stats().clone()).unwrap();
            let owner: crate::tracking::ResourceOwner = owner;
            owners.add(owner).unwrap();
            let ticket = device.reserve_retirement().unwrap();
            device.lifetime.retirement.enqueue_reserved(crate::sync::SyncSet::EMPTY,
                crate::device::graveyard::Doomed::Owners(owners), ticket);
        }
        let queued_usage = device.allocator_stats().cpu_memory_usage().total();
        assert!(queued_usage > initial);
        let mut reclaim = crate::device::allocator::AllocationReclaim::new(&device);
        assert_eq!(device.evict_idle_caches_reclaim(u64::MAX, &mut reclaim), 0);
        assert_eq!(device.retirement_stats().pending, 4);
        assert_eq!(weak_owners.iter().filter(|owner| owner.strong_count() != 0).count(), 4);
        assert_eq!(reclaim.remaining.max_destroys, 0);
        let snapshot = reclaim.completed;
        assert!(snapshot.as_ref().unwrap().is_some());
        let remaining = reclaim.remaining;
        let after_proactive = device.allocator_stats().cpu_memory_usage().total();
        assert!(after_proactive < queued_usage && after_proactive > initial);
        let mut attempts = 0;
        let result = crate::device::allocator::with_eviction_retry(&device, 1, &mut reclaim, || {
            attempts += 1;
            if attempts == 1 || !retry_succeeds { Err(crate::Error::OutOfDeviceMemory) }
            else { Ok(17u32) }
        });
        assert_eq!(attempts, 2, "one first attempt plus at most one retry");
        if retry_succeeds { assert_eq!(result.unwrap(), 17); }
        else { assert!(matches!(result, Err(crate::Error::OutOfDeviceMemory))); }
        assert_eq!(reclaim.completed, snapshot, "the observed completion prefix is retained");
        assert_eq!(reclaim.remaining, remaining, "retry cannot replenish an exhausted destroy allowance");
        assert_eq!(device.retirement_stats().pending, 4);
        assert_eq!(device.allocator_stats().cpu_memory_usage().total(), after_proactive);
        // New logical attempts can make bounded progress after the previous
        // attempt has exhausted its allowance; they never drain all four.
        for expected_pending in [2, 0] {
            let mut next = crate::device::allocator::AllocationReclaim::new(&device);
            assert_eq!(device.evict_idle_caches_reclaim(u64::MAX, &mut next), 0);
            assert_eq!(device.retirement_stats().pending, expected_pending);
            assert_eq!(next.remaining.max_destroys, 0);
            assert_eq!(weak_owners.iter().filter(|owner| owner.strong_count() != 0).count(), expected_pending);
        }
        assert_eq!(device.allocator_stats().cpu_memory_usage().total(), initial);
        assert_eq!(device.retirement_stats().idle_waits, 0);
    }
}

#[test]
fn host_oom_and_device_lost_do_not_retry_or_replenish_a_sticky_failed_snapshot() {
    let device = match Device::new_headless(&crate::device::DeviceConfig::headless()) {
        Ok(device) => device,
        Err(error) => {
            assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
            return;
        }
    };
    let initial = device.allocator_stats().cpu_memory_usage().total();
    let mut owners = crate::collections::temp_list::TempList::new(1, device.allocator_stats().clone()).unwrap();
    owners.reserve(1).unwrap();
    let ticket = device.reserve_retirement().unwrap();
    device.lifetime.retirement.enqueue_reserved(crate::sync::SyncSet::EMPTY,
        crate::device::graveyard::Doomed::Owners(owners), ticket);
    let queued_usage = device.allocator_stats().cpu_memory_usage().total();
    let mut reclaim = crate::device::allocator::AllocationReclaim::new(&device);
    // Represent the existing failed-snapshot state directly. We observe
    // stickiness, not the number or behavior of actual device poll calls.
    reclaim.completed = Some(None);
    let remaining = reclaim.remaining;
    for device_lost in [false, true] {
        let mut attempts = 0;
        let result: crate::Result<()> = crate::device::allocator::with_eviction_retry(&device, 1, &mut reclaim, || {
            attempts += 1;
            if device_lost { Err(crate::Error::DeviceLost) } else { Err(crate::Error::OutOfHostMemory) }
        });
        assert_eq!(attempts, 1);
        if device_lost { assert!(matches!(result, Err(crate::Error::DeviceLost))); }
        else { assert!(matches!(result, Err(crate::Error::OutOfHostMemory))); }
        assert_eq!(device.evict_idle_caches_reclaim(u64::MAX, &mut reclaim), 0);
        assert_eq!(reclaim.completed, Some(None));
        assert_eq!(reclaim.remaining, remaining);
        assert_eq!(device.retirement_stats().pending, 1);
        assert_eq!(device.allocator_stats().cpu_memory_usage().total(), queued_usage);
    }
    // The injected closure error did not poison the real test device.
    assert!(!device.is_device_lost());
    assert_eq!(device.evict_idle_caches(u64::MAX), 0);
    assert_eq!(device.retirement_stats().pending, 0);
    assert_eq!(device.allocator_stats().cpu_memory_usage().total(), initial);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}
