# rhapsody managed 域算法级深挖（raw/managed：GPU 资源托管生命周期/句柄）

- 日期：2026-10-15。基线：B patched EXE；机器码/PDB 权威。
- 域：`src/raw/managed/{buffer,descriptor_set,framebuffer,handle,render_pass,resources,sampler,swapchain,texture}.rs`，42 compiled / 12 有地址（域清单），S_PUB32 再解析出 17 个带地址符号 + 8 个 trait 实现地址。
- 数据：`domains/managed.json`、`decomp/decomp-<va>.txt`（12 个全成功，无 NotFound）、`skeleton/B/crates/rhapsody/src/raw/managed/*.rs`（PDB 函数元数据+调用注解+结构关系）、`evidence/render-architecture/public-symbols.json`（S_PUB32 demangled+va）、`reconstruction/rhapsody-rebuild/src/raw/managed/**`。
- 标注：**O**=机器码/PDB 实测；**I**=推断；**U**=未知。
- 重要前置认知（CALL-CHAIN §4 #13 已验）：原 `ManagedHandle` 是**泛型** `ManagedHandle<T:Send+Sync>`；country-core 合约是非泛型 u64 包装。

---

## 1. 模块总览

### 1.1 文件→职责

| 文件 | 职责 | 关键类型 |
|---|---|---|
| `resources.rs` | 资源托管核心：7 类资源 store、退役队列、跨命令缓冲销毁桶、设备归属 | `ManagedResources`、`ManagedResourceStore<T>`、`ManagedResource<T>`、`ManagedResourceMeta`、trait `AccessManagedResource`/`DisposeManagedResource` |
| `handle.rs` | 句柄全族：CPU/GPU 双跟踪、所有权转移、绑定载荷 | `ManagedHandle<T>`、`GpuOwned<T,Tracker>`、`GpuOwnedInner<T,Tracker>`、`ManagedRef<'a,T>`、`RawManagedHandle`、`DisposedRawResourceHandle`、`ManagedResourceChecksum`、trait `TrackGpuLifetime`/`ManagedResourceType` |
| `swapchain.rs` | 交换链托管（RwLock 保护） | `ManagedSwapchain`、`SwapchainInner`（含 leek 超时获取/呈现/退役） |
| `texture.rs` | 图像托管 + 图像视图缓存 | `ManagedTexture`、视图缓存（位图+0x48B 槽）、`create_vk_image_view` |
| `buffer.rs`/`sampler.rs`/`framebuffer.rs`/`render_pass.rs`/`descriptor_set.rs` | 各类型托管包装 | `ManagedBuffer`/`ManagedSampler`/`ManagedFramebuffer`/`ManagedRenderPass`/`ManagedPersistentDescriptorSet`（`ManagedResourceType::dispose` + `ManagedResources::create_*`） |

### 1.2 主要数据结构与布局

**ManagedResources**（O，`ManagedResources::new` 0x1418c08e0）
```
@0x00: *mut 0x20B 块 —— parking_lot::Mutex<Vec<DisposedRawResourceHandle>> 头
       （O：0x140161c30 = drop_in_place<MutexGuard<RawMutex, Vec<DisposedRawResourceHandle>>>；
         头字节 0→1 锁语义 = parking_lot RawMutex state；块内 {lock u8@0, [8]u64, [0x10]u64(置8), [0x18]u64}）
@0x08: 7 × ManagedResourceStore<T>，每 0x178 B（O：new 内 7×0x178 memset；impl$1::default×7 内联）
@0xa50: u16 owning_device（O：new 写 param_2；get_maybe_tracked 读 self+0xa50 比对）
```
- 7 类 store 顺序（I，rebuild 假设：texture=0/buffer=1/swapchain=2/render_pass=4/sampler=5/descriptor_set=6/framebuffer=7；tag 证据见 §1.4）。

**ManagedResourceStore<T>**（大小 0x178B O）
- 槽位表 = **`append_only_vec::AppendOnlyVec<ManagedResource<T>>`**（O：`impl$6::allocate`@0xbdbd50 直接调 `AppendOnlyVec<T>::push`/`indices`/`len`/`index` + `AtomicPtr::compare_exchange_weak`；`texture.rs` 结构区 `AppendOnlyVec::push<ManagedResource<ManagedTexture>>`）。依赖 = h3r2tic/append-only-vec **0.1.2，git rev 4031d674025f4765ad8fdd7fd41ef3bc7bea65e8**（O）。
- 空闲链表/复用索引：`append_only_vec::indices` + `impl$6::allocate`（线369）/`impl$7::release`（线383）等 7 对 per-type allocate/release 暗示复用小索引；**内部布局 U**（rebuild 用 `Mutex<Vec<u32>>` 候选）。

**ManagedResource<T>**（槽位元素）内部布局 U。含句柄 + 元数据（generation/checksum/seq）。rebuild 候选 `Slot{control: Arc<ControlBlock>, handle: ArcSwapWeak<ManagedInner<T>>}`。

**ManagedResourceMeta / 控制块**（I→O 混合）
- O：`track` 路径 `ManagedResourceMeta::checksum`(线43) + `AtomicU64::fetch_max`（`RawManagedHandle::track` @0xbd6a70 内联）；`dispose` 路径 `impl$32/40/47::eq`（线14/23/32 三个 Eq 类型）+ `arc_swap::{Debt::pay_all, ref_cnt::RefCnt::dec, wait_for_readers}`（O，arc-swap **1.7.1**）+ `Arc::drop_slow/from_raw`。
- 结论：原资源控制块是**原子字段集合**（seq 用 `fetch_max`、refcount 用 `AtomicI32`、校验和递增），rebuild `ControlBlock{last_sequence:AtomicU64, references:AtomicU32, resource_checksum:AtomicU32}` 是对照物；字段精确顺序/校验和增量语义 U。

