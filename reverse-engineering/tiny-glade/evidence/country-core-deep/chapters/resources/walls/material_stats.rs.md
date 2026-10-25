# resources/walls/material_stats.rs

> 文件作用：墙材质统计：7 特征长度 prev@+0x28..+0x40/cur@+0x90..+0xa8（plain/lintels/arches/栅栏/gates/stairs/sewers）、8 段/柱计数 u32@+0xac..+0xc8、seed u64@+0x88、pillar placements Vec@+0x70{cap,ptr,len}（自旋锁@+0x68 保护）；add_*_length 为 CAS 原子累加（加本特征扣 plain 预算）；which_material_changed_the_most = 7 对 |cur-prev| argmax（plain 带 -0.01 bias）。

函数总计 34：业务 18 / 内联无独立符号 14 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::material_stats::MaterialStats::init_this_update` | 0x140a88280 | 45 | init_this_update（帧初状态推进） | O | wall\decomp\decomp-0x140a88280.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements` | 0x140a88320 | 63 | set_pillar_placements（自旋锁 + futex 式等待 + 换入 Vec@+0x70{cap,ptr,len}） | O | wall\decomp\decomp-0x140a88320.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count` | 0x140a88420 | 75 | set_lintel_pillars_count（u32@+0xb4） | O | wall\decomp\decomp-0x140a88420.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count` | 0x140a88430 | 81 | set_arch_pillars_count（u32@+0xbc） | O | wall\decomp\decomp-0x140a88430.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_path_segmentation_count` | 0x140a88440 | 87 | set_path_segmentation_count（u32@+0xac） | O | wall\decomp\decomp-0x140a88440.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_lintels_length` | 0x140a88450 | 93 | add_lintels_length（CAS +0x94+=x, +0x9c-=x） | O | wall\decomp\decomp-0x140a88450.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_arches_length` | 0x140a884a0 | 98 | add_arches_length（CAS +0x98+=x, +0x90-=x） | O | wall\decomp\decomp-0x140a884a0.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_gate_length` | 0x140a884f0 | 103 | add_gate_length（CAS +0xa0+=x, +0x90-=x） | O | wall\decomp\decomp-0x140a884f0.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_sewer_length` | 0x140a88540 | 108 | add_sewer_length（CAS +0xa8+=x, +0x90-=x） | O | wall\decomp\decomp-0x140a88540.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_stairs_length` | 0x140a88590 | 113 | add_stairs_length（CAS +0xa4+=x, +0x90-=x） | O | wall\decomp\decomp-0x140a88590.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_lintels_segments_count` | 0x140a885e0 | 118 | set_lintels_segments_count（u32@+0xb0） | O | wall\decomp\decomp-0x140a885e0.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_arches_segments_count` | 0x140a885f0 | 124 | set_arches_segments_count（u32@+0xb8） | O | wall\decomp\decomp-0x140a885f0.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_gates_segments_count` | 0x140a88600 | 128 | set_gates_segments_count（u32@+0xc0） | O | wall\decomp\decomp-0x140a88600.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_sewer_segments_count` | 0x140a88610 | 132 | set_sewer_segments_count（u32@+0xc4） | O | wall\decomp\decomp-0x140a88610.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_count` | 0x140a88620 | 136 | set_fences_segments_count（u32@+0xc8） | O | wall\decomp\decomp-0x140a88620.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_seed` | 0x140a88630 | 140 | set_fences_segments_seed（u64@+0x88） | O | wall\decomp\decomp-0x140a88630.txt |
| `country_core::resources::walls::material_stats::MaterialStats::max_pillar_movement` | 0x140a88640 | 204 | max_pillar_movement | I | wall\decomp\decomp-0x140a88640.txt |
| `country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most` | 0x140a887d0 | 280 | which_material_changed_the_most（7 对 |cur-prev| argmax 索引 0..6，plain 带 -0.01 bias，前 5 对和>0 门控） | O | wall\decomp\decomp-0x140a887d0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::material_stats::MaterialStats::pillar_count`（行 67）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::path_segmentation_delta`（行 143）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::has_fence_seed_changed`（行 157）— 谓词/校验
- `country_core::resources::walls::material_stats::MaterialStats::fence_length_delta`（行 161）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta`（行 166）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta`（行 171）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::gates_length_delta`（行 176）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta`（行 180）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::lintels_pillar_delta`（行 194）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::arches_pillar_delta`（行 199）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::arches_length_delta`（行 220）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta`（行 225）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::has_material_switched`（行 230）— 谓词/校验
- `country_core::resources::walls::material_stats::MaterialStats::latest_update_material_proportions`（行 267）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
## 实现方式（算法级）

### MaterialStats 布局（O，DIGEST §1.6 全落地）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x28/+0x2c/+0x30/+0x34/+0x38/+0x3c/+0x40 | prev 特征长度 7×f32（plain/lintels/arches/?/gates/stairs/sewers） | which_material_changed_the_most 0x140a887d0 差分 |
| +0x68 | 自旋锁字节（LOCK bts/btr） | set_pillar_placements 0x140a88320 |
| +0x70/+0x78/+0x80 | pillar placements Vec（cap-first {cap,ptr,len}，8B 元素） | set_pillar_placements / init_this_update |
| +0x88 | fences_segments_seed u64 | set_fences_segments_seed 0x140a88630 |
| +0x90/+0x94/+0x98/+0x9c/+0xa0/+0xa4/+0xa8 | cur 特征长度 7×f32（同序） | add_*_length 族 |
| +0xac/+0xb0/+0xb4/+0xb8/+0xbc/+0xc0/+0xc4/+0xc8 | 段/柱计数 u32（path_segmentation/lintels_segments/pillar/lintel_pillars… 见 DIGEST） | setter 族 0x140a88440..0x140a88620 |

