# T3 CONTRACT — ivy 常量/flag_8c/send_to_gpu 定点定案

> 逆向 agent ① 交付。逐项定点（RNE 二进制 float 检索 + capstone ivy_spawner 常量提取 +
> grower.asm 手解）。状态：**VERTICAL_MIN/0.68 两项定案可落地；count_range.end/seed 待取证**。

---

## 1. VERTICAL_MIN 疑点定案 = **0.0**（纠错 0.000928482）

- **结论**：`vertical_u` 高度采样的下限 = **0.0**，位于 **0x142960690**（f32 0x00000000）。
  ivy_spawner（0x14094cf90）常量提取：`0x142960690 = 0.0` 与 `0x142925904 = 1.0` 同时被
  `(fVar31 <= XMM11) && (0x142960690 <= fVar31)` 夹取门使用（decomp-0x14094cf90.txt 行 ~610）。
- **0.000928482 为编造值**：其 f32 字节 `0x3a73655f` 在整 EXE 中 **0 次出现**（rne-read 全文件检索）。
- **② 待办**：`ivy_spawner.rs` `VERTICAL_MIN` 0.000928482 → **0.0**；测试断言 `[0.0, 1.0]`。
  公式（O）：`value = (rand * 1.5 - 0.75) / divisor + addend`，clamp `[0.0, 1.0]`；
  常量 1.5@0x1429258d0、-0.75@0x142ab3220、1.0@0x142925904、0.0@0x142960690、
  epsilon 0.001@0x1429cd060、1e-20@0x142a7baa0（divisor.max 用）。

## 2. 0.68 vs 0.63 冲突定案 = **0.68**（随机根法向偏移）

- 0x142ab31f0 = `7b142e3f` = **0.68f**；0x142ab3210 = `7b142ebf` = **-0.68f**（ivy_spawner 常量表，
  capstone 直接引用）。当前实现 0.68 **正确**。
- 0x1429258d4 = `ae47213f` = **0.63f**：不同常量（非根偏移；用途待定，未在 ivy_spawner 引用集内）。
- 相关：±0.315@0x142aa3280/0x142ab31b4（切向偏移候选）、2.56@0x142ab3214（=1.6² 随机根距离平方，
  现 `RANDOM_ROOT_DISTANCE_SQ=2.56` 吻合）。
- SPEC 若仍写 0.63，是取错了常量表地址；以 0x142ab31f0 为准。

## 3. flag_8c 消费的 leaf 清理谓词 = **affected-leaves 局部 Vec<u64>**

grower（0x14094a910）0x14094ae63..96 手解：
```
0x14094ae63  cmp byte [r14-4], 0          ; ivy.flag_8c（记录 +0x8c，r14=记录基）
0x14094ae69  je   skip
0x14094ae6b  rax = [rbp+0x50]             ; 受影响叶 id 列表 base
0x14094ae6f  rdx = [rbp+0x58]             ; 列表 len
0x14094ae73  r8  = [r14-0x30]             ; 记录 +0x60 字段
0x14094ae80  loop: while i<len: if r8 == list[i] → je 命中
0x14094ae96  mov byte [r14-4], 0          ; 命中 → 清零 flag_8c
```
- **list 身份**：grower 局部 `Vec<u64>`（ptr@rbp+0x50、len@rbp+0x58；函数退出处 0x14094c03b/
  0x14094c5c3 经 `0x140613c20` drop，元素 stride 8）。为**墙洞命中产生的受影响叶/根 id 列表**
  （洞约束 0x14094b7be..b86e 段填充）。
- **谓词**：`flag_8c 置位 且 记录+0x60 字段 ∈ 受影响叶列表 → 清零 flag_8c`。
- ② 待办：`ivy_grower.rs` 现"直接清零"→ 补局部 `Vec<u64>` affected_ids（洞命中时 push 该洞相关
  叶 id）+ 消费谓词查表。`+0x60` 字段语义（叶组 id/墙段 id）待取证；不阻断（可先按洞命中的
  ivy id 建模，标注）。

## 4. count_range.end（SPAWN_COUNT_RANGE 上限）— **机制定案，RNG 界值待取证**

- 起点 **0x14（20）** 定案：ivy_spawner（0x14094cf90）spawn_ivy 调用点 `[rsp+0x28] = 0x14`
  （capstone 0x14094db33）。`[rsp+0x30]` = 记录数组基指针（= `[rcx]`，0x14094db1f/0x14094d010；
  DELTA"存储指针"**证实**，非计数）。
- **每根点计数**（spawn_ivy 0x1408696f0，O）：`count = 0x14 + usize_less_than(RNG_bound)`：
  `mov rdx,[rbp+0x190]`（RNG 界）→ `call 0x141625a50` → `mov r14,rax` → `add r14,r13`（r13=0x14）
  → `[rbp+0x80]=r14`（count 存盘）。`RNG_bound`（[rbp+0x190] 的调用方映射）**待取证**。
- 现 `20..40` 的 40 上限**未证实**（非简单常数；上限 = 20 + RNG 界）。② 保持注释锁定，
  **不改为猜测值**；RNG_bound 值经 ivy_spawner 栈参完整映射后可定（低优先）。

## 5. send_to_gpu seed / wall_normal — **待取证（GPU 后置）**

- seed：现 `frame` 为候选；shader 布局（instanced_ivy_branch/leaf.raster.hlsl）seed 字段的
  原生来源未在 send_to_gpu decomp（0x140a9b4e0/0x1408c1580）直接可见（位图迭代 + 段聚合，
  decomp-0x140a9b4e0.txt）。**待取证，不编造**。
- wall_normal：现 `(-d.z/len, 0, d.x/len)`（XZ 平面切向垂线）是合理候选；原生段数据里法向
  计算的精确 swizzle 未逐指令核。**待取证**。GPU 真机执行后置，本项不阻塞 CPU 侧。

## 6. 证据索引
- ivy_spawner 常量：capstone 0x14094cf90 全 float-const 提取（0x142925870=0.5、0x1429258d0=1.5、
  0x142925904=1.0、0x142960690=0.0、0x1429cd060=0.001、0x142a7baa0=1e-20、0x142ab31f0=0.68、
  0x142ab3210=-0.68、0x142ab3214=2.56、0x142ab3220=-0.75、0x142aa3280=0.315）。
- grower.asm.txt 行 305-315（flag_8c 消费）、1213/1518（list drop）。
- decomp-0x14094cf90.txt（ivy_spawner budget/batch/vertical_u）。
- rne-read：0x142960690 全字节、0x1429258d4、0x142ab31e0/3200/3218；全文件 0x3a73655f 检索 0 次。