**RawManagedHandle**（O 行为，字段布局 U）
- `TrackGpuLifetime::track` @0xbd6a70（compiled，线96）：`CommandBufferSeqTracker::track`（raw::command_buffer 线54）+ `ManagedResourceMeta::checksum` + `impl$40::eq` + `AtomicU64::fetch_max`。
- `Clone` @0xbd6b00（线109）：**`AtomicI32::fetch_add`**（O —— 原 refcount 是 AtomicI32，rebuild 是 AtomicU32，类型半对）。
- `Drop` @0xbe24b0（线127）：`lock_api::Mutex::lock`（parking_lot）+ `AtomicI32::fetch_sub` + `AtomicU8::compare_exchange` + `Vec::push` → **引用归零时把 `DisposedRawResourceHandle` 推入 `Mutex<Vec<...>>` 退役队列**（O）。
- `PartialEq` @0xbd6b30（线152）、`Debug::fmt` @0xbd69a0（线85，S_PUB32 `RawManagedHandle as Debug`）。

**DisposedRawResourceHandle**（O 元素；布局 I/U）
- 元素 16B（O：take_out_dropped_waste 迭代 `count×16B`），标签字节 @0xe（O：`switch(*(u8*)(item+0xe))`，case≥9，Ghidra 截断）。可能是 7 变体类型擦除枚举（I）。

**ManagedHandle<T: Send+Sync>**（O 泛型）
- `into_raw_gpu_owned`(线232) / `into_gpu_owned`(线237) → 均内联 `GpuOwned::new`(线469)。
- `Debug::fmt` @0x1403ac2d0 / 0x141621b00（S_PUB32 `ManagedHandle<T> as Debug`）。

**GpuOwned<T, Tracker> / GpuOwnedInner<T, Tracker>**（O，泛型两参数）
- 实际单态（O）：`GpuOwned<DescriptorSetBinding, RawManagedHandle>`（出现在 `Vec<(BindingIndex, GpuOwned<...>)>` / `hashbrown RawTableInner`，即 bind_descriptor_set 的绑定载荷）；`GpuOwned<[DescriptorSetBinding;2], RawManagedHandle>`（数组2绑定）；`GpuOwned<(), ManagedHandle<ManagedBuffer>>`（`Vec<...>`/`IntoIter<...>`）。
- `GpuOwnedInner<T,Tracker>::clone` @0x1403abd10 / 0x141620a10（compiled，线379 = `impl$21::clone<[DSB;2], RawManagedHandle>`）：`core::array::clone`/`try_from_fn`（拷资源数组）+ `RawManagedHandle::clone`（0x140bd6b00）+ unwind 时 `RawManagedHandle::drop`（0x140be24b0）。
- `GpuOwned::track_unwrap`(线424)：`impl$22::track`(线397：`Cell::set` 置已跟踪标志 + `impl$17::track`→seq tracker) + `impl$25::drop`(线550) + `ManuallyDrop::take`。
- `GpuOwned::mark_unused`(线447)：`Cell::set`（置未用标志）。
- `GpuOwned::map_tracker`(线507)：内联 `raw::dynamic_constants::{into_uniform_buffer_binding[_and_data], into_uniform_and_storage_buffer_binding}::closure` + `GraphicsDevice::get_vk_buffer` → **map_tracker 用于把 tracker 资源接入描述符绑定**（I）。
- `impl$25::drop`(线550)：GpuOwnedInner 析构（ManuallyDrop::take + drop 资源+tracker）。

**ManagedRef<'a,T>**（O）
- new/as_owner/into_owner/as_managed_handle/get_untracked（线302–320，无依赖=纯拷贝）。
- `get_tracked`(线327)：内联 `impl$17::track`(线250)→`CommandBufferSeqTracker::track` + `fetch_max` —— **原 get_tracked 是“获取并跟踪”动作，不是返回序号**（rebuild 返回 `u64 tracked_sequence`，语义偏差）。

**ManagedResourceChecksum**（O）
- `invalidate`(线27) + `Eq/Debug/Hash`(线32，`impl$47/51/52`，`impl$51::fmt`@0xbd6bd0)。rebuild 包装 u32 ✓。

**ManagedSwapchain**（O）= `lock_api::RwLock<parking_lot::raw_rwlock::RawRwLock, SwapchainInner>`
- O：`ManagedSwapchain::on_focus`(线64) 内联 `RwLock::write` + `SwapchainInner::on_focus` + `RawRwLock::{lock_exclusive,unlock_exclusive}`；0x1400412d0 = drop_in_place<RwLockWriteGuard<RawRwLock,SwapchainInner>>。
- 锁字观察（O，decomp）：写锁 0→8（bit3=写者），读锁每读者 +0x10，末读者唤醒；**与 vanilla parking_lot 0.12.1 常量（写1/读+2 或 +4）不符 → 可能 Bevy 分叉改常量；精确编码 U**。

**SwapchainInner**（O 部分字段，语义 I）
```
@0x00: RwLock state 字
@0x08..0x30: 当前 desc 5×u64（O：desc() 拷这 5 个 u64 成 40B 输出；I：format@8/colorspace@0x10/presentMode@0x18/extent_w@0x20/extent_h@0x28）
@0x30: u32 状态枚举（2 = 已创建；O：resize/present/acquire 都判 [6]==2）
@0x34..0x50: 旧 desc 槽（O：state==2 时旋转 current→old）
@0x58: ? （O：view 相关非零标志）
@0x60: *mut 租约表（0x38B/项，O：acquire 按 image_index×0x38 读）
@0x68: image count u64（O：acquire 模运算）
@0x78: *mut 获取记录数组（8B/项，O：acquire 读 uVar27=*(base+idx*8)）
@0x80: u64 界（O：acquire 界检查 panic）
@0x90: *mut 图像记录数组（8B/项，O：acquire 读 plStack_70）
@0x98: u64 界（O：界检查）
@0xa0: swapchain handle u64（O：acquire 传 vkAcquireNext；retire 返回）
@0xa8: 状态字节 2/3/4（O：acquire/present/retire 判）
@0xb0: next image index u64（O：acquire 推进 %image_count）
@0xb8/0xb9/0xbb: 状态/回调门字节（O：retire 读 byte[0xb9]，present 读 byte[0xb9]）
@0xc0/0xc1: 字节（O：new 写）
```

