# rhapsody raw-init 域：函数骨架/数据结构/业务算法深挖

- 日期：2026-10-15。范围：`src/raw/{init,instance,physical_device,format,allocator,crash,disposal,error,bytes,workarounds,ray_tracing}.rs` + `src/leek.rs` + `src/lib.rs`。
- 域清单：`domains/raw-init.json`（87 compiled / 47 有地址）。反编译 `domains/raw-init/decomp/`（dbid `tg_rh4`），9 个 NotFound 用 rne-read 原字节补证。
- 标注：**O**=机器码/PDB 实测；**I**=推断；**U**=未知。权威 = B patched EXE / 原 RNE。
- 对照基准：`reconstruction/rhapsody-rebuild/src/**` + `MODULE_STATUS.md`。**不改任何 reconstruction 源码。**

## 0. 结论速览

1. **设备筛选策略**（AvailableGraphicsDevice 判据）与 rebuild **完全对上**（device-policy-functional-validation：24 输入 0 mismatch，原驱动判定 `driver_support` 真入口执行）。内存门槛 1.0GB/400MB（分离）/1.4GB（共享）、`is_old_intel` 的 11 条串表逐字对上。
2. **未知驱动/未知预算的处理**：原实现遇未知驱动编码或无预算扩展时**不是**排除设备，而是把该设备/预算标为"未知"，排序与筛选策略保留 Unknown（重建候选 `allow_unknown_original_policy` 是候选扩展，原 `Unknown` 语义经 24 案例验证一致）。
3. **原 `apply_vulkan_blocklist` 是环境变量驱动的隐式层屏蔽**（DISABLE_RTSS/VKBASALT/GAMEPP/SAMPLE_LAYER/NV_OPTIMUS/AMD_SWITCHABLE、VK_LOADER_LAYERS_DISABLE=~implicit~、VK_LAYER_bandicam_helper_DEBUG_1）。rebuild 的 modern/legacy/doubletap 三个黑名单是**空表候选**，与原行为差异明确。
4. **disposal 原结构是 `hashbrown::HashMap<CommandBufferSeq, Mutex<DisposalBucket>>`**（PDB 类型图 O），每桶 parking_lot Mutex；`insert` 断言 `prev.is_none()`（重复 seq panic），`remove` 缺桶 panic（"bucket removed prematurely or command buffer begin() never called?"）。rebuild 用无锁 `BTreeMap<u64, Vec<DestroyAction>>`、允许重复、remove 返回 Option——**结构与方法语义均偏离**。
5. **`refresh_memory_budget` 有逐堆滞回钳位**：新预算钳制在 `[旧-旧/8, 旧+旧/8]`（O）。rebuild 只重查询不加滞回——算法差异。
6. **lib.rs 顶层日志（init_logging/print_log_dump/dump_log + 0x143634e40 全局 64KB 环形缓冲）rebuild 缺失**（rebuild lib.rs 只有模块声明）。dump_log 被 `tiny_glade::panic_reporter::panic_dump::dump_rhapsody_log` 调用（O）。
7. **格式三入口**与 rebuild observed 表逐案例对上（816 对比 0 mismatch），`FormatInfo` 10B 布局、aspect 表、block_bytes 位掩码分支确认。
8. **GpuAllocator 峰值包装布局确认**：`track_peaks@+0x688`、`peak_report(Option<Box/&>)@+0x620`、`peak_device_local@+0x670`、`peak_system@+0x678`、`peak_total@+0x680`；allocate 后采样、free 不采样（allocator-functional-validation 16 案例 0 mismatch）。
9. **ray_tracing.rs** 全为 inlinee_declaration（无地址、无反编译）；rebuild 无该模块——确认"ray tracing 完整路径未验收"。

---

## 1. 模块总览

### 1.1 文件 → 职责

| 文件 | 职责 | 关键类型（PDB O） |
|---|---|---|
| `raw/init.rs` | 加载器/实例/设备候选构建与筛选 | `GraphicsInstanceLoader`、`GraphicsInstanceLayer{name,description}`、`GraphicsInstanceLayerAllowlist`、`GraphicsInstanceLayerAllowlistItem{Layer=0,Vendor=1}`、`GraphicsInstance`、`AvailableGraphicsDevice`（0xa0B）、`AvailableGraphicsDeviceInitState`、`GraphicsDevicesForSurface`、`UnavailableGraphicsDevice`、`GraphicsInitError`、`LocationToTry`（loader 候选） |
| `raw/instance.rs` | 实例构建与层黑名单 | `InstanceBuilder{app_name,api_version,extensions,layers,debug_markers}`、`Instance`（0x118B 对象） |
| `raw/physical_device.rs` | 物理设备能力/预算/驱动版本 | `PhysicalDevice`（heap count@+0x504、heaps@+0x510、预算 ArcSwap@+0x608）、`DeviceDriverVersion{vendor_tag@0(u32),…}`、`DeviceDriverInfo`、`QueueFamily`、`PerformanceInfo`（预算数组@+0x20） |
| `raw/format.rs` | Vulkan 格式注册表 | `FormatInfo` 10B {word0@0, word1@4, byte@8=byte@9}；表 `0x142ccdb70/0x142ccddbc/0x142cce008`；aspect 表 `0x142ccd890` |
| `raw/allocator.rs` | gpu-allocator 私有峰值包装 | `GpuAllocator`{Allocator 0x620B, peak_report@+0x620, peaks@+0x670/678/680, track_peaks@+0x688} |
| `raw/crash.rs` | GPU 崩溃诊断 | `CrashMarkerNames`、debug 记录 {kind<<24\|line, src_id, 2 operands} |
| `raw/disposal.rs` | 命令缓冲销毁桶 | `CommandBufferDisposalBuckets`=HashMap<CommandBufferSeq, Mutex<DisposalBucket>>、`DisposalBucket`、`SequentialDisposal{last_retired}` |
| `raw/error.rs` | `BackendError` 枚举 | Debug/Display/From 派生 |
| `raw/bytes.rs` | 字节重解释 | `slice_as_bytes`/`bytes_as_slice` |
| `raw/workarounds.rs` | `GraphicsWorkarounds` serde（Ron 白名单 item 格式） | 枚举 serde visitor |
| `raw/ray_tracing.rs` | 光线追踪加速结构（仅声明） | `RayTracingAcceleration`(Arc)、`GeometryInstance` |
| `leek.rs` | leek 错误包装 + 错误格式化 | `leek::Error`、`impl$0::fmt`(Debug)、`impl$1::from`(utf8 校验转换)、`impl$2::fmt`(Display) |
| `lib.rs` | crate 根 + 日志初始化 | `init_logging`/`print_log_dump`/`dump_log` + 全局 `0x143634e40` 64KB 缓冲 |

