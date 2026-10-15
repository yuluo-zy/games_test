//! 设备排序键的逐指令手解向量（0x1418b9130 内联比较器）。
//!
//! 手解结果（`.rne` 反汇编，2026-10 第三批）：
//! * 类别字节跳表 0x142d03954：0 Discrete→0x1418b917f、1 Integrated→0x1418b91e9、2 Virtual→0x1418b9237、
//!   3 Cpu→0x1418b9252、4 Other→0x1418b91cc。
//! * Discrete/Integrated 先对名字调 0x140bd7250（子串查找）找 "Direct3D12"(len 10) 再找
//!   "Microsoft Basic Render Driver"(len 0x1d)；命中任一→`-2000`（0xfffffffffffff830），都不命中→
//!   Discrete `-4000`（0xfffffffffffff060）/ Integrated `-3000`（0xfffffffffffff448）。
//! * Virtual 保持入口默认 `-2000`；Cpu `-1000`（0xfffffffffffffc18）；Other `0`（`xor esi,esi`）。
//! * 键 = base − (`[+0x88]` 本地堆大小 `shr 0x1e`)，有符号比较；`cmp rbx,r15; jge` 即严格 `<` 才前移。
use ash::vk;
use rhapsody::raw::init::{sort_devices_original, AvailableGraphicsDevice, DeviceMemoryMetrics};
use rhapsody::raw::physical_device::{
    DeviceDriverVersion, DriverVendor, GraphicsDeviceKind, HeapBudget, PerformanceInfo,
};

const GIB: u64 = 1 << 30;

fn device(index: usize, name: &str, kind: GraphicsDeviceKind, local: u64) -> AvailableGraphicsDevice {
    AvailableGraphicsDevice {
        index,
        handle: index as u64,
        name: name.into(),
        device_type: vk::PhysicalDeviceType::OTHER,
        kind,
        local_heap_bytes: local,
        host_heap_bytes: 0,
        device_uuid: [index as u8; 16],
        luid: None,
        driver: DeviceDriverVersion::from_parts(DriverVendor::Unknown, [0; 4]),
        memory: PerformanceInfo {
            snapshot: None,
            device_local_bytes: local,
            host_bytes: 0,
            device_local_budget: None,
            host_budget: None,
            unified_memory: false,
            heaps: Vec::<HeapBudget>::new(),
            vendor_specific: None,
        },
        original_memory_metrics: Some(DeviceMemoryMetrics { local, host: 0, shared: false }),
        api_version: vk::API_VERSION_1_3,
        timeline_semaphore: true,
        graphics_families: vec![0],
        present_families: None,
        extensions: vec![],
        errors: vec![],
    }
}

#[test]
fn full_kind_by_emulated_by_heap_matrix_matches_hand_decode() {
    use GraphicsDeviceKind::*;
    // (kind, 命中 emulated 名字后的 base, 不命中的 base)
    let kinds = [
        (Discrete, -2000i64, -4000i64),
        (Integrated, -2000, -3000),
        (Virtual, -2000, -2000),
        (Cpu, -1000, -1000),
        (Other, 0, 0),
    ];
    // Cpu / Other 即使名字命中也不变：它们的跳表目标不检查名字，故 emulated 列等于普通列。
    let names = [
        ("GeForce", false),
        ("NVIDIA Direct3D12 Wrapper", true),
        ("Microsoft Basic Render Driver", true),
        ("microsoft basic render driver", false), // 区分大小写
    ];
    for (kind, emu_base, plain_base) in kinds {
        for (name, emulated) in names {
            for heap in [0u64, GIB - 1, GIB, 3 * GIB + 5, 24 * GIB, 1 << 40] {
                let expected_base = match kind {
                    Discrete | Integrated if emulated => emu_base,
                    Virtual | Cpu | Other => plain_base,
                    _ => plain_base,
                };
                let key = device(0, name, kind, heap).device_order_key();
                assert_eq!(key, expected_base - (heap >> 30) as i64, "{kind:?} {name} {heap}");
            }
        }
    }
}

#[test]
fn bigger_heap_sorts_first_and_ties_keep_enumeration_order() {
    use GraphicsDeviceKind::*;
    let mut d = vec![
        device(0, "a", Discrete, 8 * GIB),
        device(1, "b", Discrete, 8 * GIB + 1), // 同为 8 GiB（shr 30 向下取整）→ tie
        device(2, "c", Discrete, 12 * GIB),
        device(3, "d", Integrated, 64 * GIB), // -3000-64 > -4000-12
    ];
    sort_devices_original(&mut d);
    let order: Vec<usize> = d.iter().map(|x| x.index).collect();
    assert_eq!(order, vec![2, 0, 1, 3]);
}
