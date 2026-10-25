# raw-init 域（实例 / 黑名单 / 销毁桶 / 分配器 / 崩溃 / 格式 / 日志 / leek）复现报告

日期 2026-10-15。验证：`cargo test --offline -p rhapsody -p tiny-glade -p country-core --all-features`（lavapipe）= 1513 passed / 0 failed；`original_oracle*` 未回归。
图例：✅原版对齐[证据]　⚠️候选/假设　❌读不出（写明读了什么）。

## 本轮新读出的机器码事实（纠正了既有分析文档）
1. **disposal 外层是 `parking_lot::RwLock`，不是 Mutex**：remove/insert（0x1418c4100/0x1418c4260）CAS 状态字 0→8（`WRITER_BIT=8`），释放 8→0；`dispose_cb_resource<T>`（0x1418c65a0 等）用 `lock cmpxchg [state], rax+0x10`（`ONE_READER=0x10`）取读锁，探测 hashbrown（桶步长 0x100 = key 8B + `Mutex<DisposalBucket>` 0xf8B），再对桶内 Mutex 字节 CAS 0→1 后推入动作。
2. **缺桶不 panic**：`dispose_cb_resource` 缺桶时写日志环 kind 4 记录并把资源放进调用方回退桶（`take_out_dropped_waste` 的栈桶），随后 `immediate_empty_bucket` 立即销毁；命中写 kind 3（`[name16][u64 seq][3]`，25B）。
3. **`+0xa50` 不是 owning_device**：`dispose_in_order` 里 `lea r15,[rdi+0xa50]; call 0x1418c4100(remove)` ⇒ `ManagedResources+0xa50` 就是 `CommandBufferDisposalBuckets`（RwLock 状态字）。managed ALGORITHMS 的“+0xa50 双语义矛盾”就此消解（u16 device 是 16B 退役项的 +0xc 字段，另一回事）。
4. **`SequentialDisposal::dispose_in_order`（0x1418c7ae0）**：`{ VecDeque<u64> @0(cap/ptr/head/len), last_retired @0x20 }`；`assert seq > last_retired`（串 0x142d06fb0，文件 0x142d06f88）；窗口 `pending[i]` ↔ 序号 `last_retired+1+i`，0=未完成。idx==0：`last_retired+=1`→日志 kind 0x1e（`[u64][0x1e]` 9B）→`remove` 桶(+0xa50)→执行桶(0x140bf92f0)→释放(0x140bdfcf0)→`pop_front`，再循环弹出连续已完成的（`assert_eq!(last_retired, front)`）；idx==len：push_back；idx>len：补 0 后 push_back；idx<len：槽必须为 0 否则 panic。**允许乱序完成**，rebuild 旧实现“只收 prefix+1”是偏差。
5. **日志环 0x143634e40 = `leekspin::RingBuffer`**：`{data ptr@0, cap@8=0x10000, cursor AtomicU64@0x10}`，写 `FUN_1418c1380`=`fetch_add(len)` + 取模 + 单/双段拷贝（`len>cap` 断言 `part1 <= self.data.len()`，cap=0 除零 panic），**无锁**；转储 `FUN_1418c1430`：`cursor<=cap` 取前 cursor 字节，否则整份 `rotate_left(cursor%cap)`；`init_logging` 先写全局后断言 `prev.is_none()`。
6. **`apply_vulkan_blocklist` 的入参是 `GraphicsInstanceLayerAllowlist` 的条目表**（32B 元素 `{tag@0, String@8}`），不是发现的层名；`tag&1==1`=Vendor → `format!("VK_LAYER_{vendor}*")`（pieces "VK_LAYER_"、"*" @0x142b19478），`tag&1==0`=Layer → 层名原样；`,` 连接成 `VK_LOADER_LAYERS_ALLOW`。旧文档“筛 bit0=1 且匹配 VK_LAYER_*”是误读。
7. **`log_gpu_debug_info` 末段**：日志行 = `"🔥 {idx}: {msg} in {src}:{line}"`（pieces @0x142d06d60，info 级），源名读文件 `tmp/shader-cache/0x{src_id:x}.srcname`，失败显示 `???`。
8. **`leek::impl$1::from`（0x140be2880）= `From<&str> for [u8;16]`**：非 ASCII 码点被**跳过**，ASCII 依次填入，满 16 停；`dispose_cb_resource` 内联了同一循环（日志记录的 16B 名字字段）。
9. **take_out_dropped_waste 跳表（0x142d060f0）tag 3 = panic(`todo!`)**（函数内嵌 "not yet implemented"），不是未知类型。
10. **`InstanceBuilder::build`（0x140c08f10）**：默认扩展 `[VK_KHR_get_physical_device_properties2 (0x142b1909b), VK_KHR_get_surface_capabilities2 (0x142b190c2)]` + 标志位追加 `VK_EXT_debug_utils`(0x142b190e3) + 调用方扩展；探测 `VK_EXT_layer_settings`（22 字节含 NUL memcmp，常量 0x142b18de0/df0，VkExtensionProperties 步长 0x104）：可用则 info 日志并追加（0x142b19210），不可用则 info 日志跳过，均不致命；builder 标志 `+0x18==1` 时请求层 `VK_LAYER_KHRONOS_validation`（0x142b19110，0x1b 字符，缺失则 panic 路径 0x142b18f88）。

