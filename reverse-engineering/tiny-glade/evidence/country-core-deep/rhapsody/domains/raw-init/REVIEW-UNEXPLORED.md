# rhapsody raw-init 域深挖审查：未补全 / 未探究清单

- 日期：2026-10-15。审查对象：`domains/raw-init/ALGORITHMS.md`（396 行）。
- 核对基准：`domains/raw-init.json`（183 条目 / **87 compiled / 47 有地址**）+ `decomp/`（46 文件，10 个 NotFound 已 rne-read 手解）+ `CALL-CHAIN.md` + `PLAYBOOK.md` + `reconstruction/rhapsody-rebuild/src/raw/{init,instance,physical_device,format,allocator,crash,disposal,error,bytes,setup,suballoc}.rs` + `src/{leek.rs,lib.rs}` + `MODULE_STATUS.md`。
- 审查纪律同 PLAYBOOK：O=机器码/PDB 实测、I=推断、U=未知；不改任何 reconstruction 源码、不改 ALGORITHMS.md。
- 本次为补查核实（对本域反编译/字符串/rebuild 逐项抽验），抽查结论基于：rne-read 字符串直读、`decomp/decomp-0x140bdbc30.txt`、`decomp-0x1418c4100.txt`、`decomp-0x1418c4260.txt`、`decomp-0x140c01620.txt`、`decomp-0x140c0a530.txt`、`decomp-0x140be6a90.txt`、`decomp-0x140bda7b0.txt`、`decomp-0x140bda840/0x140bda8f0.txt`、`decomp-0x140c00a80.txt` 等。

---

## 0. 特别核对项（用户点名的四项）

| 核对项 | 结果 | 说明 |
|---|---|---|
| **47 个有地址函数是否全部进 ALGORITHMS.md** | ✓ | 47 条目全部有分析行（46 个唯一 VA + `driver_support` 在 init.rs/lib.rs 双归属重复）；46 个唯一 VA 全部有 `decomp/` 文件。其中 **10 个为 NotFound**（文档写成 **9**，见 §4 事实偏差 #1），全部 rne-read 原字节补证为 O |
| **ray_tracing.rs 全部 inlinee_declaration 是否单独列明** | ✓ | 7 个全部在 §2.9 单独成行：`create_ray_tracing_bottom_acceleration::closure$0/1/2`（line 99/128/143）、`create_ray_tracing_top_acceleration::closure$0`（171）、`GraphicsDevice::create_ray_tracing_acceleration`（250）、`GeometryInstance::{new,set_mask}`（881/907）。另有 1 个非 ray_tracing 的 inlinee_declaration（`instance::impl$2::default`）在 §2.2 有行 |
| **lib.rs 顶层 compiled 是否覆盖** | ✓（数量前提纠偏） | 骨架实际 **9 个** lib.rs compiled（任务提示写 12；9 = TwoWaySearcher + create_buffer + create_texture + update_buffer_bytes + driver_support 重复 + enumerate_vk_data 重复 + init_logging + print_log_dump + dump_log），全部有行；create_buffer/texture/update_buffer_bytes 属 raw-core 域不展开（§2.10 已注明） |
| **compiled 逐函数覆盖** | △ | 87 个 compiled 中 **1 个完全无行**：`rhapsody::raw::init::impl$26::fmt`（init.rs line 425，无地址）。其余名义全有行，但约 17 个为表面行（见 §2） |

---

## 1. 未覆盖函数（87 compiled 逐函数核对表）

标注：**✓**=有分析行（含组合行）；**△**=有行但表面（role/算法空泛、verdict 未落到 I/U 以下、无新增取证）；**✗**=无对应行。

### 1.1 src/raw/init.rs（22 compiled）

