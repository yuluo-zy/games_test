# rhapsody managed 域深挖审查清单（REVIEW-UNEXPLORED）

- 日期：2026-10-15。审查 agent：rhapsody-managed-review。
- 审查对象：`domains/managed/ALGORITHMS.md`（282 行）↔ 骨架 `domains/managed.json`（137 符号 / 42 compiled / 12 有地址）。
- 只产出审查清单，不改任何源码与已交付文档。机器码 / PDB / S_PUB32 为权威（B patched EXE）。
- 结论速览：**42 compiled 名义全覆盖（0 个完全无行）、12 有地址全进表；但"有行 ≠ 有深挖"**。
  关键证据缺口 3 处可在**已引用数据源内部**直接闭合而不需要新取反编译（见 §3.1 / §3.2 / §3.3），这是"完成度不高"的根因之一。

---

## 1. 未覆盖函数（42 compiled 逐函数核对表）

判定标准：✓=ALGORITHMS §2 有实质行且证据可用；△=有行但为空泛/仅骨架转述/证据级别不足；✗=完全无行（本域为 0）。

| # | 函数（骨架名，42 compiled） | §2 行 | 核对 | 审查说明 |
|---|---|---|---|---|
| 1 | `buffer::impl$0::dispose`（线24） | buffer 表 | ✓ | 有行；证据=S_PUB32 buffer@0x1418afb90 |
| 2 | `handle::impl$36::fmt`（线14 Debug） | handle 表 impl$36/44/51 合并行 | △ | 算法列"—"，类型身份 U；地址 0xbd6b50 不在 S_PUB32 |
| 3 | `handle::impl$44::fmt`（线23 Debug） | 同上 | △ | 同上，地址 0xbd6b90 不在 S_PUB32 |
| 4 | `handle::impl$51::fmt`（线32 Debug） | 同上 | △ | 同上，地址 0xbd6bd0 不在 S_PUB32 |
| 5 | `handle::impl$3::fmt`（RawManagedHandle Debug，线85） | handle 表 | △ | 算法列"—"；地址 0x140bd69a0 有 S_PUB32 ✓ |
| 6 | `handle::impl$4::track`（RawManagedHandle track，线96） | handle 表 | ✓ | 行为据骨架 callee 注解（非字节级）；地址 0x140bd6a70 S_PUB32 ✓ |
| 7 | `handle::impl$5::clone`（线109） | handle 表 | ✓ | AtomicI32::fetch_add 据骨架注解；**未做字节级宽度验证**（见 §2/§4） |
| 8 | `handle::impl$6::drop`（线127） | handle 表 | ✓ | Mutex+fetch_sub+CAS+Vec::push 据骨架注解；未字节级验证 |
| 9 | `handle::impl$7::eq`（线152） | handle 表 | △ | 算法列"比较（元数据/索引）"空泛；地址 0x140bd6b30 S_PUB32 ✓ 但无 decomp |
| 10 | `handle::impl$15::fmt<ManagedTexture>`（线208） | handle 表 | △ | 算法列"—"；地址 0x1403ac2d0/0x141621b00 S_PUB32 ✓ 无 decomp |
| 11 | `handle::impl$57::fmt`（线361） | handle 表 | △ | 算法列"—"；地址 0x1408fcd10 **不在 S_PUB32**（骨架 RVA） |
| 12 | `handle::impl$21::clone<[DSB;2],RawManagedHandle>`（GpuOwnedInner Clone，线379） | handle 表 | ✓ | 地址 0x1403abd10/0x141620a10 S_PUB32 ✓；行为据骨架 callee |
| 13 | `resources::ManagedResourceStore::allocate<ManagedBuffer>` | resources 表（4 单态合并行） | △ | 4 单态地址 0x1418ae7e0..0x1418aead0 列了，**无一个 decomp**；allocate 算法=内联的 AppendOnlyVec push/空闲复用，槽布局 U |
| 14 | `resources::ManagedResourceStore::allocate<ManagedPersistentDescriptorSet>` | 同上 | △ | 同上 |
| 15 | `resources::ManagedResourceStore::allocate<ManagedFramebuffer>` | 同上 | △ | 同上 |
| 16 | `resources::ManagedResourceStore::allocate<ManagedRenderPass>` | 同上 | △ | 同上 |
| 17 | `resources::ManagedResources::new` | resources 表 | ✓ | 0x1418c08e0 有 decomp；**已实挖** |
| 18 | `resources::ManagedResources::take_out_dropped_waste` | resources 表 | △ | 0x1418c0a70 有 decomp；**两个被调函数身份未查 S_PUB32（见 §3.2）** |
| 19 | `resources::AccessManagedResource::get_maybe_tracked<ManagedResources,ManagedBuffer>` | resources 表 | △ | 地址挂载错位 + 槽布局已从 decomp 可见未挖（见 §3.1） |
| 20 | `resources::impl$4::get_tracked<ManagedBuffer>` | resources 表（get/get_tracked 合并行） | △ | 地址 0x1418aed30 S_PUB32，无 decomp；行为为骨架推断 |
| 21 | `resources::impl$4::get_tracked<ManagedPersistentDescriptorSet>` | 同上 | △ | 同上 0x1418aee50 |
| 22 | `resources::impl$4::get_tracked<ManagedRenderPass>` | 同上 | △ | 同上 0x1418aef70 |
| 23 | `resources::impl$5::dispose<ManagedBuffer>` | resources 表（×7 合并行） | ✓ | 地址 S_PUB32 系列 ✓；行为"type-dispose+release+arc_swap 排空+Arc drop"基于骨架 |
| 24 | `resources::impl$5::dispose<ManagedPersistentDescriptorSet>` | 同上 | ✓ | 同上 |
| 25 | `resources::impl$5::dispose<ManagedFramebuffer>` | 同上 | ✓ | 同上 |
| 26 | `resources::impl$5::dispose<ManagedRenderPass>` | 同上 | ✓ | 同上 |
| 27 | `resources::impl$5::dispose<ManagedSampler>` | 同上 | ✓ | 同上 |
| 28 | `resources::impl$5::dispose<ManagedSwapchain>` | 同上 | ✓ | 同上 |
| 29 | `resources::impl$5::dispose<ManagedTexture>` | 同上 | ✓ | 同上 |
| 30 | `resources::impl$6::allocate`（线369） | resources 表 | △ | 地址 0xbdbd50；**无 decomp**；空闲链表/AppendOnlyVec::indices 语义未解（U#1） |
| 31 | `swapchain::impl$1::dispose`（ManagedSwapchain as ManagedResourceType） | swapchain 表 | △ | 地址 0x1418ccdd0 S_PUB32 ✓，无 decomp；"vkDestroySwapchain 路径"为推断 |
| 32 | `texture::impl$0::dispose`（ManagedTexture as ManagedResourceType） | texture 表 | ✓ | 地址 0x140bdbe40 S_PUB32 ✓ |
| 33 | `texture::create_vk_image_view` | texture 表 | ✓ | 0x140bdcba0 ICF 折叠（与 create_texture 同址）✓ |
| — | 其余 9 个有地址 compiled | — | ✓ | 12 个有地址符号（11 compiled + `get_maybe_tracked` trait 声明 0x1418aeba0）**全部**进 §2，12 个 decomp 全成功无 NotFound（summary.json 无 error） |