## 工作包逐项

### 1 disposal（D4）
- ✅ 结构改为 `RwLock<HashMap<seq, Mutex<DisposalBucket>>>`（`raw/disposal.rs`）。`insert` 先替换后断言 `assertion failed: prev.is_none()`；`remove` 缺桶 panic 原文；`dispose_cb_resource(seq, res, &mut fallback)` 命中/缺桶回退 + kind 3/4 日志；`SequentialDisposal` 按 §4 实现（panic 文本 `assertion failed: seq > self.last_retired`）。[0x1418c4100/0x1418c4260/0x1418c65a0/0x1418c7ae0]
- ✅ 兼容：`try_insert/try_remove/begin_command_bucket/drain_completed_sequence/immediate_empty_bucket` 保留 `Failure`（既有 managed 测试断言继续有效）；主入口 `insert/remove/begin/retire_in_order` 为原 panic 语义。
- ✅ `SharedResources`：`sequential: Mutex<SequentialDisposal>`（`drained_prefix` 为其无锁镜像）；新增 `retire_in_order`（原语义，允许乱序）；`drain_completed_sequence` 保持严格兼容。`take_out_dropped_waste` 新增 `strict` 开关：`*_checked` = strict（缺桶 `MissingBucket`，旧行为）；无后缀版 = 原语义（缺桶立即销毁 + kind 4）。
- ⚠️ `DisposalBucket` 原 0xf0B（多 Vec/容量字段，每槽 8B 动作载荷）：Rust 侧动作是闭包，用 `Vec<DestroyAction>`；8B 载荷的语义（推测为资源句柄/指针）**未逐项解码**，不伪造。
- ⚠️ 日志记录里 16B 名字参数（调用方 `&str`）的来源语义未证实，rebuild 写空名。
- ⚠️ `device.rs`（非我文件）仍用 `Mutex<CommandBufferDisposalBuckets>` 包一层（双锁无害）；其 `let mut disposal` 现产生 unused-mut 警告，需 device 归属人去掉 `mut`。

### 2 managed 同包偏差
- ✅ refcount AtomicI32 未回退。`+0xa50` 双语义矛盾已解（见事实 3）。
- ✅ 新增 `ResourceTag`（handle.rs）：Texture0/Buffer1/Swapchain2/(3=todo!)/RenderPass4/Sampler5/DescriptorSet6/Framebuffer7，`ResourceManager::new` 改用之。[0x142d060f0 + 7 个 dispose_cb_resource 地址]
- ⚠️ `RawManagedHandle` 24B / 控制块 `{seq,refcount,checksum,resource}` / `DisposedRawResourceHandle` 16B 布局：rebuild 仍是 Arc+ControlBlock 的安全等价体（行为对齐、布局不对齐），未改成 ABI 布局。
- ❌ `GpuOwnedState::Tracked` 真值：`GpuOwned::track_unwrap` 无 VA；扫 `.text` 中 `mov byte [reg+0x58], imm` 共 133 处（imm0:48/1:60/2:8/3:17），抽查 imm=2 的两处（0x140f88442/0x141afc337）上下文是别的结构，无法在不推断的前提下认定 `Cell::set(Tracked)`。仍为候选值 1。

### 3 instance.rs
- ✅ `apply_vulkan_blocklist(&GraphicsInstanceLayerAllowlist)`：入参类型改为白名单，Vendor→`VK_LAYER_{v}*`，Layer→原名，`,` 连接；18 次 set_var 链保持；debug 日志 "Selectively enabling…"/"[legacy] Force-disabling…"。[0x140c0a530] 测试 `env_chain_matches_original_write_order` 已更新并断言 `VK_LAYER_foo,VK_LAYER_NV*,other`。无外部调用方。
- ✅ `InstanceBuilder::build_with_loader`：默认 2 扩展（必须可用）、去重、`VK_EXT_layer_settings` 可用性探测追加。[0x140c08f10] lavapipe 实测通过。
- ⚠️ 未加 builder 的 validation 标志（原 `+0x18==1` → `VK_LAYER_KHRONOS_validation`）：现有 `layers` 显式列表等效，字段 ABI 未对齐；`+0x19` 追加 debug_utils 对应现有 `debug_markers()`。
- ⚠️ 错误串 "creating a Vulkan instance" 属 `GraphicsInstance::new`（init.rs，非我文件），instance.rs 的 Error 变体未带该上下文。
- ❌ `legacy_disable_layer`、`Instance::{extension_names,layer_names,create}`、`vulkan_debug_messenger` 回调：分析文档均标 I/inlinee、无 VA，无机器码可对；保持现状。