| 骨架名 | 地址 | 行号 | 判定 |
|---|---|---|---|
| `GraphicsInstanceLoader::new` | 0x140bd8af0 | — | △ 有行（O，30KB 反编译仅摘要；隐式层/loader fallback 细节未深挖，见 §3.6） |
| `impl$10::fmt` | — | 35 | ✓ I（derive） |
| `impl$14::from` | — | 35 | △ 仅 "From 转换" U，无目标类型/调用点 |
| `GraphicsInstanceLayer::vendor` | 0x140bd7bc0 | — | ✓ O（与 rebuild 逐字节等价） |
| `_::impl$0::serialize<ron>` | — | 93 | ✓ I（derive） |
| `GraphicsInstanceLayerAllowlist::contains_layer` | 0x140bd7cf0 | — | ✓ O |
| `GraphicsInstanceLayerAllowlist::contains_vendor` | 0x140bd7d60 | — | ✓ O |
| `GraphicsInstance::new` | 0x140bd7dd0 | — | ✓ O（13KB 反编译有行，但枚举→build 内细节未展开） |
| `GraphicsInstanceLoader::implicit_layers` | 0x140bda7b0 | — | △ 有行（O 枚举+panic），但返回类型/是否过滤 implicit 未定（decomp 已示返回原始 0x208B 切片，见 §3.7） |
| **`impl$26::fmt`** | — | 425 | **✗ 无行**。位于 `implicit_layers`(388) 与 `AvailableGraphicsDeviceInitState::is_error`(438) 之间的 fmt impl；无地址、无反编译；类型归属（GraphicsInstanceLoader/LocationToTry 相关？）未定 |
| `AvailableGraphicsDeviceInitState::is_error` | 0x140a9fa70 | — | ✓ O（rne-read 原始字节） |
| `impl$28::fmt` | — | 442 | ✓ I（与 impl$30::eq/32::clone/33::fmt 组合行） |
| `AvailableGraphicsDevice::is_supported` | 0x140bda840 | — | ✓ O（复核：decomp 尾 `old_intel^1` 一致） |
| `AvailableGraphicsDevice::is_compatible` | 0x140bda8f0 | — | ✓ O（复核：无 old_intel 调用，门槛 1.0GB/400MB/1.4GB、偏移 +0x88/+0x90/+0x99 一致） |
| `AvailableGraphicsDevice::driver_support` | 0x140bda990 | — | ✓ O（24 输入 0 mismatch；RTX50 "580.97/580.95"） |
| `AvailableGraphicsDevice::is_emulated` | 0x140bdaec0 | — | △ 有行（O 双串 SIMD memmem）；但该地址反编译 32774B（SIMD 展开伪影），doc 未注 "逐字一致" 的串来源细节 |
| `AvailableGraphicsDevice::is_old_intel` | 0x140bdaf20 | — | ✓ O（11 串表逐字对上） |
| `GraphicsDevicesForSurface::new<Arc<Window>>` | — | 693 | △ 仅 "遍历设备→surface present 查询" I/U；surface 查询循环/错误处理未解（无地址，但 `with_presentation_support`/`enumerate` 相关调用点可追） |
| `GraphicsDevicesForSurface::missing_devices` | 0x140bdb090 | — | △ 有行（O/I）；missing 条目 0x28B、vendor id 0x1002/0x10de 匹配逻辑仍 U（§3.10） |
| `GraphicsDevicesForSurface::available_devices` | 0x140bdb820 | — | ✓ O（返回物化 0xa0B Vec，非迭代器） |
| `GraphicsDevicesForSurface::create_device` | 0x140bdb940 | — | △ 有行（O/I）；+0x690/+0x698 两 qword 匹配语义未定（§3.10） |
| `GraphicsDevicesForSurface::order_and_filter_devices` | 0x140bdbc30 | — | △ 有行（O 排序存在/U 比较器）；`closure$0/$1`（line 838/857）未恢复（§3.1）。复核 decomp：0xa0B 元素、len<0x15 插入排序/≥0x15 introsort、随后 FUN_1418b5460 过滤，与 doc 一致 |

### 1.2 src/raw/instance.rs（3 compiled）

| 骨架名 | 地址 | 判定 |
|---|---|---|
| `InstanceBuilder::build` | 0x140c08f10 | ✓ O |
| `apply_vulkan_blocklist` | 0x140c0a530 | △ 有行（O env 链），但 **env 枚举不完整**（漏 8 个变量名，见 §3.2）；"值含 1 触发"仅对部分分支逐字证实 |
| `vulkan_debug_messenger` | —（va null） | △ 仅 "VK_EXT_debug_utils" I；回调 `closure$0/$1`（line 349/352）未分析 |

### 1.3 src/raw/physical_device.rs（19 compiled）

| 骨架名 | 地址 | 行号 | 判定 |
|---|---|---|---|
| `impl$1/3/13/16/18/21::fmt` | — | 0/24/36/42/73 | ✓ I（组合行） |
| `DeviceDriverVersion::vendor_string` | 0x140c001c0 | — | ✓ O（rne-read） |
| `DeviceDriverVersion::is_amd` | 0x140c001e0 | — | ✓ O（rne-read，tag==3） |
| `DeviceDriverVersion::is_apple` | 0x140c001f0 | — | ✓ O（rne-read，tag==5） |
| `PhysicalDevice::name` | 0x140c00400 | — | ✓ O |
| `PhysicalDevice::device_local_memory_bytes` | 0x140c00530 | — | ✓ O |
| `PhysicalDevice::device_local_memory_budget_bytes` | 0x140c006a0 | — | ✓ O |
| `PhysicalDevice::host_memory_budget_bytes` | 0x140c00890 | — | ✓ O |
| `PhysicalDevice::performance_info` | 0x140c00a80 | — | △ 有行（O/I）；6 次堆查询（FUN_1418b1a40 族）各自类型未解析、0x18B 记录字段仍 I（decomp 已示 3 qword {name&str, value}，见 §3.9） |
| `PhysicalDevice::refresh_memory_budget` | 0x140c01620 | — | △ 有行（O 滞回钳位存在）；钳位除数实为 **max(1, 旧/8)** 未注（见 §3.4）；新值=驱动查询结果、快照 0x110B 字段排布仍 I |
| `impl$5::fmt` | — | 300 | ✓ I |
| `enumerate_vk_data<u32,PhysicalDevice>` | —（skeleton lib.rs 引用 0xc01e11） | 305 | ✓ O（两次调用模式） |
| `enumerate_physical_devices` | 0x140c01e00 | — | ✓ O |
| `find_vendor_specific_device_handle` | 0x140c01e80 | — | △ 有行（O/I）；FUN_1418b1340 清单来源未定、仅 0x10de 分支（§3.11） |
| `impl$6::with_presentation_support` | —（va null） | 540 | △ 仅 "逐队列 get_physical_device_surface_support" I |
| `enumerate_physical_devices::closure$0..4` | — | 336–473 | △ 组合行仅 "内联（失败处理/过滤）" U |

