# N3 取证：raw::init / raw::instance / raw::physical_device

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra 12.1.4，`run_auto_analysis:false`，`--lanes 2`（**两条 lane 伪代码逐字节一致 = 独立复现**）。
- 产物：`job.json`/`job2.json`、`REPORT.json`/`REPORT2.json`、**39 份** `0x*.c`。
- 仅取证，未修改任何源码。

## 0. 工具限制（实测）

`disassemble_function` 返回 0 指令；`find_code_by_string` 失败；`get_xrefs_to` 只有 DATA 引用（`0x1436A*`），**无 CALL 边 ⇒ 不能据 xref 断 caller**。扩展名改由**对 exe 直接字节取证**（VA = 文件偏移 + `0x140000000`，已用 "Microsoft Basic Render Driver" 锚点校验）。

## 1. 实例扩展 / 图层（观察）

**字符串表**（`instance.rs` 段，偏移 45184632）：
`VK_KHR_get_physical_device_properties2`、`VK_KHR_get_surface_capabilities2`、`VK_EXT_debug_utils`、`VK_LAYER_KHRONOS_validation`、`VK_EXT_layer_settings`（+ supported/unsupported 分支）；surface 另用 `VK_KHR_surface`。
流程串：`Creating a Vulkan instance` / `Enabling Vulkan validation layers` / `create_debug_utils_messenger failed` / `Force-disabling all implicit Vulkan layers`。

**反编译**：`GraphicsInstance::new`(`0x140bd7dd0`) 装配层名列表后调 `InstanceBuilder::build`(`0x140c08f10`)；`build` 经实例 vtable 间接调 `vkCreateInstance`（`createInfo.sType = 1`），validation 启用时条件调 `vkCreateDebugUtilsMessengerEXT`，回调 = `FUN_140c0cd40`(`vulkan_debug_messenger`)。

`apply_vulkan_blocklist`(`0x140c0a530`) 屏蔽隐式层：`reshade`/`mangohud`/`twitch`/`optimus`/`amd switchable`/`vkbasalt`，用 `VK_LOADER_LAYERS_DISABLE`/`ALLOW`。

## 2. apiVersion / 常量（观察 + 推断）

- `InstanceBuilder::build` 见常量 **`0x00402000` = `VK_API_VERSION_1_2`**（`uStack_2ec`，line 483）。**推断 `apiVersion = 1.2`**（唯一对齐常量；无 `0x401000`/`0x403000`）。
- `VkInstanceCreateInfo.sType = 1`（观察）。
- `applicationName` / `engineName` 字符串**未唯一恢复 = 未知**。

## 3. 物理设备选择（观察）

- `GraphicsDevicesForSurface::new`(`0x140340bd0`) → `enumerate_physical_devices`(`0x140c01e00`) + `with_presentation_support`(`0x140bdcf90`)；`order_and_filter_devices`(`0x140bdbc30`) 对 `AvailableGraphicsDevice`（**stride 0xa0 = 160 B**）排序（≤0x15 用插入排序 / `FUN_1418b9130`，否则 `FUN_1418bb250`）+ `retain`。
- `is_supported`(`0x140bda840`) / `is_compatible`(`0x140bda8f0`)：要求 `driver_support` 返回码 **== 5**；名**不含** `Direct3D12`(10B) / `Microsoft Basic Render Driver`(29B)；阈值：标志 `+0x99 == 0` 时需 `deviceLocalMem ≥ 1_000_000_000` **且**另一堆 `≥ 400_000_000`，否则 `deviceLocalMem + host ≥ 1_400_000_000`；`is_supported` 另拒 `is_old_intel`。
- `driver_support`(`0x140bda990`)：按 `vendorId`(`+0x58`) 跳表，**支持 = 5、默认 = 2**。
- `is_emulated`(`0x140bdaec0`)：名含 `Direct3D12` 即模拟。
- `is_old_intel`(`0x140bdaf20`)：`+0x58 == 1`(Intel) 且名匹配 `HD Graphics`/`Iris(R)`/`Xe`/`11thGen`/`12thGen`/`13thGen`/`Graphics 7`/`Graphics P7`。

**device extensions 串表**：`VK_KHR_swapchain`、`VK_KHR_maintenance1/2/3`、`VK_EXT_descriptor_indexing`、`VK_KHR_image_format_list`、`VK_EXT_shader_demote_to_helper_invocation`、`VK_KHR_buffer_device_address`、`VK_EXT_full_screen_exclusive`、`VK_EXT_memory_budget`、`VK_EXT_subgroup_size_control`、`VK_KHR_external_memory_win32`、`VK_NV_device_diagnostic_checkpoints`。

