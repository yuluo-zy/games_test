# clutter F3 补挖：populate_autoclutter_regions 种类分派完整契约（re-analyst tg_xb1，2026-10-16）

> 任务来源：coordinator 转 reviewer-B F3 —— restorer 的 `clutter_kind_for_cell` 恒返回 1，
> 机器码 44KB switch 种类分派未实现。本文件以 **B/RNE 机器码**（workspace/input/tiny-glade.exe，
> capstone 直解 + 既有 reva 187KB `evidence/toolchain/clutter/populate_decompiled_full.c`）为据，
> 精读 populate_autoclutter_regions（0x1422d6ed0, size 44132, 源行 114–950）的种类分派。
> O=机器码证实 / I=推断 / U=未知。反编译落盘：groupB/decomp/decomp-0x1422d6ed0.txt（Ghidra 201KB，
> 碎片化严重，**以本文机器码结论为准**）。

## 1. 核心事实（O，机器码）

**clutter 的"种类"不是单一字节**：每脏格候选记录（stride 0x48 区域表项）携带两个判别字节 + 两个写入字段：

| 记录偏移（表项基） | 字段 | 消费方 |
|---|---|---|
| +0x00 | u64 key（ClutterRegionId = `calculate_hash<tuple<GlobalId,u32>>` 0x140fabc80） | 区域表键 |
| +0x40 | byte **mesh-type**（网格挑选类型） | 过滤器判别（r12b，0x1422d810e）+ 跳表 DAT_14304382c（0x1422d7dd7） |
| +0x41 | byte **kind**（种类） | 跳表 0x143043818（0x1422db587）→ 预设表构建 |
| +0x42 | u16 candidate_index | （写入点未定位，U；**非恒 0**） |
| +0x44 | byte epoch（= state+0x90） | remove_stale_regions |
| +0x20..+0x3c | 7×f32 中心/基向量 | add_region is_equivalent 比较（0x140ee3fd0 decomp） |

注：populate 内记录指针 = 表项基 + 0x48（负偏移读字段）；`[rax-0x28]/[rax-0x24]` = +0x20/+0x24。

## 2. kind 跳表（record+0x41，O）

`0x1422db580: movzx eax, byte ptr [rax-7]; cmp rax,4; ja 0x1422dd7ca; LEA 表0x143043818; 分派`。
表 0x143043818（5 项，基址相对偏移）：

| kind | 目标 | case 行为（O：预设资源 [rbp+0x6f8] 读取） |
|---|---|---|
| 0 | 0x1422db5b0 | 读预设槽 +0x48/0x40/0x18/0x20/…（权重常量各异） |
| 1 | 0x1422dd1db | 读预设槽（同构，槽序不同） |
| 2 | 0x1422dc378 | 读预设槽 +0x50/… |
| 3 | 0x1422dcaab | 读预设槽 +0xc0/… |
| 4 | 0x1422db834 | 读预设槽 |
| >4 | 0x1422dd7ca | 读预设槽 +0x168/…（default） |

每槽读取模式（O，0x1422db5b0/0x1422dc378/0x1422dcaab/0x1422dd7ca 逐条验证）：
`rec = [预设资源 + off]; if ([rec+0x30] == -1) 权重 = 0.0; else 权重 = .data 常量`——
**预设记录 +0x30 qword = Option 哨兵（-1 = None → 权重 0）**；每槽 {ptr, f32 权重} 对写入
0x10-stride 候选表（local_188；计数 [rbp+0x698]）。**即 kind = 选中"该格可用哪组网格 + 各网格权重"的枚举**。

**万圣节门控（O）**：`is_halloween2024`（0x140b46860）@0x1422d78a5；true → 万圣节预设表（0x1422d78b2，
槽序 +0x70..+0x168）；false → 上述 kind 跳表（0x1422db580）。

## 3. mesh-type 跳表（record+0x40，O）

