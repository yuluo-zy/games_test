# RE-ANALYSIS2: ivy_spawner 路径 B 证据补挖（生态组 · ivy）

> 会话：eco re-analyst 续派（2026-10-15）。范围：ivy_spawner(0x14094cf90) 路径 B 的 5 处证据缺口。
> 依据：B patched EXE 机器码（RNE oracle 字节级）+ 反编译全量 + S_PUB32 身份 + perchance/fastnoise 依赖确认。
> 记号：O=机器码证实。不改 reconstruction 源码。

## 1. func_0x140c9cdc0 译码（O，S_PUB32 权威）

**`func_0x000140c9cdc0` = `glam::f32::vec2::Vec2::x0y`**（S_PUB32 `_$LT$glam..f32..vec2..Vec2$u20$as$u20$utils..swizzlers..SwizzlerX0Y$GT$::x0y::h6dae267728567868`，同址）。

- **签名/输入输出**：`x0y(&mut Vec3 /*out*/, &Vec2 /*xz*/)` → `out = { xz.x, 0.0, xz.y }`（2D 水平向量扩成 3D，y 置 0）。纯 swizzle，无分支、无 RNG。
- **行为结论**：ivy_spawner 用它将 2D 方向/垂向（曲线切向 × 法向偏移量）展开为世界 3D 向量，供 `auStack_148` 根位 = `x0y(perpendicular) × ±0.68 × 0.5 + 曲线点`。
- **不是侧向掷硬币**：左右法向选择的随机源在**另一个调用** `func_0x000141625a00` = **`perchance::PerchanceContext::get_bool`**（S_PUB32 确认；ivy_spawner 内调用点 0x14094d6b0）——见 §5 相位图。

## 2. 2.56 距校验（O，机器码证据）

- **常量**：`_UNK_142ab3214 = 2.56f`（rne-read f32 2.56000018；u64 0x404000004023d70b = 相邻 f32 {2.56@+0x14, 3.0@+0x18}）。
- **比较机器码**（shared spawn 区，反编译 decomp-0x14094cf90 行 534-567）：遍历既有根（`lStack_1b8` 个，`FUN_1412fa210` 收集的根位置指针数组，步长 8）：
  ```
  判向：0.0 <= existing.dir·(new−existing)（existing+0x1c 与 +0x14 打包 f32 点积）
  距校验：|existing.pos − new.pos|² < THRESHOLD
    uVar21 < 3  → THRESHOLD = 0.25（0x142925950）＝ 距 0.5（路径 A）
    uVar21 >= 3 → THRESHOLD = 2.56（0x142ab3214）＝ 距 1.6（路径 B）
  命中任一既有根 → 该次 spawn 尝试 return（放弃）
  ```
- **路径归属**：距校验在**共享 spawn 区**（两路径经各自根位计算后都到达），阈值由 `uVar21 = *(u32*)(IvyStorage@+0x120)` 决定——**路径 B（uVar21≥3）用 2.56（距 1.6）**，路径 A（uVar21<3）用 0.25（距 0.5）。

## 3. 垂直分量公式（O，非 /1.0 非中点）

`u_spawn = (uniform_f32 × 1.5 − 0.75) / curve_length + u_on_curve`

- `uniform_f32` = `perchance::PerchanceContext::uniform_f32`（FUN_141625ae0，调用点 0x14094dbb4）∈ [0,1)。
- **除数 = curve_length（墙曲线长度）＝ `fStack_158`＝ `*(f32*)(墙 entry − 0x270)`＝ PublicWallState 值 **+0x30**（Curve2 长 字段，per MODULE_MAP）+0x30）**；同字段被 `<= 0.0 → 跳过` 门控（decomp 行 350）。**非 /1.0 硬编码**。常量：1.5@0x1429258d0、−0.75@0x142ab3220。
- **加数 = `u_on_curve`＝ `fStack_1cc`＝ `clamp01(Curve::get_approx_u_from_pos)`**（FUN_141469a20 = **`utils::curve::Curve::get_approx_u_from_pos`**，S_PUB32 确认；decomp 行 353-362 clamp01）——生长点在墙曲线的**精确投影 u**，**非中点近似**。
- **钳位**：`[0.000928482 @0x142960690, 1.0 @0x142925904]`（decomp 行 612）。u_spawn 越界 → 放弃该次尝试。
- 该 u_spawn 是**合法性闸门**（合法才进入随机格 spawn 路径），最终根位由格采样 + closest_segment_index 落点（与路径 A 共享机制）。

