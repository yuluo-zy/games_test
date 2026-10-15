# rhapsody managed 域审查轮闭合证据（rne-read / rne-disasm 字节级）

- 日期：2026-10-15。agent：rhapsody-managed-closure。
- 方法：rne-disasm（capstone）/ rne-read 直读 B patched EXE 原字节；S_PUB32（public-symbols.json）与骨架 PDB RVA 为权威。全部结论 O=机器码实测。
- 对应：domains/managed/REVIEW-UNEXPLORED.md P0（退役分派链 / 槽位布局 / refcount 宽度）+ P1（GpuOwned / swapchain 锁字 / disposal 改判）。

---

## 1. P0-1 退役分派链全闭合（take_out_dropped_waste 0x1418c0a70）

### 1.1 函数身份（S_PUB32 核验）

| 地址 | 身份 | 依据 |
|---|---|---|
| 0x1418c0a70 | `ManagedResources::take_out_dropped_waste` | managed.json spub32 |
| 0x140bf92f0 | `GraphicsDevice::immediate_empty_bucket` | S_PUB32；骨架 disposal.rs:3590 编译实例 RVA 0xbf92f0 ✓ |
| 0x140bdfcf0 | `core::ptr::drop_in_place<DisposalBucket>` | S_PUB32 |
| 0x1418c5b00..0x1418c7590 | `CommandBufferDisposalBuckets::dispose_cb_resource` ×7 单态 | S_PUB32 hash 后缀 |

### 1.2 排水（Mutex<Vec> 取空）

```
0x1418c0d5d  mov rdi, [rdi]                ; rdi = *param_1 = Mutex<Vec> 指针
0x1418c0d60  mov cl, 1; xor eax,eax
0x1418c0d64  lock cmpxchg byte ptr [rdi], cl ; parking_lot RawMutex 字节锁 0→1
0x1418c0d71  call 0x140c81c70               ; parking_lot_core deadlock acquire_resource
0x1418c0d76  mov rax,[rdi+8]   → [rbp+0xd8]  ; Vec.cap（cap-first {cap@8,ptr@0x10,len@0x18}）
0x1418c0d81  mov rax,[rdi+0x10] → [rbp+0xe0] ; Vec.ptr
0x1418c0d8c  mov r14,[rdi+0x18]              ; Vec.len
0x1418c0d90  mov [rdi+8],0; mov [rdi+0x10],8; mov [rdi+0x18],0  ; mem::take 复位 {cap=0, ptr=8悬垂, len=0}
0x1418c0db2  call 0x140c81e10               ; deadlock release_resource
...
0x1418c0fe4  mov rdx,[rbp+0xd8]; test rdx,rdx; je skip
0x1418c0ff0  shl rdx,4; mov r8d,8; mov rcx,[rbp+0xe0]; call 0x140613c20
                                           ; __rust_dealloc(ptr, cap*16, 8) —— 释放 drained Vec 缓冲
```
- Vec 为游戏 cap-first 布局 {cap@+8, ptr@+0x10, len@+0x18}；`ptr=8` = 空 Vec 的 `NonNull::dangling()`（align=8），cap=0 时 dealloc 跳过。

### 1.3 元素（DisposedRawResourceHandle）布局与分派

- 元素 16B（O，由 drop 0x140be24b0 写入序列 与 分派读取序列互证）：
```
@0x00: *mut ManagedResource<T> 控制块（元素+0 deref = inner.seq，作 CommandBufferSeq 键）
@0x08: u32 checksum（handle+0x10 拷贝）
@0x0c: u16 device（handle+0x14 拷贝）
@0x0e: u8 资源类型 tag（handle+0x16 拷贝）  ← 分派键
@0x0f: u8（分派读取、drop 未写 —— 栈残值/未定义）
```
- drop 0x140be24b0 入队写入：`[elem+0]=inner`、`[elem+8]=checksum`、`[elem+0xc]=device`、`[elem+0xe]=[handle+0x16]`（15B；+0xf 未写）。

### 1.4 循环与跳表分派（0x1418c0e30..0x1418c0fdf）