### 4 allocator / crash / format
- ✅ `GpuAllocator` 峰值包装（allocate 后采样、free 不采样）：既有 16 案例测试仍通过，本轮未改代码。
- ✅ crash：`log_gpu_debug_info` 补全日志行格式 `format_debug_record_line` / `log_debug_records` / `srcname_path`（info 级），既有 kind 1..6 消息串已与 0x142d06be0..cf0 pieces 核对一致（abort({}, {})、assertion failed、({} > / < / >= / <= {})、default `???`）。[0x1418c46c0]
- ⚠️ record_crash_marker 的 history slot=`id&4095` 等沿用既有 functional-validation，本轮未重读；`has_any_gpu_assertion_failed` 原短 buffer panic，rebuild 返回 Error（既有登记差异）。
- ✅ format 三入口：未改动（816 案例 0 mismatch 保持）。

### 5 lib.rs 日志 / leek
- ✅ `log_ring.rs` 重写为 `RingBuffer{data,cap,cursor}`，写/转储/init 行为逐条对应事实 5；`write`（严格）/`record`（宽松，未初始化丢弃）/`record_event`/`record_named`/`dump_log_vec`；`init_logging` 重复调用 panic 但全局已替换（原顺序）。[0x140be6a90/0x140be6b50/0x1418c1310/0x1418c1380/0x1418c1430]
- ✅ **谁写环**：`dispose_cb_resource`（kind 3/4）、`dispose_in_order`（kind 0x1e）——已接入 rebuild（`disposal.rs`、`resources.rs`）。`FUN_1418c1380` 的其余调用点（约 50+，raw-core/rg/managed）**未逐个枚举**，其余记录 kind 未恢复。
- ✅ `print_log_dump` = ICF thunk，保留为 stderr 输出。`dump_log` 原返回 Vec；公共 `dump_log(out)` 保留，新增 `dump_log_vec`。
- ⚠️ 谁调用 `dump_log`（原 `panic_reporter::dump_rhapsody_log`）属 tiny-glade 侧，本轮未接线。
- ✅ `leek.rs` 新增 `Name([u8;16])`（`From<&str>` 跳非 ASCII、满 16 停）。⚠️ Debug 取首个 0 前缀带引号，Display 形态未证实；`leek::Error{context,source}`/`wait_submits_done` 未恢复（无证据）。

### 6 U/半/✗ 落地
- ✅ 已闭合：D4 结构与语义、dispose_in_order、日志环布局/写者、blocklist 入参、log_gpu_debug_info 日志行、leek From、tag3、+0xa50。
- ⚠️ 保留候选：DisposalBucket 0xf0B 内部布局、托管句柄 ABI 布局、GpuOwnedState::Tracked、validation 标志。
- ❌ 读不出：ray_tracing（零地址 inlinee）、leek::Error、vulkan_debug_messenger 回调体。

## 改动文件
`crates/rhapsody/src/raw/disposal.rs`（重写）、`raw/instance.rs`、`raw/crash.rs`、`raw/managed/{resources.rs,handle.rs}`、`log_ring.rs`（重写）、`leek.rs`（新）、`lib.rs`；测试 `tests/state.rs`（同步 API）。（`lifecycle.rs/tracking.rs/swapchain.rs` 的 diff 非本轮所改。）

## 新增测试
`tests/disposal.rs`（4：重复 seq/缺桶 panic 原文、try_* 兼容、dispose_cb_resource 命中/回退、SequentialDisposal 乱序窗口）；`log_ring.rs` 2（环语义、全局+记录布局）；`leek.rs` 1；`instance.rs` 2（blocklist 白名单格式、默认扩展/layer_settings）；`crash.rs` 1（日志行/srcname 路径）。

## 遗留 / 需他人配合
- device.rs 去掉 `let mut disposal`（unused-mut 警告）；可选：把 `Mutex<CommandBufferDisposalBuckets>` 改为裸 `CommandBufferDisposalBuckets`（现在内部已有 RwLock）。
- vulkan.rs 的 `begin_command_bucket`/`drain_completed_sequence` 调用点保持兼容未改；若要贯彻原语义，应改用 `retire_in_order`（允许乱序、缺桶 panic）。
- 与销毁桶耦合的 `SharedResources::complete()` 旧兼容路径仍按 timeline 凭据排出，未接 SequentialDisposal。
