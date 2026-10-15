# systems/wall/calc_wall_bend.rs

> 文件作用：墙弯折角重算系统：消费 OnWallChanged，读发布曲线前 4 段法向 delta 相邻点积平均到 acos 到 clamp<=1.0（点>5 才算，否则 0），写 WallBendChangedEvent（16B 行）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::calc_wall_bend::calc_wall_bend` | 0x140a8a710 | 13 | calc_wall_bend（>5 点算 bend，acos clamp<=1.0；写 WallBendChangedEvent） | O | wall\decomp\decomp-0x140a8a710.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）（O，decomp-0x140a8a710）

**处理链**：puffin scope（串 0x142ad8b93）→ EventReader<OnWallChanged>（行 16B `{event_count, WallId}`）
逐事件 → **PublicWalls hashbrown 查表**（条目步长 **0x2a8** = `{key WallId u64@+0, value PublicWallState 0x2a0@+8}`；
find 先经 map+0x20 区 FUN_1405b0d70 + SIMD ctrl 探测）→ 未命中跳过 → 命中判退化 → 计算 bend → 写事件。

**退化校验（硬 panic，不跳过）**：`value+0x10`（raw 曲线点计数）`< 2` 或 `value+0x30`（raw 长度 f32）`<= 0.0`
→ `panic_str("assertion failed: w.simplified_curve_2d.is_valid()" @0x142ada193, line 0x32=50, public_walls.rs @0x142ad91d0)`。

**bend 算法**：发布曲线 `value+0x40`=点 ptr、`value+0x48`=计数，点 **12B `{f32 x@0, f32 y@4, f32 z@8}`**
（本函数只读 x@0 与 +8，即地面 (x,z)）。计数 `< 6` → bend=0。否则 `for i in 0..4` 取
**`P[n-1-i] - P[n-2-i]`（从曲线末端倒数 4 段）**归一化（1/len，SQRT），入 `SmallVec<[Vec2;4]>`
（8B 元素；grow FUN_140b81f90）；随后相邻 3 点积 d0·d1/d1·d2/d2·d3 累加 → `divss/(n-1)` →
`maxss`(-1)+`minss`(1) 双夹取 → **单次 acos**（0x140a8acf0 段，movsd/mulps/movshdup/addss 水平点积）→
结果 `minss ≤ 1.0`（常量 1.0f @0x142925904）。

**事件写出**：ResMut<Events<WallBendChangedEvent>>（param_3，`param_3[2]=param_3+0x1c` 回写），
行 16B `{event_count u64@+0, bend f32@+8}`；Vec 扩容 FUN_140562630（16B 元素）。

**纠错（vs 重建）**：重建 `compute_bend` 从 `points[i]→points[i+1]`（首端 4 段）；机器码从
**末端** `P[n-2-i]→P[n-1-i]`，方向相反需改。重建「value+0x48/+0x50」= 条目相对 = value+0x40/+0x48，一致。