**GraphicsDevice**（managed 相关偏移，O）：`+0x140`/`+0x5c8`（create_texture/view 传参）、vtable `+0x6c8`（acquire_next_image）+ arg `+0x6f0`、Mutex `+0x718`（present 用 0→1 字节锁）、主队列 `+0x730`、队列表 `+0x7e8`（0xb0B/项）+ 计数 `+0x7f0`、提交 `+0x800`、退役回调 vtable `+0x858` + arg `+0x868` + 门 `+0x850`。

### 1.3 处理链（函数调用图）

```
分配: ManagedResources::create_* (buffer.rs:58 等 inline) 
   → ManagedResourceStore<T>::allocate<T> (0x1418ae7e0..0x1418aead0 ×4 compiled; 泛型线71)
   → AppendOnlyVec::push<ManagedResource<T>> / 复用空闲索引 (impl$6::allocate 线369)
   → 返回 ManagedHandle<T>

跟踪: RawManagedHandle::track (TrackGpuLifetime @0xbd6a70)
   → CommandBufferSeqTracker::track + ManagedResourceMeta::checksum + AtomicU64::fetch_max

绑定: GpuOwned<T,Tracker>（track_unwrap/map_tracker）→ dynamic_constants closure + get_vk_buffer

退役: ManagedHandle/RawManagedHandle::drop (@0xbe24b0)
   → AtomicI32::fetch_sub 引用计数归零
   → Mutex<Vec<DisposedRawResourceHandle>> push（parking_lot @0x140161c30）
   → [GPU 边界] ManagedResources::take_out_dropped_waste (0x1418c0a70)
   → 按 16B 项 tag@0xe 分派 → CommandBufferDisposalBuckets::dispose_cb_resource ×7 (0x1418c5b00..0x1418c7590)
   → 销毁桶按命令缓冲序号组织；序号时间线完成后回收

外部 API: DisposeManagedResource::dispose<T> ×7 (0x1418af090..0x1418af800)
   → impl$N::release (线373..433 per-type) + ManagedResourceType::dispose
     (buffer@0x1418afb90 / texture@0x140bdbe40 / swapchain@0x1418ccdd0)
   → arc_swap 读者排空 (wait_for_readers) + Arc 释放

交换链: ManagedSwapchain::new (0x1418ca030, raw::device::GraphicsDevice::create_swapchain @0xbfd7b2 调)
   → RwLock 写保护 SwapchainInner；acquire_next(0x1418cb520) / present(0x1418cc1b0) / retire(0x1418ccb50)
   → raw::swapchain::Swapchain::extent/acquire_next_image/mark_stale 转发到 managed
     (O：extent@0xc0eac4→desc@0x18cb100、acquire_next_image@0xc0ed21→acquire_next@0x18cb520)

纹理: ManagedTexture::new(0x140bdc2d0)/view(0x140bdc5d0)
   → FUN_140bdcba0 = create_texture ≙ create_vk_image_view (ICF 折叠，S_PUB32 @0x140bdcba0)
   → 全局纹理 id 计数器 (FUN_140d4d740 全局 {flag,u64 cnt@8,u64 dev@0x10})
   → 视图缓存：位图@+0x40(ptr)/+0x48(mask)、0x48B 槽、RwLock@+0x38
```

### 1.4 资源类型 tag（O 边界 + I 映射）

- O：7 个 `CommandBufferDisposalBuckets::dispose_cb_resource` 单态（0x1418c5b00/0x1418c5b00..0x1418c7590）+ 7 个 `DisposeManagedResource::dispose` 地址。
- 具体 tag→类型对应 U（rebuild 依 `TypeId` 给 texture=0/buffer=1/swapchain=2/render_pass=4/sampler=5/descriptor_set=6/framebuffer=7，候选）。

---

## 2. 逐函数表（compiled_procedure 全覆盖；关键 inline 单列）

地址均 S_PUB32 或骨架 `编译实例 RVAs` 解析。

### resources.rs

