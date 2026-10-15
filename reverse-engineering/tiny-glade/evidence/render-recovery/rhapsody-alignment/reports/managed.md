# rhapsody managed 域（raw/managed）复现报告

日期 2026-10-15。图例：✅原版对齐[证据地址]　⚠️候选/假设　❌读不出（写明读了什么）。
基线：B patched EXE `tiny-glade.exe`，`/tmp/dis.py` objdump 字节级核对（本轮新读 ≈ 20 个函数/表）。

## 一、本轮新读出的机器码事实（含对前序文档的更正）

1. **`GpuOwnedState` 三个判别值全部读出：Owned=0 / Tracked=1 / Unused=2。**
   - Tracked=1：`GpuOwned<(), RawManagedHandle>` 的 `track_unwrap` 内联在 0x1415fb550 的循环里（步长 `shl r14,5` = 0x20 B，元素 = tracker 24 B + state@0x18）：`0x1415fb600 call RawManagedHandle::track` → `0x1415fb605 mov byte [rbp+8],1`（本地副本 +8 = 元素 +0x18）→ `0x1415fb638 call RawManagedHandle::drop`（tracker 析构）。
   - Unused=2：`ComputePass::bind_external_raw`（0x14096e540）未命中分支：`0x14096e658 mov byte [rbp-0x18],2` → `0x14096e65c call RawManagedHandle::drop`；资源 0x20 B @-0x50、tracker @-0x30、state @-0x18（= `GpuOwned<DescriptorSetBinding, RawManagedHandle>`，state@0x38，共 0x40 B）。
   - Owned=0：0x1403abd10 克隆/新建 `mov byte [dst+0x58],0`。旧候选名 `Discarded` 改为 `Unused`（`GpuOwned::drop_unused` 的内联宿主正是 `bind_external_raw`）。
   - `GpuOwnedInner` 的 drop（0x140161b50）整份拷贝 0x60 B 后只析构 tracker（0x140be24b0）与资源，**不读 state**：Drop 无状态断言（release 构建）。
2. **`ResourceType` 枚举名字表读出，tag 3 = `TextureView`**（不是“无效 tag”）：`RawManagedHandle as Debug`（0x140bd69a0）→ `ResourceType as Debug`（0x140bd6bd0）查 8 项表（长度表 0x142b10168 + i32 偏移表 0x142b101a8）：`Texture, Buffer, Swapchain, TextureView, RenderPass, Sampler, PersistentDescriptorSet, Framebuffer`。该类型存在但没有 store、没有 `dispose_cb_resource` 单态，跳表 tag 3 = `todo!()`（"not yet implemented"）。tag 8 = `Option<DisposedRawResourceHandle>::None` 的 niche（循环哨兵 `cmp cl,8; je done`）。
3. **`ManagedResources+0xa50` 的矛盾彻底消解，且更正前序 agent 的更正**：`dispose<T>`×7（0x1418af090..0x1418afa20）全部 `cmp ax,[rcx+0xa50]`（`rcx = &ManagedResources`，`ax = [退役项+0xc]` u16 device）——**`ManagedResources+0xa50` 就是 u16 `owning_device`**（7 个 store 占 `8+7×0x178 = 0xa50`，其后正好是该字段）。销毁桶（`RwLock<HashMap>`）在**另一个结构** `GraphicsDevice+0xa50`：`take_out_dropped_waste`（0x1418c0a70）真实签名是 `(&ManagedResources, &GraphicsDevice)`——`rdi=rcx` 取 Mutex 指针（`[rcx]`），`rsi=rdx`，`0x1418c0dd3 lea rdi,[rsi+0xa50]` 作为 `dispose_cb_resource` 首参，`0x1418c1011 mov rcx,rsi; call immediate_empty_bucket` 也是 `&GraphicsDevice`；`GraphicsDevice+0x648` 是指向 `ManagedResources` 的指针（`mov rcx,[rsi+0x648]`）。两个“+0xa50”分属两个结构，不矛盾。`dispose_in_order` 的 `rdi+0xa50` 同理是 `GraphicsDevice`。
   - 含义：原版销毁桶表属于**设备**，不属于 `ManagedResources`；当前代码把桶放进 `SharedResources`（见遗留）。