```
0x1418c0e30  cmp rax,r14; je done            ; while ptr < end
0x1418c0e39  movzx ecx, byte ptr [rax+0xe]   ; tag = elem+0xe
0x1418c0e3d  cmp ecx,8; je done              ; tag==8 哨兵退出
0x1418c0e46  lea r13,[rax+0x10]              ; 下一元素（16B 步进）
0x1418c0e4a  movzx edx, byte ptr [rax+0xf]
0x1418c0e4e  mov r8,[rax]                    ; elem+0 = inner
0x1418c0e51  mov rax,[rax+6]                 ; elem+6 重叠读（ptr 高2B+checksum+device）
0x1418c0e71  movsxd rax, dword ptr [r15+rax*4]; add rax,r15; jmp rax   ; r15 = 0x142d060f0
```
- 跳表基址 r15 = 0x142d060f0（`lea r15,[rip+0x144530f]` @0x1418c0dda，next_rip=0x1418c0de1）。
- 跳表 8 项 × i32 相对偏移（原始字节，LE）：

| tag | 字节 | 偏移(i32) | 目标 | 调用的 dispose_cb_resource 单态 |
|---|---|---|---|---|
| 0 | 8a ad bb fe | -0x1445276 | 0x1418c0e7a | **ManagedTexture** @0x1418c65a0 |
| 1 | 3f ae bb fe | -0x14451C1 | 0x1418c0f2f | **ManagedBuffer** @0x1418c6af0 |
| 2 | 02 ae bb fe | -0x14451FE | 0x1418c0ef2 | **ManagedSwapchain** @0x1418c55b0 |
| 3 | 02 b0 bb fe | -0x1444FFE | 0x1418c10f2 | **panic**（invalid tag，disposal.rs 线 0x13=19 处 panic） |
| 4 | c5 ad bb fe | -0x144523B | 0x1418c0eb5 | **ManagedRenderPass** @0x1418c6050 |
| 5 | fe ac bb fe | -0x1445302 | 0x1418c0dee | **ManagedSampler** @0x1418c5b00 |
| 6 | 7a ae bb fe | -0x1445186 | 0x1418c0f6a | **ManagedPersistentDescriptorSet** @0x1418c7040 |
| 7 | b7 ae bb fe | -0x1445149 | 0x1418c0fa7 | **ManagedFramebuffer** @0x1418c7590 |

- 单态身份（骨架 disposal.rs 编译实例 RVA，全部 = va 0x1418cxxxx）：buffer 0x18c6af0、descriptor_set 0x18c7040、framebuffer 0x18c7590、render_pass 0x18c6050、sampler 0x18c5b00、swapchain 0x18c55b0、texture 0x18c65a0。
- **tag→类型映射定死**：texture=0 / buffer=1 / swapchain=2 / (3 无效) / render_pass=4 / sampler=5 / descriptor_set=6 / framebuffer=7。与 rebuild 依 TypeId 的候选（texture=0/buffer=1/swapchain=2/render_pass=4/sampler=5/descriptor_set=6/framebuffer=7）**完全一致**。
- tag3 目标 0x1418c10f2：`lea rcx,[rip+..]; lea r8,[rip+..]; mov edx,0x13; call 0x1428d9430; ud2` = panic（disposal.rs 行号 19）。

### 1.5 桶（DisposalBucket）结构

- 栈上 10 个 24B 槽，基址 [rbp-0x48]（`lea rbx,[rbp-0x48]` 传给所有 dispose 调用）。init 序列：`[b]=0; [b+8]=8; [b+0x10]=0` 重复 10 次（`8` = 空 Vec dangling ptr）。
- DisposalBucket 整体 0xF0B（insert/remove 0x1418c4100/0x4260 经 `func_0x000142923c00(param_1, auStack_230, 0xf0)` 拷贝 = 10 × 24B Vec 槽）。
- 每槽 = cap-first Vec：`{cap@0, ptr@8, len@0x10}`；drop_in_place<DisposalBucket>（0x140bdfcf0）对每槽 `if cap!=0: __rust_dealloc(ptr, cap*8, 8)`（0x140613c20 = __rust_dealloc，S_PUB32）—— 槽元素 8B。
- 尾部 `add rsi,0xd8; jmp 0x140be1710` 处理第 10 槽及后续。
- 调用 dispose_cb_resource 时 `r8 = rbx = &桶基址`；桶内槽的选取由 dispose 内部按动作类型决定（对应骨架 impl$4..impl$13::dispose_to_bucket ×10，disposal.rs 线 292..346）。