| 函数（骨架名） | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `ManagedResources::new` | 0x1418c08e0 | 构造 | 7×0x178 清零=7 store；分配 0x20B Mutex<Vec<DisposedRawResourceHandle>> 头；写 owning_device u16@0xa50 | O | decomp-0x1418c08e0；骨架线159 callee 0x140613c10/142923c20×8/impl$1::default×7 |
| `ManagedResources::owning_device` | — (inline) | 读设备 | 返回设备（u16 或宿主） | O | 骨架线173；get_maybe_tracked 用 +0xa50 |
| `ManagedResources::take_out_dropped_waste` | 0x1418c0a70 | 退役排水 | 锁 Mutex 取空 Vec；逐 16B 项按 tag@0xe 分派到 dispose_cb_resource×7；FUN_140bf92f0/FUN_140bdfcf0；dealloc 桶；内嵌 log（文件串+“not yet implemented”） | O | decomp-0x1418c0a70；callee 0x1418c5b00..0x1418c7590 |
| `AccessManagedResource::get_maybe_tracked<ManagedResources,ManagedBuffer>` | 0x1418aeba0 | 取+跟踪 | 设备比对（panic）；tracked 标志字节；已跟踪→get_tracked（内含 meta checksum+fetch_max CAS），未跟踪→get | O | decomp-0x1418aeba0；骨架线277 内联 impl$4::get/get_tracked |
| `AccessManagedResource::get_maybe_tracked`（trait 声明） | — (inlinee_declaration) | trait 默认 | 无体证据 | I | 骨架线277 |
| `ManagedResourceStore::allocate<T>` 泛型 | — (inline) | 分配入口 | 线71；复用空闲或 push 新槽 | O | 骨架线75/2433 |
| `ManagedResourceStore::allocate<ManagedBuffer/…DescriptorSet/…Framebuffer/…RenderPass>` | 0x1418ae7e0/0x1418ae8a0/0x1418ae9f0/0x1418aead0 | 4 单态 | 泛型 allocate 的单态 | O | S_PUB32；managed.json 4×compiled |
| `ManagedResourceStore::release` | — (inline) | 槽回收 | 线119；回空闲链表 | I | 骨架线450/1750..2392（dispose 链内联引用） |
| `ManagedResourceMeta::checksum` | — (inline) | 读校验和 | 线43 | O | track 内联 |
| `impl$1::default`（store Default） | — (inline) | 默认 | 线62，×7（每 store） | O | new 内联×7 |
| `impl$3::throw_out_managed_resource::closure$0` | — (inline) | 弃资源闭包 | 线200；含 CommandBufferSeqTracker::current、debug_name、DisposedRawResourceHandle::resource | O | 骨架线649 callee |
| `impl$4::create`/`get`/`get_tracked` | — (inline) | store 访问 | 线298/315/329；get_tracked 单态×3（buffer@?/dset/render_pass，S_PUB32 ManagedResources::get_tracked @0x1418aed30/0xaee50/0xaef70） | O | 骨架线1375..1591；S_PUB32 |
| `impl$5::dispose<T>` ×7 | 0x1418af090/0x1418af220/0x1418af390/0x1418af500/0x1418af690/0x1418afa20/0x1418af800 | 外部位弃 | `DisposeManagedResource`：type-dispose + store release + arc_swap 读者排空 + Arc drop + 3 个 Eq | O | S_PUB32；骨架线1592 callee（0x140be0100 drop_in_place、0x1418afb90、arc_swap 族、impl$7::release） |
| `impl$6::allocate` | 0xbdbd50 | 分配实现 | 线369；AppendOnlyVec::push/indices/len/index + AtomicPtr CAS + panic | O | 骨架线2402 callee |
| `impl$6..impl$12::{allocate,release}` | — (inline) | 7 对 per-type | 线369..433 | I | 骨架线2402..2941 |
| `ManagedResources::throw_out_managed_resource` | — (inline) | 弃资源 | 线177，×14 实例 | O | 骨架线604 |

### handle.rs