### 1.2 处理链（函数调用图，O）

```
tiny_glade::main
  ├─ init_logging(0x140be6a90)  # 初始化全局 64KB 日志环
  ├─ print_log_dump(0x140be6b40)  # jmp thunk（ICF）
  └─ … run_game → driver_support(0x140bda990) ×多（设备判定）
tiny_glade::panic_reporter::panic_dump::dump_rhapsody_log
  └─ dump_log(0x140be6b50)  # 输出全局日志

GraphicsInstanceLoader::new(0x140bd8af0)
  ├─ ash Entry 加载："vulkan-1.dll"(0x142b11288) → "fallback-vulkan-1.dll"(0x142b112c8)
  │   失败 panic "called `Result::unwrap()` on an `Err` value"(0x142b10490)
  ├─ try_enumerate_instance_version()（loader 版本；<1.3.261 走 1.0 兜底 0x400000）
  ├─ enumerate_instance_extension_properties / layer_properties
  └─ 输出 {extensions, layers, attempts, api_version}

GraphicsInstance::new(0x140bd7dd0)
  ├─ 克隆 layer/extension 名 Vec（0x10B 元素）
  ├─ 枚举 instance layer properties（FUN_1418b09f0；失败 panic "Failed to enumerate instance layer properties" 0x142b1178e）
  │   （每个 0x208B LayerProperties → GraphicsInstanceLayer，puffin scope "rhapsody::raw::init"）
  └─ InstanceBuilder::build(0x140c08f10) → Instance 对象（0x118B）

InstanceBuilder::build(0x140c08f10)
  ├─ 组装扩展/层名（默认含 VK_KHR_surface 等常量 0x142b1909b/0x142b190c2）
  ├─ 可用性检查（扩展 0x16 字符名 memcmp；层查询 "…"）
  ├─ loader vkCreateInstance 间接调用（函数表 +0x28）
  └─ Result{tag, Box<Instance 0x118B>}；错误路径 "creating a Vulkan instance"(0x142b11250)

apply_vulkan_blocklist(0x140c0a530)
  ├─ 逐层过滤（0x20B 条目）
  └─ 环境变量层屏蔽链：VK_LOADER_LAYERS_DISABLE(~implicit~) → VK_LOADER_LAYERS_ALLOW →
     DISABLE_RTSS_LAYER → DISABLE_VKBASALT → VK_LAYER_bandicam_helper_DEBUG_1 →
     NODEVICE_SELECT → DISABLE_GAMEPP_LAYER → DISABLE_SAMPLE_LAYER →
     DISABLE_LAYER_NV_OPTIMUS_1 → DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1（值含 "1" 触发）

enumerate_physical_devices(0x140c01e00) → PhysicalDevice 列表
  └─ enumerate_vk_data<PhysicalDevice>(0x18cd4d0)：两次调用（count 查询 + 取数）

AvailableGraphicsDevice::is_supported(0x140bda840)
  ├─ driver_support(0x140bda990)：vendor 跳表(0x142b12544)
  ├─ is_emulated(0x140bdaec0)：Direct3D12 / Microsoft Basic Render Driver
  ├─ has_enough_video_memory(内联)：local≥1.0GB && host≥400MB；shared 则和≥1.4GB
  └─ is_old_intel(0x140bdaf20)：11 条串表
GraphicsDevicesForSurface::order_and_filter_devices(0x140bdbc30)
  ├─ 0xa0B 元素排序（<0x15 插入排序 / ≥0x15 introsort；比较器 closure 未完全恢复）
  └─ 填 available / missing（missing 条目 0x28B，含 vendor id 0x1002/0x10de）
GraphicsDevicesForSurface::create_device(0x140bdb940)
  ├─ 遍历 available 匹配 (+0x690/+0x698 两 qword)
  ├─ clone 实例 Arc（param_2+0x18）
  └─ FUN_140bf2560 → 真实 vkCreateDevice

GpuAllocator::allocate(0x140c0da40)
  ├─ gpu_allocator Allocator::allocate（FUN_140c12dc0）
  └─ 若 [self+0x688]==1：generate_report → 峰值 u128 比较(+0x670) → total 严格增则替换 peak_report(+0x620)；puffin 事件
GpuAllocator::free(0x140c0e150)  # 不采样峰值；内部释放 + 断言

PhysicalDevice::refresh_memory_budget(0x140c01620)
  ├─ 驱动预算查询（对象@+0x88 vtable +0xd8，arg@+0xc0）
  ├─ 逐堆滞回钳位 [旧-旧/8, 旧+旧/8]
  └─ ArcSwap store 新快照(0x110B payload) 到 +0x608
PhysicalDevice::device_local_memory_budget_bytes(0x140c006a0)
  └─ ArcSwap load → 按 DEVICE_LOCAL 堆求和 budget 数组(@+0x20)

crash 链（rebuild 对照 + crash-functional-validation）：
  record_crash_marker：id 分配，history slot = id&4095，cmdFillBuffer 写 4B
  has_any_gpu_assertion_failed：buffer word[1]!=0
  needs_vulkan_error_info：TIMEOUT(2)/DEVICE_LOST(-4)
  log_gpu_debug_info(0x1418c46c0)：逐 record kind 1..6 格式化（abort/>/</>=/<=）+ 输出
```

---

## 2. 逐函数表（name | addr | role | 算法要点 | verdict | evidence）