### 1.6 序列号（CommandBufferSeq）语义

- dispose_cb_resource 的 `rdx = *element = inner.seq`（控制块 +0x00 AtomicU64 值）。该 seq 为资源最后被跟踪的命令缓冲序号（track 时 `AtomicU64::fetch_max`）。
- dispose_cb_resource 以 seq 为键做 hashbrown RawTable 探测（0x1418c65a0 起始 SIMD control-byte 探测 + `lock_slow`/`unlock_slow` + RawRwLock `lock_shared_slow`/`unlock_shared_slow`），把销毁动作写入 `HashMap<CommandBufferSeq, Mutex<DisposalBucket>>` 中该 seq 的桶。
- 即：**退役排水按资源 seq 归入对应命令缓冲的销毁桶；桶由命令缓冲时间线完成驱动排出** —— 与 raw-init 域 `CommandBufferDisposalBuckets = hashbrown::HashMap<CommandBufferSeq, Mutex<DisposalBucket>>` 结论对接，闭合 REVIEW §3.2/§3.5 断点。
- 每 dispose 调用还带类型特定载荷（texture 传 inner+0x30/+0x38；sampler 传常量 1/0）、`[rsp+0x28]=[resources+0x648]`、`[rsp+0x30]=&局部元素拷贝`。残留 U：`&resources.owning_device`（rcx）在 dispose 内被当 RawTable 用 —— `+0xa50` 字段同时被 get_maybe_tracked 读为 u16 device、被 dispose 当桶表 self，二者语义矛盾未解（待集成裁决）。

### 1.7 尾部

```
0x1418c1001  call 0x140613c20            ; __rust_dealloc drained Vec（见 1.2）
0x1418c1014  call 0x140bf92f0            ; immediate_empty_bucket(resources) —— 冲洗设备侧桶
0x1418c101d  call 0x140bdfcf0            ; drop_in_place<DisposalBucket>(&栈桶)
0x1418c102c..  call 0x140c18130 / 0x140c170f0   ; puffin scope 收尾
```

---

## 2. P0-2 ManagedResource<T> 槽布局 + 空闲链表（O）

### 2.1 槽 = 控制块（store 元素）

- 由 get_maybe_tracked（0x1418aeba0）读路径 + clone（0x140bd6b00）/drop（0x140be24b0）+ allocate 单态（0x1418ae7e0/0x1418ae8a0）写路径互证：

```
ManagedResource<T>（store AppendOnlyVec 槽 / 控制块 / 空闲链表结点，O）：
@0x00: AtomicU64 seq      （init 0；track 时 fetch_max 目标）
@0x08: AtomicI32 refcount （init 1；clone `lock inc dword [inner+8]`、drop `lock dec dword`）
@0x0c: u32 checksum       （与 handle+0x10 期望校验和比较）
@0x10: T resource 内联    （buffer=8B→槽 0x18B；descriptor_set=112B→槽 0x78B）
```
- get_maybe_tracked 判定序：`[h+0x14]u16 device == [resources+0xa50]` → `[h+0x16]u8 tag==1(buffer)` → `[inner+0xc] == [h+0x10]` → 返回 `inner+0x10`（资源地址）；带 tracker 时 `fetch_max(seq, 期望)` CAS 循环后返回。

### 2.2 RawManagedHandle（24B，O）

```
@0x00: *mut Mutex<Vec<DisposedRawResourceHandle>>  （退役队列，drop 时 0→1 字节锁 + 入队）
@0x08: *mut ManagedResource<T>                    （控制块指针）
@0x10: u32 期望校验和
@0x14: u16 device
@0x16: u8 资源类型 tag（0..7；3 无效；8=哨兵）   ← 修正 ALGORITHMS §1.2 “tracked 标志字节”：实为类型 tag
@0x17: 1B pad
```

### 2.3 空闲链表（allocate insert 路径，O）

