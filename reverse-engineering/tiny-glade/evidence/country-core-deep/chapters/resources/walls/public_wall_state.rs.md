# resources/walls/public_wall_state.rs

> 文件作用：公共墙状态 PublicWallState 0x2a0B：raw Curve2{ptr@+8,len@+0x10,长@+0x30}、3D 曲线@+0x88、高度数组{ptr@+0xe0,len@+0xe8}、max_y/gable 缓存{+0x108/+0x10c, +0x110/+0x114}、形状记录 28B@+0x258（0=Circle/1=Rectangle/2=Freehand）、发布长度@+0x28c、style@+0x299；方法面含 wall_space/u_space（Option<f32>，u_space=1.0f 常量）与锚高投影。

函数总计 17：业务 12 / 内联无独立符号 2 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape` | 0x140b2a5d0 | 126 | get_wall_shape（tag<2 否则 unwrap panic） | O | wall\decomp\decomp-0x140b2a5d0.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape` | 0x140b2a600 | 131 | try_get_wall_shape（tag>=2 → None） | O | wall\decomp\decomp-0x140b2a600.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::max_y` | 0x140b2a620 | 146 | max_y（+0x108 位0 → +0x10c，否则 panic wall heights not calculated） | O | wall\decomp\decomp-0x140b2a620.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable` | 0x140b2a680 | 151 | max_y_gable（+0x110 位0 → +0x114；PDB 名 is_premade_shape 错位） | O | wall\decomp\decomp-0x140b2a680.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y` | 0x140b2a6e0 | 156 | flat_roof_y（max_y - 0.79） | O | wall\decomp\decomp-0x140b2a6e0.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::wall_space` | 0x140b2a740 | 161 | wall_space（{tag!=2, f32@+0x28c} = Option<f32> 发布长度） | O | wall\decomp\decomp-0x140b2a740.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::wall_space_length` | 0x140b2a760 | 173 | wall_space_length（纯 +0x28c） | O | wall\decomp\decomp-0x140b2a760.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::u_space` | 0x140b2a770 | 177 | u_space（{tag!=2, 1.0f@0x142925904} = Option<f32> 归一全区间） | O | wall\decomp\decomp-0x140b2a770.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle` | 0x140b2a790 | 184 | is_rectangle（tag==1 复制 24B） | O | wall\decomp\decomp-0x140b2a790.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::is_circle` | 0x140b2a7d0 | 195 | is_circle（tag==0 复制 16B） | O | wall\decomp\decomp-0x140b2a7d0.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz` | 0x140b2a800 | 217 | corner_pos_and_normal_xz（仅 Rectangle：Rectangle2d::get_point/get_corner_normal） | O | wall\decomp\decomp-0x140b2a800.txt |
| `country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height` | 0x140b2a890 | 231 | get_wall_attachment_height（premade→max_y；freehand→u×raw_len→clamp01→2D 插值→3D 投影→高度数组插值） | O | wall\decomp\decomp-0x140b2a890.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape`（行 113）— 谓词/校验
- `country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed`（行 118）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项
## 实现方式（算法级）

### PublicWallState 布局（O，DIGEST §1.1 深化）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x08/+0x10/+0x30 | raw Curve2：点数组 ptr / 点计数 u64 / 曲线长度 f32 | get_wall_attachment_height 0x140b2a890 |
| +0x88 | 3D 曲线（Curve3 区） | FUN_141469e00(self+0x88,…) 投影 |
| +0xe0/+0xe8 | f32 高度数组 ptr / len | 同上第二组插值 |
| +0x108/+0x10c | max_y 缓存存在位(bit0) / f32 | max_y 0x140b2a620、get_wall_attachment_height |
| +0x110/+0x114 | gable max_y 缓存位 / f32 | max_y_gable 0x140b2a680（PDB 名 is_premade_shape 错位） |
| +0x258..+0x274 | 形状记录 28B（tag u32 + 24B 联合；0=Circle 16B / 1=Rectangle 24B / 2=Freehand） | get_wall_shape / is_rectangle / is_circle |
| +0x28c | 发布曲线长度 f32 | wall_space_length 0x140b2a760 直返 |
| +600(0x258) | 形状 tag | get_wall_attachment_height `*(uint*)(+600)<2` |

### 形状访问器（O）