| 函数（骨架名） | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$3::fmt`（RawManagedHandle Debug） | 0x140bd69a0 | Debug | — | O | S_PUB32 |
| `impl$4::track`（RawManagedHandle as TrackGpuLifetime） | 0x140bd6a70 | 跟踪 | CommandBufferSeqTracker::track + meta checksum + impl$40::eq + AtomicU64::fetch_max | O | S_PUB32；骨架线295 |
| `impl$5::clone`（RawManagedHandle Clone） | 0x140bd6b00 | 克隆 | AtomicI32::fetch_add（refcount++） | O | 骨架线343 |
| `impl$6::drop`（RawManagedHandle Drop） | 0x140be24b0 | 析构 | Mutex lock + fetch_sub + AtomicU8 CAS + Vec::push（退役入队） | O | 骨架线369 |
| `impl$7::eq`（RawManagedHandle PartialEq） | 0x140bd6b30 | 相等 | 比较（元数据/索引） | O | S_PUB32 |
| `impl$15::fmt<ManagedTexture>`（ManagedHandle Debug） | 0x1403ac2d0 / 0x141621b00 | Debug | — | O | S_PUB32 |
| `impl$21::clone<[DSB;2],RawManagedHandle>`（GpuOwnedInner Clone） | 0x1403abd10 / 0x141620a10 | 克隆 | 数组 try_from_fn + tracker clone + unwind drop | O | S_PUB32；骨架线1093 |
| `impl$57::fmt` | 0x1408fcd10 | Debug | — | O | 骨架线1039 |
| `impl$36/44/51::fmt`（线14/23/32 三类型 Debug） | 0xbd6b50/0xbd6b90/0xbd6bd0 | Debug | — | O（地址）/U（类型身份） | 骨架线56/124/210 |
| `ManagedHandle::into_raw_gpu_owned`/`into_gpu_owned` | — (inline) | 转 GpuOwned | GpuOwned::new | O | 骨架线747/769 |
| `impl$17::track`（线250） | — (inline) | 跟踪 | impl$4::track + fetch_max | O | 骨架线791 |
| `impl$18::clone`（线256，ManagedHandle/GpuOwned 克隆） | — (inline) | 克隆 | impl$5::clone | O | 骨架线840 |
| `ManagedRef::{new,as_owner,into_owner,as_managed_handle,get_untracked}` | — (inline) | 借用投影 | 无依赖拷贝 | O | 骨架线870..957 |
| `ManagedRef::get_tracked` | — (inline) | 取+跟踪 | impl$17::track（真的跟踪） | O | 骨架线960 |
| `impl$22::track`（线397，GpuOwned tracker 标志） | — (inline) | 置跟踪标志 | Cell::set/Cell::replace | O | 骨架线1179 |
| `GpuOwned::{track_unwrap,mark_unused,drop_unused,drop_duplicate,new,map,map_tracker}` | — (inline) | GpuOwned 操作 | 见 §1.2；map_tracker→dynamic_constants+get_vk_buffer | O | 骨架线1217..1476 |
| `impl$25::drop`（线550，GpuOwnedInner Drop） | — (inline) | 析构 | ManuallyDrop::take + drop 资源+tracker | O | 骨架线1479 |
| `impl$28::as_mut`（线600） | — (inline) | 可变访问 | 无依赖 | O | 骨架线1536 |

### swapchain.rs

| 函数（骨架名） | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `ManagedSwapchain::new` | 0x1418ca030 | 创建 | ash 0.38 surface caps/present modes/create_swapchain/get_swapchain_images（设备 vtable [0xca][4]/[0xca][8]/[0xcb][0x1a]/[0xd6]/[0xde]/[0x10a]）；格式/呈现模式扫描；建 SwapchainInner 各字段 | O | decomp-0x1418ca030；骨架线43 callee |
| `ManagedSwapchain::on_focus` | — (inline) | 焦点 | RwLock::write + SwapchainInner::on_focus | O | 骨架线424 |
| `ManagedSwapchain::desc` | 0x1418cb100 | 读 desc | 读锁；拷 SwapchainInner+8..+0x30 的 5 u64 → 40B desc | O | decomp-0x1418cb100；raw Swapchain::extent@0xc0eac4 调 |
| `ManagedSwapchain::resize` | 0x1418cb1b0 | 改尺寸 | 写锁；state==2 旋转旧 desc；写 extent | O | decomp-0x1418cb1b0 |
| `ManagedSwapchain::set_vsync` | 0x1418cb370 | 改 vsync | 写锁；置 vsync 字节；state==2 旋转 | O | decomp-0x1418cb370 |
| `ManagedSwapchain::mark_stale` | — (inline) | 置陈旧 | 转发 SwapchainInner | O | 骨架线4973 内联宿主 |
| `ManagedSwapchain::acquire_next` | 0x1418cb520 | 获取 | 写锁；state@0xa8==2 且 state@0x30==2：设备 vtable[0x6c8] acquire_next_image(handle@0xa0, 3s)；成功→租约表@0x60（0x38B）取记录、lease+8 refcount++（AtomicI32）、next-index@0xb0 %image_count@0x68 推进、返回 {idx,fence,lease}；NOT_READY/TIMEOUT(1/2)→Err 旋转；OUT_OF_DATE(0xc4653214)→Err；其它→Err(Other)；未创建/陈旧→Err | O | decomp-0x1418cb520；骨架线1033 |
| `ManagedSwapchain::present` | 0x1418cc1b0 | 呈现 | 写锁；state 校验；设备 Mutex@0x718（0→1）；队列选择（主队列+0x730 或队列表+0x7e8+idx*0xb0，count@0x7f0）；FUN_140c03d70=queue present；结果码（OUT_OF_DATE/-0x3b9eae18、0x3b9acdeb、-0x3b9acdec）；state==2 旋转；返回低位置位 bool | O | decomp-0x1418cc1b0；骨架线1683 |
| `ManagedSwapchain::retire` | 0x1418ccb50 | 退役 | 写锁；state@0xa8∈{2,3,4}；state==2 且 device+0x850 非空/回调门@0xbb → 调 device vtable[0x858](arg@0x868, handle@0xa0)；拷 state byte@0xb9；返回 handle@0xa0 | O | decomp-0x1418ccb50；骨架线2224 |
| `impl$1::dispose`（ManagedSwapchain as ManagedResourceType） | 0x1418ccdd0 | 交换链销毁 | vkDestroySwapchain 路径（SwapchainInner::dispose 转发） | O | S_PUB32 |
| `SwapchainInner::new`/`new_impl`（线125/136） | — (inline) | 内层创建 | ash caps/modes/create/get_images；闭包选格式（线203/322/348/359） | O | 骨架线2625/3038 callee |
| `SwapchainInner::set_fullscreen_exclusive` | — (inlinee_declaration) | 全屏独占 | **有符号但 0 编译/内联体证据** | U（存在性 O） | 骨架线3758 无依赖 |
| `SwapchainInner::acquire_next_image` | — (inline) | 超时获取 | leek::{acquire_next_image,acquire_next_image_timeout,present_err,present_out_of_date,swapchain_pending_recreation} + ash acquire + mark_stale + Texture::clone + puffin scope + log | O | 骨架线3776 callee |
| `SwapchainInner::present_image`（线557） | — (inline) | 呈现 | leek 助手 + ash present | O | 骨架线4266 |
| `SwapchainInner::retire`/`dispose`/`on_focus`/`resize`/`set_vsync`/`mark_stale` | — (inline) | 薄转发 | 线638/670/391/436/441/446 | O | 骨架线3571..4919 |

### texture.rs

| 函数（骨架名） | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$0::dispose`（ManagedTexture as ManagedResourceType） | 0x140bdbe40 | 销毁 | vkDestroyImage 路径 | O | S_PUB32 |
| `ManagedTexture::new` | 0x140bdc2d0 | 创建 | FUN_140bdcba0（=create_texture，ICF 与 create_vk_image_view 同址）建 raw；全局 id 计数器（FUN_140d4d740）递增；填 ManagedTexture（desc、id、device、raw 字段、静态调试名指针） | O | decomp-0x140bdc2d0；骨架线411/1346 |
| `ManagedTexture::view` | 0x140bdc5d0 | 视图缓存 | param_4=None 且缓存放@0x70→直接返回；RwLock@0x38；位图（ptr@0x40/mask@0x48）SIMD 搜索 0x48B 视图槽（按 view_desc checksum）；命中→Ok；未命中→FUN_140bdcba0/1418ad1d0 建视图，位图找空闲槽写入，更新计数 | O | decomp-0x140bdc5d0；骨架线622 |
| `create_vk_image_view` | 0x140bdcba0（ICF 折叠） | 建视图/纹理 | ash vkCreateImageView 包装 | O | S_PUB32（.llvm 后缀=折叠） |
| `view_desc_impl`（线165）及其 closure | — (inline) | 视图描述 | 线165/181 | O | 骨架线1180/1211 |
| `convert_image_type_to_view_type` | — (inline) | 类型映射 | 2D/3D/1D/array | I | 骨架线1233 |

### buffer/descriptor_set/framebuffer/render_pass/sampler.rs（全 inline，无地址）

| 函数 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|
| `impl$0::dispose`（buffer，线24） | 销毁 | vkDestroyBuffer | O | S_PUB32 buffer@0x1418afb90（ManagedResourceType::dispose） |
| `impl$0::debug_name`（线33）/`allocation`（线39）/`mapped_slice_mut`（线47）+closure（线50） | Buffer 访问 | debug_name 被 throw_out 内联引用 | O | 骨架线626 |
| `ManagedResources::create_buffer`（线58） | 创建 | store allocate | O | 骨架 |
| `impl$1::dispose`（descriptor_set，线22）/`create_persistent_descriptor_set`（线34） | 持久描述集 | store 分配/销毁 | O | 骨架 |
| `impl$0::dispose`（framebuffer，线17）/`create_framebuffer`（线29） | 帧缓冲 | store | O | 骨架 |
| `impl$0::dispose`（render_pass，线19）/`create_render_pass`（线31）+closure（线42） | 渲染通道 | store；“Must have at least one framebuffer attachment”断言（CALL-CHAIN §5 0x142fd9448） | O | 骨架 |
| `impl$0::dispose`（sampler，线17）/`create_sampler`（线29，inlinee_declaration） | 采样器 | store | O | 骨架 |

