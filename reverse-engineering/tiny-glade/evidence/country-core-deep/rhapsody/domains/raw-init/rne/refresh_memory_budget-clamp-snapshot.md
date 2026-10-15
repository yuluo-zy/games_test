# refresh_memory_budget（FUN_140c01620）滞回公式 + 快照排布（审查轮闭合证据 P0-3）

- 日期：2026-10-15。标注 O/I/U。方法：decomp-0x140c01620.txt（16 处钳位块 + 快照组装段）+ rne-read 原始字节复核峰值 getters。

## 1. 滞回钳位公式：除数 = max(1, old>>3)（O）

decomp 16 处重复（每堆一个），以某堆为例（line 297-311）：
```
delta = (old >> 3) + (old >> 3 == 0);        # = max(1, old/8)
lo    = old - delta   （若 delta<=old 否则 0）
hi    = old + delta
new   = clamp(驱动查询值, lo, hi)             # if lo<=hi: min(max(new,lo),hi)
```
16 个堆各自一个 uStack_XXX 槽位（budget 数组）。**旧 doc"旧±旧/8"在小预算堆（old<8）不成立：实为 `[old - max(1,old>>3), old + max(1,old>>3)]`，old∈[0,7] 时钳到 [old-1, old+1]。**

## 2. 快照 / ArcSwap 排布（O）

```
PhysicalDevice@+0x608  ArcSwap<快照>
快照（0x110B，272B）：16 堆 {budget: u128, usage: u128} 对，预算数组起点 +0x20、每堆步长 0x20
  —— device_local_memory_budget_bytes(FUN_140c006a0) 按 plStack_50 + i*4 + 4（+0x20+i*0x20）读 budget、
     +6（+0x30+i*0x20）读 usage（各取 low/high u64），heap flags 掩码 DEVICE_LOCAL(0x1)
ArcSwap 分配 0x120B：{refcount@+0, u64@+8, snapshot 0x110B@+0x10}；+0x608 存的指针 = 分配+0x10（指向 snapshot 头）
刷新路径（line 402-429）：
  func_142923c00(&plStack_368, auStack_488, 0x110)   # 栈上组装 0x110B
  puVar23 = alloc(0x120); *puVar23=1; puVar23[1]=1;  # ArcSwap 引用计数初始化
  func_142923c00(puVar23+2, &plStack_368, 0x110)     # 写 snapshot
  LOCK; *(param_1+0x608)=puVar23+2; UNLOCK           # ArcSwap store
  旧快照 refcount-- → 0 则 FUN_140bdd240（drop）
```

## 3. 峰值字段（GpuAllocator，O，rne-read 原始字节复核）

| 字段 | 偏移 | getter 字节证据 |
|---|---|---|
| peak_total_allocated_bytes | +0x680 | `48 8b 81 80 06 00 00 c3`（mov rax,[rcx+0x680]; ret）|
| peak_allocator_report(Option) | +0x620 | `31 d2 48 3b 91 20 06 00 00 … 48 0f 40 c2 c3`（null→None）|
| peak_device_local_bytes | +0x670 | `48 8b 81 70 06 00 00 c3` |
| peak_system_bytes | +0x678 | `48 8b 81 78 06 00 00 c3` |
| track_peaks（bool，GpuAllocator::new 写） | +0x688 | doc §2.5（allocate 采样/free 不采样，16 案例验证）|

## 4. rebuild 对照

rebuild `refresh_memory_budget` 直查直存无滞回 → **偏差成立**；应补逐堆 `clamp(新, [旧-max(1,旧>>3), 旧+max(1,旧>>3)])`。
快照字段（+0x20 起 {budget,usage}×16，步长 0x20）供 rebuild PerformanceInfo/snapshot 对齐。
