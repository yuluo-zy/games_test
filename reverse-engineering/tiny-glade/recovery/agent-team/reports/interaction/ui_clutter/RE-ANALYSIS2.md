# RE-ANALYSIS2: ui_clutter（二轮补挖 · ui_clutter）

> 会话：interaction re-analyst（2026-10-15，二轮）。reviewer 第 4 项：rmb/ui_convert 旋转公式
> （疑 acos(position.y)）与 SPIRAL_ANGLE/类型表。依据：B EXE 机器码 + RNE 常量直读 +
> CircularF32 定案。记号：O/I/U；「旧结论 X → 新证据 Y」=修正。不改 reconstruction 源码。

## 1. handle_rmb_autoclutter 旋转公式（O，疑错确认）

**疑错确认：旋转 NOT `acos(position.y)`。**

rmb_autoclutter @0x140976ca0 旋转链（本轮常量直读 + 机器码）：
```
# 输入：param_1 = 位置/quat {fVar23=x, fVar24=y, fVar27=z, fVar25=w（第4分量）}
fVar30 = -1·fVar23 + (-0)·fVar24 + (-0)·fVar27   = -x
X = sqrt( (2·w)·(y·(-1) − z·0) + (2·(-x))·x + (w²−(z²+y²+x²))·0 )
  = fabs( sqrt(−2·w·y − 2·x²) )                    # ^_UNK_1429258f0(0x80000000) = 符号翻转
# 角度（两分支）：
if 条目字节@+8 bit0 == 0:                           # 分支 A
    angle = −π/2 − CircularF32::as_f32(条目+4)       # CircularF32::as_f32 = fmod(·, 2π) @0x1416f3210
else:                                               # 分支 B
    angle = fabs( sqrt(条目+0x10) )                  # 条目值开方（^0x80000000）
rot = |X − angle| · _UNK_142925870(0.5)
输出 = { cos(rot), sin(rot) }                        # func_0x142923e80 / func_0x142923fd0
```
常量（RNE 直读）：
| 常量 | 值 | 语义 |
|---|---|---|
| _UNK_142a10ff0 | **−1.5708（−π/2）** | 旋转基偏置（**非 acos 基**） |
| _UNK_1429a9080/84/88 | −0.0, −0.0, **−1.0** | 「点积」实为 −x 分量取反 |
| _UNK_142a923f0 | −1.0 | 交叉项系数 |
| _UNK_1429cd100 | −0.0 | 交叉项（z 项清零） |
| _UNK_142a923e0 | −0.0 | 模方项清零 |
| _UNK_1429258f0 | −0.0(0x80000000) | sqrt 参数符号翻转（fabs） |
| _UNK_142925870 | 0.5 | 半角 |
| CircularF32::as_f32 @0x1416f3210 | fmod(·, 2π) | 角度回绕 |

- **旧结论 X → 新证据 Y**：章节「三向量点积 fVar30 = a·p → 平方根 → 角度」——
  机器码系数 {−0,−0,−1} 使 fVar30 = −x（非点积）；角度含 **CircularF32 (fmod 2π) + −π/2 偏置
  + 0.5 半角**。reconstruction 若用 acos(position.y) 应改。

## 2. ui_convert_autoclutter_to_decoclutter 旋转（O）

@0x14099ca60 旋转 = **quaternion 矩阵**（非 acos）：
- `fVar52 = (v[2] + v[0x14]) · 0.5 · 0.5 + _UNK_142925984(0.1)`（旋转角合成）。
- `fVar53/57/58/59 = 分量 ^ _UNK_1429cd100/104/108/10c`（符号翻转 = quat 轴取反）。
- 矩阵乘 `_UNK_1429cd170..19c`（**±1.0** 反射矩阵）+ `_UNK_1429258e0/e4`（0.5 scale）合成
  旋转（屋顶空间，get_clutter_rot_in_roof_space 内联）。
- `FUN_1408b83d0`(ClutterMeshTy::from)×N 取杂饰类型；最终推 0x48B 事件。
- **旧结论 X → 新证据 Y**：旋转非 acos；为 quat 轴翻转 + ±1 反射矩阵 + 0.5 合成。

## 3. debug_spawn_all_clutter 螺旋常量（O）

- **SPIRAL_ANGLE `_UNK_142abbcec` = 2.399827718734741**（≈黄金角 2.39996，rad）；`θ = n·SPIRAL_ANGLE`，
  `x = cos(θ)·√n`、`z = sin(θ)·√n`（func_0x142923e80/142923fd0）。
- **类型表 `&UNK_142aa0013`（42 项，ClutterMeshTy 变体索引）**：
  `{0x0,0xf,0x2b,0x13,0x1,0x2,0x3,0x4,0x5,0x6,0x7,0x8,0x9,0xa,0xb,0xd,0xc,0x10,0x11,0x12,
   0x14,0x15,0x16,0x25,0x26,0x27,0x28,0x36,0x17,0x18,0x29,0x37,0x19,0x1a,0x2c,0x2d,0x2e,0x2f,
   0xe,0x31,0x30,0x32}` —— 逐项 `(&UNK_142aa0013)[n]` 推 0x48B 事件 type 槽。
- y = `TerrainHeightsData::sample_at_world_xz`(0x1409e9640, x, z)。
- **旧结论维持**：章节「0x2a=42、r=√n、θ=n·_UNK_142abbcec」正确；SPIRAL_ANGLE 数值本轮定案。

## 4. 0x48B spawn 事件布局（互证，O）

`{pos 12B@0…, subtype u8@+0x29=3, rot u32@+0x2c, pos/entity@+0x30..0x38, type u8@+0x3c,
  entity u64@+0x40}`（rmb 写 rot@+0x2c=§1 公式结果；debug_spawn 写 rot=0）。

## 5. 二轮证据落盘

- `interaction-group/decomp/decomp-0x1416f3210.txt`（CircularF32::as_f32 = fmod 2π）
- RNE 常量直读（§1/§2/§3 全表）+ 类型表 42 字节
- `results/out-rot.json`（FUN_1416f3210 身份批）