---

## 3. 原版 ↔ rebuild 对照表

| 原版（O） | rebuild `reconstruction/rhapsody-rebuild/src/raw/managed/**` | 判定 | 差异说明 |
|---|---|---|---|
| `ManagedResources`：7×`ManagedResourceStore<T>`（0x178B）+`Mutex<Vec<DisposedRawResourceHandle>>`+`owning_device:u16@0xa50` | `ManagedResources`：7×`ResourceManager<T>`+`SharedResources{dropped: Mutex<Vec<Dropped>>, buckets, drained_prefix, legacy}`+`device: Option<Arc<..>>` | 半 | 形状一致；原 7 store 内联单结构、设备 id 入参构造（`new(u16)`）；rebuild `new()` 无参、设备 Arc 可选 |
| 槽位表 `AppendOnlyVec<ManagedResource<T>>`（h3r2tic append-only-vec 0.1.2 git rev 4031d67） | `slots: AppendOnlyVec<Slot<T>>` + `available: Mutex<Vec<u32>>` 空闲链表 | ✓（表）/U（槽内布局） | 依赖与 push/indices/len/index/AtomicPtr 用法有 PDB 证据；**`ManagedResource<T>` 槽内布局与空闲链表实现未恢复** |
| `ManagedResourceMeta`/控制块：原子字段（seq `fetch_max`、refcount `AtomicI32`、checksum 递增） | `ControlBlock{last_sequence:AtomicU64, references:AtomicU32, resource_checksum:AtomicU32}` | 半 | **原 refcount 是 `AtomicI32`（O），rebuild 用 `AtomicU32`**；字段顺序/校验和 CAS 语义 U |
| `RawManagedHandle`（Clone AtomicI32 / Drop 推入 Mutex<Vec<DisposedRawResourceHandle>> / TrackGpuLifetime） | `RawManagedHandle<T> = Managed<T>`（Arc<ManagedInner>），drop 经 `enqueue_retirement` | 半 | 原指针式句柄+独立原子计数；rebuild Arc+ControlBlock 承载同语义，ABI 不同 |
| `GpuOwned<T,Tracker>`（Tracker=RawManagedHandle/ManagedHandle，绑定载荷） | `GpuOwned<T>`（只 1 个类型参数，包 Managed<T>） | ✗ | **缺 Tracker 泛型**；原 `GpuOwned<(),ManagedHandle<ManagedBuffer>>` 与 `GpuOwned<[DSB;2],RawManagedHandle>` 两类单态 rebuild 无法表达 |
| `GpuOwned::map_tracker`→dynamic_constants binding closure + `get_vk_buffer` | `map_tracker` 返回单调序号快照 | ✗ | 原 map_tracker 把 tracker 接入描述符绑定（I）；rebuild 语义不同 |
| `ManagedRef::get_tracked`=真的跟踪资源 | `get_tracked` 返回 u64 序号 | ✗ | 行为语义偏差（“取+跟踪” vs “查序号”） |
| `ManagedResourceChecksum`（u32 包装 + invalidate） | 同 | ✓ | 一致 |
| `DisposedRawResourceHandle`（16B 元素，tag@0xe，Mutex<Vec>） | `Dropped{key,control,task}` | 半 | 原类型擦除枚举（I）+16B 布局；rebuild 结构化记录 |
| `CommandBufferDisposalBuckets::dispose_cb_resource`×7（raw::disposal） | rebuild `raw/disposal.rs::CommandBufferDisposalBuckets` | ✓ | 名称/职责对上（rebuild 有 begin_command_bucket/append_existing/remove） |
| `ManagedResources::take_out_dropped_waste`（排水+分派桶） | `SharedResources::take_out_dropped_waste` | ✓ | 行为一致；rebuild 增加 MissingBucket 校验（接线缺口） |
| `ManagedSwapchain` = `RwLock<SwapchainInner>` | `ManagedSwapchain = Managed<Swapchain>` | 半 | **原锁保护内层；rebuild 用 Managed<Swapchain> 包装 raw**，RwLock 语义由 raw 内部承担 |
| `SwapchainInner` 内嵌字段（desc/租约表/图像记录/state） | `SwapchainInner{inner: Arc<Swapchain>}` 委托 raw | ✗ | 原 SwapchainInner 是完整状态体（desc/租约/状态字节全在内）；rebuild 是薄壳 |
| `SwapchainInner::set_fullscreen_exclusive` | `Err(Unsupported("full_screen_exclusive 扩展未接入"))` | U/半 | **原符号存在（inlinee_declaration）但 0 编译/内联体证据**——不能证明原版实现；rebuild 占位合理 |
| `ManagedSwapchain::{new,desc,resize,set_vsync,acquire_next,present,retire}` | `Managed<Swapchain>::{new_buffer…/desc/resize/set_vsync/acquire_next/present/retire}` 委托 raw | 半 | 方法名对上；原 acquire 3s 超时+租约 refcount+OUT_OF_DATE 旋转等细节 rebuild 未逐一对齐（raw Swapchain 内部） |
| `ManagedTexture`（视图缓存：位图+0x48B 槽+RwLock） | `Managed<Texture>` + `image_view(desc)` | ✗ | **原纹理自带视图缓存（查重+位图分配）；rebuild 每次现建 TextureView** |
| `create_vk_image_view`/`view_desc_impl`/`convert_image_type_to_view_type` | 同名 free fn | ✓ | 形状一致 |
| 7 类 `ManagedX` + `ManagedResourceType::dispose` + `create_*` | 同名实现 + `allocate_*`/`new_*` | ✓ | 形状一致 |
| trait `AccessManagedResource`/`DisposeManagedResource`/`TrackGpuLifetime`/`ManagedResourceType` | `AccessManagedResource` 在 rebuild；其余无同名 trait | ✗ | rebuild 未恢复 trait 层（只恢复具体方法） |