4. **store 次序与空闲链表头偏移**（`dispose<T>` 的 `lock cmpxchg [rsi+X]`）：texture 0x178、buffer 0x2f0、swapchain 0x468、render_pass 0x5e0、sampler 0x758、descriptor_set 0x8d0、framebuffer 0xa48 = `8 + 0x178·k + 0x170`，k = 0,1,2,3,4,5,6。即 store 数组顺序 = texture, buffer, swapchain, render_pass, sampler, descriptor_set, framebuffer（**按此顺序紧凑排列，不按 tag 编号留 3 的空位**）；`ManagedResources` = `{@0 Mutex 指针, @8 stores[7]×0x178, @0xa50 u16 owning_device}`。
5. **`dispose<T>` 完整判定序**（以 swapchain 0x1418af500 为例）：`[退役项+0xc] == [ManagedResources+0xa50]`（否则 panic）→ `[退役项+0xe] == 本类型 tag`（否则 panic）→ `[控制块+0xc] == [退役项+8]`（校验和，否则 panic）→ 把槽 +0x10 起的资源按值拷出（swapchain 0xc0 B）→ `T::dispose(&mut 拷贝, r8)`（0x1418ccdd0）→ 析构拷贝 → `refcount [ctl+8] == 0`（否则 panic）→ `inc [ctl+0xc]`（校验和 +1）→ `[ctl+0x10] = head; lock cmpxchg head ← ctl` 压入空闲链表（循环重试）。与代码里 `dispose_control_kind` 顺序一致。
6. **`RawManagedHandle` 全部方法字节核对**：
   - `Clone` 0x140bd6b00：`lock inc dword [ctl+8]` + 逐字段拷 24 B。
   - `PartialEq` 0x140bd6b30：比较 `[+8..+0x10)` 与 `[+0xf..+0x17)`（控制块指针 + 校验和/device/tag），**不比较 +0 队列指针与 +0x17 填充**。
   - `track` 0x140bd6a70：`[ctl+0xc] != [self+0x10]` → 静默返回；`[cmd+0] != 1` → panic（带格式化参数）；否则 `fetch_max([ctl+0], [cmd+8])`（CAS 循环 `cmova`）。
   - `Debug` 0x140bd69a0：`debug_struct("RawManagedHandle")` 4 个字段：`resource`@+8、`resource_checksum`@+0x10(u32)、`resource_type`@+0x16(u8, 见事实 2)、`device_id`（u16，newtype，Debug 0x140bd6b50）。⚠️ 第 4 个字段的值指针汇编里是 `mov r9,rsi`（= self+0），与 Clone/Eq/退役项都指向 +0x14 的 device 冲突，未解；+0x14 为 device 以 Clone/Eq/dispose 为准。
7. **栈桶 0xE0 vs `DisposalBucket` 0xF0 闭合**：`take_out_dropped_waste` 栈桶 10 个 24 B 槽从 `rbp-0x48` 起（`+0xc0 ... [rbp+0x98]=8、[rbp+0xa0]=0`，槽首址步长 0x18），末端 `rbp+0xa8`，总长 **0xF0** = 10×24，无尾部额外字段。“0xE0”是 Ghidra 栈变量边界的反编译假象。
8. **元素 `+0x0f` 写入方闭合**：没有写入方。`Drop`（0x140be24b0）只写 15 B（+0/+8/+0xc/+0xe）；排水循环（0x1418c0e4a `movzx edx,[rax+0xf]`）把这个未初始化填充字节当 `edx` 传给 `dispose_cb_resource`，是栈残值，无语义。
9. **swapchain 锁字编码：前序结论“非 stock parking_lot”是错的。** 读 `~/.cargo/registry/.../parking_lot-0.12.1/src/raw_rwlock.rs`：`WRITER_BIT = 0b1000`、`ONE_READER = 0b10000`、`READERS_MASK = !0b1111`——写锁 `0→8`、读锁 `+0x10`、快路径 `state & 8 == 0 && state < 0xff..f0` 与二进制逐位吻合（0x1418cb54f / 0x1418cb10d / 0x1418c65f7）；`lock_shared_slow` 的 `r9d=0x3b9aca00` 是 `Option<Instant>::None` 的 nanos niche（1e9），不是 1 秒超时。把 `RawMutex LOCKED_BIT=1` 当成了 RwLock 写位。**结论：就是 Cargo.lock 里的 stock parking_lot 0.12.1，无分叉改常量。** ✅
10. **`ManagedResource<T>` 槽布局 / 空闲链表**（读 allocate 0x1418ae7e0 + dispose 0x1418af500，印证前序闭合）：槽 = `{seq u64@0, refcount i32@8, checksum u32@0xc, resource@0x10}`，空闲时 `+0x10` 兼作 next；新槽元素 `{0,1,resource}` 24 B 起（`idx*24`）；头 CAS。

## 二、工作包逐项

