# crates 域复现报告（vk-sync-fork / aftermath-rs / gpu-profiler / dxgi-enumerate-gpus / shader-pipeline 核对）

日期：2026-10-15。权威 = B patched EXE（`/home/ubuntu/test/games_test/tiny-glade.exe`，机器码直读）。
标注：✅原版对齐[证据地址] / ⚠️候选或假设 / ❌读不出（写明读了什么）/ ➡️依赖库实现（不复现）。

## 0. 先判定：哪些是依赖库、哪些是本地 crate

`skeleton/B/Cargo.lock`：`vk-sync-fork 0.4.0`、`aftermath-rs 0.1.3`、`gpu-profiler 0.1.0`、`dxgi-enumerate-gpus 0.1.0`
**四个都没有 `source` 字段**（对比 `sysreq`/`speedy`/`puffin` 有 `registry+...`），PDB 注解为 `local_or_path`，仓库里
`skeleton/C/rhapsody2/crates/{vk-sync-rs,aftermath-rs,gpu-profiler,dxgi-enumerate-gpus}` 就是它们 → **全是原版本地 crate，
不是 registry 依赖**，需要复现（而不是"直接复用"）。例外的"依赖库"部分：

| 项 | 判定 |
|---|---|
| NVIDIA Aftermath SDK 本体 | ➡️ 外部 DLL（导入表 `GFSDK_Aftermath_Lib.x64.dll` 恰好 4 个导入：Disable/EnableGpuCrashDumps、GetCrashDumpStatus、GetShaderDebugInfoIdentifier），不复现；`aftermath.rs` 用 `AftermathApi` trait 抽象这 4 个入口 |
| DXGI / `windows` 0.37 / `CreateDXGIFactory`（导入 `dxgi.dll`） | ➡️ 系统 API，不复现；`dxgi.rs` 只复现本地 crate 的枚举/记录逻辑 |
| speedy（wire 读取）、puffin、std DefaultHasher | ➡️ 依赖库（`speedy 0.8.6` registry） |
| vk-sync-fork 的公开同名 crate（registry 0.4.0，离线缓存有） | 逻辑与原版**几乎一致**（见 §1）；但原版是本地分叉（多 6 个变体 + 2 项数值不同），所以不能直接依赖，在 `raw/access_table.rs` 复现本地版并用测试锚定公开版差异 |

## 1. vk-sync-fork（`raw/access_table.rs`）

- ✅ 64 字符三表：`get_access_info`（0x140c10dd0）= 三个 256×u32 子表 `stage @0x142b1a6c8 / access @0x142b1a7c8 / layout @0x142b1a8c8`，
  输出写 `out+0/+4/+8`。**我用脚本把 EXE 三表逐项与 `access_table.rs::ACCESS_TABLE` 比对：64 项 0 差异**。
  （纠正 `decomp/crates-get-access-info-table.md` 里若干条目：例如 0x08/0x0b/0x0e/0x11/0x18/0x20/0x21 的 access 是 `0x8`(UNIFORM_READ)，0x2f 的 access 是 `0x600` 不是 `0x300`；代码里的表是从 EXE 直接读的，是对的，文档表有误。）
- ✅ `is_write = (c-0x28) < 0x10`（0x140c10e10；0x140c107c0/0x140c10940/0x140c10af0 内联同一判定 `add bpl,0xd8; cmp bpl,0x10`）→ 写码恰为 0x28..=0x37（16 个）。
- ✅ **与公开 vk-sync-fork 0.4.0 的逐变体对照**（在 /tmp 临时工程里用公开 crate 的 `get_image_memory_barrier` 逐变体探测，期望值内嵌到测试）：
  58 个公开变体中 **56 个与原版码值逐值一致**；差异恰好：
  1. `IndirectBuffer`(0x02)：原版 stage=`0x10000`(ALL_COMMANDS)、access=`0x21`(INDIRECT_COMMAND_READ|SHADER_READ)；公开版 `DRAW_INDIRECT/INDIRECT_COMMAND_READ`。
  2. `DepthStencilAttachmentWrite`(0x2f)：原版 access=`0x600`；公开版 `0x400`。
  3. 原版新增 6 个读变体（公开枚举没有）：0x1b,0x1c,0x1d,0x1e,0x1f,0x23（stage `0xf8`/`0x3f8` 等，layout 0/5/4/1），导致 `AnyShaderRead*` 与其后的码整体后移，写区间落在 0x28..0x37。
  （落测试 `table_matches_public_vk_sync_fork_except_documented_differences`。）