### 1.4 src/raw/format.rs（3 compiled）

| 骨架名 | 地址 | 判定 |
|---|---|---|
| `vk_format_valid_aspect_flags` | 0x141738fe0 | ✓ O（816 案例 0 mismatch） |
| `vk_format_info` | 0x141739060 | ✓ O |
| `vk_format_block_bytes` | 0x1417390e0 | ✓ O |

（本文件三入口对照已闭合，唯一遗留为 word0/word1/byte8 字段语义 I，doc 已列 U 卡点 #13。）

### 1.5 src/raw/allocator.rs（8 compiled）

| 骨架名 | 地址 | 判定 |
|---|---|---|
| `GpuAllocator::new` | 0x140c0d980 | ✓ O |
| `GpuAllocator::allocate` | 0x140c0da40 | ✓ O（16 案例 0 mismatch） |
| `GpuAllocator::free` | 0x140c0e150 | △ 有行（O 不采样峰值）；内部 FUN_140c16a00/0d550/16d40/14680 路径与全局计数器未解（§3.5） |
| `GpuAllocator::rename_allocation`（inline） | — | 87 | △ 仅 "改名" I；rebuild 用 `allocator_stats.rename` 候选 |
| `GpuAllocator::inner` | 0x14067dd80 | ✓ O（rne-read） |
| `peak_total_allocated_bytes` | 0x140c0e600 | ✓ O（rne-read） |
| `peak_allocator_report` | 0x140c0e610 | ✓ O（rne-read） |
| `peak_device_local_bytes` | 0x140c0e630 | ✓ O（rne-read） |
| `peak_system_bytes` | 0x140c0e640 | ✓ O（rne-read） |

### 1.6 src/raw/crash.rs（6 compiled）

| 骨架名 | 地址 | 判定 |
|---|---|---|
| `log_gpu_debug_info` | 0x1418c46c0 | ✓ O（kind 1..6 消息串；`log_gpu_debug_info::closure$0`(line 162) 未单独列） |
| `GraphicsDevice::log_gpu_crash_info` | — | △ I（对照 crash-validation）；shader 诊断 producer/vendor dump 未验收（doc §3.6 已记半） |
| `CrashMarkerNames::{insert_name,get_name}`（inline） | — | 12/23 | △ 仅 "双向表" I |
| `GraphicsDevice::record_crash_marker` | — | ✓ O（crash-validation native_marker_entries） |
| `GraphicsDevice::log_vulkan_error_info` | — | ✓ O（native_error_classification） |
| `GraphicsDevice::log_error_info` | — | 128 | △ 仅 "候选包装" I |

### 1.7 src/raw/disposal.rs（11 compiled）

| 骨架名 | 地址 | 判定 |
|---|---|---|
| `SequentialDisposal::dispose_in_order<reclaim closure>` | — | △ 有行（O 断言串证据，**无反编译**）；`seq > self.last_retired` 单调语义仅靠串（断言串地址归属见 §4 偏差 #4）；rebuild 缺 last_retired |
| `CommandBufferDisposalBuckets::remove` | 0x1418c4100 | ✓ O（复核：外层 parking_lot 锁 0→8 + 1s 超时 + hashbrown remove + 缺桶 panic，与 doc 一致） |
| `CommandBufferDisposalBuckets::insert` | 0x1418c4260 | ✓ O（复核：同锁 + hashbrown insert + prev.is_none() panic，与 doc 一致） |
| `dispose_cb_resource<Managed*>` ×**7** | — | △ 有行但 **数量错：doc 写 "6 实例/6 族"，实际 7**（Buffer/PersistentDescriptorSet/Framebuffer/RenderPass/Sampler/Swapchain/Texture，见 §4 偏差 #2）；语义仅 "闭包 drop(managed)" I |
| `GraphicsDevice::immediate_empty_bucket` | —（0xbf92f0） | △ 仅 "直接排空给定 seq 桶" I；managed 域审查已证 S_PUB32 `0x140bf92f0=GraphicsDevice::immediate_empty_bucket`、`0x140bdfcf0=drop_in_place<DisposalBucket>`，本域未采纳 |
| `impl$4..13::dispose_to_bucket` ×10 | — | 292–346 | △ 仅 "每种 managed 资源的入桶模板" I |
| `impl$14/15/21::default`、`impl$17::fmt`、`impl$19::eq` | — | I（组合行） |

### 1.8 src/raw/{error,bytes,workarounds}.rs