## 4. spawn range / 逐墙随机 u（O）

- **随机墙**：`FUN_141625a50 = perchance::PerchanceContext::usize_less_than`（S_PUB32）调用点 0x14094d2e7，`usize_less_than(wall_count)` → 墙序号 ∈ [0, wall_count)。
- **逐墙 u**：`func_0x000141625ae0 = perchance::uniform_f32` 调用点 0x14094d394 → **u ∈ [0,1) 单位曲线参数**（非按曲线长度缩放）；校验 `0.0 <= u <= 1.0`（否则 CurveU panic @0x142aa53c0/0x142aa5498）→ `Curve::get_coord_at_u(curve, u)`（FUN_141467ce0）取曲线点。
- **噪声密度闸门（O，路径 B 特有）**：`fVar29 = FastNoise::get_noise(resource)`（FUN_141743760 = **`bracket_noise::fastnoise::FastNoise::get_noise`**，S_PUB32 确认；decomp 行 836），`if (noise < *(f32*)(resource+0x78) || ==) return`——噪声低于阈值@+0x78 则跳过该墙（spawn 密度过滤）。resource = `*param_2`（IvySpawnNoise 类资源）。

## 5. 相位图（ivy_spawner 全调用点，O）

```
ivy_spawner(0x14094cf90):
  模式门 AppMode==1 → 株数上限≈40000 → uVar21 = IvyStorage@+0x120
  ├─ 路径 A（uVar21<3）：光标→格(RNG 32/65)→closest_segment_index(0.5f)→墙 loop
  └─ 路径 B（uVar21>=3）：
       usize_less_than(墙数) @0x14094d2e7 → 选墙
       uniform_f32 → u∈[0,1) @0x14094d394 → get_coord_at_u 取点
       x0y(curve 点) @func_0x140c9cdc0 → FastNoise 密度闸门（≥阈值@+0x78 才继续）
       → 跳共享墙 loop（0x14094d49f）
  共享：get_bool 侧向掷硬币 @0x14094d6b0（±0.68 法向）→ 根位=x0y(垂向)×±0.68×0.5+曲线点
       → 距校验（0.25/2.56）→ uniform_f32 垂直 @0x14094dbb4 → u_spawn 钳[0.000928,1.0]
       → 再采格（usize_less_than @0x14094da34 + uniform_f32）→ closest_segment_index → spawn_ivy(0x1408696f0)
```

**grower 10 相状态机（点 5，已挖透，指认位置）**：`chapters/systems/ivy/ivy_grower.rs.md`（10 相跳表 @0x142ab3be0，分派 `world+0x118`；相 0=守卫翻转 / 1,3,4=主入口 / 2=计时 / 5=二级跳表 0x142ab3bf4 / 6-9=panic 尾入口）+ `collision-eco/ALGORITHMS.md` §4.1（跳表目标表 + 状态结构 {+0x10 写回, +0x1c 相值, +0x48 守卫 bit0, +0x11c 计时 f32}）。无需重挖。

## 6. 未决（U，不臆造）

1. 噪声闸门 `resource+0x78` 阈值的来源/默认值（IvySpawnNoise 构造参数，insert_ivy_resources 0x1409440c0 内，FastNoise 频率 0.3/lacunarity 2.0 已 O）。
2. `Curve::get_approx_u_from_pos` 的精确投影算法（共享 utils/curve.rs，跨域；此处仅确认其为 u 来源）。

## 7. 证据

- decomp-0x14094cf90.txt（collision-eco/decomp/，38KB 全量）
- S_PUB32：0x140c9cdc0=Vec2::x0y、0x141469a20=Curve::get_approx_u_from_pos、0x141467ce0=Curve::get_coord_at_u、0x141743760=FastNoise::get_noise、0x141625a50=PerchanceContext::usize_less_than、0x141625ae0=uniform_f32、0x141625a00=get_bool
- rne-read：2.56@0x142ab3214、0.25@0x142925950、1.5@0x1429258d0、−0.75@0x142ab3220、0.000928@0x142960690、1.0@0x142925904