- ✅ 三个 barrier 转换函数的逻辑与公开版一致，输出不是"自定义 32/56/72B 结构"而是 **`(src_stage:u32, dst_stage:u32, vk::XxxBarrier)`**（sType 0x2e/0x2c/0x2d = 46/44/45 = 普通 `VK_STRUCTURE_TYPE_{MEMORY,BUFFER_MEMORY,IMAGE_MEMORY}_BARRIER`，**不是** Synchronization2 的 *_BARRIER_2；纠正旧文档）。输出布局：+0 src_stage、+4 dst_stage、+8 起即 vk 结构（Memory 共 0x20B、Image 共 0x50B）。
  - `get_memory_barrier`（0x140c107c0）：src_access 只累计写码的 access；dst_access **仅当 src_access≠0** 才累计（WAR/RAR 清零）；空 stage 回落 TOP_OF_PIPE(1)/BOTTOM_OF_PIPE(0x2000)（`cmp r,1; adc r,0` / `test; cmovne eax,0x2000`）。
  - `get_buffer_memory_barrier`（0x140c10940）：同上 + 透传 queue family/buffer/offset/size（输入 +0x20..0x40 → 输出 +0x20..0x40）。
  - `get_image_memory_barrier`（0x140c10af0）：dst_access **无条件**累计（0x140c10c52 `or esi`/`or edi` 无 test）；`+0x44/+0x45` 字节 = previous/next layout（0 Optimal、1 General、其余走 "not implemented" panic，字面量 0x142b1a658），`+0x46` = discard_contents（真 → old_layout=0）；General + code 0x27(Present) → `0x3b9acdea`(PRESENT_SRC_KHR)，否则 GENERAL(1)；空 next → new_layout 0。
  - 落码：`access_table::{get_memory_barrier, get_buffer_memory_barrier, get_image_memory_barrier, BufferBarrier, ImageBarrier, BarrierLayout}`（纯函数，访问码切片入参，返回 ash 结构）。测试：`memory_barrier_*`、`buffer_barrier_passes_fields_through`、`image_barrier_layouts_and_unconditional_dst_access`。
- ❌ `cmd::pipeline_barrier`（0x1418afd20，经 device vtable +0x368 下发）属于 raw `GraphicsDevice::issue_pipeline_barrier`（别人的文件），未改。

## 2. aftermath-rs（`raw/aftermath.rs`，新增）

