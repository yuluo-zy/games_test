# raw-init 域：设备枚举 / 筛选 / 排序 / 物理设备预算 复现报告

日期：2026-10-15。权威 = B patched EXE（`/home/ubuntu/test/games_test/tiny-glade.exe`，`/tmp/dis.py` 直读机器码）。
标注：✅ 原版对齐[证据] / ⚠️ 候选或假设 / ❌ 读不出（写明读了什么）。

## 0. 本次最重要的更正（推翻旧分析文档的点）

1. **AvailableGraphicsDevice+0x99 不是“shared 标志”，是设备类别字节**。`0x1418b3560`：`eax = 0x302000104 >> (VkPhysicalDeviceType*8)`，类型 ≥5 取 4，即 Discrete=0、Integrated=1、Virtual=2、Cpu=3、Other=4。`is_compatible`(0x140bda8f0)/retain(0x1418b5680) 里 `[+0x99]==0` 的“分离内存”分支其实是 Discrete；其余类别一律走 `local+host>=1.4GB`。旧实现用 `unified_memory` 当 shared，已改。
2. **门槛输入是堆大小，不是 budget**。`0x1418b3300` 构造 0xa0B 元素时对 `memory_heaps[]` 求和：DEVICE_LOCAL 堆 size → +0x88，非 DEVICE_LOCAL 堆 size → +0x90（循环 `test byte [heap+flags],1`）。旧实现用 budget（无预算扩展就 Unknown），已改成堆大小。
3. **排序是稳定的 driftsort，不是不稳定 introsort**。`FUN_1418bb250` 会分配暂存区（`0x140613c10`，上限 `0xc350 = 8MB/160B` 元素）并以 `eager_sort = (len < 0x41)` 调 `FUN_14213bee0`——这是 Rust `slice::sort_by` 的 `driftsort_main`（`<0x15` 个是 std 的插入排序，也稳定）。所以 tie 保持枚举顺序，Rust 的 `sort_by` 与原版排列相同。
4. **`order_and_filter_devices` 的 retain 只看内存门槛**（`0x1418b5680`），不看驱动/emulated/is_supported；被筛掉的以 tag=3 的 0xf0B 条目 push 进 missing。驱动判定在其调用方。
5. **`missing_devices` 与“被筛掉的设备”无关**：它枚举**系统 DXGI 适配器**（`FUN_1418b29c0`，日志串 "Enumerating DXGI devices" / "DXGI adapters:"），只保留 vendor 0x1002(AMD)/0x10de(NVIDIA)，凡在 Vulkan 设备里找不到的记 tag=2 条目。
6. **`create_device` 按 16B 键选设备，不做逐候选回退，缺失即 panic**。
7. **`performance_info` 返回的是厂商（NVAPI）遥测文本**（"Clocks: "/"P-state: "/"Utilization: "/"Power: "/"Memory: "/"Sensors: "，换行连接；无厂商信息→"Unsupported"），不是堆预算。`refresh_memory_budget` 的 0x110B 快照就是一份 `VkPhysicalDeviceMemoryBudgetPropertiesEXT`（不是 `{u128,u128}×16`）。
8. **`find_vendor_specific_device_handle` 的清单是 NVAPI 物理 GPU**，匹配键是 PCI device id 与 Vulkan `deviceID` 相等，不是名字。

## 1. 排序比较器（工作包 1）

✅ [`0x1418b9130`，跳表 `0x142d03954/68/7c/90` 各 5 项，常量 `mov rsi,-0x7d0/-0xfa0/-0xbb8/-0x3e8`]
- 键：`key = base(kind, emulated) - (local_heap_bytes >> 30)`（有符号 64 位），**升序**；插入排序里 `cmp new,prev; jge → 不移动`，即严格 `<` 才前移（稳定）。
- base 表：Discrete：emulated -2000 / 否则 -4000；Integrated：emulated -2000 / 否则 -3000；Virtual -2000；Cpu -1000；Other 0。emulated 检查（名含 "Direct3D12"(0xa 字节)/"Microsoft Basic Render Driver"(0x1d 字节)，`FUN_140bd7250`）只在 Discrete/Integrated 两个分支里做。
- 4 个跳表（新元素/前一元素，各有“插入主循环”副本）常量逐项核对一致。
- 落码：`AvailableGraphicsDevice::device_order_key`、`sort_devices_original`（`sort_by`，稳定；说明见 §0.3）。`GraphicsDevicesForSurface::order_and_filter_devices` 先对 `devices` 做原排序再算 available/missing；旧的“supported/compatible/类型/budget/heap”候选排序键已删除。
- 测试：`comparator_keys_match_the_original_constants`、`sort_is_ascending_by_key`、`equal_keys_keep_enumeration_order_for_both_sort_paths`（count=2/20/21/64/65/200）。
- ⚠️ 未做“原函数行为注入”（没在 RNE 里喂多设备跑原比较器）：键公式来自机器码常量逐项读出，未用动态 oracle 复核。