**启用 features（Debug 串实测）**：
- `VkPhysicalDeviceFeatures`：`robust_buffer_access`、`draw_indirect_first_instance`、`fill_mode_non_solid`、`sampler_anisotropy`、`texture_compression_bc`、`shader_storage_image_extended_formats`、`shader_storage_image_write_without_format`、`dual_src_blend`、`multi_draw_indirect`、`fragment_stores_and_atomics`、`vertex_pipeline_stores_and_atomics`；
- `VkPhysicalDeviceVulkan12Features`：`timeline_semaphore`、`descriptor_indexing`、`descriptor_binding_partially_bound`、`descriptor_binding_sampled_image_update_after_bind`、`shader_sampled_image_array_non_uniform_indexing`、`descriptor_binding_update_unused_while_pending`、`descriptor_binding_variable_descriptor_count`、`runtime_descriptor_array`、`buffer_device_address`；
- `ShaderDemoteToHelperInvocationFeatures`：`shader_demote_to_helper_invocation`。

`create` 中的 sType：`1000059000`(FEATURES_2)、`1000059001`(PROPERTIES_2)、`1000225000`(SUBGROUP_SIZE_CONTROL_PROPERTIES)、`1000276000`、`1000300001`（**后两个未匹配名**）。

## 4. 调试 / validation（观察）

`vulkan_debug_messenger`(`0x140c0cd40`) = `VkDebugUtilsMessengerEXT` 回调，**仅在 validation 启用时安装**；层 `VK_LAYER_KHRONOS_validation` + 扩展 `VK_EXT_debug_utils`（条件）。

## 5. 设备创建的队列族（补 `0x140bf2560`，**独立复现 A7**）

- 队列族 **stride 0x1c**（line 465/533）。
- **必建**首份 `VkDeviceQueueCreateInfo`：`queueFlags & 1`(GRAPHICS) 族，`sType = 2`、`queueCount = 1`、`priority = 0x3f000000`(0.5f)。
- **条件建**第二份：仅当存在 `(queueFlags & 3) == 2` 专用 compute 族，`queueCount` = 该族值。
- `create_device`(`0x140bdb940`) 按句柄（`+0xd2`/`+0xd3`）匹配 + `Arc` 计数后调 `0x140bf2560`。

## 6. `raw::physical_device` 20 个函数映射（观察）

`enumerate_physical_devices 0x140c01e00`；`enumerate_vk_data 0x1418cd4d0`（两次调用助手）；`with_presentation_support 0x140bdcf90`（队列族 `0x690` stride 谓词过滤）；`find_vendor_specific_device_handle 0x140c01e80`（NVIDIA `vendorId 0x10DE` 按 `deviceId` 匹配 NVAPI）；`PhysicalDevice::name 0x140c00400`（属性 `+0xdc`）；`device_local_memory_bytes 0x140c00530`（累加 DEVICE_LOCAL 堆）；`device_local_memory_budget_bytes 0x140c006a0`；`host_memory_budget_bytes 0x140c00890`（用 `VK_EXT_memory_budget`）；`performance_info 0x140c00a80`（NVAPI/ADL 的 P-state/利用率/功耗，否则返回 `"Unsupported"`）；`refresh_memory_budget 0x140c01620`；`DriverVersion::vendor_string`/`is_amd`/`is_apple` `0x140c001c0`/`e0`/`f0`；其余为 `fmt`。

**内存属性缓存**：`+0x504` heapCount(≤16)、`+0x508` 堆数组（stride 0x10）、`+0x608` budget；**厂商字段 `+0x58`**。

## 7. 光追独立复核（**第三次独立确认，无反例**）

三重证据：
1. `VK_KHR_acceleration_structure` / `ray_query` / `ray_tracing_pipeline` / `VK_NV_ray_tracing` / `VK_KHR_ray` **全 0**；
2. `vkCreateAccelerationStructureKHR` / `vkCmdTraceRaysKHR` 等函数名**全 0**；符号层无 AS/RT；
3. 本节第 3 条的 **device extensions 表与 enabled features 表中均无任何光追项**（这是最直接的证据面）。

`supports_ray_tracing`(3)、`AccelerationStructure`(8) 均属**能力字段名 / vk-sync access-flag 枚举串**，非用法。
软件路径：`obvhs` 出现 **14 次**且带源码路径 `/home/h3/.cargo/git/checkouts/obvhs-8e4538d3c2ff8290/a3f3b3b`（**CPU BVH**）。

## 8. 仍未知

- `ppEnabledExtensionNames` / `ppEnabledLayerNames` 数组的**实际内容**（反编译未标注字符串、无反汇编，现由串表 + 流程推断）；
- `applicationName` / `engineName`；
- 结构 `1000276000` / `1000300001` 的名称；
- `is_supported` / `is_compatible` 的外部 caller；
- `order_and_filter_devices` 的具体打分键。