**结论（未覆盖）**：0 个 ✗；**有地址 12 个全部覆盖**（含 trait 声明符号）。
**△ 共 14 个**，集中在：4 个 `allocate<T>` 单态、3 个 `get_tracked<T>` 单态、4 个 `fmt`（线14/23/32 + 线361）、`impl$7::eq`、`impl$15::fmt<ManagedTexture>`、`take_out_dropped_waste`、`get_maybe_tracked`、`impl$6::allocate`、`swapchain::impl$1::dispose`。
**30 个无地址 compiled** 的职责均已按消费端/骨架给出行（无"完全没推断"的），但其中 create_* 族（buffer/descriptor_set/framebuffer/render_pass/sampler 的 `ManagedResources::create_*`）行内容只有"store 分配"，没有下沉到 raw 层创建算法（`create_buffer_impl`/`create_vk_image_view`/`vkCreateFramebuffer` 的 desc 组装），建议沿 §3.1 的 store 布局一起补。

**数据卫生**：`AccessManagedResource::get_maybe_tracked` 的 0x1418aeba0 在 managed.json 里挂在 **inlinee_declaration（trait 声明）** 符号上，compiled 单态 `get_maybe_tracked<ManagedResources,ManagedBuffer>` 的 va=null；ALGORITHMS §2 把地址写到了单态行。不影响结论，但登记说明。