## 2. missing / available / create_device（工作包 2）

- ✅ `missing_devices`(0x140bdb090)：输入 0x28B 元素的 DXGI 清单；vendor 0x1002→低字节 1、0x10de→低字节 0，其它厂商丢弃；匹配 =（available 前 16B `Option<u64>` tag==1 且值==适配器 u64@+0x18）**或**（名字长度相等且 memcmp 为 0）；未匹配 → 0xf0B 条目 tag=2。落码：`DxgiAdapter`、`MissingVendor`、`MissingGraphicsDevice::{NotInVulkan,InsufficientMemory}`、`missing_devices`。测试 `missing_devices_matches_by_luid_or_name_and_only_for_amd_nvidia`。
  - ⚠️ 前 16B 的 u64 是 LUID（来自 `PhysicalDevice+0x10`）是候选；证据只到“与 DXGI 适配器 8B 标识比较”。代码用 `VkPhysicalDeviceIDProperties.deviceLUID`。
  - ❌ DXGI 枚举本身（FUN_1418b29c0，Windows API）：不重实现，由调用方提供 `&[DxgiAdapter]`。
  - ❌ puffin scope / 日志级别：debug 级文本 "Enumerating DXGI devices"/"DXGI adapters:" 已加（`3 < max_level` 即 ≥Debug）。
- ✅ `available_devices`(0x140bdb820)：物化 `Vec`（对 8B 元素表调 `FUN_1418b3300` 逐个构造 0xa0B）。`available_devices()` 现返回 `Vec<AvailableGraphicsDevice>`（克隆），不再是借用迭代器。
- ✅ 0xa0B 元素字段（`0x1418b3300` 读出）：+0..0x10 `Option<u64>`(PhysicalDevice+0x10)；+0x10 名 String；+0x28/+0x40 两个 `Option<String>`（来自 PhysicalDevice+0x48/+0x60，疑为驱动名/信息，⚠️ 未用）；+0x58 20B DeviceDriverVersion；+0x78 16B 键（PhysicalDevice+0x690）；+0x88/+0x90 堆大小和；+0x98 恒 0（含义未证实）；+0x99 类别字节。落码：`kind/local_heap_bytes/host_heap_bytes/device_uuid/luid`。
- ✅ `create_device`(0x140bdb940)：在 8B 元素表里找 `[elem+0x690]==key.0 && [elem+0x698]==key.1`；找不到 → `FUN_1428d9310`（`Option::unwrap` on None，init.rs:822）；info 日志 "Attempting to use device {name} driver \"{driver}\" uuid \"{uuid}\""、debug 日志 "Physical device properties: .."；克隆实例 Arc（`lock xadd` 强计数，溢出检查）；然后只对该设备调 `FUN_140bf2560`。落码：`create_device_by_uuid(graphics, [u8;16])`（缺失 panic）。测试 `create_by_uuid_selects_that_device_and_panics_when_absent`（lavapipe 实测）。
  - ⚠️ 16B 键 = `deviceUUID`：证据是日志串标 uuid、16B、create_device 按它匹配；具体取自哪个 Vulkan 字段未读到（`enumerate_physical_devices` closure 未解）。
  - ⚠️ 旧 `create_device(&self, graphics)`（逐候选回退）保留为**候选**，因为现有调用方 `create_headless_device`/测试用它；与原版不同（原版不回退）。