- ✅ **原版是否真启用**：真启用但有条件。`Aftermath::new`（0x140176e70）只在 `[cfg+0xbe]==1` 时被调用（0x140593dbb `cmp al,1; jne` → 0x140593ddb 构造 `NvAftermathDelegate::default`(0x140244460) → 0x140593dfc call new）。
- ✅ `new`：delegate(0x38B) 装箱后**泄漏**；再分配 16B `{data, vtable(0x1429603f8)}` 作 userData（也泄漏）；`GFSDK_Aftermath_EnableGpuCrashDumps(0x217, 2, 1, cbGpuCrashDump, cbShaderDebugInfo, cbDescription, cbResolveMarker, userData)`（0x142923580 = IAT 导入名，S_PUB32/导入表双证），**返回值不看**。`0x217/2/1` 的头文件名（`Version_API` / `WatchedApi_Vulkan` / `DeferDebugInfoCallbacks`）⚠️是 I，数值是 O。
- ✅ `Drop`（0x140c107b0）= `jmp 0x142923880` = `GFSDK_Aftermath_DisableGpuCrashDumps`（导入表 IAT 0x143192d38 首项）。**修正 ALGORITHMS 悬空证据**：原字节 `e9 cb 30 d1 01`。
- ✅ 回调 vtable（0x1429603f8）：+0x00 drop 0x1405f80b0、+0x08 size 0x38、+0x10 align 8、**+0x18 = `dumped`(0x140244790)、+0x20 = `shader_debug_info`(0x140245340)、+0x28 = `description`(0x14003bb80)**；三个 C 回调（0x140c106c0/0x140c10710/0x140c10760）按此转发。
- ✅ **新发现**：第四个 C 回调 `ResolveMarkerCallback` = 0x14003bb80 = **单条 `ret`**（ICF 折叠 437 个空函数），不转发、不写出参；游戏 `NvAftermathDelegate::description` 同址 = 不添加任何描述。旧文档 "vtable+0x30（I）" 作废。
- ✅ `wait_for_status`（0x140c104e0，返回 `al` 即最后一次状态）：先 `GetCrashDumpStatus`；状态字节 ∈{2,4} 返回；否则（有超时且 `elapsed>=timeout` 返回当前状态）；`sleep(50ms)`，`elapsed+=50ms`（`overflow when adding durations` panic，0x142b1a5d0）。`Option<Duration>::None` = nanos niche `1_000_000_000`。游戏侧调用 `wait_for_status(5s, 0)`（0x140bf03a6：`ecx=5, edx=0`，且仅当 `[rsi+0x1b]` 非 0）。
- ✅ `get_shader_debug_info_identifier`（0x140c10620）：`GetShaderDebugInfoIdentifier(0x217, ptr, len, &mut [u64;2])`；`Success`==1（`iVar6 != 1` 判错）；错误文案 = `"GFSDK_Aftermath_GetShaderDebugInfoIdentifier: {code:?}\n"`（0x142b1a608 + `\n` 两段字面量）。
- ❌/⚠️ 状态值 2/4 的 SDK 枚举名未证实（凭记忆的 SDK 头有两种编号方案，不采信；代码只保留原始数值常量 `STATUS_EXIT_A/B`）。
- 落码：`NvAftermathDelegate`（dumped/shader_debug_info/description）、`AftermathApi`、`Aftermath<A>`（`new/status/wait_for_status[_with]/shader_debug_info_identifier/Drop`）、4 个 `extern "C"` 回调、`UnavailableApi`（非 Windows）、`windows_api::SdkApi`（`cfg(windows)`，`raw-dylib`，**未编译验证**：仓库没有 Windows 目标，写法按 raw-dylib 规范）。偏差：保存了 Enable 的返回码（原版丢弃）。
- 测试：`aftermath_registers_with_original_arguments_forwards_callbacks_and_disables_on_drop`（mock API 抓回调并直接调用，含 ResolveMarker 不写出参）、`aftermath_wait_for_status_*`（50ms 步进/超时边界）、`aftermath_identifier_error_text_*`、`aftermath_unavailable_api_*`。

## 3. gpu-profiler（`raw/gpu_profiler.rs` 新增；`raw/profiler.rs` 追加输出缓冲常量）