| 骨架名 | 判定 |
|---|---|
| `error::impl$1::fmt` / `impl$3::fmt`（line 4） | ✓ I（BackendError Debug/Display） |
| `error::impl$0::from`（line 21，inline） | ✓ I（From<vk::Result>） |
| `bytes::{slice_as_bytes,bytes_as_slice}`（inline） | ✓ I |
| `workarounds::_::impl$0::serialize<ron>` | ✓ I（与 init allowlist 同构） |

### 1.9 src/leek.rs（3 compiled）

| 骨架名 | 判定 |
|---|---|
| `leek::impl$0::fmt`（line 63） | △ I（String::new/push×2，转换目标未定） |
| `leek::impl$1::from`（line 81） | △ O（调用点）/I（语义）；utf8 校验确认但**转换目标（错误上下文字符串？）未定**；rebuild `Error` 无 From 实现 |
| `leek::impl$2::fmt`（line 103） | ✓ I |

### 1.10 src/lib.rs（9 compiled）

| 骨架名 | 判定 |
|---|---|
| `core::str::pattern::TwoWaySearcher::next_back` | ✓（std 忽略） |
| `GraphicsDevice::{create_buffer,create_texture,update_buffer_bytes}` | ✓（raw-core 域，不展开） |
| `AvailableGraphicsDevice::driver_support`（重复归属） | ✓（同 §1.1） |
| `enumerate_vk_data`（重复归属） | ✓ |
| `rhapsody::init_logging` | ✓ O（复核：FUN_1418c1310(&buf,0x10000) 建 64KB 环、写全局 0x143634e40、断言 `assertion failed: prev.is_none()`@0x142b12f50 命中） |
| `rhapsody::print_log_dump` | ✓ O（rne-read ICF thunk） |
| `rhapsody::dump_log` | ✓ O |

**小结：✗=1（`init::impl$26::fmt`）；△≈17；其余 ✓。名义覆盖高，但深度不足集中在 §2/§3。**

---

## 2. 表面分析清单（△：有行但算法/证据空泛）

| # | 函数 | 现有行 | 卡点 | 建议补法 |
|---|---|---|---|---|
| 1 | `init::impl$26::fmt`（425，无地址） | **无行** | 类型归属不明 | 用 skeleton line 425 邻近上下文推断类型（implicit_layers 388 / is_error 438 之间）；对 B 二进制扫该区域小 fmt（debug/display 模式）或查 PDB 类型图 |
| 2 | `init::impl$14::from`（35） | "From 转换" U | 目标类型/源类型未定 | PDB 类型图看 impl$14 的 self/target；检查 line 35 附近类型 |
| 3 | `GraphicsDevicesForSurface::new<Arc<Window>>`（693） | I/U 一句 | surface present 循环、错误处理未解 | 追 `enumerate_physical_devices` 后的 surface 查询链（`get_physical_device_surface_support` 调用点）；B 二进制按 693 行号邻近符号定位 |
| 4 | `order_and_filter_devices::closure$0/$1`（838/857） | U | 比较维度/权重未恢复 | 对 `order_and_filter_devices` 反编译做行为注入：构造多设备场景（不同 driver 支持/内存/类型），观察排序结果反推比较器；或找排序调用点 FUN_1418b9130/1418bb250 的闭包参数布局 |
| 5 | `instance::vulkan_debug_messenger::closure$0/$1`（349/352） | 无 | 回调行为未分析 | 该函数 va null，但可查 B 二进制 `vulkan_debug_messenger` 名符号是否有真实地址；回调两闭包为消息过滤逻辑 |
| 6 | `apply_vulkan_modern/legacy/doubletap_blocklist` | U 一句 | 三套黑名单 inline 语义 | 见 §3.2（env 表枚举完整化后，逐分支确认过滤粒度） |
| 7 | `enumerate_physical_devices::closure$0..4`（336–473） | U | 失败处理/过滤细节 | 追 0x140c01e00 反编译的闭包内联段（该函数 535B，全量读即可） |
| 8 | `physical_device::with_presentation_support`（540） | I 一句 | 逐队列查询细节 | 追 `get_physical_device_surface_support` 调用点与队列遍历顺序 |
| 9 | `parse_driver_version`（581） | "原驱动版本格式" I | 每厂商编码（NVIDIA >>22 / Intel >>14 等）未逐字证 | 该函数 inline；对照 rebuild `DeviceDriverVersion::from_vulkan` 的编码分支，用 rne-read 验证 |
| 10 | `allocator::rename_allocation`（87） | "改名" I | 原实现是否只改内部名 | 该函数 inline（GpuAllocator::allocate 族）；对照 rebuild `allocator_stats.rename` |
| 11 | `allocator::free` 内部 | O 只证不采样 | FUN_140c16a00/0d550/16d40/14680 路径/断言/全局计数器 | 用 Ghidra 解 FUN_140c16d40（free 内部）及 panic 串 0x142d06498/0x142d06508 归属 |
| 12 | `crash::{log_gpu_crash_info,log_error_info}`、`CrashMarkerNames::*` | I | 快照读回/名称表细节 | `log_gpu_crash_info` 有 crash-validation 部分证据；补 shader 诊断 producer 与 vendor dump 路径 |
| 13 | `disposal::{DisposalBucket::dispose, immediate_empty_bucket, dispose_to_bucket×10}` | I | 桶结构 0xf0B、每 managed 族入桶差异 | `immediate_empty_bucket` 0x140bf92f0 有 S_PUB32 名，直接反编译；`dispose_to_bucket` 各族的入桶参数面（sequence 来源） |
| 14 | `SequentialDisposal::dispose_in_order` | 仅串证据 | 无反编译 | 用 S_PUB32 查地址（managed 域已定位相关 drop_in_place<DisposalBucket>）；补反编译 |
| 15 | `workarounds::*` serde | I | Ron item 格式细节 | 找 `vulkan_workarounds.ron` 资产/解析调用点，确认 item 变体序 |
| 16 | `leek::impl$1::from` | O 调用点/I 语义 | 转换目标未定 | 追 record_batch/dispose_cb_resource 两调用点的入参（0xf86a8/0x2c3405），确认 utf8 校验后转成什么 |
| 17 | `format::FormatInfo` word0/word1/byte8 字段语义 | I | 表值验证但字段含义未定 | 用 Khronos format 注册表交叉推字段名（通道数/存储字节/平面），不必 O 也可登记为 I 映射表 |