### 2.1 raw/init.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `GraphicsInstanceLoader::new` | 0x140bd8af0 | loader 加载+枚举 | 尝试 "vulkan-1.dll"→"fallback-vulkan-1.dll"→系统；记录 attempts；枚举扩展/层；api_version（<1.3.261 兜底 0x400000） | O | decomp 0x140bd8af0；串 0x142b11288/0x142b112c8/0x142b10490 |
| `GraphicsInstanceLoader::implicit_layers` | 0x140bda7b0 | implicit 层清单 | 枚举 layer properties（FUN_1418b09f0）；元素 0x208B；失败 panic "Failed to enumerate instance layer properties" | O | decomp；串 0x142b1178e |
| `impl$10::fmt`（line 35） | — | GraphicsInstanceLayer Debug | Formatter debug | I（std 派生） | 骨架 |
| `impl$14::from`（line 35） | — | layer 转换 | From 转换 | U | 骨架 |
| `GraphicsInstanceLayer::vendor` | 0x140bd7bc0 | 解析 layer 厂商 | 校验 "VK_LAYER_"（8B bswap 比较）+byte[8]=='_'；扫描首个 '_'，返回前缀（空→None） | O（与 rebuild 等价） | decomp；rebuild init.rs:20 |
| `GraphicsInstanceLayerAllowlist::contains_layer` | 0x140bd7cf0 | 白名单含 layer | 遍历 0x20B 条目：tag&1==0（Layer）+ String 相等 | O | decomp；条目布局 {tag@0,pad,String@0x10} |
| `GraphicsInstanceLayerAllowlist::contains_vendor` | 0x140bd7d60 | 白名单含 vendor | tag==1（Vendor）+ String 相等 | O | decomp |
| `_::impl$0::deserialize`（line 93） | — | Allowlist Ron deserialize | serde visitor（visit_str/visit_map/visit_enum） | I（derive） | 骨架 |
| `_::impl$0::serialize<ron>`（line 93） | — | Allowlist Ron serialize | Ron serializer | I（derive） | 骨架 |
| `impl$18::clone`/`impl$20::eq`/`impl$22::partial_cmp`/`impl$24::clone` | — | 派生 impl | clone/eq/partial_cmp | I（derive） | 骨架 |
| `GraphicsInstance::new` | 0x140bd7dd0 | 构建实例+枚举层 | 克隆名列表→枚举层→`InstanceBuilder::build`→Result{0x18/0x16 tag}；错误串 "creating a Vulkan instance" | O | decomp |
| `AvailableGraphicsDeviceInitState::is_error` | 0x140a9fa70 | init 状态判错 | `cmp byte[rcx],1; sete al`（self+0==1） | O | rne-read 原始字节 |
| `impl$28::fmt`/`impl$30::eq`/`impl$32::clone`/`impl$33::fmt`（line 442/451） | — | DriverSupport/D 派生 | 枚举 fmt/eq/clone | I | 骨架 |
| `AvailableGraphicsDevice::driver_support` | 0x140bda990 | 驱动支持判定 | 先查 emulated 名；再 vendor 跳表(0x142b12544，key=+0x58 u32) 分支；输出 10-dword DriverSupport；**RTX50 特定坏驱动 "580.97/580.95"** | O（24 输入 0 mismatch） | decomp；device-policy-functional-validation；串 0x142b117c0 含 "RTX 50/580.97" |
| `AvailableGraphicsDevice::is_emulated` | 0x140bdaec0 | emulated 判定 | 名含 "Direct3D12"(10) 或 "Microsoft Basic Render Driver"(29，SIMD memmem) → true | O | decomp；串 0x142b1188b/0x142b11895 |
| `AvailableGraphicsDevice::is_old_intel` | 0x140bdaf20 | 旧 Intel 判定 | vendor==1(IntelWindows) && 含 "HD Graphics"||"Iris(R)" && !含 {Xe,11th,Gen11,12th,Gen12,13th,Gen13,Graphics 7,Graphics P7} | O（11 串逐字对上） | decomp；串 0x142b118b2..0x142b118eb |
| `AvailableGraphicsDevice::is_compatible` | 0x140bda8f0 | 兼容判定 | driver==Supported && !emulated && 内存门槛（**不含 old_intel**） | O | decomp |
| `AvailableGraphicsDevice::is_supported` | 0x140bda840 | 支持判定 | is_compatible 且再排除 old_intel（`old_intel^1`） | O | decomp |
| `has_enough_video_memory`（line 674，inline） | — | 内存门槛 | 分离：local≥1.0GB && host≥400MB；共享(+0x99)：和≥1.4GB | O | decomp is_supported/is_compatible 内联 |
| `GraphicsDevicesForSurface::new<Arc<Window>>`（line 693） | — | surface 设备集构造 | 遍历设备→surface present 查询 | I/U | 骨架 |
| `GraphicsDevicesForSurface::order_and_filter_devices` | 0x140bdbc30 | 排序+过滤 | 0xa0B 元素排序（<0x15 插入排序，≥0x15 introsort FUN_1418bb250）；比较器 = closure$0/$1（line 838/857）**未完全恢复**；然后填 available/missing | O（排序存在）/ U（比较器） | decomp |
| `GraphicsDevicesForSurface::missing_devices` | 0x140bdb090 | 缺设备清单 | 物化 missing 列表（条目 0x28B，含 vendor id 0x1002/0x10de 匹配）；puffin 日志 | O/I | decomp |
| `GraphicsDevicesForSurface::available_devices` | 0x140bdb820 | 可用设备清单 | 由 available 索引（8B 元素）物化 0xa0B 设备副本 Vec | O（返回物化 Vec，非迭代器） | decomp |
| `GraphicsDevicesForSurface::create_device` | 0x140bdb940 | 创建设备 | 遍历 available 匹配设备 (+0x690/+0x698 字段)；clone 实例 Arc(+0x18)；FUN_140bf2560 创建；缺候选 panic | O/I | decomp |
| `impl$7::*::closure$0/$1/$2` | — | 各闭包 | 内联（排序比较器/surface 查询） | U | 骨架 |