- store+0x170 = 空闲链表头（AtomicPtr，CAS）。
- allocate<ManagedBuffer>（0x1418ae7e0）：
```
0x1418ae7e8  mov rax,[rcx+0x170]          ; head
0x1418ae7f0  test rax,rax; je push_new     ; 空 → AppendOnlyVec::push
0x1418ae7f8  mov rcx,[rax+0x10]; lock cmpxchg [rsi+0x170], rcx  ; CAS head→node.next（node+0x10 即槽+0x10 兼作 next）
0x1418ae807  mov [r8+0x10], rdx           ; 槽+0x10 = resource
0x1418ae80b  mov dword [r8+8], 1          ; refcount = 1
0x1418ae813  mov qword [r8], 0            ; seq = 0
; push_new:
0x1418ae81c  mov [rsp+0x28],0; mov [rsp+0x30],1; mov [rsp+0x38],rdx   ; 元素 {0, 1, resource}
0x1418ae83b  call 0x1418bbeb0             ; append_only_vec::AppendOnlyVec<T>::push（S_PUB32）
0x1418ae840  mov rcx,[rsi+0x160]; cmp rcx,rax; jbe panic
0x1418ae84f  call 0x140c111b0             ; append_only_vec::indices（S_PUB32）
0x1418ae85b  lea r8,[rdx+rdx*2]; shl r8,3; add r8,[rsi+rcx*8]        ; 槽 = base[rcx] + idx*24
```
- store 结构：0x178B = AppendOnlyVec 头 + `indices`（0x140c111b0）+ 计数@0x160 + 空闲链表头@0x170（原子 CAS）。元素 24B 对齐（`idx*24`）。
- 槽 +0x10 在空闲时存 `next`，占用时存 `resource` —— 空闲链表结点与槽内存复用（与 append-only-vec 0.1.2 rev 4031d67 语义一致，依赖直接复用）。

---

## 3. P0-3 refcount 原子宽度定论（O，字节级）

- clone 0x140bd6b00：`mov r8,[rdx+8]; lock inc dword ptr [r8+8]` —— **32 位** 原子，控制块 +0x08。
- drop 0x140be24b0：`lock dec dword ptr [rax+8]; jne skip` —— 32 位递减。
- PDB 注解 `AtomicI32::fetch_add/fetch_sub` 与机器码一致 → **原 refcount = AtomicI32（32 位）**，位置控制块 +0x08。rebuild `AtomicU32` 半对（有符号语义哨兵未证）。

---

## 4. P1-4 GpuOwned 变体与字段序（O）

### 4.1 类型参数

- `GpuOwned<T, Tracker>`，**Tracker 有默认值 = RawManagedHandle**（S_PUB32 同时见单参 `GpuOwned<[DescriptorSetBinding;2]>` / `GpuOwned<DescriptorSetBinding>` 与双参 `GpuOwned<(), ManagedHandle<ManagedBuffer>>`）。
- 单态实例（S_PUB32）：`GpuOwned<DSB, RawManagedHandle>`（bind 载荷 `Vec<(BindingIndex, GpuOwned<DSB>)>`）、`GpuOwned<[DSB;2], RawManagedHandle>`、`GpuOwned<(), ManagedHandle<ManagedBuffer>>`。

### 4.2 布局（O，GpuOwnedInner::clone 0x1403abd10 + drop_in_place 0x140161b50 + unwind drop 0x1403abdf0）

```
GpuOwnedInner<T, Tracker> = { resource T @0x00, tracker Tracker @0x00+size(T) }
GpuOwned<[DSB;2], RawManagedHandle>：
@0x00..0x40: resource [DSB;2]（2×32B）
@0x40..0x58: tracker RawManagedHandle（24B）
@0x58:       state Cell<GpuOwnedState>（1B，克隆/新建时=0）
size = 0x60（对齐后）
```
- 证据：clone 拷 [DSB;2] 到 dst+0x20..0x40 与 dst+0..0x20、`RawManagedHandle::clone(src+0x40→dst+0x40)`、`mov byte [dst+0x58],0`；drop_in_place 存 [src+0..0x40] 资源 + [src+0x40..0x58] tracker 并经 `RawManagedHandle::drop(0x140be24b0)` 析构 tracker；unwind drop（0x1403abdf0）`lea rbp,[rdx+0x80]` 再 `RawManagedHandle::drop([rbp+0x40])`。
- **字段序定死：resource 在前、tracker 在后、state Cell 在末尾**（修正 ALGORITHMS §1.2 “{state, inner}”顺序未定；也闭合 REVIEW §3.3）。