---

## 2. 表面分析（有行但空泛 / 证据级别不足）

1. **handle.rs 五个 Debug::fmt（线14/23/32 + 线85/208/361）**：§2 算法列全是"—"。这些是**编译实例**（非 derive 展开就是内联不生成符号），`impl$36::fmt`@0xbd6b50 的骨架注解已示调用 `core::fmt::Formatter::debug_tuple_field1_finish` = **单字段 tuple 结构体 Debug**（见 §3.3）。建议补法：rne-read 0xbd6b50/0xbd6b90/0xbd6bd0/0x140bd69a0 三个 fmt 的前 16B，看 `debug_tuple_field1_finish` 打印的字段名/值，直接锚定三类型身份与 RawManagedHandle 的 Debug 载荷；这是**低投入高回报**。
2. **handle.rs 四个 compiled 行为行（track/clone/drop/eq，+ GpuOwnedInner::clone）**：role/算法有文字，但证据全部是"骨架 callee 注解"（`AtomicI32::fetch_add`/`AtomicU8::compare_exchange`/`arc_swap Debt`），**没有一次字节级/反编译复核**。§1.2 已把它当 O 用，实际是 PDB 级 O、机器码级未验。建议补法：rne-read 0x140bd6b00（clone）看 `lock xadd` 宽度 → 直接裁决 **AtomicI32 vs AtomicU32**；rne-read 0x140be24b0（drop）看 push 到 Mutex<Vec> 的写入序列。这两个是 rebuild 对照的关键差异点（§4），值得字节级钉死。
3. **`impl$7::eq`（RawManagedHandle PartialEq @0x140bd6b30）**：算法列"比较（元数据/索引）"。骨架注解示它内联 `impl$37/45/52::hash`（线14/23/32 三类型的 hash），即 eq 是**按三类型 hash 比较**。建议补法：读骨架线 503..545 完整注解 + rne-read 0x140bd6b30，确认"eq = 三个新类型 eq 组合"还是 hash 比较。
4. **`impl$6..impl$12::{allocate,release}` 7 对 per-type**：合并成一行、verdict=I。每对 `allocate/release` 对应一个 tag 资源类型，但**tag→类型映射仍是候选假设**（U#9），且 `impl$6::allocate`@0xbdbd50（有地址）的 AppendOnlyVec::push/indices 语义未解。建议补法：反编译 0xbdbd50 + 4 个 allocate 单态 0x1418ae7e0..ad0，比对每单态内 tag 常量/检查点 → 直接闭合 U#9 的 tag 映射。
5. **`ManagedResources::create_*` ×5（buffer/descriptor_set/framebuffer/render_pass/sampler）**：行内容只有"store 分配"。原 create_* 是内联到 `ManagedResourceStore::allocate<T>` 单态的，真正业务在 raw 层（`create_buffer_impl`/`create_vk_image_view`/`vkCreateRenderPass` desc 组装）。建议补法：沿 §3.1 的 allocate 单态 decomp 一起下沉，至少确认 desc→raw 创建参数的装配点（与 raw-core 域 cross-check）。
6. **`get_maybe_tracked`**：§2 说"已跟踪→get_tracked，未跟踪→get"——但 decomp 实际语义更深（见 §3.1），且**槽位布局线索就在这份手证据里**没挖。这是最典型的"证据在眼前没挖"。
7. **`swapchain::impl$1::dispose`**：`ManagedResourceType::dispose` @0x1418ccdd0，无 decomp；"vkDestroySwapchain 路径（SwapchainInner::dispose 转发）"是推断。建议补法：反编译 0x1418ccdd0（应为小函数）确认是否真走 vkDestroySwapchain + 是否旋转 state。

---

## 3. 业务逻辑缺口（处理链断点 + U 算法）

### 3.1 ManagedResources 7×0x178B store 内部布局与空闲链表（最高优先，手证据在握）

`ManagedResource<T>` 槽位内部布局 **ALGORITHMS §4#1 标 U**，但 **decomp-0x1418aeba0（get_maybe_tracked，已落盘）直接给出局部布局**：