### 2.2 raw/instance.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$2::default`（line 18） | — | InstanceBuilder Default | 默认 api_version/空列表 | I | 骨架 |
| `InstanceBuilder::build` | 0x140c08f10 | 创建 Vulkan 实例 | 组装扩展/层名；可用性检查（0x16 字符 memcmp）；loader 函数表 vkCreateInstance；Result{tag,Box<Instance 0x118B>} | O | decomp；串 0x142b1909b/0x142b190c2/0x142b11250 |
| `InstanceBuilder::debug_markers`（line 40） | — | 启用 debug utils | push "VK_EXT_debug_utils" | I | 骨架 |
| `Instance::extension_names`/`layer_names`（line 58/101） | — | 名称访问器 | 返回 Vec<String> 引用 | I | 骨架 |
| `Instance::create`（line 109）+ closures | — | 创建入口 | 内联 | I/U | 骨架 |
| `apply_vulkan_blocklist` | 0x140c0a530 | 隐式层屏蔽 | 逐层过滤 + **环境变量链**：VK_LOADER_LAYERS_DISABLE(~implicit~)、VK_LOADER_LAYERS_ALLOW、DISABLE_RTSS_LAYER、DISABLE_VKBASALT、VK_LAYER_bandicam_helper_DEBUG_1、NODEVICE_SELECT、DISABLE_GAMEPP_LAYER、DISABLE_SAMPLE_LAYER、DISABLE_LAYER_NV_OPTIMUS_1、DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1（值含 "1"） | O | decomp；串 0x142b19428..0x142b196f8 |
| `apply_vulkan_modern_blocklist`/`legacy`/`doubletap`（line 233/265/290） | — | 三套黑名单 | 内联于 apply_vulkan_blocklist | U（原黑名单条目=上面 env 表，无独立空表） | 骨架 |
| `legacy_disable_layer`（line 322） | — | 移除层 | retain 非目标层 | I | 骨架 |
| `vulkan_debug_messenger`（line 330，va null） | — | 创建 debug messenger | VK_EXT_debug_utils | I | 骨架 |

### 2.3 raw/physical_device.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$1/3/13/16/18/21::fmt`（line 0/24/36/42/73） | — | 各 fmt | Debug/Display | I | 骨架 |
| `impl$17/19::clone` | — | clone | 派生 | I | 骨架 |
| `DeviceDriverVersion::vendor_string` | 0x140c001c0 | 厂商名字符串 | 表驱动 {offset,len} 字符串表；self+0 u32 tag | O | rne-read 原始字节 |
| `DeviceDriverVersion::is_amd` | 0x140c001e0 | AMD 判定 | `cmp dword[rcx],3; sete al`（tag==3） | O | rne-read |
| `DeviceDriverVersion::is_apple` | 0x140c001f0 | Apple 判定 | `cmp dword[rcx],5`（tag==5） | O | rne-read |
| `PhysicalDevice::name` | 0x140c00400 | 设备名 | CStr@+0xdc → String | O | decomp |
| `PhysicalDevice::device_local_memory_bytes` | 0x140c00530 | DEVICE_LOCAL 堆大小和 | heaps@+0x510，flags mask 0x1（DEVICE_LOCAL）筛堆求和 | O | decomp |
| `PhysicalDevice::device_local_memory_budget_bytes` | 0x140c006a0 | 设备本地预算 | ArcSwap load(+0x608)→budget 数组(@+0x20)；仅 DEVICE_LOCAL 堆（~auVar14&）；释放 Arc | O | decomp |
| `PhysicalDevice::host_memory_budget_bytes` | 0x140c00890 | 宿主预算 | 同上，仅非 DEVICE_LOCAL 堆（auVar13&） | O | decomp |
| `PhysicalDevice::host_memory_bytes`（line 191） | — | 宿主内存大小 | 非 DEVICE_LOCAL 堆和 | I | 骨架 |
| `PhysicalDevice::performance_info` | 0x140c00a80 | 完整能力/预算 | 无预算扩展→vendor_specific="Unsupported"(0x726f707075736e55)；有→6 次堆查询（FUN_1418b1a40/b1bb0/b1cd0/b1e40/b18f0/b1fb0）物化 0x18B 记录 | O/I | decomp |
| `PhysicalDevice::refresh_memory_budget` | 0x140c01620 | 刷新预算 | 驱动查询（对象@+0x88 vtable+0xd8，arg@+0xc0）→**逐堆滞回 [旧-旧/8, 旧+旧/8]**→ArcSwap store 0x110B 快照 | O | decomp |
| `impl$5::fmt`（line 300） | — | fmt | Debug | I | 骨架 |
| `enumerate_vk_data<u32,PhysicalDevice>` | 0x18cd4d0 | 枚举助手 | 先 count 查询再取数（两次调用） | O | skeleton lib.rs 引用 0xc01e11 |
| `enumerate_physical_devices` | 0x140c01e00 | 设备枚举 | ash enumerate；Result{Err niche=0x8000000000000000} | O | decomp |
| `enumerate_physical_devices::closure$0..4`（line 336-473） | — | 枚举闭包 | 内联（失败处理/过滤） | U | 骨架 |
| `find_vendor_specific_device_handle` | 0x140c01e80 | 厂商特定设备 | 仅 vendorID==0x10de(NVIDIA)；枚举清单匹配 deviceID(+0xc)；puffin | O/I | decomp |
| `string_from_c_array`（line 524） | — | C 数组→String | 到 '\0' 截断 | I | 骨架 |
| `impl$6::with_presentation_support`（line 540，va null） | — | present 队列查询 | 逐队列 get_physical_device_surface_support | I | 骨架 |
| `parse_driver_version`（line 581） | — | 版本解析 | 原驱动版本格式 | I | 骨架 |

### 2.4 raw/format.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `vk_format_valid_aspect_flags` | 0x141738fe0 | aspect 标志 | index=format-1 < 0xb8(184) → 表 0x142ccd890[4B]；否则 panic | O | decomp；串 0x142ccd7b8 |
| `vk_format_info` | 0x141739060 | 格式信息 | format<0x93(147) → 写 10B {word0@0x142ccdb70, word1@0x142ccddbc, byte8@0x142cce008 ×2}；否则 panic | O（816 案例 0 mismatch） | decomp；format-functional-validation |
| `vk_format_block_bytes` | 0x1417390e0 | 块字节 | format-1<0x82(130) → 1；uVar3<0x10 → 位掩码 0xccc0→16 / (0x3\|idx==8)→8；否则 panic | O | decomp |
| `impl$1::eq`（line 197） | — | FormatInfo eq | 派生 | I | 骨架 |

