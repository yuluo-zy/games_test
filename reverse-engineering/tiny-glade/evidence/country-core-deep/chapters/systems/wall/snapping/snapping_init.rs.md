# systems/wall/snapping/snapping_init.rs

> 文件作用：吸附初始化：重置 mode/snapped；DebugGui 禁用或进行中墙空返回；首段长<=1.65 返回；后续点投影首段 <=1.5 到 mode=1+snapped。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_init::snapping_init` | 0x140ada600 | 11 | snapping_init（重置；首段<=1.65 返回；投影<=1.5 → mode=1） | O | wall\decomp\decomp-0x140ada600.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

### snapping_init 0x140ada600（O，全读 + 机器码核验）

签名（4 参）：param_1 = ResMut\<SnappingMode>（tick 回写 @+0x1c），param_2 = &InnerWalls（hashbrown，条目步长 0x78 = {WallId, WallState 0x70}），param_3 = InProgressCurve 资源（+0x0 = Option\<WallId> 哨兵，+0x80 = 当前墙 WallId，曲线 Vec @+0x8/+0x10），param_4 = 调试资源（+0xfd 字节）。

处理链：
1. **重置**：`*SnappingMode = 0`（只清零 mode@0；snapped@1 不动）。
2. `param_4+0xfd != 0` → return（O：`cmp byte[+0xfd],0`。重建注释"DebugGui 禁用返回"方向存疑：机器码是 **flag 置位即 return**，即标志=启用则跳过吸附）。
3. `InProgressCurve.+0x0 == i64::MIN`（None 哨兵，机器码 `xor eax,eax; cmp [r12],rax; jo`）→ return（无进行中墙）。
4. 以 +0x80 的 WallId 在 InnerWalls 做 hashbrown SIMD 探针（FUN_1405b0d70 哈希 + 16B ctrl 组比较）。
5. **命中后阈值**：`f32@WallState+0x30 <= 1.65f (0x3fd33334 @0x142ae0d64)` → return。**纠错**：章节/重建注释写"首段长度<=1.65"，机器码读的是 WallState+0x30 的 f32（PublicWallState/WallState 共用的**曲线长度**字段，同址亦被 is_valid 断言 `0.0<len` 使用）——语义应为"墙曲线长度 <=1.65 不进入吸附"，非"首段"。
6. 读墙曲线：`points ptr@value+0x8, count@value+0x10`（cap-first Vec）。
7. **投影步行**（count>=2 才进入；count<2 直接跳末段）：
   - `dist² = |p1-p0|²`（首段长平方）；`if dist²<=0 → 向量清零`。
   - 逐后续点 p：`t = ((p-p1)·(p0-p1))/dist²`（投影参数，从 p1 向 p0）；残差 `res = p - (t·p0 + (1-t)·p1)`（垂直距离分量）。
   - 循环条件 `while SQRT(res²) <= 1.5f (0x3fc00000 @0x1429258d0)`：**首点偏离 >1.5 即停**。
   - 除零保护：`dist²` 先 `max(eps)`，eps = **1e-20f (0x1e3ce508 @0x142a7baa0)**。
8. 末：tick 回写 → `mode=1`，`snapped = (count*8 != 停处索引)`。

### snapped@1 语义（I，推断）

snapped=true 当且仅当**存在点偏离首段 >1.5**（曲线未贴首段，段吸附模式"生效"）；snapped=false 表示曲线已直（沿首段），此时 snapping_to_ends 的遍历条件 `while snapped==0` 才放行去探测端吸附。字段名与直观"吸附中"相反，实为"段吸附当前占据"标志。

### 常量表

| 常量 | 值 | 含义 |
|---|---|---|
| 0x142ae0d64 | 1.65f | 墙曲线长度阈值（≤ 跳过） |
| 0x1429258d0 | 1.5f | 首段垂直距离阈值 |
| 0x142a7baa0 | ~1e-20f | 除零 eps（dist² 下限） |
| 0x142925904 | 1.0f | 1-u 权重（lerp） |