---

## 3. 业务逻辑缺口（处理链断点与算法未深挖）

### 3.1 设备排序比较器（order_and_filter_devices closure$0/$1）
**现状**：doc §4#1 自认 U。复核 decomp-0x140bdbc30（611B）：0xa0B 元素、len<0x15 插入排序（FUN_1418b9130）/≥0x15 introsort（FUN_1418bb250），比较器在排序 helper 内联不可见；随后 FUN_1418b5460 过滤填 available/missing。
**缺口**：比较器维度/权重是设备选择的最终仲裁，当前 rebuild 的 sort_by_key（支持/兼容优先+类型+budget/heap+index）是候选，未与原文对齐。
**建议补法**：行为注入——对原函数构造 N≥2 设备场景（driver tag 不同/内存不同/类型不同），观测排序结果反推比较器优先级；或解 FUN_1418bb250 的闭包参数（比较器函数指针）反编译。

### 3.2 apply_vulkan_blocklist 隐式层链枚举不完整（本次新发现）
**现状**：doc §3.2 列 11 个 env/层名。**rne-read 0x142b19400..0x142b196f8 字符串表实测还包含**：
- `VK_LOADER_LAYERS_DISABLE` + `~implicit~`（组合值，非单独变量）
- `VK_LOADER_LAYERS_ALLOW`
- `DISABLE_RTSS_LAYER`、`DISABLE_VKBASALT`、`VK_LAYER_bandicam_helper_DEBUG_1`、`NODEVICE_SELECT`、`DISABLE_GAMEPP_LAYER`、`DISABLE_SAMPLE_LAYER`、`DISABLE_LAYER_NV_OPTIMUS_1`、`DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1`（doc 已列）
- **漏列**：`EOS_OVERLAY_DISABLE`、`VULKAN_WIN64_DISABLE`、`VULKAN_OW_OBS_CAPTURE_DISABLE`、`VULKAN_OW_OVERLAY_LAYER_DISABLE`、`VK_LAYER_reshade_1`、`DISABLE_TWITCH_VULKAN_OVERLAY`、`DISABLE_MANGOHUD`、`VK_LAYER_gshade_1` 相关（"DISABLE_LAYER_DISABLE_VK_LAYER_gshade_1"）、`VK_LAYER_*`（通配？）
- 另有日志串：`Selectively enabling implicit Vulkan layers: `、`[legacy] Force-disabling a Vulkan layer via `（"legacy" 路径的日志文案）

即实际变量名 ≥19 个（doc 只列 11）。`VK_LAYER_reshade_1` 与 doc §3.1 提到的 allowlist 默认 `VULKAN_LAYER_ALLOWLIST`（VALVE/OBS→VK_LAYER_reshade）直接相关，说明该 env 链与 allowlist 是同一条屏蔽/放行逻辑，值得合并建模。
**建议补法**：逐字符串 rne-read 完整枚举 + 在反编译中按分支定位每个 env 的读法（`getenv` 调用点、值"1"判定），确认每个变量作用于单个层名还是整组 implicit。

### 3.3 disposal 销毁桶锁拓扑
**现状**：doc §4#9 自认 U（外层整表 Mutex + 每桶 Mutex 关系未定）。
**本次复核可收敛**：
- remove（0x1418c4100）/insert（0x1418c4260）反编译均先做 parking_lot 外层锁（状态 0→8、慢路径超时 1e9ns=1s）——**外层整表锁确认存在**；
- hashbrown map 偏移矛盾：remove 在 `param_2+0x28`、insert 在 `param_1+0x8`（Ghidra this 指针基址不一致，需裁决）；
- 桶 value 约 **0xf0–0xf8B**（remove 拷贝 0xf0B 回传），不是裸 `Vec`（含多 Vec/容量字段，`capacity overflow` 串 0x142d069b0 佐证桶内 Vec push）；
- PDB 类型图（doc §0#4）给 `HashMap<CommandBufferSeq, Mutex<DisposalBucket>>`，与 decomp 的外层 Mutex 并存——两层互斥拓扑未最终定论。
**建议补法**：以 S_PUB32 解析 0x140bdfcf0=drop_in_place<DisposalBucket>（managed 域已登记），反编译桶 drop 路径看桶内各 Vec 释放顺序；对 remove/insert 的 map 偏移矛盾用 rne-read 原字节裁决结构体首字段。