- 槽位（param_2）上：`+0x08` = 指向 inner 的指针；`+0x10` = u32 期望校验和；`+0x14` = u16 device id；`+0x16` = u8 tracked 标志（==1 已跟踪）。
- inner 上：`+0x00` = AtomicU64 seq（fetch_max 目标）；`+0x0c` = u32 checksum（与槽位 +0x10 比较）；`+0x10` = resource 指针（返回值）。

即槽位元素远小于"0x178B/7 份大结构"，store 0x178B 是 `AppendOnlyVec<ManagedResource<T>>` 容器头 + 空闲链表字段的组合。**这条证据链未在 ALGORITHMS 里体现，是"完成度不高"的头号实例。**

建议补法：
1. 反编译 4 个 `ManagedResourceStore::allocate<T>` 单态（0x1418ae7e0 / 0x1418ae8a0 / 0x1418ae9f0 / 0x1418aead0）→ 看每个单态对槽位的初始化写入序列（哪些字段被置位、空闲索引从哪取）；
2. 反编译 `impl$6::allocate` @0xbdbd50 → 解开 `AppendOnlyVec::push`/`indices`/`AtomicPtr::compare_exchange_weak` 的确切复用算法（B 的 append-only-vec 0.1.2 git rev 4031d67 可直接按依赖源码核对行为，不需逆向）；
3. 与 `impl$4::get`/`get_tracked` 单态（0x1418aed30/0xaee50/0xaef70）交叉 → 拼出完整槽布局 + 空闲链表。

### 3.2 退役链：take_out_dropped_waste 的 tag@0xe 分派（S_PUB32 可即时闭合）

ALGORITHMS §4#2 把 `FUN_140bf92f0`/`FUN_140bdfcf0` 标为"真实身份 U"。**S_PUB32（public-symbols.json，ALGORITHMS 自己引用的数据源）直接解出**：

- `FUN_140bf92f0` = `rhapsody::raw::disposal::GraphicsDevice::immediate_empty_bucket`（无 hash 后缀，单符号）。
- `FUN_140bdfcf0` = `core::ptr::drop_in_place<rhapsody::raw::disposal::DisposalBucket>`。

且 decomp-0x1418c0a70 栈上有 10×`{u64=0, u64=8, u64=0}` 的 24B 三元组（约 30 个 u64），即 **10 个 `DisposalBucket`（每个内含 Vec<16B 元素>，cap=8）**——take_out_dropped_waste 按 tag 把退役记录分派进 per-tag DisposalBucket，然后 `immediate_empty_bucket(resources)` 冲洗设备桶、`drop_in_place<DisposalBucket>` 逐桶处置。**这与 raw-init 域结论（原 disposal = `hashbrown::HashMap<CommandBufferSeq, Mutex<DisposalBucket>>`）正好接上。**

建议补法：
1. 反编译 `GraphicsDevice::immediate_empty_bucket`（应是小函数）→ 确认它排空设备侧 HashMap 哪些桶；
2. 反编译 `DisposalBucket` 的 Drop/dispose → 确认每桶如何按 CommandBufferSeq 写回 `CommandBufferDisposalBuckets`；
3. tag 判别值（case≥9，Ghidra 跳表截断）仍 U：用 `rne-read --hex` 读 0x1418c0a70 的跳表区（或 rne-disasm）手解 7 个 case 目标 → 闭合"7 变体 → 7 类资源 → tag 值"。

### 3.3 GpuOwned 双泛型单态化（骨架 PDB 已给结构，枚举未挖）

骨架 PDB 类型拼写已示：`GpuOwned<T,Tracker>` 含 `core::cell::Cell<GpuOwnedState>`（fmt 实例 `core::fmt::impl$74::fmt<Cell<GpuOwnedState>>`）与 `core::mem::manually_drop::ManuallyDrop<GpuOwnedInner<T,Tracker>>`（对应 `impl$25::drop` 线550）。**GpuOwned = { state: Cell<GpuOwnedState>, inner: ManuallyDrop<GpuOwnedInner> } 的结构已被 PDB 暴露**，但 `GpuOwnedState` 的枚举变体（tracked/untracked/unused？）与 `GpuOwnedInner<T,Tracker>` 的字段顺序（资源 T 在前还是 Tracker 在前）未挖。