- ✅ `Profiler::new`（0x140c10f20）：`vkCreateQueryPool{sType=0xb(QUERY_POOL_CREATE_INFO), queryType=2(TIMESTAMP), queryCount=0x400}`（**纠正旧文档把 sType 的 0xb 当 queryType**）；失败 panic；索引缓冲 0x1ff8B=0x3ff 个 u64，初值全 `0xffffffff`（`0x142a16e60` 16B 模式铺满，尾 3 个 u64 也是）；结构 `{buf@0, cap=0x3ff@8, pool@0x10, used@0x18}`。
- ✅ `begin_frame`（0x140c11090）：`vkCmdResetQueryPool(0,0x400)`；`used=0`；返回 ActiveProfiler。
- ✅ `create_marker`（0x140c110f0）：无去重 push，返回下标；元素 16B=`&str`（我用 `String`，⚠️偏差仅在所有权）。
- ✅ `write_marker`（0x140c11150）：`idx=lock xadd used`；`idx<0x3ff` 才写；`buf[idx]=marker`；`vkCmdWriteTimestamp(cmd, BOTTOM_OF_PIPE(0x2000), pool, idx)`——**第 4 实参 r9 是 idx（xadd 旧值）**，`vtable+0x388`；越界 panic（index out of bounds）。
- ✅ `end_frame`（0x1418c3910）：`vkCmdCopyQueryPoolResults(cmd, pool, 0, used, *out, 0, stride 8, flags 3=64_BIT|WAIT)`（vtable +0x390）；打包 `{markers, 输出缓冲描述拷贝, query_indices Vec<u32>}`。⚠️`query_indices = buf[0..used] as u32` 是对 `FUN_1418b8a60` 迭代器的推断（读出源范围 `[buf, buf+cap*8)`、长度 used，元素收窄 u32 的具体 closure 未单独反汇编）。
- ✅ `retrieve_results`（0x140294cf0）：把 `(query_index[i], timestamp[i])` 配成 16B `{u32 marker, u64 tick}`，`sort_unstable_by_key(tick)`（`<0x15` 插入排序 0x1405c3580，否则 ipnsort 0x1405c75b0；比较读条目 u64 字段，经 0x14067dd80 恒等访问器）；两处 panic（对齐检查/越界）。`0x140bd3c80` = `ProfilerBuffer::mapped_slice`（读 u64 映射数组）。
- ✅ 换算：`GpuTick::duration_since`(0x140c10e50) 饱和差；`GpuTickDuration::nanoseconds`(0x140c10e60) = `(ticks as f64 * f32 period as f64) as u64`（f32 在 xmm1，即 `timestampPeriod`；`as u64` 饱和）；`NanoSecond::from_raw_ns`(0x14067dd80) 恒等；`MarkerId::from_u64`(0x140c10e20) u64::MAX panic，否则截断。
- ✅ 输出缓冲：`create_gpu_profiler_output_buffer`（0x140bff4a0，日志目标 `"rhapsody::raw::profiler"` 级别>3）→ `create_query_result_buffer`（0x140bff5a0）：`VkBufferCreateInfo{sType=12,size=r8=0x2000,usage=2(**只有 TRANSFER_DST**),sharing=0}`，分配名 `"buffer"`，位置 `GpuToCpu(4)`。**纠正** raw-core ALGORITHMS 的 "STORAGE|TRANSFER_DST"。落码 `profiler::{GPU_PROFILER_OUTPUT_BYTES, gpu_profiler_output_buffer_desc, read_output_timestamps}`。
- ⚠️ 与候选 `raw/profiler.rs::GpuProfiler`（每区段两条时间戳）**无等价**：原版是"每 marker 一条 BOTTOM 时间戳 + 一次 CopyQueryPoolResults"。候选原样保留（有现有测试/调用方）。
- ❌ puffin 输出（`TimedFrame::send_to_puffin` 0x140bd4030）：puffin 非本工程依赖，保持空操作；`ProfilerSubmittedFrame` 里 0x58B 输出缓冲描述拷贝只保留语义（我只保存 marker 表与 query_indices）。
- 测试：`gpu_profiler_shared_types_match_the_machine_code`、`gpu_profiler_retrieve_results_*`、**`gpu_profiler_chain_runs_on_a_real_device`（lavapipe 真跑整条链，未跳过：Profiler::new→begin_frame→3×write_marker→end_frame→submit→读回→retrieve_results 升序）**。

## 4. dxgi-enumerate-gpus（`raw/dxgi.rs` 新增）

- ✅ 纠正：不是 `EnumAdapters1/GetDesc1`，是 **`IDXGIFactory::EnumAdapters`（vtable +0x38 = 槽 7，S_PUB32 0x14213b8e0）+ `IDXGIAdapter::GetDesc`（+0x40 = 槽 8，0x14213b950）**；拷贝 0x130B = `DXGI_ADAPTER_DESC`（无 Flags）。`CreateDXGIFactory` IID 字节 @0x142d0324c = `{7b7166ec-21c7-44ae-b21a-c9ae321ae369}`（IDXGIFactory），失败上下文串 `"CreateDXGIFactory"`（0x142d0325c，17B）。
- ✅ 40B 元素：`name String{ptr@0,len@8,cap@0x10}`、**`luid u64 @0x18`（`(HighPart as i64 as u64)<<32 | LowPart`，desc+0x128/+0x12c，`movsxd`/`mov r12d`）**、**`vendor_id u32 @0x20`**（desc+0x100）；`+0x24` 从不写。**DeviceId/SubSysId/Revision/内存大小都不入元素**。名字 = `Description[128]` 扫到首个 0（无则 128）→ UTF-16 解码。
- ✅ 控制流：`EnumAdapters` 任何错误即结束并 `Ok`（不是只认 NOT_FOUND）；`GetDesc` 失败只跳过该适配器；仅 `CreateDXGIFactory` 失败返回 `Err`。
- 落码：`DxgiAdapter`（size 0x28 常量断言）、`DxgiAdapterDesc`（0x130，offset 断言）、`DxgiSource` trait、`FixedDxgi`（注入）、`enumerate`、`cfg(windows)` `SystemDxgi`（`raw-dylib`+COM vtable，**未编译验证**）。
- ⚠️ **给 `raw/init.rs` 的 owner**：`init.rs::DxgiAdapter` 有 `device_id: u32`，原 40B 元素没有这个字段（`+0x24` 是未写的补齐）；`missing_devices`（0x140bdb090）读的是 `+0x18`(luid)、`+0x20`(vendor)、名字——建议改用 `raw::dxgi::DxgiAdapter` 或删去 `device_id`。我没动 init.rs。
- 测试：`dxgi_record_layout_and_field_mapping`、`dxgi_enumerate_skips_failed_getdesc_stops_at_first_enum_error_and_maps_factory_failure`。