### 2.5 raw/allocator.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `GpuAllocator::new` | 0x140c0d980 | 构造 | gpu_allocator Allocator::new（FUN_140c120d0，0x5f8B）→ init peak_report=Option None(i64::MIN)、peaks=0、track_peaks=param_3(u8)@+0x688 | O | decomp |
| `GpuAllocator::allocate` | 0x140c0da40 | 分配+峰值采样 | Allocator::allocate→若[+0x688]==1：report→u128 峰值比较(+0x670 XOR 0x1429258f0)→total 严格增则替换 report(+0x620)；puffin 事件；**free 不采样** | O（16 案例 0 mismatch） | decomp；allocator-functional-validation |
| `GpuAllocator::free` | 0x140c0e150 | 释放 | 内部释放（FUN_140c16d40）+ 断言（全局 0x143634e4? 计数器）；不采样峰值 | O | decomp |
| `GpuAllocator::rename_allocation`（line 87） | — | 命名 | 改名 | I | 骨架 |
| `GpuAllocator::inner` | 0x14067dd80 | 内部访问 | `mov rax, rcx; ret`（返回 &self） | O | rne-read |
| `peak_total_allocated_bytes` | 0x140c0e600 | 峰值总计 | `mov rax,[rcx+0x680]` | O | rne-read |
| `peak_allocator_report` | 0x140c0e610 | 峰值报表 | +0x620 空判→Option（null→None） | O | rne-read |
| `peak_device_local_bytes` | 0x140c0e630 | 峰值本地 | `mov rax,[rcx+0x670]` | O | rne-read |
| `peak_system_bytes` | 0x140c0e640 | 峰值系统 | `mov rax,[rcx+0x678]` | O | rne-read |

### 2.6 raw/crash.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `log_gpu_debug_info` | 0x1418c46c0 | 解析 GPU 调试包 | 逐 0x10B record：kind=word0>>24；1=abort({l},{r})、2="assertion failed"、3/4/5/6=>/<//=/<=、default "???"；行号=word0&0xffffff；src_id=word1；operands=word2/3；puffin scope "rhapsody::raw::crash" | O | decomp；串 0x142d06be0..0x142d06d30、0x142d06ba0 |
| `GraphicsDevice::log_gpu_crash_info`（line 0，va null） | — | 快照读回 | wait_idle→marker/debug buffer 读回→snapshot | I（对照 crash-validation） | crash-functional-validation |
| `CrashMarkerNames::insert_name`/`get_name`（line 12/23） | — | 名称↔id | 双向表 | I | 骨架 |
| `GraphicsDevice::record_crash_marker`（line 34，va null） | — | 写 marker | id 分配；history slot=id&4095；cmdFillBuffer 写 4B=id | O | crash-functional-validation native_marker_entries |
| `GraphicsDevice::has_any_gpu_assertion_failed`（line 118，va null） | — | 断言检测 | buffer word[1]!=0；空 buffer false | O | crash-validation native_assertion_cases |
| `GraphicsDevice::log_vulkan_error_info`（line 51，va null） | — | 错误门控 | TIMEOUT(2)/DEVICE_LOST(-4) 才请求诊断 | O | crash-validation native_error_classification |
| `GraphicsDevice::log_error_info`（line 128，va null） | — | 错误信息 | 候选包装 | I | 骨架 |

### 2.7 raw/disposal.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `CommandBufferDisposalBuckets::remove` | 0x1418c4100 | 取出桶 | 加锁（parking_lot 状态 0→8；wait 1s）→hashbrown remove→**缺桶 panic "bucket removed prematurely or command buffer begin() never called?"(0x142d069c1)** | O | decomp；PDB 类型图（HashMap<CommandBufferSeq, Mutex<DisposalBucket>>） |
| `CommandBufferDisposalBuckets::insert` | 0x1418c4260 | 插入桶 | 加锁→新空 Vec→hashbrown insert→**prev 非 None 时 panic "assertion failed: prev.is_none()"(0x142d06a40)** | O | decomp |
| `dispose_cb_resource<Managed*>`（6 实例，line 35） | — | 资源入桶 | 闭包 drop(managed) | I | 骨架 |
| `SequentialDisposal::dispose_in_order<reclaim closure>`（line 0，0x18c7ae0） | — | 顺序销毁 | 断言 `seq > self.last_retired`（0x142d06f88 后接 "crates/rhapsody/src/raw/disposal.rs"）；按序排桶 | O | 串；rebuild 缺 last_retired |
| `DisposalBucket::dispose`（line 143） | — | 桶执行 | 执行延迟销毁动作 | I | 骨架 |
| `GraphicsDevice::immediate_empty_bucket`（line 175，0xbf92f0） | — | 立即排空 | 直接排空给定 seq 桶 | I | 骨架 |
| `impl$4..13::dispose_to_bucket`（line 292-346，10 实例） | — | managed 族入桶 | 每种 managed 资源的入桶模板 | I | 骨架 |
| `impl$14/15/21::default`、`impl$17::fmt`、`impl$19::eq` | — | 派生 | default/fmt/eq | I | 骨架 |

### 2.8 raw/error.rs / bytes.rs / workarounds.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `error::impl$1::fmt`（line 4） | — | BackendError Debug | debug_struct（51 实例） | I | 骨架 |
| `error::impl$3::fmt`（line 4） | — | BackendError Display | Formatter::write_fmt | I | 骨架 |
| `error::impl$0::from`（line 21） | — | From<vk::Result> | 内联于 create_framebuffer/wait_submit_done/queue_submit | I | 骨架 |
| `bytes::slice_as_bytes`（line 8） | — | 重解释为字节 | 123 实例 | I | 骨架 |
| `bytes::bytes_as_slice`（line 22） | — | 字节重解释 | 128 实例 | I | 骨架 |
| `workarounds::_::impl$0::deserialize/serialize`（line 1） | — | GraphicsWorkarounds serde | Ron serde visitor（与 init allowlist 同构） | I | 骨架 |

### 2.9 raw/ray_tracing.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$0::create_ray_tracing_bottom_acceleration::closure$0/1/2`（line 99/128/143） | — | BLAS 构建闭包 | 仅 inlinee_declaration；无地址/反编译 | U | 骨架 |
| `impl$0::create_ray_tracing_top_acceleration::closure$0`（line 171） | — | TLAS 闭包 | 同上 | U | 骨架 |
| `GraphicsDevice::create_ray_tracing_acceleration`（line 250） | — | 加速结构创建 | 同上 | U | 骨架 |
| `GeometryInstance::new`（line 881） | — | 几何实例构造 | 同上 | U | 骨架 |
| `GeometryInstance::set_mask`（line 907） | — | 掩码 | 同上 | U | 骨架 |

> 说明：PDB 类型图给出 `RayTracingAcceleration`（Arc drop_slow 引用）。rebuild **无 ray_tracing 模块**，MODULE_STATUS "ray tracing完整路径未验收" 确认。

### 2.10 leek.rs / lib.rs