建议补法：
1. grep 骨架 `handle.rs`/PDB 找 `GpuOwnedState` 的枚举定义（`enum2$<...GpuOwnedState>` 的构造/判别证据）；
2. 反编译 `GpuOwnedInner::clone` 0x1403abd10 / 0x141620a10 → 看数组拷资源 vs tracker 拷顺序，钉死字段排布；
3. 反编译 `impl$22::track`（线397，Cell::set）与 `impl$25::drop`（线550）实际单态宿主 → 确认 GpuOwnedState 与 ManuallyDrop 交互。

### 3.4 SwapchainInner 锁字编码（Bevy 分叉疑点）

ALGORITHMS 已观察到写锁 0→8（bit3=写者）、读锁每读者 +0x10，与 vanilla parking_lot 0.12.1（写1/读+2/4）不符。**锁字编码 = U**，且 rebuild 用 `Managed<Swapchain>` 而原用 `RwLock<SwapchainInner>`，锁语义由 raw 内部承担——锁字是否 Bevy 分叉改常量直接影响"原版锁协议 vs rebuild 委托 raw"的等价性判断。

建议补法：
1. rne-read --hex 反汇编 0x1418cb520（acquire_next）与 0x1418cc1b0（present）的 lock_exclusive/unlock_exclusive 指令 + 常量；
2. 对照 build-info/Cargo.lock 里 parking_lot/parking_lot_core 的版本与 fork 记录（0.12.1 + 0.11.2 双版本已见）；若 fork，登记常量差异即可，不逆向 parking_lot 本体。

### 3.5 序号→时间线回收（跨域断点）

managed 侧 `dispose_cb_resource×7`（0x1418c5b00..0x1418c7590）把 vkDestroy 写进 `CommandBufferDisposalBuckets`（按 CommandBufferSeq 键）；**谁按时间线/队列完成来排出这些桶**，managed ALGORITHMS 只画到"销毁桶按命令缓冲序号组织；序号时间线完成后回收"一句，链断在 raw submit 侧。CALL-CHAIN §3.2 提到 rebuild 有 `raw/submit.rs` reclaim，但没和 managed 桶对接。

建议补法：跨域补 raw/submit.rs / raw-init 的"CommandBufferSeq → timeline"映射如何驱动桶 drain（`GraphicsDevice::immediate_empty_bucket` 就是这链上的一个环节，见 §3.2）；把 `dispose_cb_resource` → HashMap<CommandBufferSeq,…> → 队列时间线完成 → vkDestroy 的完整环路画出来。这是 **rebuild `SharedResources::record_submission/complete`（seq→timeline 阈值 BTreeMap）的原版对照物**，缺口直接决定 rebuild 的 `LegacyReceipts` 设计是否成立。

### 3.6 handle.rs 线14/23/32 三类型身份

ALGORITHMS §4#8 标 U。骨架已示三类型**都是单字段 tuple 结构体**（`impl$36::fmt` 调 `debug_tuple_field1_finish`，`impl$37/45/52::hash` 内联 `write_u16`+`to_ne_bytes`），即 2 字节字段的新类型（u16）。三者可能对应 `(u64 seq 包装/u16 device/…)` 之类——但资源句柄字段多为 u32/u64，u16 单字段更可能指向 device id 或 tag 包装。

建议补法：rne-read 0xbd6b50/0xbd6b90/0xbd6bd0 三个 fmt 打印的字段名串（Debug 名会内嵌在 .rdata）→ 一次锚定三个类型名；与 `impl$5::clone` 内联的 `impl$40::eq`（线23，track 路径用）交叉验证。

### 3.7 ManagedSwapchain::new 与 ManagedTexture::view 的算法深度

- `ManagedSwapchain::new`（0x1418ca030，20KB decomp 已落盘）：ALGORITHMS 只给了 vtable 调用清单，**格式/呈现模式扫描算法、surface caps 处理、租约表/图像记录建立过程没展开**。建议补法：分节 grep decomp（常量、`vk::PresentMode` 比较、格式回退链），按 §5.2 的 SwapchainInner 字段表反推每个字段的赋值来源。
- `ManagedTexture::view` 视图缓存（0x140bdc5d0，19KB decomp）：0x48B 槽字段表、位图字宽、`bVar37&1` 判定、checksum 计算 `FUN_1418bd030` 身份全 U（§4#11 已列）。建议补法：反编译 FUN_1418bd030（view checksum）+ rne-read 位图字宽 + 对照 raw-core 的 `TextureViewDesc` 0x3cB 布局。

