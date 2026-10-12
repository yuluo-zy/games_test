//! 原RON资产→loader诊断→surface候选清单→选设备→实际GPU读回。
use ash::vk;
use rhapsody::raw::{
    init::{
        DeviceSelectionPolicy, GraphicsInstance, GraphicsInstanceLayerAllowlist,
        GraphicsInstanceLoader,
    },
    instance::InstanceBuilder,
    physical_device::DriverVendor,
};
use serde_json::{Value, json};
use std::sync::Arc;
// 原码对比输入复用同一候选对象；测试夹具不会进入运行时选择策略。
#[path = "../tests/device_policy.rs"]
#[allow(dead_code, unused_imports)]
mod fixture;
fn cpu_cases() -> Value {
    let mut cases = vec![];
    for (vendor, name, components) in [
        (DriverVendor::Nvidia, "RTX 5090", [580, 88, 0, 0]),
        (DriverVendor::Nvidia, "GTX 1080", [580, 88, 0, 0]),
        (DriverVendor::Nvidia, "NVIDIA", [511, 22, 0, 0]),
        (DriverVendor::Nvidia, "NVIDIA", [472, 12, 0, 0]),
        (
            DriverVendor::IntelWindows,
            "Intel(R) UHD Graphics 630",
            [101, 2135, 0, 0],
        ),
        (
            DriverVendor::IntelWindows,
            "Iris(R) Xe Graphics",
            [101, 5522, 0, 0],
        ),
        (DriverVendor::IntelUnix, "Intel Mesa", [24, 1, 4, 0]),
        (DriverVendor::IntelUnix, "Intel Mesa", [24, 1, 5, 0]),
        (DriverVendor::Amd, "AMD", [2, 0, 179, 0]),
        (DriverVendor::Qualcomm, "Qualcomm", [512, 798, 6, 0]),
        (DriverVendor::Apple, "Apple", [0, 2, 2108, 0]),
        (
            DriverVendor::Unknown,
            "Microsoft Basic Render Driver",
            [0; 4],
        ),
    ] {
        let mut d = fixture::candidate(0, name, vendor, components);
        for (local, host, shared) in [
            (1_000_000_000, 400_000_000, false),
            (999_999_999, 400_000_001, true),
        ] {
            d.original_memory_metrics = Some(rhapsody::raw::init::DeviceMemoryMetrics {
                local,
                host,
                shared,
            });
            cases.push(json!({"name":name,"vendor":vendor.original_tag(),"components":components,"local":local,"host":host,"shared":shared,"driver_tag":d.driver_support().original_tag(),"emulated":d.is_emulated(),"old_intel":d.is_old_intel(),"supported":d.is_supported()==rhapsody::raw::init::DeviceDecision::Yes,"compatible":d.is_compatible()==rhapsody::raw::init::DeviceDecision::Yes}));
        }
    }
    json!(cases)
}
#[cfg(target_os = "windows")]
mod window {
    use std::ffi::c_void;
    type Handle = *mut c_void;
    #[link(name = "user32")]
    unsafe extern "system" {
        fn CreateWindowExW(
            ex: u32,
            class: *const u16,
            name: *const u16,
            style: u32,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            parent: Handle,
            menu: Handle,
            instance: Handle,
            param: Handle,
        ) -> Handle;
        fn DestroyWindow(handle: Handle) -> i32;
        fn IsWindowVisible(handle: Handle) -> i32;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleW(name: *const u16) -> Handle;
    }
    pub struct Hidden {
        pub handle: Handle,
        pub instance: Handle,
    }
    impl Hidden {
        pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
            let class = "STATIC\0".encode_utf16().collect::<Vec<_>>();
            let name = "Rhapsody设备策略验收\0".encode_utf16().collect::<Vec<_>>();
            let instance = unsafe { GetModuleHandleW(std::ptr::null()) };
            // WS_POPUP不含VISIBLE；验证窗口不会显示或抢焦点。
            let handle = unsafe {
                CreateWindowExW(
                    0,
                    class.as_ptr(),
                    name.as_ptr(),
                    0x80000000,
                    0,
                    0,
                    320,
                    240,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    instance,
                    std::ptr::null_mut(),
                )
            };
            if handle.is_null() {
                return Err(std::io::Error::last_os_error().into());
            }
            assert_eq!(unsafe { IsWindowVisible(handle) }, 0);
            Ok(Self { handle, instance })
        }
    }
    impl Drop for Hidden {
        fn drop(&mut self) {
            unsafe { DestroyWindow(self.handle) };
        }
    }
}
fn inventory(list: &rhapsody::raw::init::GraphicsDevicesForSurface) -> Value {
    json!({"available":list.available,"missing":list.missing,"devices":list.devices.iter().map(|d|json!({"index":d.index,"name":d.name,"driver":d.driver.to_string(),"driver_tag":d.driver_support().original_tag(),"driver_encoding_known":d.driver.encoding_known,"supported":format!("{:?}",d.is_supported()),"compatible":format!("{:?}",d.is_compatible()),"old_intel":d.is_old_intel(),"graphics":d.graphics_families,"present":d.present_families,"memory":{"local_size":d.memory.device_local_bytes,"host_size":d.memory.host_bytes,"local_budget":d.memory.device_local_budget,"host_budget":d.memory.host_budget,"unified":d.memory.unified_memory}})).collect::<Vec<_>>()})
}
fn gpu_run() -> Result<Value, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(
        "D:/game/ljxsj_92385/Tiny Glade/assets/vulkan_allowlist_defaults.ron",
    )?;
    let mut allowlist = GraphicsInstanceLayerAllowlist::default();
    allowlist.update_from_ron(&input)?;
    let loader = GraphicsInstanceLoader::from_candidates(&[std::path::PathBuf::from(
        "D:/game/nonexistent-rhapsody-loader.dll",
    )])?;
    assert!(loader.attempts[0].error.is_some());
    let missing_layer = InstanceBuilder {
        layers: vec![std::ffi::CString::new("VK_LAYER_rhapsody_missing_fixture").unwrap()],
        ..Default::default()
    }
    .build()
    .is_err();
    let missing_extension = InstanceBuilder {
        extensions: vec![std::ffi::CString::new("VK_RHAPSODY_missing_fixture").unwrap()],
        ..Default::default()
    }
    .build()
    .is_err();
    let blocked_layer = InstanceBuilder {
        layers: vec![std::ffi::CString::new("VK_LAYER_USER_blocked").unwrap()],
        ..Default::default()
    }
    .build_with_allowlist(&allowlist)
    .is_err();
    assert!(missing_layer && missing_extension && blocked_layer);
    let loader_report = json!({"version":loader.api_version,"attempts":loader.attempts.iter().map(|a|json!({"candidate":a.candidate,"error":a.error})).collect::<Vec<_>>(),"layers":loader.layers.iter().map(|l|json!({"name":l.name,"vendor":l.vendor(),"allowed":allowlist.permits(l)})).collect::<Vec<_>>(),"implicit_layer_origins_known":loader.implicit_layers().is_some()});
    let mut builder = InstanceBuilder::default();
    #[cfg(target_os = "windows")]
    {
        builder.extensions = vec![
            ash::khr::surface::NAME.to_owned(),
            ash::khr::win32_surface::NAME.to_owned(),
        ];
    }
    let graphics = GraphicsInstance::new_with_allowlist(&builder, &allowlist)?;
    let headless = graphics.available_for_surface(None, DeviceSelectionPolicy::default())?;
    #[cfg(target_os = "windows")]
    let window = window::Hidden::new()?;
    #[cfg(target_os = "windows")]
    let surface = unsafe {
        rhapsody::raw::swapchain::Surface::from_win32(
            graphics.instance.clone(),
            window.instance as isize,
            window.handle as isize,
        )
    }?;
    #[cfg(target_os = "windows")]
    let list =
        graphics.available_for_surface(Some(surface.handle), DeviceSelectionPolicy::default())?;
    #[cfg(not(target_os = "windows"))]
    let list = graphics.available_for_surface(None, DeviceSelectionPolicy::default())?;
    let device = list
        .create_device(&graphics)
        .map_err(|e| std::io::Error::other(e.join("\n")))?;
    #[cfg(target_os = "windows")]
    let surface_report = {
        let capabilities = unsafe {
            surface
                .loader
                .get_physical_device_surface_capabilities(device.physical, surface.handle)
        }?;
        let formats = unsafe {
            surface
                .loader
                .get_physical_device_surface_formats(device.physical, surface.handle)
        }?;
        let modes = unsafe {
            surface
                .loader
                .get_physical_device_surface_present_modes(device.physical, surface.handle)
        }?;
        assert!(!formats.is_empty() && !modes.is_empty());
        json!({"real_surface":true,"format_count":formats.len(),"present_mode_count":modes.len(),"min_image_count":capabilities.min_image_count,"selected_family":device.queue_family})
    };
    #[cfg(not(target_os = "windows"))]
    let surface_report = json!({"real_surface":false});
    let buffer = Arc::new(rhapsody::raw::buffer::Buffer::new(
        device.clone(),
        &rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu(64, vk::BufferUsageFlags::TRANSFER_DST),
    )?);
    let pool = rhapsody::raw::command_buffer::VulkanCommandPool::new(device.clone())?;
    let mut command = pool.allocate()?;
    command.begin()?;
    command.keep_alive(buffer.clone());
    unsafe {
        device
            .device
            .cmd_fill_buffer(command.handle, buffer.handle, 0, 64, 0x12345678);
    }
    command.end()?;
    let pending = command.submit()?;
    pending.wait(10_000_000_000)?;
    let mut bytes = [0; 64];
    buffer.read_bytes(0, &mut bytes)?;
    assert!(
        bytes
            .chunks_exact(4)
            .all(|b| b == 0x12345678u32.to_le_bytes())
    );
    let mut refreshed = headless.clone();
    refreshed.refresh_memory_budgets(&graphics)?;
    Ok(
        json!({"status":"passed","original_ron_fork_used":true,"allowlist_version":allowlist.version,"allowlist":allowlist.entries,"loader":loader_report,"missing_layer_rejected":missing_layer,"missing_extension_rejected":missing_extension,"blocked_layer_rejected":blocked_layer,"headless":inventory(&headless),"surface_candidates":inventory(&list),"budget_refresh":inventory(&refreshed),"surface":surface_report,"gpu_readback":true,"bytes":64,"pattern":"12345678","bda_enabled":device.capabilities.buffer_device_address,"runtime_array_enabled":device.capabilities.runtime_descriptor_array,"auxiliary_count":device.additional_queues.len(),"limits":["原sort comparator及implicit层来源/控制策略未知","无budget或驱动编码未确认时保持Unknown，候选fallback不宣称原支持","vendor专有性能/NVAPI不重写"]}),
    )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a == "--cpu") {
        println!("{}", cpu_cases());
    } else {
        println!("{}", gpu_run()?);
    }
    Ok(())
}