### 3.4 refresh_memory_budget 滞回快照
**现状**：doc §3.3/§4#4 称滞回 `[旧-旧/8, 旧+旧/8]`，快照字段排布 I。
**本次复核（decomp-0x140c01620）**：
- 每堆钳位实为 `[旧 - max(1, 旧/8), 旧 + max(1, 旧/8)]`：`uVar24 = (old>>3) + (old>>3==0)`，即除数带 `max(1,·)` 下限；旧值 0..7 时钳到 [旧-1, 旧+1]，**doc 的"旧±旧/8"在小预算堆不成立**；
- "旧"=上一快照 @+0x608 ArcSwap 载荷 offset+0x20 起的 16 个 u64（预算数组）；"新"=驱动 vtable 查询（object@+0x88 vtable+0xd8）写回栈 16 组 budget/usage，逐组钳制后 store；
- 快照 0x110B 内 16 budget + 16 usage 排布可由 `plStack_368[4..0x13]` 位置直接推出预算数组起点 +0x20。
**建议补法**：把钳位公式精确为 `max(1,old/8)`；确认 0x110B 内 usage 数组位置与 `performance_info` 读法一致；rebuild 侧补滞回。

### 3.5 GpuAllocator 峰值包装
**现状**：doc §2.5/§4#8 峰值 getters O（rne-read 偏移 +0x620/670/678/680/688 全证）、allocate 采样/free 不采样 O；但 `free` 内部 FUN_140c16a00/140c0d550/140c16d40/140c14680 路径、全局计数器（doc §2.5 行 201 引 "全局 0x143634e4? 计数器"）未解。
**建议补法**：解 FUN_140c16d40（free 内部释放）与其断言串 0x142d06498/0x142d06508；确认峰值全局计数器是 `peak_dirty` 式惰性物化还是即时写。

### 3.6 init 全流程（GraphicsInstanceLoader::new）
**现状**：doc §1.2/§2.1 给 loader 加载"vulkan-1.dll"→"fallback-vulkan-1.dll"→系统、attempts、enumerate、api_version<1.3.261 兜底 0x400000。本次 rne-read 已证 `vulkan-1.dll`@0x142b112b0、`fallback-vulkan-1.dll`@0x142b112f8、unwrap panic 串（含 rustc commit `05f9846f8…`，可锁定工具链）。
**缺口**：30KB 反编译只摘出骨架；fallback 候选的选取逻辑、`LocationToTry` 表（loader 候选，doc §1.1 提及）未展开；api_version 兜底条件 <1.3.261 的语义（loader 版本 vs 实例版本）未逐字证。
**建议补法**：读 0x140bd8af0 反编译的候选表部分，确认 LocationToTry 数量/顺序；rne-read 验证 0x400000 兜底分支。

### 3.7 GraphicsInstanceLoader::implicit_layers 返回类型
**现状**：doc §4#2 问"原是否为显式 implicit 清单"未定。
**本次复核（decomp-0x140bda7b0）**：函数直接返回 FUN_1418b09f0 的枚举结果切片（元素 0x208B，`param_1[3]=ptr + count*0x208`），**没有过滤到 implicit-only、也没有转换为 20B GraphicsInstanceLayer**。即原 `implicit_layers` 返回**原始 vk::LayerProperties 切片**，rebuild `Option<&[GraphicsInstanceLayer]>`（且恒 None）在类型与语义上都不对位。
**建议补法**：定案为"返回原始 0x208B 枚举切片"，rebuild 应改为返回 `&[vk::LayerProperties]` 或等价物；确认 0x208B 条目与 GraphicsInstanceLayer 的 20B 差异是否在调用方转换。

### 3.8 lib.rs 日志（init_logging/print_log_dump/dump_log）
**现状**：doc §2.10/§4#11 已 O（64KB 环、0x143634e40 全局、dump_log 被 panic_reporter::dump_rhapsody_log 调用）。
**复核补充**：init_logging 反编译示全局写 5 个 qword（+0..+0x14 与 +0x10 重叠，Ghidra 栈别名），实际结构 ≥0x18B；断言串 `assertion failed: prev.is_none()`@0x142b12f50 逐字命中；panic 位置参数 `&UNK_142b12f70` 引用的 fat 串指向 `update_pending_submission_timestamp…`（归属 submit.rs 区，疑似 Location 复用，未深解）。
**缺口**：环形 {ptr,len,cap} vs {head,tail} 仍 U；dump_log 输出目标类型 U；`0x143634e40` 是 32B 结构还带 +0x18 未写字段。
**建议补法**：解 FUN_1418c1310/1418c1430（环写/读），用 rne-read 固定全局结构大小；确认 dump_log 输出目标（`Box<dyn Write>`?）。