### 4.3 GpuOwnedState

- `impl$22::track`（线397）内联 `Cell::set` + `Cell::replace` + `impl$17::track`（线250 → seq 跟踪）；`GpuOwned::mark_unused`（线447）内联 `Cell::set`。克隆/新建 state=0。
- 枚举判别值（0=Unused? / 1=Tracked? / 其余?）**残留 U**：未在本轮找到写非零 state 的编译宿主（track 宿主在 rg/country-core pass 闭包内联）。候选映射 tracked=1 待补。

---

## 5. P1-5 swapchain 锁字编码（O，非 vanilla parking_lot）

### 5.1 机器码

- 写锁（acquire_next 0x1418cb54f）：`mov ecx,8; xor eax,eax; lock cmpxchg qword ptr [rdx], rcx` —— **[lockword] 0→8（bit3=写者）**；失败 → 慢路径 0x1418cbf09。
- 读锁（desc 0x1418cb10d）：`mov rax,[rdx]; and ecx,8; cmp rax,-0x10; setae` —— 快路径条件 `(state&8)==0 && state < 0xfffffffffffffff0`；`lea rcx,[rax+0x10]; lock cmpxchg [rdx], rcx` —— **每读者 +0x10（16）**；否则 `lock_shared_slow`（1s 超时 0x3b9aca00）。
- 与 REVIEW 观察（写 0→8 / 读 +0x10）一致。

### 5.2 对照 Cargo.lock

- `evidence/multi-version/B-original-Cargo.lock` 与 `skeleton/B/Cargo.lock`：parking_lot 0.11.2（checksum 7d17b780…）+ 0.12.1（checksum 3742b2c1…），source=**registry**，**无 git/fork**。
- stock parking_lot 0.12.1 RawRwLock 写锁快路径为 `state 0→LOCKED_BIT(=1)`（bit0），与本二进制 bit3=8 不符；读者步进亦非 0x10 系。

### 5.3 结论

- 二进制锁字编码（写 bit3=8 / 读 +0x10 / 慢路径阈 ≥-0x10）**非 stock parking_lot 0.12.1**；Cargo.lock 未声明 fork → **本机构建使用本地改常量（或自定义 RwLock），不能归因于 Cargo.lock 可见的“Bevy 分叉”**。锁协议按二进制编码登记，rebuild 委托 raw 内部承担锁语义的等价性判定以本编码为准（且 raw 侧同样需按二进制编码核对）。

---

## 6. P1-6 disposal 对照改判（文档口径修正）

- 原始：`CommandBufferDisposalBuckets = hashbrown::HashMap<CommandBufferSeq, Mutex<DisposalBucket>>`（每桶 parking_lot Mutex；insert 断言 prev.is_none，remove 缺桶 panic “bucket removed prematurely or command buffer begin() never called?” @0x142d069c1）。
- rebuild：`raw/disposal.rs` = `BTreeMap<u64, Vec<DestroyAction>>` 无锁、覆盖用 forget、remove 返回 Option。
- ALGORITHMS §3 原标 `✓（名称/职责对上）` → **应改 ✗/半**：容器类型、键类型、每桶互斥、insert/remove 语义均偏离（raw-init 域证据 + 本证据 §1.6 的 hashbrown 探测确认）。
- 补登记：rebuild `raw/graveyard.rs`（设备级 idle submit epoch 延迟销毁）在 ALGORITHMS 未登记 —— 与原版“仅按命令缓冲 seq 时间线”双轨并存，判定缺口保持 U/✗（需 submit 域 seq→timeline 环路闭合）。

---

## 7. 残留 U（本轮未闭合）

1. dispose_cb_resource 首参 `&resources.owning_device`（+0xa50）被当 hashbrown RawTable 用 vs get_maybe_tracked 读为 u16 device —— 字段双语义矛盾（§1.6）。
2. DisposalBucket 每槽 8B 元素的确切载荷（destroy 动作编码）未逐项解码。
3. GpuOwnedState 判别值（§4.3）。
4. DisposedRawResourceHandle +0x0f 字节的写入方（drop 只写 15B）。
5. 栈桶 0xE0 vs DisposalBucket 0xF0 的 0x10 差（可能含尾部非 Vec 字段，drop_in_place 经 0x140be1710 处理）。