- `get_wall_shape` 0x140b2a5d0：tag<2 → 返回 &形状记录；否则 unwrap panic。
- `try_get_wall_shape` 0x140b2a600：tag>=2 → null（None）。
- `is_rectangle` 0x140b2a790：tag==1 → 复制 24B；`is_circle` 0x140b2a7d0：tag==0 → 复制 16B。
- `corner_pos_and_normal_xz` 0x140b2a800：仅 Rectangle 有效（Rectangle2d::get_point / get_corner_normal）。
- `wall_space` 0x140b2a740：`{al: tag!=2, xmm0: f32@+0x28c}` = Option<f32>（**判别是形状 tag 非 enclosure**——合同需改）。
- `wall_space_length` 0x140b2a760：纯 movss +0x28c。
- `u_space` 0x140b2a770：`{al: tag!=2, xmm0: 1.0f@0x142925904}` = Option<f32> 归一全区间。
- `max_y` 0x140b2a620：(+0x108)&1==0 → panic "wall heights have not been calculated yet..."（0x142aeb068）；否则 +0x10c。
- `max_y_gable` 0x140b2a680：(+0x110)&1==0 → 同上 panic；否则 +0x114。
- `flat_roof_y` 0x140b2a6e0：max_y - 0.79。

### get_wall_attachment_height 0x140b2a890（O，全流程）

```
签: get_wall_attachment_height(self, u f32) -> u64 {u32, f32}
1. if ((+0x108 & 1) == 0) → fmt panic "wall heights have not been calculated yet. Perhaps you are trying to access it before `init_wall_height_and_arch_segment_height` system had a chance to run?"（0x142aeb068，file 0x142aeb078）
2. if (形状 tag@+600 < 2) → return {0, max_y@+0x10c}                    // premade 直接用缓存
3. Freehand:
   param_2 = u * raw_len@+0x30
   if (param_2 为 NaN) → panic_str "assertion failed: !u.is_nan()"（0x142aa53d0, line 0x1d=29, file 0x142aeaf98）
   if (|param_2| == INFINITY) → panic_str "assertion failed: !u.is_infinite()"（0x142aa5498, line 0x22=34）
   fVar6 = max(0, param_2)；t = min(fVar6, 1.0)                          // clamp01
4. FUN_141467ce0(self, t) → Curve2 取段 idx u64 + 段内分数 extraout_XMM0_Da
   越界: idx>=点计数+0x10 或 idx+1>=计数 → panic
5. 2D 线性插值: x = lerp(pt[idx].x, pt[idx+1].x, t)；y = lerp(pt[idx].y, pt[idx+1].y, t)（raw 点@+8，8B 元素）
6. FUN_141469e00(self+0x88, x, y) → 3D 曲线投影 → 索引 u64 + 分数 extraout_XMM0_Da_00 + 附加 u32 extraout_XMM0_Db
7. if (idx < 高度计数@+0xe8): idx+1 < 计数 → fVar7 = heights[idx@+0xe0]；
   return {extraout_XMM0_Db, t'*(heights[idx+1]-heights[idx]) + heights[idx]}   // 高度数组线性插值
   越界 → swi(3) trap
```

### 新常量表

| 常量 | 地址 | 语义 |
|---|---|---|
| 1.0f | 0x142925904 | u_space / clamp01 上界 |
| "wall heights have not been calculated yet. Perhaps you are trying to access it before `init_wall_height_and_arch_segment_height` system had a chance to run?" | 0x142aeb068 | max_y/height 缓存缺失 |
| "assertion failed: !u.is_nan()" | 0x142aa53d0 | freehand u×len NaN（line 29） |
| "assertion failed: !u.is_infinite()" | 0x142aa5498 | freehand u×len Inf（line 34） |

### 跨模块调用

FUN_141467ce0（Curve2 取段+分数）、FUN_141469e00（2D→3D 投影，utils::curve 域）、init_wall_height_and_arch_segment_height 系统（panic 文案引用，索引外）。PublicWallState 消费于 publish_public_wall_state 0x140a8b530、spatial_hash_update 0x140a8fd90、clamp_wall_height 0x140ac7f30、get_closest_point_u_to_cursor 0x140ac7400。

### 遗留 U

- 3D 曲线@+0x88 的 Curve3 结构内部字段（投影由 FUN_141469e00 封装）。
- 返回 u64 的 u32 分量（extraout_XMM0_Db）语义 I（疑为投影段/索引）。