| 骨架名 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `leek::impl$0::fmt`（line 63，0xbe2710） | — | leek 错误 Debug | String::new/push×2/str Debug（错误上下文字符串） | I | 骨架 |
| `leek::impl$1::from`（line 81，0xbe2880） | — | 错误转换 | utf8 校验（is_ascii/next_code_point/utf8_acc_cont_byte）；由 record_batch 并行闭包与 dispose_cb_resource 调用 | O（调用点）/ I（语义） | 骨架 enrichment（0xf86a8/0x2c3405） |
| `leek::impl$2::fmt`（line 103，0xbe2990） | — | Display | Formatter::write_fmt | I | 骨架 |
| `rhapsody::init_logging` | 0x140be6a90 | 日志初始化 | 建 0x10000(64KB) 环；写全局 0x143634e40；**断言 prev.is_none()**（0x142b12f50），重复调用 panic | O | decomp；串 0x142b12f50 |
| `rhapsody::print_log_dump` | 0x140be6b40 | 打印日志 | jmp 0x140be2b40（ICF thunk） | O | rne-read 原始字节 |
| `rhapsody::dump_log` | 0x140be6b50 | 输出日志 | 全局!=0 → FUN_1418c1430(输出, 0x143634e40)；否则 panic 0x142b12f70 | O | decomp；调用方 panic_reporter::panic_dump::dump_rhapsody_log |
| `lib.rs::GraphicsDevice::create_buffer<Option<&[u8]>>`（0xc03460） | — | device.rs 归属 | 属 raw-core 域（populate_resources/GraphExecutor::new 调用） | — | 骨架；不展开 |
| `lib.rs::GraphicsDevice::create_texture<TextureInitialData>`（0xc07950） | — | 同上 | 属 raw-core 域 | — | 骨架 |
| `lib.rs::GraphicsDevice::update_buffer_bytes`（0xbeefa0） | — | 同上 | 属 raw-core 域（36 调用点） | — | 骨架 |
| `core::str::pattern::TwoWaySearcher::next_back` | — | std | 标准库，忽略 | — | 骨架 |

---

## 3. 原版 ↔ rebuild 对照表

### 3.1 init / 设备筛选

| 原版函数/行为 | rebuild 对应 | 判定 | 差异说明 |
|---|---|---|---|
| `GraphicsInstanceLoader::new`（vulkan-1.dll→fallback-vulkan-1.dll→系统 loader，attempts 记录） | `GraphicsInstanceLoader::from_candidates(&[])→Entry::load()` | ✓ | 结构等价；rebuild 无 "fallback-vulkan-1.dll" 显式候选（系统 loader 兜底）；`implicit_layers` rebuild 返回 `None`（原枚举层但 0x208B 截断，语义待核） |
| `GraphicsInstanceLayer::vendor`（VK_LAYER_ 前缀+首下划线） | 同 | ✓ | 逐字节逻辑一致 |
| `GraphicsInstanceLayerAllowlist::{contains_layer,contains_vendor}` | 同 | ✓ | 条目 tag 0/1 + String 布局一致；**PDB 名 `AllowlistItem` vs rebuild `AllowlistEntry`**（命名差） |
| Ron allowlist/migration | `update_from_ron` | ✓ | 原 asset `vulkan_allowlist_defaults.ron`（VALVE/OBS→VK_LAYER_reshade）解析通过（device-policy-validation） |
| `AvailableGraphicsDevice::driver_support` | 同 | ✓ | 24 输入 0 mismatch（含 RTX50 KnownIssue tag4）；rebuild 加 `encoding_known`/`original_tag` 候选 |
| `is_emulated`（Direct3D12/Basic Render Driver） | 同 | ✓ | 双串 SIMD memmem，逐字一致 |
| `is_old_intel`（11 串表） | 同 | ✓ | 逐字一致 |
| `is_compatible`（无 old_intel）/`is_supported`（含 !old_intel） | 同 | ✓ | 内存门槛 1.0GB/400MB/1.4GB 一致 |
| `has_enough_video_memory`（+0x88/+0x90/+0x99） | `is_compatible` 内存段 | ✓ | 布局偏移一致 |
| `order_and_filter_devices`（0xa0B 排序） | `order_and_filter_devices`（sort_by_key 候选） | 半 | **原排序比较器（closure$0/$1）未恢复**；rebuild 用"支持/兼容优先+类型+budget/heap"候选排序 |
| `available_devices`（物化 0xa0B Vec） | `impl Iterator<Item=&AvailableGraphicsDevice>` | 半 | 原返回物化 Vec（复制设备），rebuild 返回借用迭代器（接口差） |
| `missing_devices`（0x28B 条目，含 vendor 匹配） | `&[(usize, Vec<String>)]` | 半 | 原物化 Owned Vec 且含 vendor id 0x1002/0x10de 逻辑；rebuild 简单借用 |
| `create_device`（匹配 +0x690/+0x698，Arc clone，逐候选 fallback） | `create_device` | 半 | 原匹配字段未对齐到 rebuild 的 index 匹配；rebuild 是候选逐设备 fallback |
| `GraphicsDevicesForSurface::new<Arc<Window>>` | `available_for_surface` | 半 | 原泛型 Arc<Window>×2；rebuild 以 Option<vk::SurfaceKHR> |

### 3.2 instance / 层屏蔽

| 原版 | rebuild | 判定 | 差异 |
|---|---|---|---|
| `InstanceBuilder::build`（默认扩展名常量） | `build_with_loader` | ✓/半 | 原默认启用扩展常量（0x142b1909b/0x142b190c2，内容 U）；rebuild 扩展空列表由调用方给 |
| `Instance` 对象 0x118B | `VulkanInstance{entry,instance,enabled_extensions,layers}` | ✓ | 布局候选一致（rebuild Drop 显式 destroy） |
| `apply_vulkan_blocklist`：**11 个环境变量/层名屏蔽链** | `apply_vulkan_blocklist(layers, blocked)` 通用过滤 | ✗ | rebuild modern 传 `&[]`（空表）→ **不屏蔽任何层**；原按 DISABLE_RTSS/VKBASALT/GAMEPP/SAMPLE_LAYER/NV_OPTIMUS/AMD_SWITCHABLE/VK_LOADER_LAYERS_*/bandicam/NODEVICE_SELECT 过滤 |
| `apply_vulkan_modern/legacy/doubletap_blocklist` | 同 3 个函数（legacy 含 LUNARG_standard_validation，doubletap 去重） | ✗/半 | 原三套为环境变量驱动的同一链（inline）；rebuild 的 legacy/doubletap 是候选条目，非原行为 |
| `legacy_disable_layer` | 同 | ✓ | retain 移除 |
| `vulkan_debug_messenger` | 同（no-op 回调） | 半 | 原回调行为未恢复（closure$0/$1） |