`0x1422d7dd7: movzx eax, byte ptr [rax-8]; LEA 表0x14304382c; 分派`。表 0x14304382c（28 项，基址相对）：

| 值 | 目标 | 行为（O，反汇编/反编译） |
|---|---|---|
| 0,1 | 0x1422d7df6/7dfa | **加权随机挑网格**：权重表（0x10 stride {ptr,f32}）累和；`init_rng_u64(键哈希)` + `fastrand::f32`（0x140caf380）；`r ≤ 累和(权重/总)` 选中（bounds_check 0x1422de821）；随后位面数学（见 §4） |
| 2 | break | 跳过（不挑网格） |
| 3 | 0x1422d7e34 区 | 候选槽 +0x25 |
| 4 | 0x1422d7e32 区 | 候选槽 +0x26 |
| 7 | 0x1422d839d | 计数累积：`[cand+0xad] += 表值`（byte） |
| 9,10,0x11,0x14,0x15,0x10,0x16 | 0x1422d8902 | 特殊分支（第三跳表 0x143043840 分发，见 §5） |
| 其余 | 0x1422d7d8e | 默认 |

## 4. 过滤器与判别条件（O，0x1422d8107–0x1422d8468）

每脏格（位集迭代 → cell 解码 → 记录填充）：
1. `r12b = record+0x40 (mesh-type)`；`r12b += 0xfe`（= type-2 字节运算，供 cmp）。
2. **RaycastWorld::legacy_raycast**（0x1408cc870，掩码 `0x30fb`）→ `[rbp+0x1d4] == 2` 才继续水面采样（≠2 → 0x1422d8320 另一路径）。
3. 地形高度 `TerrainHeightsData::sample_at_world_xz`（0x141246fb0）→ 位面矩形（0x140c9cde0）。
4. **4 角水面采样** `bilinear_sample_at_world_xz<512>`（0x140a14df0 ×4，半径常量 130.0 = 0x142a7b658）→ 和 → `maxss(0,sum); minss(1.0)` clamp。
5. **判别 A（水位）**：`0.0 < clamp_sum`（阈值 **0x142960690 = 0.0**，非 0.01！）：
   - wet：`[rbp+0x6d8] = 1`；`cmp r12b,3; jb 0x1422d7da2`（type<3 → 网格挑选路径）；`jae 0x1422d8468`。
   - dry（0x1422d83e0）：`cmp r12b,3; jae 0x1422d845d → 0x1422d8468`；type<3 → 固定位
     `pos = (记录+0x24 取负, 记录+0x20)`（`[rax-0x28]/[rax-0x24]` + `xorps 0x80000000`）→ 0x1422d8510。
6. **判别 B（type≥3 → 圆周落点）**（0x1422d8468）：`init_rng_u64(键哈希 0x140fabc80 结果)`（0x140c96270）
   → `fastrand f32 r`（0x140caf380）→ **θ = r × 2π**（常量 0x142934674 = 6.283185；`(1-r)×0.0 + r×2π`）
   → `cosθ/sinθ`（0x142923e80/0x142923fd0）→ 位面中心 = (cosθ, sinθ)（单位圆）。

**0.01（0x3c23d70a）的真实用途（O）**：不是水位阈值；packed 常量 **0x1429cd300 =
{0.01,0.01}** 用于 mesh-type 0/1 的**位面落点最小钳制**（`maxps(0.01, Δ)` 后 `floorf` 网格化、
xvy swizzle）——多基线文档"==2 走水位阈值 0.01"为误读，**水位阈值是 0.0（0x142960690）**；
0x142960674=0.01 为另一处单采样 skip 门（片段内，语义 I）。restorer 的 `kind==2 && water<=0.01 → continue`
模型与机器码结构不符（机器码无 kind==2 水位门）。

## 5. 第三跳表（O）