### 3.8 退役链尾：DisposedRawResourceHandle 16B 元素与 take_out_dropped_waste 的 Vec 头

take_out_dropped_waste 的 Vec 头操作（decomp-0x1418c0a70：读 ptr/end/cap → 清零头 + 置 cap=8）显示**它把全局 waste Vec 取空（mem::take 语义）后逐个元素处理**；元素 16B、tag@0xe 已确认。ALGORITHMS §1.2 已写"元素16B O"，但 16B 里 tag 之外的 15B 载荷（是 Arc 指针 + 类型名？还是 key+checksum？）未解。建议补法：rne-read 0x1418c0a70 的迭代区 + 对 7 个 `dispose_cb_resource` 单态（0x1418c5b00 族）各取一个 decomp → 看每个单态消费元素哪几个偏移 → 反推 16B 载荷结构。

---

## 4. rebuild 对照缺口

| 对照点 | ALGORITHMS §3 判定 | 审查复核 |
|---|---|---|
| AtomicI32 vs AtomicU32 refcount | 半（已列） | ✓ 已列，但**未做字节级验证**（见 §2#2）。建议 rne-read 0x140bd6b00 确认 `lock xadd` 宽度。若实测 32 位，rebuild `AtomicU32` 需改；若 64 位，ALGORITHMS 的"AtomicI32"要纠偏 |
| GpuOwned 单泛型 vs 双泛型 | ✗（已列） | ✓ 已列。缺"rebuild 能否表达两个单态"的接线评估：`GpuOwned<(),ManagedHandle<ManagedBuffer>>`（managed 侧）与 `GpuOwned<[DSB;2],RawManagedHandle>`（bind 载荷侧）两类都收不进 `GpuOwned<T>`。且 `GpuOwned::map_tracker` 原语义=把 tracker 接入描述符绑定（I），rebuild 返回单调序号快照 ✗ 已列 |
| ManagedHandle 非泛型 vs 泛型 | 半（已列） | ✓ 已列。**文档冲突需统一**：CALL-CHAIN §3.2 把 managed 全族标"对得上"，本域标"半"——两文档口径不一致，建议以本域为准（rebuild `Managed<T>` Arc 式 vs 原指针+独立原子计数，ABI 不同） |
| BTreeMap vs AppendOnlyVec 槽位表 | ✓表 / U槽内（已列） | ✓ 已列。rebuild 槽位表也用 AppendOnlyVec ✓；但 **rebuild 的空闲链表是 `Mutex<Vec<u32>>`（available），与原 `append_only_vec::indices` 语义是否等价 U**——§4#1 已挂 |
| **disposal 桶：BTreeMap 无锁 vs 原 hashbrown+Mutex** | **✓（名称/职责对上）** | **✗ 判定过宽**。raw-init 域已证原 `CommandBufferDisposalBuckets`= `hashbrown::HashMap<CommandBufferSeq, Mutex<DisposalBucket>>`（insert 断言 prev.is_none()、remove 缺桶 panic），rebuild `raw/disposal.rs` 是 `BTreeMap<u64, Vec<DestroyAction>>` 无锁、覆盖桶用 forget——**本域 §3 把该行标 ✓ 是错的，应为 ✗/半**。且 rebuild 还有 `raw/graveyard.rs`（设备级 epoch 延迟销毁），ALGORITHMS **完全未登记** |
| epoch vs 序列号销毁桶 | 未登记 | ✗ 缺口。rebuild 的 `raw/graveyard.rs` 用"整设备 idle 确认的 submit epoch"判定销毁，原按**命令缓冲序号时间线**；这两套并存（disposal.rs 按 seq + graveyard 按 epoch），与"原只有 seq 桶"不符。§3.5 的环路闭合后可给出定论 |
| take_out_dropped_waste 排水 | ✓（已列） | ✓ 已列（rebuild 增 MissingBucket 校验）。**但原版实际走 `immediate_empty_bucket` + `DisposalBucket::drop`（§3.2），与 rebuild 的 `append_existing/begin_command_bucket` 流程未逐点对齐**；建议 §3.2 闭合后回填 |
| `ManagedRef::get_tracked` 取+跟踪 vs 返回序号 | ✗（已列） | ✓ 已列，行为语义偏差确认 |
| ManagedTexture 视图缓存 | ✗（已列） | ✓ 已列：原位图查重+0x48B 槽缓存，rebuild 每次现建 TextureView |
| SwapchainInner 完整状态体 vs 薄壳 + 无 RwLock | ✗/半（已列） | ✓ 已列。补：rebuild `ManagedSwapchain=Managed<Swapchain>` 无 RwLock，原 `RwLock<SwapchainInner>` 的读/写分离语义（desc 读锁 vs acquire 写锁）由 raw 内部承担，等价性未论证 |
| `set_fullscreen_exclusive` 原符号存在但 0 编译/内联体 | U/半（已列） | ✓ 已列，rebuild Unsupported 占位合理，需按"原无实现证据"登记 |
| trait 层（AccessManagedResource/DisposeManagedResource/TrackGpuLifetime/ManagedResourceType） | ✗（已列） | ✓ 已列 |

