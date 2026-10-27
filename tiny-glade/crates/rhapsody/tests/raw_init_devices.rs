//! raw-init 域：设备排序 / 内存门槛 / missing_devices / 预算快照 / 厂商句柄匹配。
mod gpu_test_support;
use ash::vk;
use rhapsody::device::selection::{AvailableGraphicsDevice, DeviceDecision, DeviceMemoryMetrics, DxgiAdapter, GraphicsInstance, MissingGraphicsDevice, MissingVendor, meets_original_memory_threshold, missing_devices, order_and_filter_devices_original, sort_devices_original};
use rhapsody::native::physical_device::{DeviceDriverVersion, DriverVendor, GraphicsDeviceKind, HeapBudget, MAX_MEMORY_HEAPS, MemoryBudgetSnapshot, PerformanceInfo, VendorDeviceEntry, VendorPerformanceQueries, format_vendor_performance_info, match_vendor_device_handle};

const GIB: u64 = 1 << 30;

fn device(
    index: usize,
    name: &str,
    kind: GraphicsDeviceKind,
    local: u64,
    host: u64,
) -> AvailableGraphicsDevice {
    AvailableGraphicsDevice {
        index,
        handle: index as u64,
        name: name.into(),
        device_type: vk::PhysicalDeviceType::OTHER,
        kind,
        local_heap_bytes: local,
        host_heap_bytes: host,
        device_uuid: [index as u8; 16],
        luid: None,
        driver: DeviceDriverVersion::from_parts(DriverVendor::Unknown, [0; 4]),
        memory: PerformanceInfo {
            snapshot: None,
            device_local_bytes: local,
            host_bytes: host,
            device_local_budget: None,
            host_budget: None,
            unified_memory: false,
            heaps: Vec::<HeapBudget>::new(),
            vendor_specific: None,
        },
        original_memory_metrics: Some(DeviceMemoryMetrics {
            local,
            host,
            shared: kind != GraphicsDeviceKind::Discrete,
        }),
        api_version: vk::API_VERSION_1_3,
        timeline_semaphore: true,
        graphics_families: vec![0],
        present_families: None,
        extensions: vec![],
        errors: vec![],
    }
}

#[test]
fn sort_is_ascending_by_key() {
    use GraphicsDeviceKind::*;
    let mut devices = vec![
        device(0, "other", Other, 8 * GIB, 0),
        device(1, "cpu", Cpu, 8 * GIB, 0),
        device(2, "emulated dgpu", Discrete, 16 * GIB, 0),
        device(3, "virtual", Virtual, 4 * GIB, 0),
        device(4, "igpu", Integrated, 2 * GIB, 0),
        device(5, "dgpu", Discrete, 8 * GIB, 0),
    ];
    // emulated 名字
    devices[2].name = "Direct3D12 dgpu".into();
    sort_devices_original(&mut devices);
    let order: Vec<usize> = devices.iter().map(|d| d.index).collect();
    // keys: dgpu -4008, igpu -3002, emulated -2016, virtual -2004, cpu -1008, other -8
    assert_eq!(order, vec![5, 4, 2, 3, 1, 0]);
}

#[test]
fn equal_keys_keep_enumeration_order_for_both_sort_paths() {
    use GraphicsDeviceKind::*;
    // 原版 <0x15 个用插入排序，≥0x15 个用 driftsort；两者都稳定，所以 tie 保持原序。
    for count in [2usize, 20, 21, 64, 65, 200] {
        let mut devices: Vec<_> = (0..count)
            .map(|i| {
                // 三组 key，组内全相同；index 递增。
                let (kind, local) = match i % 3 {
                    0 => (Discrete, 4 * GIB),
                    1 => (Integrated, 4 * GIB),
                    _ => (Cpu, 4 * GIB),
                };
                device(i, "gpu", kind, local, 0)
            })
            .collect();
        sort_devices_original(&mut devices);
        for pair in devices.windows(2) {
            let (a, b) = (&pair[0], &pair[1]);
            assert!(a.device_order_key() <= b.device_order_key());
            if a.device_order_key() == b.device_order_key() {
                assert!(a.index < b.index, "count={count}: tie 必须保持枚举顺序");
            }
        }
    }
}

#[test]
fn memory_threshold_edges() {
    use GraphicsDeviceKind::*;
    // Discrete（类别字节 0）：local>=1e9 且 host>=4e8。
    assert!(meets_original_memory_threshold(
        Discrete,
        1_000_000_000,
        400_000_000
    ));
    assert!(!meets_original_memory_threshold(
        Discrete,
        999_999_999,
        400_000_000
    ));
    assert!(!meets_original_memory_threshold(
        Discrete,
        1_000_000_000,
        399_999_999
    ));
    // 其余类别：local+host>=1.4e9，host 不单独要求。
    for kind in [Integrated, Virtual, Cpu, Other] {
        assert!(meets_original_memory_threshold(kind, 1_400_000_000, 0));
        assert!(!meets_original_memory_threshold(kind, 1_399_999_999, 0));
        assert!(meets_original_memory_threshold(
            kind,
            1_000_000_000,
            400_000_000
        ));
    }
}