`0x1422d8393: uVar28 = 结果位字节 & 0xff; LEA 表 0x143043840; 分派`。表 0x143043840（24 项，
0x14304382c 的偏移副本：0→7da2、2→839d、4→de849、14→89df、16→de875、18→de881、19→de885、
0x15→8961、0x1a→8ee2、0x1b→8ea6、0x17→8ed1…）——mesh-type 特殊分支的二级行为表。

## 6. 区域 center / candidate_index（对 restorer 的纠正）

- **center 非恒"格中心"**（O）：wet/type<3 = 地形高度修正位面中心（0x140c9cde0(&rbp+0x608) 结果，
  含 terrain 采样与 ×2 格坐标换算）；type≥3 = RNG 圆周点（cos/sin × 2π，种子=键哈希）。
  restorer 的 `cell_center_world(x,y) = ((x+0.5)*8 - 65, ...)` 为推断（I），与机器码换算（`×2 后 trunc`
  0x1422d781e 与位面中心流）**未对齐**——按 §4 流程重写。
- **candidate_index（u16@+0x42）写入点未定位（U）**：不得恒 0 冒充；候选索引由区域/候选表
  交互产生（add_region 的 u16 push 语义，见 placement doc §AutoClutterState：state+0x30 Vec<u16>）。

## 7. restorer 落地建议（clutter_kind_for_cell 重写）

```text
per cell:
  kind = ???（record+0x41 的写入方未在 populate 内找到——I：由 filter_clutter_object 闭包
         按"可用网格组"选定；写入点在函数外/内联处，见 decomp 分段）
  mesh_type = ???（record+0x40，同源）
  # 机器码已钉的判别链（可直接落地）：
  water_sum = clamp01(Σ 4×bilinear(x±130, z±130))
  hit = legacy_raycast(掩码 0x30fb)
  if hit==2 and water_sum > 0.0:        # 0.0 = 0x142960690
      if mesh_type < 3: weighted_mesh_pick(mesh_type 0/1 分支)   # 权重表+fastrand
      else:            circular_pos(θ = rng*2π)                  # type>=3
  else:  # dry
      if mesh_type < 3: fixed_pos(record+0x20/+0x24, y 取负)
      else:            circular_pos(θ = rng*2π)
  # kind → 预设槽组（0x143043818: 0→+0x48/0x40/0x18/0x20…, 2→+0x50…, 3→+0xc0…, >4→+0x168…,
  #    万圣节→+0x70..+0x168 组）；槽记录 +0x30==-1 → 权重 0
  # mesh-type 0/1 落点：maxps(0.01, Δ) → floorf → xvy（0.01=0x1429cd300）
```

## 8. 证据落盘

- 本文件：RE-ANALYSIS-F3.md（契约）。
- `groupB/decomp/decomp-0x1422d6ed0.txt`（Ghidra 201KB，碎片化参考）。
- `evidence/toolchain/clutter/populate_decompiled_full.c`（reva 187KB，跳表 case 全结构）。
- 关键跳表：0x143043818（kind，5 项）、0x14304382c（mesh-type，28 项）、0x143043840（24 项）。
- 关键常量：水位阈值 0.0@0x142960690、2π@0x142934674、0.01@0x142960674/0x1429cd300、
  采样半径 130.0@0x142a7b658、1.0@0x142925904、射线掩码 0x30fb。

## 9. 仍 U（不臆造）

- record+0x40/+0x41 两字节的**写入方**（filter_clutter_object 闭包内，populate 内未见写指令——疑位于
  0x1422d73ce–0x1422d8107 的位集迭代/记录初始化区，因反汇编窗口未全覆盖；**供下一轮定点**）。
- candidate_index（u16@+0x42）写入点与语义。
- 万圣节预设槽的精确权重常量表（.data 各 case 逐槽值，反编译可见、未全读）。
- mesh-type 3/4 的候选槽 +0x25/+0x26 语义与 0x1422d8902 特殊分支内部（第三跳表各目标体）。