## 5. shader-pipeline 核对（只读，未改 crates/shader-pipeline）

| 项 | 结论 |
|---|---|
| `with_suffix_dims`（0x140c87650） | ✅ `48 8d 41 01 c3` = `lea rax,[rcx+1]; ret` = `wrapping_add(1)`；`reflection.rs:15` 一致 |
| `BindingNameHash::new` | ✅ 沿用既有 O 证（`_dims` 剥 5 且 +1；FNV-1a64）；`reflection.rs:11` 一致 |
| `Type::nbyte`（0x1420bda00，跳表 0x142fd8f84，`xor 0x8000000000000000`，<14 项，default=case 10=Struct） | ✅ 规则与 rebuild（经 spirq `nbyte`）一致，**纠正旧文档的 "Struct 累加 Σ"：原版是最后一个成员 `offset + nbyte(ty)`**（`[end-8]`、递归 `end-0x38`），空结构=Some(0)；DeviceAddress/DevicePointer=8；Image/CombinedImageSampler/SampledImage/StorageImage/Sampler/SubpassData/AccelStruct=无尺寸 |
| `ScalarType::nbyte`（0x1420bd380，表 0x142fd8f5c） | ✅ Void→0、Boolean→1（返回 tag 值）；Signed/Unsigned/Float(n)→n（payload 原值，`mov eax,[rcx+4]`）；rebuild 的 spirq 路径给出相同数值（测试 `reflection_type_nbyte_matches_the_machine_code_rules`）。Vector/Matrix/Array 的 `stride×n` 分支（0x1420bdac2/0x1420bdb09）只读到结构，未逐 case 构造向量对拍 → ⚠️ |
| `read_from_buffer`（0x1409e4670）、尾字节、30B 最小长度、无 magic/版本头 | ✅ 沿用 `rne/crates-notfound-closure.md §6` 的 O 证；`compiled.rs::decode` 的 `trailing_bytes` 建模一致。❌ 我没有重新分段读 50KB 反编译 |
| `calculate_hash`/`read_from` 是否有地址 | ❌ 两者在 B 里均 inline、无独立地址（skeleton `compiled_procedure` 无 RVA）；speedy 派生由依赖库生成 ➡️ |
| defines 哈希 | ✅ 与 shader-pipeline owner 的结论**独立复现**：`acc ^= SipHash-1-3(key0,0)((String,String))`（std DefaultHasher；String 哈希 = 字节+0xff），`("SHADER_VARIANT_DEPTH_ONLY","0")→0xb903e43ffb3da915`、`"1"→0xf0adff76827e73ff`，即发行 234 个 raster .bin 的仅有两个哈希；compute 空 defines=0。我用独立 Python SipHash 与 Rust std 各算一遍均吻合；测试 `shipped_raster_defines_hashes_are_siphash13_xor_of_pair_hashes`（不经 shader-pipeline）。机器码位置（owner 记录）0x141795d10/0x141796080 循环 + 0x1420cfbf0 单对哈希 |
| `RasterPipelineDesc`/`ComputePipelineDesc` | 未触及（描述符包/shader-pipeline owner） |

## 6. ALGORITHMS crates ✗/半/U 逐项去向