---

## 4. U 卡点清单

1. **`ManagedResource<T>` 槽位内部布局与空闲链表**（最高优先）：store 0x178B 字段表、`AppendOnlyVec<ManagedResource<T>>` 槽内 {句柄弱引用/元数据/空闲链指针} 具体排布；`impl$6::allocate`（线369）复用空闲索引的确切算法（`append_only_vec::indices` 语义）。
2. **`DisposedRawResourceHandle` 16B 布局 + take_out_dropped_waste 分派表**：tag@0xe 的 7 变体判别值；`FUN_140bf92f0`/`FUN_140bdfcf0` 的真实身份。
3. **`ManagedResourceMeta`/checksum 原子字段精确布局**：seq/refcount/checksum 字段顺序与宽度；原 `AtomicI32` refcount 的溢出/哨兵语义。
4. **`GpuOwnedInner<T,Tracker>` 字段排布**：tracker 与资源 T 的存储顺序、GpuOwned 对 ManagedHandle 的持有方式（`into_gpu_owned/into_raw_gpu_owned` 为何都走 `GpuOwned::new`）。
5. **`ManagedSwapchain`/`SwapchainInner` 字段语义**：@8..0x30 五个 u64 逐一（format/colorspace/presentMode/extent）映射、state 枚举（0xa8 的 2/3/4）含义、@0x30 状态 2 的旋转条件、present 队列表（0xb0B/项）结构。
6. **swapchain 锁字编码**：写 0→8 / 读 +0x10 与 vanilla parking_lot 0.12.1（写1/读+2/4）不符——疑似 Bevy 分叉改常量；需按原 Cargo.lock 确认 parking_lot 版本与 fork。
7. **`set_fullscreen_exclusive`**：原符号（inlinee_declaration，线450）无编译/内联体证据——原版是否真实实现/调用未知；rebuild 的 Unsupported 占位合理但需按“原无实现”登记。
8. **handle.rs 线14/23/32 三类型的 Eq/Debug/Hash 身份**（`impl$32/36/37`、`impl$40/44/45`、`impl$47/51/52`）：`impl$51::fmt`@0xbd6bd0；三个类型是否为 `ManagedResourceMeta`/`ManagedResource`/`ManagedResourceChecksum`。
9. **`impl$6..impl$12` 7 对 allocate/release 与资源类型 tag 的映射**（rebuild 依 TypeId 假设 texture=0/buffer=1/…）。
10. **`ManagedResources::new` 设备入参**：u16 device id 来源与分配规则；`FUN_140d4d740` 全局纹理 id 计数器的归属（每设备？）。
11. **视图缓存细节**：`FUN_1418bd030`（view checksum 计算）、位图字宽与 `bVar37&1` 判定、0x48B 视图槽字段表。

---

## 5. 边界说明与证据卫生

- 本域 12 个反编译全部成功（无 NotFound），可作为 O 证据；summary.json 无 error。
- 全部结论以 S_PUB32 demangled 名为权威（ICF 折叠：`create_vk_image_view`@0x140bdcba0 带 `.llvm` 后缀=与 create_texture 折叠；`drop_in_place` 各类型同址折叠）。
- 依赖版本（O，骨架注解）：append-only-vec 0.1.2（git h3r2tic rev 4031d67）、arc-swap 1.7.1、parking_lot 0.12.1/0.11.2、lock_api 0.4.9、ash 0.38.0+1.3.281、log 0.4.27、puffin 0.14.1。按约束直接复用，不逆向实现。
- rebuild 为候选实现，本文件只登记差距供接线，未改任何 reconstruction 源码。

---

## 6. 审查轮闭合（2026-10-15，agent=rhapsody-managed-closure）

本小节追加 REVIEW-UNEXPLORED.md P0（退役分派链 / 槽位布局 / refcount 宽度）与 P1（GpuOwned / swapchain 锁字 / disposal 改判）的闭合结论。全部字节级证据落盘 `rne/managed-closure.md`。**本节不改写既有结论行，仅登记新事实与改判。**

### 6.1 退役分派链全闭合（O，原 §1.3 / U#2 → 闭合）

- `FUN_140bf92f0` = `GraphicsDevice::immediate_empty_bucket`、`FUN_140bdfcf0` = `drop_in_place<DisposalBucket>` 身份落实（S_PUB32 + 骨架 RVA 0xbf92f0 互证）。
- take_out_dropped_waste 排水：锁 `Mutex<Vec>`（byte 0→1 parking_lot 锁，deadlock acquire/release）→ 取空 cap-first Vec {cap@+8, ptr@+0x10, len@+0x18}（`mem::take` 复位 {0, 8悬垂, 0}）→ 逐 16B 元素按 `+0xe` tag 分派 → 尾部 `__rust_dealloc(ptr, cap*16, 8)` + `immediate_empty_bucket(resources)` + `drop_in_place<DisposalBucket>(&栈桶)`。
- **跳表（0x142d060f0，8×i32 相对偏移）与 tag→类型映射定死**：texture=0 / buffer=1 / swapchain=2 / (3=panic) / render_pass=4 / sampler=5 / descriptor_set=6 / framebuffer=7，分别调 `dispose_cb_resource<Managed*>@0x1418c65a0/0x1418c6af0/0x1418c55b0/0x1418c6050/0x1418c5b00/0x1418c7040/0x1418c7590`。**与 rebuild 依 TypeId 的候选映射完全一致**（原 §1.4 U#9 闭合）。
- 元素 16B = {*mut 控制块@0, u32 checksum@8, u16 device@0xc, u8 tag@0xe}；`rdx=*元素=inner.seq` 作 CommandBufferSeq 键 → dispose_cb_resource 经 hashbrown RawTable 探测把销毁动作写入 `HashMap<CommandBufferSeq, Mutex<DisposalBucket>>` 对应桶。**序号时间线驱动桶回收的环路闭合**（原 §3.5 断点接上，rebuild `SharedResources::record_submission/complete` 的原版对照物成立）。
- 桶结构：栈上 10×24B 槽（{cap@0, ptr@8, len@0x10} 空 Vec，ptr=8 悬垂），DisposalBucket 整体 0xF0B；`drop_in_place<DisposalBucket>` 逐槽 `__rust_dealloc(ptr, cap*8, 8)`。对应骨架 `impl$4..13::dispose_to_bucket` ×10。
- 残留 U：dispose_cb_resource 首参 `&resources.owning_device`（+0xa50）在函数内被当 hashbrown RawTable 用，与 get_maybe_tracked 读为 u16 device 的字段双语义矛盾未解；每槽 8B 动作载荷未逐项解码。