#[test]
fn original_filter_drops_by_memory_only_and_records_insufficient_memory() {
    use GraphicsDeviceKind::*;
    let mut devices = vec![
        device(0, "small igpu", Integrated, 512 << 20, 512 << 20),
        device(1, "big dgpu", Discrete, 8 * GIB, 16 * GIB),
        // 驱动未知/emulated 也不会被这个 retain 筛掉（原谓词只看堆大小）。
        device(2, "Direct3D12 mid", Discrete, 2 * GIB, 2 * GIB),
    ];
    let mut missing = vec![MissingGraphicsDevice::NotInVulkan {
        name: "ghost".into(),
        vendor: MissingVendor::Nvidia,
    }];
    order_and_filter_devices_original(&mut devices, &mut missing);
    assert_eq!(
        devices.iter().map(|d| d.index).collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(missing.len(), 2);
    match &missing[1] {
        MissingGraphicsDevice::InsufficientMemory(d) => assert_eq!(d.index, 0),
        other => panic!("应为 tag 3: {other:?}"),
    }
}

#[test]
fn missing_devices_matches_by_luid_or_name_and_only_for_amd_nvidia() {
    use GraphicsDeviceKind::*;
    let mut a = device(0, "NVIDIA GeForce RTX 4090", Discrete, 24 * GIB, 24 * GIB);
    a.luid = Some(0xAAAA);
    let b = device(1, "AMD Radeon RX 7900", Discrete, 24 * GIB, 24 * GIB);
    let adapters = vec![
        // LUID 命中
        DxgiAdapter {
            name: "renamed".into(),
            luid: 0xAAAA,
            vendor_id: 0x10de,
            device_id: 1,
        },
        // 名字相同命中（LUID 不同）
        DxgiAdapter {
            name: "AMD Radeon RX 7900".into(),
            luid: 0xBBBB,
            vendor_id: 0x1002,
            device_id: 2,
        },
        // 缺失的 NVIDIA / AMD
        DxgiAdapter {
            name: "Quadro".into(),
            luid: 0xCCCC,
            vendor_id: 0x10de,
            device_id: 3,
        },
        DxgiAdapter {
            name: "Radeon Pro".into(),
            luid: 0xDDDD,
            vendor_id: 0x1002,
            device_id: 4,
        },
        // 其它厂商直接丢弃，即使 Vulkan 里没有
        DxgiAdapter {
            name: "Intel Arc".into(),
            luid: 0xEEEE,
            vendor_id: 0x8086,
            device_id: 5,
        },
    ];
    let missing = missing_devices(&[a, b], &adapters);
    assert_eq!(missing.len(), 2);
    assert!(
        matches!(&missing[0], MissingGraphicsDevice::NotInVulkan { name, vendor: MissingVendor::Nvidia } if name == "Quadro")
    );
    assert!(
        matches!(&missing[1], MissingGraphicsDevice::NotInVulkan { name, vendor: MissingVendor::Amd } if name == "Radeon Pro")
    );
    // luid 为 None 时第一个条件（tag==1）不成立，不会拿 0 去匹配。
    let c = device(2, "x", Discrete, 24 * GIB, 24 * GIB);
    let zero = vec![DxgiAdapter {
        name: "other".into(),
        luid: 0,
        vendor_id: 0x10de,
        device_id: 0,
    }];
    assert_eq!(missing_devices(&[c], &zero).len(), 1);
}

#[test]
fn compatibility_uses_heap_sizes_and_the_kind_byte() {
    use GraphicsDeviceKind::*;
    let mut d = device(0, "gpu", Discrete, 1_000_000_000, 400_000_000);
    d.driver = DeviceDriverVersion::from_parts(DriverVendor::Nvidia, [600, 0, 0, 0]);
    assert_eq!(d.is_compatible(), DeviceDecision::Yes);
    d.kind = Integrated;
    d.original_memory_metrics = Some(DeviceMemoryMetrics {
        local: 1_000_000_000,
        host: 399_999_999,
        shared: true,
    });
    assert!(matches!(d.is_compatible(), DeviceDecision::No(_)));
}

#[test]
fn budget_snapshot_clamp_and_sums() {
    let mut old = MemoryBudgetSnapshot {
        heap_budget: [0; 16],
        heap_usage: [0; 16],
    };
    let mut new = old;
    for i in 0..MAX_MEMORY_HEAPS {
        old.heap_budget[i] = 800 + i as u64;
        new.heap_budget[i] = 10_000; // 超过上沿
        new.heap_usage[i] = 7 + i as u64;
    }
    old.heap_budget[3] = 5; // 小预算：delta 取 1
    new.heap_budget[3] = 100;
    old.heap_budget[4] = 1000;
    new.heap_budget[4] = 0; // 低于下沿
    let clamped = new.clamped_against(&old);
    assert_eq!(clamped.heap_budget[0], 900); // 800 + 100
    assert_eq!(clamped.heap_budget[3], 6);
    assert_eq!(clamped.heap_budget[4], 875); // 1000 - 125
    // usage 不钳位，原样取驱动值。
    assert_eq!(clamped.heap_usage, new.heap_usage);
    // 与 vk 结构双向无损。
    let vk = clamped.to_vk();
    assert_eq!(MemoryBudgetSnapshot::from_vk(&vk), clamped);

    let heaps = [
        vk::MemoryHeap {
            size: 1,
            flags: vk::MemoryHeapFlags::DEVICE_LOCAL,
        },
        vk::MemoryHeap {
            size: 1,
            flags: vk::MemoryHeapFlags::empty(),
        },
        vk::MemoryHeap {
            size: 1,
            flags: vk::MemoryHeapFlags::DEVICE_LOCAL | vk::MemoryHeapFlags::MULTI_INSTANCE,
        },
    ];
    let snap = MemoryBudgetSnapshot {
        heap_budget: {
            let mut b = [0; 16];
            b[0] = 10;
            b[1] = 20;
            b[2] = 30;
            b[3] = 999;
            b
        },
        heap_usage: [0; 16],
    };
    assert_eq!(snap.sum_budget(&heaps, true), 40);
    assert_eq!(snap.sum_budget(&heaps, false), 20);
}

#[test]
fn vendor_handle_match_is_nvidia_only_and_skips_entries_without_pci_id() {
    let entries = [
        VendorDeviceEntry {
            handle: 1,
            pci_device_id: None,
        },
        VendorDeviceEntry {
            handle: 2,
            pci_device_id: Some(0x2684),
        },
        VendorDeviceEntry {
            handle: 3,
            pci_device_id: Some(0x2504),
        },
    ];
    assert_eq!(
        match_vendor_device_handle(0x10de, 0x2504, &entries),
        Some(3)
    );
    assert_eq!(match_vendor_device_handle(0x10de, 0x9999, &entries), None);
    // 非 NVIDIA 一律不查表。
    assert_eq!(match_vendor_device_handle(0x1002, 0x2504, &entries), None);
    assert_eq!(match_vendor_device_handle(0x8086, 0x2504, &entries), None);
}

#[test]
fn performance_info_text_follows_the_original_sections() {
    assert_eq!(format_vendor_performance_info(None), "Unsupported");
    let q = VendorPerformanceQueries {
        clocks: Some("c".into()),
        p_state: None, // 查询返回非 0 时整段省略
        utilization: Some("u".into()),
        power: None,
        memory: Some("m".into()),
        sensors: Some("s".into()),
    };
    assert_eq!(
        format_vendor_performance_info(Some(&q)),
        "Clocks: c\nUtilization: u\nMemory: m\nSensors: s"
    );
    assert_eq!(
        format_vendor_performance_info(Some(&Default::default())),
        ""
    );
}

#[test]
fn create_by_uuid_selects_that_device_and_returns_unsupported_when_absent() {
    let Some(graphics) = gpu_test_support::initialize("UUID instance", GraphicsInstance::new())
    else {
        return;
    };
    let Some(surface) = gpu_test_support::initialize(
        "UUID physical device selection",
        graphics.available_for_surface(None, Default::default()),
    ) else {
        return;
    };
    let first = surface
        .available_devices()
        .into_iter()
        .next()
        .expect("an available GPU is required");
    // 物化 Vec：克隆而非借用。
    assert_eq!(surface.available_devices().len(), surface.available.len());
    let device = rhapsody::device::Device::new(&graphics.instance, None, &rhapsody::device::DeviceConfig {
            preferred_device: rhapsody::device::DevicePreference::ByUuid(first.device_uuid),
            ..rhapsody::device::DeviceConfig::headless()
        })
        .expect("按 uuid 选中的设备应能创建");
    assert_eq!(device.device_uuid(), first.device_uuid);
    drop(device);
    let missing_uuid = [0xEE; 16];
    if surface
        .available_devices()
        .iter()
        .all(|d| d.device_uuid != missing_uuid)
    {
        let error = rhapsody::device::Device::new(&graphics.instance, None, &rhapsody::device::DeviceConfig {
                preferred_device: rhapsody::device::DevicePreference::ByUuid(missing_uuid),
                ..rhapsody::device::DeviceConfig::headless()
            })
            .err()
            .expect("missing UUID must be rejected");
        assert_eq!(error.kind(), rhapsody::ErrorKind::Unsupported);
        // Rejection does not mutate the candidate list or poison later creation.
        let retried = rhapsody::device::Device::new(&graphics.instance, None, &rhapsody::device::DeviceConfig {
                preferred_device: rhapsody::device::DevicePreference::ByUuid(first.device_uuid),
                ..rhapsody::device::DeviceConfig::headless()
            })
            .unwrap();
        assert_eq!(retried.device_uuid(), first.device_uuid);
    }
}
