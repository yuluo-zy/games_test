# utils/debug_viz.rs

> 文件作用：**调试可视化图元（DebugSphere / DebugColor / DebugRectangle3d / DebugRectangle2d /
> DebugLine）**。builder 结构 + `show()` 把图元构造成 box 结构（0x48/0x50 B，含 fmt `Arguments`
> 数据表）并经 `FUN_14146c880`（Commands 族写入）排队渲染——**非 bevy Gizmos，是游戏自有
> 命令队列**。算法级：show 全为"几何端点计算 → box 打包 → fmt 渲染 → 队列写"四步；与共享
> `utils` 无重叠（共享 geometry 无调试图元族）。

## 核心算法

```
DebugSphere::new:   {tag/type@0, radius=0.1f@0x10, center Vec3@0x14, alpha=1.0f@0x20}（原字节）
  .color/.radius/.solid: 字段 setter（短函数）
  .show: FUN_14146c880 排队（box + fmt Arguments）
DebugRectangle3d::show(center, size, rot?):
  4 角点 = 中心 ± 半尺寸经 FUN_140c99600（角点展开）+ FUN_140c9cde0（点变换，rotation 应用）
  边长@box+0x3c；默认线宽 1.0；dashed_line 开关
DebugRectangle2d::show: 4 f32 角点（FUN_140c991f0，2D 版本）+ box 0x48 + fmt
DebugLine::show(points): 折线总长 = 逐段 sqrt 累加（0x50B box）+ 判别跳表 UNK_142ae731c
  （按段数选 box 形状）+ FUN_1412f7540（fmt）+ FUN_14146c880（排队）
```

函数总计 29：业务 17 / 内联无独立符号 4 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `DebugSphere::new` | 0x140b02490 | 84 | 构造（radius=0.1@0x10, center@0x14, alpha@0x20=1.0） | **O** | 原字节手解 |
| `DebugSphere::color` | 0x140b024c0 | 94 | 设置颜色 | **O** | 原字节短函数 |
| `DebugSphere::radius` | 0x140b024e0 | 100 | 设置半径 | **O** | 原字节短函数 |
| `DebugSphere::solid` | 0x140b02500 | 105 | 实心开关 | **O** | 原字节短函数 |
| `DebugSphere::show` | 0x140b02520 | 116 | 构 box + 排队 | **O** | FUN_14146c880 写 |
| `DebugRectangle3d::new` | 0x140b02e10 | 186 | 构造（默认线宽 1.0） | **O** | 原字节短函数 |
| `DebugRectangle3d::color` | 0x140b02e40 | 195 | 设置颜色 | **O** | 原字节短函数 |
| `DebugRectangle3d::line_width` | 0x140b02e70 | 201 | 线宽 | **O** | 原字节短函数 |
| `DebugRectangle3d::dashed_line` | 0x140b02ea0 | 206 | 虚线开关 | **O** | 原字节短函数 |
| `DebugRectangle3d::show` | 0x140b02ed0 | 211 | 4 角点 + 变换 + box 0x48 + fmt + 排队 | **O** | FUN_140c99600/FUN_140c9cde0/FUN_140d79f90/FUN_14146c880 |
| `DebugRectangle2d::show` | 0x140b04640 | 332 | 4 f32 角点 + box + fmt + 排队 | **O** | FUN_140c991f0/FUN_140d79f90 |
| `DebugLine::new` | 0x140b04e70 | 369 | 构造 | **O** | 原字节短函数 |
| `DebugLine::color` | 0x140b04eb0 | 391 | 设置颜色 | **O** | 原字节短函数 |
| `DebugLine::set_ui_pass` | 0x140b04ee0 | 396 | UI pass 标记 | **O** | 原字节短函数 |
| `DebugLine::dashed` | 0x140b04f10 | 401 | 虚线开关 | **O** | 原字节短函数 |
| `DebugLine::line_width` | 0x140b04f40 | 407 | 线宽 | **O** | 原字节短函数 |
| `DebugLine::show` | 0x140b04f70 | 412 | 折线总长（逐段 SQRT 累加）+ box 0x50 + 判别跳表 + 排队 | **O** | UNK_142ae731c 跳表/FUN_1412f7540/FUN_14146c880 |

## 内联/无独立符号函数

- `DebugSphere::line_width`（行 111）— **I**（名称推断，无独立符号）
- `DebugColor::as_array`（行 165）— **O**（RGBA→[f32;4]）
- `DebugRectangle2d::new`（行 312）— **O**（构造，原字节内联宿主）
- `DebugRectangle2d::color`（行 321）— **O**（设置颜色）

## 派生胶水（8 项：closure、fmt 等）

- `impl$6::fmt`（0x140b05b60）/ `impl$7::fmt`（0x140b05c70）— Debug fmt 胶水 **O**；fmt 经
  `FUN_140d79f90`（fmt::format，与 load_json 共用）渲染 box 内 Arguments。