### 1 剩余 U
- ✅ `GpuOwnedState` Tracked=1 / Unused=2 [0x1415fb605 / 0x14096e658]。`GpuOwned` 改 `#[repr(C)]`，字段序/偏移用 3 个原单态做断言测试（见测试）。
- ✅ 元素 `+0x0f`：无写入方（事实 8）。栈桶 0xE0/0xF0：同一个 0xF0（事实 7）。
- ✅ `RawManagedHandle` 24 B → `RawManagedHandleAbi`（`repr(C)`，`size_of==0x18`、偏移 0x08/0x10/0x14/0x16 编译期 `const assert`）；`DisposedRawResourceHandle` 16 B → `DisposedRawResourceHandleAbi`（`size 0x10`，偏移 0x08/0x0c/0x0e）；tag 7 变体 + `TextureView=3`。`Managed::abi()` 给出投影，`ManagedResources::pending_disposed_records()` 给出待排水的真实 16 B 项；`RawManagedHandleAbi::original_eq` 复刻原 `PartialEq` 的比较集合。
  - ⚠️ 这是**投影**而非把 `Managed<T>` 本身换成裸指针 ABI：拥有权仍是 `Arc<ManagedInner>`（安全 Rust 必须），指针字段以 `usize` 表示、不可解引用。
- ✅ 空闲链表：`Mutex<Vec<u32>>` 换成原版同构的**无锁 CAS 头 + 每槽 next**（`ResourceState::{pop_free,push_free}`，LIFO；高 32 位版本号防 ABA，Rust 不能依赖地址唯一）。[0x1418ae7e0 / 0x1418af590]
- ✅ 依赖：`append-only-vec`（git rev 4031d674…）早已在 Cargo.lock、`Cargo.toml` 里并被 `resources.rs` 使用，离线可编译；`arc-swap 1.7.1` 同。**无需新增依赖。**
- ⚠️ `ManagedResource<T>` 槽内 `resource` 仍不内联：原版把 T 按值放在槽 +0x10（并在 dispose 时 bitwise 拷出）；这里槽只放 `Arc<ControlBlock>` + `ArcSwapWeak<ManagedInner>`，资源在 `ManagedInner::resource: Arc<T>`。原因：`legacy` 收据路径需要类型擦除地持有控制块（`Weak<ControlBlock>`），且 `Managed<T>` 要 `Deref` 到 T 并跨线程共享。行为（refcount/checksum/seq/复用顺序）对齐，内存布局不对齐。`ControlBlock` 本身是 `repr(C)` `{seq@0, refcount i32@8, checksum@0xc}` 16 B，有偏移测试。

### 2 对照表“✗/半”
- ✅ `ManagedRef::get_tracked`：已是“获取并跟踪”（先 `track` 再交资源引用）[get_maybe_tracked 0x1418aeba0 内联 impl$17::track]。
- ✅ `TrackGpuLifetime::track`（0x140bd6a70）：判定序 = 校验和不等→静默跳过（`TrackOutcome::GenerationSkipped`）→ tag≠1 → 失败 → `fetch_max`。新增测试断言“校验和不等时连 tag 也不看”。⚠️ 原版 tag≠1 是 **panic**，这里仍返回 `Err(InvalidCommandState)`（保持既有 API；调用者 `?` 即可）。
- ✅ `RawManagedHandle::{Clone, PartialEq}`、`ManagedResourceChecksum::{Eq,Hash,invalidate}`（`Hash` 本轮补 derive）。✅ `Drop`（0x140be24b0）：refcount 归零推入退役队列（已有）；队列项现在带真实 16 B `DisposedRawResourceHandleAbi`。⚠️ `Debug` 字段名/顺序已核对（事实 6），Rust 侧 `Managed` 的 `Debug` 仍是自有格式（`Managed { key, tracked_sequence }`），未改成原格式，因为 `ResourceTag::name()` 已提供原名字表，可按需接线。
- ✅ `GpuOwned::{new, track_unwrap, mark_unused, map_tracker, drop_unused}`、3 个原单态（`<(),ManagedHandle<ManagedBuffer>>`、`<DSB,RawManagedHandle>`、`<[DSB;2],RawManagedHandle>`）：泛型 `GpuOwned<R,Tracker>` 已能表达，布局测试覆盖前两种 tracker 形态。`drop_unused` 新增（置 Unused 后析构，不 track）。
- ⚠️ `GpuOwned::map_tracker`：原版内联 `dynamic_constants::into_*_binding::closure` + `get_vk_buffer`（把 tracker 接进描述符绑定）——这是 country-core/descriptor 侧的调用，rebuild 保留“换 tracker、保留 state”的通用语义。`drop_duplicate`：内联宿主 `bind_texture_to_draw_builder`（0x1408ee800 / 0x140908610…）里没找到写 state 的立即数，**读不出**，未实现。
- ✅ `ManagedHandle<T>` 泛型 + `into_raw_gpu_owned/into_gpu_owned` 都走 `GpuOwned::new`（原 PDB 线 232/237，已有，未改）。

