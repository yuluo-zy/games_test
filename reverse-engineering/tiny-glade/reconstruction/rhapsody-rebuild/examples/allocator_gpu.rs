//! 原游戏分配包装与专用 Vulkan 适配的功能验收；不替代缺失的原 gpu-allocator 分叉池。
use ash::vk;
use rhapsody::raw::{
    allocator::{
        AllocationBackend, AllocationRecord, AllocationRequest, AllocationTotals, AllocatorReport,
        DirectMemoryBackend, GpuAllocator, MemoryLocation,
    },
    buffer::{Buffer, BufferDesc},
    error::{Error, Result},
    init::create_headless_device,
};
use std::collections::VecDeque;
struct ReportFixture {
    reports: VecDeque<AllocatorReport>,
    current: AllocatorReport,
}
impl AllocationBackend for ReportFixture {
    type Allocation = ();
    fn allocate(&mut self, _: &AllocationRequest) -> Result<()> {
        self.current = self.reports.pop_front().unwrap();
        Err(Error::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY))
    }
    fn free(&mut self, _: ()) -> Result<()> {
        Ok(())
    }
    fn report(&self) -> AllocatorReport {
        self.current.clone()
    }
}
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("--rules") {
        let totals = [
            (0, 0, 0),
            (10, 6, 4),
            (10, 4, 6),
            (8, 2, 6),
            (12, 7, 5),
            (9, 8, 1),
            (u64::MAX, u64::MAX, 0),
            (u64::MAX, 0, u64::MAX),
        ];
        let mut cases = Vec::new();
        for tracking in [false, true] {
            let reports = totals
                .iter()
                .enumerate()
                .map(|(marker, (total, device, system))| AllocatorReport {
                    totals: AllocationTotals {
                        total_allocated_bytes: *total,
                        device_local_bytes: *device,
                        system_bytes: *system,
                    },
                    allocations: vec![AllocationRecord {
                        id: marker as u64,
                        size: 0,
                        memory_type: 0,
                        heap: 0,
                        properties: 0,
                        name: "边界身份标记".into(),
                    }],
                })
                .collect();
            let mut allocator = GpuAllocator::new(
                ReportFixture {
                    reports,
                    current: AllocatorReport::default(),
                },
                tracking,
            );
            for (marker, (total, device, system)) in totals.iter().copied().enumerate() {
                let request = AllocationRequest {
                    name: "fixture".into(),
                    requirements: vk::MemoryRequirements::default(),
                    location: MemoryLocation::Unknown,
                    flags: vk::MemoryAllocateFlags::empty(),
                };
                assert!(allocator.allocate(&request).is_err());
                cases.push(serde_json::json!({"tracking":tracking,"marker":marker,"report":[total,device,system],"peaks":[allocator.peak_total_allocated_bytes(),allocator.peak_device_local_bytes(),allocator.peak_system_bytes()],"peak_snapshot_marker":allocator.peak_allocator_report().map(|report|report.allocations[0].id)}));
            }
        }
        println!("{}", serde_json::json!({"cases":cases}));
        return Ok(());
    }
    let device = create_headless_device()?;
    let baseline = device.allocator_stats.report();
    assert_eq!(baseline.totals.total_allocated_bytes, 0);
    let a = Buffer::new(
        device.clone(),
        &BufferDesc::new_cpu_to_gpu(4096, vk::BufferUsageFlags::STORAGE_BUFFER),
    )?;
    a.write_bytes(0, &0xabcdefu32.to_le_bytes())?;
    let b = Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_only(8192, vk::BufferUsageFlags::STORAGE_BUFFER),
    )?;
    let two = device.allocator_stats.report();
    assert_eq!(two.allocations.len(), 2);
    assert!(two.totals.total_allocated_bytes >= 12288);
    let saved = device.allocator_stats.peak_allocator_report().unwrap();
    drop(a);
    assert_eq!(device.allocator_stats.report().allocations.len(), 1);
    assert_eq!(
        device.allocator_stats.peak_allocator_report().unwrap(),
        saved
    );
    let mut allocator = GpuAllocator::new(
        DirectMemoryBackend {
            device: device.clone(),
        },
        true,
    );
    let bits = if device.memory.memory_type_count == 32 {
        u32::MAX
    } else {
        (1 << device.memory.memory_type_count) - 1
    };
    let request = AllocationRequest {
        name: "游戏包装宿主分配".into(),
        requirements: vk::MemoryRequirements {
            size: 512,
            alignment: 4,
            memory_type_bits: bits,
        },
        location: MemoryLocation::CpuToGpu,
        flags: vk::MemoryAllocateFlags::empty(),
    };
    let allocation = allocator.allocate(&request)?;
    let current = allocator.inner().report();
    let wrapper_snapshot = allocator.peak_allocator_report().unwrap().clone();
    let wrapper_peak = allocator.peak_total_allocated_bytes();
    assert_eq!(current.totals.total_allocated_bytes, wrapper_peak);
    let selected = rhapsody::raw::allocator::choose_memory_type(
        &device.memory,
        bits,
        vk::MemoryPropertyFlags::HOST_VISIBLE,
        vk::MemoryPropertyFlags::HOST_COHERENT,
    )
    .unwrap();
    let heap = device.memory.memory_types[selected as usize].heap_index;
    let used = current
        .allocations
        .iter()
        .filter(|allocation| allocation.heap == heap)
        .map(|allocation| allocation.size)
        .sum();
    device.allocator_stats.set_heap_budget(heap, used)?;
    assert!(allocator.allocate(&request).is_err());
    assert_eq!(allocator.inner().report(), current);
    assert_eq!(
        allocator.peak_allocator_report().unwrap(),
        &wrapper_snapshot
    );
    drop(b);
    allocator.free(allocation)?;
    assert_eq!(
        device.allocator_stats.report().totals.total_allocated_bytes,
        0
    );
    assert_eq!(allocator.peak_total_allocated_bytes(), wrapper_peak);
    assert_eq!(wrapper_snapshot.allocations.len(), 2);
    assert_eq!(
        device.allocator_stats.peak_allocator_report().unwrap(),
        saved
    );
    let c = Buffer::new(
        device.clone(),
        &BufferDesc::new_cpu_to_gpu(256, vk::BufferUsageFlags::STORAGE_BUFFER),
    )?;
    drop(c);
    assert_eq!(
        device.allocator_stats.peak_allocator_report().unwrap(),
        saved
    );
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name =
        unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
    println!(
        "{}",
        serde_json::json!({"status":"candidate_allocator_gpu_passed","device":name,"live_final":0,"device_peak_bytes":saved.totals.total_allocated_bytes,"wrapper_peak_bytes":wrapper_peak,"resource_allocations":3,"wrapper_allocations":1,"budget_failure_no_pollution":true,"peak_snapshot_immutable":true,"original_gpu_allocator_pool_equivalence":false})
    );
    Ok(())
}