### 3.3 physical_device / 预算

| 原版 | rebuild | 判定 | 差异 |
|---|---|---|---|
| `PhysicalDevice::name` | 同 | ✓ | CStr→String |
| `device_local_memory_bytes`（DEVICE_LOCAL 堆和） | 同 | ✓ | mask 0x1 筛堆 |
| `device_local/host_memory_budget_bytes`（ArcSwap load+按堆求和） | `snapshot_or_refresh→budget` | ✓ | +0x608 ArcSwap 位置对应 rebuild `budget_snapshot` |
| `refresh_memory_budget`（**滞回钳位 [旧±旧/8]**，0x110B 快照） | `refresh_memory_budget`（直查直存） | 半 | **rebuild 缺滞回**；快照字段布局为候选 |
| `performance_info`（无扩展→vendor_specific="Unsupported"；6 堆查询物化 0x18B 记录） | `performance_info→memory_snapshot` | 半 | 原 6 次独立查询（budget/usage 各堆）+名字串；rebuild 一次 memory_properties2；字段语义待对齐 |
| `DeviceDriverVersion::{vendor_string,is_amd,is_apple}` | 同 | ✓ | tag 0..6 编码一致（is_amd==3/is_apple==5 原始字节证实） |
| `enumerate_physical_devices`/`enumerate_vk_data` | 同 | ✓ | 两次调用模式一致 |
| `find_vendor_specific_device_handle`（仅 0x10de 分支+清单匹配） | 名字子串匹配 | 半 | **原仅 NVIDIA**；rebuild 通用厂商子串 |
| `string_from_c_array`/`parse_driver_version`/`with_presentation_support` | 同 | ✓ | 布局一致 |
| PDB 类型 `QueueFamily`、`DeviceDriverInfo` | rebuild `queues: Vec<vk::QueueFamilyProperties>` | 半 | 原为自定义 QueueFamily 类型；rebuild 用原始 vk 属性 |

### 3.4 format

| 原版 | rebuild | 判定 | 差异 |
|---|---|---|---|
| `vk_format_valid_aspect_flags`（<184 表查，否则 panic） | `observed::try_*`（index>=184 → UnsupportedAspect） | ✓ | 816 对比 0 mismatch |
| `vk_format_info`（<147 写 10B，byte9=byte8） | `observed::try_*`（index>=147 → UnsupportedInfo；bytes[9]=bytes[8]） | ✓ | 10B 布局逐案例一致 |
| `vk_format_block_bytes`（<130→1；0xccc0→16；0x3/idx8→8） | `observed::try_*` 同逻辑 | ✓ | 位掩码分支一致 |
| 原私有（格式号区间/panic）与 Khronos 通用注册表 | `mod observed` 与 `mod khronos` 分离 | ✓ | rebuild 提供 249 格式公共注册表为额外接口 |

### 3.5 allocator

| 原版 | rebuild | 判定 | 差异 |
|---|---|---|---|
| `GpuAllocator::new(backend, track_peaks:bool)`（+0x688） | `GpuAllocator::new(backend, track_peaks)` | ✓ | 16 案例 0 mismatch |
| `GpuAllocator::allocate`（flag 置位才采样 report；total 严格增才换 peak_report；device/system 独立 max） | `allocate→observe_report` | ✓ | 语义一致（含 u128 峰值比较） |
| `GpuAllocator::free`（不采样） | `free`（仅 backend.free） | ✓ | free_preserves_peaks 2 案例 |
| peak getters（+0x670/678/680/620） | `peaks.*` / `peak_allocator_report` | ✓ | 原始字节证实偏移 |
| `inner` | `inner/inner_mut` | ✓ | 返回 self |
| 底层 gpu-allocator 0.26 本地分叉（ash0.38+arc-swap/egui/puffin） | gpu-allocator 0.27 公开版（子分配候选，默认关） | 半 | **块大小策略/统计口径可能与分叉不同**（MODULE_STATUS 已登记）；`rename_allocation` rebuild 用 `allocator_stats.rename` 候选 |

### 3.6 crash

| 原版 | rebuild | 判定 | 差异 |
|---|---|---|---|
| debug 记录 {kind<<24\|line, src_id, 2 operands} 格式化（kind 1..6 消息串） | `parse_gpu_debug_info` | ✓ | 消息串逐案例一致（crash-validation） |
| `has_any_gpu_assertion_failed`（word[1]!=0） | 同 | ✓ | native_assertion_cases 6 案例一致 |
| 错误门控 TIMEOUT/DEVICE_LOST | `needs_vulkan_error_info` | ✓ | native_error_classification 6 案例一致 |
| marker（id 分配、slot=id&4095、cmdFillBuffer 写 id） | `record_crash_marker` | ✓ | native_marker_entries 3 案例一致 |
| `log_gpu_crash_info`/`log_vulkan_error_info`/`log_error_info` | `GpuCrashDiagnostics` 方法 | ✓/半 | **原 shader 诊断 producer、vendor dump、真实 device-lost 未验收**（crash-validation limits） |
| source 名静态表（CrashMarkerNames） | `CrashMarkerNames{insert_name,get_name}` | ✓ | 结构等价（原 crash.rs 静态表） |

### 3.7 disposal

| 原版 | rebuild | 判定 | 差异 |
|---|---|---|---|
| `CommandBufferDisposalBuckets` = **hashbrown::HashMap<CommandBufferSeq, Mutex<parking_lot,DisposalBucket>>** | `BTreeMap<u64, Vec<DestroyAction>>`（无锁） | ✗ | **容器类型、键类型、每桶互斥全不同**；并行录制场景原靠每桶 Mutex 串行化 |
| `insert`（prev.is_none() 断言，重复 seq panic） | `insert`（entry().or_default().push，允许重复） | ✗ | 原重复 seq 是错误；rebuild 容忍 |
| `remove`（缺桶 panic "bucket removed prematurely…"） | `remove`（返回 Option，不 panic） | ✗ | 原缺桶是错误路径（panic） |
| `DisposalBucket`（Mutex 内动作 Vec，"capacity overflow" 串） | `Vec<DestroyAction>` | 半 | 原桶类型独立结构；rebuild 合并 |
| `SequentialDisposal::dispose_in_order`（断言 seq>last_retired；带 reclaim closure） | `pub type SequentialDisposal = CommandBufferDisposalBuckets` | ✗ | **rebuild 无 last_retired 单调断言**；原按序列号单调排桶，rebuild `dispose_in_order` 直接排空 ≤completed |
| `dispose_cb_resource<Managed*>`（6 族） | `dispose_cb_resource<T: Send>` | ✓ | 泛型等价 |
| `immediate_empty_bucket` | `immediate_empty_bucket` | ✓ | 立即排空 |
| 销毁桶→managed 退役闭环（throw_out_managed_resource closure） | `raw/graveyard.rs`（epoch 销毁） | 半 | **rebuild 用 graveyard epoch 替代原按序列号桶**（MODULE_STATUS "raw析构保守等待" 已登记） |

