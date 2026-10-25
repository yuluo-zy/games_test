# resources/roofs/roof_noise.rs

> 文件作用：**屋顶顶点噪声位移**——`gpunoise` 是 shader 风格 2D value-noise 的 CPU 复刻（floor + 4 角 Wang 哈希 + smoothstep 插值），
> `apply_roof_noise` 把噪声按强度叠加到屋顶顶点 y。`revert_roof_noise` 为逆操作（内联）。
> 消费方：roof_visual_shape 的 `get_height_01_unclamped_from_ws`（0x1408e4790 直接调 noise 参与反向高度采样）与屋顶网格生成。

函数总计 8：业务 2 / 内联无独立符号 6 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `apply_roof_noise` | 0x1412348f0 | 6 | `strength=clamp(s,0,1)`；`vertex.y += noise(x*0.1, z*0.1) * 1.5 * (1-2s) * s*s` —— 顶点 y 叠加噪声×强度 smoothstep（振幅 1.5） | O | roofs-rt/decomp/decomp-0x1412348f0.txt |
| `gpunoise::noise` | 0x140a152c0 | 62 | 2D value noise：`floor` 取格，4 角经 hash 链（0x19660d/0x8001/0xae135d3 Wang 型混合 + 异或/移位），`smoothstep`（x²(3-2x)）双线性插值，返回 [0,1] | O | roofs-rt/decomp/decomp-0x140a152c0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `revert_roof_noise`（行 12）— 逆操作（从顶点 y 减噪声），内联进调用点
- `gpunoise::hash1`（行 24）— 单 u32 哈希（异或/移位混合）
- `gpunoise::hash_combine2`（行 34）— 双 u32 哈希合并
- `gpunoise::hash2`（行 50）— 2D 坐标哈希（角点种子）
- `gpunoise::uint_to_u01_float`（行 54）— 哈希 → [0,1) 浮点
- `gpunoise::lerp`（行 79）— 线性插值

## 关键数值（已证实）

- 噪声采样尺度 = 0.1（x/z 均乘 0.1）；振幅 = 1.5（_UNK_1429258d8）；smoothstep = `(1-2s)*s*s`。
- hash 常量：`0x19660d`（乘）、`0x8001`、`0xae135d3`（加/异或）——Wang/int 型散列。

## 实现方式（算法级）

完整算法见 `roofs-rt/ALGORITHMS.md` §5。**常量更正（机器码，覆盖上方"关键数值"节）**：

### gpunoise::noise（0x140a152c0）
`floor` 取格（饱和截断 NaN→0、>0x4effffff→0x7fffffff）→ 4 角 Wang 型哈希链（`(v·0x401 >> 6 ^ v·0x401)·9 → (h >> 0xb ^ h)·0x8001 → (y·0x19660d + h + 0x19660d)·0x19660d + 0xae135d3`；角种子表 @0x142ac7e10..e1c：e10/e14 = 0x0ae135d3、e18/e1c = 0）→ finalizer（0x13a58ad 掩码 + 移位异或）→ `u01 = (bits | 0x3f800000) − 1.0` → x/y 双轴 smoothstep `(3−2u)·u²` 双线性 → [0,1]。

### apply_roof_noise（0x1412348f0）
`s = clamp(strength, 0, 1)`；`n = noise(x·0.5, z·0.5)`（**尺度 0.5**，原记 0.1 错）；`vertex.y += n · 0.3 · (3−2s) · s²`（**振幅 0.3**，原记 1.5 错；**smoothstep (3−2s)s²**，原记 (1−2s)s² 错）。