### init_this_update 0x140a88280（O；纠错：不含 cur→prev 拷贝循环）

```
签: (self, base_len u32, flag u8)
if (pillar Vec cap@+0x70 != 0) → 释放元素数组（func_0x140613c20(ptr@+0x78, cap<<3, 4)）
+0x68 锁字节 = 0；pillar Vec 三字段重置；+0x88 seed=0
cur 特征长度/计数清零: +0x90..+0xb8 qword、+0xbc、+0xc4 = 0
if (!flag): +0x90 = base_len；base_len = 0      // 无 lintels 帧: plain 预算 = base_len
else:      +0x90 = 0                            // 有 lintels 帧: plain=0
+0x9c = base_len                                // lintels 预算
```
注意：任务假设"cur→prev 拷贝循环 7 对"不在此函数内（prev 拷贝/推进在调用方或他处，U）。语义：帧初重置 cur 特征并注入 plain/lintels 初始预算。

### set_pillar_placements 0x140a88320（O，完整 futex 式同步）

```
签: (self, new_Vec {cap,ptr,len})
锁: LOCK; if (*+0x68 == 0) *+0x68 = 1; UNLOCK        // test-and-set
若忙: FUN_1428cd210(&+0x68, new_Vec, 1000000000)      // park（等待队列入队，1e9ns=1s 超时）
FUN_140c81c70（内存序/重试）
释放旧 pillar Vec（cap@+0x70 != 0 → 8B 元素数组）
换入: +0x70=cap、+0x78=ptr、+0x80=len
FUN_140c81e10（fence）→ 释放锁（owned 才清 0）
若原持有者非我: 遍历全局等待队列 plRam00000001436308f0：
  fibonacci 哈希（puVar1 * -0x61c8864680b583eb）定位桶（0x40B 条目，锁地址键）→ 出队唤醒匹配等待者 → 引用计数递减（<4 或 bit1 检查后可能释放）
```

### add_*_length 六函数（O，手解 CAS 重试循环）

均为 `cmpxchg [f32 目标], [新值]` 原子累加循环（重试直至 CAS 成功），语义（DIGEST §1.6）：
- add_lintels_length 0x140a88450：+0x94 += x，+0x9c -= x
- add_arches_length 0x140a884a0：+0x98 += x，+0x90 -= x
- add_gate_length 0x140a884f0：+0xa0 += x，+0x90 -= x
- add_sewer_length 0x140a88540：+0xa8 += x，+0x90 -= x
- add_stairs_length 0x140a88590：+0xa4 += x，+0x90 -= x
（lintels 扣 lintels 预算 +0x9c；其余扣 plain 预算 +0x90）

### which_material_changed_the_most 0x140a887d0（O；PDB 名 arches_length_delta 错位，实为 which_material_changed_the_most）

```
7 对 |cur-prev|（& 0x7fffffff = fabs 位掩码，常量 _UNK_14292e980/984）：
  fVar8  = |+0x9c - +0x34|    // 第 3 对（?）
  fVar9  = |+0x98 - +0x30|    // 第 2 对（arches）
  fVar7  = |+0xa0 - +0x38|    // 第 4 对（gates）
  fVar10 = |+0x94 - +0x2c|    // 第 1 对（lintels）
  fVar11 = -0.01f@0x142ad8a14 + |+0x90 - +0x28|   // 第 0 对（plain，带 -0.01 bias）
  fVar3  = |+0xa8 - +0x40|    // 第 5 对（sewers）
  stairs = |+0xa4 - +0x3c|    // 第 6 对（stairs）
门控: if (fVar8 + fVar9 + fVar10 + fVar11 + fVar7 > 0)   // 前 5 对增量之和 > 0
argmax 链（分支序，>= 平局更新 => 后比较者平局胜）:
  max(fVar8, fVar11) → lintels → arches → gates → sewers → stairs
  索引 uVar5/uVar4: lintels=1、arches=2、gates=4、sewers=5
  最终: uVar1 = 6（stairs 默认）；仅当 |stairs| < 当前 max 才回退 uVar4（前 max 索引）
返回值: 0..6（0=plain,1=lintels,2=arches,3=?,4=gates,5=sewers,6=stairs），门控失败返回 7
```
**纠错**：机器码比较序 = sewers(索引 5) 先、stairs(索引 6) 后；DIGEST §1.6"5=stairs,6=sewers"顺序反了（语义不变，索引号按此处为准）。

### max_pillar_movement 0x140a88640（I）

pillar placements 位移量计算（遍历 pillar Vec 求最大移动距离），未逐行（I，等待 O 复核）。

### 新常量表

| 常量 | 地址 | 语义 |
|---|---|---|
| 0x7fffffff 位掩码 | _UNK_14292e980 / 0x14292e984 | fabs（cur-prev 差分） |
| -0.01f | 0x142ad8a14 | plain 增量 bias |
| 1000000000 | 立即数 | futex park 超时（ns） |
| fibonacci 哈希因子 | -0x61c8864680b583eb | 等待队列桶定位 |

### 跨模块调用

FUN_1428cd210（park/wait 队列）、FUN_140c81c70/FUN_140c81e10（内存序）、全局等待队列 plRam00000001436308f0、FUN_140613c20（元素释放）、FUN_1428d9518（panic）。set_pillar_placements 与 DeferredDecoratorOpEvents::send 共用同一 futex 等待队列机制。MaterialStats 消费于 publish_public_wall_state 0x140a8b530（FUN_1409e9640）与材质音频系统。

### 遗留 U

- prev 特征拷贝循环的实际位置（调用方/帧推进系统，U）。
- +0x34（第 3 对）特征名（I：lintels 预算余量/檐口）。
- max_pillar_movement 0x140a88640 逐行算法 I。