### 3.8 error / bytes / workarounds / ray_tracing / leek / lib

| 原版 | rebuild | 判定 | 差异 |
|---|---|---|---|
| `error::BackendError` Debug/Display/From | `raw::error::Error{Vulkan,Loader,Unsupported,Invalid}` | 半 | 原枚举变体未恢复（PDB 名 BackendError，serde/anyhow 传播）；rebuild 候选 4 变体 |
| `bytes::{slice_as_bytes,bytes_as_slice}` | 同（+BufferBytes trait） | ✓ | 语义一致 |
| `workarounds::GraphicsWorkarounds` serde | 无（allowlist 内） | 半 | 原为独立 Ron 结构；rebuild 并入 allowlist |
| `ray_tracing.rs`（BLAS/TLAS/GeometryInstance，全 inlinee） | **无** | ✗ | 原 PDB 声明存在但零地址；rebuild 无模块；"ray tracing完整路径未验收" |
| `leek::impl$0::fmt/impl$1::from/impl$2::fmt` | `leek::Error{context,source}+Display+wait_submits_done` | 半 | 原 impl$1::from 带 utf8 校验（record_batch/disposal 调用）；rebuild Error 结构为候选；wait_submits_done 原归属 device.rs |
| `init_logging`（64KB 环，断言 once） | **无** | ✗ | rebuild lib.rs 仅模块声明；**缺全局日志初始化** |
| `dump_log` / `print_log_dump` | **无** | ✗ | 原被 panic_reporter::dump_rhapsody_log 调用（崩溃转储链）；rebuild 缺 |
| `lib.rs` GraphicsDevice::create_buffer/texture/update_buffer_bytes | raw-core 域已实现 | — | 属 device.rs，raw-core 域覆盖，不在此域重复 |

---

## 4. U 卡点清单

1. **原设备排序比较器**：`order_and_filter_devices` closure$0/closure$1（line 838/857）——比较维度/权重未恢复。卡点：反编译只见排序骨架（0xa0B 插入排序/introsort），比较器内联不可见；需要逐对设备的行为注入或对原二进制构造多设备场景。
2. **原 implicit 层来源**：`implicit_layers` 返回 0x208B 枚举条目的截断（GraphicsInstanceLayer 20B vs 枚举 0x208B）；原是否为显式 implicit 清单（`~implicit~` 前缀层）未定。rebuild 返回 None。
3. **apply_vulkan_blocklist 的过滤粒度**：env 值"1"的判定确证（0x142b196f8），但每个 env 分支是对**单个层名**还是**整组 implicit** 过滤未逐分支确认；`~implicit~` 在 VK_LOADER_LAYERS_DISABLE 中的语义细节（路径 vs 值）未定。
4. **`refresh_memory_budget` 滞回钳位的语义**：`[旧-旧/8, 旧+旧/8]` 的确认来自 16 堆展开的 min/max 链，但"新值"从何读取（0x110B 快照内 16 budget + 16 usage 的排布）仍 I；快照字段名与 rebuild `PerformanceInfo` 未逐字段对齐。
5. **`performance_info` 的 6 次堆查询**：FUN_1418b1a40/b1bb0/b1cd0/b1e40/b18f0/b1fb0 各自查询类型（budget/usage/名称）未解析；物化 0x18B 记录字段（{name ptr,len,value}）是 I。
6. **`missing_devices`/`create_device` 的字段偏移语义**：missing 条目 0x28B 含 vendor id 0x1002/0x10de 的匹配逻辑、create_device 匹配 +0x690/+0x698 两 qword 的确切含义（物理句柄+队列族？）未定。
7. **`find_vendor_specific_device_handle` 清单来源**：FUN_1418b1340 枚举内容未定（设备名列表？）；原仅 NVIDIA(0x10de) 分支，rebuild 通用子串是候选差异。
8. **`GpuAllocator::free` 内部**：FUN_140c16a00/140c0d550/140c16d40/140c14680 的路径解析/断言语义（含 panic 串 0x142d06498/0x142d06508）未解；free 的全局计数器结构未定。
9. **disposal 的锁拓扑**：外层是否有整表 Mutex（remove/insert 首 qword 0→8 状态）+ 每桶 Mutex（PDB 类型图）的关系未定；`CommandBufferSeq` 类型定义（command_buffer.rs）未在本域恢复。
10. **`leek::impl$1::from` 语义**：utf8 校验确认（is_ascii/next_code_point 内联），但转换目标（错误上下文字符串？）未定；rebuild Error 是候选。
11. **`init_logging` 全局缓冲布局**：0x143634e40 全局的 {ptr,len,cap} 或环形 {head,tail} 布局未定（FUN_1418c1310/1418c1430 内部）；`dump_log` 输出目标参数类型未定。
12. **ray_tracing 全部**：无地址、无反编译、rebuild 无实现。BLAS/TLAS 构建、GeometryInstance{new,set_mask}、create_ray_tracing_acceleration 全部 U。
13. **格式 word0/word1/byte8 字段语义**：表值经 816 案例验证但字段含义（通道数/存储字节/平面）是 I（rebuild observed 注明"未把这些推断当完整原字段名"）。
14. **`Instance::create`/`InstanceBuilder::default`/`vulkan_debug_messenger` 回调**：均为 inlinee 或 va null，行为 I。
15. **lib.rs `create_buffer/create_texture/update_buffer_bytes`**：PDB 归属 lib.rs CU 但实为 device.rs（0xc03460/0xc07950/0xbeefa0），raw-core 域覆盖；本域不重复取证。