### 3 ManagedSwapchain / SwapchainInner（只核对，未改 raw 下文件）
- ✅ 锁字编码：stock parking_lot 0.12.1（事实 9），**不是 Bevy 分叉常量**。
- ⚠️ 差异（不在我文件内，仅登记）：原版 `ManagedSwapchain = RwLock<SwapchainInner>`，`SwapchainInner` 持有 desc 5×u64@8、状态@0x30、租约表@0x60（0x38 B/项）、image count@0x68、acquire/present/retire 状态字节@0xa8/0xb9/0xbb；当前 `raw/swapchain.rs::Swapchain` 用两把 `parking_lot::Mutex`（`requested`、`slots`），`managed/swapchain.rs::SwapchainInner` 只是 `Arc<Swapchain>` 薄壳。`ACQUIRE_TIMEOUT_NS = 3e9` 与原版 acquire 的 3 s 一致。
- ❌ `set_fullscreen_exclusive`：原符号只有 inlinee 声明，无任何编译体/调用，保持“不暴露”（无证据）。
- ❌ SwapchainInner 租约表 0x38 B/项各字段、状态字节含义：本轮未重读 acquire/present/retire 反编译，沿用 ALGORITHMS §1.2 的 O 级观察，不在 managed 文件里伪造。

### 4 ManagedTexture 视图缓存 / dispose×7 / store 顺序
- ✅ 视图缓存：`Managed<Texture>::view(Option<&TextureViewDesc>)`——`None` 取默认视图（`OnceLock`，对应原 `+0x70` 缓存直接返回），`Some` 在纹理自带的 `RwLock<HashMap<描述符, Arc<TextureView>>>` 查找，未命中才 `create_vk_image_view`（0x140bdcba0）并登记；`image_view(desc)`/`GraphicsDevice::get_vk_image_view` 经它走缓存；无效范围不入缓存。[0x140bdc5d0]
  - ⚠️ 原版缓存实现是位图@+0x40/掩码@+0x48 + 0x48 B 槽 + `RwLock@+0x38`，按描述符**校验和**（`FUN_1418bd030`）SIMD 查找；校验和算法、槽字段、`bVar37&1` 判定未读出，这里用完整描述符作键（碰撞时更严格）。缓存挂在 `ManagedInner::views`（仅纹理使用，其他类型恒空）。
  - ❌ `ManagedTexture::new` 的全局纹理 id 计数器（`FUN_140d4d740`，`{flag, cnt@8, dev@0x10}`）：`Managed<Texture>` 是 `Managed<T>` 别名，无处挂 id；raw `Texture` 非本人文件。未实现。
- ✅ `DisposeManagedResource::dispose<T>`×7 的判定序（事实 5）与 7 个 `lock cmpxchg` 头偏移（事实 4）；store 顺序 = 事实 4 的紧凑顺序；tag 映射 texture=0/buffer=1/swapchain=2/**TextureView=3(todo!)**/render_pass=4/sampler=5/descriptor_set=6/framebuffer=7（跳表 0x142d060f0）。`take_out_dropped_waste` 遇到 tag 3 现在 `todo!()`（与原版一致，测试 `tag_3_retirement_hits_todo...`）。`ManagedResources::create_*` 本身在 `raw/device_api.rs`（非我文件）。

