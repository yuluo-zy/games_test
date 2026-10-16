# systems/ivy/ivy_grower.rs

> 文件作用：**常春藤生长：10 相状态机（grower 7.9KB 单函数）沿墙推进折线**

函数总计 11：业务 4 / 内联无独立符号 3 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_grower::ivy_grower` | 0x14094a910 | 47 | 系统:生长 | O：ivy_grower（0x14094a910..0x14094c7fb，7915B 单函数）——10 相状态机（跳表 0x142ab3be0，索引 world+0x118）；主闭包 0x14094ac20：last_point→get_direction→closest_segment_index→墙跳→add_point | collision-eco\decomp\decomp-0x14094a910.txt |
| `country_core::systems::ivy::ivy_grower::closest_segment_index` | 0x14094c800 | 401 | 几何/碰撞 | O：closest_segment_index（666B）——**56B 步长**墙段（循环 `add r9,0x38`；xy0@0/xy1@8/h0@+0x18/h1@+0x1c）+ 高度≤max_ivy_grow_height 过滤（h0<=max 或 h1<=max 跳过）【review-audit 纠错：步长 28→56、h0@+0xc→+0x18】 | collision-eco\decomp\decomp-0x14094c800.txt |
| `country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal` | 0x14094caa0 | 471 | 几何/碰撞 | O：get_wall_double_sided_normal——双面法线（v2 名址） | collision-eco\decomp\decomp-0x14094caa0.txt |
| `country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment` | 0x14094cc60 | 513 | 几何/碰撞 | O：intersect_ivy_growth_w_wall_segment——生长线是否穿墙段（v2 名址） | collision-eco\decomp\decomp-0x14094cc60.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ivy::ivy_grower::get_growth_point_proposal`（行 227）— 查询/计算
- `country_core::systems::ivy::ivy_grower::check_for_wall_jump`（行 308）— 谓词/校验
- `country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction`（行 491）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项

## 实现方式（算法级）

完整算法见 `collision-eco/ALGORITHMS.md` §4。要点（本轮跳表字节级验证）：

### 10 相跳表状态机（@0x142ab3be0，10 项 int32 相对偏移，基址=表自身）
分派（0x14094aa1b/0x14094aa2f 原指令）：`phase = *(u32*)(world+0x118)` → `jmp [表 + phase*4]`。
| 相 | 目标 | 语义（反汇编） |
|---|---|---|
| 0 | 0x14094C09F | 守卫相：xor 状态字节 bit0（@+0x48）、经 `[state+0x10]` 写回相值（+0x1c）；置位 → 守卫体 0x94ac51，否则主体 0x94c0c0 |
| 1/3/4 | 0x14094AC20 | 主入口（共用）：bit0 置位 → 守卫，否则主体 |
| 2 | 0x14094AC31 | 计时入口：`ucomiss [state+0x11c], 0`；≥0 → 主体 0x94c0a6，否则守卫检查 |
| 5 | 0x14094F2F2 | 二级跳表（0x142ab3bf4，r8 索引）分派 |
| 6-9 | 0x14094F306/31C/… | 尾部入口：串+长度 → jmp 0x140d86b90（panic 类） |

状态结构：{+0x10 ptr（相值写回通道）, +0x1c 相值, +0x48 守卫标志 bit0, +0x11c 计时 f32}。生长管线（CLOSURE e8 扫描）：每株 → last_point（0x1408694f0）→ get_direction（0x141234590）→ closest_segment_index（0x14094c800）→ 墙跳检查 → add_point（0x140869460）。

### closest_segment_index（0x14094c800，666B）
56B 步长墙段（{p0.xy@+0, p1.xy@+8, f32@+0x10/+0x14（打包进结果）, h0@+0x18, h1@+0x1c}）；过滤 1（**机器码字节级复核 2026-10-15，纠正旧"互斥"疑点**）：`ucomiss h0,max; jbe 进体` + `ucomiss h1,max; ja 跳过` → **h0≤max 或 h1≤max 均进体，仅 h0>max 且 h1>max 跳过**（= 保留至少一端点 ≤max 的段；旧 decompiler 的 `h0<=max||h1<=max → break` 为误读，实际是仅双端都 >max 才跳过）；过滤 2：t 处插值高 `h1·t+h0·(1−t) > max` → 跳过（ucomiss;ja 0x14094c95e）。**两过滤自洽**：语义 = 只保留生长投影点处墙高 ≤ max_ivy_grow_height 的段（穿过 max 边界的段按 t 裁）。t = dot(P−P0,D)/|D|² 夹取 [0,1]（1.0@0x142925904、eps 1e-20@0x142a7baa0）；断言 t∈[0,1]（0x142aa53ed）；距离度量 = 垂直分量 + 沿段分量（机器码 0x14094c970 区）；输出 {found u64, index u64, dist f32@+0x10, h 打包 u32@+0x14}。**调用方参数序**（ivy_spawner 调用点证实）：`(out, xz, 0, segments_ptr, count, max_height)`。