- ✅ 未知驱动/未知预算：原 `order_and_filter` 根本不看驱动；驱动 `Unknown` 在原 `is_compatible` 里 `driver_support != 5` 即 false（0x140bda902-0x140bda90c）。重建保留 `DeviceDecision::Unknown`（24 输入 0 mismatch 的验证未动）并由候选策略 `allow_unknown_original_policy` 放行——这是**候选扩展**。
- ❌ `GraphicsDevicesForSurface::new<Arc<Window>>`：真实函数不在 B 的 rhapsody 段里（n3 的 `0x140340bd0` 来自另一份二进制，字符串地址体系不同，未采信）。B 里 `order_and_filter`/`missing_devices`/`available_devices` 的唯一调用点是 tiny-glade 的大状态机函数（`0x14059538x..0x140595410`，调用顺序：`available_devices`→`missing_devices`→`order_and_filter_devices`），`new` 被内联。重建的 `available_for_surface` 仍是候选构造。

## 3. PhysicalDevice（工作包 3）

- ✅ `performance_info`(0x140c00a80)：`[info+0x10]&1==0` → 11 字节 "Unsupported"（`0x726f707075736e55` + `0x64657472`）；否则按序做 6 次厂商查询并用 "\n"（`0x142b17b60`，1 字节 0x0a）连接：
  1. `FUN_1418b1a40(handle,0)` → "Clocks: "（结果 tag bit0==0 才输出）
  2. `FUN_1418b1bb0` → "P-state: "（返回 0 才输出）
  3. `FUN_1418b1cd0` → "Utilization: "
  4. `FUN_1418b1e40` → "Power: "（首 qword ≥0 才输出）
  5. `FUN_1418b18f0` → "Memory: "（NVAPI `MemoryInfo{dedicated, dedicated_available, system_shared, dedicated_available_current, dedicated_evictions_size, dedicated_evictions}` 的 Debug；tag≠1 才输出）
  6. `FUN_1418b1fb0(handle,0)` → "Sensors: "
  落码：`VendorPerformanceQueries`、`format_vendor_performance_info`、`PhysicalDevice::vendor_performance_info`。测试 `performance_info_text_follows_the_original_sections`。
  - ❌ 5 个值格式化闭包（`FUN_1418c03b0/c02b0/b6390/bffd70/b6310`）与 P-state 枚举（P0..P15/Undefined/All）的文本未逐个读；NVAPI 在 Linux 不可用，值文本由调用方提供。
  - 原 rebuild 的 `PerformanceInfo`（堆预算汇总）保留为**候选**名，现有公开访问器/测试不变，新增 `snapshot: Option<MemoryBudgetSnapshot>` 字段。
- ✅ `refresh_memory_budget`(0x140c01620)：快照载荷 0x110B = `VkPhysicalDeviceMemoryBudgetPropertiesEXT`（驱动查询经对象@+0x88 vtable+0xd8 写入栈上 0x110B，`auStack_488[0]=0x3b9e67c8` 即 sType 1000237000）；旧预算读自载荷+0x10..0x90（16×u64）；16 处同构钳位 `lo=old-max(1,old>>3)`、`hi=old+max(1,old>>3)`、`new=clamp(new,lo,hi)`，**只钳 budget，usage 取驱动值**；存入 ArcSwap（分配 0x120B = 2×u64 计数 + 0x110B 载荷，`xchg` 到 +0x608，旧值 `-1` 引用计数）。落码：`MemoryBudgetSnapshot{heap_budget[16],heap_usage[16]}`、`BUDGET_SNAPSHOT_BYTES=0x110`、`MAX_MEMORY_HEAPS=16`、`clamped_against`、`to_vk/from_vk`，`refresh_memory_budget` 现对全部 16 槽做钳位。测试 `budget_snapshot_layout_clamp_and_sums`（含 size_of==0x110）。**更正旧文档**：不是 `{u128,u128}` 步长 0x20，而是 8B 步长两个数组（getter 标量循环 `plStack_50[uVar6+4]` 即 `base+0x20+8i`）。
  - ⚠️ 首次刷新：原版在枚举阶段先放入初始快照（未读该 closure）；重建用“无旧快照则原样采用”等价。
  - ⚠️ 原 `hi=old+delta` 溢出时（旧值逼近 u64::MAX）`lo<=hi` 为假的分支未读，重建用饱和运算。