**rebuild 对照缺口汇总**：已列的半/✗ 项质量可接受，但有 2 处**实际判定错误/遗漏**需要修正口径：(a) disposal 桶 ✓ 应为 ✗/半（hashbrown+Mutex vs BTreeMap 无锁，raw-init 证据）；(b) graveyard epoch 双轨销毁完全未登记。另 AtomicI32/U32 与 lock 字两处是**可即时字节级裁决**的高价值补证。

---

## 5. 补全优先级排序

### P0（手证据在握 / S_PUB32 可即时闭合，闭环业务断点）
1. **退役分派链闭合**：把 `FUN_140bf92f0`= `GraphicsDevice::immediate_empty_bucket`、`FUN_140bdfcf0`= `drop_in_place<DisposalBucket>` 写进 ALGORITHMS；反编译这两个小函数 + 7 个 `dispose_cb_resource` 单态（0x1418c5b00 族），rne-read 跳表区手解 tag case 值 → 闭合 §3.2 / §3.8。
2. **ManagedResource<T> 槽位布局 + 空闲链表**：从 decomp-0x1418aeba0 挖已可见布局，补反编译 4 个 allocate 单态（0x1418ae7e0..ad0）+ `impl$6::allocate`@0xbdbd50 → 拼出槽/空闲链表/7 对 allocate-release 的 tag 映射（闭合 §4 U#1/U#9）。

### P1（需新取证，但直接决定 rebuild 等价性）
3. **AtomicI32 vs AtomicU32 字节级裁决**：rne-read 0x140bd6b00（clone）/ 0x140be24b0（drop）→ 确认原子宽度与 push 序列（§4 首行 + §2#2）。
4. **GpuOwnedState 枚举 + GpuOwnedInner 字段序**：grep 骨架/PDB 定义 + 反编译 0x1403abd10 / 0x141620a10（§3.3）。
5. **swapchain 锁字编码**：rne-read 反汇编 acquire/present 锁指令 + 对照 Cargo.lock 的 parking_lot fork（§3.4）。
6. **disposal 桶判定纠偏 + graveyard 登记**：按 raw-init 证据把 §3 的 disposal ✓ 改 ✗/半，登记 graveyard epoch 双轨（§4 两处）——纯文档修订，无新取证。

### P2（深度展开 / 身份锚定）
7. **线14/23/32 三类型身份**：rne-read 0xbd6b50/0xbd6b90/0xbd6bd0 的 fmt 字段名串（§3.6）。
8. **`ManagedSwapchain::new` 算法分节**：格式/模式扫描、租约表建立（§3.7）。
9. **视图缓存位图/checksum**：反编译 FUN_1418bd030 + rne-read 位图字宽（§3.7）。
10. **5 个 create_* 下沉**：沿 allocate 单态确认 desc→raw 创建装配（§2#5）。

### 结束语
本域 ALGORITHMS 的**覆盖（覆盖了谁）是完整的，但深度（讲到什么粒度）不达标**。最高价值修正集中在"**已引用数据源（public-symbols.json / 已落盘 decomp / 骨架 PDB 类型拼写）内部可解而未解**"的 3 处（§3.1 槽布局、§3.2 分派身份、§3.3 GpuOwned 结构）与 2 处判定纠偏（disposal ✓→✗/半、graveyard 未登记）。这五处不需要新增反编译之外的取证，即可把本域从"表齐全"推进到"链闭合"。