| 原条目 | 去向 |
|---|---|
| gpu-profiler ✗（无链/无 SubmittedFrame/无回读） | ✅ 落码 `gpu_profiler.rs` 全链 + lavapipe 实跑 |
| vk_sync_fork ✗ | ✅ 表 64 项=EXE（0 差异）、barrier 三函数落码、与公开版差异锁进测试 |
| aftermath_rs ✗ | ✅ 注册/回调/Drop/wait/identifier 落码（SDK 本体 ➡️，Windows 外壳⚠️未编译） |
| dxgi ✗ / 40B 字段 U | ✅ 字段映射读出并纠正（luid+vendor）；Windows 外壳⚠️未编译 |
| Aftermath::Drop 悬空 | ✅ `jmp DisableGpuCrashDumps` 原字节 |
| `ResolveMarkerCallback` vtable+0x30（I） | ✅ 更正：它是 `ret` |
| get_access_info 全表 / role 修正 | ✅ stage/access/layout 三表 |
| GpuTick/Duration/NanoSecond/0x140bd3c80 | ✅ 见 §3 |
| with_suffix_dims O/I | ✅ O |
| Type::nbyte case 映射 I / ScalarType 表 | ✅ Scalar/Struct/地址/opaque 读定；Vector/Matrix/Array 乘法分支 ⚠️ |
| defines 哈希 U | ✅（owner 已解，本报告独立复核） |
| read_from_buffer 50KB | ✅ 沿用既有 O（未重读） |
| `Profiler` 无销毁路径 | ⚠️ 原版未观察到 drop；`destroy` 为显式补充 |
| `ActiveProfiler::end_frame` 的 query_indices 迭代器 closure（FUN_1418b8a60） | ⚠️ 推断 |
| 6 个 reflection fmt / inline eq-hash closure 的登记 | ❌ 未做（derive 复用类胶水，无业务逻辑，非本包范围） |

## 7. 改动文件 / 新增测试 / 遗留

改动：`crates/rhapsody/src/raw/access_table.rs`（追加 barrier 三函数 + 类型，原 64 项表未改）、`raw/profiler.rs`（追加输出缓冲常量/描述/读回，候选 `GpuProfiler` 未改）、`raw/mod.rs`（仅追加 `pub mod aftermath; dxgi; gpu_profiler;`）。
新增：`raw/aftermath.rs`、`raw/dxgi.rs`、`raw/gpu_profiler.rs`、`tests/crates_domain.rs`（15 个测试，含 1 个 lavapipe 实跑）。未改 `shader-pipeline/shader-compiler`、`rg/`、`managed/`、其他 raw 文件；没有新增依赖。
验证：`cargo test --offline ... -p rhapsody -p tiny-glade -p country-core --all-features` —— 第一次整跑时 `rhapsody` lib 有 1 个失败 `rg::compiled::original_layout::layout_segments_tile_the_0x218_bytes`（他人正在改的 `rg/compiled.rs`，非我文件）；随后 `-p rhapsody --all-features --no-fail-fast` 重跑，lib 65 + 全部集成测试通过（含 crates_domain 15/15）。`-p tiny-glade -p country-core --no-fail-fast`：country-core lib 1383 通过、1 失败 `resources::time_of_day::tests::midpoints_interpolate_linearly`（非我文件，我没碰 country-core），其余 target 无错误。

遗留 / 需他人处理：
1. `raw/device.rs::create_gpu_profiler_output_buffer(size)`（非我文件）用 `STORAGE_BUFFER | TRANSFER_DST`；原版 usage 只有 `TRANSFER_DST`、名 "buffer"、`GpuToCpu`、大小 0x2000（0x140bff5a0）。我的 `profiler::gpu_profiler_output_buffer_desc()` 给出原描述，请 device.rs owner 对齐。
2. `raw/init.rs::DxgiAdapter.device_id` 无原版对应（见 §4）。
3. Windows 外壳（`aftermath::windows_api`、`dxgi::SystemDxgi`）未在 Windows 目标编译；本机仅 `x86_64-unknown-linux-gnu`。
4. 旧 evidence 文档里的 EnumAdapters1/GetDesc1、STORAGE|TRANSFER_DST、ResolveMarker vtable+0x30、Struct "Σ"、queryType=11、get_access_info 部分 access 值等应按本报告更正（未改动历史文档）。
5. 收到用户中途消息 "空袭" / "继续"：前者无可执行含义，按"继续"处理。