### 3.9 performance_info 6 次堆查询
**现状**：doc §4#5 未解析 FUN_1418b1a40/b1bb0/b1cd0/b1e40/b18f0/b1fb0 各自类型；0x18B 记录字段 I。
**复核**：decomp-0x140c00a80 每次查询写 3 qword（uStack_40/48/108）= {**&str 名, 长度, u64 值**}，0x18B 记录布局相当清楚；无预算扩展分支返回 `"Unsupported"`（0x726f707075736e55 逐字节证实）。
**建议补法**：六函数逐一识别（budget/usage/名称堆查询），把 0x18B 记录命名为 {name &str, value u64}；对照 rebuild `PerformanceInfo.heaps[i].{size,budget,usage}` 对齐字段语义。

### 3.10 missing_devices / create_device 字段偏移
**现状**：doc §4#6 未定 0x28B missing 条目与 vendor 匹配、create_device +0x690/+0x698 两 qword。
**建议补法**：missing_devices（0x140bdb090，7907B）反编译内已可见大量 0x28B 栈记录与 vendor 比对（0x1002/0x10de），逐段解；create_device（0x140bdb940）解 +0x690/+0x698 字段类型（物理句柄+队列族候选）。

### 3.11 find_vendor_specific_device_handle 清单来源
**现状**：doc §4#7 未定 FUN_1418b1340 枚举内容；原仅 0x10de 分支。
**建议补法**：反编译 FUN_1418b1340 看枚举来源（设备名列表常量表?）；确认原是否真的只匹配 NVIDIA（rebuild 通用子串候选差异）。

### 3.12 ray_tracing 全部
**现状**：doc §2.9/§4#12 全部 U（7 个 inlinee_declaration、无地址、rebuild 无实现）。这是"还原路径"外的最完整未知块。
**建议补法**：ray_tracing 涉及 BLAS/TLAS 构建，属 GPU 后置范围。至少先恢复 `GeometryInstance`（881/907）的字段语义（transform/mask/instanceOffset）与 `create_ray_tracing_acceleration`（250）的 PDB 签名，登记为数据合同级成果，不追求机器码闭环。

### 3.13 crash 链 shader producer / vendor dump
**现状**：doc §3.6 半对（marker/assertion/error 门控 O；`log_gpu_crash_info` 的 shader 诊断 producer、vendor dump、真实 device-lost 未验收）。
**建议补法**：crash 的 shader 侧 producer（写 debug buffer 的 compute）与 `require_vendor_crash_dump`（rebuild 恒 Err）路径——确认原是否有 nv-aftermath 接线（crates 域 aftermath 已恢复 4 回调）。

---

## 4. rebuild 对照缺口

| # | 原版 | rebuild 现状 | doc 判定 | 本次核实 / 补强 |
|---|---|---|---|---|
| 1 | `apply_vulkan_blocklist` 隐式层屏蔽链（≥19 env 名） | `apply_vulkan_blocklist(layers, blocked)` 通用过滤；modern 传 `&[]`（空表）、legacy 仅 LUNARG、doubletap 去重 | ✗/半 | **确认偏差**；可补"由 env 驱动"的对照实现（读 getenv 决定屏蔽集），至少登记为候选差异并列出完整 env 表 |
| 2 | `CommandBufferDisposalBuckets`=hashbrown::HashMap<Seq, Mutex<DisposalBucket>> + 外层整表锁 | `BTreeMap<u64, Vec<DestroyAction>>`（无锁）+ 另有 `begin_command_bucket`（重复返回 Err） | ✗ | **确认偏差**；补充说明 rebuild 已有 `begin_command_bucket` 重复检测候选（`Failure::DuplicateBucket`），与原"断言 panic"仍不同 |
| 3 | `refresh_memory_budget` 滞回 `[旧±max(1,旧/8)]` | 直查直存，无滞回 | 半 | **确认偏差**；补钳位公式 `max(1,·)` 精化 |
| 4 | gpu-allocator 0.26 本地分叉（ash0.38） | gpu-allocator 0.27 公开版子分配，默认关（Dedicated） | 半 | MODULE_STATUS 行 33 一致；`suballoc.rs` 头注已明示"块大小策略/统计口径可能不同" |
| 5 | `init_logging`/`print_log_dump`/`dump_log` + 0x143634e40 64KB 环 | lib.rs 仅模块声明 | ✗ | **确认缺失**；rebuild crash.rs 头注提到原宿主侧 CrashDiagnostics 标记环"不等价于原作 leekspin::RingBuffer 已删除"——**leekspin::RingBuffer 结构恢复状态 doc 未登记**，建议补 |
| 6 | loader fallback（"vulkan-1.dll"→"fallback-vulkan-1.dll"→系统） | `from_candidates(&[])`→系统 loader；无 fallback 显式候选 | ✓/半 | 确认；fallback 语义（第二个候选 DLL 何时加载）待原逻辑展开（§3.6） |
| 7 | `implicit_layers` 返回原始 0x208B 枚举切片 | `Option<&[GraphicsInstanceLayer]>` 恒 None | 半 | **类型/语义双重不对位**（§3.7），rebuild 应改返回类型或明确 None 即"未启用"语义 |
| 8 | `workarounds::GraphicsWorkarounds` 独立 Ron 结构 | **完全无 workarounds.rs**（doc §3.8 写"并入 allowlist 内"，实为不存在） | 半 | 纠偏：rebuild 无该结构；`update_from_ron`（AllowlistMigration）是另一机制，不能算并入 |
| 9 | `dispose_cb_resource` 7 族实例 | `dispose_cb_resource<T: Send>` 泛型 | ✓（但数量错） | 泛型等价成立；修正 doc "6 族"→7 |
| 10 | `performance_info` 6 次独立堆查询 + 0x18B {name,value} | `memory_snapshot` 一次 memory_properties2 | 半 | 确认；建议按 §3.9 把 0x18B 记录语义登记，供 rebuild 字段对齐 |
| 11 | `log_gpu_crash_info` shader producer/vendor dump | `require_vendor_crash_dump` 恒 Err | 半 | 确认；补 aftermath 接线调查（§3.13） |
| 12 | 原 10 个 NotFound 均 rne-read 补证 | — | O | **证据计数偏差**：doc 写 9，summary.json 实为 **10**（0x14067dd80/0x140a9fa70/0x140be6b40/0x140c001c0/0x140c001e0/0x140c001f0/0x140c0e600/0x140c0e610/0x140c0e630/0x140c0e640） |

