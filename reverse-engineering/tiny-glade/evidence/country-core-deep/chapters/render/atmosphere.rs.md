# render/atmosphere.rs

> 文件作用：**太阳/天空调色板**——`SunSkyColors`（48 B = 3×Vec4）+ `DerivedAtmosphere`（4×Vec3 序列化类型）。
> 证据：`reconstruction/country-core-render/wp4/contracts/view_frame_light_atmos_texture.md`（O）+ render-gap DIGEST §1 #5。

函数总计 10：业务 2 / 内联无独立符号 0 / 派生胶水 8

## 布局表（O）

| 类型 | 布局 | 证据 |
|---|---|---|
| `SunSkyColors` | 48 B = 3 × Vec4（`lerp` 读写 [0..11] f32） | lerp 0x140a97650 |
| `DerivedAtmosphere` | 4 × Vec3 0xc B（`{sun_tint_color, moon_tint_color, sky_low_color, sky_high_color}` @ +0/+0xc/+0x18/+0x24） | serialize 0x140322a10（字段串 0x1429bd228/236/245/252） |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `_::impl$0::serialize<…PrettyFormatter>` | 0x140322a10 | 4 | 猜测：serde 序列化；证实：**derive(Serialize) 胶水**，宿主 = `DerivedAtmosphere`（非 SunSkyColors） | **O**：`{` + 4 字段 `sun_tint_color/moon_tint_color/sky_low_color/sky_high_color` + `}`；各字段 0xc B（Vec3）。**纠错**：`SunSkyColors::lerp` 读 3 个 xmmword（3×Vec4），故 serialize 对象是同模块 `DerivedAtmosphere`（4×Vec3，48 B）——两者不同型 | render-gap §1 #5 |
| `SunSkyColors::lerp` | 0x140a97650 | 15 | 猜测：插值；证实：逐分量 `self*(1-t) + other*t` | **O**（87 B）：12 个 f32 逐分量 `other*t + self*(1-t)`（mulps×2 + addps；1-t 先算）。**不是** glam 的 `a+(b-a)*t`（舍入不同，t=1 时后者可能得不到恰好 other） | wp4 view_frame_light_atmos_texture |

## 派生胶水（8 项：default、deserialize、expecting、visit_map、visit_seq、visit_str 等）

- serde Deserialize/Default 派生族（SunSkyColors 的 derive 展开）——不补业务。

## 纠错/差异

- reconstruction `atmosphere.rs` 现有实现与机器码一致（`lerp` 用 `other*t + self*w` 已正确落地；注释已列 O 证据），无差异。
- 命名槽映射（sun tint day/dusk、sky base/high/low/horizon/clouds）为 I（typed gap）。