- ✅ `device_local_memory_budget_bytes`(0x140c006a0)/`host_memory_budget_bytes`(0x140c00890)：ArcSwap load 后对 `memory_heap_count`(≤16，>16 即 slice panic)个堆，flags&1 为 DEVICE_LOCAL 的求和 budget[i]（host 为反向），加法回绕。落码 `MemoryBudgetSnapshot::sum_budget`（wrapping）。
- ✅ `host_memory_bytes`：非 DEVICE_LOCAL 堆 size 之和（原 +0x90 的求和循环确认，此前为 I）。`device_local_memory_bytes`（+0x88）同。
- ✅ `find_vendor_specific_device_handle`(0x140c01e80)：仅 `vendorID==0x10de`；遍历 NVAPI 枚举清单（FUN_1418b1340），每项取 PCI 标识（FUN_1418b17b0）：失败→warn "NVAPI: No PCI identifier"（level≥2）并继续；`pci.device_id == props.deviceID` → info "Matched to NVAPI device"（level≥3）并返回 `(Ok, handle)`；否则 `FUN_1428caaa0("No vendor-specific info")` 后返回 1。落码：`VendorDeviceEntry`、`match_vendor_device_handle`、`PhysicalDevice::find_vendor_specific_device_handle(&[VendorDeviceEntry]) -> Option<u64>`（签名变了，无外部调用方）。旧的“名字包含”实现已删。测试 `vendor_handle_match_is_nvidia_only_and_skips_entries_without_pci_id`。❌ NVAPI 清单来源不实现；末尾那次 `FUN_1428caaa0` 的日志级别未证实。
- ✅ `DeviceDriverVersion::vendor_string`(0x140c001c0)：tag→串表 `{NVIDIA(6),Intel(5),Intel(5),AMD(3),Qualcomm(8),Apple(5),Unknown(7)}`（表 `0x142b17d38`/跳表 `0x142b17d70`，自读）；`is_amd`：`cmp dword [rcx],3`；`is_apple`：`cmp dword [rcx],5`。现实现一致，未改。
- ❌ `parse_driver_version`（line 581）：无地址、无反编译（私有 inlinee），原驱动版本编码未读到；现有 `from_vulkan`（NVIDIA `>>22/>>14&255/>>6&255/&63`、Intel Windows `>>14/&0x3fff`）由 24 输入 driver_support 验证背书，未动。`pub fn parse_driver_version(raw)->[u32;3]` 是旧候选，证据不足，原样保留。
- ❌ `enumerate_physical_devices` closure$0..4（line 336-473）、`impl$6::with_presentation_support`（0x140bdcf90 属于哪个类型不明）：无反编译，未改；n3 报告说 `with_presentation_support` 是“队列族 stride 0x690/谓词过滤”，与 B 不一致处未采信。

## 4. init.rs 其余（工作包 4）

- ✅ `AvailableGraphicsDeviceInitState::is_error`(0x140a9fa70)：`cmp byte [rcx],1; sete al`。新增 `AvailableGraphicsDeviceInitState{tag:u8}`；❌ 其余判别值/载荷未证实。
- ✅ `GraphicsInstanceLayer::vendor`、`Allowlist::contains_layer/vendor`、Ron migration：与证据一致，未改（Allowlist serde 是 derive，I）。
- ❌/⚠️ `GraphicsInstanceLoader::new`(0x140bd8af0)：该类型在 `instance.rs`（非我拥有）。已读证据：`vulkan-1.dll`→`fallback-vulkan-1.dll`→系统，`try_enumerate_instance_version`，<1.3.261 兜底 0x400000。**需要 instance.rs 的 owner 对照**；我没有改它。
- ⚠️ `implicit_layers`(0x140bda7b0)：同在 instance.rs。读到的返回形态是 `vec::IntoIter<vk::LayerProperties>`（`{buf,ptr,cap,end=ptr+count*0x208}`，不是 `&[..]`），失败 `.expect("Failed to enumerate instance layer properties")`；`rne/implicit-layers-slice.md` 的 “&[LayerProperties]” 表述不精确。需 instance.rs 的 owner 处理。
- ⚠️ `GraphicsInstance::new`(0x140bd7dd0)：原签名 `(out, loader, &[&str] 层名, &(u8,u8))`，克隆名表、枚举层并逐个 debug 日志、`InstanceBuilder::build`(0x140c08f10)、info "Initializing NVAPI"，随后 NVAPI 初始化(FUN_1418b1240)并列出 NVAPI 设备。重建 `GraphicsInstance::new()/new_with_allowlist()` 是候选（无 NVAPI），未改。