### 5 ALGORITHMS §4 / REVIEW-UNEXPLORED / DEEP-SUMMARY 的 U 落地
| U | 状态 |
|---|---|
| §4#1 槽布局 + 空闲链表 | ✅ 布局/复用算法落码（无锁 CAS，LIFO）；⚠️ 资源不内联（见上） |
| §4#2 `DisposedRawResourceHandle` 16 B + 分派表 | ✅ `DisposedRawResourceHandleAbi` + `ResourceTag`(8 变体，3=todo!) |
| §4#3 meta 原子字段 | ✅ `repr(C)` `{u64,i32,u32}` 偏移测试 |
| §4#4 `GpuOwnedInner` 字段排布 | ✅ `repr(C)` `{R, Tracker, state}`，3 种实例偏移测试 |
| §4#5 swapchain 字段语义 | ❌/⚠️ 见 §3，未重读，非本人文件 |
| §4#6 swapchain 锁字 | ✅ 就是 stock 0.12.1（更正前序结论） |
| §4#7 `set_fullscreen_exclusive` | ❌ 无证据，不实现 |
| §4#8 线 14/23/32 三类型 Eq/Debug/Hash | ⚠️ `impl$36/44/51::fmt` 地址已知；其中 0xbd6bd0 = `ResourceType as Debug`（事实 2）、0xbd6b50 = `DeviceId` newtype Debug；其余两个类型身份未再追 |
| §4#9 impl$6..12 与 tag | ✅ 紧凑 store 顺序 + 跳表（事实 4） |
| §4#10 `ManagedResources::new` 设备入参 | ✅ `owning_device` u16@0xa50（事实 3）；入参来源/全局纹理 id 计数器归属 ❌ 未读 |
| §4#11 视图缓存细节 | ❌ 校验和/位图/槽字段（见 §4）|
| 栈桶 0xE0 vs 0xF0 | ✅ 同为 0xF0 |
| 元素 +0x0f | ✅ 无写入方 |
| +0xa50 双语义 | ✅ 两个结构（更正前序） |
| GpuOwnedState 值 | ✅ 0/1/2 |

## 三、改动文件
- `crates/rhapsody/src/raw/managed/handle.rs`：`ResourceTag` 补 `TextureView=3`/`name`/`is_disposable`；新增 `RawManagedHandleAbi`、`DisposedRawResourceHandleAbi`、`Managed::abi`；`GpuOwnedState::{Tracked=1,Unused=2}`；`GpuOwned` `repr(C)` + `drop_unused`；`ManagedResourceChecksum: Hash`；`ManagedInner::views`。
- `.../resources.rs`：无锁空闲链表（`free_head` + `Slot::next_free`）；`Dropped::abi` + tag 3 `todo!()`；`pending_disposed_records`。
- `.../texture.rs`：视图缓存（`ViewCache`、`view(Option)`、`cached_view_count`）。
- `.../tracking.rs`：`ControlBlock` 文档 + 偏移/判定序测试。
- 新增 `crates/rhapsody/tests/managed_alignment.rs`。未改 `render_pass/framebuffer/descriptor_set/sampler.rs`、`raw/` 其它文件、`tests/managed.rs`、`original_oracle*.rs`。公开 API 全部向后兼容（仅 `GpuOwnedState::Discarded` 改名 `Unused`，全仓库无外部引用）。

## 四、新增测试
- `handle.rs`：`state_discriminants_are_read_from_machine_code`、`gpu_owned_layout_matches_original_instances`（3 个实例的 offset/size）、`drop_unused_marks_unused_and_never_tracks`、`resource_tag_matches_original_resource_type_table`、`abi_projection_layout_and_original_equality`。
- `tracking.rs`：`control_block_header_matches_original_slot_layout`、`track_follows_original_decision_order`。
- `resources.rs`：`pending_records_are_original_16_byte_items_in_enqueue_order`、`tag_3_retirement_hits_todo_like_the_original_jump_table`。
- `tests/managed_alignment.rs`：`free_list_is_lifo...`、`free_list_survives_concurrent_churn...`（6 线程）、`real_buffer_retirement_is_a_16_byte_record...`、`texture_view_cache_returns_the_same_view...`（lavapipe）。
- 验证：见文末“验证结果”。

## 五、遗留 / 需他人配合
1. **销毁桶归属**：原版桶表在 `GraphicsDevice+0xa50`，`take_out_dropped_waste(&ManagedResources, &GraphicsDevice)`；当前 `SharedResources.buckets` 在 managed 内。接线方（device.rs 归属人）若要对齐，应把 `CommandBufferDisposalBuckets` 放进设备并把 `take_out_dropped_waste` 加设备参数；本轮保持 API 兼容未动。
2. swapchain：`raw/swapchain.rs` 两把 Mutex 与原 `RwLock<SwapchainInner>` 结构不同；租约表/状态字节未重读。
3. `ManagedTexture` 全局 id 计数器、视图缓存校验和算法、`drop_duplicate` 的 state 值、`RawManagedHandle::Debug` 第 4 字段指针异常：读不出/未读。
4. tag≠1 的 `track` 原版是 panic，这里 `Err`；`Managed::Debug` 未用原版格式。

## 验证结果
`cargo test --offline -p rhapsody -p tiny-glade -p country-core --all-features`（lavapipe）= **1559 passed / 0 failed**；`original_oracle*`、`tests/managed.rs`（11）未回归；`managed_alignment`（4）通过。