### 4.x 已发现的事实/证据偏差（需订正登记，不改原文）
1. **NotFound 计数 9→10**（summary.json 逐条核实；ALGORITHMS.md 第 4 行与 MODULE-DEEP-DIG 登记行均写 9）。
2. **dispose_cb_resource 实例 6→7**（骨架含 ManagedTexture 第 7 族；ALGORITHMS.md §2.7"6 实例"、§3.7"6 族"）。
3. **滞回钳位公式**：`[旧±旧/8]` → `[旧±max(1,旧/8)]`（旧<8 时 ±1）。
4. **dispose_in_order 断言串地址**：0x142d06f88 实为文件路径 `crates/rhapsody/src/raw/disposal.rs`，断言文本 `assertion failed: seq > self.last_retired` 紧随其后（约 0x142d06fb1）；ALGORITHMS.md §2.7 行 228 的"0x142d06f88 后接路径"表述次序相反。
5. **init_logging 断言串**：`assertion failed: prev.is_none()` 确认在 0x142b12f50（非 0x142b12f70）；0x142b12f70 是 panic Location fat 串，指向 `update_pending_submission_timestamp…`（submit.rs 区），doc §2.10 行 264/266 的地址归属需复核。

---

## 5. 补全优先级排序

**P0（业务核心，直接决定设备选择/内存预算正确性）**
1. 设备排序比较器 `order_and_filter_devices` closure$0/$1（行为注入反推优先级）——设备选择最终仲裁。
2. `apply_vulkan_blocklist` env 链完整枚举（≥19 变量）+ 逐分支过滤粒度（含 `VK_LOADER_LAYERS_DISABLE~implicit~` 与 `VK_LAYER_reshade_1` 与 allowlist 合并建模）。
3. `refresh_memory_budget` 滞回公式精化 `max(1,旧/8)` + 0x110B 快照字段排布（预算数组@+0x20 已定，usage 待定）。

**P1（结构/拓扑，影响并发与生命周期正确性）**
4. disposal 锁拓扑裁决（外层 Mutex + map 偏移 +0x28/+0x8 矛盾 + 0xf0B 桶值；S_PUB32 0x140bdfcf0 drop_in_place<DisposalBucket>）。
5. `implicit_layers` 返回类型定案（原始 0x208B 切片），rebuild 对齐。
6. `GpuAllocator::free` 内部 FUN_140c16a00/16d40 族 + 峰值全局计数器 + 断言串归属。
7. lib.rs 日志环结构（0x143634e40 32B + 环写/读 FUN_1418c1310/1418c1430）+ leekspin::RingBuffer 恢复状态登记。

**P2（字段语义/调用细节，低风险高确定性）**
8. `performance_info` 6 次堆查询识别 + 0x18B {name,value} 定案。
9. `missing_devices`/`create_device` 0x28B 条目与 +0x690/+0x698 字段。
10. `impl$26::fmt` 类型归属；`vulkan_debug_messenger` 回调；`parse_driver_version` 编码；`leek::impl$1::from` 转换目标。
11. ray_tracing `GeometryInstance` 字段/`create_ray_tracing_acceleration` 签名（数据合同级）。

**零成本订正**（改文档措辞即可，不动原文）：NotFound 9→10、dispose_cb_resource 6→7、滞回 `max(1,·)`、dispose_in_order 断言串地址次序、init_logging 断言串地址。

---

未改任何 reconstruction 源码，未改 ALGORITHMS.md 等既有交付文档。