## 5. ALGORITHMS §4 / 审查节 U 点逐个落点

| U/半/✗ | 结论 |
|---|---|
| §4.1 比较器权重 | ✅ 已落码（§1），键公式为机器码常量 |
| §4.2 implicit_layers 来源 | ⚠️ 读到 `IntoIter<LayerProperties>`，在 instance.rs，交付给其 owner |
| §4.3 blocklist 粒度 | 不在我文件，未处理 |
| §4.4 refresh 滞回/快照 | ✅ 快照 = `VkPhysicalDeviceMemoryBudgetPropertiesEXT`，只钳 budget |
| §4.5 performance_info 6 查询 | ✅ 含义已定（§3）；❌ 值格式化闭包与 NVAPI 本体未做 |
| §4.6 missing/create_device 字段 | ✅ missing=DXGI 对 Vulkan 的 LUID/名字比对；create_device 键=16B（⚠️ deviceUUID） |
| §4.7 find_vendor 清单 | ✅ NVAPI 物理 GPU + PCI deviceID；仅 NVIDIA |
| 半：`available_devices` 物化 | ✅ 改为返回 Vec |
| 半：`GraphicsDevicesForSurface::new` | ❌ 被内联，未找到独立函数（见 §2） |
| ✗：order 比较器 closure$0/$1 | ✅（§1，内联在 std helper 里） |
| ✗：`PhysicalDevice::host_memory_bytes` I | ✅ 升为 O |

## 6. 改动文件 / 测试 / 遗留

改动（仅我拥有的）：
- `crates/rhapsody/src/raw/init.rs`：`device_order_key`/`sort_devices_original`/`meets_original_memory_threshold`/`missing_devices`/`order_and_filter_devices_original`/`DxgiAdapter`/`MissingGraphicsDevice`/`AvailableGraphicsDeviceInitState`/`create_device_by_uuid`；`AvailableGraphicsDevice` 新增 `kind/local_heap_bytes/host_heap_bytes/device_uuid/luid`；`available_devices` 返回 Vec；`original_memory_metrics` 改为堆大小+类别字节；删旧候选排序键；`GraphicsInstance::create_device`（设备创建段）未动。
- `crates/rhapsody/src/raw/physical_device.rs`：`GraphicsDeviceKind`、`MemoryBudgetSnapshot`、`PerformanceInfo.snapshot`、`refresh_memory_budget` 全 16 槽钳位、`find_vendor_specific_device_handle` 重写、`id_properties/luid`、`vendor_performance_info`。
- 新增测试 `crates/rhapsody/tests/raw_init_devices.rs`（13 项）；`tests/device_creation.rs`、`tests/budget_snapshot.rs` 未改且通过。
- 验证：`cargo test --offline -p rhapsody -p tiny-glade -p country-core --all-features` 全部通过（32 个 result 行均 ok，无 failed）；另用 `VK_ICD_FILENAMES=…/lvp_icd.json` 单独跑 rhapsody 三个设备测试通过。

遗留 / 需他人配合：
1. `instance.rs` owner：核对 `GraphicsInstanceLoader::new`、`implicit_layers` 返回形态（`IntoIter<LayerProperties>`）。
2. 行为差异（有意保留的候选）：`order_and_filter_devices` 仍叠加候选策略原因（api 版本/timeline/队列/surface/扩展/驱动）；旧 `create_device` 逐候选回退。若要完全字面，调用方应改用 `original_order_and_filter(adapters)` 与 `create_device_by_uuid`。
3. 未做动态 oracle：排序键与门槛来自机器码常量逐项核对，没有在 RNE 里对原比较器喂多设备复核。
4. `device_uuid`/`luid` 的 Vulkan 字段归属、`AvailableGraphicsDevice+0x98`、+0x28/+0x40 两个 `Option<String>` 的含义未证实。