### 6.2 ManagedResource<T> 槽位布局 + 空闲链表（O，原 §1.2 U / U#1/#4 → 闭合）

- **store 槽 = 控制块**（AppendOnlyVec 元素，也是空闲链表结点）：
  - `+0x00 AtomicU64 seq`（init 0；track 时 fetch_max）
  - `+0x08 AtomicI32 refcount`（init 1；clone/drop 均为 `lock inc/dec dword`）
  - `+0x0c u32 checksum`
  - `+0x10 T resource 内联`（buffer=8B→槽 0x18B；descriptor_set=112B→槽 0x78B）
- **RawManagedHandle（24B，O）**：`+0x00 *mut Mutex<Vec<DisposedRawResourceHandle>>`（退役队列）、`+0x08 *mut 控制块`、`+0x10 u32 期望校验和`、`+0x14 u16 device`、`+0x16 u8 类型 tag`。**纠偏：+0x16 实为资源类型 tag（分派证据），非 §1.2 所述 “tracked 标志”**。
- **空闲链表**：store+0x170 = 原子头；allocate（0x1418ae7e0 族）CAS head→槽+0x10（空闲时兼作 next），复用槽重初始化 {seq=0, refcount=1, resource}；空表则 `AppendOnlyVec::push`（0x1418bbeb0，元素 {0,1,resource}）+ `append_only_vec::indices`（0x140c111b0）→ 槽 = base[idx*24]（bounds vs store+0x160）。store 0x178B = AppendOnlyVec 头 + indices + 计数@0x160 + 空闲链表头@0x170。

### 6.3 refcount 原子宽度定论（O，原 §2#2 / §4 首行 → 闭合）

- clone 0x140bd6b00 `lock inc dword ptr [inner+8]`、drop 0x140be24b0 `lock dec dword ptr [inner+8]`：**32 位**，控制块 +0x08。PDB `AtomicI32::fetch_add/fetch_sub` 一致 → **原 refcount = AtomicI32**。rebuild `AtomicU32` 半对维持。

### 6.4 GpuOwned 字段序（O，原 §1.2 / §3.3 / U#4 → 部分闭合）

- `GpuOwned<T, Tracker>`，**Tracker 默认 = RawManagedHandle**（S_PUB32 见单参 `GpuOwned<[DSB;2]>` 与双参 `GpuOwned<(), ManagedHandle<ManagedBuffer>>` 并存）。
- **字段序（O，clone 0x1403abd10 + drop_in_place 0x140161b50 互证）**：`GpuOwnedInner = { resource T @0x00, tracker @0x00+size(T) }`；以 `<[DSB;2], RawManagedHandle>` 为例：resource 0x40B@0、tracker 24B@0x40、**state Cell<GpuOwnedState> @0x58（克隆/新建=0）**。state 在末尾，非 §1.2 候选顺序。
- GpuOwnedState 判别值（0=Unused? / 1=Tracked?）残留 U（track 宿主为 rg/country-core pass 闭包内联，本轮未取编译宿主）。

### 6.5 swapchain 锁字编码（O，原 U#6 / §3.4 → 闭合）

- 写锁（acquire_next 0x1418cb54f）`lock cmpxchg [lockword] 0→8`（bit3=写者）；读锁（desc 0x1418cb10d）`lock cmpxchg [lockword] cur→cur+0x10`（每读者 +0x10），快路径条件 `(state&8)==0 && state<0xfffffffffffffff0`，否则 lock_shared_slow（1s）。
- **对照 Cargo.lock**（B-original / skeleton/B）：parking_lot 0.11.2 + 0.12.1 均 **registry 源、无 git/fork**。本二进制编码（写 bit3=8 / 读 +0x10）**与 stock parking_lot 0.12.1（写 LOCKED_BIT=1）不符** → 本地改常量/自定义 RwLock，**不能归因于 Cargo.lock 可见的 Bevy 分叉**；rebuild 委托 raw 承担锁语义的等价性以本编码为准。

### 6.6 disposal 对照改判（原 §3 行 ✓ → ✗/半，纯文档口径）

- 原 `CommandBufferDisposalBuckets = hashbrown::HashMap<CommandBufferSeq, Mutex<DisposalBucket>>`（每桶 Mutex，insert 断言 prev.is_none、remove 缺桶 panic @0x142d069c1）vs rebuild `BTreeMap<u64, Vec<DestroyAction>>` 无锁 → **判定 ✗/半**（容器/键/每桶互斥/insert-remove 语义均偏离；raw-init 证据 + 本证据 §6.1 hashbrown 探测确认）。
- **补登记**：rebuild `raw/graveyard.rs`（设备级 idle submit epoch 延迟销毁）原 ALGORITHMS 未登记 —— 与原版“仅按 seq 时间线”双轨并存的等价性缺口保持 U/✗（需 submit 域 seq→timeline 环路闭合，§6.1 已把 managed 侧端闭合）。
